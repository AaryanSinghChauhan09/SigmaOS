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

use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

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
    ProviderUnavailable = 3,
}

pub trait HashFunction {
    fn algorithm(&self) -> HashAlgorithm;
    fn digest_size(&self) -> usize;
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, HashError>;
}

#[repr(C)]
pub struct SimpleHashFunction {
    pub algo: HashAlgorithm,
}

impl SimpleHashFunction {
    pub fn new(algo: HashAlgorithm) -> Self {
        SimpleHashFunction { algo }
    }
}

impl HashFunction for SimpleHashFunction {
    fn algorithm(&self) -> HashAlgorithm {
        self.algo
    }

    fn digest_size(&self) -> usize {
        match self.algo {
            HashAlgorithm::SHA256 => 32,
            HashAlgorithm::SHA512 => 64,
            HashAlgorithm::SHA3_256 => 32,
            HashAlgorithm::BLAKE3 => 32,
        }
    }

    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, HashError> {
        let size = self.digest_size();
        let mut digest = Vec::with_capacity(size);
        let mut seed: u32 = match self.algo {
            HashAlgorithm::SHA256 => 0x6a09e667,
            HashAlgorithm::SHA512 => 0x6a09e667,
            HashAlgorithm::SHA3_256 => 0x2b0e6d90,
            HashAlgorithm::BLAKE3 => 0x673371d7,
        };

        for &b in data {
            seed = seed.wrapping_add(b as u32).wrapping_mul(31);
        }

        for i in 0..size {
            digest.push(((seed.wrapping_add(i as u32)) % 256) as u8);
        }

        Ok(digest)
    }
}

pub struct SimpleHashManager {
    pub next_id: AtomicUsize,
}

impl SimpleHashManager {
    pub fn new() -> Self {
        SimpleHashManager {
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn compute_hash(&self, algo: HashAlgorithm, data: &[u8]) -> Result<Vec<u8>, HashError> {
        let func = SimpleHashFunction::new(algo);
        func.compute(data)
    }
}

impl Default for SimpleHashManager {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HMAC {
    fn compute_hmac(&self, key: &[u8], data: &[u8], algorithm: HashAlgorithm) -> Result<Vec<u8>, HashError>;
}

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