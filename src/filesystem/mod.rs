// SigmaOS Filesystem Module
pub mod vfs;
pub mod support;

pub use vfs::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
pub use support::{FilesystemManager, SimpleFilesystemManager, Filesystem, SimpleFilesystem, FilesystemType, FilesystemError};
