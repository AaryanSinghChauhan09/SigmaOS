use std::vec::Vec;

use std::boxed::Box;

/// Prototype encryption service API for SigmaOS.
/// Based on Roadmap Item 15: Encryption service
/// The former XOR-based transform was not encryption; operations now fail closed.

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
    pub key_len: u8,
}

impl SimpleEncryptionKey {
    pub fn new(id: KeyID, cipher_type: CipherType, key_data: &[u8]) -> Self {
        let mut key_array = [0u8; 32];
        let key_len = key_data.len().min(32);
        key_array[..key_len].copy_from_slice(&key_data[..key_len]);
        SimpleEncryptionKey {
            id,
            cipher_type,
            key_data: key_array,
            key_len: key_len as u8,
        }
    }
}

impl EncryptionKey for SimpleEncryptionKey {
    fn id(&self) -> KeyID { self.id }
    fn cipher_type(&self) -> CipherType { self.cipher_type }
    fn key_data(&self) -> &[u8] {
        // Bolt ⚡ Optimization: Store explicit key length on instantiation to eliminate
        // O(N) zero-byte linear scanning (.position(|&b| b == 0)) on every key slice lookup,
        // reducing key data access to instantaneous O(1) constant time.
        if self.key_len > 0 {
            &self.key_data[..self.key_len.min(32) as usize]
        } else {
            let len = self.key_data.iter().position(|&b| b == 0).unwrap_or(32);
            &self.key_data[..len]
        }
    }
}

pub trait EncryptionService {
    fn encrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError>;
    fn add_key(&mut self, key: Box<dyn EncryptionKey>) -> Result<KeyID, CryptoError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CryptoError {
    Success = 0,
    KeyNotFound = 1,
    EncryptionFailed = 2,
    InvalidKey = 3,
    CryptoUnavailable = 4,
}
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
                    let key_bytes = key.key_data();
                    if key_bytes.is_empty() {
                        return Err(CryptoError::InvalidKey);
                    }
                    let _ = data;
                    return Err(CryptoError::CryptoUnavailable);
                }
            }
        }
        Err(CryptoError::KeyNotFound)
    }
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError> {
        for key_option in &self.keys {
            if let Some(ref key) = key_option {
                if key.id() == key_id {
                    let key_bytes = key.key_data();
                    if key_bytes.is_empty() {
                        return Err(CryptoError::InvalidKey);
                    }
                    let _ = data;
                    return Err(CryptoError::CryptoUnavailable);
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
    fn encryption_service_fails_closed_without_a_crypto_provider() {
        let mut service = SimpleEncryptionService::new();
        // Use a customized key that is NOT 0x42
        let key_data = b"MY_CUSTOM_SECRET_KEY_FOR_TESTS";
        let key = SimpleEncryptionKey::new(101, CipherType::XOR, key_data);
        service.add_key(Box::new(key)).unwrap();

        let plaintext = b"Hello, World!";
        assert!(matches!(
            service.encrypt(plaintext, 101),
            Err(CryptoError::CryptoUnavailable)
        ));
        assert!(matches!(
            service.decrypt(plaintext, 101),
            Err(CryptoError::CryptoUnavailable)
        ));
    }
}
