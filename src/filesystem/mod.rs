// SigmaOS Filesystem Module
pub mod smart_symlink;
pub mod vfs;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
