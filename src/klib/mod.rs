extern crate alloc;

// Core Library Collection Modules for SigmaOS
// Sovereign implementations - no external crate dependencies
pub mod vec; // declared first in module tree to prevent circular dependencies
pub mod buddy_allocator;
pub mod paging;
pub mod hashmap;
pub mod hashset;
pub mod uuid;
pub mod hash;
pub mod string;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;
pub mod custom_string;
pub mod custom_allocator;

// Conditional compilation of other collection modules
#[cfg(target_os = "none")]
pub mod btreemap;
#[cfg(target_os = "none")]
pub mod vecdeque;

// Re-exports
pub use vec::Vec;
pub use hashmap::BTreeMap as HashMap;
pub use hashmap::BTreeMap;

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap as StdBTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;

// New zero-dependency klib modules
pub mod ringbuf;
pub mod slab;
pub mod bitmap;

// Re-exports for convenience
pub use ringbuf::RingBuf;
pub use ringbuf::MpscRingBuf;
pub use slab::SlabCache;
pub use slab::SlabRegistry;
pub use bitmap::AtomicBitmap;
pub use bitmap::PageFrameBitmap;
pub use bitmap::PidBitmap;
