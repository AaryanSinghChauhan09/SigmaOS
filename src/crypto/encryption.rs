#![no_std]

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
    fn key_bytes(&self) -> &[u8];
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
    fn key_bytes(&self) -> &[u8] { &self.key_data }
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
            if let Some(key) = key_option {
                let key_ref: &dyn EncryptionKey = &**key;
                if key_ref.id() == key_id {
                    let mut encrypted = Vec::new();
                    let key_bytes = key_ref.key_bytes();
                    let len = key_bytes.len();
                    if len == 0 {
                        return Err(CryptoError::EncryptionFailed);
                    }
                    for (i, byte) in data.iter().enumerate() {
                        encrypted.push(*byte ^ key_bytes[i % len]);
                    }
                    return Ok(encrypted);
                }
            }
        }
        Err(CryptoError::KeyNotFound)
    }
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError> {
        for key_option in &self.keys {
            if let Some(key) = key_option {
                let key_ref: &dyn EncryptionKey = &**key;
                if key_ref.id() == key_id {
                    let mut decrypted = Vec::new();
                    let key_bytes = key_ref.key_bytes();
                    let len = key_bytes.len();
                    if len == 0 {
                        return Err(CryptoError::EncryptionFailed);
                    }
                    for (i, byte) in data.iter().enumerate() {
                        decrypted.push(*byte ^ key_bytes[i % len]);
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

pub use crate::klib_vec::Vec;
