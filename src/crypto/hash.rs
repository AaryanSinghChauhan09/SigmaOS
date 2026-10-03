use std::boxed::Box;
use std::vec::Vec;
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
    pub algorithm: AtomicUsize,
    pub state: [u8; 64],
    pub buffer: Vec<u8>,
    pub multiplier: AtomicUsize,
    pub offset_factor: AtomicUsize,
}

impl SimpleHashFunction {
    pub fn new(id: HashID, algorithm: HashAlgorithm) -> Self {
        SimpleHashFunction {
            id,
            algorithm: AtomicUsize::new(algorithm as usize),
            state: [0u8; 64],
            buffer: Vec::new(),
            multiplier: AtomicUsize::new(31),
            offset_factor: AtomicUsize::new(17),
        }
    }

    pub fn with_salt_params(self, mult: usize, offset: usize) -> Self {
        self.multiplier.store(mult, Ordering::SeqCst);
        self.offset_factor.store(offset, Ordering::SeqCst);
        self
    }
}

impl HashFunction for SimpleHashFunction {
    fn id(&self) -> HashID {
        self.id
    }

    fn algorithm(&self) -> HashAlgorithm {
        let raw = self.algorithm.load(Ordering::SeqCst);
        match raw {
            1 => HashAlgorithm::SHA3_256,
            2 => HashAlgorithm::BLAKE3,
            _ => HashAlgorithm::SHA256,
        }
    }

    fn hash_size(&self) -> usize {
        32
    }

    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, HashError> {
        let mut hash = Vec::new();
        let mut digest: usize = 0;
        let mult = self.multiplier.load(Ordering::SeqCst);
        let offset = self.offset_factor.load(Ordering::SeqCst);

        for &byte in data {
            digest = digest.wrapping_add(byte as usize);
            digest = digest.wrapping_mul(mult);
        }

        for i in 0..32 {
            hash.push(((digest + i * offset) % 256) as u8);
        }

        Ok(hash)
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
        let sha256 = SimpleHashFunction::new(self.next_id.fetch_add(1, Ordering::SeqCst), HashAlgorithm::SHA256);
        self.hashes.push(Some(Box::new(sha256)));

        let sha3 = SimpleHashFunction::new(self.next_id.fetch_add(1, Ordering::SeqCst), HashAlgorithm::SHA3_256);
        self.hashes.push(Some(Box::new(sha3)));

        let blake3 = SimpleHashFunction::new(self.next_id.fetch_add(1, Ordering::SeqCst), HashAlgorithm::BLAKE3);
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
    fn compute_hmac(&self, key: &[u8], data: &[u8], algorithm: HashAlgorithm) -> Result<Vec<u8>, HashError>;
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
    fn compute_hmac(&self, key: &[u8], data: &[u8], algorithm: HashAlgorithm) -> Result<Vec<u8>, HashError> {
        let mut combined = Vec::new();
        for &byte in key {
            combined.push(byte);
        }
        for &byte in data {
            combined.push(byte);
        }

        self.hash_manager.compute_hash(algorithm, &combined)
    }
}

pub trait HashVerification {
    fn verify_hash(&self, data: &[u8], expected: &[u8], algorithm: HashAlgorithm) -> Result<bool, HashError>;
    fn verify_file_integrity(&self, file_data: &[u8], signature: &[u8]) -> Result<bool, HashError>;
}

#[repr(C)]
pub struct SimpleHashVerification {
    pub hash_manager: SimpleHashManager,
}

impl SimpleHashVerification {
    pub fn new(hash_manager: SimpleHashManager) -> Self {
        SimpleHashVerification { hash_manager }
    }
}

impl HashVerification for SimpleHashVerification {
    fn verify_hash(&self, data: &[u8], expected: &[u8], algorithm: HashAlgorithm) -> Result<bool, HashError> {
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
