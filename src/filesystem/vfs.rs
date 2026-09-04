// SPDX-License-Identifier: MIT
/// SigmaOS: Virtual File System (VFS) Layer
/// Provides unified filesystem abstraction supporting multiple filesystem types
/// Integrates with syscall dispatcher for read, write, open, close operations

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    SymbolicLink,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
}

/// File mode bits (permissions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMode {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
}

impl FileMode {
    pub fn new(mode: u32) -> Self {
        Self {
            owner_read: (mode & 0o400) != 0,
            owner_write: (mode & 0o200) != 0,
            owner_execute: (mode & 0o100) != 0,
            group_read: (mode & 0o040) != 0,
            group_write: (mode & 0o020) != 0,
            group_execute: (mode & 0o010) != 0,
            other_read: (mode & 0o004) != 0,
            other_write: (mode & 0o002) != 0,
            other_execute: (mode & 0o001) != 0,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut mode = 0u32;
        if self.owner_read { mode |= 0o400; }
        if self.owner_write { mode |= 0o200; }
        if self.owner_execute { mode |= 0o100; }
        if self.group_read { mode |= 0o040; }
        if self.group_write { mode |= 0o020; }
        if self.group_execute { mode |= 0o010; }
        if self.other_read { mode |= 0o004; }
        if self.other_write { mode |= 0o002; }
        if self.other_execute { mode |= 0o001; }
        mode
    }
}

/// Inode - represents a file or directory on disk
#[derive(Debug, Clone)]
pub struct Inode {
    pub inode_number: u64,
    pub file_type: FileType,
    pub mode: FileMode,
    pub size: u64,
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub created_time: u64,
    pub modified_time: u64,
    pub accessed_time: u64,
    pub hard_links: u32,
    pub data_blocks: Vec<u64>,
}

impl Inode {
    pub fn new(inode_number: u64, file_type: FileType, mode: u32) -> Self {
        Self {
            inode_number,
            file_type,
            mode: FileMode::new(mode),
            size: 0,
            owner_uid: 0,
            owner_gid: 0,
            created_time: 0,
            modified_time: 0,
            accessed_time: 0,
            hard_links: 1,
            data_blocks: Vec::new(),
        }
    }
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub inode_number: u64,
    pub file_type: FileType,
}

/// File handle for open files
#[derive(Debug, Clone)]
pub struct FileHandle {
    pub fd: i32,
    pub inode_number: u64,
    pub position: u64,
    pub flags: u32,
    pub mode: u32,
}

/// VFS Error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    InvalidArgument,
    IsDirectory,
    NotDirectory,
    FileExists,
    OutOfSpace,
    IoError,
    BadFileDescriptor,
    NoMemory,
    TooManyOpenFiles,
    NameTooLong,
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "No such file or directory"),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
            Self::IsDirectory => write!(f, "Is a directory"),
            Self::NotDirectory => write!(f, "Not a directory"),
            Self::FileExists => write!(f, "File exists"),
            Self::OutOfSpace => write!(f, "No space left on device"),
            Self::IoError => write!(f, "Input/output error"),
            Self::BadFileDescriptor => write!(f, "Bad file descriptor"),
            Self::NoMemory => write!(f, "Cannot allocate memory"),
            Self::TooManyOpenFiles => write!(f, "Too many open files"),
            Self::NameTooLong => write!(f, "File name too long"),
        }
    }
}

/// Filesystem trait - implemented by ext4, NTFS, FAT32, etc.
pub trait FileSystem: Send + Sync {
    /// Initialize filesystem
    fn init(&mut self) -> Result<(), VfsError>;

    /// Read inode
    fn read_inode(&self, inode_number: u64) -> Result<Inode, VfsError>;

    /// Write inode
    fn write_inode(&mut self, inode: &Inode) -> Result<(), VfsError>;

    /// Read data from inode at offset
    fn read_data(&self, inode_number: u64, offset: u64, buffer: &mut [u8]) -> Result<usize, VfsError>;

    /// Write data to inode at offset
    fn write_data(&mut self, inode_number: u64, offset: u64, data: &[u8]) -> Result<usize, VfsError>;

    /// List directory entries
    fn list_dir(&self, inode_number: u64) -> Result<Vec<DirEntry>, VfsError>;

    /// Find inode by path within filesystem
    fn lookup(&self, parent_inode: u64, name: &str) -> Result<u64, VfsError>;

    /// Create new file
    fn create(&mut self, parent_inode: u64, name: &str, mode: u32) -> Result<u64, VfsError>;

    /// Create new directory
    fn mkdir(&mut self, parent_inode: u64, name: &str, mode: u32) -> Result<u64, VfsError>;

    /// Delete file
    fn unlink(&mut self, parent_inode: u64, name: &str) -> Result<(), VfsError>;

    /// Delete directory
    fn rmdir(&mut self, parent_inode: u64, name: &str) -> Result<(), VfsError>;

    /// Get filesystem name
    fn name(&self) -> &'static str;
}

/// Mount point - associates filesystem with path
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub path: String,
    pub fs_type: String,
}

