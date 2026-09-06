#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! # SigmaOS Virtual Filesystem Layer (VFS)
//!
//! A Linux-inspired VFS abstraction that decouples the kernel from concrete
//! filesystem implementations such as Sigma-EXT, Sigma-ZFS-parity, OverlayFS,
//! tmpfs, procfs and sysfs.
//!
//! ## Architecture
//!
//! ```text
//!  Syscall layer (open/read/write/stat/readdir/…)
//!        │
//!  ┌─────▼──────────────────────────────────────┐
//!  │              VFS Layer                      │
//!  │  VfsPath ──► VfsDentry ──► VfsInode        │
//!  │  VfsMount table     VfsSuperblock trait     │
//!  └──────┬──────────────────────────────────────┘
//!         │  (dispatches via trait objects)
//!    ┌────▼────┐  ┌──────────┐  ┌───────┐  ┌────────┐
//!    │Sigma-EXT│  │Sigma-ZFS │  │tmpfs  │  │procfs  │  …
//!    └─────────┘  └──────────┘  └───────┘  └────────┘
//! ```
//!
//! ## Key types
//!
//! * [`VfsNode`] — trait implemented by all filesystem objects (files, dirs, …).
//! * [`VfsSuperblock`] — trait for registering a filesystem type with the VFS.
//! * [`VfsMount`] — a single entry in the mount table.
//! * [`VfsDentry`] — directory-entry cache node linking a name to an inode.
//! * [`VfsInode`] — generic inode with reference count.
//! * [`VfsPath`] — path resolution with symlink following.

#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// VFS errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VfsError {
    /// No such file or directory.
    NotFound(String),
    /// The operation is not supported by this filesystem.
    NotSupported,
    /// Permission denied.
    PermissionDenied,
    /// The path contains too many symbolic link levels.
    TooManySymlinks,
    /// Not a directory.
    NotADirectory,
    /// Is a directory (when a file was expected).
    IsADirectory,
    /// Input / output error.
    Io(String),
    /// The filesystem is read-only.
    ReadOnly,
    /// The path is already mounted.
    AlreadyMounted,
    /// The filesystem type is unknown.
    UnknownFilesystem(String),
}

impl core::fmt::Display for VfsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VfsError::NotFound(p)            => write!(f, "vfs: not found: {}", p),
            VfsError::NotSupported           => write!(f, "vfs: operation not supported"),
            VfsError::PermissionDenied       => write!(f, "vfs: permission denied"),
            VfsError::TooManySymlinks        => write!(f, "vfs: too many symbolic links"),
            VfsError::NotADirectory          => write!(f, "vfs: not a directory"),
            VfsError::IsADirectory           => write!(f, "vfs: is a directory"),
            VfsError::Io(msg)                => write!(f, "vfs: I/O error: {}", msg),
            VfsError::ReadOnly               => write!(f, "vfs: filesystem is read-only"),
            VfsError::AlreadyMounted         => write!(f, "vfs: already mounted"),
            VfsError::UnknownFilesystem(fs)  => write!(f, "vfs: unknown filesystem: {}", fs),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Stat / metadata
// ─────────────────────────────────────────────────────────────────────────────

/// Inode type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeType {
    /// Regular file.
    File,
    /// Directory.
    Directory,
    /// Symbolic link.
    Symlink,
    /// Character device.
    CharDevice,
    /// Block device.
    BlockDevice,
    /// Named pipe (FIFO).
    Fifo,
    /// Unix-domain socket.
    Socket,
}

/// File stat information, analogous to POSIX `struct stat`.
#[derive(Debug, Clone)]
pub struct VfsStat {
    /// Inode number (unique within a filesystem).
    pub ino: u64,
    /// Type of this inode.
    pub inode_type: InodeType,
    /// File size in bytes.
    pub size: u64,
    /// Permissions bitmask (Unix `rwxrwxrwx`).
    pub mode: u32,
    /// Owner user ID.
    pub uid: u32,
    /// Owner group ID.
    pub gid: u32,
    /// Number of hard links.
    pub nlink: u32,
    /// Last-access time (seconds since epoch).
    pub atime: u64,
    /// Last-modification time.
    pub mtime: u64,
    /// Status-change time.
    pub ctime: u64,
    /// Block size used by this filesystem.
    pub blksize: u32,
    /// Number of 512-byte blocks allocated.
    pub blocks: u64,
}

