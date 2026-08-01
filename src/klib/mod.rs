// SigmaOS Kernel Library
pub mod vec;
pub mod buddy_allocator;
pub mod paging;
pub mod error;
pub mod string;
pub mod uuid;
pub mod math;
pub mod hash;

pub use vec::Vec;
pub use string::{strlen, strcmp, strcpy, strcat, strstr, strchr, atoi, itoa, memcpy, memset, memcmp};
pub use uuid::Uuid;
pub use math::{abs, min, max, clamp, pow, sqrt, log2, log10, gcd, lcm, is_prime, round, floor, ceil};
pub use hash::{djb2_hash, simple_hash, xor_hash, fnv1a_hash, combine_hashes, SimpleHasher};
