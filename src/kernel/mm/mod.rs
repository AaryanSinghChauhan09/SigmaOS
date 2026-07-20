pub mod slab_allocator;
pub mod vmalloc;
pub mod huge_pages;
pub mod oom_killer;
pub mod numa_aware;

pub use slab_allocator::{SlabAllocator, SlabCache, Slab};
pub use vmalloc::{VmallocManager, VmallocRegion};
pub use huge_pages::{HugePageManager, HugePageSize};
pub use oom_killer::OomKiller;
pub use numa_aware::{NumaTopologyManager, NumaNode};
