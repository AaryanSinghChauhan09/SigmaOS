// SigmaOS Filesystem Module
// Virtual filesystem, journaling filesystems (Btrfs, XFS), and storage support

pub mod btrfs;
pub mod filesystem;
pub mod sigmacas;
pub mod sigmafs;
pub mod support;
pub mod vfs;
pub mod xfs;

pub use btrfs::{BtrfsExtent, BtrfsFilesystem, BtrfsSnapshot, BtrfsSubvolume, CompressionType, ChecksumType};
pub use sigmacas::{CasBlock, SigmaFsCasEngine, SHA256_HASH_SIZE, DILITHIUM5_SIGNATURE_SIZE};
pub use sigmafs::{
    AhciSataController, BlockStorageDevice, BlockStorageError, JournalBlock, JournalBlockType,
    MerkleNode, NvmeStorageController, SigmaFs, TransactionalJournal,
};
pub use xfs::{AllocationStrategy, XfsAllocationGroup, XfsExtent, XfsFilesystem, XfsInode, XfsJournal, XfsState};
