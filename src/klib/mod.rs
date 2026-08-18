extern crate alloc;

pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
pub mod error;
pub mod uuid;
pub mod hash;
pub mod string;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;
pub mod btreemap;
pub mod vecdeque;
pub mod adt;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};
#[cfg(target_os = "none")]
pub use hashset::HashSet;
pub use uuid::Uuid;
#[cfg(target_os = "none")]
pub use string::String;
pub use string::ToString;
pub use arc::Arc;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};
pub use hashmap::{HashMap, BTreeMap};
pub use hashset::HashSet;
pub use uuid::Uuid;
