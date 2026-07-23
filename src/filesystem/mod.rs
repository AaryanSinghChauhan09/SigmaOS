// SigmaOS Filesystem Module
pub mod vfs;

pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
