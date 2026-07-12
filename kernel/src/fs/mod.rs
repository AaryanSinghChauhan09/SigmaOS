/// Virtual File System (VFS) Abstraction
/// This module provides the OOP-based traits for mounting and interacting with
/// diverse file systems (FAT32, Ext2, and the future Sovereign FS).

pub struct Box<T: ?Sized> {
    _ptr: *mut T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    AccessDenied,
    HardwareError,
    Unsupported,
}

/// Represents a generic File System.
pub trait FileSystem {
    /// Mount the file system on a given block device.
    fn mount(&mut self, device_id: usize) -> Result<(), FsError>;
    
    /// Unmount the file system.
    fn unmount(&mut self) -> Result<(), FsError>;

    /// Open a file and return a descriptor/inode.
    fn open(&self, path: &str) -> Result<Box<dyn Inode>, FsError>;
}

/// Represents a generic file or directory node (Inode).
pub trait Inode {
    /// Read data from the file into a buffer.
    fn read(&mut self, buffer: &mut [u8], offset: usize) -> Result<usize, FsError>;

    /// Write data from a buffer into the file.
    fn write(&mut self, data: &[u8], offset: usize) -> Result<usize, FsError>;
    
    /// Get the size of the file.
    fn size(&self) -> usize;

    /// Check if this inode is a directory.
    fn is_dir(&self) -> bool;
}

/// The central VFS Registry that tracks all mounted file systems.
pub struct VfsRegistry {
    // In a dynamic kernel, this maps mount points (e.g., "/") to Box<dyn FileSystem>
}

impl VfsRegistry {
    pub fn new() -> Self {
        Self {}
    }

    pub fn register_fs(&mut self, _name: &str, _fs: &dyn FileSystem) {
        // Registration logic
    }
}
