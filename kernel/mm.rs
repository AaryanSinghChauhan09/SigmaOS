// Memory management module - exports Phase G memory components
#[path = "mm/buddy_allocator.rs"]
pub mod buddy_allocator;
#[path = "mm/slab_allocator.rs"]
pub mod slab_allocator;
#[path = "mm/page_table_walker.rs"]
pub mod page_table_walker;

pub use buddy_allocator::*;
pub use slab_allocator::*;
pub use page_table_walker::*;
