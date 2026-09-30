use std::boxed::Box;
use std::vec::Vec;

/// OOP-based Key Derivation Function for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 502
/// Implements HKDF and PBKDF2 key derivation
use core::sync::atomic::{AtomicUsize, Ordering};

pub type KDFID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KDFAlgorithm {
    HKDF_SHA256 = 0,
    HKDF_SHA512 = 1,
    PBKDF2 = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KDFError {
    Success = 0,
    InvalidKey = 1,
    InvalidLength = 2,
    ProviderUnavailable = 3,
}

pub trait KeyDerivation {
    fn id(&self) -> KDFID;
    fn algorithm(&self) -> KDFAlgorithm;
    fn derive(
        &self,
        key: &[u8],
        salt: &[u8],
        info: &[u8],
        length: usize,
    ) -> Result<Vec<u8>, KDFError>;
}

#[repr(C)]
pub struct SimpleKeyDerivation {
    pub id: KDFID,
    algorithm: KDFAlgorithm,
}

impl SimpleKeyDerivation {
    pub fn new(id: KDFID, algorithm: KDFAlgorithm) -> Self {
        SimpleKeyDerivation { id, algorithm }
    }
}

impl KeyDerivation for SimpleKeyDerivation {
    fn id(&self) -> KDFID {
        self.id
    }
    fn algorithm(&self) -> KDFAlgorithm {
        self.algorithm
    }

    fn derive(
        &self,
        _key: &[u8],
        _salt: &[u8],
        _info: &[u8],
        _length: usize,
    ) -> Result<Vec<u8>, KDFError> {
        Err(KDFError::ProviderUnavailable)
    }
}

pub trait KDFManager {
    fn register_kdf(&mut self, kdf: Box<dyn KeyDerivation>) -> Result<KDFID, KDFError>;
    fn derive_key(
        &self,
        algorithm: KDFAlgorithm,
        key: &[u8],
        salt: &[u8],
        info: &[u8],
        length: usize,
    ) -> Result<Vec<u8>, KDFError>;
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
        let hkdf = SimpleKeyDerivation::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            KDFAlgorithm::HKDF_SHA256,
        );
        self.kdfs.push(Some(Box::new(hkdf)));

        let pbkdf2 = SimpleKeyDerivation::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            KDFAlgorithm::PBKDF2,
        );
        self.kdfs.push(Some(Box::new(pbkdf2)));
    }
}

impl KDFManager for SimpleKDFManager {
    fn register_kdf(&mut self, kdf: Box<dyn KeyDerivation>) -> Result<KDFID, KDFError> {
        let id = kdf.id();
        self.kdfs.push(Some(kdf));
        Ok(id)
    }

    fn derive_key(
        &self,
        algorithm: KDFAlgorithm,
        key: &[u8],
        salt: &[u8],
        info: &[u8],
        length: usize,
    ) -> Result<Vec<u8>, KDFError> {
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

#[cfg(test)]
mod fail_closed_tests {
    use super::{
        KDFAlgorithm, KDFError, KeyDerivation, PasswordHashing, SimpleKDFManager,
        SimpleKeyDerivation, SimplePasswordHashing,
    };

    #[test]
    fn kdf_and_password_hash_apis_report_missing_provider() {
        let kdf = SimpleKeyDerivation::new(1, KDFAlgorithm::PBKDF2);
        assert_eq!(
            kdf.derive(b"password", b"salt", b"info", 32),
            Err(KDFError::ProviderUnavailable)
        );

        let mut manager = SimpleKDFManager::new();
        manager.seed_with_defaults();
        let password_hashing = SimplePasswordHashing::new(manager);
        assert_eq!(
            password_hashing.hash_password(b"password", b"salt"),
            Err(KDFError::ProviderUnavailable)
        );
    }
}

impl SimplePasswordHashing {
    pub fn new(kdf_manager: SimpleKDFManager) -> Self {
        SimplePasswordHashing { kdf_manager }
    }
}

impl PasswordHashing for SimplePasswordHashing {
    fn hash_password(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>, KDFError> {
        self.kdf_manager
            .derive_key(KDFAlgorithm::PBKDF2, password, salt, b"password", 32)
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
