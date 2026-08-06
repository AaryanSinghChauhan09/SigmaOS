// SigmaOS Filesystem Module
pub mod archive;
pub mod btrfs_inspired;
pub mod complete_filesystems;
pub mod cow_snapshot;
pub mod disk_usage;
pub mod manager;
pub mod support;
pub mod tmpfs;
pub mod vfs;
pub mod zfs_inspired;

pub use tmpfs::{TmpfsFileSystem, TmpfsConfig, TmpfsInode, TmpfsFileType};
pub use archive::{
    ArchiveEntry, ArchiveError, ArchiveFormat, ArchiveHandler, ArchiveManager, ArchiveResult,
    CompressionLevel, TarArchiveHandler, ZipArchiveHandler,
};
pub use complete_filesystems::{
    BtrfsFileSystem, ExFatFileSystem, ExtFileSystem, ExtVersion, FatFileSystem, FatVersion,
    FileSystem, HfsPlusFileSystem, NtfsFileSystem,
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
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem, SimpleFilesystemManager,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use zfs_inspired::{
    AdaptiveReplacementCache, CompressionAlgorithm, DatasetConfig, DatasetManager, DeduplicationTable,
    RaidLevel, Snapshot, SnapshotManager, StorageStats, WriteSync, ZfsInspiredFilesystem, Zpool, ZpoolManager,
    ChecksumAlgorithm,
};
pub use btrfs_inspired::{
    BtrfsCompression, BtrfsDevice, BtrfsManager, BtrfsRaidProfile, BtrfsStats, Chunk, QuotaManager,
    SendReceiveManager, Subvolume, SubvolumeManager,
};
