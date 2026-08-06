// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod async_runtime;
pub mod error;
pub mod isa;
pub mod store;
// SigmaOS Kernel Library

extern crate alloc;

pub mod vec;
// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod paging;
pub mod hashmap;
pub mod hashset;
pub mod arc;
// New sovereign klib modules (Linux/BSD inspired)
pub mod ring_buffer;   // Lock-free SPSC ring buffer (like Linux kfifo)
pub mod linked_list;   // Intrusive doubly/singly linked list (like Linux list.h / BSD TAILQ)
pub mod slab;          // Slab allocator (like Linux SLUB / FreeBSD UMA)
pub mod btreemap;
pub mod vecdeque;
pub mod string;
pub mod hash;
pub mod time;
pub mod math;
pub mod uuid;
pub mod conversion;
pub mod ffi;
pub mod collections;

// Re-export FFI functions for reduced std dependency
pub use ffi::{cstr_to_rust_string, rust_string_to_cstr, cstrlen, cstrcmp, cstrcpy, cstrcat};

// Re-export custom collections for reduced std dependency
pub use collections::{SimpleHashSet, SimpleBinaryHeap, SimpleOrderedSet, SimpleDeque};

// Re-export existing modules if present
#[cfg(feature = "default")]
pub use async_runtime::{AsyncExecutor, Task};
pub use error::{CryptoError, FsError, KernelError, NetError, SecurityError, SigmaError};
pub use isa::{CpuIsaAssessor, IsaLevel};
pub use store::{Reducer, Store, Subscriber};
// Re-export common types
pub use vec::Vec;
pub use hashmap::HashMap;
pub use hashset::HashSet;
pub use arc::Arc;
// New exports
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};

// Re-export String and related types from custom_string if present
#[cfg(all())]
pub use crate::klib::string::String as KString;
pub use custom_string::SigmaString;
pub use hashmap::{HashMap, Entry};
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
pub use alloc::string::String;

// Re-export custom time types
pub use time::{Duration, Instant, Time, Date, Timestamp};

/// Format integer to string without std::fmt
pub fn format_int(mut num: u64) -> String {
    if num == 0 {
        return String::from("0");
    }
    let mut buffer = alloc::vec::Vec::new();
    while num > 0 {
        let digit = (num % 10) as u8;
        buffer.push(b'0' + digit);
        num /= 10;
    }
    buffer.reverse();
    String::from_utf8(buffer).unwrap_or_else(|_| String::from("ERR"))
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
