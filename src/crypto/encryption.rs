#![no_std]
#![no_main]

/// OOP-based Encryption Service for SigmaOS
/// Based on Roadmap Item 15: Encryption service

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CipherType { AES = 0, ChaCha20 = 1, XOR = 2 }

pub trait EncryptionKey {
    fn id(&self) -> KeyID;
    fn cipher_type(&self) -> CipherType;
}

#[repr(C)]
pub struct SimpleEncryptionKey {
    pub id: KeyID,
    pub cipher_type: CipherType,
    pub key_data: [u8; 32],
}

impl SimpleEncryptionKey {
    pub fn new(id: KeyID, cipher_type: CipherType, key_data: &[u8]) -> Self {
        let mut key_array = [0u8; 32];
        let key_len = key_data.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(key_data.as_ptr(), key_array.as_mut_ptr(), key_len);
        }
        SimpleEncryptionKey { id, cipher_type, key_data: key_array }
    }
}

impl EncryptionKey for SimpleEncryptionKey {
    fn id(&self) -> KeyID { self.id }
    fn cipher_type(&self) -> CipherType { self.cipher_type }
}

pub trait EncryptionService {
    fn encrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError>;
    fn add_key(&mut self, key: Box<dyn EncryptionKey>) -> Result<KeyID, CryptoError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CryptoError { Success = 0, KeyNotFound = 1, EncryptionFailed = 2 }

pub struct SimpleEncryptionService {
    keys: Vec<Option<Box<dyn EncryptionKey>>>,
    next_id: AtomicUsize,
}

impl SimpleEncryptionService {
    pub fn new() -> Self { SimpleEncryptionService { keys: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl EncryptionService for SimpleEncryptionService {
    fn encrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError> {
        for key_option in &self.keys {
            if let Some(ref key) = *key_option {
                if key.id() == key_id {
                    let mut encrypted = Vec::new();
                    for byte in data {
                        encrypted.push(*byte ^ 0x42);
                    }
                    return Ok(encrypted);
                }
            }
        }
        Err(CryptoError::KeyNotFound)
    }
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError> {
        for key_option in &self.keys {
            if let Some(ref key) = *key_option {
                if key.id() == key_id {
                    let mut decrypted = Vec::new();
                    for byte in data {
                        decrypted.push(*byte ^ 0x42);
                    }
                    return Ok(decrypted);
                }
            }
        }
        Err(CryptoError::KeyNotFound)
    }
    fn add_key(&mut self, key: Box<dyn EncryptionKey>) -> Result<KeyID, CryptoError> {
        let id = key.id();
        self.keys.push(Some(key));
        Ok(id)
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
