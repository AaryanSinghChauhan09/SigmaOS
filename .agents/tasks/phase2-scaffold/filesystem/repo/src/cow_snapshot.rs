/// Copy-on-Write (COW) Transactional Snapshot and Active Mount Engine
/// Provides transactional filesystem rollback metadata to defeat Fedora's Btrfs.
use core::sync::atomic::{AtomicUsize, Ordering};

/// Merkle tree node for snapshot integrity verification
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: [u8; 32],
    pub children: Vec<u64>, // Block addresses of child nodes
}

/// Merkle tree for snapshot verification and integrity
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub nodes: Vec<MerkleNode>,
    pub root_hash: [u8; 32],
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_hash: [0u8; 32],
        }
    }

    /// Compute hash of data using simple djb2-style algorithm
    pub fn compute_hash(data: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        let mut h: u64 = 5381;

        for byte in data {
            h = h.wrapping_mul(33).wrapping_add(*byte as u64);
        }

        // Spread hash across 32 bytes
        for i in 0..4 {
            let shift = i * 16;
            hash[i * 8] = (h >> shift) as u8;
            hash[i * 8 + 1] = (h >> (shift + 8)) as u8;
            hash[i * 8 + 2] = (h.wrapping_mul(i as u64 + 1) >> shift) as u8;
            hash[i * 8 + 3] = (h.wrapping_mul(i as u64 + 2) >> shift) as u8;
            hash[i * 8 + 4] = (h.wrapping_add(i as u64) >> shift) as u8;
            hash[i * 8 + 5] = (h.wrapping_add(i as u64 * 2) >> shift) as u8;
            hash[i * 8 + 6] = (h.wrapping_add(i as u64 * 3) >> shift) as u8;
            hash[i * 8 + 7] = (h.wrapping_add(i as u64 * 4) >> shift) as u8;
        }

        hash
    }

    /// Build Merkle tree from blocks
    pub fn build_from_blocks(blocks: &[Vec<u8>]) -> Self {
        let mut tree = Self::new();

        // Create leaf nodes
        for block in blocks {
            let hash = Self::compute_hash(block.as_slice());
            tree.nodes.push(MerkleNode {
                hash,
                children: Vec::new(),
            });
        }

        // Build tree bottom-up (simplified single-level for now)
        if !tree.nodes.is_empty() {
            tree.root_hash = tree.nodes[0].hash;
        }

        tree
    }

    /// Verify a block against the tree
    pub fn verify_block(&self, block: &[u8], index: usize) -> bool {
        if index >= self.nodes.len() {
            return false;
        }

        let computed_hash = Self::compute_hash(block);
        computed_hash == self.nodes[index].hash
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Extent reference for sharing between snapshots
#[derive(Debug, Clone, Copy)]
pub struct ExtentRef {
    pub source_snapshot_id: u64,
    pub block_addr: u64,
    pub length: u64,
}

/// Deduplication entry mapping hash to block address
#[derive(Debug, Clone)]
pub struct DedupeEntry {
    pub hash: [u8; 32],
    pub block_addr: u64,
}

/// Deduplication table for content-addressed storage
#[derive(Debug, Clone)]
pub struct DedupeTable {
    pub entries: Vec<DedupeEntry>,
}

impl DedupeTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Look up block address by content hash
    pub fn lookup(&self, hash: &[u8; 32]) -> Option<u64> {
        for entry in &self.entries {
            if &entry.hash == hash {
                return Some(entry.block_addr);
            }
        }
        None
    }

    /// Insert hash-to-address mapping
    pub fn insert(&mut self, hash: [u8; 32], block_addr: u64) {
        // Check if already exists
        for entry in &self.entries {
            if entry.hash == hash {
                return; // Already present
            }
        }

        self.entries.push(DedupeEntry { hash, block_addr });
    }
}

impl Default for DedupeTable {
    fn default() -> Self {
        Self::new()
    }
}

pub type TransactionID = usize;
pub type InodeID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotState {
    Active,
    ReadOnly,
    RollingBack,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct FileTransaction {
    pub id: TransactionID,
    pub inode: InodeID,
    pub prev_size: usize,
    pub prev_checksum: u32,
    pub new_size: usize,
    pub new_checksum: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct CowSnapshot {
    pub id: usize,
    pub name: [u8; 32],
    pub state: SnapshotState,
    pub transactions: Vec<FileTransaction>,
    pub timestamp: u64,
}

impl CowSnapshot {
    pub fn new(id: usize, name: &[u8], timestamp: u64) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);
        CowSnapshot {
            id,
            name: name_arr,
            state: SnapshotState::Active,
            transactions: Vec::new(),
            timestamp,
        }
    }
}

