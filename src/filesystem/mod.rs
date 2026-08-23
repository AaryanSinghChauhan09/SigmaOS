// SigmaOS Filesystem Module
pub mod archive;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod manager;
pub mod self_healing_fs;
pub mod support;
pub mod sigma_fs;
pub mod smart_symlink;
pub mod tmpfs;
pub mod vfs;

pub use self_healing_fs::{JournalEntry, SovereignFilesystem, TransactionStatus};
pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use sigma_fs::{
    Blake3BlockDeduplicationEngine, JournalState, PseudoFilesystemNamespace,
    SigmaFS, SigmaFhsAuditor, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsRouter,
};
