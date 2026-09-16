// SigmaOS Filesystem Module
pub mod archive;
pub mod bsd_linux_innovations;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod erofs;
pub mod ext4;
pub mod ext4_mount;
pub mod ext4_ntfs_security;
pub mod file_monitor;
pub mod manager;
pub mod mount_namespace;
pub mod smart_symlink;
pub mod support;
pub mod vfs;
pub mod watch;
pub use bsd_linux_innovations::{
    BsdSoftUpdatesEngine, GoboLinuxPathResolver, LinuxOverlayFsManager, LinuxProcSysfsEmulator,
    MetadataDependency, MetadataOp, OpenBsdMountEnforcer, SovereignFhsHierarchyEngine,
};
pub use erofs::{ErofsEngine, ErofsInode, ErofsInodeFormat, ErofsSuperblock};

pub use crate::filesystem::vfs::{
    DirEntry, FileHandle, FileMode, FileSystem as VfsFileSystem, FileType, Inode, MountPoint,
    VfsError, VfsError as FsError, VirtualFileSystem, VirtualFileSystem as VirtualFilesystem,
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
pub use ext4::{BlockGroupDescriptor, Ext4FileSystem, Ext4Superblock as Ext4SB};
pub use file_monitor::{
    EventFilter, EventId, FileEvent, FileEventType, WatchConfig, WatchId, WatchManager,
};
pub use manager::{
    ClipboardOperation, FileItem, FileManager, FileManagerError, FileOperation,
    FileType as ManagerFileType, SortOrder, StandardFileOperation, ViewMode,
};
pub use mount_namespace::{
    MountFlags, MountId, MountInfo, MountNamespace, MountNamespaceStats, MountSource,
};
pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use support::{FilesystemError, FilesystemType, SimpleFilesystem, SimpleFilesystemManager};
pub use watch::{EventQueue, ThreadSafeEventQueue, COALESCE_WINDOW_MS, RING_BUFFER_SIZE};
