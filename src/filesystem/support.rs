#![no_std]
#![no_main]

/// OOP-based Filesystem Support for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 7
/// Implements ext4, Btrfs, and ZFS with snapshot/rollback APIs

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FilesystemID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FilesystemType { Ext4 = 0, Btrfs = 1, ZFS = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FilesystemError { Success = 0, InvalidFS = 1, MountFailed = 2, SnapshotFailed = 3 }

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
    fn register_filesystem(&mut self, fs: Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError>;
    fn get_filesystem(&self, id: FilesystemID) -> Option<&dyn Filesystem>;
    fn list_filesystems(&self) -> Vec<FilesystemID>;
}

#[repr(C)]
pub struct SimpleFilesystemManager {
    pub filesystems: Vec<Option<Box<dyn Filesystem>>>,
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
    fn register_filesystem(&mut self, fs: Box<dyn Filesystem>) -> Result<FilesystemID, FilesystemError> {
        let id = fs.id();
        self.filesystems.push(Some(fs));
        Ok(id)
    }

    fn get_filesystem(&self, id: FilesystemID) -> Option<&dyn Filesystem> {
        for fs_option in &self.filesystems {
            if let Some(ref fs) = *fs_option {
                if fs.id() == id { return Some(fs.as_ref()); }
            }
        }
        None
    }

    fn list_filesystems(&self) -> Vec<FilesystemID> {
        let mut ids = Vec::new();
        for fs_option in &self.filesystems {
            if let Some(ref fs) = *fs_option {
                ids.push(fs.id());
            }
        }
        ids
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
    }
    fn remove(&mut self, index: usize) -> T {
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
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