/// A single entry in a directory listing.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Filename (not including path).
    pub name: String,
    /// Inode number.
    pub ino: u64,
    /// Inode type.
    pub inode_type: InodeType,
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsNode trait
// ─────────────────────────────────────────────────────────────────────────────

/// Core trait implemented by every VFS object.
///
/// Filesystem drivers provide concrete implementations; the VFS layer calls
/// these methods through trait objects.
pub trait VfsNode: Send + Sync {
    /// Read up to `buf.len()` bytes starting at `offset`.
    ///
    /// Returns the number of bytes actually read (0 = EOF).
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize, VfsError>;

    /// Write `data` at `offset`.
    ///
    /// Returns the number of bytes written.
    fn write(&self, data: &[u8], offset: u64) -> Result<usize, VfsError>;

    /// Return metadata for this node.
    fn stat(&self) -> Result<VfsStat, VfsError>;

    /// List the contents of a directory.
    ///
    /// Returns `VfsError::NotADirectory` if called on a non-directory node.
    fn readdir(&self) -> Result<Vec<DirEntry>, VfsError>;

    /// Follow a symlink target (one level).
    ///
    /// Returns `VfsError::NotSupported` for non-symlink nodes.
    fn readlink(&self) -> Result<String, VfsError> {
        Err(VfsError::NotSupported)
    }

    /// Return the inode type of this node.
    fn inode_type(&self) -> InodeType;
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsSuperblock trait
// ─────────────────────────────────────────────────────────────────────────────

/// Trait for filesystem registration with the VFS.
///
/// Every concrete filesystem (Sigma-EXT, tmpfs, procfs, …) implements this
/// trait and registers with [`VfsContext::register_filesystem`].
pub trait VfsSuperblock: Send + Sync {
    /// The name of this filesystem type (e.g., `"sigma-ext"`, `"tmpfs"`).
    fn fs_type(&self) -> &str;

    /// Mount this filesystem at the given device path.
    ///
    /// `source` is the block-device path (or `""` for virtual filesystems).
    /// Returns an `Arc<dyn VfsNode>` pointing at the root inode.
    fn mount(&self, source: &str, flags: MountFlags) -> Result<Arc<dyn VfsNode>, VfsError>;

    /// Unmount and flush any pending I/O.
    fn umount(&self) -> Result<(), VfsError>;

