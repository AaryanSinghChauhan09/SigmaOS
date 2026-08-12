// SigmaOS Filesystem Module
pub mod smart_symlink;
pub mod vfs;
pub mod schemes;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use schemes::{Scheme, NullScheme, RandScheme, LogScheme, ShmScheme, SchemeRegistry};
