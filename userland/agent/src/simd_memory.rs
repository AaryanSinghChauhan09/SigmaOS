// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// SIMD-optimized memory operations for SigmaOS
// Zero-allocation, performance-critical memory operations

use core::arch::x86_64::*;

/// SIMD-optimized memory set (memset)
#[inline(always)]
#[target_feature(enable = "sse2")]
pub unsafe fn simd_memset(dst: *mut u8, value: u8, len: usize) {
    let vec_value = _mm_set1_epi8(value as i8);
    let chunks = len / 16;
    let remainder = len % 16;

    // Set 16 bytes at a time
    for i in 0..chunks {
        _mm_storeu_si128(dst.add(i * 16) as *mut __m128i, vec_value);
    }

    // Set remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            *dst.add(offset + i) = value;
        }
    }
}

/// SIMD-optimized memory compare (memcmp)
#[inline(always)]
#[target_feature(enable = "sse4.2")]
pub unsafe fn simd_memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
    if len == 0 {
        return 0;
    }

    let chunks = len / 16;
    let remainder = len % 16;

    // Compare 16 bytes at a time
    for i in 0..chunks {
        let a_vec = _mm_loadu_si128(a.add(i * 16) as *const __m128i);
        let b_vec = _mm_loadu_si128(b.add(i * 16) as *const __m128i);

        let result = _mm_cmpestri(
            a_vec,
            16,
            b_vec,
            16,
            _SIDD_UBYTE_OPS | _SIDD_CMP_EQUAL_EACH | _SIDD_NEGATIVE_POLARITY
        );

        if result != 16 {
            // Find the differing byte
            for j in 0..16 {
                let a_byte = *a.add(i * 16 + j);
                let b_byte = *b.add(i * 16 + j);
                if a_byte != b_byte {
                    return (a_byte as i32) - (b_byte as i32);
                }
            }
        }
    }

    // Compare remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            let a_byte = *a.add(offset + i);
            let b_byte = *b.add(offset + i);
            if a_byte != b_byte {
                return (a_byte as i32) - (b_byte as i32);
            }
        }
    }

    0
}

/// SIMD-optimized memory move (memmove)
#[inline(always)]
#[target_feature(enable = "sse2")]
pub unsafe fn simd_memmove(dst: *mut u8, src: *const u8, len: usize) {
    // Handle overlapping regions
    if src < dst && (src as usize + len) > dst as usize {
        // Copy backwards
        for i in (0..len).rev() {
            *dst.add(i) = *src.add(i);
        }
    } else {
        // Use SIMD for non-overlapping or forward copy
        simd_memcpy(dst, src, len);
    }
}

/// SIMD-optimized memory copy (memcpy) - already defined in simd_string.rs
#[inline(always)]
#[target_feature(enable = "sse2")]
pub unsafe fn simd_memcpy(dst: *mut u8, src: *const u8, len: usize) {
    let chunks = len / 16;
    let remainder = len % 16;

    // Copy 16 bytes at a time
    for i in 0..chunks {
        let vec = _mm_loadu_si128(src.add(i * 16) as *const __m128i);
        _mm_storeu_si128(dst.add(i * 16) as *mut __m128i, vec);
    }

    // Copy remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            *dst.add(offset + i) = *src.add(offset + i);
        }
    }
}

/// SIMD-optimized zero memory
#[inline(always)]
#[target_feature(enable = "sse2")]
pub unsafe fn simd_memzero(dst: *mut u8, len: usize) {
    let zero_vec = _mm_setzero_si128();
    let chunks = len / 16;
    let remainder = len % 16;

    // Zero 16 bytes at a time
    for i in 0..chunks {
        _mm_storeu_si128(dst.add(i * 16) as *mut __m128i, zero_vec);
    }

    // Zero remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            *dst.add(offset + i) = 0;
        }
    }
}

/// SIMD-optimized memory search (memchr)
#[inline(always)]
#[target_feature(enable = "sse4.2")]
pub unsafe fn simd_memchr(s: *const u8, c: u8, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }

    let target_vec = _mm_set1_epi8(c as i8);
    let chunks = len / 16;
    let remainder = len % 16;

    // Search 16 bytes at a time
    for i in 0..chunks {
        let s_vec = _mm_loadu_si128(s.add(i * 16) as *const __m128i);

        let result = _mm_cmpestri(
            target_vec,
            1,
            s_vec,
            16,
            _SIDD_UBYTE_OPS | _SIDD_CMP_EQUAL_ORDERED | _SIDD_LEAST_SIGNIFICANT
        );

        if result < 16 {
            return Some(i * 16 + result);
        }
    }

    // Search remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            if *s.add(offset + i) == c {
                return Some(offset + i);
            }
        }
    }

    None
}

