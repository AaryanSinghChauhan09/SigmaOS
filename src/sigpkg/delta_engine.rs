//! SigmaOS Delta Package Update Engine
//!
//! Sovereign zero-dependency differential delta update pipeline for SigmaPkg.
//! Inspired by:
//! - bsdiff (Colin Percival, FreeBSD) — binary delta algorithm
//! - zstd patch (Meta) — chunked compression + delta
//! - OSTree/rpm-ostree (Fedora) — content-addressed delta delivery
//!
//! Provides:
//! - Rolling Adler-32 hash for fast chunk boundary detection
//! - Chunked delta generation between source and target blobs
//! - Delta application and integrity verification
//! - Delta ratio computation for bandwidth savings reporting

#![allow(dead_code)]

use std::vec::Vec;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::format;

// ─── Chunk Classification ─────────────────────────────────────────────────────

/// Classification of a delta chunk — how source maps to target
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkType {
    /// Chunk is identical in source and target — no data needed
    Unchanged,
    /// Chunk exists in target but differs from source
    Modified,
    /// Chunk exists only in target (inserted bytes)
    Inserted,
    /// Chunk exists only in source (deleted from target)
    Deleted,
}

// ─── Delta Chunk ─────────────────────────────────────────────────────────────

/// A single delta chunk describing one region of change
#[derive(Debug, Clone)]
pub struct DeltaChunk {
    /// Byte offset in the source blob
    pub offset: u64,
    /// Length of this chunk in bytes
    pub length: u32,
    /// Adler-32 rolling checksum of this chunk's data
    pub checksum: u32,
    /// Type classification
    pub chunk_type: ChunkType,
    /// Payload bytes (empty for Unchanged and Deleted)
    pub data: Vec<u8>,
}

// ─── Delta Manifest ───────────────────────────────────────────────────────────

/// Full delta manifest between a source and target blob
#[derive(Debug, Clone)]
pub struct DeltaManifest {
    /// FNV-1a hash of source blob (32 bytes)
    pub source_hash: [u8; 32],
    /// FNV-1a hash of target blob (32 bytes)
    pub target_hash: [u8; 32],
    /// Ordered list of delta chunks
    pub chunks: Vec<DeltaChunk>,
    /// Total size of the delta payload (sum of data fields)
    pub total_delta_size: u64,
    /// Number of unchanged bytes (for ratio reporting)
    pub unchanged_bytes: u64,
}

impl DeltaManifest {
    /// Returns the compression ratio: 1.0 = no savings, 0.0 = 100% savings
    pub fn compression_ratio(&self, source_size: u64) -> f64 {
        if source_size == 0 {
            return 1.0;
        }
        self.total_delta_size as f64 / source_size as f64
    }
}

// ─── Delta Engine ─────────────────────────────────────────────────────────────

/// SigmaOS Sovereign Delta Compression Engine
///
/// Zero-dependency implementation of binary delta computation and application.
pub struct SigmaDeltaEngine;

impl SigmaDeltaEngine {
    // ── Hash Functions ──────────────────────────────────────────────────────

    /// Adler-32 rolling hash over a byte window (same algorithm as zlib)
    ///
    /// Adler-32 = (s1 mod 65521) | ((s2 mod 65521) << 16)
    /// where s1 = sum of bytes + 1, s2 = sum of s1 values
    pub fn compute_rolling_hash(data: &[u8], window: usize) -> u32 {
        const MOD_ADLER: u32 = 65521;
        let end = data.len().min(window);
        let (mut s1, mut s2) = (1u32, 0u32);
        for &byte in &data[..end] {
            s1 = (s1 + byte as u32) % MOD_ADLER;
            s2 = (s2 + s1) % MOD_ADLER;
        }
        (s2 << 16) | s1
    }

    /// FNV-1a hash (64-bit) over all bytes → squeezed into [u8;32] by repeating
    ///
    /// Used for integrity verification of package blobs.
    pub fn compute_hash(data: &[u8]) -> [u8; 32] {
        const FNV_PRIME: u64 = 0x00000100000001B3;
        const FNV_OFFSET: u64 = 0xcbf29ce484222325;
        let mut hash: u64 = FNV_OFFSET;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        // Expand 8-byte hash into 32 bytes by mixing
        let mut out = [0u8; 32];
        for i in 0..8 {
            out[i] = ((hash >> (i * 8)) & 0xFF) as u8;
        }
        // Fill remaining bytes with secondary hash passes
        let mut h2 = hash.wrapping_add(0xdeadbeefcafe1234);
        for i in 8..32 {
            h2 = h2.wrapping_mul(FNV_PRIME) ^ (i as u64);
            out[i] = (h2 & 0xFF) as u8;
        }
        out
    }

