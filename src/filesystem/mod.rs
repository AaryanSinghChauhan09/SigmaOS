// SigmaOS Filesystem Module
pub mod archive;
pub mod cow_snapshot;
pub mod disk_usage;
pub mod manager;
pub mod support;
||||||| 43be3a7e8
pub mod smart_symlink;
||||||| 43be3a7e8
pub mod proc;
pub mod vfs;
pub mod defragmenter;

pub use defragmenter::{ClusterState, FragmentedFile, DefragStats, DiskDefragmenter};
pub use archive::{
    ArchiveEntry, ArchiveError, ArchiveFormat, ArchiveHandler, ArchiveManager, ArchiveResult,
    CompressionLevel, TarArchiveHandler, ZipArchiveHandler,
};
pub use cow_snapshot::{CowSnapshot, CowSnapshotManager, FileTransaction, SnapshotState};
pub use disk_usage::{
    AnalysisMode, AnalysisStrategy, DeepAnalysisStrategy, DirectorySizeInfo, DiskUsageAnalyzer,
    DiskUsageError, DiskUsageInfo, FileSizeInfo, QuickAnalysisStrategy,
};
pub use manager::{
    ClipboardOperation, FileItem, FileManager, FileManagerError, FileOperation,
    FileType as ManagerFileType, SortOrder, StandardFileOperation, ViewMode,
};
pub use support::{
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem,
    SimpleFilesystemManager,
};
||||||| 43be3a7e8
pub use proc::SovereignProcFS;
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
||||||| 43be3a7e8
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{
    FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem, O_APPEND,
    O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY,
};
