pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;
pub mod custom_allocator;
pub mod custom_string;
pub mod hashset;
pub mod uuid;
pub mod collections;
pub mod conversion;
pub mod error;
pub mod ffi;
pub mod io;
pub mod isa;
pub mod linked_list;
pub mod math;
pub mod math_ops;
pub mod rand;
pub mod random;
pub mod ring_buffer;
pub mod ringbuf;
pub mod rng;
pub mod sigmalib;
pub mod slab;
pub mod bitmap;
pub mod store;
pub mod string;
pub mod string_ops;
pub mod time;
pub mod time_impl;
pub mod uvm;
pub mod vecdeque;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;
pub use custom_string::{SigmaString, SigmaStringBuilder, Utf8Error};
pub use hashset::HashSet;
pub use custom_allocator::*;
pub use uuid::*;

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;

// Re-exports for convenience
pub use ringbuf::RingBuf;
pub use ringbuf::MpscRingBuf;
pub use slab::SlabCache;
pub use slab::SlabRegistry;
pub use bitmap::AtomicBitmap;
pub use bitmap::PageFrameBitmap;
pub use bitmap::PidBitmap;
