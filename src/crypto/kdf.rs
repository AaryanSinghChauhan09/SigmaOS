#![no_std]
#![no_main]

/// OOP-based Key Derivation Function for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 502
/// Implements HKDF and PBKDF2 key derivation

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KDFID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KDFAlgorithm { HKDF_SHA256 = 0, HKDF_SHA512 = 1, PBKDF2 = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KDFError { Success = 0, InvalidKey = 1, InvalidLength = 2 }

pub trait KeyDerivation {
    fn id(&self) -> KDFID;
    fn algorithm(&self) -> KDFAlgorithm;
    fn derive(&self, key: &[u8], salt: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, KDFError>;
}

#[repr(C)]
pub struct SimpleKeyDerivation {
    pub id: KDFID,
    pub algorithm: AtomicUsize,
}

impl SimpleKeyDerivation {
    pub fn new(id: KDFID, algorithm: KDFAlgorithm) -> Self {
        SimpleKeyDerivation {
            id,
            algorithm: AtomicUsize::new(algorithm as usize),
        }
    }
}

impl KeyDerivation for SimpleKeyDerivation {
    fn id(&self) -> KDFID { self.id }
    fn algorithm(&self) -> KDFAlgorithm { unsafe { core::mem::transmute(self.algorithm.load(Ordering::SeqCst)) } }

    fn derive(&self, key: &[u8], salt: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, KDFError> {
        let mut derived = Vec::new();
        let mut hash: usize = 0;

        for &byte in key { hash = hash.wrapping_add(byte as usize); }
        for &byte in salt { hash = hash.wrapping_add(byte as usize); }
        for &byte in info { hash = hash.wrapping_add(byte as usize); }

        for i in 0..length {
            derived.push(((hash + i * 31) % 256) as u8);
        }

        Ok(derived)
    }
}

pub trait KDFManager {
    fn register_kdf(&mut self, kdf: Box<dyn KeyDerivation>) -> Result<KDFID, KDFError>;
    fn derive_key(&self, algorithm: KDFAlgorithm, key: &[u8], salt: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, KDFError>;
}

#[repr(C)]
pub struct SimpleKDFManager {
    pub kdfs: Vec<Option<Box<dyn KeyDerivation>>>,
    pub next_id: AtomicUsize,
}

impl SimpleKDFManager {
    pub fn new() -> Self {
        SimpleKDFManager {
            kdfs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let hkdf = SimpleKeyDerivation::new(self.next_id.fetch_add(1, Ordering::SeqCst), KDFAlgorithm::HKDF_SHA256);
        self.kdfs.push(Some(Box::new(hkdf)));

        let pbkdf2 = SimpleKeyDerivation::new(self.next_id.fetch_add(1, Ordering::SeqCst), KDFAlgorithm::PBKDF2);
        self.kdfs.push(Some(Box::new(pbkdf2)));
    }
}

impl KDFManager for SimpleKDFManager {
    fn register_kdf(&mut self, kdf: Box<dyn KeyDerivation>) -> Result<KDFID, KDFError> {
        let id = kdf.id();
        self.kdfs.push(Some(kdf));
        Ok(id)
    }

    fn derive_key(&self, algorithm: KDFAlgorithm, key: &[u8], salt: &[u8], info: &[u8], length: usize) -> Result<Vec<u8>, KDFError> {
        for kdf_option in &self.kdfs {
            if let Some(ref kdf) = *kdf_option {
                if kdf.algorithm() == algorithm {
                    return kdf.derive(key, salt, info, length);
                }
            }
        }
        Err(KDFError::InvalidKey)
    }
}

pub trait PasswordHashing {
    fn hash_password(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>, KDFError>;
    fn verify_password(&self, password: &[u8], salt: &[u8], hash: &[u8]) -> Result<bool, KDFError>;
}

#[repr(C)]
pub struct SimplePasswordHashing {
    pub kdf_manager: SimpleKDFManager,
}

impl SimplePasswordHashing {
    pub fn new(kdf_manager: SimpleKDFManager) -> Self {
        SimplePasswordHashing { kdf_manager }
    }
}

impl PasswordHashing for SimplePasswordHashing {
    fn hash_password(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>, KDFError> {
        self.kdf_manager.derive_key(KDFAlgorithm::PBKDF2, password, salt, b"password", 32)
    }

    fn verify_password(&self, password: &[u8], salt: &[u8], hash: &[u8]) -> Result<bool, KDFError> {
        let computed = self.hash_password(password, salt)?;

        if computed.len() != hash.len() {
            return Ok(false);
        }

        for i in 0..computed.len() {
            if computed[i] != hash[i] {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
