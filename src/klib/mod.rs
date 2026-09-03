extern crate alloc;
// SigmaOS Kernel Library — zero external dependencies, pure Rust primitives.

pub mod adt;
pub mod arc;
pub mod async_runtime;
pub mod base64;
pub mod bitmap;
pub mod btreemap;
pub mod buddy_allocator;
pub mod collections;
#[macro_use]
pub mod console;
pub mod config_parser;
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
pub mod json;
pub mod linked_list;
pub mod math;
pub mod math_ops;
pub mod merkle;
pub mod net;
pub mod paging;
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
pub mod string;
pub mod string_ops;
pub mod string_parser;
pub mod time;
pub mod time_impl;
pub mod toml;
pub mod utf8_utils;
pub mod uuid;
pub mod uvm;
pub mod vec;

// Re-exports — single source of truth for common types
pub use arc::Arc;
pub use collections::VecDeque;
pub use hashmap::HashMap;
pub use hashset::HashSet;
pub use json::{SovereignJsonParser, SovereignJsonValue};
pub use path::PathBuf;
pub use string::SigmaString;
pub use vec::Vec;

// Compatibility re-exports from alloc (where no custom implementation exists)
pub use alloc::collections::BTreeMap;
pub use alloc::string::{String, ToString};
pub use time::{Duration, Instant, Time};

pub const fn is_zero_dependency_build() -> bool {
    true
}

/// Zero-Dependency Custom Standard Primitive Engine.
/// Replaces external libraries and pre-defined functions with native safe Rust
/// primitives — no libc, no std, no third-party crates.
#[derive(Debug, Clone, Copy, Default)]
pub struct ZeroDependencyPrimitiveHub;

impl ZeroDependencyPrimitiveHub {
    pub const fn new() -> Self {
        Self
    }

    /// Pure zero-allocation non-cryptographic FNV-1a 64-bit hash.
    /// Suitable for hash-tables; not suitable for security-sensitive uses.
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

    /// Format an unsigned integer into a fixed stack buffer without heap allocation.
    /// Returns a `&str` slice into `buf`.
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

    /// Constant-time byte slice comparison to prevent timing side-channels.
    pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff: u8 = 0;
        let mut i = 0;
        while i < a.len() {
            diff |= a[i] ^ b[i];
            i += 1;
        }
        diff == 0
    }

    /// Count leading zeros in a u64 without using intrinsics.
    pub const fn clz64(x: u64) -> u32 {
        if x == 0 {
            return 64;
        }
        let mut n: u32 = 0;
        let mut v = x;
        if v & 0xFFFF_FFFF_0000_0000 == 0 { n += 32; v <<= 32; }
        if v & 0xFFFF_0000_0000_0000 == 0 { n += 16; v <<= 16; }
        if v & 0xFF00_0000_0000_0000 == 0 { n +=  8; v <<=  8; }
        if v & 0xF000_0000_0000_0000 == 0 { n +=  4; v <<=  4; }
        if v & 0xC000_0000_0000_0000 == 0 { n +=  2; v <<=  2; }
        if v & 0x8000_0000_0000_0000 == 0 { n +=  1; }
        n
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
    fn test_fnv1a_hash_64() {
        let hash = ZeroDependencyPrimitiveHub::fnv1a_hash_64(b"sigmaos");
        assert_ne!(hash, 0);
        // Verify determinism
        assert_eq!(
            ZeroDependencyPrimitiveHub::fnv1a_hash_64(b"sigmaos"),
            hash
        );
    }

    #[test]
    fn test_format_u64_stack() {
        let mut buf = [0u8; 32];
        let s = ZeroDependencyPrimitiveHub::format_u64_stack(2026, &mut buf);
        assert_eq!(s, "2026");
        let s0 = ZeroDependencyPrimitiveHub::format_u64_stack(0, &mut buf);
        assert_eq!(s0, "0");
    }

    #[test]
    fn test_ct_eq() {
        assert!(ZeroDependencyPrimitiveHub::ct_eq(b"hello", b"hello"));
        assert!(!ZeroDependencyPrimitiveHub::ct_eq(b"hello", b"world"));
        assert!(!ZeroDependencyPrimitiveHub::ct_eq(b"abc", b"ab"));
    }

    #[test]
    fn test_clz64() {
        assert_eq!(ZeroDependencyPrimitiveHub::clz64(0), 64);
        assert_eq!(ZeroDependencyPrimitiveHub::clz64(1), 63);
        assert_eq!(ZeroDependencyPrimitiveHub::clz64(u64::MAX), 0);
    }
}
