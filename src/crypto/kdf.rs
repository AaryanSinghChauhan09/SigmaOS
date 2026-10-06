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
use std::vec::Vec;

pub type KDFID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KDFAlgorithm {
    PBKDF2 = 0,
    Argon2id = 1,
    HKDF = 2,
    Scrypt = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KDFError {
    Success = 0,
    InvalidPassword = 1,
    InvalidSalt = 2,
    DerivationFailed = 3,
    CryptoUnavailable = 4,
}

pub trait KeyDerivation {
    fn id(&self) -> KDFID;
    fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
        key_len: usize,
    ) -> Result<Vec<u8>, KDFError>;
}

#[repr(C)]
pub struct SimpleKeyDerivation {
    pub id: KDFID,
    pub algorithm: AtomicUsize,
    pub iterations: u32,
}

impl SimpleKeyDerivation {
    pub fn new(id: KDFID, algorithm: KDFAlgorithm, iterations: u32) -> Self {
        SimpleKeyDerivation {
            id,
            algorithm: AtomicUsize::new(algorithm as usize),
            iterations,
        }
    }
}

impl KeyDerivation for SimpleKeyDerivation {
    fn id(&self) -> KDFID {
        self.id
    }

    fn derive_key(
        &self,
        _password: &[u8],
        _salt: &[u8],
        _key_len: usize,
    ) -> Result<Vec<u8>, KDFError> {
        Err(KDFError::CryptoUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_creation() {
        let kdf = SimpleKeyDerivation::new(1, KDFAlgorithm::Argon2id, 3);
        assert_eq!(kdf.id(), 1);
    }

    #[test]
    fn test_kdf_fails_closed() {
        let kdf = SimpleKeyDerivation::new(1, KDFAlgorithm::PBKDF2, 10000);
        assert!(matches!(
            kdf.derive_key(b"password", b"salt", 32),
            Err(KDFError::CryptoUnavailable)
        ));
    }
}
