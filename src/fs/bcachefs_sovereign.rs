#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]


// SigmaOS Sovereign bcachefs Copy-on-Write Filesystem Layer
// Implements bcachefs-inspired CoW filesystem concepts in 100% safe Rust.
//
// bcachefs was developed by Kent Overstreet and merged into Linux 6.7 (2024).
// It combines bcache (block-layer caching) with a full filesystem providing:
// - Copy-on-write semantics
// - Checksumming (CRC32/CRC64/xxHash/SHA-256)
// - Inline compression (lz4/gzip/zstd)
// - Encryption (ChaCha20/Poly1305)
// - Snapshots and subvolumes
// - RAID (levels 0, 1, 10, 5, 6)
// - Reflinks (like Btrfs/XFS)


#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

// ─── Checksum types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ChecksumAlgorithm {
    None,
    Crc32c,
    Crc64,
    XxHash,
    Sha256,
}

impl ChecksumAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            ChecksumAlgorithm::None   => "none",
            ChecksumAlgorithm::Crc32c => "crc32c",
            ChecksumAlgorithm::Crc64  => "crc64",
            ChecksumAlgorithm::XxHash => "xxhash",
            ChecksumAlgorithm::Sha256 => "sha256",
        }
    }
}

/// Compute a deterministic CRC32c-like checksum in pure Rust (no tables).
pub fn sovereign_crc32c(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0x82F6_3B78;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

// ─── Compression ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CompressionType {
    None,
    Lz4,
    Gzip,
    Zstd,
}

impl CompressionType {
    pub fn name(&self) -> &'static str {
        match self {
            CompressionType::None  => "none",
            CompressionType::Lz4   => "lz4",
            CompressionType::Gzip  => "gzip",
            CompressionType::Zstd  => "zstd",
        }
    }

    /// Simulated compression ratio for testing (not actual compression).
    pub fn simulated_ratio(&self) -> f64 {
        match self {
            CompressionType::None  => 1.0,
            CompressionType::Lz4   => 0.70,
            CompressionType::Gzip  => 0.55,
            CompressionType::Zstd  => 0.45,
        }
    }
}

// ─── Extent (bcachefs key-value B-tree leaf) ──────────────────────────────────

#[derive(Debug, Clone)]
pub struct BcachefsExtent {
    pub inode: u64,
    pub offset: u64,      // file offset in bytes
    pub size: u32,        // size of extent in bytes
    pub checksum: u32,
    pub checksum_algo: ChecksumAlgorithm,
    pub compression: CompressionType,
    pub is_cow_shared: bool, // reflinked — multiple inodes reference this extent
    pub snapshot_id: u32,
    pub device_offset: u64, // physical device offset
    pub encrypted: bool,
}

impl BcachefsExtent {
    pub fn new(inode: u64, offset: u64, size: u32, data: &[u8]) -> Self {
        BcachefsExtent {
            inode,
            offset,
            size,
            checksum: sovereign_crc32c(data),
            checksum_algo: ChecksumAlgorithm::Crc32c,
            compression: CompressionType::Zstd,
            is_cow_shared: false,
            snapshot_id: 0,
            device_offset: offset, // simplified
            encrypted: false,
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        match self.checksum_algo {
            ChecksumAlgorithm::None   => true,
            ChecksumAlgorithm::Crc32c => sovereign_crc32c(data) == self.checksum,
            _                          => true, // Other algos: pass-through in simulation
        }
    }

    pub fn make_reflink_copy(&self) -> Self {
        let mut copy = self.clone();
        copy.is_cow_shared = true;
        copy
    }
}

// ─── Snapshot ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BcachefsSnapshot {
    pub id: u32,
    pub parent_id: u32, // 0 = root snapshot
    pub name: String,
    pub inode_count: u64,
    pub is_writable: bool,
    pub creation_epoch: u64,
}

