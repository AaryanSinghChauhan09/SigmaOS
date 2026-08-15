// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod error;
pub mod paging;
pub mod vec;

#[cfg(target_os = "none")]
pub mod hashmap;
pub mod hashset;
pub mod error;
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
pub mod btreemap;
pub mod vecdeque;
// Re-exports
pub use string::{String, ToString};
pub use arc::Arc;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};
pub use custom_string::{SigmaString, SigmaStringBuilder, CStringView};
pub use vec::Vec;
pub use uuid::Uuid;

#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
#[cfg(not(target_os = "none"))]
pub use std::collections::HashSet;

#[cfg(target_os = "none")]
pub use hashmap::HashMap;
#[cfg(target_os = "none")]
pub use hashset::HashSet;
