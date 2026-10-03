// SigmaOS Filesystem Module
pub mod archive;
pub mod bsd_linux_innovations;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod ext4_mount;
pub mod ext4_ntfs_security;
pub mod file_monitor;
pub mod manager;
pub mod mount_namespace;
pub mod sigma_fs;
pub mod smart_symlink;
pub mod support;
pub mod vfs;
pub mod watch;
pub mod ext4;
pub mod tmpfs;
pub mod btrfs;
pub mod zfs;

pub use crate::filesystem::vfs::{FileType, FsError, Inode, VfsError, VirtualFilesystem};
pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
// Removed non-existent vfs exports: FileDescriptor, FilePermissions
pub use crate::filesystem::sigma_fs::{
    JournalState,
    SigmaFS,
    SigmaFhsAuditor,
    SigmaFhsHook,
    SigmaFhsNamespace,
    SigmaFhsRouter,
    SigmaFsCrypt,
    SigmaFsVirtio,
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
    ActivePane, BatchRegexRenamer, ClipboardOperation, FileItem, FileManager, FileManagerError,
    FileOperation, FileTagAnnotation, FileTagColor, FileTagManager, FileType as ManagerFileType,
    SortOrder, SplitPaneView, StandardFileOperation, TabEntry, TabbedBrowsingManager, ViewMode,
    YaziSpatialPreviewEngine,
};
pub use mount_namespace::{
    MountFlags, MountId, MountInfo, MountNamespace, MountNamespaceStats, MountSource,
};
pub use support::{FilesystemError, FilesystemType, SimpleFilesystem, SimpleFilesystemManager};
// Removed duplicate vfs imports - already imported above
// pub use ext4::{Ext4FileSystem, Ext4Superblock as Ext4SB, BlockGroupDescriptor};
pub use file_monitor::{
    EventFilter, EventId, FileEvent, FileEventType, WatchConfig, WatchId, WatchManager,
};
pub mod fscrypt_autofs;
pub mod sovereign_filesystem_hierarchy;
pub use fscrypt_autofs::{
    AutofsMountTrigger, FscryptInodeRecord, FscryptPolicy, SovereignFscryptAutofsEngine,
};
pub use sovereign_filesystem_hierarchy::{
    EphemeralTmpfsMountGovernor, SovereignAtomicGenerationRootfsGuard,
    SovereignCanonicalFhsResolver, SovereignMultiDistroFhsHierarchyEngine,
    SyntheticProcSysfsProvider,
};
pub use watch::{EventQueue, ThreadSafeEventQueue, COALESCE_WINDOW_MS, RING_BUFFER_SIZE};
pub use ext4::{
    Ext4Filesystem, Ext4Superblock, Ext4Inode, Ext4Error, Ext4Stats,
    Ext4ExtentHeader, Ext4Extent, Ext4GroupDesc,
};
pub use tmpfs::{TmpfsFilesystem, TmpfsInode, TmpfsError};
pub use btrfs::{
    BtrfsFilesystem, BtrfsSuperblock, BtrfsError, BtrfsSnapshot,
    BtrfsStats, BtrfsCompression, BtrfsRaidLevel,
};
pub use zfs::{
    ZfsPool, ZfsVdev, ZfsDataset, ZfsError, ZfsScrubStats,
    VdevType, VdevState, PoolState, DatasetType,
};

pub type FileDescriptor = i32;
pub type FilePermissions = u32;
