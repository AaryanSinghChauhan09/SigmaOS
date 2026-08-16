// SigmaOS Filesystem Module
pub mod archive;
pub mod cow_snapshot;
pub mod defragmenter;
pub mod disk_usage;
pub mod manager;
pub mod support;
pub mod vfs;

pub mod bsd_linux_innovations;
pub mod complete_filesystems;
pub mod legacy_fs;
pub mod linux_package_parity;
pub mod s_fs;
pub mod sigma_fs;
pub mod sigmafs;
pub mod smart_symlink;
pub mod vdbe_doom;

pub use bsd_linux_innovations::{
    BsdSoftUpdatesEngine, MetadataDependency, MetadataOp,
    OpenBsdMountEnforcer, MNT_RDONLY, MNT_NOEXEC, MNT_NOSUID, MNT_NODEV,
    LinuxOverlayFsManager, LinuxProcSysfsEmulator,
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
pub use support::{
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, LegacyLinuxRule,
    LinuxPersonaRule, SimpleFilesystem, SimpleFilesystemManager, SmartSymlink, SymlinkResolverRule,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