    /// Return filesystem statistics (free blocks, total blocks, …).
    fn statfs(&self) -> FsStats;
}

// ─────────────────────────────────────────────────────────────────────────────
// Mount flags / stats
// ─────────────────────────────────────────────────────────────────────────────

/// Flags controlling mount behaviour.
#[derive(Debug, Clone, Copy, Default)]
pub struct MountFlags {
    /// Mount read-only.
    pub read_only: bool,
    /// Do not allow `setuid` executables.
    pub no_suid: bool,
    /// Do not interpret device files.
    pub no_dev: bool,
    /// Do not allow execution of binaries.
    pub no_exec: bool,
    /// Disable access-time updates.
    pub no_atime: bool,
}

/// Filesystem-level statistics.
#[derive(Debug, Clone, Copy)]
pub struct FsStats {
    /// Total number of data blocks.
    pub total_blocks: u64,
    /// Number of free data blocks.
    pub free_blocks: u64,
    /// Total number of inodes.
    pub total_inodes: u64,
    /// Number of free inodes.
    pub free_inodes: u64,
    /// Preferred block size.
    pub block_size: u32,
    /// Maximum filename length.
    pub max_name_len: u32,
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsMount
// ─────────────────────────────────────────────────────────────────────────────

/// A single entry in the VFS mount table.
///
/// Analogous to `struct mount` in the Linux kernel.
#[derive(Clone)]
pub struct VfsMount {
    /// Absolute mount point path (e.g., `/proc`).
    pub mount_point: String,
    /// Filesystem type name (e.g., `"procfs"`).
    pub fs_type: String,
    /// Source device (or `""` for virtual filesystems).
    pub source: String,
    /// Mount flags in effect.
    pub flags: MountFlags,
    /// Root inode of the mounted filesystem.
    pub root: Arc<dyn VfsNode>,
}

impl core::fmt::Debug for VfsMount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VfsMount")
            .field("mount_point", &self.mount_point)
            .field("fs_type", &self.fs_type)
            .field("source", &self.source)
            .finish()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsDentry — directory-entry cache
// ─────────────────────────────────────────────────────────────────────────────

/// A cached directory entry.
///
/// The dentry cache (dcache) maps `(parent, name)` tuples to inodes, avoiding
/// repeated filesystem lookups for frequently-accessed paths.
#[derive(Clone)]
pub struct VfsDentry {
    /// The filename component (not a full path).
    pub name: String,
    /// The inode this entry points to.
    pub inode: Arc<VfsInode>,
    /// Reference count (shared with inode).
    ref_count: Arc<AtomicUsize>,
}

impl VfsDentry {
    /// Create a new dentry.
    pub fn new(name: String, inode: Arc<VfsInode>) -> Self {
        VfsDentry {
            name,
            inode,
            ref_count: Arc::new(AtomicUsize::new(1)),
        }
    }

