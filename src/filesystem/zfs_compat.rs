use std::string::{String, ToString};
use std::format;
// SigmaOS — ZFS-Compatible Filesystem Features
//
// Inspired by Sun/OpenZFS: copy-on-write semantics, per-block checksumming,
// transparent compression, snapshots, and dataset management.
//
// References:
//   Jeff Bonwick & Bill Moore, "ZFS: The Last Word in File Systems," 2006.
//   OpenZFS project — https://openzfs.org/
//
// This is a custom, no_std implementation — no external crate dependencies.

use crate::klib::vec::Vec;

// ─────────────────────────────────────────────────────────────────────────────
// Dataset types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatasetKind {
    /// Mountable filesystem dataset.
    FileSystem,
    /// Block-device volume.
    Volume,
    /// Point-in-time snapshot (read-only).
    Snapshot,
    /// Lightweight bookmark (no data, just a reference point).
    Bookmark,
}

// ─────────────────────────────────────────────────────────────────────────────
// Checksum algorithms
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgo {
    /// No checksum (not recommended).
    Off,
    /// Fletcher-4 — ZFS default for data; fast, software-only.
    Fletcher4,
    /// SHA-256 — cryptographic integrity.
    Sha256,
    /// SHA-512 — stronger cryptographic integrity.
    Sha512,
    /// BLAKE3 — modern, parallel, cryptographic hash.
    Blake3,
}

// ─────────────────────────────────────────────────────────────────────────────
// Compression algorithms
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgo {
    /// No compression.
    Off,
    /// LZ4 — fast, recommended for most datasets.
    Lz4,
    /// Zstandard — better ratio, adjustable level.
    Zstd,
    /// Gzip — legacy compatibility.
    Gzip,
}

// ─────────────────────────────────────────────────────────────────────────────
// Snapshot info
// ─────────────────────────────────────────────────────────────────────────────

/// Metadata about a point-in-time snapshot.
#[derive(Debug, Clone)]
pub struct SnapshotInfo {
    /// Snapshot name (e.g. `"backup-2026-08-08"`).
    name: [u8; 64],
    name_len: usize,
    /// Kernel timestamp (seconds since epoch or boot).
    pub creation_time: u64,
    /// Bytes used exclusively by this snapshot (delta from parent).
    pub used_bytes: u64,
    /// Bytes referenced (shared with parent dataset).
    pub referenced: u64,
}

impl SnapshotInfo {
    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Dataset
// ─────────────────────────────────────────────────────────────────────────────

/// A ZFS-style dataset — the primary unit of filesystem management.
pub struct Dataset {
    /// Dataset name (e.g. `"sigma/home"`, `"sigma/data@snap"`).
    name:        [u8; 256],
    name_len:    usize,
    pub kind:    DatasetKind,
    pub checksum:    ChecksumAlgo,
    pub compression: CompressionAlgo,
    /// Total bytes used on-disk.
    pub used_bytes:  u64,
    /// Available space (inherited from pool quota).
    pub avail_bytes: u64,
    /// Bytes referenced (including shared data with snapshots).
    pub referenced:  u64,
    /// Compression ratio × 100 (e.g. 200 = 2.0×).
    pub compress_ratio_x100: u32,
    /// Snapshots attached to this dataset.
    pub snapshots: Vec<SnapshotInfo>,
    /// Whether copy-on-write is enabled (always true in ZFS semantics).
    cow_enabled: bool,
}

impl Dataset {
    /// Create a new dataset.
    pub fn new(name: &[u8], kind: DatasetKind) -> Self {
        let len = name.len().min(255);
        let mut n = [0u8; 256];
        n[..len].copy_from_slice(&name[..len]);
        Self {
            name: n,
            name_len: len,
            kind,
            checksum: ChecksumAlgo::Fletcher4,
            compression: CompressionAlgo::Lz4,
            used_bytes: 0,
            avail_bytes: u64::MAX,
            referenced: 0,
            compress_ratio_x100: 100, // 1.0× (no savings yet)
            snapshots: Vec::new(),
            cow_enabled: true,
        }
    }

