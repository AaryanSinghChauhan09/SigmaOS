// SigmaOS Filesystem Module
pub mod archive;
pub mod bsd_linux_innovations;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod ext4_ntfs_security;
pub mod manager;
pub mod smart_symlink;
pub mod support;
pub mod vfs;
pub use bsd_linux_innovations::*;

pub use archive::{
    ArchiveEntry, ArchiveError, ArchiveFormat, ArchiveHandler, ArchiveManager, ArchiveResult,
    CompressionLevel, TarArchiveHandler, ZipArchiveHandler,
};
pub use cow_snapshot::{CowSnapshot, CowSnapshotManager, FileTransaction, SnapshotState};
pub use defragmenter::{ClusterState, DefragStats, DiskDefragmenter, FragmentedFile};
pub use disk_usage::{
    AnalysisMode, AnalysisStrategy, DeepAnalysisStrategy, DirectorySizeInfo, DiskUsageAnalyzer,
    DiskUsageError, DiskUsageInfo, FileSizeInfo, QuickAnalysisStrategy,
};
pub use manager::{
    ClipboardOperation, FileItem, FileManager, FileManagerError, FileOperation,
    FileType as ManagerFileType, SortOrder, StandardFileOperation, ViewMode,
};
pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use support::{FilesystemError, FilesystemType, SimpleFilesystem, SimpleFilesystemManager};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
