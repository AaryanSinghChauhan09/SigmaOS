pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod custom_string;

pub use custom_string::custom_allocator;
pub use custom_string::uuid;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use custom_string::SigmaString;
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;
