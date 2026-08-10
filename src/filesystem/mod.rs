// SigmaOS Filesystem Module
pub mod smart_symlink;
pub mod vfs;
pub mod sigma_fs;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use sigma_fs::{
    SovereignFhsHierarchy, SovereignFsJournal, DistributedSovereignFS, PqcFileEncryptor,
    SigmaFS, SigmaFhsRouter, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsAuditor,
    JournalState, RaidLevel, SigmaFsJournal, SigmaFsCow, SigmaFsVolume, SigmaFsRaid,
    SigmaFsCrypt, SigmaFsVirtio,
};
