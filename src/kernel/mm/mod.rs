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