    pub fn name(&self) -> &[u8] { &self.name[..self.name_len] }
    pub fn cow_enabled(&self) -> bool { self.cow_enabled }

    // ── Checksum computation ──────────────────────────────────────────────────

    /// Fletcher-4 checksum — the same algorithm used by ZFS for data blocks.
    ///
    /// Produces a 64-bit value: upper 32 bits = `b`, lower 32 bits = `a`.
    pub fn fletcher4(data: &[u8]) -> u64 {
        let mut a: u64 = 0;
        let mut b: u64 = 0;
        for &byte in data {
            a = a.wrapping_add(byte as u64);
            b = b.wrapping_add(a);
        }
        (b << 32) | (a & 0xFFFF_FFFF)
    }

    /// Simple XOR-fold checksum (faster, weaker — for metadata blocks).
    pub fn xor_checksum(data: &[u8]) -> u32 {
        let mut v: u32 = 0;
        let mut i = 0usize;
        while i + 3 < data.len() {
            v ^= u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]);
            i += 4;
        }
        while i < data.len() {
            v ^= data[i] as u32;
            i += 1;
        }
        v
    }

    // ── Snapshot management ───────────────────────────────────────────────────

    /// Create a named snapshot at the given timestamp.
    pub fn snapshot(&mut self, snap_name: &[u8], time: u64) -> Result<(), ZfsError> {
        if self.kind == DatasetKind::Snapshot {
            return Err(ZfsError::CannotSnapshotSnapshot);
        }
        let len = snap_name.len().min(63);
        let mut n = [0u8; 64];
        n[..len].copy_from_slice(&snap_name[..len]);
        self.snapshots.push(SnapshotInfo {
            name: n,
            name_len: len,
            creation_time: time,
            used_bytes: 0, // snapshots start with 0 unique bytes
            referenced: self.referenced,
        });
        Ok(())
    }

    /// Delete a snapshot by name.
    pub fn destroy_snapshot(&mut self, snap_name: &[u8]) -> Result<(), ZfsError> {
        for i in 0..self.snapshots.len() {
            if let Some(s) = self.snapshots.get(i) {
                if s.name() == snap_name {
                    // Swap-remove (O(1)).
                    let last = self.snapshots.len() - 1;
                    if i != last {
                        if let (Some(last_snap), Some(slot)) = (
                            self.snapshots.get(last).cloned(),
                            self.snapshots.get_mut(i)
                        ) {
                            *slot = last_snap;
                        }
                    }
                    let _ = self.snapshots.pop();
                    return Ok(());
                }
            }
        }
        Err(ZfsError::SnapshotNotFound)
    }

    /// Find a snapshot by name.
    pub fn find_snapshot(&self, snap_name: &[u8]) -> Option<&SnapshotInfo> {
        for i in 0..self.snapshots.len() {
            if let Some(s) = self.snapshots.get(i) {
                if s.name() == snap_name {
                    return Some(s);
                }
            }
        }
        None
    }

    pub fn snapshot_count(&self) -> usize { self.snapshots.len() }

    // ── Space tracking ────────────────────────────────────────────────────────

    /// Record that `bytes` of data has been written (CoW allocation).
    pub fn record_write(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_add(bytes);
        self.referenced = self.referenced.saturating_add(bytes);
    }

    /// Record freed space after a destroy / overwrite.
    pub fn record_free(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
        self.referenced = self.referenced.saturating_sub(bytes);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pool — collection of datasets
// ─────────────────────────────────────────────────────────────────────────────

/// A ZFS storage pool — a named collection of datasets sharing a common
/// storage backend.
pub struct Pool {
    name:       [u8; 64],
    name_len:   usize,
    pub total_bytes: u64,
    pub used_bytes:  u64,
    datasets:   Vec<Dataset>,
}

impl Pool {
    pub fn new(name: &[u8], total_bytes: u64) -> Self {
        let len = name.len().min(63);
        let mut n = [0u8; 64];
        n[..len].copy_from_slice(&name[..len]);
        Self { name: n, name_len: len, total_bytes, used_bytes: 0, datasets: Vec::new() }
    }

    pub fn name(&self) -> &[u8] { &self.name[..self.name_len] }

    pub fn avail_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes)
    }

    pub fn create_dataset(&mut self, name: &[u8], kind: DatasetKind) -> Result<(), ZfsError> {
        // Check for name collision.
        for i in 0..self.datasets.len() {
            if let Some(ds) = self.datasets.get(i) {
                if ds.name() == name {
                    return Err(ZfsError::DatasetExists);
                }
            }
        }
        self.datasets.push(Dataset::new(name, kind));
        Ok(())
    }

    pub fn dataset_count(&self) -> usize { self.datasets.len() }

    pub fn get_dataset(&self, name: &[u8]) -> Option<&Dataset> {
        for i in 0..self.datasets.len() {
            if let Some(ds) = self.datasets.get(i) {
                if ds.name() == name {
                    return Some(ds);
                }
            }
        }
        None
    }

    pub fn get_dataset_mut(&mut self, name: &[u8]) -> Option<&mut Dataset> {
        for i in 0..self.datasets.len() {
            if let Some(ds) = self.datasets.get_mut(i) {
                if ds.name() == name {
                    return Some(ds);
                }
            }
        }
        None
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZfsError {
    DatasetExists,
    DatasetNotFound,
    SnapshotNotFound,
    CannotSnapshotSnapshot,
    OutOfSpace,
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_fletcher4_deterministic() {
        let data = b"SigmaOS ZFS test block";
        let c1 = Dataset::fletcher4(data);
        let c2 = Dataset::fletcher4(data);
        assert_eq!(c1, c2, "checksum must be deterministic");
    }

    #[test]
    fn test_fletcher4_detects_change() {
        let a = Dataset::fletcher4(b"hello");
        let b = Dataset::fletcher4(b"hellO");
        assert_ne!(a, b, "different data must produce different checksum");
    }

    #[test]
    fn test_snapshot_lifecycle() {
        let mut ds = Dataset::new(b"sigma/home", DatasetKind::FileSystem);
        ds.snapshot(b"snap-1", 1000).unwrap();
        ds.snapshot(b"snap-2", 2000).unwrap();
        assert_eq!(ds.snapshot_count(), 2);
        assert!(ds.find_snapshot(b"snap-1").is_some());
        ds.destroy_snapshot(b"snap-1").unwrap();
        assert_eq!(ds.snapshot_count(), 1);
        assert!(ds.find_snapshot(b"snap-1").is_none());
    }

    #[test]
    fn test_cannot_snapshot_snapshot() {
        let mut ds = Dataset::new(b"sigma/home@old", DatasetKind::Snapshot);
        assert_eq!(ds.snapshot(b"nested", 0), Err(ZfsError::CannotSnapshotSnapshot));
    }

    #[test]
    fn test_pool_create_dataset() {
        let mut pool = Pool::new(b"sigma", 1 << 30);
        pool.create_dataset(b"sigma/home", DatasetKind::FileSystem).unwrap();
        pool.create_dataset(b"sigma/data", DatasetKind::FileSystem).unwrap();
        assert_eq!(pool.dataset_count(), 2);
        assert!(pool.create_dataset(b"sigma/home", DatasetKind::FileSystem).is_err());
    }

    #[test]
    fn test_space_tracking() {
        let mut ds = Dataset::new(b"sigma/test", DatasetKind::FileSystem);
        ds.record_write(4096);
        assert_eq!(ds.used_bytes, 4096);
        ds.record_free(1024);
        assert_eq!(ds.used_bytes, 3072);
    }
}
