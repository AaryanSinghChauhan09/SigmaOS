pub mod huge_pages;
pub mod numa_aware;
pub mod oom_killer;
pub mod page_cache;
pub mod slab_allocator;
pub mod vmalloc;

pub use huge_pages::{HugePageManager, HugePageSize};
pub use numa_aware::{NumaNode, NumaTopologyManager};
pub use oom_killer::OomKiller;
pub use page_cache::{CachedPage, PageCache, PageStatus};
pub use slab_allocator::{Slab, SlabAllocator, SlabCache};
pub use vmalloc::{VmallocManager, VmallocRegion};
