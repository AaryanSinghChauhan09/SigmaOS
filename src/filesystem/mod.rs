// SigmaOS Filesystem Module
pub mod vfs;
pub mod schemes;

pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use schemes::{Scheme, NullScheme, RandScheme, LogScheme, ShmScheme, SchemeRegistry};
