// SigmaOS Kernel Library
pub mod arc;
pub mod btreemap;
pub mod buddy_allocator;
pub mod conversion;
pub mod error;
pub mod hash;
pub mod hashmap;
pub mod hashset;
pub mod math;
pub mod paging;
pub mod string;
pub mod time;
pub mod uvm;
pub mod uuid;
pub mod vec;
pub mod vecdeque;

pub use arc::Arc;
pub use btreemap::BTreeMap;
pub use conversion::{
    base64_encode, base_to_dec, binary_to_bytes, bytes_to_binary, bytes_to_hex, dec_to_base,
    hex_to_bytes,
};
pub use hash::{combine_hashes, djb2_hash, fnv1a_hash, simple_hash, xor_hash, SimpleHasher};
pub use hashmap::HashMap;
pub use hashset::HashSet;
pub use math::{
    abs, ceil, clamp, floor, gcd, is_prime, lcm, log10, log2, max, min, pow, round, sqrt,
};
pub use string::{
    atoi, itoa, memcmp, memcpy, memset, strcat, strchr, strcmp, strcpy, strlen, strstr,
    String, ToString,
};
pub use time::{monotonic_ms, sleep_ms, uptime_ms, Date, Time, Timestamp};
pub use uuid::Uuid;
pub use vec::Vec;
pub use vecdeque::VecDeque;
