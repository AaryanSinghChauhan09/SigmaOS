//! HAMMER2 Filesystem — DragonFlyBSD Inspired Implementation for SigmaOS
//!
//! HAMMER2 is the second-generation filesystem from DragonFlyBSD, designed by
//! Matthew Dillon. It features:
//! - Copy-on-write (CoW) semantics with a B-tree media structure
//! - Built-in compression (LZ4, zlib, zstd)
//! - Built-in deduplication via content-addressed blocks
//! - Clustered operation (multiple volumes, multi-master)
//! - Snapshots with O(1) complexity
//! - PFS (Pseudo-Filesystems) for isolation
//! - CRC32/SHA256 data integrity
//!
//! References:
//! - HAMMER2 Design: https://www.dragonflybsd.org/hammer/
//! - Matthew Dillon's HAMMER2 spec: https://gitweb.dragonflybsd.org/dragonfly.git
//! - ZFS (FreeBSD/TrueNAS) for deduplication inspiration
//! - Btrfs (Linux) for subvolume concepts
//!
//! Future Development:
//! - Real block device I/O integration with NVMe/AHCI drivers
//! - Multi-master clustering support (HAMMER2 Cluster Protocol)
//! - Online resizing and defragmentation
//! - TRIM/discard support for SSDs
//! - Integration with SigmaOS VFS layer

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// HAMMER2 block size (always 64KB for media blocks)
pub const HAMMER2_PBUFSIZE: usize = 65536;
/// HAMMER2 minimum block size (512 bytes)
pub const HAMMER2_MINBLOCKSIZE: usize = 512;
/// HAMMER2 max PFS (pseudo-filesystem) name length
pub const HAMMER2_INODE_MAXNAME: usize = 256;

/// HAMMER2 Media Compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Hammer2Compress {
    None = 0,
    Autozero = 1, // Auto-detect zero-filled blocks
    Lz4 = 2,
    Zlib = 3,
    Zstd = 4, // Added in HAMMER2 v7+
}

/// HAMMER2 Check algorithm for data integrity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Hammer2Check {
    None = 0,
    Disabled = 1,
    Crc32 = 2,
    Xxhash64 = 3,
    Sha256 = 4,
}

/// HAMMER2 Inode types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Hammer2InodeType {
    Unknown = 0,
    Directory = 1,
    File = 2,
    Symlink = 3,
    Blockdev = 4,
    Chardev = 5,
    Fifo = 6,
    Socket = 7,
    PFS = 8, // Pseudo-filesystem root
    Snapshot = 9,
}

/// HAMMER2 Inode Structure (in-memory representation)
#[derive(Debug, Clone)]
pub struct Hammer2Inode {
    /// Inode number
    pub inum: u64,
    /// Inode type
    pub itype: Hammer2InodeType,
    /// File size in bytes
    pub size: u64,
    /// Number of hard links
    pub nlinks: u64,
    /// Permissions (Unix mode bits)
    pub mode: u32,
    /// Owner UID
    pub uid: u32,
    /// Owner GID
    pub gid: u32,
    /// Creation time (nanoseconds since epoch)
    pub ctime: u64,
    /// Modification time
    pub mtime: u64,
    /// Inode name (for directory entries)
    pub filename: [u8; HAMMER2_INODE_MAXNAME],
    /// Filename length
    pub name_len: u16,
    /// Compression algorithm for this inode's data
    pub comp_algo: Hammer2Compress,
    /// Check algorithm for data integrity
    pub check_algo: Hammer2Check,
    /// Block references (content-addressed storage)
    pub block_refs: Vec<Hammer2BlockRef>,
    /// Inode flags
    pub flags: u32,
}

