#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use core::sync::atomic::{AtomicUsize, Ordering};
use std::boxed::Box;
use std::vec::Vec;

pub type CipherID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherMode {
    ECB = 0,
    CBC = 1,
    GCM = 2,
    CTR = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherError {
    Success = 0,
    InvalidKey = 1,
    InvalidIV = 2,
    EncryptionFailed = 3,
    CryptoUnavailable = 4,
}

pub trait BlockCipher {
    fn id(&self) -> CipherID;
    fn block_size(&self) -> usize;
    fn key_size(&self) -> usize;
    fn encrypt(
        &self,
        plaintext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError>;
    fn decrypt(
        &self,
        ciphertext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError>;
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
    fn id(&self) -> CipherID {
        self.id
    }
    fn block_size(&self) -> usize {
        16
    }
    fn key_size(&self) -> usize {
        32
    }

    fn encrypt(
        &self,
        plaintext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError> {
        if key.len() != 32 {
            return Err(CipherError::InvalidKey);
        }
        let _ = (plaintext, iv);
        Err(CipherError::CryptoUnavailable)
    }

    fn decrypt(
        &self,
        ciphertext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError> {
        if key.len() != 32 {
            return Err(CipherError::InvalidKey);
        }
        let _ = (ciphertext, iv);
        Err(CipherError::CryptoUnavailable)
    }
}

pub trait CipherManager {
    fn register_cipher(&mut self, cipher: Box<dyn BlockCipher>) -> Result<CipherID, CipherError>;
    fn get_cipher(&self, id: CipherID) -> Option<&dyn BlockCipher>;
    fn encrypt_data(
        &self,
        cipher_id: CipherID,
        plaintext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError>;
    fn decrypt_data(
        &self,
        cipher_id: CipherID,
        ciphertext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError>;
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

impl Default for SimpleCipherManager {
    fn default() -> Self {
        Self::new()
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
                if cipher.id() == id {
                    return Some(cipher.as_ref());
                }
            }
        }
        None
    }

    fn encrypt_data(
        &self,
        cipher_id: CipherID,
        plaintext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError> {
        if let Some(cipher) = self.get_cipher(cipher_id) {
            cipher.encrypt(plaintext, key, iv)
        } else {
            Err(CipherError::InvalidKey)
        }
    }

    fn decrypt_data(
        &self,
        cipher_id: CipherID,
        ciphertext: &[u8],
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Vec<u8>, CipherError> {
        if let Some(cipher) = self.get_cipher(cipher_id) {
            cipher.decrypt(ciphertext, key, iv)
        } else {
            Err(CipherError::InvalidKey)
        }
    }
}

pub trait AuthenticatedEncryption {
    fn encrypt_auth(
        &self,
        plaintext: &[u8],
        key: &[u8],
        iv: &[u8],
        aad: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), CipherError>;
    fn decrypt_auth(
        &self,
        ciphertext: &[u8],
        tag: &[u8],
        key: &[u8],
        iv: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, CipherError>;
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
    fn encrypt_auth(
        &self,
        plaintext: &[u8],
        key: &[u8],
        iv: &[u8],
        _aad: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), CipherError> {
        let ciphertext = self.cipher_manager.encrypt_data(3, plaintext, key, Some(iv))?;

        let tag = vec![0u8; 16];
        Ok((ciphertext, tag))
    }

    fn decrypt_auth(
        &self,
        ciphertext: &[u8],
        _tag: &[u8],
        key: &[u8],
        iv: &[u8],
        _aad: &[u8],
    ) -> Result<Vec<u8>, CipherError> {
        self.cipher_manager.decrypt_data(3, ciphertext, key, Some(iv))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes_compatibility_api_fails_closed_without_a_provider() {
        let cipher = SimpleAES::new(1, CipherMode::GCM);
        assert!(matches!(
            cipher.encrypt(b"secret", &[7; 32], Some(&[9; 12])),
            Err(CipherError::CryptoUnavailable)
        ));
        assert!(matches!(
            cipher.decrypt(b"ciphertext", &[7; 32], Some(&[9; 12])),
            Err(CipherError::CryptoUnavailable)
        ));
    }

    #[test]
    fn authenticated_encryption_fails_closed_without_a_provider() {
        let mut manager = SimpleCipherManager::new();
        manager.seed_with_defaults();
        let aead = SimpleAuthenticatedEncryption::new(manager);

        assert!(matches!(
            aead.encrypt_auth(b"secret", &[7; 32], &[9; 12], b"context"),
            Err(CipherError::CryptoUnavailable)
        ));
    }
}
