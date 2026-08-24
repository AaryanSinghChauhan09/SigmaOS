pub mod custom_string;
pub mod net;
pub mod time;
pub mod uuid;
pub mod error;
pub use custom_string::SigmaString as ToString;
// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod adt;

pub use std::vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;
pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};

pub mod string {
    pub use std::string::String;
    pub use crate::klib::custom_string::SigmaString as ToString;
    pub use crate::klib::custom_string::SigmaString;
}
pub mod collections {
    pub use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
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

pub use string::String;
