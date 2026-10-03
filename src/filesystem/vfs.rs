//! # Virtual File System (VFS) Layer
//!
//! Generic filesystem abstraction layer inspired by Linux VFS and BSD's vnode architecture.
//! Provides unified interface for different filesystem implementations (ext4, FAT32, NTFS, ZFS, etc.)

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

// Re-export VFS stubs
pub use crate::stubs::vfs_stubs::*;

/// File types supported by VFS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileType {
    Regular = 1,
    Directory = 2,
    CharDevice = 3,
    BlockDevice = 4,
    Fifo = 5,
    Socket = 6,
    Symlink = 7,
}

/// File access modes (inspired by Linux O_* flags)
#[derive(Debug, Clone, Copy)]
pub struct OpenFlags {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub create: bool,
    pub truncate: bool,
    pub exclusive: bool,
    pub directory: bool,
    pub no_follow: bool,
}

impl OpenFlags {
    pub const READ_ONLY: Self = Self {
        read: true,
        write: false,
        append: false,
        create: false,
        truncate: false,
        exclusive: false,
        directory: false,
        no_follow: false,
    };

    pub const WRITE_ONLY: Self = Self {
        read: false,
        write: true,
        append: false,
        create: false,
        truncate: false,
        exclusive: false,
        directory: false,
        no_follow: false,
    };

    pub const READ_WRITE: Self = Self {
        read: true,
        write: true,
        append: false,
        create: false,
        truncate: false,
        exclusive: false,
        directory: false,
        no_follow: false,
    };
}

/// File permissions (POSIX-style mode bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMode(pub u16);

impl FileMode {
    pub const fn new(mode: u16) -> Self {
        Self(mode & 0o7777)
    }

    pub const fn is_readable_by_owner(self) -> bool {
        (self.0 & 0o400) != 0
    }

    pub const fn is_writable_by_owner(self) -> bool {
        (self.0 & 0o200) != 0
    }

    pub const fn is_executable_by_owner(self) -> bool {
        (self.0 & 0o100) != 0
    }
}

/// Inode metadata (inspired by Linux struct inode)
#[derive(Debug, Clone)]
pub struct InodeMetadata {
    pub inode_number: u64,
    pub file_type: FileType,
    pub mode: FileMode,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub block_count: u64,
    pub link_count: u32,
    pub access_time: u64,
    pub modify_time: u64,
    pub change_time: u64,
}

/// VFS inode operations trait (inspired by Linux inode_operations)
pub trait InodeOps {
    fn lookup(&self, name: &str) -> Result<u64, VfsError>;
    fn create(&mut self, name: &str, mode: FileMode) -> Result<u64, VfsError>;
    fn mkdir(&mut self, name: &str, mode: FileMode) -> Result<u64, VfsError>;
    fn unlink(&mut self, name: &str) -> Result<(), VfsError>;
    fn rmdir(&mut self, name: &str) -> Result<(), VfsError>;
    fn rename(&mut self, old_name: &str, new_name: &str) -> Result<(), VfsError>;
    fn symlink(&mut self, target: &str, link_name: &str) -> Result<u64, VfsError>;
    fn readlink(&self) -> Result<String, VfsError>;
}

/// VFS file operations trait (inspired by Linux file_operations)
pub trait FileOps {
    fn read(&mut self, offset: u64, buffer: &mut [u8]) -> Result<usize, VfsError>;
    fn write(&mut self, offset: u64, buffer: &[u8]) -> Result<usize, VfsError>;
    fn flush(&mut self) -> Result<(), VfsError>;
    fn seek(&mut self, offset: i64, whence: SeekWhence) -> Result<u64, VfsError>;
    fn truncate(&mut self, size: u64) -> Result<(), VfsError>;
}

/// Seek position reference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekWhence {
    Start = 0,
    Current = 1,
    End = 2,
}

/// VFS superblock operations (inspired by Linux super_operations)
pub trait SuperblockOps {
    fn sync(&mut self) -> Result<(), VfsError>;
    fn statfs(&self) -> Result<FilesystemStats, VfsError>;
    fn remount(&mut self, flags: MountFlags) -> Result<(), VfsError>;
    fn unmount(&mut self) -> Result<(), VfsError>;
}

/// Filesystem statistics (inspired by Linux struct statfs)
#[derive(Debug, Clone)]
pub struct FilesystemStats {
    pub fs_type: u32,
    pub block_size: u64,
    pub total_blocks: u64,
    pub free_blocks: u64,
    pub available_blocks: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
    pub max_filename_len: u64,
}

/// Mount flags
#[derive(Debug, Clone, Copy)]
pub struct MountFlags {
    pub read_only: bool,
    pub no_exec: bool,
    pub no_suid: bool,
    pub no_dev: bool,
    pub synchronous: bool,
    pub no_atime: bool,
}

impl MountFlags {
    pub const READ_WRITE: Self = Self {
        read_only: false,
        no_exec: false,
        no_suid: false,
        no_dev: false,
        synchronous: false,
        no_atime: false,
    };

    pub const READ_ONLY: Self = Self {
        read_only: true,
        no_exec: false,
        no_suid: false,
        no_dev: false,
        synchronous: false,
        no_atime: false,
    };
}

