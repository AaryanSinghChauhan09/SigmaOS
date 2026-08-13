extern crate alloc;

// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod buddy_allocator;
pub mod paging;
pub mod hashmap;
pub mod hashset;
pub mod uuid;
pub mod hash;
pub mod string;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;
pub mod custom_string;
pub mod custom_allocator;

// Conditional compilation of other collection modules
#[cfg(target_os = "none")]
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
#[cfg(target_os = "none")]
pub mod btreemap;
#[cfg(target_os = "none")]
pub mod vecdeque;

// Re-exports
pub use string::{String, ToString};
pub use arc::Arc;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};
pub use custom_string::{SigmaString, SigmaStringBuilder, CStringView};

#[cfg(target_os = "none")]
pub mod vec;

#[cfg(target_os = "none")]
pub use vec::Vec;
#[cfg(not(target_os = "none"))]
pub use alloc::vec::Vec;
pub use hashmap::HashMap;
pub use hashset::HashSet;
pub use uuid::Uuid;
