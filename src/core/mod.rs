//! SigmaOS Core Module
//! Zero-dependency core utilities and functions
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;


pub mod string;
pub mod math;

pub use string::{
    StringBuilder, StringError, string_compare, string_concat, string_copy, string_ends_with,
    string_find, string_len, string_starts_with, string_to_lowercase, string_to_uppercase,
    string_trim,
};
pub use math::{
    abs_f64, abs_i32, abs_i64, ceil_f64, clamp_i32, cos_f64, deg_to_rad, floor_f64, gcd,
    is_prime, lcm, lerp, log10_f64, log2_f64, max_i32, max_usize, min_i32, min_usize, pow_i32,
    rad_to_deg, round_f64, sin_f64, sqrt_f64,
};
