// SigmaOS Filesystem Module
pub mod archive;
pub mod complete_filesystems;
pub mod cow_snapshot;
pub mod disk_usage;
pub mod manager;
pub mod smart_symlink;
pub mod support;
pub mod vfs;
pub mod vdbe_doom;

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
pub use smart_symlink::{
    LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkKernelPersona, SymlinkResolverRule,
};
pub use support::{
    Filesystem, FilesystemError, FilesystemManager, FilesystemType, SimpleFilesystem,
    SimpleFilesystemManager,
};
pub use vfs::{FileDescriptor, FilePermissions, FileType, FsError, Inode, VirtualFilesystem};
pub use vdbe_doom::{VdbeCc, VdbeOpcode, VdbeVirtualMachine};