/// SIMD-optimized memory reverse search (memrchr)
#[inline(always)]
#[target_feature(enable = "sse4.2")]
pub unsafe fn simd_memrchr(s: *const u8, c: u8, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }

    let target_vec = _mm_set1_epi8(c as i8);
    let chunks = len / 16;
    let remainder = len % 16;

    // Search remaining bytes first (reverse)
    if remainder > 0 {
        let offset = chunks * 16;
        for i in (0..remainder).rev() {
            if *s.add(offset + i) == c {
                return Some(offset + i);
            }
        }
    }

    // Search 16-byte chunks in reverse
    for i in (0..chunks).rev() {
        let s_vec = _mm_loadu_si128(s.add(i * 16) as *const __m128i);

        let result = _mm_cmpestri(
            target_vec,
            1,
            s_vec,
            16,
            _SIDD_UBYTE_OPS | _SIDD_CMP_EQUAL_ORDERED | _SIDD_MOST_SIGNIFICANT
        );

        if result < 16 {
            return Some(i * 16 + result);
        }
    }

    None
}

/// SIMD-optimized memory swap
#[inline(always)]
#[target_feature(enable = "sse2")]
pub unsafe fn simd_memswap(a: *mut u8, b: *mut u8, len: usize) {
    let chunks = len / 16;
    let remainder = len % 16;

    // Swap 16 bytes at a time
    for i in 0..chunks {
        let a_vec = _mm_loadu_si128(a.add(i * 16) as *const __m128i);
        let b_vec = _mm_loadu_si128(b.add(i * 16) as *const __m128i);
        _mm_storeu_si128(b.add(i * 16) as *mut __m128i, a_vec);
        _mm_storeu_si128(a.add(i * 16) as *mut __m128i, b_vec);
    }

    // Swap remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            let temp = *a.add(offset + i);
            *a.add(offset + i) = *b.add(offset + i);
            *b.add(offset + i) = temp;
        }
    }
}

/// Fallback implementations for non-SIMD systems
pub mod fallback {
    pub fn memset(dst: *mut u8, value: u8, len: usize) {
        unsafe {
            for i in 0..len {
                *dst.add(i) = value;
            }
        }
    }

    pub fn memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
        unsafe {
            for i in 0..len {
                let a_byte = *a.add(i);
                let b_byte = *b.add(i);
                if a_byte != b_byte {
                    return (a_byte as i32) - (b_byte as i32);
                }
            }
        }
        0
    }

    pub fn memmove(dst: *mut u8, src: *const u8, len: usize) {
        unsafe {
            if src < dst && (src as usize + len) > dst as usize {
                for i in (0..len).rev() {
                    *dst.add(i) = *src.add(i);
                }
            } else {
                for i in 0..len {
                    *dst.add(i) = *src.add(i);
                }
            }
        }
    }

    pub fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
        unsafe {
            for i in 0..len {
                *dst.add(i) = *src.add(i);
            }
        }
    }

    pub fn memzero(dst: *mut u8, len: usize) {
        unsafe {
            for i in 0..len {
                *dst.add(i) = 0;
            }
        }
    }

    pub fn memchr(s: *const u8, c: u8, len: usize) -> Option<usize> {
        unsafe {
            for i in 0..len {
                if *s.add(i) == c {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn memrchr(s: *const u8, c: u8, len: usize) -> Option<usize> {
        unsafe {
            for i in (0..len).rev() {
                if *s.add(i) == c {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn memswap(a: *mut u8, b: *mut u8, len: usize) {
        unsafe {
            for i in 0..len {
                let temp = *a.add(i);
                *a.add(i) = *b.add(i);
                *b.add(i) = temp;
            }
        }
    }
}

/// Runtime dispatch based on CPU feature detection
pub fn memset(dst: *mut u8, value: u8, len: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_memset(dst, value, len) }
        } else {
            fallback::memset(dst, value, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memset(dst, value, len)
    }
}

pub fn memcmp(a: *const u8, b: *const u8, len: usize) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            unsafe { simd_memcmp(a, b, len) }
        } else {
            fallback::memcmp(a, b, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memcmp(a, b, len)
    }
}

pub fn memmove(dst: *mut u8, src: *const u8, len: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_memmove(dst, src, len) }
        } else {
            fallback::memmove(dst, src, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memmove(dst, src, len)
    }
}

pub fn memzero(dst: *mut u8, len: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_memzero(dst, len) }
        } else {
            fallback::memzero(dst, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memzero(dst, len)
    }
}

pub fn memchr(s: *const u8, c: u8, len: usize) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            unsafe { simd_memchr(s, c, len) }
        } else {
            fallback::memchr(s, c, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memchr(s, c, len)
    }
}

pub fn memrchr(s: *const u8, c: u8, len: usize) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            unsafe { simd_memrchr(s, c, len) }
        } else {
            fallback::memrchr(s, c, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memrchr(s, c, len)
    }
}

pub fn memswap(a: *mut u8, b: *mut u8, len: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_memswap(a, b, len) }
        } else {
            fallback::memswap(a, b, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memswap(a, b, len)
    }
}
