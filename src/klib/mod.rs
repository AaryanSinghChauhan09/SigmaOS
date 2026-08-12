pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod uvm;
pub mod uuid;

extern crate alloc;
pub use alloc::vec::Vec;
pub use alloc::collections::BTreeMap;
pub use alloc::string::String;

#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
