#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Memory Module
// Virtual memory management, paging, physical memory allocation, and TLB caching

pub mod paging;
pub mod zone;
pub mod kswapd;
pub mod cgroups;
pub mod segmentation_paging;
pub mod tlb_associative;

pub use paging::{
    MemoryError, PageDirectory, PageDirectoryPointerTable, PageTable, PageTableEntry,
    PhysicalAddress, SimpleVMM, VirtualAddress, PAGE_SIZE_BYTES, PAGE_TABLE_ENTRIES,
};

pub use zone::{BsdZoneAllocator, Zone, ZoneStats, Slab};
pub use kswapd::{LinuxKswapd, PageState};
pub use cgroups::{MemCgroupManager, MemCgroup};
pub use segmentation_paging::{
    AddressBindingMode, AddressType, AslrEntropyConfig, CpuRing, ExecutableAddressBinding,
    RandomizedAddressSpace, SegmentDescriptor, SegmentSelector, SegmentationPagingEngine,
    SpaceProtectionFlags, SystemControlRegisters,
};
pub use tlb_associative::{
    AssociativeTlbCache, TlbAssociativityMode, TlbEntry, TlbPageFlags,
};
