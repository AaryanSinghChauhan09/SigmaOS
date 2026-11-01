//! # Virtual File System (VFS) Layer
//!
//! Generic filesystem abstraction layer inspired by Linux VFS and BSD's vnode
//! architecture. Provides a unified interface for different filesystem
//! implementations (ext4, tmpfs, devfs, etc.).
//!
//! ## Principles
//! - Separation of concerns: VFS abstracts concrete FS implementations.
//! - Stable public API: consumers depend on traits, not concrete types.
//! - POSIX compliance: file types, modes, and seek semantics follow POSIX.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use thiserror::Error;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// VFS-level errors following POSIX errno semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum VfsError {
    #[error("no such file or directory")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("file already exists")]
    AlreadyExists,
    #[error("not a directory")]
    NotDirectory,
    #[error("is a directory")]
    IsDirectory,
    #[error("invalid argument")]
    InvalidArgument,
    #[error("I/O error")]
    IoError,
    #[error("no space left on device")]
    NoSpace,
    #[error("read-only filesystem")]
    ReadOnlyFilesystem,
    #[error("too many links")]
    TooManyLinks,
    #[error("file name too long")]
    NameTooLong,
    #[error("directory not empty")]
    NotEmpty,
    #[error("device or resource busy")]
    Busy,
    #[error("not supported")]
    NotSupported,
}

/// FS-level errors (alias kept for source compatibility).
pub type FsError = VfsError;

// ---------------------------------------------------------------------------
// File type and permissions
// ---------------------------------------------------------------------------

/// File types supported by VFS (POSIX S_IF* flags encoded as enum).
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

/// File permissions (POSIX-style 12-bit mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMode(pub u16);

impl FileMode {
    /// Create a new mode, masking to 12 bits.
    pub const fn new(mode: u16) -> Self {
        Self(mode & 0o7777)
    }

    pub const fn is_readable_by_owner(self) -> bool { (self.0 & 0o400) != 0 }
    pub const fn is_writable_by_owner(self) -> bool { (self.0 & 0o200) != 0 }
    pub const fn is_executable_by_owner(self) -> bool { (self.0 & 0o100) != 0 }
}

// ---------------------------------------------------------------------------
// Open flags
// ---------------------------------------------------------------------------

/// File access / creation flags (modelled on Linux O_* flags).
#[derive(Debug, Clone, Copy)]
pub struct OpenFlags {
    pub read:      bool,
    pub write:     bool,
    pub append:    bool,
    pub create:    bool,
    pub truncate:  bool,
    pub exclusive: bool,
    pub directory: bool,
    pub no_follow: bool,
}

impl OpenFlags {
    pub const READ_ONLY: Self = Self {
        read: true, write: false, append: false, create: false,
        truncate: false, exclusive: false, directory: false, no_follow: false,
    };
    pub const WRITE_ONLY: Self = Self {
        read: false, write: true, append: false, create: false,
        truncate: false, exclusive: false, directory: false, no_follow: false,
    };
    pub const READ_WRITE: Self = Self {
        read: true, write: true, append: false, create: false,
        truncate: false, exclusive: false, directory: false, no_follow: false,
    };
}

// ---------------------------------------------------------------------------
// Seek
// ---------------------------------------------------------------------------

/// Seek position reference (POSIX SEEK_SET/SEEK_CUR/SEEK_END).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekWhence {
    Start   = 0,
    Current = 1,
    End     = 2,
}

// ---------------------------------------------------------------------------
// Inode metadata
// ---------------------------------------------------------------------------

/// Inode metadata (inspired by Linux `struct inode`).
#[derive(Debug, Clone)]
pub struct InodeMetadata {
    pub inode_number: u64,
    pub file_type:    FileType,
    pub mode:         FileMode,
    pub uid:          u32,
    pub gid:          u32,
    pub size:         u64,
    pub block_count:  u64,
    pub link_count:   u32,
    pub access_time:  u64,
    pub modify_time:  u64,
    pub change_time:  u64,
}

