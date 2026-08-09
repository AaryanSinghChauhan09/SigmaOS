extern crate alloc;

// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod buddy_allocator;
pub mod paging;
pub mod vec;
pub mod time;
pub mod math;
pub mod uuid;
pub mod hash;
pub mod string;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;

// Conditional compilation of other collection modules
#[cfg(target_os = "none")]
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
#[cfg(target_os = "none")]
pub mod btreemap;
#[cfg(target_os = "none")]
pub mod vecdeque;

// Re-exports
pub use string::{String, ToString};
pub use arc::Arc;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};

#[cfg(target_os = "none")]
pub use vec::Vec;
#[cfg(target_os = "none")]
pub use hashmap::HashMap;
#[cfg(target_os = "none")]
pub use hashset::HashSet;
#[cfg(target_os = "none")]
pub use btreemap::BTreeMap;
#[cfg(target_os = "none")]
pub use vecdeque::VecDeque;

#[cfg(not(target_os = "none"))]
pub use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
pub use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

pub use time::{Duration, Instant, monotonic_ms};
pub use math::{abs, min, max, clamp, pow, log2, sqrt};
pub use uuid::Uuid;
pub use hash::{djb2_hash, simple_hash, fnv1a_hash, xor_hash, SimpleHasher, combine_hashes};

/// Format integer to string without std::fmt
pub fn format_int(mut num: u64) -> alloc::string::String {
    if num == 0 {
        return alloc::string::String::from("0");
    }
    let mut s = alloc::string::String::new();
    while num > 0 {
        let d = (num % 10) as u8;
        s.insert(0, (b'0' + d) as char);
        num /= 10;
    }
    s
}
