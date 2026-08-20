// SigmaOS Kernel Library
pub mod vec;
pub mod buddy_allocator;
pub mod paging;
pub mod hashmap;
pub mod hashset;
pub mod btreemap;
pub mod vecdeque;
pub mod error;
pub mod hash;
pub mod custom_string;

pub use custom_string::custom_allocator;
pub use custom_string::uuid;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use custom_string::SigmaString;
pub use vec::Vec;
pub use hashmap::{HashMap, Entry};
pub use hashset::HashSet;
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
