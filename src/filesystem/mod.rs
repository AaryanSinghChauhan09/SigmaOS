// SigmaOS Filesystem Module
pub mod archive;
pub mod complete_filesystems;
pub mod cow_snapshot;
pub mod disk_usage;
pub mod manager;
pub mod support;
pub mod tmpfs;
pub mod vfs;

<<<<<<< HEAD
||||||| 23ef22a4a
pub use defragmenter::{ClusterState, FragmentedFile, DefragStats, DiskDefragmenter};
=======
pub use tmpfs::{TmpfsFileSystem, TmpfsConfig, TmpfsInode, TmpfsFileType};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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
<<<<<<< HEAD
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, LegacyLinuxRule,
    LinuxPersonaRule, SimpleFilesystem, SimpleFilesystemManager, SmartSymlink, SymlinkResolverRule,
||||||| 23ef22a4a
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem, SimpleFilesystemManager,
=======
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem,
    SimpleFilesystemManager,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
