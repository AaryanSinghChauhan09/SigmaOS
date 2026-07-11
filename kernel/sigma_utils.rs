// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Program
//
// kernel/sigma_utils.rs — Custom Utility Functions
//
// This module provides custom utility functions to reduce dependencies on external
// libraries. All implementations are pure Rust with no external dependencies.
//
// Key features:
// - String manipulation utilities
// - Math utilities
// - Hash functions
// - Time utilities
// - No external dependencies

#![no_std]
#![allow(dead_code)]

// ─────────────────────────────────────────────────────────────────────────────
// String Utilities
// ─────────────────────────────────────────────────────────────────────────────

/// Compare two byte slices for equality
pub fn memcmp(a: &[u8], b: &[u8]) -> i32 {
    let min_len = a.len().min(b.len());
    for i in 0..min_len {
        if a[i] < b[i] { return -1; }
        if a[i] > b[i] { return 1; }
    }
    if a.len() < b.len() { return -1; }
    if a.len() > b.len() { return 1; }
    0
}

/// Copy bytes from source to destination
pub fn memcpy(dest: &mut [u8], src: &[u8]) -> usize {
    let min_len = dest.len().min(src.len());
    for i in 0..min_len {
        dest[i] = src[i];
    }
    min_len
}

/// Fill a byte slice with a value
pub fn memset(dest: &mut [u8], value: u8) {
    for byte in dest.iter_mut() {
        *byte = value;
    }
}

/// Calculate string length (null-terminated)
pub fn strlen(s: &[u8]) -> usize {
    let mut len = 0;
    while len < s.len() && s[len] != 0 {
        len += 1;
    }
    len
}

/// Compare two null-terminated strings
pub fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    let a_len = strlen(a);
    let b_len = strlen(b);
    let min_len = a_len.min(b_len);
    
    for i in 0..min_len {
        if a[i] < b[i] { return -1; }
        if a[i] > b[i] { return 1; }
    }
    
    if a_len < b_len { return -1; }
    if a_len > b_len { return 1; }
    0
}

/// Copy null-terminated string
pub fn strcpy(dest: &mut [u8], src: &[u8]) -> usize {
    let src_len = strlen(src);
    let copy_len = dest.len().min(src_len);
    for i in 0..copy_len {
        dest[i] = src[i];
    }
    if copy_len < dest.len() {
        dest[copy_len] = 0;
    }
    copy_len
}

/// Concatenate two strings
pub fn strcat(dest: &mut [u8], src: &[u8]) -> usize {
    let dest_len = strlen(dest);
    let src_len = strlen(src);
    let remaining = dest.len().saturating_sub(dest_len);
    let copy_len = remaining.min(src_len);
    
    for i in 0..copy_len {
        dest[dest_len + i] = src[i];
    }
    
    let new_len = dest_len + copy_len;
    if new_len < dest.len() {
        dest[new_len] = 0;
    }
    new_len
}

// ─────────────────────────────────────────────────────────────────────────────
// Math Utilities
// ─────────────────────────────────────────────────────────────────────────────

/// Calculate absolute value
pub fn abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

/// Calculate minimum of two values
pub fn min<T: Ord>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

/// Calculate maximum of two values
pub fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// Clamp value between min and max
pub fn clamp<T: Ord>(value: T, min_val: T, max_val: T) -> T {
    if value < min_val { min_val } 
    else if value > max_val { max_val }
    else { value }
}

/// Calculate greatest common divisor
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Calculate least common multiple
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// Calculate power (exponentiation)
pub fn pow(mut base: u64, exp: u32) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }
    result
}

/// Calculate integer square root
pub fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Calculate logarithm base 2
pub fn log2(n: u64) -> u32 {
    if n == 0 { return 0; }
    63 - n.leading_zeros() as u32
}

/// Count trailing zeros
pub fn ctz(n: u64) -> u32 {
    n.trailing_zeros()
}

/// Count leading zeros
pub fn clz(n: u64) -> u32 {
    n.leading_zeros()
}

/// Count population count (number of set bits)
pub fn popcount(n: u64) -> u32 {
    n.count_ones()
}

/// Round up to next power of 2
pub fn next_power_of_two(n: u64) -> u64 {
    if n == 0 { return 1; }
    1u64 << (64 - n.leading_zeros())
}

/// Check if number is power of 2
pub fn is_power_of_two(n: u64) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

/// Align value to alignment
pub fn align_up(value: u64, alignment: u64) -> u64 {
    (value + alignment - 1) & !(alignment - 1)
}

/// Align value down to alignment
pub fn align_down(value: u64, alignment: u64) -> u64 {
    value & !(alignment - 1)
}

