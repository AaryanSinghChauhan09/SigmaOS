// SigmaOS Filesystem Module
pub mod configfs;
pub mod proc;
pub mod vfs;
pub mod cow_snapshot;

pub use configfs::{ConfigFileNode, ConfigFileType, SovereignConfigFS};
pub use proc::SovereignProcFS;
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use cow_snapshot::{
    CowSnapshot, CowSnapshotManager, FileTransaction, SnapshotState,
};
