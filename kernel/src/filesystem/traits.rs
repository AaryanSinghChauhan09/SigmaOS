// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// OOP-based filesystem traits for SigmaOS
// Zero-allocation, performance-optimized filesystem interfaces

/// Core filesystem trait - all filesystems must implement this
pub trait Filesystem {
    /// Initialize the filesystem
    fn init(&mut self) -> Result<(), FsError>;
    
    /// Get filesystem name
    fn name(&self) -> &str;
    
    /// Get filesystem type
    fn fs_type(&self) -> FsType;
    
    /// Get total size
    fn total_size(&self) -> u64;
    
    /// Get available size
    fn available_size(&self) -> u64;
    
    /// Get used size
    fn used_size(&self) -> u64;
    
    /// Sync filesystem to storage
    fn sync(&mut self) -> Result<(), FsError>;
}

/// Inode trait for file/directory operations
pub trait Inode {
    /// Get inode number
    fn inode_number(&self) -> u64;
    
    /// Get inode type
    fn inode_type(&self) -> InodeType;
    
    /// Get permissions
    fn permissions(&self) -> u32;
    
    /// Set permissions
    fn set_permissions(&mut self, perms: u32) -> Result<(), FsError>;
    
    /// Get size
    fn size(&self) -> u64;
    
    /// Get owner
    fn owner(&self) -> u32;
    
    /// Get group
    fn group(&self) -> u32;
    
    /// Get modification time
    fn mtime(&self) -> u64;
    
    /// Set modification time
    fn set_mtime(&mut self, time: u64) -> Result<(), FsError>;
}

/// File trait for file operations
pub trait File: Inode {
    /// Read data from file
    fn read(&mut self, offset: u64, buffer: &mut [u8]) -> Result<usize, FsError>;
    
    /// Write data to file
    fn write(&mut self, offset: u64, buffer: &[u8]) -> Result<usize, FsError>;
    
    /// Truncate file
    fn truncate(&mut self, size: u64) -> Result<(), FsError>;
    
    /// Flush file to storage
    fn flush(&mut self) -> Result<(), FsError>;
    
    /// Get file position
    fn position(&self) -> u64;
    
    /// Set file position
    fn seek(&mut self, pos: u64) -> Result<(), FsError>;
}

/// Directory trait for directory operations
pub trait Directory: Inode {
    /// Open directory
    fn open(&mut self) -> Result<(), FsError>;
    
    /// Close directory
    fn close(&mut self) -> Result<(), FsError>;
    
    /// Read directory entry
    fn read_entry(&mut self) -> Result<Option<DirEntry>, FsError>;
    
    /// Create directory
    fn create(&mut self, name: &str, perms: u32) -> Result<(), FsError>;
    
    /// Remove directory
    fn remove(&mut self, name: &str) -> Result<(), FsError>;
    
    /// Lookup directory entry
    fn lookup(&self, name: &str) -> Result<u64, FsError>;
    
    /// Get directory entry count
    fn entry_count(&self) -> usize;
}

/// Mount point trait for filesystem mounting
pub trait MountPoint {
    /// Mount filesystem
    fn mount(&mut self, fs: Box<dyn Filesystem>) -> Result<(), FsError>;
    
    /// Unmount filesystem
    fn unmount(&mut self) -> Result<(), FsError>;
    
    /// Get mount path
    fn mount_path(&self) -> &str;
    
    /// Get mounted filesystem
    fn filesystem(&self) -> Option<&dyn Filesystem>;
    
    /// Get mutable mounted filesystem
    fn filesystem_mut(&mut self) -> Option<&mut dyn Filesystem>;
}

/// Virtual filesystem trait for VFS layer
pub trait VirtualFilesystem {
    /// Register filesystem
    fn register(&mut self, fs: Box<dyn Filesystem>) -> Result<(), FsError>;
    
    /// Unregister filesystem
    fn unregister(&mut self, name: &str) -> Result<(), FsError>;
    
    /// Mount filesystem at path
    fn mount(&mut self, fs: Box<dyn Filesystem>, path: &str) -> Result<(), FsError>;
    
    /// Unmount filesystem at path
    fn unmount(&mut self, path: &str) -> Result<(), FsError>;
    
    /// Open file
    fn open(&mut self, path: &str, flags: u32) -> Result<Box<dyn File>, FsError>;
    
    /// Create directory
    fn mkdir(&mut self, path: &str, perms: u32) -> Result<(), FsError>;
    
    /// Remove directory
    fn rmdir(&mut self, path: &str) -> Result<(), FsError>;
    
    /// Remove file
    fn unlink(&mut self, path: &str) -> Result<(), FsError>;
    
    /// Get file stats
    fn stat(&self, path: &str) -> Result<FileStats, FsError>;
    
    /// Set file stats
    fn set_stat(&mut self, path: &str, stats: FileStats) -> Result<(), FsError>;
}

/// Cache trait for filesystem caching
pub trait FsCache {
    /// Read from cache
    fn read(&mut self, offset: u64, size: usize) -> Result<Vec<u8>, FsError>;
    
    /// Write to cache
    fn write(&mut self, offset: u64, data: &[u8]) -> Result<(), FsError>;
    
    /// Invalidate cache
    fn invalidate(&mut self, offset: u64, size: usize) -> Result<(), FsError>;
    
    /// Flush cache
    fn flush(&mut self) -> Result<(), FsError>;
    
    /// Get cache size
    fn size(&self) -> usize;
    
    /// Clear cache
    fn clear(&mut self) -> Result<(), FsError>;
}

/// Error types for filesystem operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    InvalidParameter,
    IoError,
    NoSpace,
    NotDirectory,
    NotFile,
    Exists,
    NotEmpty,
    InvalidState,
    ReadOnly,
    Corrupted,
}

/// Filesystem types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    SigmaFS,
    Ext4,
    Fat32,
    TmpFS,
    ProcFS,
    DevFS,
    Other,
}

/// Inode types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub inode: u64,
    pub name: String,
    pub entry_type: InodeType,
}

/// File statistics
#[derive(Debug, Clone, Copy)]
pub struct FileStats {
    pub inode: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
}

/// File open flags
pub mod flags {
    pub const O_RDONLY: u32 = 0o0000;
    pub const O_WRONLY: u32 = 0o0001;
    pub const O_RDWR: u32 = 0o0002;
    pub const O_CREAT: u32 = 0o0100;
    pub const O_EXCL: u32 = 0o0200;
    pub const O_TRUNC: u32 = 0o1000;
    pub const O_APPEND: u32 = 0o2000;
    pub const O_DIRECTORY: u32 = 0o200000;
}
