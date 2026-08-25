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

// SigmaOS Memory Module
// Virtual memory management, paging, physical memory allocation, and TLB caching

pub mod cgroups;
pub mod kswapd;
pub mod paging;
pub mod segmentation_paging;
pub mod tlb_associative;
pub mod zone;

pub use paging::{
    MemoryError, PageDirectory, PageDirectoryPointerTable, PageTable, PageTableEntry,
    PhysicalAddress, SimpleVMM, VirtualAddress, PAGE_SIZE_BYTES, PAGE_TABLE_ENTRIES,
};

pub use cgroups::{MemCgroup, MemCgroupManager};
pub use kswapd::{LinuxKswapd, PageState};
pub use segmentation_paging::{
    AddressBindingMode, AddressType, AslrEntropyConfig, CpuRing, ExecutableAddressBinding,
    RandomizedAddressSpace, SegmentDescriptor, SegmentSelector, SegmentationPagingEngine,
    SpaceProtectionFlags, SystemControlRegisters,
};
pub use tlb_associative::{AssociativeTlbCache, TlbAssociativityMode, TlbEntry, TlbPageFlags};
pub use zone::{BsdZoneAllocator, Slab, Zone, ZoneStats};
