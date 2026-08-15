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
pub use smart_symlink::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use sigma_fs::{
    SigmaFS, SigmaFhsRouter, SigmaFhsHook, SigmaFhsNamespace, SigmaFhsAuditor,
    JournalState, RaidLevel, SigmaFsJournal, SigmaFsCow, SigmaFsVolume, SigmaFsRaid,
    SigmaFsCrypt, SigmaFsVirtio,
=======
pub use tmpfs::{TmpfsFileSystem, TmpfsConfig, TmpfsInode, TmpfsFileType};
pub use archive::{
    ArchiveEntry, ArchiveError, ArchiveFormat, ArchiveHandler, ArchiveManager, ArchiveResult,
    CompressionLevel, TarArchiveHandler, ZipArchiveHandler,
>>>>>>> origin/jules-880081283500171861-1eb07604
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
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem,
    SimpleFilesystemManager,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
