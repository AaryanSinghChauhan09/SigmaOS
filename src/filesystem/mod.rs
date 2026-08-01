// SigmaOS Filesystem Module
pub mod vfs;
pub mod cow_snapshot;

pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use cow_snapshot::{
    CowSnapshot, CowSnapshotManager, FileTransaction, SnapshotState,
};
