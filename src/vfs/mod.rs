//! Virtual File System (VFS)
//!
//! POSIX-compatible VFS implementation with path resolution

pub mod posix_path;

pub use posix_path::*;
pub mod ramfs;
