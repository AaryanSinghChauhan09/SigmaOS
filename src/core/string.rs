//! SigmaOS Custom String Operations
//! Zero-dependency string manipulation functions
//! Reduces dependency on standard library string functions
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


// (no_std only applicable at crate root - removed)

/// Custom string builder for zero-dependency string operations
pub struct StringBuilder {
    buffer: [u8; 256],
    length: usize,
}

impl StringBuilder {
    pub const fn new() -> Self {
        StringBuilder {
            buffer: [0u8; 256],
            length: 0,
        }
    }

    /// Append a single character
    pub fn append_char(&mut self, c: u8) -> Result<(), StringError> {
        if self.length >= 256 {
            return Err(StringError::BufferOverflow);
        }
        self.buffer[self.length] = c;
        self.length += 1;
        Ok(())
    }

    /// Append a string slice
    pub fn append_str(&mut self, s: &[u8]) -> Result<(), StringError> {
        for &byte in s {
            self.append_char(byte)?;
        }
        Ok(())
    }

    /// Append a number as decimal string
    pub fn append_u32(&mut self, num: u32) -> Result<(), StringError> {
        if num == 0 {
            return self.append_char(b'0');
        }

        let mut temp = num;
        let mut digits = [0u8; 10];
        let mut digit_count = 0;

        while temp > 0 {
            digits[digit_count] = (temp % 10) as u8 + b'0';
            temp /= 10;
            digit_count += 1;
        }

        for i in (0..digit_count).rev() {
            self.append_char(digits[i])?;
        }

        Ok(())
    }

    /// Append a number as hexadecimal string
    pub fn append_hex(&mut self, num: u32) -> Result<(), StringError> {
        if num == 0 {
            return self.append_char(b'0');
        }

        let mut temp = num;
        let mut digits = [0u8; 8];
        let mut digit_count = 0;

        while temp > 0 {
            let digit = (temp & 0xF) as u8;
            digits[digit_count] = if digit < 10 {
                digit + b'0'
            } else {
                digit - 10 + b'A'
            };
            temp >>= 4;
            digit_count += 1;
        }

        for i in (0..digit_count).rev() {
            self.append_char(digits[i])?;
        }

        Ok(())
    }

    /// Get string slice
    pub fn as_str(&self) -> &[u8] {
        &self.buffer[..self.length]
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.length = 0;
    }
}

/// Custom string comparison
pub fn string_compare(a: &[u8], b: &[u8]) -> i32 {
    let min_len = if a.len() < b.len() { a.len() } else { b.len() };

    for i in 0..min_len {
        if a[i] != b[i] {
            return (a[i] as i32) - (b[i] as i32);
        }
    }

    (a.len() as i32) - (b.len() as i32)
}

/// Custom string length calculation
pub fn string_len(s: &[u8]) -> usize {
    s.len()
}

/// Custom string copy
pub fn string_copy(src: &[u8], dst: &mut [u8]) -> Result<usize, StringError> {
    if dst.len() < src.len() {
        return Err(StringError::BufferOverflow);
    }

    dst[..src.len()].copy_from_slice(src);
    Ok(src.len())
}

/// Custom string concatenation
pub fn string_concat(a: &[u8], b: &[u8], dst: &mut [u8]) -> Result<usize, StringError> {
    if dst.len() < a.len() + b.len() {
        return Err(StringError::BufferOverflow);
    }

    dst[..a.len()].copy_from_slice(a);
    dst[a.len()..a.len() + b.len()].copy_from_slice(b);
    Ok(a.len() + b.len())
}

/// Find substring in string
pub fn string_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    if haystack.len() < needle.len() {
        return None;
    }

    for i in 0..=(haystack.len() - needle.len()) {
        let mut found = true;
        for j in 0..needle.len() {
            if haystack[i + j] != needle[j] {
                found = false;
                break;
            }
        }
        if found {
            return Some(i);
        }
    }

    None
}

/// Check if string starts with prefix
pub fn string_starts_with(s: &[u8], prefix: &[u8]) -> bool {
    if s.len() < prefix.len() {
        return false;
    }

    for i in 0..prefix.len() {
        if s[i] != prefix[i] {
            return false;
        }
    }

    true
}

/// Check if string ends with suffix
pub fn string_ends_with(s: &[u8], suffix: &[u8]) -> bool {
    if s.len() < suffix.len() {
        return false;
    }

    let start = s.len() - suffix.len();
    for i in 0..suffix.len() {
        if s[start + i] != suffix[i] {
            return false;
        }
    }

    true
}

/// Trim whitespace from string
pub fn string_trim(s: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = s.len();

    while start < end && (s[start] == b' ' || s[start] == b'\t' || s[start] == b'\n' || s[start] == b'\r') {
        start += 1;
    }

    while end > start && (s[end - 1] == b' ' || s[end - 1] == b'\t' || s[end - 1] == b'\n' || s[end - 1] == b'\r') {
        end -= 1;
    }

    &s[start..end]
}

/// Convert string to uppercase
pub fn string_to_uppercase(s: &[u8], dst: &mut [u8]) -> Result<usize, StringError> {
    if dst.len() < s.len() {
        return Err(StringError::BufferOverflow);
    }

    for (i, &byte) in s.iter().enumerate() {
        dst[i] = if byte >= b'a' && byte <= b'z' {
            byte - 32
        } else {
            byte
        };
    }

    Ok(s.len())
}

/// Convert string to lowercase
pub fn string_to_lowercase(s: &[u8], dst: &mut [u8]) -> Result<usize, StringError> {
    if dst.len() < s.len() {
        return Err(StringError::BufferOverflow);
    }

    for (i, &byte) in s.iter().enumerate() {
        dst[i] = if byte >= b'A' && byte <= b'Z' {
            byte + 32
        } else {
            byte
        };
    }

    Ok(s.len())
}

#[derive(Debug, PartialEq, Eq)]
pub enum StringError {
    BufferOverflow,
    InvalidUtf8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_builder() {
        let mut builder = StringBuilder::new();
        builder.append_char(b'H').unwrap();
        builder.append_char(b'i').unwrap();
        assert_eq!(builder.as_str(), b"Hi");
    }

    #[test]
    fn test_append_u32() {
        let mut builder = StringBuilder::new();
        builder.append_u32(12345).unwrap();
        assert_eq!(builder.as_str(), b"12345");
    }

    #[test]
    fn test_append_hex() {
        let mut builder = StringBuilder::new();
        builder.append_hex(0xABCD).unwrap();
        assert_eq!(builder.as_str(), b"ABCD");
    }

    #[test]
    fn test_string_compare() {
        assert_eq!(string_compare(b"hello", b"hello"), 0);
        assert_eq!(string_compare(b"hello", b"world"), -15);
        assert_eq!(string_compare(b"world", b"hello"), 15);
    }

    #[test]
    fn test_string_find() {
        assert_eq!(string_find(b"hello world", b"world"), Some(6));
        assert_eq!(string_find(b"hello world", b"xyz"), None);
    }

    #[test]
    fn test_string_starts_with() {
        assert!(string_starts_with(b"hello world", b"hello"));
        assert!(!string_starts_with(b"hello world", b"world"));
    }

    #[test]
    fn test_string_ends_with() {
        assert!(string_ends_with(b"hello world", b"world"));
        assert!(!string_ends_with(b"hello world", b"hello"));
    }

    #[test]
    fn test_string_trim() {
        assert_eq!(string_trim(b"  hello  "), b"hello");
        assert_eq!(string_trim(b"\thello\n"), b"hello");
    }
}
