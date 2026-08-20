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
pub mod io;
pub mod time;
pub mod net;

pub use vec::Vec;
pub use hashmap::{HashMap, Entry};
pub use hashset::HashSet;
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
pub use custom_string::SigmaString;
pub mod string {
    pub use super::SigmaString as String;
}
pub mod collections {
    pub use super::{HashMap, HashSet, BTreeMap, VecDeque};
}
pub mod path {
    pub use super::custom_string::SigmaString as PathBuf;
}

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