/// HAMMER2 Block Reference — content-addressed block pointer
#[derive(Debug, Clone)]
pub struct Hammer2BlockRef {
    /// Content hash (SHA256/CRC32 depending on check_algo)
    pub key: u64,
    /// Physical block offset on device
    pub data_off: u64,
    /// Logical data size (before compression)
    pub data_count: u32,
    /// Compressed data size (0 = uncompressed)
    pub data_compress_count: u32,
    /// Compression algorithm used for this block
    pub methods: u8, // compression | check algorithm
    /// Type of data at this ref
    pub btype: u8,
    /// Embed (inline data for small files <= 512 bytes)
    pub embed: [u8; 64],
    /// Is this block embedded (inline data)?
    pub is_embedded: bool,
}

impl Hammer2BlockRef {
    pub fn new_embedded(data: &[u8]) -> Self {
        let mut embed = [0u8; 64];
        let len = data.len().min(64);
        embed[..len].copy_from_slice(&data[..len]);
        Self {
            key: Self::crc32_hash(&embed[..len]),
            data_off: 0,
            data_count: len as u32,
            data_compress_count: 0,
            methods: 0,
            btype: 0,
            embed,
            is_embedded: true,
        }
    }

    pub fn new_block(offset: u64, size: u32, hash: u64) -> Self {
        Self {
            key: hash,
            data_off: offset,
            data_count: size,
            data_compress_count: 0,
            methods: 0,
            btype: 1,
            embed: [0u8; 64],
            is_embedded: false,
        }
    }

    /// Simple CRC32-like hash for content addressing
    fn crc32_hash(data: &[u8]) -> u64 {
        let mut h: u64 = 0x811c9dc5;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x1000193);
        }
        h
    }
}

impl Hammer2Inode {
    pub fn new_file(inum: u64, mode: u32, uid: u32, gid: u32) -> Self {
        Self {
            inum,
            itype: Hammer2InodeType::File,
            size: 0,
            nlinks: 1,
            mode,
            uid,
            gid,
            ctime: 0,
            mtime: 0,
            filename: [0u8; HAMMER2_INODE_MAXNAME],
            name_len: 0,
            comp_algo: Hammer2Compress::Lz4,
            check_algo: Hammer2Check::Crc32,
            block_refs: Vec::new(),
            flags: 0,
        }
    }

    pub fn new_dir(inum: u64, mode: u32, uid: u32, gid: u32) -> Self {
        let mut inode = Self::new_file(inum, mode, uid, gid);
        inode.itype = Hammer2InodeType::Directory;
        inode.nlinks = 2; // . and parent
        inode
    }

    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(HAMMER2_INODE_MAXNAME - 1);
        self.filename[..len].copy_from_slice(&bytes[..len]);
        self.name_len = len as u16;
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.filename[..self.name_len as usize]).unwrap_or("")
    }
}

/// HAMMER2 Pseudo-Filesystem (PFS) — isolation boundary within a volume
#[derive(Debug, Clone)]
pub struct Hammer2PFS {
    /// PFS name (e.g. "ROOT", "data", "snapshots")
    pub name: String,
    /// PFS type: master, slave, soft-slave, cache, etc.
    pub pfs_type: u8,
    /// PFS cluster ID
    pub pfs_clid: u128,
    /// PFS filesystem ID  
    pub pfs_fsid: u128,
    /// Root inode number for this PFS
    pub iroot_inum: u64,
    /// Snapshot generation number
    pub mirror_tid: u64,
}

impl Hammer2PFS {
    pub const TYPE_MASTER: u8 = 1;
    pub const TYPE_SLAVE: u8 = 2;
    pub const TYPE_SOFT_SLAVE: u8 = 3;
    pub const TYPE_CACHE: u8 = 5;

    pub fn new_master(name: &str) -> Self {
        Self {
            name: String::from(name),
            pfs_type: Self::TYPE_MASTER,
            pfs_clid: 0,
            pfs_fsid: 0,
            iroot_inum: 1,
            mirror_tid: 1,
        }
    }
}

