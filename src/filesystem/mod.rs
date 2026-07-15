// SigmaOS Filesystem Module
pub mod vfs;

pub use vfs::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
