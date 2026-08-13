pub mod vec;
pub mod paging;
pub mod buddy_allocator;

// For now, we use our custom Vec
pub use vec::Vec;

// For other collections, use std when available
#[cfg(not(target_os = "none"))]
pub use std::collections::BTreeMap;

#[cfg(not(target_os = "none"))]
pub use std::string::String;
