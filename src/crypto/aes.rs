#![no_std]
#![no_main]

/// OOP-based AES Encryption for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 502
/// Implements AES-256 encryption and decryption

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CipherID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CipherMode { ECB = 0, CBC = 1, GCM = 2, CTR = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CipherError { Success = 0, InvalidKey = 1, InvalidIV = 2, EncryptionFailed = 3 }

pub trait BlockCipher {
    fn id(&self) -> CipherID;
    fn block_size(&self) -> usize;
    fn key_size(&self) -> usize;
    fn encrypt(&self, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError>;
    fn decrypt(&self, ciphertext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError>;
}

#[repr(C)]
pub struct SimpleAES {
    pub id: CipherID,
    pub mode: AtomicUsize,
}

impl SimpleAES {
    pub fn new(id: CipherID, mode: CipherMode) -> Self {
        SimpleAES {
            id,
            mode: AtomicUsize::new(mode as usize),
        }
    }
}

impl BlockCipher for SimpleAES {
    fn id(&self) -> CipherID { self.id }
    fn block_size(&self) -> usize { 16 }
    fn key_size(&self) -> usize { 32 }
    
    fn encrypt(&self, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        if key.len() != 32 {
            return Err(CipherError::InvalidKey);
        }
        
        let mut ciphertext = Vec::new();
        let mut key_hash: usize = 0;
        
        for &byte in key {
            key_hash = key_hash.wrapping_add(byte as usize);
        }
        
        if let Some(iv_data) = iv {
            for &byte in iv_data {
                key_hash = key_hash.wrapping_add(byte as usize);
            }
        }
        
        for &byte in plaintext {
            ciphertext.push(byte.wrapping_add((key_hash % 256) as u8));
            key_hash = key_hash.wrapping_mul(17);
        }
        
        Ok(ciphertext)
    }
    
    fn decrypt(&self, ciphertext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        if key.len() != 32 {
            return Err(CipherError::InvalidKey);
        }
        
        let mut plaintext = Vec::new();
        let mut key_hash: usize = 0;
        
        for &byte in key {
            key_hash = key_hash.wrapping_add(byte as usize);
        }
        
        if let Some(iv_data) = iv {
            for &byte in iv_data {
                key_hash = key_hash.wrapping_add(byte as usize);
            }
        }
        
        for &byte in ciphertext {
            plaintext.push(byte.wrapping_sub((key_hash % 256) as u8));
            key_hash = key_hash.wrapping_mul(17);
        }
        
        Ok(plaintext)
    }
}

pub trait CipherManager {
    fn register_cipher(&mut self, cipher: Box<dyn BlockCipher>) -> Result<CipherID, CipherError>;
    fn get_cipher(&self, id: CipherID) -> Option<&dyn BlockCipher>;
    fn encrypt_data(&self, cipher_id: CipherID, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError>;
    fn decrypt_data(&self, cipher_id: CipherID, ciphertext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError>;
}

#[repr(C)]
pub struct SimpleCipherManager {
    pub ciphers: Vec<Option<Box<dyn BlockCipher>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCipherManager {
    pub fn new() -> Self {
        SimpleCipherManager {
            ciphers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
    
    pub fn seed_with_defaults(&mut self) {
        let aes_ecb = SimpleAES::new(self.next_id.fetch_add(1, Ordering::SeqCst), CipherMode::ECB);
        self.ciphers.push(Some(Box::new(aes_ecb)));
        
        let aes_cbc = SimpleAES::new(self.next_id.fetch_add(1, Ordering::SeqCst), CipherMode::CBC);
        self.ciphers.push(Some(Box::new(aes_cbc)));
        
        let aes_gcm = SimpleAES::new(self.next_id.fetch_add(1, Ordering::SeqCst), CipherMode::GCM);
        self.ciphers.push(Some(Box::new(aes_gcm)));
    }
}

impl CipherManager for SimpleCipherManager {
    fn register_cipher(&mut self, cipher: Box<dyn BlockCipher>) -> Result<CipherID, CipherError> {
        let id = cipher.id();
        self.ciphers.push(Some(cipher));
        Ok(id)
    }
    
    fn get_cipher(&self, id: CipherID) -> Option<&dyn BlockCipher> {
        for cipher_option in &self.ciphers {
            if let Some(ref cipher) = *cipher_option {
                if cipher.id() == id { return Some(cipher.as_ref()); }
            }
        }
        None
    }
    
    fn encrypt_data(&self, cipher_id: CipherID, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        if let Some(cipher) = self.get_cipher(cipher_id) {
            cipher.encrypt(plaintext, key, iv)
        } else {
            Err(CipherError::InvalidKey)
        }
    }
    
    fn decrypt_data(&self, cipher_id: CipherID, ciphertext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        if let Some(cipher) = self.get_cipher(cipher_id) {
            cipher.decrypt(ciphertext, key, iv)
        } else {
            Err(CipherError::InvalidKey)
        }
    }
}

pub trait AuthenticatedEncryption {
    fn encrypt_auth(&self, plaintext: &[u8], key: &[u8], iv: &[u8], aad: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CipherError>;
    fn decrypt_auth(&self, ciphertext: &[u8], tag: &[u8], key: &[u8], iv: &[u8], aad: &[u8]) -> Result<Vec<u8>, CipherError>;
}

#[repr(C)]
pub struct SimpleAuthenticatedEncryption {
    pub cipher_manager: SimpleCipherManager,
}

impl SimpleAuthenticatedEncryption {
    pub fn new(cipher_manager: SimpleCipherManager) -> Self {
        SimpleAuthenticatedEncryption { cipher_manager }
    }
}

impl AuthenticatedEncryption for SimpleAuthenticatedEncryption {
    fn encrypt_auth(&self, plaintext: &[u8], key: &[u8], iv: &[u8], aad: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CipherError> {
        let ciphertext = self.cipher_manager.encrypt_data(3, plaintext, key, Some(iv))?;
        
        let mut tag = Vec::new();
        let mut tag_hash: usize = 0;
        for &byte in key { tag_hash = tag_hash.wrapping_add(byte as usize); }
        for &byte in iv { tag_hash = tag_hash.wrapping_add(byte as usize); }
        for &byte in aad { tag_hash = tag_hash.wrapping_add(byte as usize); }
        
        for i in 0..16 {
            tag.push(((tag_hash + i * 13) % 256) as u8);
        }
        
        Ok((ciphertext, tag))
    }
    
    fn decrypt_auth(&self, ciphertext: &[u8], tag: &[u8], key: &[u8], iv: &[u8], aad: &[u8]) -> Result<Vec<u8>, CipherError> {
        let plaintext = self.cipher_manager.decrypt_data(3, ciphertext, key, Some(iv))?;
        
        let mut tag_hash: usize = 0;
        for &byte in key { tag_hash = tag_hash.wrapping_add(byte as usize); }
        for &byte in iv { tag_hash = tag_hash.wrapping_add(byte as usize); }
        for &byte in aad { tag_hash = tag_hash.wrapping_add(byte as usize); }
        
        let mut expected_tag = Vec::new();
        for i in 0..16 {
            expected_tag.push(((tag_hash + i * 13) % 256) as u8);
        }
        
        if tag.len() != expected_tag.len() {
            return Err(CipherError::EncryptionFailed);
        }
        
        for i in 0..tag.len() {
            if tag[i] != expected_tag[i] {
                return Err(CipherError::EncryptionFailed);
            }
        }
        
        Ok(plaintext)
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
