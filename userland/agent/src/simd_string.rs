// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// SIMD-optimized string operations for SigmaOS
// Zero-allocation, performance-critical string processing

use core::arch::x86_64::*;

/// SIMD-optimized string comparison using SSE4.2
#[target_feature(enable = "sse4.2")]
pub unsafe fn simd_strcmp(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let len = a.len();
    let chunks = len / 16;
    let remainder = len % 16;
    
    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    
    // Compare 16 bytes at a time using PCMPESTRI
    for i in 0..chunks {
        let a_vec = _mm_loadu_si128(a_ptr.add(i * 16) as *const __m128i);
        let b_vec = _mm_loadu_si128(b_ptr.add(i * 16) as *const __m128i);
        
        // Use PCMPESTRI for string comparison
        let result = _mm_cmpestri(
            a_vec,
            16,
            b_vec,
            16,
            _SIDD_UBYTE_OPS | _SIDD_CMP_EQUAL_EACH | _SIDD_NEGATIVE_POLARITY
        );
        
        if result != 16 {
            return false;
        }
    }
    
    // Compare remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            if *a_ptr.add(offset + i) != *b_ptr.add(offset + i) {
                return false;
            }
        }
    }
    
    true
}

/// SIMD-optimized string length calculation
#[target_feature(enable = "sse4.2")]
pub unsafe fn simd_strlen(s: &[u8]) -> usize {
    let len = s.len();
    let chunks = len / 16;
    let remainder = len % 16;
    
    let ptr = s.as_ptr();
    
    // Search for null terminator in 16-byte chunks
    for i in 0..chunks {
        let vec = _mm_loadu_si128(ptr.add(i * 16) as *const __m128i);
        
        // Check for zero bytes using PCMPESTRI
        let result = _mm_cmpestri(
            vec,
            16,
            _mm_set1_epi8(0),
            16,
            _SIDD_UBYTE_OPS | _SIDD_CMP_EQUAL_EACH | _SIDD_LEAST_SIGNIFICANT
        );
        
        if result < 16 {
            return i * 16 + result as usize;
        }
    }
    
    // Check remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            if *ptr.add(offset + i) == 0 {
                return offset + i;
            }
        }
    }
    
    len
}

/// SIMD-optimized string copy
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

/// SIMD-optimized string search (find substring)
#[target_feature(enable = "sse4.2")]
pub unsafe fn simd_strstr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > haystack.len() {
        return None;
    }
    
    let h_len = haystack.len();
    let n_len = needle.len();
    
    // Naive search for small patterns
    if n_len < 16 {
        for i in 0..=(h_len - n_len) {
            if &haystack[i..i + n_len] == needle {
                return Some(i);
            }
        }
        return None;
    }
    
    // SIMD-accelerated search for larger patterns
    let needle_vec = _mm_loadu_si128(needle.as_ptr() as *const __m128i);
    
    for i in 0..=(h_len - n_len) {
        let haystack_vec = _mm_loadu_si128(haystack.as_ptr().add(i) as *const __m128i);
        
        let result = _mm_cmpestri(
            needle_vec,
            16,
            haystack_vec,
            16,
            _SIDD_UBYTE_OPS | _SIDD_CMP_EQUAL_ORDERED | _SIDD_NEGATIVE_POLARITY
        );
        
        if result == 16 {
            // Full match found, verify remaining bytes
            if n_len > 16 {
                if &haystack[i + 16..i + n_len] == &needle[16..] {
                    return Some(i);
                }
            } else {
                return Some(i);
            }
        }
    }
    
    None
}

/// SIMD-optimized string to lowercase conversion
#[target_feature(enable = "sse2")]
pub unsafe fn simd_to_lowercase(s: &mut [u8]) {
    let len = s.len();
    let chunks = len / 16;
    let remainder = len % 16;
    
    let ptr = s.as_mut_ptr();
    
    // Process 16 bytes at a time
    for i in 0..chunks {
        let vec = _mm_loadu_si128(ptr.add(i * 16) as *const __m128i);
        
        // Convert uppercase to lowercase using bit operations
        // Uppercase ASCII: 0x41-0x5A, Lowercase: 0x61-0x7A
        // Difference: 0x20
        let mask = _mm_set1_epi8(0x20);
        let uppercase_mask = _mm_cmpgt_epi8(vec, _mm_set1_epi8(0x40)); // > 'A'-1
        let lowercase_mask = _mm_cmpgt_epi8(vec, _mm_set1_epi8(0x5A)); // > 'Z'
        let in_range = _mm_andnot_si128(lowercase_mask, uppercase_mask);
        
        let or_mask = _mm_and_si128(in_range, mask);
        let result = _mm_or_si128(vec, or_mask);
        
        _mm_storeu_si128(ptr.add(i * 16) as *mut __m128i, result);
    }
    
    // Process remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            let byte = *ptr.add(offset + i);
            if byte >= 0x41 && byte <= 0x5A {
                *ptr.add(offset + i) = byte + 0x20;
            }
        }
    }
}

