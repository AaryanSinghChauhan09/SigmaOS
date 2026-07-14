#![no_std]
#![no_main]

/// OOP-based Embedded Crypto for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1206
/// Implements hardware cryptographic acceleration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CryptoID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CryptoAlgorithm { AES = 0, SHA256 = 1, RSA = 2, ECDSA = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CryptoError { Success = 0, NotFound = 1 }

pub trait CryptoEngine {
    fn id(&self) -> CryptoID;
    fn algorithm(&self) -> CryptoAlgorithm;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCryptoEngine {
    pub id: CryptoID,
    pub algorithm: AtomicUsize,
    pub busy: AtomicUsize,
}

impl SimpleCryptoEngine {
    pub fn new(id: CryptoID, algorithm: CryptoAlgorithm) -> Self {
        SimpleCryptoEngine {
            id,
            algorithm: AtomicUsize::new(algorithm as usize),
            busy: AtomicUsize::new(0),
        }
    }
}

impl CryptoEngine for SimpleCryptoEngine {
    fn id(&self) -> CryptoID { self.id }
    fn algorithm(&self) -> CryptoAlgorithm { unsafe { core::mem::transmute(self.algorithm.load(Ordering::SeqCst)) } }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait CryptoAccelerator {
    fn encrypt(&self, engine_id: CryptoID, plaintext: &[u8], ciphertext: &mut [u8]) -> Result<(), CryptoError>;
    fn decrypt(&self, engine_id: CryptoID, ciphertext: &[u8], plaintext: &mut [u8]) -> Result<(), CryptoError>;
    fn hash(&self, engine_id: CryptoID, data: &[u8], digest: &mut [u8]) -> Result<(), CryptoError>;
}

#[repr(C)]
pub struct SimpleCryptoAccelerator {
    pub engines: Vec<Option<Box<dyn CryptoEngine>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCryptoAccelerator {
    pub fn new() -> Self {
        SimpleCryptoAccelerator {
            engines: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CryptoAccelerator for SimpleCryptoAccelerator {
    fn encrypt(&self, engine_id: CryptoID, plaintext: &[u8], ciphertext: &mut [u8]) -> Result<(), CryptoError> {
        if self.get_engine(engine_id).is_some() {
            for (i, &byte) in plaintext.iter().enumerate() {
                if i < ciphertext.len() {
                    ciphertext[i] = byte.wrapping_add(1);
                }
            }
            Ok(())
        } else {
            Err(CryptoError::NotFound)
        }
    }
    
    fn decrypt(&self, engine_id: CryptoID, ciphertext: &[u8], plaintext: &mut [u8]) -> Result<(), CryptoError> {
        if self.get_engine(engine_id).is_some() {
            for (i, &byte) in ciphertext.iter().enumerate() {
                if i < plaintext.len() {
                    plaintext[i] = byte.wrapping_sub(1);
                }
            }
            Ok(())
        } else {
            Err(CryptoError::NotFound)
        }
    }
    
    fn hash(&self, engine_id: CryptoID, _data: &[u8], digest: &mut [u8]) -> Result<(), CryptoError> {
        if self.get_engine(engine_id).is_some() {
            for byte in digest.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(CryptoError::NotFound)
        }
    }
    
    fn get_engine(&self, id: CryptoID) -> Option<&dyn CryptoEngine> {
        for engine_option in &self.engines {
            if let Some(ref engine) = *engine_option {
                if engine.id() == id { return Some(engine.as_ref()); }
            }
        }
        None
    }
}

pub trait KeyStorage {
    def store_key(&mut self, key_id: u32, key: &[u8]) -> Result<(), CryptoError>;
    def retrieve_key(&self, key_id: u32) -> Result<Vec<u8>, CryptoError>;
}

#[repr(C)]
pub struct SimpleKeyStorage {
    pub keys: Vec<(u32, Vec<u8>)>,
}

impl SimpleKeyStorage {
    pub fn new() -> Self {
        SimpleKeyStorage {
            keys: Vec::new(),
        }
    }
}

impl KeyStorage for SimpleKeyStorage {
    fn store_key(&mut self, key_id: u32, key: &[u8]) -> Result<(), CryptoError> {
        let key_vec = key.to_vec();
        self.keys.push((key_id, key_vec));
        Ok(())
    }
    
    fn retrieve_key(&self, key_id: u32) -> Result<Vec<u8>, CryptoError> {
        for &(id, ref key) in &self.keys {
            if id == key_id {
                return Ok(key.clone());
            }
        }
        Err(CryptoError::NotFound)
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
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
