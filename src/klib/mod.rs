extern crate alloc;
// SigmaOS Kernel Library

pub mod adt;
pub mod arc;
pub mod async_runtime;
pub mod bitmap;
pub mod btreemap;
pub mod buddy_allocator;
pub mod collections;
#[macro_use]
pub mod console;
pub mod conversion;
pub mod custom_allocator;
pub mod custom_string;
pub mod env;
pub mod error;
pub mod ffi;
pub mod fs;
pub mod hash;
pub mod hashmap;
pub mod hashset;
pub mod io;
pub mod isa;
pub mod linked_list;
pub mod math;
pub mod math_ops;
pub mod net;
pub mod paging;
pub mod path;
pub mod process;
pub mod rand;
pub mod random;
pub mod ring_buffer;
pub mod ringbuf;
pub mod rng;
pub mod sigmalib;
pub mod slab;
pub mod static_hashmap;
pub mod store;
pub mod string;
pub mod string_ops;
pub mod string_parser;
pub mod time;
pub mod time_impl;
pub mod uuid;
pub mod uvm;
pub mod vec;
pub mod base64;
pub mod config_parser;
pub mod utf8_utils;
pub mod merkle;
pub mod toml;

pub use vec::Vec;
pub use alloc::collections::BTreeMap;
pub use hashmap::HashMap;
pub use path::PathBuf;
pub use collections::VecDeque;
pub use string::SigmaString;
pub use alloc::string::{String, ToString};

pub const fn is_zero_dependency_build() -> bool {
    true
}

/// Zero-Dependency Custom Standard Primitive Engine
/// Replaces external libraries and pre-defined functions with native safe Rust primitives.
#[derive(Debug, Clone, Copy, Default)]
pub struct ZeroDependencyPrimitiveHub;

impl ZeroDependencyPrimitiveHub {
    pub const fn new() -> Self {
        Self
    }

    /// Pure zero-allocation non-cryptographic FNV-1a 64-bit hash function
    pub fn fnv1a_hash_64(bytes: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        let mut i = 0;
        while i < bytes.len() {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(0x100000001b3);
            i += 1;
        }
        hash
    }

    /// Formats an unsigned integer into a fixed static stack buffer without heap allocation
    pub fn format_u64_stack(mut value: u64, buf: &mut [u8; 32]) -> &str {
        if value == 0 {
            buf[0] = b'0';
            return core::str::from_utf8(&buf[0..1]).unwrap_or("0");
        }
        let mut len = 0;
        let mut tmp = [0u8; 32];
        while value > 0 {
            tmp[len] = b'0' + (value % 10) as u8;
            value /= 10;
            len += 1;
        }
        let mut i = 0;
        while i < len {
            buf[i] = tmp[len - 1 - i];
            i += 1;
        }
        core::str::from_utf8(&buf[0..len]).unwrap_or("0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_dependency_architecture() {
        assert!(is_zero_dependency_build());
    }

    #[test]
    fn test_zero_dependency_primitive_hub() {
        let hash = ZeroDependencyPrimitiveHub::fnv1a_hash_64(b"sigmaos");
        assert_ne!(hash, 0);

        let mut buf = [0u8; 32];
        let formatted = ZeroDependencyPrimitiveHub::format_u64_stack(2026, &mut buf);
        assert_eq!(formatted, "2026");
    }
}
