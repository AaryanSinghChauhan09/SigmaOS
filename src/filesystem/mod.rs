// SigmaOS Filesystem Module
pub mod smart_symlink;
pub mod vfs;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem, O_APPEND,
    O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY,
};
