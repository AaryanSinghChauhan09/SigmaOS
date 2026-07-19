// SigmaOS Filesystem Module
pub mod archive;
pub mod manager;
pub mod vfs;

pub use archive::{
    ArchiveEntry, ArchiveError, ArchiveFormat, ArchiveHandler, ArchiveManager, ArchiveResult,
    CompressionLevel, TarArchiveHandler, ZipArchiveHandler,
};
pub use manager::{
    ClipboardOperation, FileItem, FileManager, FileManagerError, FileOperation, FileType as ManagerFileType,
    SortOrder, StandardFileOperation, ViewMode,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
