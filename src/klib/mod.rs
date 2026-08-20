// SigmaOS Kernel Library
extern crate alloc;

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

pub use vec::Vec;
pub use hashmap::{HashMap, Entry};
pub use hashset::HashSet;
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
pub use alloc::string::{String, ToString};
pub use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid([u8; 16]);

impl Uuid {
    pub fn new_v4() -> Self {
        Uuid([0x12; 16])
    }
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "00000000-0000-0000-0000-000000000000")
    }
}
