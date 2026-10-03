//! Virtual File System (VFS)
//!
//! POSIX-compatible VFS implementation with path resolution

pub mod overlayfs;
pub mod posix_path;
pub mod ramfs;

pub use overlayfs::{
    overlay_lookup, overlay_mount, overlay_open_read, overlay_open_write, overlay_readdir,
    overlay_unlink, overlay_write, FileHandle, FileLayer, FsLayer, OverlayEntry, OverlayError,
    OverlayFs,
};
pub use posix_path::*;
pub use ramfs::*;
