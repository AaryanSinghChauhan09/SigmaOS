// SigmaOS Binary Delta Update & Zsync Engine
// Reference: zsync(1), rsync rolling checksum algorithm, Fedora drpm / deltarpm

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const DEFAULT_DELTA_BLOCK_SIZE: usize = 4096;

/// Rolling Adler-32 Checksum
pub struct RollingAdler32 {
    a: u32,
    b: u32,
    window: Vec<u8>,
}

impl RollingAdler32 {
    const MOD_ADLER: u32 = 65521;

    pub fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            window: Vec::new(),
        }
    }

    pub fn compute(data: &[u8]) -> u32 {
        let mut a: u32 = 1;
        let mut b: u32 = 0;
        for &byte in data {
            a = (a + byte as u32) % Self::MOD_ADLER;
            b = (b + a) % Self::MOD_ADLER;
        }
        (b << 16) | a
    }
}

/// Block Signature in a .zsync file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZsyncBlockSignature {
    pub block_index: u32,
    pub adler32: u32,
    pub sha256_prefix: [u8; 8],
}

/// Delta Instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeltaOp {
    CopyBlock { old_block_index: u32 },
    LiteralBytes { bytes: Vec<u8> },
}

/// Delta Package Updater
pub struct ZsyncDeltaEngine {
    block_size: usize,
    signatures: BTreeMap<u32, ZsyncBlockSignature>,
}

impl ZsyncDeltaEngine {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size,
            signatures: BTreeMap::new(),
        }
    }

    /// Generates block signature index from an existing local package binary
    pub fn index_binary(&mut self, data: &[u8]) {
        self.signatures.clear();
        let chunks = data.chunks(self.block_size);
        for (i, chunk) in chunks.enumerate() {
            let adler = RollingAdler32::compute(chunk);
            let mut prefix = [0u8; 8];
            let len = chunk.len().min(8);
            prefix[..len].copy_from_slice(&chunk[..len]);

            self.signatures.insert(
                adler,
                ZsyncBlockSignature {
                    block_index: i as u32,
                    adler32: adler,
                    sha256_prefix: prefix,
                },
            );
        }
    }

    /// Generates delta instructions given a newer binary version
    pub fn compute_delta(&self, new_data: &[u8]) -> Vec<DeltaOp> {
        let mut ops = Vec::new();
        let chunks = new_data.chunks(self.block_size);

        for chunk in chunks {
            let adler = RollingAdler32::compute(chunk);
            if let Some(sig) = self.signatures.get(&adler) {
                ops.push(DeltaOp::CopyBlock {
                    old_block_index: sig.block_index,
                });
            } else {
                ops.push(DeltaOp::LiteralBytes {
                    bytes: chunk.to_vec(),
                });
            }
        }

        ops
    }

    /// Reconstructs new target binary by applying delta operations against old binary
    pub fn apply_delta(&self, old_data: &[u8], delta: &[DeltaOp]) -> Vec<u8> {
        let mut reconstructed = Vec::new();

        for op in delta {
            match op {
                DeltaOp::CopyBlock { old_block_index } => {
                    let start = (*old_block_index as usize) * self.block_size;
                    let end = (start + self.block_size).min(old_data.len());
                    if start < old_data.len() {
                        reconstructed.extend_from_slice(&old_data[start..end]);
                    }
                }
                DeltaOp::LiteralBytes { bytes } => {
                    reconstructed.extend_from_slice(bytes);
                }
            }
        }

        reconstructed
    }
}
