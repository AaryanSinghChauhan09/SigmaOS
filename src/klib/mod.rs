pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod adt;
pub mod io;
pub mod custom_string;
pub mod custom_allocator;

// New modules for std elimination
pub mod env;
pub mod fs;
pub mod process;
pub mod console;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;
pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};
pub use custom_string::SigmaString;
pub use io::{KlibRead, KlibWrite};
pub use custom_allocator::SIGMA_ALLOCATOR;

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;