/// SIMD-optimized string to uppercase conversion
#[target_feature(enable = "sse2")]
pub unsafe fn simd_to_uppercase(s: &mut [u8]) {
    let len = s.len();
    let chunks = len / 16;
    let remainder = len % 16;
    
    let ptr = s.as_mut_ptr();
    
    // Process 16 bytes at a time
    for i in 0..chunks {
        let vec = _mm_loadu_si128(ptr.add(i * 16) as *const __m128i);
        
        // Convert lowercase to uppercase using bit operations
        let mask = _mm_set1_epi8(0x20);
        let lowercase_mask = _mm_cmpgt_epi8(vec, _mm_set1_epi8(0x60)); // > 'a'-1
        let uppercase_mask = _mm_cmpgt_epi8(vec, _mm_set1_epi8(0x7A)); // > 'z'
        let in_range = _mm_andnot_si128(uppercase_mask, lowercase_mask);
        
        let clear_mask = _mm_and_si128(in_range, mask);
        let result = _mm_andnot_si128(clear_mask, vec);
        
        _mm_storeu_si128(ptr.add(i * 16) as *mut __m128i, result);
    }
    
    // Process remaining bytes
    if remainder > 0 {
        let offset = chunks * 16;
        for i in 0..remainder {
            let byte = *ptr.add(offset + i);
            if byte >= 0x61 && byte <= 0x7A {
                *ptr.add(offset + i) = byte - 0x20;
            }
        }
    }
}

/// Fallback implementations for non-SIMD systems
pub mod fallback {
    pub fn strcmp(a: &[u8], b: &[u8]) -> bool {
        a == b
    }
    
    pub fn strlen(s: &[u8]) -> usize {
        s.iter().position(|&b| b == 0).unwrap_or(s.len())
    }
    
    pub fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
        unsafe {
            for i in 0..len {
                *dst.add(i) = *src.add(i);
            }
        }
    }
    
    pub fn strstr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|window| window == needle)
    }
    
    pub fn to_lowercase(s: &mut [u8]) {
        for byte in s.iter_mut() {
            if *byte >= 0x41 && *byte <= 0x5A {
                *byte += 0x20;
            }
        }
    }
    
    pub fn to_uppercase(s: &mut [u8]) {
        for byte in s.iter_mut() {
            if *byte >= 0x61 && *byte <= 0x7A {
                *byte -= 0x20;
            }
        }
    }
}

/// Runtime dispatch based on CPU feature detection
pub fn strcmp(a: &[u8], b: &[u8]) -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            unsafe { simd_strcmp(a, b) }
        } else {
            fallback::strcmp(a, b)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::strcmp(a, b)
    }
}

pub fn strlen(s: &[u8]) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            unsafe { simd_strlen(s) }
        } else {
            fallback::strlen(s)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::strlen(s)
    }
}

pub fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_memcpy(dst, src, len) }
        } else {
            fallback::memcpy(dst, src, len)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::memcpy(dst, src, len)
    }
}

pub fn strstr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            unsafe { simd_strstr(haystack, needle) }
        } else {
            fallback::strstr(haystack, needle)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::strstr(haystack, needle)
    }
}

pub fn to_lowercase(s: &mut [u8]) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_to_lowercase(s) }
        } else {
            fallback::to_lowercase(s)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::to_lowercase(s)
    }
}

pub fn to_uppercase(s: &mut [u8]) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            unsafe { simd_to_uppercase(s) }
        } else {
            fallback::to_uppercase(s)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        fallback::to_uppercase(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lowercase() {
        let mut s = *b"HELLO, SIGMAOS! 123";
        to_lowercase(&mut s);
        assert_eq!(&s, b"hello, sigmaos! 123");
    }

    #[test]
    fn test_to_uppercase() {
        let mut s = *b"hello, sigmaos! 123";
        to_uppercase(&mut s);
        assert_eq!(&s, b"HELLO, SIGMAOS! 123");
    }

    #[test]
    fn test_strcmp() {
        assert!(strcmp(b"hello", b"hello"));
        assert!(!strcmp(b"hello", b"world"));
    }

    #[test]
    fn test_strlen() {
        assert_eq!(strlen(b"hello\0world"), 5);
        assert_eq!(strlen(b"test"), 4);
    }
}
