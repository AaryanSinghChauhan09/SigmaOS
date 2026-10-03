#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Filesystem Module
// Virtual filesystem, journaling filesystems (Btrfs, XFS), and storage support

pub mod btrfs;
// pub mod filesystem;
pub mod btrfs_send_receive;
pub mod vfs;
pub use btrfs::{BtrfsExtent, BtrfsFilesystem, BtrfsSnapshot, ChecksumType};
pub use btrfs_send_receive::{
    BtrfsReceiveContext, BtrfsSendCommand, BtrfsSendOp, BtrfsSendReceiveManager, BtrfsSendStream,
    BtrfsSubvolume,
};
pub mod sigmacas;
pub mod sigmafs;
// pub mod support;
// pub mod vfs;
pub mod autofs;
pub mod fscrypt;
pub mod xfs;
pub use sigmacas::{CasBlock, SigmaFsCasEngine, DILITHIUM5_SIGNATURE_SIZE, SHA256_HASH_SIZE};
pub use sigmafs::{
    AhciSataController, BlockStorageDevice, BlockStorageError, JournalBlock, JournalBlockType,
    MerkleNode, NvmeStorageController, SigmaFs, TransactionalJournal,
};
pub use xfs::{
    AllocationStrategy, XfsAllocationGroup, XfsExtent, XfsFilesystem, XfsInode, XfsJournal,
    XfsState,
};

pub use fscrypt::{EncryptionContext, EncryptionPolicy, FscryptManager};

pub use autofs::{AutoFsManager, MountPoint, MountTriggerType};

pub mod bcachefs_sovereign;
pub use bcachefs_sovereign::{
    sovereign_crc32c, BcachefsExtent, BcachefsInode, BcachefsSnapshot, ChecksumAlgorithm,
    SovereignBcachefsVolume,
};

pub mod overlayfs_sovereign;
pub use overlayfs_sovereign::{OverlayEntry, OverlayEntryKind, OverlayLayer, SovereignOverlayFs};

pub mod zfs_arc_sovereign;
pub use zfs_arc_sovereign::{ArcBufferHeader, SovereignZfsArc};

pub mod fanotify_sovereign;
pub use fanotify_sovereign::{
    FanotifyEvent, FanotifyEventKind, FanotifyMark, FanotifyResponse, SovereignFanotifyGroup,
};
pub use vfs::{
    FilePermissions, FileType, Vfs, VfsDentry, VfsFile, VfsInode, VfsMount, VfsSuperblock,
};
