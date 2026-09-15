// SigmaOS Kernel Library

// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod arc;
pub mod async_runtime;
pub mod base64;
pub mod bitmap;
pub mod btreemap;
pub mod collections;
pub mod config_parser;
#[macro_use]
pub mod console;
pub mod buddy_allocator;
pub mod conversion;
pub mod custom_allocator;
pub mod custom_string;
pub mod env;
pub mod ffi;
pub mod fs;
pub mod hash;
pub mod hashmap;
pub mod hashset;
pub mod io;
pub mod isa;
pub mod json;
pub mod linked_list;
pub mod math;
pub mod math_ops;
pub mod merkle;
pub mod net;
pub mod path;
pub mod process;
pub mod rand;
pub mod random;
pub mod ring_buffer;
pub mod ringbuf;
pub mod rng;
pub mod sigma_string_utils;
pub mod sigmalib;
pub mod slab;
pub mod static_hashmap;
pub mod store;
pub mod string_ops;
pub mod string_parser;
pub mod time;
pub mod time_impl;
pub mod toml;
pub mod utf8_utils;
pub mod uuid;
pub mod uvm;

pub mod string {
    pub use crate::klib::custom_string::SigmaString;
}

pub mod vec {
    pub use std::vec::Vec;
}

// Re-exports
pub use arc::Arc;
pub use custom_string::SigmaString;
pub use linked_list::{LinkedList, SList};
pub use ring_buffer::{HeapRingBuffer, RingBuffer};
pub use slab::{SlabCache, TypedSlabCache};
pub use uuid::Uuid;

#[cfg(not(target_os = "none"))]
pub use std::vec::Vec;

#[cfg(target_os = "none")]
pub use vec::Vec;

#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
#[cfg(not(target_os = "none"))]
pub use std::collections::HashSet;

#[cfg(target_os = "none")]
pub use hashmap::HashMap;
#[cfg(target_os = "none")]
pub use hashset::HashSet;

pub struct ZeroDependencyPrimitiveHub;

impl ZeroDependencyPrimitiveHub {
    pub fn fnv1a_hash_64(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    pub fn format_u64_stack(val: u64, buf: &mut [u8]) -> usize {
        if val == 0 {
            if !buf.is_empty() {
                buf[0] = b"0";
                return 1;
            }
            return 0;
        }
        let mut temp = [0u8; 20];
        let mut i = 0;
        let mut n = val;
        while n > 0 {
            temp[i] = b"0" + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        let len = i;
        if buf.len() < len {
            return 0;
        }
        for j in 0..len {
            buf[j] = temp[len - 1 - j];
        }
        len
    }
}
pub use ZeroDependencyPrimitiveHub as PrimitiveHub;