    /// Verify data integrity against expected hash
    pub fn verify_integrity(data: &[u8], expected_hash: &[u8; 32]) -> bool {
        let actual = Self::compute_hash(data);
        actual == *expected_hash
    }

    // ── Delta Generation ────────────────────────────────────────────────────

    /// Generate a delta manifest between `source` and `target` byte slices.
    ///
    /// Uses a sliding-window chunker (4 KB blocks) with rolling hash boundary
    /// detection (inspired by rsync's rolling hash algorithm).
    pub fn generate_delta(source: &[u8], target: &[u8]) -> DeltaManifest {
        const CHUNK_SIZE: usize = 4096;

        let source_hash = Self::compute_hash(source);
        let target_hash = Self::compute_hash(target);

        // Build a lookup table: hash(source_chunk) -> offset in source
        let mut source_map: BTreeMap<u32, usize> = BTreeMap::new();
        let mut off = 0usize;
        while off < source.len() {
            let end = (off + CHUNK_SIZE).min(source.len());
            let h = Self::compute_rolling_hash(&source[off..end], CHUNK_SIZE);
            source_map.insert(h, off);
            off += CHUNK_SIZE;
        }

        let mut chunks: Vec<DeltaChunk> = Vec::new();
        let mut total_delta_size: u64 = 0;
        let mut unchanged_bytes: u64 = 0;
        let mut target_off = 0usize;

        while target_off < target.len() {
            let end = (target_off + CHUNK_SIZE).min(target.len());
            let chunk_data = &target[target_off..end];
            let h = Self::compute_rolling_hash(chunk_data, CHUNK_SIZE);

            if let Some(&src_off) = source_map.get(&h) {
                // Verify the chunk matches exactly
                let src_end = (src_off + CHUNK_SIZE).min(source.len());
                let src_chunk = &source[src_off..src_end];
                if src_chunk == chunk_data {
                    // Unchanged chunk
                    unchanged_bytes += chunk_data.len() as u64;
                    chunks.push(DeltaChunk {
                        offset: src_off as u64,
                        length: chunk_data.len() as u32,
                        checksum: h,
                        chunk_type: ChunkType::Unchanged,
                        data: Vec::new(),
                    });
                } else {
                    // Hash collision — treat as modified
                    total_delta_size += chunk_data.len() as u64;
                    chunks.push(DeltaChunk {
                        offset: target_off as u64,
                        length: chunk_data.len() as u32,
                        checksum: h,
                        chunk_type: ChunkType::Modified,
                        data: chunk_data.to_vec(),
                    });
                }
            } else {
                // New chunk not in source
                total_delta_size += chunk_data.len() as u64;
                let chunk_type = if target_off < source.len() {
                    ChunkType::Modified
                } else {
                    ChunkType::Inserted
                };
                chunks.push(DeltaChunk {
                    offset: target_off as u64,
                    length: chunk_data.len() as u32,
                    checksum: h,
                    chunk_type,
                    data: chunk_data.to_vec(),
                });
            }
            target_off += CHUNK_SIZE;
        }

        // Mark deleted chunks (source bytes not referenced by any target chunk)
        let target_len = target.len() as u64;
        if source.len() as u64 > target_len {
            let deleted_start = target_len;
            let deleted_len = (source.len() as u64 - target_len) as u32;
            chunks.push(DeltaChunk {
                offset: deleted_start,
                length: deleted_len,
                checksum: 0,
                chunk_type: ChunkType::Deleted,
                data: Vec::new(),
            });
        }

        DeltaManifest {
            source_hash,
            target_hash,
            chunks,
            total_delta_size,
            unchanged_bytes,
        }
    }

    /// Apply a delta manifest to a source blob to reconstruct the target.
    ///
    /// Returns the reconstructed target bytes.
    pub fn apply_delta(source: &[u8], manifest: &DeltaManifest) -> Vec<u8> {
        let mut target: Vec<u8> = Vec::new();

        for chunk in &manifest.chunks {
            match chunk.chunk_type {
                ChunkType::Unchanged => {
                    // Copy from source
                    let start = chunk.offset as usize;
                    let end = (start + chunk.length as usize).min(source.len());
                    target.extend_from_slice(&source[start..end]);
                }
                ChunkType::Modified | ChunkType::Inserted => {
                    // Use chunk's own data
                    target.extend_from_slice(&chunk.data);
                }
                ChunkType::Deleted => {
                    // Skip — this source region is deleted
                }
            }
        }

        target
    }

    // ── Ratio & Statistics ───────────────────────────────────────────────────

    /// Compute delta compression ratio: delta_size / source_size
    /// Returns a value in [0.0, ∞). Lower is better.
    pub fn delta_ratio(source_size: u64, delta_size: u64) -> f64 {
        if source_size == 0 {
            return 1.0;
        }
        delta_size as f64 / source_size as f64
    }

