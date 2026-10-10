// SigmaOS Filesystem Module
pub mod archive;
pub mod bsd_linux_innovations;
pub mod btrfs;
pub mod btrfs_snapshots;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod ext4;
pub mod ext4_mount;
pub mod ext4_ntfs_security;
pub mod file_monitor;
pub mod manager;
pub mod mount_namespace;
pub mod sigma_fs;
pub mod smart_symlink;
pub mod snapshot_manager;
pub mod support;
pub mod tmpfs;
pub mod vfs;
pub mod watch;
pub mod zfs;
pub mod zfs_arc;

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
pub use cow_snapshot::{
    CowSnapshot, CowSnapshotManager, DedupeEntry, DedupeTable, ExtentRef, FileTransaction,
    MerkleNode, MerkleTree, SnapshotState,
};
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
pub mod sovereign_link_engine;
pub use btrfs::{
    BtrfsCompression, BtrfsError, BtrfsFilesystem, BtrfsRaidLevel, BtrfsSnapshot, BtrfsStats,
    BtrfsSuperblock,
};
pub use btrfs_snapshots::{
    BtrfsExtentTree, BtrfsSnapshotEngine, BtrfsSnapshotError, BtrfsSubvolume, DiffKind, ExtentDiff,
    ExtentFlags, ExtentItem,
};
pub use ext4::{
    Ext4Error, Ext4Extent, Ext4ExtentHeader, Ext4Filesystem, Ext4GroupDesc, Ext4Inode, Ext4Stats,
    Ext4Superblock,
};
pub use fscrypt_autofs::{
    AutofsMountTrigger, FscryptInodeRecord, FscryptPolicy, SovereignFscryptAutofsEngine,
};
pub use snapshot_manager::{
    RetentionPolicy, SnapshotBackend, SnapshotManager, SnapshotManagerError, SnapshotRecord,
    SnapshotSchedule,
};
pub use sovereign_filesystem_hierarchy::{
    EphemeralTmpfsMountGovernor, SovereignAtomicGenerationRootfsGuard,
    SovereignCanonicalFhsResolver, SovereignMultiDistroFhsHierarchyEngine,
    SyntheticProcSysfsProvider,
};
pub use sovereign_link_engine::*;
pub use tmpfs::{TmpfsError, TmpfsFilesystem, TmpfsInode};
pub use watch::{EventQueue, ThreadSafeEventQueue, COALESCE_WINDOW_MS, RING_BUFFER_SIZE};
pub use zfs::{
    DatasetType, PoolState, VdevState, VdevType, ZfsDataset, ZfsError, ZfsPool, ZfsScrubStats,
    ZfsVdev,
};
pub use zfs_arc::{ArcConfig, ArcEntry, ArcStats, ZfsArc};

pub type FileDescriptor = i32;
pub type FilePermissions = u32;

// HAMMER2: DragonFlyBSD-inspired CoW filesystem with dedup and snapshots
pub mod hammer2;
pub use hammer2::{
    Hammer2BlockRef, Hammer2Check, Hammer2Compress, Hammer2Inode, Hammer2PFS, Hammer2Snapshot,
    Hammer2Statfs, Hammer2Volume,
};
