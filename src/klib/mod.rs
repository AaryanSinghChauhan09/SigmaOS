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
pub use std::collections::HashMap;
pub use hashmap::{HashMap, Entry};
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
pub use alloc::string::String;
