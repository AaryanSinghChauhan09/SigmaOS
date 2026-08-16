//! Custom String implementation for SigmaOS
//! Based on alloc::string::String for compatibility

extern crate alloc;
use alloc::string::String as AllocString;
use alloc::string::ToString as AllocToString;

pub use alloc::string::String;
pub use alloc::string::ToString;
