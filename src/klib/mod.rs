pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod adt;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;
pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;