/// Virtual File System - main VFS layer
pub struct VirtualFileSystem {
    filesystems: Vec<(String, u64)>, // (fs_type, block_device_id)
    mounts: Vec<MountPoint>,
    open_files: Vec<FileHandle>,
    next_fd: i32,
    inode_cache: Vec<(u64, Inode)>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self {
            filesystems: Vec::new(),
            mounts: Vec::new(),
            open_files: Vec::new(),
            next_fd: 3, // 0, 1, 2 are stdin, stdout, stderr
            inode_cache: Vec::new(),
        }
    }

    /// Register a filesystem type
    pub fn register_filesystem(&mut self, fs_type: String, block_device_id: u64) -> Result<(), VfsError> {
        // Check if already registered
        for (ft, _) = &self.filesystems {
            if ft == &fs_type {
                return Err(VfsError::FileExists);
            }
        }

        self.filesystems.push((fs_type, block_device_id));
        Ok(())
    }

    /// Mount filesystem at path
    pub fn mount(&mut self, path: String, fs_type: String) -> Result<(), VfsError> {
        // Verify filesystem is registered
        if !self.filesystems.iter().any(|(ft, _)| ft == &fs_type) {
            return Err(VfsError::NotFound);
        }

        // Check if path is already mounted
        if self.mounts.iter().any(|m| m.path == path) {
            return Err(VfsError::FileExists);
        }

        self.mounts.push(MountPoint { path, fs_type });
        Ok(())
    }

    /// Unmount filesystem
    pub fn unmount(&mut self, path: &str) -> Result<(), VfsError> {
        if let Some(pos) = self.mounts.iter().position(|m| m.path == path) {
            self.mounts.remove(pos);
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    /// Open file - returns file descriptor
    pub fn open(&mut self, path: &str, flags: u32, mode: u32) -> Result<i32, VfsError> {
        if path.len() > 4096 {
            return Err(VfsError::NameTooLong);
        }

        if self.open_files.len() >= 1024 {
            return Err(VfsError::TooManyOpenFiles);
        }

        // For now, create a stub file handle
        let fd = self.next_fd;
        self.next_fd += 1;

        let handle = FileHandle {
            fd,
            inode_number: 0, // Would be populated by actual filesystem
            position: 0,
            flags,
            mode,
        };

        self.open_files.push(handle);
        Ok(fd)
    }

    /// Close file descriptor
    pub fn close(&mut self, fd: i32) -> Result<(), VfsError> {
        if let Some(pos) = self.open_files.iter().position(|h| h.fd == fd) {
            self.open_files.remove(pos);
            Ok(())
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Read from file descriptor
    pub fn read(&mut self, fd: i32, buffer: &mut [u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            // Stub implementation - would read from actual filesystem
            let bytes_read = buffer.len().min(512); // Limit read size
            handle.position += bytes_read as u64;
            Ok(bytes_read)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Write to file descriptor
    pub fn write(&mut self, fd: i32, data: &[u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            // Stub implementation - would write to actual filesystem
            let bytes_written = data.len().min(512); // Limit write size
            handle.position += bytes_written as u64;
            Ok(bytes_written)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Seek in file
    pub fn seek(&mut self, fd: i32, offset: i64, whence: u32) -> Result<u64, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            match whence {
                0 => { // SEEK_SET
                    if offset < 0 {
                        return Err(VfsError::InvalidArgument);
                    }
                    handle.position = offset as u64;
                }
                1 => { // SEEK_CUR
                    if offset < 0 && (offset.abs() as u64) > handle.position {
                        return Err(VfsError::InvalidArgument);
                    }
                    handle.position = ((handle.position as i64) + offset) as u64;
                }
                2 => { // SEEK_END
                    // Would need file size from actual filesystem
                    return Err(VfsError::InvalidArgument);
                }
                _ => return Err(VfsError::InvalidArgument),
            }
            Ok(handle.position)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Get file statistics
    pub fn stat(&self, path: &str) -> Result<Inode, VfsError> {
        // Stub implementation - would query actual filesystem
        Ok(Inode::new(0, FileType::Regular, 0o644))
    }

    /// List directory contents
    pub fn readdir(&self, path: &str) -> Result<Vec<DirEntry>, VfsError> {
        // Stub implementation - would query actual filesystem
        Ok(Vec::new())
    }

    /// Get number of open files
    pub fn open_file_count(&self) -> usize {
        self.open_files.len()
    }

    /// Get mount points
    pub fn get_mounts(&self) -> &[MountPoint] {
        &self.mounts
    }
}

impl Default for VirtualFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_creation() {
        let vfs = VirtualFileSystem::new();
        assert_eq!(vfs.open_file_count(), 0);
        assert_eq!(vfs.next_fd, 3);
    }

    #[test]
    fn test_file_open_close() {
        let mut vfs = VirtualFileSystem::new();
        let fd = vfs.open("/test.txt", 0, 0o644).unwrap();
        assert!(fd >= 3);
        assert_eq!(vfs.open_file_count(), 1);
        
        vfs.close(fd).unwrap();
        assert_eq!(vfs.open_file_count(), 0);
    }

    #[test]
    fn test_bad_fd() {
        let mut vfs = VirtualFileSystem::new();
        let result = vfs.close(999);
        assert_eq!(result, Err(VfsError::BadFileDescriptor));
    }

    #[test]
    fn test_file_mode() {
        let mode = FileMode::new(0o755);
        assert!(mode.owner_read);
        assert!(mode.owner_write);
        assert!(mode.owner_execute);
        assert!(mode.group_read);
        assert!(mode.group_execute);
        assert!(mode.other_read);
        assert!(mode.other_execute);
    }

    #[test]
    fn test_seek_operations() {
        let mut vfs = VirtualFileSystem::new();
        let fd = vfs.open("/test.txt", 0, 0o644).unwrap();
        
        // SEEK_SET
        let pos = vfs.seek(fd, 100, 0).unwrap();
        assert_eq!(pos, 100);
        
        // SEEK_CUR
        let pos = vfs.seek(fd, 50, 1).unwrap();
        assert_eq!(pos, 150);
        
        vfs.close(fd).unwrap();
    }
}