impl BcachefsSnapshot {
    pub fn new(id: u32, parent_id: u32, name: &str, writable: bool) -> Self {
        BcachefsSnapshot {
            id,
            parent_id,
            name: name.to_string(),
            inode_count: 0,
            is_writable: writable,
            creation_epoch: 0,
        }
    }
}

// ─── Inode ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BcachefsInode {
    pub inode: u64,
    pub size: u64,
    pub nlinks: u32,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub snapshot_id: u32,
    pub extents: Vec<BcachefsExtent>,
    pub inline_data: Vec<u8>, // small files stored inline in inode (like ext4 inline data)
}

impl BcachefsInode {
    const INLINE_LIMIT: usize = 3072; // 3KB inline threshold

    pub fn new(inode: u64, uid: u32, gid: u32, mode: u32) -> Self {
        BcachefsInode {
            inode,
            size: 0,
            nlinks: 1,
            uid,
            gid,
            mode,
            snapshot_id: 0,
            extents: Vec::new(),
            inline_data: Vec::new(),
        }
    }

    pub fn write_data(&mut self, offset: u64, data: &[u8]) {
        if data.len() <= Self::INLINE_LIMIT && self.extents.is_empty() {
            // Inline storage
            self.inline_data.clear();
            self.inline_data.extend_from_slice(data);
        } else {
            // Extent-based storage
            self.inline_data.clear();
            let extent = BcachefsExtent::new(self.inode, offset, data.len() as u32, data);
            self.extents.push(extent);
        }
        self.size = self.size.max(offset + data.len() as u64);
    }

    pub fn is_inline(&self) -> bool {
        !self.inline_data.is_empty() && self.extents.is_empty()
    }

    pub fn extent_count(&self) -> usize { self.extents.len() }
}

// ─── bcachefs Volume ──────────────────────────────────────────────────────────

pub struct SovereignBcachefsVolume {
    pub uuid: [u8; 16],
    pub label: String,
    pub block_size: u32,
    pub total_sectors: u64,
    pub free_sectors: u64,
    pub inodes: Vec<BcachefsInode>,
    pub snapshots: Vec<BcachefsSnapshot>,
    pub next_inode: u64,
    pub next_snapshot_id: u32,
    pub default_compression: CompressionType,
    pub default_checksum: ChecksumAlgorithm,
    pub write_count: u64,
    pub read_count: u64,
    pub cow_writes: u64, // writes that triggered CoW
}

impl SovereignBcachefsVolume {
    pub fn new(label: &str, total_gb: u64) -> Self {
        let total_sectors = total_gb * 1024 * 1024 * 1024 / 512;
        SovereignBcachefsVolume {
            uuid: [0xBC, 0xAC, 0x4E, 0xF5, 0x00, 0x01, 0x02, 0x03,
                   0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B],
            label: label.to_string(),
            block_size: 4096,
            total_sectors,
            free_sectors: total_sectors,
            inodes: Vec::new(),
            snapshots: Vec::new(),
            next_inode: 2, // 0 = invalid, 1 = root dir
            next_snapshot_id: 1,
            default_compression: CompressionType::Zstd,
            default_checksum: ChecksumAlgorithm::Crc32c,
            write_count: 0,
            read_count: 0,
            cow_writes: 0,
        }
    }

    pub fn create_inode(&mut self, uid: u32, gid: u32, mode: u32) -> u64 {
        let ino = self.next_inode;
        self.next_inode = self.next_inode.saturating_add(1);
        self.inodes.push(BcachefsInode::new(ino, uid, gid, mode));
        ino
    }

    pub fn write_inode(&mut self, ino: u64, offset: u64, data: &[u8]) -> bool {
        if let Some(inode) = self.inodes.iter_mut().find(|i| i.inode == ino) {
            let sectors_needed = (data.len() as u64).div_ceil(512);
            if sectors_needed > self.free_sectors { return false; }
            // CoW: if data being written to existing extent with reflinks
            let is_cow = inode.extents.iter().any(|e| e.is_cow_shared);
            if is_cow { self.cow_writes = self.cow_writes.saturating_add(1); }
            inode.write_data(offset, data);
            self.free_sectors = self.free_sectors.saturating_sub(sectors_needed);
            self.write_count  = self.write_count.saturating_add(1);
            true
        } else { false }
    }

