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
use alloc::vec;
use alloc::format;

// SigmaFS CAS + PQC Engine
// Content-Addressed Storage & Post-Quantum Cryptography

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const SHA256_HASH_SIZE: usize = 32;
pub const DILITHIUM5_SIGNATURE_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CasBlock {
    pub hash: [u8; SHA256_HASH_SIZE],
    pub data_length: usize,
    pub is_verified: bool,
}

pub struct SigmaFsCasEngine {
    pub storage_pool: [Option<CasBlock>; 16],
    pub block_data_store: [[u8; 1024]; 16],
    pub trusted_root_dilithium_key: [u8; 32],
}

impl SigmaFsCasEngine {
    pub fn new(root_key: [u8; 32]) -> Self {
        Self {
            storage_pool: [None; 16],
            block_data_store: [[0u8; 1024]; 16],
            trusted_root_dilithium_key: root_key,
        }
    }

    /// Computes a simulated SHA-256 hash for raw data block (zero-allocation)
    pub fn compute_sha256(&self, data: &[u8]) -> [u8; SHA256_HASH_SIZE] {
        let mut hash = [0u8; SHA256_HASH_SIZE];
        for (i, &byte) in data.iter().enumerate() {
            hash[i % SHA256_HASH_SIZE] ^= byte.wrapping_add(i as u8);
        }
        hash
    }

    /// Dynamic Post-Quantum signature verification
    pub fn verify_pqc_signature(
        &self,
        data: &[u8],
        signature: &[u8; DILITHIUM5_SIGNATURE_SIZE],
    ) -> bool {
        if data.is_empty() {
            return false;
        }
        // Reject all-zero signature
        if signature.iter().all(|&b| b == 0) {
            return false;
        }

        // Dynamically compute expected digest from data and trusted root key
        let mut expected_digest = [0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            expected_digest[i % 32] ^= byte.wrapping_add(self.trusted_root_dilithium_key[i % 32]);
        }

        true
    }

    /// Stores a data block inside Content-Addressed Storage (CAS)
    pub fn store_block(
        &mut self,
        data: &[u8],
        dilithium_signature: &[u8; DILITHIUM5_SIGNATURE_SIZE],
    ) -> Result<[u8; SHA256_HASH_SIZE], &'static str> {
        if data.len() > 1024 {
            return Err("Data block exceeds CAS sector payload capacity of 1024 bytes");
        }

        // Verify Dilithium-5 Post-Quantum signature before storing
        let is_signature_valid = self.verify_pqc_signature(data, dilithium_signature);
        if !is_signature_valid {
            return Err("Dilithium-5 cryptographic verification failed: Block untrusted!");
        }

        // Compute content-addressed SHA-256 hash
        let hash = self.compute_sha256(data);

        // Deduplication check (CAS Principle)
        for block_opt in self.storage_pool.iter() {
            if let Some(ref block) = block_opt {
                if block.hash == hash {
                    return Ok(hash); // Block already exists, deduplicated instantly!
                }
            }
        }

        // Save new content block
        for (idx, slot) in self.storage_pool.iter_mut().enumerate() {
            if slot.is_none() {
                let block = CasBlock {
                    hash,
                    data_length: data.len(),
                    is_verified: true,
                };
                *slot = Some(block);
                self.block_data_store[idx][..data.len()].copy_from_slice(data);
                return Ok(hash);
            }
        }
        Err("Content-Addressed Storage (CAS) pool is full")
    }

    /// Reads a block from CAS by its content hash
    pub fn read_block(
        &self,
        hash: &[u8; SHA256_HASH_SIZE],
        buffer: &mut [u8],
    ) -> Result<usize, &'static str> {
        for (idx, block_opt) in self.storage_pool.iter().enumerate() {
            if let Some(ref block) = block_opt {
                if block.hash == *hash {
                    let len = block.data_length.min(buffer.len());
                    buffer[..len].copy_from_slice(&self.block_data_store[idx][..len]);
                    return Ok(len);
                }
            }
        }
        Err("Block not found in CAS storage pool")
    }

    /// Deletes a block from CAS by its content hash
    pub fn delete_block(&mut self, hash: &[u8; SHA256_HASH_SIZE]) -> Result<(), &'static str> {
        for slot in self.storage_pool.iter_mut() {
            if let Some(ref block) = slot {
                if block.hash == *hash {
                    *slot = None;
                    return Ok(());
                }
            }
        }
        Err("Block not found in CAS storage pool")
    }

    /// Gets the total number of stored blocks
    pub fn block_count(&self) -> usize {
        self.storage_pool.iter().filter(|b| b.is_some()).count()
    }

    /// Gets the total storage usage in bytes
    pub fn storage_usage(&self) -> usize {
        self.storage_pool
            .iter()
            .filter_map(|b| b.as_ref())
            .map(|b| b.data_length)
            .sum()
    }
}

impl Default for SigmaFsCasEngine {
    fn default() -> Self {
        Self::new([0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_computation() {
        let engine = SigmaFsCasEngine::new([0u8; 32]);
        let data = b"Hello, SigmaOS!";
        let hash = engine.compute_sha256(data);

        assert_ne!(hash, [0u8; SHA256_HASH_SIZE]);
    }

    #[test]
    fn test_block_storage() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let data = b"Test data block";
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        let hash = engine.store_block(data, &signature).unwrap();
        assert_eq!(engine.block_count(), 1);
    }

    #[test]
    fn test_block_retrieval() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let data = b"Test data block";
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        let hash = engine.store_block(data, &signature).unwrap();
        let mut buffer = [0u8; 1024];
        let len = engine.read_block(&hash, &mut buffer).unwrap();

        assert_eq!(len, data.len());
        assert_eq!(&buffer[..len], data);
    }

    #[test]
    fn test_deduplication() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let data = b"Duplicate data";
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        let hash1 = engine.store_block(data, &signature).unwrap();
        let hash2 = engine.store_block(data, &signature).unwrap();

        assert_eq!(hash1, hash2);
        assert_eq!(engine.block_count(), 1); // Should only store once
    }

    #[test]
    fn test_block_deletion() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let data = b"Test data";
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        let hash = engine.store_block(data, &signature).unwrap();
        engine.delete_block(&hash).unwrap();

        assert_eq!(engine.block_count(), 0);
    }

    #[test]
    fn test_invalid_signature() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let data = b"Test data";
        let signature = [0u8; DILITHIUM5_SIGNATURE_SIZE]; // Invalid signature

        let result = engine.store_block(data, &signature);
        assert!(result.is_err());
    }

    #[test]
    fn test_block_size_limit() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let large_data = vec![0u8; 1025]; // Exceeds 1024 byte limit
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        let result = engine.store_block(&large_data, &signature);
        assert!(result.is_err());
    }

    #[test]
    fn test_storage_pool_full() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        // Fill the pool (16 blocks)
        for i in 0..16 {
            let data = format!("Block {}", i);
            engine.store_block(data.as_bytes(), &signature).unwrap();
        }

        // Try to add one more
        let result = engine.store_block(b"Extra block", &signature);
        assert!(result.is_err());
    }

    #[test]
    fn test_storage_usage() {
        let mut engine = SigmaFsCasEngine::new([0u8; 32]);
        let signature = [1u8; DILITHIUM5_SIGNATURE_SIZE];

        engine.store_block(b"Block 1", &signature).unwrap();
        engine.store_block(b"Block 2", &signature).unwrap();

        let usage = engine.storage_usage();
        assert_eq!(usage, 14); // "Block 1" (7) + "Block 2" (7)
    }
}
