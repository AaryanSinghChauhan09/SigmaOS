use std::boxed::Box;
use std::vec::Vec;

/// OOP-based Cryptographic Hash Functions for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 502
/// Implements SHA-256, SHA-3, and BLAKE3 hash functions
use core::sync::atomic::{AtomicUsize, Ordering};

pub type HashID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    SHA256 = 0,
    SHA3_256 = 1,
    BLAKE3 = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashError {
    Success = 0,
    InvalidInput = 1,
    AlgorithmNotSupported = 2,
    ProviderUnavailable = 3,
}

pub trait HashFunction {
    fn id(&self) -> HashID;
    fn algorithm(&self) -> HashAlgorithm;
    fn hash_size(&self) -> usize;
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, HashError>;
    fn compute_update(&mut self, chunk: &[u8]) -> Result<(), HashError>;
    fn finalize(&mut self) -> Result<Vec<u8>, HashError>;
}

#[repr(C)]
pub struct SimpleHashFunction {
    pub id: HashID,
    algorithm: HashAlgorithm,
    buffer: Vec<u8>,
}

impl SimpleHashFunction {
    pub fn new(id: HashID, algorithm: HashAlgorithm) -> Self {
        SimpleHashFunction {
            id,
            algorithm,
            buffer: Vec::new(),
        }
    }
}

impl HashFunction for SimpleHashFunction {
    fn id(&self) -> HashID {
        self.id
    }
    fn algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }
    fn hash_size(&self) -> usize {
        32
    }

    fn compute(&self, _data: &[u8]) -> Result<Vec<u8>, HashError> {
        Err(HashError::ProviderUnavailable)
    }

    fn compute_update(&mut self, chunk: &[u8]) -> Result<(), HashError> {
        for &byte in chunk {
            self.buffer.push(byte);
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<Vec<u8>, HashError> {
        self.compute(&self.buffer)
    }
}

pub trait HashManager {
    fn register_hash(&mut self, hash: Box<dyn HashFunction>) -> Result<HashID, HashError>;
    fn get_hash(&self, id: HashID) -> Option<&dyn HashFunction>;
    fn compute_hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Result<Vec<u8>, HashError>;
}

#[repr(C)]
pub struct SimpleHashManager {
    pub hashes: Vec<Option<Box<dyn HashFunction>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHashManager {
    pub fn new() -> Self {
        SimpleHashManager {
            hashes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let sha256 = SimpleHashFunction::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            HashAlgorithm::SHA256,
        );
        self.hashes.push(Some(Box::new(sha256)));

        let sha3 = SimpleHashFunction::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            HashAlgorithm::SHA3_256,
        );
        self.hashes.push(Some(Box::new(sha3)));

        let blake3 = SimpleHashFunction::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            HashAlgorithm::BLAKE3,
        );
        self.hashes.push(Some(Box::new(blake3)));
    }
}

impl HashManager for SimpleHashManager {
    fn register_hash(&mut self, hash: Box<dyn HashFunction>) -> Result<HashID, HashError> {
        let id = hash.id();
        self.hashes.push(Some(hash));
        Ok(id)
    }

    fn get_hash(&self, id: HashID) -> Option<&dyn HashFunction> {
        for hash_option in &self.hashes {
            if let Some(ref hash) = *hash_option {
                if hash.id() == id {
                    return Some(hash.as_ref());
                }
            }
        }
        None
    }

    fn compute_hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Result<Vec<u8>, HashError> {
        for hash_option in &self.hashes {
            if let Some(ref hash) = *hash_option {
                if hash.algorithm() == algorithm {
                    return hash.compute(data);
                }
            }
        }
        Err(HashError::AlgorithmNotSupported)
    }
}

pub trait HMAC {
    fn compute_hmac(
        &self,
        key: &[u8],
        data: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<Vec<u8>, HashError>;
}

#[repr(C)]
pub struct SimpleHMAC {
    pub hash_manager: SimpleHashManager,
}

impl SimpleHMAC {
    pub fn new(hash_manager: SimpleHashManager) -> Self {
        SimpleHMAC { hash_manager }
    }
}

impl HMAC for SimpleHMAC {
    fn compute_hmac(
        &self,
        _key: &[u8],
        _data: &[u8],
        _algorithm: HashAlgorithm,
    ) -> Result<Vec<u8>, HashError> {
        Err(HashError::ProviderUnavailable)
    }
}

pub trait HashVerification {
    fn verify_hash(
        &self,
        data: &[u8],
        expected: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<bool, HashError>;
    fn verify_file_integrity(&self, file_data: &[u8], signature: &[u8]) -> Result<bool, HashError>;
}

#[repr(C)]
pub struct SimpleHashVerification {
    pub hash_manager: SimpleHashManager,
}

#[cfg(test)]
mod fail_closed_tests {
    use super::{
        HashAlgorithm, HashError, HashFunction, SimpleHMAC, SimpleHashFunction, SimpleHashManager,
    };

    #[test]
    fn hash_and_mac_apis_report_missing_provider() {
        let hash = SimpleHashFunction::new(1, HashAlgorithm::SHA256);
        assert_eq!(hash.compute(b"data"), Err(HashError::ProviderUnavailable));

        let mut manager = SimpleHashManager::new();
        manager.seed_with_defaults();
        let hmac = SimpleHMAC::new(manager);
        assert_eq!(
            super::HMAC::compute_hmac(&hmac, b"key", b"data", HashAlgorithm::SHA256),
            Err(HashError::ProviderUnavailable)
        );
    }
}

impl SimpleHashVerification {
    pub fn new(hash_manager: SimpleHashManager) -> Self {
        SimpleHashVerification { hash_manager }
    }
}

impl HashVerification for SimpleHashVerification {
    fn verify_hash(
        &self,
        data: &[u8],
        expected: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<bool, HashError> {
        let computed = self.hash_manager.compute_hash(algorithm, data)?;

        if computed.len() != expected.len() {
            return Ok(false);
        }

        for i in 0..computed.len() {
            if computed[i] != expected[i] {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn verify_file_integrity(&self, file_data: &[u8], signature: &[u8]) -> Result<bool, HashError> {
        self.verify_hash(file_data, signature, HashAlgorithm::SHA256)
    }
}