/// HAMMER2 Snapshot — O(1) CoW snapshot of a PFS
#[derive(Debug, Clone)]
pub struct Hammer2Snapshot {
    /// Snapshot name
    pub name: String,
    /// Mirror transaction ID at time of snapshot
    pub mirror_tid: u64,
    /// Root inode reference for this snapshot
    pub root_ref: Hammer2BlockRef,
    /// Timestamp when snapshot was taken
    pub timestamp_ns: u64,
}

/// HAMMER2 Volume — top-level filesystem container
#[derive(Debug)]
pub struct Hammer2Volume {
    /// Volume label/name
    pub label: String,
    /// Volume size in bytes
    pub vol_size: u64,
    /// Block size
    pub block_size: u32,
    /// Inode table (inum -> inode)
    pub inodes: BTreeMap<u64, Hammer2Inode>,
    /// Directory entries (parent_inum -> [(name, child_inum)])
    pub dentries: BTreeMap<u64, Vec<(String, u64)>>,
    /// Pseudo-filesystems within this volume
    pub pfs_map: BTreeMap<String, Hammer2PFS>,
    /// Snapshots
    pub snapshots: Vec<Hammer2Snapshot>,
    /// Content-addressed dedup table: hash -> block_offset
    pub dedup_table: BTreeMap<u64, u64>,
    /// Next inode number
    next_inum: u64,
    /// Current mirror transaction ID
    mirror_tid: u64,
    /// Compression enabled
    pub compress: bool,
    /// Dedup enabled
    pub dedup: bool,
}

impl Hammer2Volume {
    /// Create a new HAMMER2 volume
    pub fn new(label: &str, size_gb: u64) -> Self {
        let mut vol = Self {
            label: String::from(label),
            vol_size: size_gb * 1024 * 1024 * 1024,
            block_size: HAMMER2_PBUFSIZE as u32,
            inodes: BTreeMap::new(),
            dentries: BTreeMap::new(),
            pfs_map: BTreeMap::new(),
            snapshots: Vec::new(),
            dedup_table: BTreeMap::new(),
            next_inum: 1,
            mirror_tid: 1,
            compress: true,
            dedup: true,
        };
        // Create root PFS
        let root_pfs = Hammer2PFS::new_master("ROOT");
        vol.pfs_map.insert(String::from("ROOT"), root_pfs);
        // Create root directory inode
        let root = Hammer2Inode::new_dir(1, 0o755, 0, 0);
        vol.inodes.insert(1, root);
        vol.next_inum = 2;
        vol
    }

