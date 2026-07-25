// SigmaOS Filesystem Module
pub mod vfs;
pub mod legacy_fs;

pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use legacy_fs::{
    LegacyFsType, LegacyFSAdapter,
};
