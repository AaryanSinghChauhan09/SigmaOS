// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod custom_allocator;
pub mod custom_string;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
extern crate alloc;
pub use alloc::vec::Vec;
pub use hashmap::BTreeMap as HashMap;
#[cfg(target_os = "none")]
pub use hashmap::BTreeMap;
pub use custom_string::SigmaString;

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
