// SigmaOS Kernel Library
pub mod vec;
pub mod buddy_allocator;
pub mod paging;
pub mod uvm;
pub mod btreemap;
pub mod hashset;
pub mod time;
pub mod hashmap;
pub mod hash;

pub use vec::Vec;
pub use btreemap::BTreeMap;
pub use hashset::HashSet;
pub use time::{Duration, Instant};
pub use hash::SimpleHasher;

#[cfg(target_os = "none")]
pub use hashmap::HashMap;
#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
