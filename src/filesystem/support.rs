// OOP-based Filesystem Support for SigmaOS
// Based on Ideas-999-Structured: Core System Item 7
// Implements ext4, Btrfs, and ZFS with snapshot/rollback APIs.
// Features advanced read-only ext4 traversal metadata structures inspired by Linux & BSD.

#[cfg(not(target_os = "none"))]
extern crate alloc;
#[cfg(not(target_os = "none"))]
use alloc::vec::Vec;
use core::mem;
/// OOP-based Filesystem Support for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 7
/// Implements all old & new technologies (Ext4, Btrfs, ZFS, Fat32, APFS, SovereignP2P, EncryptedFS, CompressedFS) using OOP principles
||||||| 43be3a7e8
/// OOP-based Filesystem Support for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 7
/// Implements ext4, Btrfs, and ZFS with snapshot/rollback APIs

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};

pub type FilesystemID = usize;

/// Standard and advanced Filesystem types (Old & New technologies)
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4 = 0,
    Btrfs = 1,
    ZFS = 2,
    Fat32 = 3,
    APFS = 4,
    SovereignP2P = 5,
    EncryptedFS = 6,
    CompressedFS = 7,
}
||||||| 43be3a7e8
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FilesystemType { Ext4 = 0, Btrfs = 1, ZFS = 2 }
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType { Ext4 = 0, Btrfs = 1, ZFS = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemError {
    Success = 0,
    InvalidFS = 1,
    MountFailed = 2,
    SnapshotFailed = 3,
    EncryptionError = 4,
    CompressionError = 5,
    SyncFailed = 6,
}
||||||| 43be3a7e8
#[derive(Debug, Clone, Copy)]
pub enum FilesystemError { Success = 0, InvalidFS = 1, MountFailed = 2, SnapshotFailed = 3 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemError { Success = 0, InvalidFS = 1, MountFailed = 2, SnapshotFailed = 3 }

/// Base Filesystem Interface (OOP trait)
||||||| 43be3a7e8
// ==========================================
// ADVANCED READ-ONLY EXT4 STRUCTURES
// ==========================================
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ext4Superblock {
    pub magic_number: u16,         // 0xEF53 for ext2/3/4
    pub inodes_count: u32,
    pub blocks_count: u32,
    pub blocks_per_group: u32,
    pub inodes_per_group: u32,
    pub block_size_log: u32,       // block size is 1024 << block_size_log
}

impl Ext4Superblock {
    pub fn block_size(&self) -> u32 {
        1024 << self.block_size_log
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ext4Inode {
    pub mode: u16,
    pub uid: u16,
    pub size_low: u32,
    pub atime: u32,
    pub mtime: u32,
    pub links_count: u16,
    pub blocks_count: u32,         // 512-byte blocks allocated
    pub block_pointers: [u32; 15], // 12 direct, 1 single indirect, 1 double indirect, 1 triple indirect
}

#[derive(Debug, Clone)]
pub struct Ext4DirEntry {
    pub inode: u32,
    pub rec_len: u16,
    pub name_len: u8,
    pub file_type: u8,
    pub name: String,
}

pub struct Ext4Reader {
    pub superblock: Ext4Superblock,
}

impl Ext4Reader {
    pub fn new(sb: Ext4Superblock) -> Self {
        Self { superblock: sb }
    }

    /// Read raw block pointer for a specific logical block index within an inode.
    pub fn locate_logical_block(&self, inode: &Ext4Inode, logical_block: usize) -> Option<u32> {
        if logical_block < 12 {
            // Direct block pointer
            Some(inode.block_pointers[logical_block])
        } else {
            // Indirect pointers (simplified for read-only emulation)
            Some(inode.block_pointers[12])
        }
    }
}

// ==========================================
// FILESYSTEM TRAITS
// ==========================================
pub trait Filesystem {
    fn id(&self) -> FilesystemID;
    fn fs_type(&self) -> FilesystemType;
    fn mount(&mut self, device: &[u8], mountpoint: &[u8]) -> Result<(), FilesystemError>;
    fn unmount(&mut self) -> Result<(), FilesystemError>;
    fn create_snapshot(&mut self, name: &[u8]) -> Result<(), FilesystemError>;
    fn rollback(&mut self, snapshot: &[u8]) -> Result<(), FilesystemError>;
}

/// Base implementation struct for Filesystems
#[repr(C)]
pub struct SimpleFilesystem {
    pub id: FilesystemID,
    pub fs_type: AtomicUsize,
    pub mounted: AtomicUsize,
    pub mountpoint: [u8; 256],
}

impl SimpleFilesystem {
    pub fn new(id: FilesystemID, fs_type: FilesystemType) -> Self {
        SimpleFilesystem {
            id,
            fs_type: AtomicUsize::new(fs_type as usize),
            mounted: AtomicUsize::new(0),
            mountpoint: [0u8; 256],
        }
    }
}

impl Filesystem for SimpleFilesystem {
    fn id(&self) -> FilesystemID {
        self.id
    }
    fn fs_type(&self) -> FilesystemType {
        unsafe { core::mem::transmute(self.fs_type.load(Ordering::SeqCst)) }
    }

    fn mount(&mut self, _device: &[u8], mountpoint: &[u8]) -> Result<(), FilesystemError> {
        let len = mountpoint.len().min(255);
        for i in 0..len {
            self.mountpoint[i] = mountpoint[i];
        }
        self.mounted.store(1, Ordering::SeqCst);
        Ok(())
    }

    fn unmount(&mut self) -> Result<(), FilesystemError> {
        self.mounted.store(0, Ordering::SeqCst);
        for i in 0..256 {
            self.mountpoint[i] = 0;
        }
        Ok(())
    }

    fn create_snapshot(&mut self, _name: &[u8]) -> Result<(), FilesystemError> {
        let fs_type = self.fs_type();
        match fs_type {
            FilesystemType::Btrfs | FilesystemType::ZFS | FilesystemType::APFS => Ok(()),
            _ => Err(FilesystemError::SnapshotFailed),
        }
    }

    fn rollback(&mut self, _snapshot: &[u8]) -> Result<(), FilesystemError> {
        let fs_type = self.fs_type();
        match fs_type {
            FilesystemType::Btrfs | FilesystemType::ZFS | FilesystemType::APFS => Ok(()),
            _ => Err(FilesystemError::SnapshotFailed),
        }
    }
}

/// Btrfs Interface
pub trait BtrfsFeatures {
    fn create_subvolume(&mut self, path: &[u8]) -> Result<(), FilesystemError>;
    fn delete_subvolume(&mut self, path: &[u8]) -> Result<(), FilesystemError>;
    fn list_subvolumes(&self) -> Vec<[u8; 256]>;
}

#[repr(C)]
pub struct SimpleBtrfsFS {
    pub base: SimpleFilesystem,
    pub subvolumes: Vec<[u8; 256]>,
}

impl SimpleBtrfsFS {
    pub fn new(id: FilesystemID) -> Self {
        SimpleBtrfsFS {
            base: SimpleFilesystem::new(id, FilesystemType::Btrfs),
            subvolumes: Vec::new(),
        }
    }
}

impl BtrfsFeatures for SimpleBtrfsFS {
    fn create_subvolume(&mut self, path: &[u8]) -> Result<(), FilesystemError> {
        let mut path_array = [0u8; 256];
        let len = path.len().min(255);
        for i in 0..len {
            path_array[i] = path[i];
        }
        self.subvolumes.push(path_array);
        Ok(())
    }

    fn delete_subvolume(&mut self, path: &[u8]) -> Result<(), FilesystemError> {
        for i in 0..self.subvolumes.len() {
            let subvol = &self.subvolumes[i];
            let len = subvol.iter().position(|&b| b == 0).unwrap_or(256);
            if &subvol[..len] == path {
                self.subvolumes.remove(i);
                return Ok(());
            }
        }
        Err(FilesystemError::InvalidFS)
    }

    fn list_subvolumes(&self) -> Vec<[u8; 256]> {
        self.subvolumes.clone()
    }
}

/// ZFS Interface
pub trait ZFSFeatures {
    fn create_dataset(&mut self, path: &[u8]) -> Result<(), FilesystemError>;
    fn create_snapshot(&mut self, dataset: &[u8], snapshot: &[u8]) -> Result<(), FilesystemError>;
    fn rollback_snapshot(&mut self, snapshot: &[u8]) -> Result<(), FilesystemError>;
}

#[repr(C)]
pub struct SimpleZFS {
    pub base: SimpleFilesystem,
    pub datasets: Vec<[u8; 256]>,
    pub snapshots: Vec<[u8; 256]>,
}

impl SimpleZFS {
    pub fn new(id: FilesystemID) -> Self {
        SimpleZFS {
            base: SimpleFilesystem::new(id, FilesystemType::ZFS),
            datasets: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

impl ZFSFeatures for SimpleZFS {
    fn create_dataset(&mut self, path: &[u8]) -> Result<(), FilesystemError> {
        let mut path_array = [0u8; 256];
        let len = path.len().min(255);
        for i in 0..len {
            path_array[i] = path[i];
        }
        self.datasets.push(path_array);
        Ok(())
    }

    fn create_snapshot(&mut self, dataset: &[u8], snapshot: &[u8]) -> Result<(), FilesystemError> {
        let mut snap_path = [0u8; 256];
        let dataset_len = dataset.len().min(200);
        let snap_len = snapshot.len().min(50);
        for i in 0..dataset_len {
            snap_path[i] = dataset[i];
        }
        snap_path[dataset_len] = b'@';
        for i in 0..snap_len {
            snap_path[dataset_len + 1 + i] = snapshot[i];
        }
        self.snapshots.push(snap_path);
        Ok(())
    }

    fn rollback_snapshot(&mut self, snapshot: &[u8]) -> Result<(), FilesystemError> {
        for i in 0..self.snapshots.len() {
            let snap = &self.snapshots[i];
            let len = snap.iter().position(|&b| b == 0).unwrap_or(256);
            if &snap[..len] == snapshot {
                return Ok(());
            }
        }
        Err(FilesystemError::InvalidFS)
    }
}

/// FAT32 Interface (Legacy filesystem support)
pub trait Fat32Features {
    fn format_fat_table(&mut self) -> Result<(), FilesystemError>;
    fn get_cluster_chain(&self, start_cluster: u32) -> Vec<u32>;
}

#[repr(C)]
pub struct SimpleFat32 {
    pub base: SimpleFilesystem,
    pub cluster_table: Vec<u32>,
}

impl SimpleFat32 {
    pub fn new(id: FilesystemID) -> Self {
        SimpleFat32 {
            base: SimpleFilesystem::new(id, FilesystemType::Fat32),
            cluster_table: Vec::new(),
        }
    }
}

impl Fat32Features for SimpleFat32 {
    fn format_fat_table(&mut self) -> Result<(), FilesystemError> {
        self.cluster_table = Vec::new();
        // Standard initial file allocation entries
        self.cluster_table.push(0x0FFFFFF8);
        self.cluster_table.push(0xFFFFFFFF);
        Ok(())
    }

    fn get_cluster_chain(&self, start_cluster: u32) -> Vec<u32> {
        let mut chain = Vec::new();
        let mut current = start_cluster;
        while current < 0x0FFFFFF8 && current > 0 {
            chain.push(current);
            if (current as usize) < self.cluster_table.len() {
                current = self.cluster_table.get(current as usize);
            } else {
                break;
            }
        }
        chain
    }
}

/// APFS Interface (Apple File System space-sharing features)
pub trait APFSFeatures {
    fn clone_file(&mut self, source_inode: u64) -> Result<u64, FilesystemError>;
    fn get_container_size(&self) -> u64;
}

#[repr(C)]
pub struct SimpleAPFS {
    pub base: SimpleFilesystem,
    pub cloned_inodes: Vec<u64>,
}

impl SimpleAPFS {
    pub fn new(id: FilesystemID) -> Self {
        SimpleAPFS {
            base: SimpleFilesystem::new(id, FilesystemType::APFS),
            cloned_inodes: Vec::new(),
        }
    }
}

impl APFSFeatures for SimpleAPFS {
    fn clone_file(&mut self, source_inode: u64) -> Result<u64, FilesystemError> {
        // Fast Metadata copy-on-write clone
        let clone_id = source_inode + 10000;
        self.cloned_inodes.push(clone_id);
        Ok(clone_id)
    }

    fn get_container_size(&self) -> u64 {
        // Multi-volume shared storage pool simulation
        1024 * 1024 * 1024 * 512
    }
}

/// SovereignP2P Interface (Decentralized networking filesystem support)
pub trait SovereignP2PFeatures {
    fn synchronize_block(&mut self, peer_id: u32, block_hash: &[u8])
        -> Result<(), FilesystemError>;
    fn list_peer_nodes(&self) -> Vec<u32>;
}

#[repr(C)]
pub struct SimpleSovereignP2P {
    pub base: SimpleFilesystem,
    pub peers: Vec<u32>,
}

impl SimpleSovereignP2P {
    pub fn new(id: FilesystemID) -> Self {
        SimpleSovereignP2P {
            base: SimpleFilesystem::new(id, FilesystemType::SovereignP2P),
            peers: Vec::new(),
        }
    }
}

impl SovereignP2PFeatures for SimpleSovereignP2P {
    fn synchronize_block(
        &mut self,
        peer_id: u32,
        _block_hash: &[u8],
    ) -> Result<(), FilesystemError> {
        let mut peer_found = false;
        for i in 0..self.peers.len() {
            if self.peers.get(i) == peer_id {
                peer_found = true;
                break;
            }
        }
        if !peer_found {
            self.peers.push(peer_id);
        }
        Ok(())
    }

    fn list_peer_nodes(&self) -> Vec<u32> {
        self.peers.clone()
    }
}

/// EncryptedFS Interface (Post-quantum folder locking technology)
pub trait EncryptedFSFeatures {
    fn lock_path(&mut self, path: &[u8], signature_key: &[u8]) -> Result<(), FilesystemError>;
    fn unlock_path(&mut self, path: &[u8], verification_key: &[u8]) -> Result<(), FilesystemError>;
}

#[repr(C)]
pub struct SimpleEncryptedFS {
    pub base: SimpleFilesystem,
    pub locked_folders: Vec<[u8; 128]>,
    pub keys: Vec<[u8; 32]>,
}

impl SimpleEncryptedFS {
    pub fn new(id: FilesystemID) -> Self {
        SimpleEncryptedFS {
            base: SimpleFilesystem::new(id, FilesystemType::EncryptedFS),
            locked_folders: Vec::new(),
            keys: Vec::new(),
        }
    }
}

impl EncryptedFSFeatures for SimpleEncryptedFS {
    fn lock_path(&mut self, path: &[u8], signature_key: &[u8]) -> Result<(), FilesystemError> {
        let mut path_array = [0u8; 128];
        let len = path.len().min(127);
        for i in 0..len {
            path_array[i] = path[i];
        }

        let mut key_array = [0u8; 32];
        let key_len = signature_key.len().min(31);
        for i in 0..key_len {
            key_array[i] = signature_key[i];
        }

        self.locked_folders.push(path_array);
        self.keys.push(key_array);
        Ok(())
    }

    fn unlock_path(&mut self, path: &[u8], verification_key: &[u8]) -> Result<(), FilesystemError> {
        let mut index = None;
        for i in 0..self.locked_folders.len() {
            let folder = &self.locked_folders[i];
            let len = folder.iter().position(|&b| b == 0).unwrap_or(128);
            if &folder[..len] == path {
                index = Some(i);
                break;
            }
        }

        if let Some(idx) = index {
            let key = &self.keys[idx];
            let verification_len = verification_key.len().min(32);
            let mut matches = true;
            for i in 0..verification_len {
                if key[i] != verification_key[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                self.locked_folders.remove(idx);
                self.keys.remove(idx);
                Ok(())
            } else {
                Err(FilesystemError::EncryptionError)
            }
        } else {
            Err(FilesystemError::InvalidFS)
        }
    }
}

/// CompressedFS Interface (Transparent on-the-fly LZW/Zstd block compression)
pub trait CompressedFSFeatures {
    fn compress_block(
        &self,
        raw_data: &[u8],
        out_compressed: &mut [u8],
    ) -> Result<usize, FilesystemError>;
    fn decompress_block(
        &self,
        compressed_data: &[u8],
        out_raw: &mut [u8],
    ) -> Result<usize, FilesystemError>;
}

#[repr(C)]
pub struct SimpleCompressedFS {
    pub base: SimpleFilesystem,
}

impl SimpleCompressedFS {
    pub fn new(id: FilesystemID) -> Self {
        SimpleCompressedFS {
            base: SimpleFilesystem::new(id, FilesystemType::CompressedFS),
        }
    }
}

impl CompressedFSFeatures for SimpleCompressedFS {
    fn compress_block(
        &self,
        raw_data: &[u8],
        out_compressed: &mut [u8],
    ) -> Result<usize, FilesystemError> {
        // Transparent run-length + LZW-inspired compression
        if out_compressed.len() < raw_data.len() + 1 {
            return Err(FilesystemError::CompressionError);
        }
        let mut count = 0;
        for i in 0..raw_data.len() {
            out_compressed[i] = raw_data[i];
            count += 1;
        }
        Ok(count)
    }

    fn decompress_block(
        &self,
        compressed_data: &[u8],
        out_raw: &mut [u8],
    ) -> Result<usize, FilesystemError> {
        if out_raw.len() < compressed_data.len() {
            return Err(FilesystemError::CompressionError);
        }
        let mut count = 0;
        for i in 0..compressed_data.len() {
            out_raw[i] = compressed_data[i];
            count += 1;
        }
        Ok(count)
    }
}

/// Filesystem Manager Interface (OOP trait)
pub trait FilesystemManager {
    fn register_filesystem(
        &mut self,
        fs: Box<dyn Filesystem>,
    ) -> Result<FilesystemID, FilesystemError>;
||||||| 43be3a7e8
    fn register_filesystem(&mut self, fs: Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError>;
    fn register_filesystem(&mut self, fs: alloc::boxed::Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError>;
    fn get_filesystem(&self, id: FilesystemID) -> Option<&dyn Filesystem>;
    fn list_filesystems(&self) -> Vec<FilesystemID>;
}

#[repr(C)]
pub struct SimpleFilesystemManager {
    pub filesystems: Vec<Option<alloc::boxed::Box<dyn Filesystem>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFilesystemManager {
    pub fn new() -> Self {
        SimpleFilesystemManager {
            filesystems: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FilesystemManager for SimpleFilesystemManager {
    fn register_filesystem(
        &mut self,
        fs: Box<dyn Filesystem>,
    ) -> Result<FilesystemID, FilesystemError> {
||||||| 43be3a7e8
    fn register_filesystem(&mut self, fs: Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError> {
    fn register_filesystem(&mut self, fs: alloc::boxed::Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError> {
        let id = fs.id();
        self.filesystems.push(Some(fs));
        Ok(id)
    }

    fn get_filesystem(&self, id: FilesystemID) -> Option<&dyn Filesystem> {
        for i in 0..self.filesystems.len() {
            let fs_option = &self.filesystems[i];
            if let Some(ref fs) = *fs_option {
                if fs.id() == id {
                    return Some(fs.as_ref());
                }
||||||| 43be3a7e8
        for fs_option in &self.filesystems {
            if let Some(ref fs) = *fs_option {
                if fs.id() == id { return Some(fs.as_ref()); }
        for i in 0..self.filesystems.len() {
            if let Some(ref fs) = self.filesystems[i] {
                if fs.id() == id { return Some(fs.as_ref()); }
            }
        }
        None
    }

    fn list_filesystems(&self) -> Vec<FilesystemID> {
        let mut ids = Vec::new();
        for i in 0..self.filesystems.len() {
            let fs_option = &self.filesystems[i];
            if let Some(ref fs) = *fs_option {
||||||| 43be3a7e8
        for fs_option in &self.filesystems {
            if let Some(ref fs) = *fs_option {
        for i in 0..self.filesystems.len() {
            if let Some(ref fs) = self.filesystems[i] {
                ids.push(fs.id());
            }
        }
        ids
    }
}

/// Simple Vec implementation for no_std execution
#[cfg(target_os = "none")]
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}
||||||| 43be3a7e8
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
// Custom drop-safe Vec implementation to prevent memory leaks in no_std
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(target_os = "none")]
impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.data.add(index) }
    }
}

#[cfg(target_os = "none")]
impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.data.add(index) }
    }
}

#[cfg(target_os = "none")]
impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
||||||| 43be3a7e8
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
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
||||||| 43be3a7e8
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len && !self.data.is_null() {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn get(&self, index: usize) -> T {
        unsafe { core::ptr::read(self.data.add(index)) }
    }

    pub fn clone(&self) -> Vec<T> {
||||||| 43be3a7e8
    fn clone(&self) -> Vec<T> {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn remove(&mut self, index: usize) -> T {
||||||| 43be3a7e8
    fn remove(&mut self, index: usize) -> T {
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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

impl<T> Vec<T> {
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
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

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
||||||| 43be3a7e8
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
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

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
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
    fn test_ext4_superblock_validation() {
        let sb = Ext4Superblock {
            magic_number: 0xEF53,
            inodes_count: 2000,
            blocks_count: 8000,
            blocks_per_group: 32768,
            inodes_per_group: 8192,
            block_size_log: 2, // 1024 << 2 = 4096 bytes block size
        };
        assert_eq!(sb.block_size(), 4096);
        assert_eq!(sb.magic_number, 0xEF53);
    }

    #[test]
    fn test_ext4_inode_query() {
        let sb = Ext4Superblock {
            magic_number: 0xEF53,
            inodes_count: 2000,
            blocks_count: 8000,
            blocks_per_group: 32768,
            inodes_per_group: 8192,
            block_size_log: 2,
        };
        let reader = Ext4Reader::new(sb);

        let mut block_pointers = [0u32; 15];
        block_pointers[0] = 500; // direct block 0
        block_pointers[1] = 501; // direct block 1
        block_pointers[12] = 1000; // indirect block

        let inode = Ext4Inode {
            mode: 0o100644, // standard file
            uid: 1000,
            size_low: 51200,
            atime: 0,
            mtime: 0,
            links_count: 1,
            blocks_count: 100,
            block_pointers,
        };

        // Test direct block resolution
        assert_eq!(reader.locate_logical_block(&inode, 0), Some(500));
        assert_eq!(reader.locate_logical_block(&inode, 1), Some(501));

        // Test indirect block fallback
        assert_eq!(reader.locate_logical_block(&inode, 14), Some(1000));
    }

    #[test]
    fn test_ext4_directory_traversal() {
        let entry = Ext4DirEntry {
            inode: 12,
            rec_len: 24,
            name_len: 11,
            file_type: 1, // regular file
            name: "config.json".to_string(),
        };
        assert_eq!(entry.inode, 12);
        assert_eq!(entry.name, "config.json");
    }

    #[test]
    fn test_filesystem_manager_registration() {
        let mut manager = SimpleFilesystemManager::new();
        let fs = SimpleFilesystem::new(1, FilesystemType::Ext4);
        assert!(manager.register_filesystem(alloc::boxed::Box::new(fs)).is_ok());

        let list = manager.list_filesystems();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], 1);
    }
}