// ─────────────────────────────────────────────────────────────────────────────
// Hash Utilities
// ─────────────────────────────────────────────────────────────────────────────

/// Simple DJB2 hash function
pub fn djb2_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in data {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

/// Simple FNV-1a hash function
pub fn fnv1a_hash(data: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;
    
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Simple XOR hash function
pub fn xor_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;
    for (i, &byte) in data.iter().enumerate() {
        hash ^= (byte as u64) << ((i % 8) * 8);
    }
    hash
}

/// Rotate left operation
pub fn rotl(x: u64, n: u32) -> u64 {
    x.rotate_left(n)
}

/// Rotate right operation
pub fn rotr(x: u64, n: u32) -> u64 {
    x.rotate_right(n)
}

// ─────────────────────────────────────────────────────────────────────────────
// Time Utilities
// ─────────────────────────────────────────────────────────────────────────────

/// Convert nanoseconds to microseconds
pub fn ns_to_us(ns: u64) -> u64 {
    ns / 1000
}

/// Convert nanoseconds to milliseconds
pub fn ns_to_ms(ns: u64) -> u64 {
    ns / 1_000_000
}

/// Convert nanoseconds to seconds
pub fn ns_to_s(ns: u64) -> u64 {
    ns / 1_000_000_000
}

/// Convert microseconds to nanoseconds
pub fn us_to_ns(us: u64) -> u64 {
    us * 1000
}

/// Convert milliseconds to nanoseconds
pub fn ms_to_ns(ms: u64) -> u64 {
    ms * 1_000_000
}

/// Convert seconds to nanoseconds
pub fn s_to_ns(s: u64) -> u64 {
    s * 1_000_000_000
}

/// Get current time in nanoseconds (placeholder - needs real implementation)
pub fn get_time_ns() -> u64 {
    // In real implementation, this would read from a hardware timer
    0
}

/// Get current time in microseconds
pub fn get_time_us() -> u64 {
    ns_to_us(get_time_ns())
}

/// Get current time in milliseconds
pub fn get_time_ms() -> u64 {
    ns_to_ms(get_time_ns())
}

/// Get current time in seconds
pub fn get_time_s() -> u64 {
    ns_to_s(get_time_ns())
}

// ─────────────────────────────────────────────────────────────────────────────
// Bit Manipulation Utilities
// ─────────────────────────────────────────────────────────────────────────────

/// Set bit at position
pub fn set_bit(value: u64, pos: u32) -> u64 {
    value | (1u64 << pos)
}

/// Clear bit at position
pub fn clear_bit(value: u64, pos: u32) -> u64 {
    value & !(1u64 << pos)
}

/// Toggle bit at position
pub fn toggle_bit(value: u64, pos: u32) -> u64 {
    value ^ (1u64 << pos)
}

/// Check if bit is set at position
pub fn test_bit(value: u64, pos: u32) -> bool {
    (value & (1u64 << pos)) != 0
}

/// Extract bits from range [start, end)
pub fn extract_bits(value: u64, start: u32, end: u32) -> u64 {
    let mask = (1u64 << (end - start)) - 1;
    (value >> start) & mask
}

/// Insert bits into range [start, end)
pub fn insert_bits(value: u64, bits: u64, start: u32, end: u32) -> u64 {
    let mask = ((1u64 << (end - start)) - 1) << start;
    (value & !mask) | ((bits << start) & mask)
}

// ─────────────────────────────────────────────────────────────────────────────
// Byte Order Utilities
// ─────────────────────────────────────────────────────────────────────────────

/// Convert u16 from big-endian to native
pub fn be16_to_cpu(x: [u8; 2]) -> u16 {
    u16::from_be_bytes(x)
}

/// Convert u16 from native to big-endian
pub fn cpu_to_be16(x: u16) -> [u8; 2] {
    x.to_be_bytes()
}

/// Convert u32 from big-endian to native
pub fn be32_to_cpu(x: [u8; 4]) -> u32 {
    u32::from_be_bytes(x)
}

/// Convert u32 from native to big-endian
pub fn cpu_to_be32(x: u32) -> [u8; 4] {
    x.to_be_bytes()
}

/// Convert u64 from big-endian to native
pub fn be64_to_cpu(x: [u8; 8]) -> u64 {
    u64::from_be_bytes(x)
}

/// Convert u64 from native to big-endian
pub fn cpu_to_be64(x: u64) -> [u8; 8] {
    x.to_be_bytes()
}

/// Convert u16 from little-endian to native
pub fn le16_to_cpu(x: [u8; 2]) -> u16 {
    u16::from_le_bytes(x)
}

/// Convert u16 from native to little-endian
pub fn cpu_to_le16(x: u16) -> [u8; 2] {
    x.to_le_bytes()
}

/// Convert u32 from little-endian to native
pub fn le32_to_cpu(x: [u8; 4]) -> u32 {
    u32::from_le_bytes(x)
}

/// Convert u32 from native to little-endian
pub fn cpu_to_le32(x: u32) -> [u8; 4] {
    x.to_le_bytes()
}

/// Convert u64 from little-endian to native
pub fn le64_to_cpu(x: [u8; 8]) -> u64 {
    u64::from_le_bytes(x)
}

/// Convert u64 from native to little-endian
pub fn cpu_to_le64(x: u64) -> [u8; 8] {
    x.to_le_bytes()
}

// ─────────────────────────────────────────────────────────────────────────────
// C-compatible exports
// ─────────────────────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn sigma_memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
    if a.is_null() || b.is_null() { return -1; }
    unsafe {
        let a_slice = core::slice::from_raw_parts(a, len);
        let b_slice = core::slice::from_raw_parts(b, len);
        memcmp(a_slice, b_slice)
    }
}

