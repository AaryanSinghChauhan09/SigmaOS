// SigmaOS Virtual Filesystem (VFS)
// Capability-based, standard Linux/BSD conforming filesystem with security, hard links, and path traversal

use crate::klib::HashMap;
use crate::security::CapabilityToken;

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    CharacterDevice,
    BlockDevice,
    Symlink,
}

/// File permissions
#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    pub fn all() -> Self {
        Self::new(true, true, true)
    }

    pub fn read_only() -> Self {
        Self::new(true, false, false)
    }
}

/// Inode (file/directory metadata)
#[derive(Debug, Clone)]
pub struct Inode {
    pub id: u64,
    pub file_type: FileType,
    pub permissions: FilePermissions,
    pub size: u64,
    pub owner: u64,
    pub group: u64,
    pub created: u64,
    pub modified: u64,
    pub capabilities: CapabilityToken,
    pub link_count: u32, // standard inode link count tracking hard links
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileDescriptor {
    pub fd: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2,
    AlreadyExists = 3,
    NotADirectory = 4,
    IsADirectory = 5,
    NotEmpty = 6,
    IOError = 7,
    InvalidParameter = 8,
}

pub struct VirtualFilesystem {
    pub root: Inode,
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        Self {
            root: Inode {
                id: 1,
                file_type: FileType::Directory,
                permissions: FilePermissions::all(),
                size: 0,
                owner: 0,
                group: 0,
                created: 0,
                modified: 0,
                capabilities: CapabilityToken::from_bits(!0),
                link_count: 1,
            },
        }
    }
}
