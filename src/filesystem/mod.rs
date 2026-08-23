// SigmaOS Filesystem Module
pub mod archive;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod manager;
pub mod self_healing_fs;
pub mod support;
pub mod sigma_fs;
pub mod smart_symlink;
pub mod tmpfs;
pub mod vfs;
pub mod legacy_fs;
pub mod sigma_fs;
pub mod smart_symlink;
pub use smart_symlink::{
    LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule,
};

pub use geom::{GeomClass, GeomProvider, GeomConsumer, GeomAccessRights};

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
pub use support::{
    FilesystemError, FilesystemType, SimpleFilesystem, SimpleFilesystemManager,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
