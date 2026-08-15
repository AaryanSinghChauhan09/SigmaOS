pub mod vec;
pub mod paging;
#[cfg(target_os = "none")]
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
pub mod vec;
pub mod error;
pub mod uuid;
pub mod hash;

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