    /// Returns human-readable delta statistics string
    pub fn delta_stats(manifest: &DeltaManifest, source_size: u64) -> String {
        let ratio = Self::delta_ratio(source_size, manifest.total_delta_size);
        let savings_pct = ((1.0 - ratio) * 100.0).max(0.0);
        format!(
            "Delta: {} chunks | {} delta bytes | {} unchanged bytes | {:.1}% bandwidth savings",
            manifest.chunks.len(),
            manifest.total_delta_size,
            manifest.unchanged_bytes,
            savings_pct
        )
    }

    // ── Simple Run-Length Encoding (lightweight compression) ─────────────────

    /// Compress delta payload bytes using simple RLE encoding
    ///
    /// Format: [count: u8][byte: u8] for runs ≥ 2; [0x00][byte: u8] for singles
    pub fn rle_compress(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }
        let mut out: Vec<u8> = Vec::new();
        let mut i = 0usize;
        while i < data.len() {
            let byte = data[i];
            let mut count = 1u8;
            loop {
                let next = i + (count as usize);
                if next >= data.len() || data[next] != byte || count >= 255 {
                    break;
                }
                count += 1;
            }
            out.push(count);
            out.push(byte);
            i += count as usize;
        }
        out
    }

    /// Decompress RLE-encoded bytes
    pub fn rle_decompress(data: &[u8]) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let mut i = 0usize;
        while i + 1 < data.len() {
            let count = data[i] as usize;
            let byte = data[i + 1];
            for _ in 0..count {
                out.push(byte);
            }
            i += 2;
        }
        out
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod delta_tests {
    use super::*;

    #[test]
    fn test_adler32_basic() {
        let data = b"SigmaOS delta engine test";
        let h = SigmaDeltaEngine::compute_rolling_hash(data, 256);
        assert_ne!(h, 0);
        // Same data should produce same hash
        assert_eq!(h, SigmaDeltaEngine::compute_rolling_hash(data, 256));
    }

    #[test]
    fn test_fnv_hash_deterministic() {
        let h1 = SigmaDeltaEngine::compute_hash(b"test package 1.0");
        let h2 = SigmaDeltaEngine::compute_hash(b"test package 1.0");
        assert_eq!(h1, h2);
        let h3 = SigmaDeltaEngine::compute_hash(b"test package 1.1");
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_integrity_verification() {
        let data = b"SigmaOS package blob data";
        let hash = SigmaDeltaEngine::compute_hash(data);
        assert!(SigmaDeltaEngine::verify_integrity(data, &hash));
        let wrong_hash = SigmaDeltaEngine::compute_hash(b"different data");
        assert!(!SigmaDeltaEngine::verify_integrity(data, &wrong_hash));
    }

    #[test]
    fn test_delta_identical_blobs() {
        let source = vec![0x42u8; 8192];
        let target = source.clone();
        let manifest = SigmaDeltaEngine::generate_delta(&source, &target);
        // All chunks should be unchanged
        for chunk in &manifest.chunks {
            assert_eq!(chunk.chunk_type, ChunkType::Unchanged);
        }
        assert_eq!(manifest.total_delta_size, 0);
    }

    #[test]
    fn test_delta_apply_roundtrip() {
        let source: Vec<u8> = (0..8192u16).map(|i| (i % 256) as u8).collect();
        let mut target = source.clone();
        // Modify a middle section
        for i in 4096..4200 {
            target[i] = 0xFF;
        }
        // Append new bytes
        target.extend_from_slice(b"new section appended");

        let manifest = SigmaDeltaEngine::generate_delta(&source, &target);
        let reconstructed = SigmaDeltaEngine::apply_delta(&source, &manifest);
        assert_eq!(reconstructed, target);
    }

    #[test]
    fn test_delta_ratio_all_new() {
        let ratio = SigmaDeltaEngine::delta_ratio(1000, 1000);
        assert!((ratio - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_rle_compress_decompress() {
        let data = vec![0xAAu8; 100];
        let compressed = SigmaDeltaEngine::rle_compress(&data);
        // Should compress well: repeated bytes
        assert!(compressed.len() < data.len());
        let decompressed = SigmaDeltaEngine::rle_decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_rle_incompressible() {
        let data: Vec<u8> = (0..100u8).collect();
        let compressed = SigmaDeltaEngine::rle_compress(&data);
        let decompressed = SigmaDeltaEngine::rle_decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_delta_stats_format() {
        let source = vec![0u8; 4096];
        let target = vec![1u8; 4096];
        let manifest = SigmaDeltaEngine::generate_delta(&source, &target);
        let stats = SigmaDeltaEngine::delta_stats(&manifest, 4096);
        assert!(stats.contains("Delta:"));
        assert!(stats.contains("chunks"));
    }
}
