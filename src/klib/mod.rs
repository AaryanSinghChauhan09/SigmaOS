// SigmaOS Kernel Library
pub mod buddy_allocator;
pub mod error;
pub mod paging;
pub mod string;
pub mod uuid;
pub mod vec;

pub use string::{
    atoi, itoa, memcmp, memcpy, memset, strcat, strchr, strcmp, strcpy, strlen, strstr,
};
pub use uuid::Uuid;
pub use vec::Vec;
