extern crate alloc;

pub mod buddy_allocator;
pub mod paging;
pub mod uuid;
pub mod hash;
pub mod string;
pub mod arc;
pub mod ring_buffer;
pub mod linked_list;
pub mod slab;
pub mod custom_string;
pub mod custom_allocator;
pub mod vec;
pub mod error;

#[cfg(target_os = "none")]
pub mod hashmap;
#[cfg(target_os = "none")]
pub mod hashset;
#[cfg(target_os = "none")]
pub mod btreemap;
#[cfg(target_os = "none")]
pub mod vecdeque;

pub use string::{String, ToString};
pub use custom_string::SigmaString;
pub use ring_buffer::{RingBuffer, HeapRingBuffer};
pub use linked_list::{LinkedList, SList};
pub use slab::{SlabCache, TypedSlabCache};
pub use custom_string::{SigmaStringBuilder, CStringView};
pub use error::*;

#[cfg(target_os = "none")]
pub use vec::Vec;

#[cfg(target_os = "none")]
pub use hashmap::HashMap;
#[cfg(target_os = "none")]
pub use hashset::HashSet;
#[cfg(target_os = "none")]
pub use arc::Arc;

#[cfg(not(target_os = "none"))]
pub use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};
#[cfg(not(target_os = "none"))]
pub use std::vec::Vec;
#[cfg(not(target_os = "none"))]
pub use std::sync::Arc;

pub use uuid::Uuid;
