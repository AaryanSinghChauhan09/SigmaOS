pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod adt;

#[cfg(target_os = "none")]
pub use vec::Vec;
#[cfg(target_os = "none")]
pub use hashmap::BTreeMap as HashMap;
#[cfg(target_os = "none")]
pub use hashmap::BTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::vec::Vec;
#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap;
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;
#[cfg(not(target_os = "none"))]
pub use std::string::String;

pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};
