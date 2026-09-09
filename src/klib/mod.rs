// SigmaOS Kernel Library

// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod arc;
pub mod async_runtime;
pub mod base64;
pub mod bitmap;
pub mod btreemap;
pub mod buddy_allocator;
pub mod buddy_allocator;
pub mod collections;
pub mod config_parser;
pub mod error;
pub mod hash;
#[cfg(target_os = "none")]
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
pub mod paging;
pub mod string;
pub mod uuid;
pub mod vec;
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

// Re-exports
pub use arc::Arc;
pub use custom_string::{CStringView, SigmaString, SigmaStringBuilder};
pub use linked_list::{LinkedList, SList};
pub use ring_buffer::{HeapRingBuffer, RingBuffer};
pub use slab::{SlabCache, TypedSlabCache};
pub use string::{String, ToString};

#[cfg(not(target_os = "none"))]
pub use std::vec::Vec;

pub use uuid::Uuid;
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