/// Minimal inode representation exposed by the VFS layer.
#[derive(Debug, Clone)]
pub struct Inode {
    pub meta: InodeMetadata,
}

// ---------------------------------------------------------------------------
// VFS operation traits
// ---------------------------------------------------------------------------

/// Inode operations (inspired by Linux `inode_operations`).
pub trait InodeOps {
    fn lookup (&self,  name: &str) -> Result<u64, VfsError>;
    fn create (&mut self, name: &str, mode: FileMode) -> Result<u64, VfsError>;
    fn mkdir  (&mut self, name: &str, mode: FileMode) -> Result<u64, VfsError>;
    fn unlink (&mut self, name: &str) -> Result<(), VfsError>;
    fn rmdir  (&mut self, name: &str) -> Result<(), VfsError>;
    fn rename (&mut self, old_name: &str, new_name: &str) -> Result<(), VfsError>;
    fn symlink(&mut self, target: &str, link_name: &str) -> Result<u64, VfsError>;
    fn readlink(&self) -> Result<String, VfsError>;
    fn stat   (&self,  name: &str) -> Result<InodeMetadata, VfsError>;
}

/// File operations (inspired by Linux `file_operations`).
pub trait FileOps {
    fn read    (&mut self, offset: u64, buffer: &mut [u8]) -> Result<usize, VfsError>;
    fn write   (&mut self, offset: u64, buffer: &[u8])     -> Result<usize, VfsError>;
    fn flush   (&mut self)                                  -> Result<(), VfsError>;
    fn seek    (&mut self, offset: i64, whence: SeekWhence) -> Result<u64, VfsError>;
    fn truncate(&mut self, size: u64)                       -> Result<(), VfsError>;
}

/// Superblock operations (inspired by Linux `super_operations`).
pub trait SuperblockOps {
    fn sync    (&mut self)                          -> Result<(), VfsError>;
    fn statfs  (&self)                              -> Result<FilesystemStats, VfsError>;
    fn remount (&mut self, flags: VfsMountFlags)    -> Result<(), VfsError>;
    fn unmount (&mut self)                          -> Result<(), VfsError>;
}

/// Combined VirtualFilesystem trait — convenience alias.
pub trait VirtualFilesystem: InodeOps + SuperblockOps {}

// ---------------------------------------------------------------------------
// Filesystem statistics and mount flags
// ---------------------------------------------------------------------------

/// Filesystem statistics (inspired by Linux `struct statfs`).
#[derive(Debug, Clone)]
pub struct FilesystemStats {
    pub fs_type:          u32,
    pub block_size:       u64,
    pub total_blocks:     u64,
    pub free_blocks:      u64,
    pub available_blocks: u64,
    pub total_inodes:     u64,
    pub free_inodes:      u64,
    pub max_filename_len: u64,
}

/// Mount flags used by the VFS layer.
#[derive(Debug, Clone, Copy)]
pub struct VfsMountFlags {
    pub read_only:    bool,
    pub no_exec:      bool,
    pub no_suid:      bool,
    pub no_dev:       bool,
    pub synchronous:  bool,
    pub no_atime:     bool,
}

impl VfsMountFlags {
    pub const READ_WRITE: Self = Self {
        read_only: false, no_exec: false, no_suid: false,
        no_dev: false, synchronous: false, no_atime: false,
    };
    pub const READ_ONLY: Self = Self {
        read_only: true, no_exec: false, no_suid: false,
        no_dev: false, synchronous: false, no_atime: false,
    };
}

// ---------------------------------------------------------------------------
// Dentry and mount table
// ---------------------------------------------------------------------------

/// Directory entry cache entry (inspired by Linux `dentry`).
#[derive(Debug, Clone)]
pub struct Dentry {
    pub name:         String,
    pub inode_number: u64,
    pub file_type:    FileType,
}

/// Mount point descriptor.
#[derive(Debug)]
pub struct MountPoint {
    pub device:     String,
    pub mount_path: String,
    pub fs_type:    String,
    pub flags:      VfsMountFlags,
}

