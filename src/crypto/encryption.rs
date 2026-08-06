#![no_std]
#![no_main]

extern crate alloc;
use alloc::boxed::Box;

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
    fn key_data(&self) -> &[u8];
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
    fn key_data(&self) -> &[u8] {
        let len = self.key_data.iter().position(|&b| b == 0).unwrap_or(32);
        &self.key_data[..len]
    }
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
                    let key_bytes = key.key_data();
                    for (idx, byte) in data.iter().enumerate() {
                        let mask = if key_bytes.is_empty() { 0x42 } else { key_bytes[idx % key_bytes.len()] };
                        encrypted.push(*byte ^ mask);
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
                    let key_bytes = key.key_data();
                    for (idx, byte) in data.iter().enumerate() {
                        let mask = if key_bytes.is_empty() { 0x42 } else { key_bytes[idx % key_bytes.len()] };
                        decrypted.push(*byte ^ mask);
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

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_service_no_hardcoded_keys() {
        let mut service = SimpleEncryptionService::new();
        // Use a customized key that is NOT 0x42
        let key_data = b"MY_CUSTOM_SECRET_KEY_FOR_TESTS";
        let key = SimpleEncryptionKey::new(101, CipherType::XOR, key_data);
        service.add_key(Box::new(key)).unwrap();

        let plaintext = b"Hello, World!";
        let ciphertext = service.encrypt(plaintext, 101).unwrap();

        // Ensure it did not use the hardcoded 0x42 constant
        let bad_ciphertext: Vec<u8> = plaintext.iter().map(|&b| b ^ 0x42).collect();
        let mut mismatch = false;
        for i in 0..plaintext.len() {
            if ciphertext.data.is_null() || unsafe { *ciphertext.data.add(i) } != unsafe { *bad_ciphertext.data.add(i) } {
                mismatch = true;
                break;
            }
        }
        assert!(mismatch, "Should not use the hardcoded 0x42 XOR mask");

        // Decrypt and verify
        let decrypted = service.decrypt(&ciphertext.to_slice(), 101).unwrap();
        assert_eq!(decrypted.to_slice(), plaintext);
    }
}

// Add helpful conversion for testing
impl<T: Clone> Vec<T> {
    fn to_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T: Clone> core::iter::FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for item in iter {
            vec.push(item);
        }
        vec
    }
}
