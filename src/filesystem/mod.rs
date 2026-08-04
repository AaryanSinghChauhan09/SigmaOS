// SigmaOS Filesystem Module
pub mod proc;
pub mod vfs;

pub use proc::SovereignProcFS;
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
