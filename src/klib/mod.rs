// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod error;
pub mod paging;
pub mod vec;

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