/// VFS mount table (tracks active mount points).
pub struct MountTable {
    mounts: Vec<MountPoint>,
}

impl MountTable {
    pub fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    pub fn register_mount(&mut self, mount: MountPoint) {
        self.mounts.push(mount);
    }

    /// Find the deepest mount point whose path is a prefix of `path`.
    pub fn find_mount(&self, path: &str) -> Option<&MountPoint> {
        let mut best: Option<&MountPoint> = None;
        let mut best_len = 0;
        for m in &self.mounts {
            if path.starts_with(&m.mount_path) && m.mount_path.len() > best_len {
                best = Some(m);
                best_len = m.mount_path.len();
            }
        }
        best
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

// ---------------------------------------------------------------------------
// Path resolver
// ---------------------------------------------------------------------------

/// POSIX path normaliser and resolver.
pub struct PathResolver {
    max_symlink_depth: u32,
}

impl PathResolver {
    pub const fn new() -> Self {
        Self { max_symlink_depth: 40 }
    }

    /// Normalise an absolute path: resolve `.` and `..` components.
    pub fn normalize(&self, path: &str) -> String {
        let mut components: Vec<&str> = Vec::new();
        for component in path.split('/') {
            match component {
                "" | "." => continue,
                ".." => { components.pop(); }
                c => components.push(c),
            }
        }
        let mut result = String::from("/");
        for (i, component) in components.iter().enumerate() {
            if i > 0 { result.push('/'); }
            result.push_str(component);
        }
        result
    }

    /// Placeholder resolver — full implementation requires inode traversal.
    pub fn resolve(&self, _path: &str, _follow_symlinks: bool) -> Result<String, VfsError> {
        Err(VfsError::NotSupported)
    }
}

// ---------------------------------------------------------------------------
// VFS manager
// ---------------------------------------------------------------------------

/// Global VFS state: owns the mount table and exposes the public VFS API.
pub struct VfsManager {
    mount_table:   MountTable,
    path_resolver: PathResolver,
}

impl VfsManager {
    pub fn new() -> Self {
        Self {
            mount_table:   MountTable::new(),
            path_resolver: PathResolver::new(),
        }
    }

    pub fn mount(
        &mut self,
        device:  String,
        path:    String,
        fs_type: String,
        flags:   VfsMountFlags,
    ) -> Result<(), VfsError> {
        self.mount_table.register_mount(MountPoint {
            device,
            mount_path: path,
            fs_type,
            flags,
        });
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

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_mode_bits() {
        let mode = FileMode::new(0o644);
        assert!(mode.is_readable_by_owner());
        assert!(mode.is_writable_by_owner());
        assert!(!mode.is_executable_by_owner());
    }

    #[test]
    fn path_normalize() {
        let r = PathResolver::new();
        assert_eq!(r.normalize("/foo/./bar/../baz"), "/foo/baz");
        assert_eq!(r.normalize("/a/b/c/../../d"),    "/a/d");
        assert_eq!(r.normalize("/"),                 "/");
    }

    #[test]
    fn mount_table_lookup() {
        let mut t = MountTable::new();
        t.register_mount(MountPoint {
            device:     String::from("/dev/sda1"),
            mount_path: String::from("/mnt/data"),
            fs_type:    String::from("ext4"),
            flags:      VfsMountFlags::READ_WRITE,
        });
        assert!(t.find_mount("/mnt/data/file.txt").is_some());
        assert!(t.find_mount("/home").is_none());
    }

    #[test]
    fn vfs_manager_roundtrip() {
        let mut vfs = VfsManager::new();
        vfs.mount(
            String::from("/dev/sda1"),
            String::from("/"),
            String::from("ext4"),
            VfsMountFlags::READ_WRITE,
        ).unwrap();
        assert!(vfs.lookup("/etc/passwd").is_ok());
        assert!(vfs.unmount("/").is_ok());
        assert_eq!(vfs.unmount("/"), Err(VfsError::NotFound));
    }
}
