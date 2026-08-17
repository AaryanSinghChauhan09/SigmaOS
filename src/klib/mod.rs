extern crate alloc;

// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod async_runtime;
pub mod error;
pub mod isa;
pub mod store;
pub mod string;
pub mod vec;
pub mod buddy_allocator;
pub mod paging;
#[cfg(target_os = "none")]
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;
pub mod btreemap;
pub mod vecdeque;
pub mod hash;
pub mod time;
pub mod math;
pub mod uuid;
pub mod conversion;

// Re-exports
pub use string::{String, ToString};
pub use vec::Vec;
pub use hashmap::{BTreeMap as HashMap, Entry};
pub use hashmap::BTreeMap;
pub use hashset::HashSet;
pub use arc::Arc;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};
pub use btreemap::BTreeMap as StdBTreeMap;
pub use vecdeque::VecDeque;
pub use time::{Duration, Instant, monotonic_ms};
pub use hash::{djb2_hash, simple_hash, fnv1a_hash, xor_hash, SimpleHasher, combine_hashes};
pub use math::{abs, min, max, clamp, pow, log2, sqrt};

<<<<<<< HEAD
/// Format integer to string without std::fmt
pub fn format_int(mut num: u64) -> alloc::string::String {
    if num == 0 {
        return alloc::string::String::from("0");
    }
    let mut buffer = alloc::vec::Vec::new();
    while num > 0 {
        let digit = (num % 10) as u8;
        buffer.push(b'0' + digit);
        num /= 10;
    }
    buffer.reverse();
    alloc::string::String::from_utf8(buffer).unwrap_or_else(|_| alloc::string::String::from("ERR"))
}

/// C-ABI compatible string length function
#[no_mangle]
pub unsafe extern "C" fn sigma_strlen(s: *const core::ffi::c_char) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

/// C-ABI compatible memory comparison function
#[no_mangle]
pub unsafe extern "C" fn sigma_memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return (a as i32) - (b as i32);
        }
    }
    0
}
=======
#[cfg(not(target_os = "none"))]
pub use std::vec::Vec;

#[cfg(target_os = "none")]
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
>>>>>>> origin/improve-security-and-access-control-16390481506940537632
