// SigmaOS Filesystem Module
pub mod smart_symlink;
pub mod vfs;
pub mod legacy_fs;
pub mod sigma_fs;
pub mod smart_symlink;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use legacy_fs::{
    LegacyFsType, LegacyFSAdapter,
};
pub use sigma_fs::{
    FileBlock, SigmaFS,
    SigmaFhsRouter, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsAuditor,
    SigmaDisasterRecoveryCleaner,
    SigmaFsJournal, SigmaFsCow, SigmaFsVolume, SigmaFsRaid, SigmaFsCrypt, SigmaFsVirtio,
};
pub use smart_symlink::{
    SymlinkError, SmartSymlink,
};
