/// Custom string operations without std

pub fn custom_strlen(s: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

pub fn custom_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    unsafe {
        loop {
            let c1 = *s1.add(i);
            let c2 = *s2.add(i);
            if c1 != c2 {
                return (c1 as i32) - (c2 as i32);
            }
            if c1 == 0 {
                return 0;
            }
            i += 1;
        }
    }
}

pub fn custom_strncpy_secure(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;
    unsafe {
        loop {
            let c = *src.add(i);
            *dest.add(i) = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
    }
    dest
}

/// ⚡ Bolt: Optimized bulk memory copy using `core::ptr::copy_nonoverlapping`
/// Avoids element-by-element byte loops and leverages target SIMD / fast memcpy CPU instructions.
pub fn custom_memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        core::ptr::copy_nonoverlapping(src, dest, n);
    }
    dest
}

/// ⚡ Bolt: Optimized memset using `core::ptr::write_bytes`
/// Replaces manual byte loops with bulk SIMD / memset intrinsics.
pub fn custom_memset(dest: *mut u8, c: u8, n: usize) -> *mut u8 {
    unsafe {
        core::ptr::write_bytes(dest, c, n);
    }
    dest
}

/// ⚡ Bolt: Fast pattern matching using slice window equality (`s.windows()`)
/// Bypasses manual double-indexed loops and leverages vectorized slice comparison.
pub fn pattern_match(s: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    if s.len() < pattern.len() {
        return None;
    }
    s.windows(pattern.len()).position(|w| w == pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_memcpy() {
        let src = b"Hello, SigmaOS!";
        let mut dest = [0u8; 15];
        custom_memcpy(dest.as_mut_ptr(), src.as_ptr(), src.len());
        assert_eq!(&dest[..src.len()], b"Hello, SigmaOS!");
    }

    #[test]
    fn test_custom_memset() {
        let mut buf = [0u8; 10];
        custom_memset(buf.as_mut_ptr(), b'A', 10);
        assert_eq!(&buf, b"AAAAAAAAAA");
    }

    #[test]
    fn test_pattern_match() {
        let haystack = b"The quick brown fox jumps over the lazy dog";
        assert_eq!(pattern_match(haystack, b"quick"), Some(4));
        assert_eq!(pattern_match(haystack, b"fox"), Some(16));
        assert_eq!(pattern_match(haystack, b"cat"), None);
        assert_eq!(pattern_match(haystack, b""), Some(0));
    }

    #[test]
    fn test_custom_strlen() {
        let s = b"SigmaOS\0";
        assert_eq!(custom_strlen(s.as_ptr()), 7);
    }

    #[test]
    fn test_custom_strcmp() {
        let s1 = b"abc\0";
        let s2 = b"abc\0";
        let s3 = b"abd\0";
        assert_eq!(custom_strcmp(s1.as_ptr(), s2.as_ptr()), 0);
        assert!(custom_strcmp(s1.as_ptr(), s3.as_ptr()) < 0);
    }

    #[test]
    fn test_custom_strncpy_secure() {
        let src = b"Hello\0";
        let mut dest = [0u8; 10];
        custom_strncpy_secure(dest.as_mut_ptr(), src.as_ptr());
        assert_eq!(&dest[..6], b"Hello\0");
    }
}
