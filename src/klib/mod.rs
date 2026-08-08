// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod paging;
pub mod uvm;
pub mod vec;

pub use vec::Vec;

#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
