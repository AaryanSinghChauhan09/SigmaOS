// Memory management module - exports Phase G memory components
pub mod buddy_allocator;
pub mod slab_allocator;
pub mod page_table_walker;

pub use buddy_allocator::*;
pub use slab_allocator::*;
pub use page_table_walker::*;
