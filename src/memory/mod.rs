#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

// SigmaOS Memory Module
// Virtual memory management, paging, physical memory allocation, and TLB caching

pub mod quota;
pub mod cgroups;
pub mod kswapd;
pub mod paging;
pub mod segmentation_paging;
pub mod tlb_associative;
pub mod zone;
pub mod demand_paging;
pub mod asid_pcid;
pub mod mglru_aging;

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
pub use quota::{MemoryController, MemoryStats, MemoryUnit, OomEvent, OomPolicy, ProcessMemoryAccount, PageCacheStat};
pub use demand_paging::{
    DemandPagingManager, LruPageReplacer, MemoryPage, DemandPagingStats,
    PageFaultError, PageState, PageTableFlags as DemandPagingFlags, Pfn, SwapDevice, SwapEntry,
    VirtualAddress as DemandPagingVirtualAddress,
};
pub use asid_pcid::{
    AddressSpaceIdentifierEngine, InvpcidMode, PcidDescriptor, CR3_NOFLUSH_BIT, KPTI_USER_PCID_MASK, MAX_X86_PCID,
};
pub use mglru_aging::{
    MglruPageAgingEngine, MglruPageDescriptor, MAX_MGLRU_GENERATIONS, PAGE_ACCESSED_BIT, PAGE_DIRTY_BIT,
};
