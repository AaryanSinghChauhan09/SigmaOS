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
pub mod adt;
pub mod vecdeque;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;
pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};
pub use vecdeque::VecDeque;

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;
