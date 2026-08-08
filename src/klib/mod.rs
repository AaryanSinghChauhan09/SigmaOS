<<<<<<< HEAD
// SigmaOS Kernel Library
pub mod vec;
pub mod buddy_allocator;
pub mod paging;
pub mod error;

pub use vec::Vec;

#[cfg(not(target_os = "none"))]
pub use std::collections::HashMap;
||||||| 43be3a7e8
=======
pub mod vec;
pub mod paging;
pub mod buddy_allocator;
pub mod uvm;
>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
