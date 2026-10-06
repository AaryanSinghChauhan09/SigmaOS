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

pub type HashID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    SHA256 = 0,
    SHA512 = 1,
    SHA3_256 = 2,
    BLAKE3 = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashError {
    Success = 0,
    InvalidInput = 1,
    HashFailed = 2,
    CryptoUnavailable = 3,
}

pub trait HashFunction {
    fn id(&self) -> HashID;
    fn digest_size(&self) -> usize;
    fn block_size(&self) -> usize;
    fn hash(&self, data: &[u8]) -> Result<Vec<u8>, HashError>;
}

#[repr(C)]
pub struct SimpleHashFunction {
    pub id: HashID,
    pub algorithm: AtomicUsize,
}

impl SimpleHashFunction {
    pub fn new(id: HashID, algorithm: HashAlgorithm) -> Self {
        SimpleHashFunction {
            id,
            algorithm: AtomicUsize::new(algorithm as usize),
        }
    }
}

impl HashFunction for SimpleHashFunction {
    fn id(&self) -> HashID {
        self.id
    }

    fn digest_size(&self) -> usize {
        match self.algorithm.load(Ordering::SeqCst) {
            0 => 32, // SHA256
            1 => 64, // SHA512
            2 => 32, // SHA3_256
            3 => 32, // BLAKE3
            _ => 32,
        }
    }

    fn block_size(&self) -> usize {
        match self.algorithm.load(Ordering::SeqCst) {
            0 => 64,  // SHA256
            1 => 128, // SHA512
            2 => 136, // SHA3_256
            3 => 64,  // BLAKE3
            _ => 64,
        }
    }

    fn hash(&self, _data: &[u8]) -> Result<Vec<u8>, HashError> {
        Err(HashError::CryptoUnavailable)
    }
}

pub trait HMAC {
    fn compute_hmac(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, HashError>;
    fn verify_hmac(&self, key: &[u8], data: &[u8], mac: &[u8]) -> Result<bool, HashError>;
}

pub struct SimpleHMAC {
    pub hash_fn: SimpleHashFunction,
}

impl SimpleHMAC {
    pub fn new(hash_fn: SimpleHashFunction) -> Self {
        SimpleHMAC { hash_fn }
    }
}

impl HMAC for SimpleHMAC {
    fn compute_hmac(&self, _key: &[u8], _data: &[u8]) -> Result<Vec<u8>, HashError> {
        Err(HashError::CryptoUnavailable)
    }

    fn verify_hmac(&self, key: &[u8], data: &[u8], mac: &[u8]) -> Result<bool, HashError> {
        let computed = self.compute_hmac(key, data)?;
        Ok(computed == mac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_creation() {
        let hash_fn = SimpleHashFunction::new(1, HashAlgorithm::SHA256);
        assert_eq!(hash_fn.digest_size(), 32);
        assert_eq!(hash_fn.block_size(), 64);
    }

    #[test]
    fn test_hash_fails_closed() {
        let hash_fn = SimpleHashFunction::new(1, HashAlgorithm::SHA256);
        assert!(matches!(
            hash_fn.hash(b"test"),
            Err(HashError::CryptoUnavailable)
        ));
    }
}
