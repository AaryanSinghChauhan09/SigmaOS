// SigmaOS Filesystem Module
pub mod manager;
pub mod vfs;

pub use manager::{
    ClipboardOperation, FileItem, FileManager, FileManagerError, FileOperation, FileType as ManagerFileType,
    SortOrder, StandardFileOperation, ViewMode,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
