pub mod adt;
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
pub mod rng;

pub use arc::Arc;
pub use collections::BTreeMap;
pub use linked_list::{LinkedList, SList};
pub use ring_buffer::{HeapRingBuffer, RingBuffer};
pub use slab::{SlabCache, TypedSlabCache};
pub use uuid::Uuid;
pub use zero_dependency_elimination::ZeroDependencyMasterHub;

/// Zero Dependency Primitive Hub
#[derive(Debug, Clone, Copy)]
pub struct ZeroDependencyPrimitiveHub;

impl ZeroDependencyPrimitiveHub {
    pub fn fnv1a_hash_64(data: &[u8]) -> u64 {
        hash::fnv1a_hash(data)
    }

    pub fn format_u64_stack<'a>(mut val: u64, buf: &'a mut [u8]) -> &'a str {
        if val == 0 {
            if !buf.is_empty() {
                buf[0] = b'0';
                return core::str::from_utf8(&buf[..1]).unwrap_or("0");
            }
            return "0";
        }
        let mut len = 0;
        let mut tmp = val;
        while tmp > 0 {
            len += 1;
            tmp /= 10;
        }
        if buf.len() < len {
            return "0";
        }
        let mut pos = len;
        while val > 0 {
            pos -= 1;
            buf[pos] = b'0' + (val % 10) as u8;
            val /= 10;
        }
        core::str::from_utf8(&buf[..len]).unwrap_or("0")
    }
}

#[cfg(not(target_os = "none"))]
pub use std::vec::Vec;

#[cfg(target_os = "none")]
pub use vec::Vec;

#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
#[cfg(not(target_os = "none"))]
pub use std::collections::HashSet;

#[cfg(target_os = "none")]
pub use collections::HashMap;
#[cfg(target_os = "none")]
pub use collections::HashSet;

pub use custom_string::SigmaString;

pub mod vec {
    pub use super::Vec;
    pub type SigmaVec<T> = super::Vec<T>;
}

/// SigmaOS kernel library prelude.
///
/// Kernel modules can use `use crate::klib::prelude::*;` to get
/// sovereign implementations of common types without stdlib imports.
pub mod prelude {
    // Collections
    pub use super::BTreeMap;
    pub use super::Vec;
    pub use super::HashMap;
    pub use super::HashSet;
    // Linked structures
    pub use super::LinkedList;
    pub use super::RingBuffer;
    pub use super::HeapRingBuffer;
    // Utilities
    pub use super::Arc;
    pub use super::Uuid;
    pub use super::SigmaString;
    pub use super::SlabCache;
    pub use super::ZeroDependencyMasterHub;
    pub use super::ZeroDependencyPrimitiveHub;
}

pub mod path;
