extern crate alloc;

pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
pub mod error;
pub mod uuid;
pub mod hash;
pub mod string;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;
pub mod btreemap;
pub mod vecdeque;
pub mod adt;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use adt::{SplayTree, RadixTree, SovereignPriorityQueue};
#[cfg(target_os = "none")]
pub use hashset::HashSet;
pub use uuid::Uuid;
#[cfg(target_os = "none")]
pub use string::String;
pub use string::ToString;
pub use arc::Arc;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
pub use hash::{djb2_hash, simple_hash, fnv1a_hash, xor_hash, SimpleHasher, combine_hashes};

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::string::String;

// Re-export string utilities
pub use string::format_int;

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
