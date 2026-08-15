pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod hashmap;
pub mod hash;

// For now, we use our custom Vec and HashMap (aliased to our bucket-based BTreeMap)
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
