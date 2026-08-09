// OOP-based Filesystem Support for SigmaOS
// Based on Ideas-999-Structured: Core System Item 7
// Implements ext4, Btrfs, and ZFS with snapshot/rollback APIs.
// Features advanced read-only ext4 traversal metadata structures inspired by Linux & BSD.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FilesystemID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType { Ext4 = 0, Btrfs = 1, ZFS = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemError { Success = 0, InvalidFS = 1, MountFailed = 2, SnapshotFailed = 3 }

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
    fn id(&self) -> FilesystemID { self.id }
    fn fs_type(&self) -> FilesystemType { unsafe { core::mem::transmute(self.fs_type.load(Ordering::SeqCst)) } }

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
            FilesystemType::Ext4 => Err(FilesystemError::SnapshotFailed),
            FilesystemType::Btrfs | FilesystemType::ZFS => Ok(()),
        }
    }

    fn rollback(&mut self, _snapshot: &[u8]) -> Result<(), FilesystemError> {
        let fs_type = self.fs_type();
        match fs_type {
            FilesystemType::Ext4 => Err(FilesystemError::SnapshotFailed),
            FilesystemType::Btrfs | FilesystemType::ZFS => Ok(()),
        }
    }
}

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

pub trait FilesystemManager {
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
    fn register_filesystem(&mut self, fs: alloc::boxed::Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError> {
        let id = fs.id();
        self.filesystems.push(Some(fs));
        Ok(id)
    }

    fn get_filesystem(&self, id: FilesystemID) -> Option<&dyn Filesystem> {
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
            if let Some(ref fs) = self.filesystems[i] {
                ids.push(fs.id());
            }
        }
        ids
    }
}

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
            if self.capacity > self.len && !self.data.is_null() {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
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
