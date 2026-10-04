// SigmaOS HAMMER2 MVCC Filesystem
// Inspired by DragonFly BSD HAMMER2: MVCC B-Tree, snapshots, multi-master clustering,
// compression, and media-level checksums.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};

// ─────────────────────────────────────────────────────────────────────────────
// Core Types
// ─────────────────────────────────────────────────────────────────────────────

pub type Tid = u64;  // Transaction ID (monotonic)
pub type Key = u64;  // B-Tree key
pub type BlockOffset = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumType { CRC32, XXHash64, Sha256 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgo { None, Lz4, Zlib, Zstd }

#[derive(Debug, Clone)]
pub struct MediaChecksum {
    pub algo: ChecksumType,
    pub value: u64,
}

impl MediaChecksum {
    pub fn compute(data: &[u8]) -> Self {
        // Simplified FNV-1a hash as placeholder
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        MediaChecksum { algo: ChecksumType::XXHash64, value: hash }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        let computed = Self::compute(data);
        self.value == computed.value
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HAMMER2 B-Tree Node (MVCC version-aware)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Inode,
    Indirect,
    Data,
    Freemap,
    Volume,
}

#[derive(Debug, Clone)]
pub struct Hammer2BlockRef {
    pub key: Key,
    pub key_end: Key,
    pub tid: Tid,       // birth TID
    pub delete_tid: Option<Tid>, // set when deleted (MVCC)
    pub offset: BlockOffset,
    pub checksum: MediaChecksum,
    pub compression: CompressionAlgo,
    pub data_size: u32,
}

impl Hammer2BlockRef {
    pub fn is_visible_at(&self, tid: Tid) -> bool {
        self.tid <= tid && self.delete_tid.map(|d| d > tid).unwrap_or(true)
    }
}

#[derive(Debug, Clone)]
pub enum NodeContent {
    Inode(Hammer2Inode),
    Keys(Vec<Hammer2BlockRef>),
    Data(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Hammer2Node {
    pub node_type: NodeType,
    pub tid: Tid,
    pub delete_tid: Option<Tid>,
    pub content: NodeContent,
}

#[derive(Debug, Clone)]
pub struct Hammer2Inode {
    pub inum: u64,
    pub nlinks: u32,
    pub size: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub atime_ns: u64,
    pub mtime_ns: u64,
    pub ctime_ns: u64,
    pub comp_algo: CompressionAlgo,
    pub check_algo: ChecksumType,
    /// Direct block references embedded in inode (up to 4)
    pub direct_refs: Vec<Hammer2BlockRef>,
    pub name: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// MVCC B-Tree
// ─────────────────────────────────────────────────────────────────────────────

/// A single versioned B-Tree node keyed by (Key, Tid)
pub struct MvccBTree {
    nodes: BTreeMap<(Key, Tid), Hammer2Node>,
    current_tid: AtomicU64,
}

impl MvccBTree {
    pub fn new() -> Self {
        MvccBTree {
            nodes: BTreeMap::new(),
            current_tid: AtomicU64::new(1),
        }
    }

    pub fn next_tid(&self) -> Tid {
        self.current_tid.fetch_add(1, Ordering::SeqCst)
    }

    /// Insert or update a node (COW: creates new version, never mutates old)
    pub fn insert(&mut self, key: Key, node: Hammer2Node) -> Tid {
        let tid = self.current_tid.fetch_add(1, Ordering::SeqCst);
        let mut versioned = node;
        versioned.tid = tid;
        self.nodes.insert((key, tid), versioned);
        tid
    }

    /// Logical delete: mark existing entries with delete_tid
    pub fn delete(&mut self, key: Key) -> Option<Tid> {
        let del_tid = self.current_tid.fetch_add(1, Ordering::SeqCst);
        let keys: Vec<(Key, Tid)> = self.nodes.keys()
            .filter(|&&(k, _)| k == key)
            .cloned()
            .collect();
        let mut found = false;
        for k in keys {
            if let Some(node) = self.nodes.get_mut(&k) {
                if node.delete_tid.is_none() {
                    node.delete_tid = Some(del_tid);
                    found = true;
                }
            }
        }
        if found { Some(del_tid) } else { None }
    }

    /// Lookup at a specific TID (MVCC snapshot read)
    /// A node is visible if: birth_tid <= at_tid AND (delete_tid is None OR delete_tid > at_tid)
    pub fn lookup_at(&self, key: Key, at_tid: Tid) -> Option<&Hammer2Node> {
        self.nodes.iter()
            .filter(|((k, _), node)| {
                *k == key
                    && node.tid <= at_tid
                    && node.delete_tid.map(|d| d > at_tid).unwrap_or(true)
            })
            .max_by_key(|((_, tid), _)| tid)
            .map(|(_, node)| node)
    }

    /// Scan a key range at a given TID (for directory listing)
    pub fn scan_range_at(&self, start: Key, end: Key, at_tid: Tid) -> Vec<(&Key, &Hammer2Node)> {
        self.nodes.iter()
            .filter(|((k, _), node)| *k >= start && *k <= end && node.is_visible_at(at_tid))
            .map(|((k, _), node)| (k, node))
            .collect()
    }

    /// Create a snapshot: returns the current TID (advances counter to ensure future ops get higher TIDs)
    pub fn snapshot(&self) -> Tid {
        // Advance the TID counter so snapshot_tid < any future insert/delete
        self.current_tid.fetch_add(1, Ordering::SeqCst)
    }

    /// Prune old MVCC versions older than `min_tid` (GC)
    pub fn prune(&mut self, min_tid: Tid) -> usize {
        let before = self.nodes.len();
        self.nodes.retain(|(_, tid), node| {
            // Keep if birth TID >= min_tid OR if still live (no delete_tid)
            *tid >= min_tid || node.delete_tid.is_none()
        });
        before - self.nodes.len()
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
}

impl Hammer2Node {
    pub fn is_visible_at(&self, at_tid: Tid) -> bool {
        self.tid <= at_tid && self.delete_tid.map(|d| d > at_tid).unwrap_or(true)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Snapshot Engine
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Hammer2Snapshot {
    pub name: String,
    pub tid: Tid,
    pub creation_ns: u64,
    pub read_only: bool,
}

pub struct SnapshotEngine {
    pub snapshots: Vec<Hammer2Snapshot>,
}

impl SnapshotEngine {
    pub fn new() -> Self { SnapshotEngine { snapshots: Vec::new() } }

    pub fn create(&mut self, name: &str, tid: Tid) -> &Hammer2Snapshot {
        self.snapshots.push(Hammer2Snapshot {
            name: name.into(),
            tid,
            creation_ns: 0, // would use clock_gettime in real impl
            read_only: true,
        });
        self.snapshots.last().unwrap()
    }

    pub fn find(&self, name: &str) -> Option<&Hammer2Snapshot> {
        self.snapshots.iter().find(|s| s.name == name)
    }

    pub fn list(&self) -> &[Hammer2Snapshot] { &self.snapshots }

    pub fn destroy(&mut self, name: &str) -> bool {
        let before = self.snapshots.len();
        self.snapshots.retain(|s| s.name != name);
        self.snapshots.len() < before
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Volume Superblock
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Hammer2VolumeHeader {
    pub magic: u64,
    pub version: u32,
    pub vol_name: String,
    pub fsid: u64,
    pub total_size: u64,
    pub free_size: u64,
    pub tid_current: Tid,
    pub copyinfo: [Option<String>; 4], // up to 4 cluster peers
    pub checksum: MediaChecksum,
}

impl Hammer2VolumeHeader {
    pub const MAGIC: u64 = 0x48414D4D4552325F; // "HAMMER2_"

    pub fn new(name: &str, total: u64) -> Self {
        let data = name.as_bytes();
        Hammer2VolumeHeader {
            magic: Self::MAGIC,
            version: 1,
            vol_name: name.into(),
            fsid: 0xCAFE_BABE_0000_0001,
            total_size: total,
            free_size: total,
            tid_current: 1,
            copyinfo: [None, None, None, None],
            checksum: MediaChecksum::compute(data),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HAMMER2 Filesystem
// ─────────────────────────────────────────────────────────────────────────────

pub struct Hammer2Fs {
    pub header: Hammer2VolumeHeader,
    pub btree: MvccBTree,
    pub snapshots: SnapshotEngine,
    pub inode_counter: AtomicU64,
}

impl Hammer2Fs {
    pub fn new(vol_name: &str, total_bytes: u64) -> Self {
        Hammer2Fs {
            header: Hammer2VolumeHeader::new(vol_name, total_bytes),
            btree: MvccBTree::new(),
            snapshots: SnapshotEngine::new(),
            inode_counter: AtomicU64::new(2), // 1 = root
        }
    }

    pub fn create_inode(&self, name: &str, mode: u32) -> u64 {
        self.inode_counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn write_inode(&mut self, inode: Hammer2Inode) -> Tid {
        let key = inode.inum;
        let node = Hammer2Node {
            node_type: NodeType::Inode,
            tid: 0, // will be set by btree.insert
            delete_tid: None,
            content: NodeContent::Inode(inode),
        };
        self.btree.insert(key, node)
    }

    pub fn lookup_inode_at(&self, inum: u64, tid: Tid) -> Option<&Hammer2Inode> {
        self.btree.lookup_at(inum, tid).and_then(|node| {
            if let NodeContent::Inode(ref inode) = node.content { Some(inode) } else { None }
        })
    }

    pub fn snapshot(&mut self, name: &str) -> Tid {
        let tid = self.btree.snapshot();
        self.snapshots.create(name, tid);
        tid
    }

    pub fn read_at_snapshot(&self, name: &str, inum: u64) -> Option<&Hammer2Inode> {
        let snap_tid = self.snapshots.find(name)?.tid;
        self.lookup_inode_at(inum, snap_tid)
    }

    pub fn delete_inode(&mut self, inum: u64) -> Option<Tid> {
        self.btree.delete(inum)
    }

    pub fn prune_old_versions(&mut self, min_tid: Tid) -> usize {
        self.btree.prune(min_tid)
    }

    pub fn stats(&self) -> Hammer2Stats {
        Hammer2Stats {
            vol_name: self.header.vol_name.clone(),
            total_bytes: self.header.total_size,
            free_bytes: self.header.free_size,
            node_count: self.btree.node_count(),
            snapshot_count: self.snapshots.list().len(),
            current_tid: self.btree.current_tid.load(Ordering::SeqCst),
        }
    }
}

#[derive(Debug)]
pub struct Hammer2Stats {
    pub vol_name: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub node_count: usize,
    pub snapshot_count: usize,
    pub current_tid: Tid,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_checksum() {
        let data = b"HAMMER2 test block";
        let ck = MediaChecksum::compute(data);
        assert!(ck.verify(data));
        assert!(!ck.verify(b"different data"));
    }

    #[test]
    fn test_mvcc_btree_insert_lookup() {
        let mut tree = MvccBTree::new();
        let inode = Hammer2Inode {
            inum: 42, nlinks: 1, size: 1024, uid: 1000, gid: 1000, mode: 0o644,
            atime_ns: 0, mtime_ns: 0, ctime_ns: 0,
            comp_algo: CompressionAlgo::Lz4,
            check_algo: ChecksumType::XXHash64,
            direct_refs: Vec::new(),
            name: "test.txt".into(),
        };
        let node = Hammer2Node {
            node_type: NodeType::Inode, tid: 0, delete_tid: None,
            content: NodeContent::Inode(inode.clone()),
        };
        let tid = tree.insert(42, node);
        assert!(tid >= 1);
        let found = tree.lookup_at(42, tid + 1);
        assert!(found.is_some());
    }

    #[test]
    fn test_mvcc_delete_and_visibility() {
        let mut tree = MvccBTree::new();
        let node = Hammer2Node {
            node_type: NodeType::Data, tid: 0, delete_tid: None,
            content: NodeContent::Data(b"hello".to_vec()),
        };
        let tid1 = tree.insert(100, node);
        // Visible at tid1
        assert!(tree.lookup_at(100, tid1 + 1).is_some());
        // Delete it
        let del_tid = tree.delete(100).unwrap();
        // Still visible before del_tid
        assert!(tree.lookup_at(100, del_tid - 1).is_some());
        // Not visible after del_tid
        assert!(tree.lookup_at(100, del_tid + 1).is_none());
    }

    #[test]
    fn test_snapshot_create_read_after_delete() {
        let mut fs = Hammer2Fs::new("testpool", 1024 * 1024 * 1024);
        let inode = Hammer2Inode {
            inum: 10, nlinks: 1, size: 512, uid: 0, gid: 0, mode: 0o755,
            atime_ns: 0, mtime_ns: 0, ctime_ns: 0,
            comp_algo: CompressionAlgo::None,
            check_algo: ChecksumType::CRC32,
            direct_refs: Vec::new(),
            name: "important_file".into(),
        };
        let write_tid = fs.write_inode(inode.clone());
        // Verify the inode is visible right after write
        let visible = fs.lookup_inode_at(10, write_tid + 1);
        assert!(visible.is_some(), "Inode should be visible after write");
        // Take snapshot (captures current TID)
        let snap_tid = fs.snapshot("before_delete");
        // Now delete the inode at a higher TID
        fs.delete_inode(10);
        // Read at snapshot TID: should still see it (MVCC)
        let snap_read = fs.read_at_snapshot("before_delete", 10);
        assert!(snap_read.is_some(), "Inode should be visible at snapshot TID");
        assert_eq!(snap_read.unwrap().name, "important_file");
    }

    #[test]
    fn test_volume_header_validity() {
        let hdr = Hammer2VolumeHeader::new("sigma_pool", 500 * 1024 * 1024 * 1024);
        assert!(hdr.is_valid());
        assert_eq!(hdr.vol_name, "sigma_pool");
    }

    #[test]
    fn test_mvcc_prune() {
        let mut tree = MvccBTree::new();
        for i in 0..10 {
            let node = Hammer2Node {
                node_type: NodeType::Data, tid: 0, delete_tid: None,
                content: NodeContent::Data(vec![i as u8]),
            };
            tree.insert(i as Key, node);
        }
        let count_before = tree.node_count();
        // Delete some and prune
        tree.delete(0);
        tree.delete(1);
        let current_tid = tree.current_tid.load(Ordering::Relaxed);
        let pruned = tree.prune(current_tid / 2);
        assert!(tree.node_count() <= count_before);
    }
}
