//! # sigmaos-filesystem
//!
//! SigmaOS Virtual Filesystem layer: VFS abstraction, ext4 driver stub, tmpfs,
//! devfs, mount manager, POSIX file operations, and inode operations.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │              User-space / syscall layer  │
//! └──────────────────────┬──────────────────┘
//!                        │ POSIX open/read/write/close/pipe
//! ┌──────────────────────▼──────────────────┐
//! │              VFS layer  (vfs.rs)         │
//! │  VfsManager · MountTable · PathResolver  │
//! └──────┬───────────────────────────┬───────┘
//!        │                           │
//! ┌──────▼──────┐           ┌────────▼──────────────┐
//! │  ext4 driver│           │  tmpfs / devfs drivers │
//! │  (ext4.rs)  │           │  (tmpfs.rs / devfs.rs) │
//! └─────────────┘           └────────────────────────┘
//!        │
//! ┌──────▼──────────────────────────────────┐
//! │  MountManager  (mount.rs)               │
//! │  InodeTable    (inode.rs)               │
//! │  FdTable       (posix.rs)               │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Principles applied
//! - **Separation of concerns** — VFS abstracts concrete FS implementations.
//! - **Stable APIs** — public traits (`InodeOps`, `FileOps`, `SuperblockOps`)
//!   are versioned and backward-compatible.
//! - **Minimal kernel philosophy** — this crate carries no kernel dependency;
//!   the `kernel` feature gate (see below) can wire in kernel types via traits.
//! - **POSIX compliance** — file types, modes, fd semantics follow POSIX.
//! - **Memory safety** — no `unsafe` code in the public API surface.
//! - **Single responsibility** — one module per concern.
//!
//! ## Cross-crate kernel integration
//!
//! When compiled with `features = ["kernel"]` (not yet wired — placeholder),
//! this crate can depend on `sigmaos-kernel` through trait objects so the VFS
//! layer can request physical block I/O without a hard compile-time dependency.

// ---------------------------------------------------------------------------
// Modules
// ---------------------------------------------------------------------------

pub mod vfs;
pub mod mount;
pub mod tmpfs;
pub mod ext4;
pub mod devfs;
pub mod posix;
pub mod inode;

// ---------------------------------------------------------------------------
// Re-exports — stable public API surface
// ---------------------------------------------------------------------------

// VFS layer
pub use vfs::{
    FileType, FileMode, OpenFlags, SeekWhence,
    InodeMetadata, Inode,
    InodeOps, FileOps, SuperblockOps, VirtualFilesystem,
    FilesystemStats, VfsMountFlags,
    Dentry, MountPoint, MountTable,
    PathResolver, VfsManager,
    VfsError, FsError,
};

// Mount manager
pub use mount::{
    MountError, MountState, MountFlags, MountEntry, MountManager,
    mount, unmount,
};

// tmpfs
pub use tmpfs::{
    TmpfsError, InodeType as TmpfsInodeType, TmpfsInode,
    TmpfsFilesystem, TmpfsStats,
};

// ext4
pub use ext4::{
    EXT4_MAGIC,
    Ext4Error, Ext4Superblock, Ext4GroupDesc, Ext4Inode,
    Ext4ExtentHeader, Ext4Extent, Ext4Stats, Ext4Filesystem,
};

// devfs
pub use devfs::{
    DevfsError, DevNodeKind, DevNode, DevFs,
};

// POSIX file operations
pub use posix::{
    PosixError, PosixResult, OFlags, FdTable, OpenFile,
};

// Inode operations
pub use inode::{
    InodeError, StatInfo, InodeRecord, InodeTable,
};