/// VFS error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    NotDirectory,
    IsDirectory,
    InvalidArgument,
    IoError,
    NoSpace,
    ReadOnlyFilesystem,
    TooManyLinks,
    NameTooLong,
    NotEmpty,
    Busy,
}

/// Dentry (directory entry) cache entry (inspired by Linux dentry)
#[derive(Debug, Clone)]
pub struct Dentry {
    pub name: String,
    pub inode_number: u64,
    pub file_type: FileType,
}

/// Mount point information
#[derive(Debug)]
pub struct MountPoint {
    pub device: String,
    pub mount_path: String,
    pub fs_type: String,
    pub flags: MountFlags,
}

/// VFS mount table (inspired by Linux mount namespace)
pub struct MountTable {
    mounts: Vec<MountPoint>,
}

impl MountTable {
    pub const fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    pub fn register_mount(&mut self, mount: MountPoint) {
        self.mounts.push(mount);
    }

    pub fn find_mount(&self, path: &str) -> Option<&MountPoint> {
        // Find longest matching mount path
        let mut best_match: Option<&MountPoint> = None;
        let mut best_len = 0;

        for mount in &self.mounts {
            if path.starts_with(&mount.mount_path) && mount.mount_path.len() > best_len {
                best_match = Some(mount);
                best_len = mount.mount_path.len();
            }
        }

        best_match
    }

    pub fn unmount(&mut self, path: &str) -> Result<(), VfsError> {
        if let Some(pos) = self.mounts.iter().position(|m| m.mount_path == path) {
            self.mounts.remove(pos);
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    pub fn list_mounts(&self) -> &[MountPoint] {
        &self.mounts
    }
}

/// Path resolver with symlink resolution (inspired by Linux path_resolution)
pub struct PathResolver {
    max_symlink_depth: u32,
}

impl PathResolver {
    pub const fn new() -> Self {
        Self {
            max_symlink_depth: 40, // Linux default
        }
    }

    /// Resolve absolute path with symlink following
    pub fn resolve(&self, _path: &str, _follow_symlinks: bool) -> Result<String, VfsError> {
        // Simplified implementation - full version would traverse components
        // and resolve symlinks iteratively
        Err(VfsError::NotFound)
    }

    /// Normalize path (remove . and .. components)
    pub fn normalize(&self, path: &str) -> String {
        let mut components = Vec::new();

        for component in path.split('/') {
            match component {
                "" | "." => continue,
                ".." => {
                    components.pop();
                }
                c => components.push(c),
            }
        }

        let mut result = String::from("/");
        for (i, component) in components.iter().enumerate() {
            if i > 0 {
                result.push('/');
            }
            result.push_str(component);
        }

        result
    }
}

/// Global VFS state manager
pub struct VfsManager {
    mount_table: MountTable,
    path_resolver: PathResolver,
}

impl VfsManager {
    pub const fn new() -> Self {
        Self {
            mount_table: MountTable::new(),
            path_resolver: PathResolver::new(),
        }
    }

    pub fn mount(
        &mut self,
        device: String,
        path: String,
        fs_type: String,
        flags: MountFlags,
    ) -> Result<(), VfsError> {
        let mount = MountPoint {
            device,
            mount_path: path,
            fs_type,
            flags,
        };
        self.mount_table.register_mount(mount);
        Ok(())
    }

    pub fn unmount(&mut self, path: &str) -> Result<(), VfsError> {
        self.mount_table.unmount(path)
    }

    pub fn lookup(&self, path: &str) -> Result<&MountPoint, VfsError> {
        self.mount_table.find_mount(path).ok_or(VfsError::NotFound)
    }

    pub fn normalize_path(&self, path: &str) -> String {
        self.path_resolver.normalize(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_mode() {
        let mode = FileMode::new(0o644);
        assert!(mode.is_readable_by_owner());
        assert!(mode.is_writable_by_owner());
        assert!(!mode.is_executable_by_owner());
    }

    #[test]
    fn test_path_normalize() {
        let resolver = PathResolver::new();
        assert_eq!(resolver.normalize("/foo/./bar/../baz"), "/foo/baz");
        assert_eq!(resolver.normalize("/a/b/c/../../d"), "/a/d");
        assert_eq!(resolver.normalize("/"), "/");
    }

    #[test]
    fn test_mount_table() {
        let mut table = MountTable::new();
        let mount = MountPoint {
            device: String::from("/dev/sda1"),
            mount_path: String::from("/mnt/data"),
            fs_type: String::from("ext4"),
            flags: MountFlags::READ_WRITE,
        };
        table.register_mount(mount);

        assert!(table.find_mount("/mnt/data/file.txt").is_some());
        assert!(table.find_mount("/home").is_none());
    }

    #[test]
    fn test_vfs_manager() {
        let mut vfs = VfsManager::new();
        vfs.mount(
            String::from("/dev/sda1"),
            String::from("/"),
            String::from("ext4"),
            MountFlags::READ_WRITE,
        )
        .unwrap();

        assert!(vfs.lookup("/etc/passwd").is_ok());
    }
}
