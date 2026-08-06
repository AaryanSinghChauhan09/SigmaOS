// SigmaOS Filesystem Module
pub mod archive;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod legacy_fs;
pub mod manager;
pub mod sigma_fs;
pub mod smart_symlink;
pub mod support;
pub mod vfs;

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
pub use legacy_fs::{LegacyFSAdapter, LegacyFsType};
pub use manager::{
    ClipboardOperation, FileItem, FileManager, FileManagerError, FileOperation,
    FileType as ManagerFileType, SortOrder, StandardFileOperation, ViewMode,
};
pub use sigma_fs::{
    FileBlock, SigmaDisasterRecoveryCleaner, SigmaFS, SigmaFhsAuditor, SigmaFhsHook,
    SigmaFhsNamespace, SigmaFhsRouter, SigmaFsCow, SigmaFsCrypt, SigmaFsJournal, SigmaFsRaid,
    SigmaFsVirtio, SigmaFsVolume,
};
pub use smart_symlink::{SmartSymlink, SymlinkError, SymlinkResolverRule, LegacyLinuxRule, LinuxPersonaRule};
pub use support::{
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem, SimpleFilesystemManager,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