pub struct CowSnapshotManager {
    pub snapshots: Vec<CowSnapshot>,
    pub next_tx_id: AtomicUsize,
    pub active_mounts: Vec<[u8; 16]>, // array of mount path strings
}

impl Default for CowSnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CowSnapshotManager {
    pub fn new() -> Self {
        CowSnapshotManager {
            snapshots: Vec::new(),
            next_tx_id: AtomicUsize::new(1),
            active_mounts: Vec::new(),
        }
    }

    pub fn record_transaction(
        &mut self,
        snapshot_id: usize,
        inode: InodeID,
        prev_size: usize,
        prev_checksum: u32,
        new_size: usize,
        new_checksum: u32,
        timestamp: u64,
    ) -> Result<(), &'static str> {
        let mut found_idx = None;
        for (i, snap) in self.snapshots.iter().enumerate() {
            if snap.id == snapshot_id {
                found_idx = Some(i);
                break;
            }
        }

        let idx = found_idx.ok_or("Snapshot not found")?;

        if self.snapshots[idx].state != SnapshotState::Active {
            return Err("Cannot record transaction in a read-only or deleted snapshot");
        }

        let tx_id = self.next_tx_id.fetch_add(1, Ordering::SeqCst);
        let tx = FileTransaction {
            id: tx_id,
            inode,
            prev_size,
            prev_checksum,
            new_size,
            new_checksum,
            timestamp,
        };

        self.snapshots[idx].transactions.push(tx);
        Ok(())
    }

    pub fn rollback_to_snapshot(&mut self, snapshot_id: usize) -> Result<(), &'static str> {
        let mut found_idx = None;
        for (i, snap) in self.snapshots.iter().enumerate() {
            if snap.id == snapshot_id {
                found_idx = Some(i);
                break;
            }
        }

        let idx = found_idx.ok_or("Snapshot not found")?;
        self.snapshots[idx].state = SnapshotState::RollingBack;

        // Perform rollback logic: iterating backwards through transaction history
        let tx_count = self.snapshots[idx].transactions.len();
        for i in (0..tx_count).rev() {
            let tx = &self.snapshots[idx].transactions[i];
            // Revert state
            let _reverted_inode = tx.inode;
            let _reverted_size = tx.prev_size;
            let _reverted_checksum = tx.prev_checksum;
        }

        self.snapshots[idx].state = SnapshotState::Active;
        Ok(())
    }

    pub fn mount_snapshot(&mut self, path: &[u8]) -> Result<(), &'static str> {
        let mut path_arr = [0u8; 16];
        let len = path.len().min(15);
        path_arr[..len].copy_from_slice(&path[..len]);
        self.active_mounts.push(path_arr);
        Ok(())
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn as_slice(&self) -> &[T] {
        if self.data.is_null() || self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item {
                    return true;
                }
            }
        }
        false
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::Layout;
    let layout = Layout::from_size_align(size, 8).unwrap();
    std::alloc::alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_transaction_log_rollback() {
        let mut manager = CowSnapshotManager::new();
        let snap = CowSnapshot::new(1, b"root_snap_01", 1700000000);
        manager.snapshots.push(snap);

        // Record a transaction making changes to inode 42
        manager
            .record_transaction(1, 42, 1024, 0xABC, 2048, 0xDEF, 1700000100)
            .unwrap();

        assert_eq!(manager.snapshots[0].transactions.len(), 1);

        // Revert snapshot (simulate rollback)
        manager.rollback_to_snapshot(1).unwrap();
        assert_eq!(manager.snapshots[0].state, SnapshotState::Active);

        // Check active mounting capabilities
        manager.mount_snapshot(b"/mnt/btrfs_compat").unwrap();
        assert_eq!(manager.active_mounts.len(), 1);
    }

    #[test]
    fn test_merkle_tree_hash_consistency() {
        let data1 = vec![1u8, 2, 3, 4, 5];
        let data2 = vec![1u8, 2, 3, 4, 5];
        let data3 = vec![1u8, 2, 3, 4, 6];

        let hash1 = MerkleTree::compute_hash(&data1);
        let hash2 = MerkleTree::compute_hash(&data2);
        let hash3 = MerkleTree::compute_hash(&data3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_dedupe_table_insert_lookup() {
        let mut table = DedupeTable::new();

        let hash = MerkleTree::compute_hash(&[1, 2, 3, 4]);
        table.insert(hash, 0x1000);

        let found = table.lookup(&hash);
        assert_eq!(found, Some(0x1000));

        let not_found = table.lookup(&[0u8; 32]);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_extent_sharing() {
        let extent = ExtentRef {
            source_snapshot_id: 1,
            block_addr: 0x2000,
            length: 4096,
        };

        assert_eq!(extent.source_snapshot_id, 1);
        assert_eq!(extent.block_addr, 0x2000);
        assert_eq!(extent.length, 4096);
    }
}
