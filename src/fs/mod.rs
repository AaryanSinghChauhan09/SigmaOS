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
pub mod sigmacas;
pub mod sigmafs;
// pub mod support;
// pub mod vfs;
pub mod xfs;

pub use btrfs::{
    BtrfsExtent, BtrfsFilesystem, BtrfsSnapshot, BtrfsSubvolume, ChecksumType, CompressionType,
};
pub use sigmacas::{CasBlock, SigmaFsCasEngine, DILITHIUM5_SIGNATURE_SIZE, SHA256_HASH_SIZE};
pub use sigmafs::{
    AhciSataController, BlockStorageDevice, BlockStorageError, JournalBlock, JournalBlockType,
    MerkleNode, NvmeStorageController, SigmaFs, TransactionalJournal,
};
pub use xfs::{
    AllocationStrategy, XfsAllocationGroup, XfsExtent, XfsFilesystem, XfsInode, XfsJournal,
    XfsState,
};

pub mod bcachefs_sovereign;
pub use bcachefs_sovereign::{SovereignBcachefsVolume, BcachefsInode, BcachefsExtent, BcachefsSnapshot, ChecksumAlgorithm, CompressionType, sovereign_crc32c};

pub mod overlayfs_sovereign;
pub use overlayfs_sovereign::{SovereignOverlayFs, OverlayLayer, OverlayEntry, OverlayEntryKind};

pub mod zfs_arc_sovereign;
pub use zfs_arc_sovereign::{SovereignZfsArc, ArcBufferHeader};

pub mod fanotify_sovereign;
pub use fanotify_sovereign::{SovereignFanotifyGroup, FanotifyEvent, FanotifyEventKind, FanotifyResponse, FanotifyMark};