    pub fn create_snapshot(&mut self, name: &str, writable: bool) -> u32 {
        let id = self.next_snapshot_id;
        self.next_snapshot_id = self.next_snapshot_id.saturating_add(1);
        self.snapshots.push(BcachefsSnapshot::new(id, 0, name, writable));
        id
    }

    pub fn reflink(&mut self, src_ino: u64, dst_ino: u64) -> bool {
        // Find source extents
        let src_extents: Vec<BcachefsExtent> = self.inodes
            .iter()
            .find(|i| i.inode == src_ino)
            .map(|i| i.extents.iter().map(|e| e.make_reflink_copy()).collect())
            .unwrap_or_default();

        if src_extents.is_empty() { return false; }

        // Mark source extents as shared
        if let Some(src) = self.inodes.iter_mut().find(|i| i.inode == src_ino) {
            for e in &mut src.extents { e.is_cow_shared = true; }
        }

        // Attach shared extents to destination
        if let Some(dst) = self.inodes.iter_mut().find(|i| i.inode == dst_ino) {
            for ext in src_extents { dst.extents.push(ext); }
            true
        } else { false }
    }

    pub fn inode_count(&self) -> usize { self.inodes.len() }
    pub fn snapshot_count(&self) -> usize { self.snapshots.len() }

    pub fn used_gb(&self) -> u64 {
        let used_sectors = self.total_sectors - self.free_sectors;
        used_sectors * 512 / (1024 * 1024 * 1024)
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32c_checksum_deterministic() {
        let data = b"SigmaOS bcachefs test data";
        let c1 = sovereign_crc32c(data);
        let c2 = sovereign_crc32c(data);
        assert_eq!(c1, c2);
        // Different data must yield different checksum
        let c3 = sovereign_crc32c(b"different data");
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_extent_create_verify() {
        let data = b"hello bcachefs";
        let ext = BcachefsExtent::new(100, 0, data.len() as u32, data);
        assert!(ext.verify(data));
        // Corrupted data fails verification
        let bad = b"hello CORRUPTED";
        assert!(!ext.verify(bad));
    }

    #[test]
    fn test_inline_inode_write() {
        let mut vol = SovereignBcachefsVolume::new("sigmafs", 10);
        let ino = vol.create_inode(0, 0, 0o644);
        assert!(vol.write_inode(ino, 0, b"small file contents"));
        let inode = vol.inodes.iter().find(|i| i.inode == ino).unwrap();
        assert!(inode.is_inline());
        assert_eq!(inode.extent_count(), 0);
    }

    #[test]
    fn test_snapshot_creation() {
        let mut vol = SovereignBcachefsVolume::new("sigmafs", 100);
        let s1 = vol.create_snapshot("snap-2024-01", false);
        let s2 = vol.create_snapshot("snap-2024-02", true);
        assert_ne!(s1, s2);
        assert_eq!(vol.snapshot_count(), 2);
    }

    #[test]
    fn test_reflink_cow() {
        let mut vol = SovereignBcachefsVolume::new("sigmafs", 100);
        let src = vol.create_inode(0, 0, 0o644);
        let dst = vol.create_inode(0, 0, 0o644);
        // Write a large file (triggers extent storage)
        let big_data = vec![0xAAu8; 4096];
        vol.write_inode(src, 0, &big_data);
        assert!(vol.reflink(src, dst));
        // dst now has shared extents
        let dst_inode = vol.inodes.iter().find(|i| i.inode == dst).unwrap();
        assert!(dst_inode.extents.iter().any(|e| e.is_cow_shared));
    }

    #[test]
    fn test_compression_types() {
        assert_eq!(CompressionType::Zstd.simulated_ratio(), 0.45);
        assert!(CompressionType::Zstd.simulated_ratio() < CompressionType::Lz4.simulated_ratio());
    }
}