#[no_mangle]
pub extern "C" fn sigma_memcpy(dest: *mut u8, src: *const u8, len: usize) -> usize {
    if dest.is_null() || src.is_null() { return 0; }
    unsafe {
        let dest_slice = core::slice::from_raw_parts_mut(dest, len);
        let src_slice = core::slice::from_raw_parts(src, len);
        memcpy(dest_slice, src_slice)
    }
}

#[no_mangle]
pub extern "C" fn sigma_memset(dest: *mut u8, value: u8, len: usize) {
    if dest.is_null() { return; }
    unsafe {
        let dest_slice = core::slice::from_raw_parts_mut(dest, len);
        memset(dest_slice, value);
    }
}

#[no_mangle]
pub extern "C" fn sigma_strlen(s: *const u8) -> usize {
    if s.is_null() { return 0; }
    unsafe {
        let len = 0;
        while *s.add(len) != 0 {
            len += 1;
        }
        len
    }
}

#[no_mangle]
pub extern "C" fn sigma_strcmp(a: *const u8, b: *const u8) -> i32 {
    if a.is_null() || b.is_null() { return -1; }
    unsafe {
        let mut i = 0;
        loop {
            let ca = *a.add(i);
            let cb = *b.add(i);
            if ca == 0 && cb == 0 { return 0; }
            if ca == 0 { return -1; }
            if cb == 0 { return 1; }
            if ca < cb { return -1; }
            if ca > cb { return 1; }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn sigma_strcpy(dest: *mut u8, src: *const u8) -> usize {
    if dest.is_null() || src.is_null() { return 0; }
    unsafe {
        let mut i = 0;
        loop {
            let byte = *src.add(i);
            *dest.add(i) = byte;
            if byte == 0 { break; }
            i += 1;
        }
        i
    }
}

#[no_mangle]
pub extern "C" fn sigma_abs(x: i32) -> i32 {
    abs(x)
}

#[no_mangle]
pub extern "C" fn sigma_min_u64(a: u64, b: u64) -> u64 {
    min(a, b)
}

#[no_mangle]
pub extern "C" fn sigma_max_u64(a: u64, b: u64) -> u64 {
    max(a, b)
}

#[no_mangle]
pub extern "C" fn sigma_clamp_u64(value: u64, min_val: u64, max_val: u64) -> u64 {
    clamp(value, min_val, max_val)
}

#[no_mangle]
pub extern "C" fn sigma_pow(base: u64, exp: u32) -> u64 {
    pow(base, exp)
}

#[no_mangle]
pub extern "C" fn sigma_isqrt(n: u64) -> u64 {
    isqrt(n)
}

#[no_mangle]
pub extern "C" fn sigma_log2(n: u64) -> u32 {
    log2(n)
}

#[no_mangle]
pub extern "C" fn sigma_popcount(n: u64) -> u32 {
    popcount(n)
}

#[no_mangle]
pub extern "C" fn sigma_next_power_of_two(n: u64) -> u64 {
    next_power_of_two(n)
}

#[no_mangle]
pub extern "C" fn sigma_is_power_of_two(n: u64) -> bool {
    is_power_of_two(n)
}

#[no_mangle]
pub extern "C" fn sigma_align_up(value: u64, alignment: u64) -> u64 {
    align_up(value, alignment)
}

#[no_mangle]
pub extern "C" fn sigma_align_down(value: u64, alignment: u64) -> u64 {
    align_down(value, alignment)
}
