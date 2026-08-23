// SigmaOS Kernel Library
pub mod adt;
pub mod arc;
pub mod async_runtime;
pub mod bitmap;
pub mod btreemap;
pub mod buddy_allocator;
pub mod collections;
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
pub mod vecdeque;

pub use vec::Vec;
pub use hashmap::{HashMap, Entry};
pub use hashset::HashSet;
pub use btreemap::BTreeMap;
pub use vecdeque::VecDeque;
pub use custom_string::SigmaString;
pub use custom_string::SigmaString as String;
pub use custom_string::SigmaString as ToString;
pub use custom_string::SigmaString as PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid([u8; 16]);

impl Uuid {
    pub fn new_v4() -> Self {
        Uuid([0x12; 16])
    }
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "00000000-0000-0000-0000-000000000000")
    }
}