    /// Mount a new PFS within this volume
    pub fn create_pfs(&mut self, name: &str) -> Result<(), &'static str> {
        if self.pfs_map.contains_key(name) {
            return Err("PFS already exists");
        }
        let pfs = Hammer2PFS::new_master(name);
        let pfs_root_inum = self.alloc_inum();
        let pfs_root = Hammer2Inode::new_dir(pfs_root_inum, 0o755, 0, 0);
        self.inodes.insert(pfs_root_inum, pfs_root);
        self.pfs_map.insert(String::from(name), pfs);
        Ok(())
    }

    /// Create a file at a path under parent_inum
    pub fn create_file(
        &mut self,
        parent_inum: u64,
        name: &str,
        mode: u32,
    ) -> Result<u64, &'static str> {
        if !self.inodes.contains_key(&parent_inum) {
            return Err("Parent inode not found");
        }
        let inum = self.alloc_inum();
        let mut inode = Hammer2Inode::new_file(inum, mode, 0, 0);
        inode.set_name(name);
        self.inodes.insert(inum, inode);
        self.dentries
            .entry(parent_inum)
            .or_insert_with(Vec::new)
            .push((String::from(name), inum));
        Ok(inum)
    }

    /// Create a directory
    pub fn create_dir(
        &mut self,
        parent_inum: u64,
        name: &str,
        mode: u32,
    ) -> Result<u64, &'static str> {
        if !self.inodes.contains_key(&parent_inum) {
            return Err("Parent inode not found");
        }
        let inum = self.alloc_inum();
        let mut inode = Hammer2Inode::new_dir(inum, mode, 0, 0);
        inode.set_name(name);
        self.inodes.insert(inum, inode);
        self.dentries
            .entry(parent_inum)
            .or_insert_with(Vec::new)
            .push((String::from(name), inum));
        Ok(inum)
    }

    /// Write data to an inode (CoW semantics — creates new block refs)
    pub fn write(&mut self, inum: u64, data: &[u8]) -> Result<usize, &'static str> {
        if !self.inodes.contains_key(&inum) {
            return Err("Inode not found");
        }
        let len = data.len();

        // Check dedup table
        let hash = Hammer2BlockRef::crc32_hash(data);
        let block_ref = if self.dedup && self.dedup_table.contains_key(&hash) {
            // Dedup hit — reuse existing block
            let offset = *self.dedup_table.get(&hash).unwrap();
            Hammer2BlockRef::new_block(offset, len as u32, hash)
        } else if len <= 64 {
            // Small file — embed inline
            Hammer2BlockRef::new_embedded(data)
        } else {
            // Allocate new block (simulated)
            let offset = self.next_inum * HAMMER2_PBUFSIZE as u64;
            if self.dedup {
                self.dedup_table.insert(hash, offset);
            }
            Hammer2BlockRef::new_block(offset, len as u32, hash)
        };

        if let Some(inode) = self.inodes.get_mut(&inum) {
            inode.block_refs.push(block_ref);
            inode.size = len as u64;
            inode.mtime = 1_000_000_000; // Simulated timestamp
            self.mirror_tid += 1;
        }
        Ok(len)
    }

    /// Take an O(1) CoW snapshot of a PFS
    pub fn snapshot(&mut self, pfs_name: &str, snap_name: &str) -> Result<(), &'static str> {
        if !self.pfs_map.contains_key(pfs_name) {
            return Err("PFS not found");
        }
        let tid = self.mirror_tid;
        // Create snapshot reference pointing to current root state
        let root_ref = Hammer2BlockRef::new_block(0, 0, self.next_inum);
        let snap = Hammer2Snapshot {
            name: String::from(snap_name),
            mirror_tid: tid,
            root_ref,
            timestamp_ns: 1_000_000_000,
        };
        self.snapshots.push(snap);
        self.mirror_tid += 1;
        Ok(())
    }

    /// List directory contents
    pub fn readdir(&self, inum: u64) -> Vec<(String, u64)> {
        self.dentries.get(&inum).cloned().unwrap_or_default()
    }

    /// Lookup a name in a directory
    pub fn lookup(&self, parent_inum: u64, name: &str) -> Option<u64> {
        self.dentries
            .get(&parent_inum)?
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, inum)| *inum)
    }

    /// Get filesystem statistics
    pub fn statfs(&self) -> Hammer2Statfs {
        Hammer2Statfs {
            vol_size: self.vol_size,
            inodes_total: self.inodes.len() as u64,
            inodes_used: self.inodes.len() as u64,
            snapshots: self.snapshots.len() as u64,
            pfs_count: self.pfs_map.len() as u64,
            dedup_entries: self.dedup_table.len() as u64,
            mirror_tid: self.mirror_tid,
            compress_enabled: self.compress,
            dedup_enabled: self.dedup,
        }
    }

    fn alloc_inum(&mut self) -> u64 {
        let inum = self.next_inum;
        self.next_inum += 1;
        inum
    }
}

