// SigmaOS Filesystem Module
pub mod archive;
pub mod file_monitor;
pub mod watch;
pub mod bsd_linux_innovations;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod ext4_ntfs_security;
pub mod ext4_mount;
pub mod manager;
pub mod mount_namespace;
pub mod smart_symlink;
pub mod support;
pub mod vfs;
pub mod sigma_fs;

pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use crate::filesystem::vfs::{FileType, FsError, Inode, VirtualFilesystem, VfsError};
// Removed non-existent vfs exports: FileDescriptor, FilePermissions
pub use crate::filesystem::sigma_fs::{
    SigmaFS, SigmaFhsRouter, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsAuditor,
    JournalState, SigmaFsCrypt, SigmaFsVirtio,
    // Removed potentially incomplete exports: RaidLevel, SigmaFsJournal, SigmaFsCow, SigmaFsVolume, SigmaFsRaid
};

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
pub use mount_namespace::{MountId, MountInfo, MountNamespace, MountNamespaceStats, MountSource, MountFlags};
pub use support::{FilesystemError, FilesystemType, SimpleFilesystem, SimpleFilesystemManager};
// Removed duplicate vfs imports - already imported above
// pub use ext4::{Ext4FileSystem, Ext4Superblock as Ext4SB, BlockGroupDescriptor};
pub use file_monitor::{
    EventFilter, FileEvent, FileEventType, WatchConfig, WatchId, WatchManager, EventId,
};
pub use watch::{EventQueue, ThreadSafeEventQueue, RING_BUFFER_SIZE, COALESCE_WINDOW_MS};
