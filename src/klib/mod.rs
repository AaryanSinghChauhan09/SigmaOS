#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Kernel Library
pub mod async_runtime;
pub mod btreemap;
pub mod buddy_allocator;
pub mod conversion;
pub mod error;
pub mod hash;
pub mod hashmap;
pub mod hashset;
pub mod math;
pub mod paging;
pub mod store;
pub mod string;
pub mod time;
pub mod uuid;
pub mod vec;
pub mod vecdeque;

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
};
pub use time::{monotonic_ms, sleep_ms, uptime_ms, Date, Time, Timestamp};
pub use uuid::Uuid;
pub use vec::Vec;
pub use vecdeque::VecDeque;
