#[cfg(feature = "standalone_test")]
#[path = "../fs/cache.rs"]
mod cache;

#[cfg(not(feature = "standalone_test"))]
use crate::fs::cache;

pub use cache::*;
