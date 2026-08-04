// SigmaOS Filesystem Module
pub mod smart_symlink;
pub mod vfs;
pub mod cow_snapshot;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use cow_snapshot::{
    CowSnapshot, CowSnapshotManager, FileTransaction, SnapshotState,
};
