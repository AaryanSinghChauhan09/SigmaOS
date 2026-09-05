use std::vec::Vec;

use std::boxed::Box;

/// OOP-based Encryption Service for SigmaOS
/// Based on Roadmap Item 15: Encryption service

use core::sync::atomic::AtomicUsize;

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
pub enum CryptoError { Success = 0, KeyNotFound = 1, EncryptionFailed = 2, InvalidKey = 3 }

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
            if let Some(ref key) = key_option {
                if key.id() == key_id {
                    let mut encrypted = Vec::new();
                    let key_bytes = key.key_data();
                    if key_bytes.is_empty() {
                        return Err(CryptoError::InvalidKey);
                    }
                    for (idx, byte) in data.iter().enumerate() {
                        let mask = key_bytes[idx % key_bytes.len()];
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
            if let Some(ref key) = key_option {
                if key.id() == key_id {
                    let mut decrypted = Vec::new();
                    let key_bytes = key.key_data();
                    if key_bytes.is_empty() {
                        return Err(CryptoError::InvalidKey);
                    }
                    for (idx, byte) in data.iter().enumerate() {
                        let mask = key_bytes[idx % key_bytes.len()];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_service_no_dynamic_derived_keys() {
        let mut service = SimpleEncryptionService::new();
        // Use a customized key that is NOT 0x42
        let key_data = b"MY_CUSTOM_SECRET_KEY_FOR_TESTS";
        let key = SimpleEncryptionKey::new(101, CipherType::XOR, key_data);
        service.add_key(Box::new(key)).unwrap();

        let plaintext = b"Hello, World!";
        let ciphertext = service.encrypt(plaintext, 101).unwrap();

        // Ensure it did not use the hardcoded 0x42 constant
        let bad_ciphertext: Vec<u8> = plaintext.iter().map(|&b| b ^ 0x42).collect();
        assert_ne!(ciphertext, bad_ciphertext);

        // Decrypt and verify
        let decrypted = service.decrypt(&ciphertext, 101).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