/// HAMMER2 Filesystem statistics
#[derive(Debug)]
pub struct Hammer2Statfs {
    pub vol_size: u64,
    pub inodes_total: u64,
    pub inodes_used: u64,
    pub snapshots: u64,
    pub pfs_count: u64,
    pub dedup_entries: u64,
    pub mirror_tid: u64,
    pub compress_enabled: bool,
    pub dedup_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_creation() {
        let vol = Hammer2Volume::new("SigmaOS-HAMMER2", 100);
        assert_eq!(vol.label, "SigmaOS-HAMMER2");
        assert_eq!(vol.vol_size, 100 * 1024 * 1024 * 1024);
        assert!(vol.pfs_map.contains_key("ROOT"));
        assert!(vol.inodes.contains_key(&1));
    }

    #[test]
    fn test_create_file_and_write() {
        let mut vol = Hammer2Volume::new("test", 10);
        let inum = vol.create_file(1, "hello.txt", 0o644).unwrap();
        let written = vol.write(inum, b"Hello, HAMMER2!").unwrap();
        assert_eq!(written, 15);
        let inode = vol.inodes.get(&inum).unwrap();
        assert_eq!(inode.size, 15);
        assert_eq!(inode.block_refs.len(), 1);
    }

    #[test]
    fn test_directory_operations() {
        let mut vol = Hammer2Volume::new("test", 10);
        let dir_inum = vol.create_dir(1, "src", 0o755).unwrap();
        let file_inum = vol.create_file(dir_inum, "main.rs", 0o644).unwrap();

        let entries = vol.readdir(dir_inum);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "main.rs");

        let looked_up = vol.lookup(dir_inum, "main.rs");
        assert_eq!(looked_up, Some(file_inum));
    }

    #[test]
    fn test_snapshot() {
        let mut vol = Hammer2Volume::new("test", 10);
        vol.create_file(1, "data.txt", 0o644).unwrap();
        vol.snapshot("ROOT", "backup-2026-10-09").unwrap();
        assert_eq!(vol.snapshots.len(), 1);
        assert_eq!(vol.snapshots[0].name, "backup-2026-10-09");
    }

    #[test]
    fn test_dedup() {
        let mut vol = Hammer2Volume::new("test", 10);
        let f1 = vol.create_file(1, "a.txt", 0o644).unwrap();
        let f2 = vol.create_file(1, "b.txt", 0o644).unwrap();

        // Write same data (>64 bytes to avoid inline embedding) to both files
        let data = b"Duplicate content that should be deduped across multiple blocks in the HAMMER2 volume storage engine";
        vol.write(f1, data).unwrap();
        vol.write(f2, data).unwrap();

        // Both should have the same block hash (dedup)
        let inode1 = vol.inodes.get(&f1).unwrap();
        let inode2 = vol.inodes.get(&f2).unwrap();
        assert_eq!(inode1.block_refs[0].key, inode2.block_refs[0].key);
        assert_eq!(vol.dedup_table.len(), 1); // Only 1 unique block
    }

    #[test]
    fn test_create_pfs() {
        let mut vol = Hammer2Volume::new("test", 10);
        vol.create_pfs("home").unwrap();
        vol.create_pfs("var").unwrap();
        assert!(vol.pfs_map.contains_key("home"));
        assert!(vol.pfs_map.contains_key("var"));

        // Duplicate PFS should fail
        assert!(vol.create_pfs("home").is_err());
    }

    #[test]
    fn test_inline_embed_small_files() {
        let mut vol = Hammer2Volume::new("test", 10);
        let inum = vol.create_file(1, "tiny.txt", 0o644).unwrap();
        vol.write(inum, b"tiny").unwrap();

        let inode = vol.inodes.get(&inum).unwrap();
        assert!(inode.block_refs[0].is_embedded);
    }

    #[test]
    fn test_statfs() {
        let mut vol = Hammer2Volume::new("test", 10);
        vol.create_file(1, "a.txt", 0o644).unwrap();
        vol.create_dir(1, "dir", 0o755).unwrap();

        let stats = vol.statfs();
        assert_eq!(stats.pfs_count, 1);
        assert!(stats.inodes_used >= 3); // root + file + dir
    }
}