    /// Increment the reference count.
    pub fn get(&self) -> usize {
        self.ref_count.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Decrement the reference count and return the new value.
    pub fn put(&self) -> usize {
        let prev = self.ref_count.fetch_sub(1, Ordering::Release);
        if prev == 1 {
            // Would reach zero → could evict from dcache
        }
        prev - 1
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsInode — inode abstraction
// ─────────────────────────────────────────────────────────────────────────────

/// An inode abstraction with reference counting.
///
/// In the Linux kernel an `inode` holds metadata and a pointer to
/// `inode_operations`. Here we wrap a `Box<dyn VfsNode>` for the filesystem-
/// specific implementation.
pub struct VfsInode {
    /// Unique inode number within the filesystem.
    pub ino: u64,
    /// Type of this inode.
    pub inode_type: InodeType,
    /// Reference count.
    pub ref_count: AtomicUsize,
    /// The underlying filesystem-specific node.
    pub node: Box<dyn VfsNode>,
}

impl VfsInode {
    /// Create a new inode wrapping `node`.
    pub fn new(ino: u64, inode_type: InodeType, node: Box<dyn VfsNode>) -> Self {
        VfsInode {
            ino,
            inode_type,
            ref_count: AtomicUsize::new(1),
            node,
        }
    }

    /// Increment the reference count (iget).
    pub fn iget(&self) {
        self.ref_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement the reference count (iput).  Returns the new count.
    pub fn iput(&self) -> usize {
        let prev = self.ref_count.fetch_sub(1, Ordering::Release);
        prev - 1
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsPath — path resolution
// ─────────────────────────────────────────────────────────────────────────────

/// Path resolution with symlink following.
///
/// Splits a path into components and resolves them against the mount table and
/// dentry cache.
pub struct VfsPath {
    raw: String,
}

impl VfsPath {
    /// Create a `VfsPath` from a string slice.
    pub fn new(path: &str) -> Self {
        VfsPath { raw: path.to_string() }
    }

    /// Return the path as a string.
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Iterate over the path components (split by `/`, skipping empty).
    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.raw.split('/').filter(|c| !c.is_empty())
    }

    /// Return `true` if this is an absolute path (starts with `/`).
    pub fn is_absolute(&self) -> bool {
        self.raw.starts_with('/')
    }

    /// Join this path with another component.
    pub fn join(&self, component: &str) -> VfsPath {
        if self.raw.ends_with('/') {
            VfsPath::new(&alloc::format!("{}{}", self.raw, component))
        } else {
            VfsPath::new(&alloc::format!("{}/{}", self.raw, component))
        }
    }

    /// Return the parent path.
    pub fn parent(&self) -> VfsPath {
        let trimmed = self.raw.trim_end_matches('/');
        match trimmed.rfind('/') {
            Some(idx) if idx > 0 => VfsPath::new(&trimmed[..idx]),
            Some(_) => VfsPath::new("/"),
            None => VfsPath::new("/"),
        }
    }

    /// Return the final component of the path (filename).
    pub fn file_name(&self) -> &str {
        let trimmed = self.raw.trim_end_matches('/');
        match trimmed.rfind('/') {
            Some(idx) => &trimmed[idx + 1..],
            None => trimmed,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VfsContext — the main VFS state
// ─────────────────────────────────────────────────────────────────────────────

/// The global VFS context.
///
/// Holds the mount table and the registry of known filesystem types.
pub struct VfsContext {
    /// Mount table, sorted by mount-point length (longest first for prefix
    /// matching).
    mounts: Vec<VfsMount>,
    /// Registered filesystem drivers.
    filesystems: Vec<Box<dyn VfsSuperblock>>,
}

impl VfsContext {
    /// Create an empty VFS context.
    pub fn new() -> Self {
        VfsContext {
            mounts: Vec::new(),
            filesystems: Vec::new(),
        }
    }

    /// Register a filesystem driver.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// ctx.register_filesystem(Box::new(TmpFsSuperblock::new()));
    /// ```
    pub fn register_filesystem(&mut self, sb: Box<dyn VfsSuperblock>) {
        self.filesystems.push(sb);
    }

    /// Mount `fs_type` from `source` at `mount_point`.
    ///
    /// # Errors
    ///
    /// - [`VfsError::UnknownFilesystem`] if no driver for `fs_type` is registered.
    /// - [`VfsError::AlreadyMounted`] if `mount_point` already has a mount.
    pub fn mount(
        &mut self,
        fs_type: &str,
        source: &str,
        mount_point: &str,
        flags: MountFlags,
    ) -> Result<(), VfsError> {
        if self.mounts.iter().any(|m| m.mount_point == mount_point) {
            return Err(VfsError::AlreadyMounted);
        }
        let sb = self
            .filesystems
            .iter()
            .find(|sb| sb.fs_type() == fs_type)
            .ok_or_else(|| VfsError::UnknownFilesystem(fs_type.to_string()))?;

        let root = sb.mount(source, flags)?;
        self.mounts.push(VfsMount {
            mount_point: mount_point.to_string(),
            fs_type: fs_type.to_string(),
            source: source.to_string(),
            flags,
            root,
        });
        // Keep longest-prefix first for correct resolution.
        self.mounts.sort_by(|a, b| b.mount_point.len().cmp(&a.mount_point.len()));
        Ok(())
    }

    /// Unmount the filesystem at `mount_point`.
    pub fn umount(&mut self, mount_point: &str) -> Result<(), VfsError> {
        let idx = self
            .mounts
            .iter()
            .position(|m| m.mount_point == mount_point)
            .ok_or_else(|| VfsError::NotFound(mount_point.to_string()))?;
        self.mounts.remove(idx);
        Ok(())
    }

    /// Resolve `path` to its filesystem root inode.
    ///
    /// Finds the longest mount-point prefix that matches `path`.
    pub fn resolve(&self, path: &str) -> Result<Arc<dyn VfsNode>, VfsError> {
        for mount in &self.mounts {
            if path.starts_with(&mount.mount_point) {
                return Ok(Arc::clone(&mount.root));
            }
        }
        Err(VfsError::NotFound(path.to_string()))
    }

    /// List all current mount points.
    pub fn list_mounts(&self) -> Vec<&VfsMount> {
        self.mounts.iter().collect()
    }
}

impl Default for VfsContext {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Built-in: TmpfsNode (in-memory, zero-content stub)
// ─────────────────────────────────────────────────────────────────────────────

/// A minimal in-memory (tmpfs) node for testing and bootstrapping.
pub struct TmpfsNode {
    stat: VfsStat,
}

impl TmpfsNode {
    /// Create a new tmpfs root directory node.
    pub fn root() -> Self {
        TmpfsNode {
            stat: VfsStat {
                ino: 1,
                inode_type: InodeType::Directory,
                size: 0,
                mode: 0o755,
                uid: 0,
                gid: 0,
                nlink: 2,
                atime: 0,
                mtime: 0,
                ctime: 0,
                blksize: 4096,
                blocks: 0,
            },
        }
    }
}

impl VfsNode for TmpfsNode {
    fn read(&self, _buf: &mut [u8], _offset: u64) -> Result<usize, VfsError> {
        Err(VfsError::IsADirectory)
    }
    fn write(&self, _data: &[u8], _offset: u64) -> Result<usize, VfsError> {
        Err(VfsError::IsADirectory)
    }
    fn stat(&self) -> Result<VfsStat, VfsError> {
        Ok(self.stat.clone())
    }
    fn readdir(&self) -> Result<Vec<DirEntry>, VfsError> {
        Ok(alloc::vec![
            DirEntry { name: ".".to_string(),  ino: 1, inode_type: InodeType::Directory },
            DirEntry { name: "..".to_string(), ino: 1, inode_type: InodeType::Directory },
        ])
    }
    fn inode_type(&self) -> InodeType {
        InodeType::Directory
    }
}

/// Superblock for the built-in tmpfs.
pub struct TmpfsSuperblock;

impl VfsSuperblock for TmpfsSuperblock {
    fn fs_type(&self) -> &str { "tmpfs" }

    fn mount(&self, _source: &str, _flags: MountFlags) -> Result<Arc<dyn VfsNode>, VfsError> {
        Ok(Arc::new(TmpfsNode::root()))
    }

    fn umount(&self) -> Result<(), VfsError> { Ok(()) }

    fn statfs(&self) -> FsStats {
        FsStats {
            total_blocks: u64::MAX,
            free_blocks: u64::MAX,
            total_inodes: u64::MAX,
            free_inodes: u64::MAX,
            block_size: 4096,
            max_name_len: 255,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_mount_and_resolve() {
        let mut ctx = VfsContext::new();
        ctx.register_filesystem(Box::new(TmpfsSuperblock));
        ctx.mount("tmpfs", "", "/tmp", MountFlags::default()).unwrap();
        let node = ctx.resolve("/tmp/anything").unwrap();
        let stat = node.stat().unwrap();
        assert_eq!(stat.inode_type, InodeType::Directory);
    }

    #[test]
    fn test_vfs_double_mount_error() {
        let mut ctx = VfsContext::new();
        ctx.register_filesystem(Box::new(TmpfsSuperblock));
        ctx.mount("tmpfs", "", "/mnt", MountFlags::default()).unwrap();
        let res = ctx.mount("tmpfs", "", "/mnt", MountFlags::default());
        assert_eq!(res, Err(VfsError::AlreadyMounted));
    }

    #[test]
    fn test_vfs_path_components() {
        let p = VfsPath::new("/usr/local/bin/sigma");
        let parts: Vec<&str> = p.components().collect();
        assert_eq!(parts, &["usr", "local", "bin", "sigma"]);
    }

    #[test]
    fn test_vfs_path_parent() {
        let p = VfsPath::new("/usr/local/bin/sigma");
        assert_eq!(p.parent().as_str(), "/usr/local/bin");
        assert_eq!(p.file_name(), "sigma");
    }

    #[test]
    fn test_vfs_tmpfs_readdir() {
        let node = TmpfsNode::root();
        let entries = node.readdir().unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, ".");
    }

    #[test]
    fn test_vfs_unknown_fs_error() {
        let mut ctx = VfsContext::new();
        let res = ctx.mount("ext4", "/dev/sda1", "/mnt", MountFlags::default());
        assert!(matches!(res, Err(VfsError::UnknownFilesystem(_))));
    }
}
