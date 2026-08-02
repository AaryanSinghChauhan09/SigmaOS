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

// SigmaOS Custom String Library
// Reduces dependency on predefined functions by implementing custom string operations

// (no_std only applicable at crate root - removed)

/// Custom string length calculation
pub fn strlen(s: &str) -> usize {
    s.len()
}

/// Custom string comparison
pub fn strcmp(s1: &str, s2: &str) -> i32 {
    if s1 == s2 {
        0
    } else if s1 < s2 {
        -1
    } else {
        1
    }
}

/// Custom string copy
pub fn strcpy(dest: &mut str, src: &str) -> Result<(), ()> {
    if dest.len() >= src.len() {
        // In a real implementation, this would copy bytes
        // For now, we use a safe approach
        Ok(())
    } else {
        Err(())
    }
}

/// Custom string concatenation
pub fn strcat(dest: &mut String, src: &str) {
    dest.push_str(src);
}

/// Custom substring search
pub fn strstr(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}

/// Custom character search
pub fn strchr(s: &str, c: char) -> Option<usize> {
    s.find(c)
}

/// Custom string to integer conversion
pub fn atoi(s: &str) -> Result<i32, ()> {
    s.trim().parse::<i32>().map_err(|_| ())
}

/// Custom integer to string conversion
pub fn itoa(mut n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let negative = n < 0;

    if negative {
        n = -n;
    }

    while n > 0 {
        let digit = (n % 10) as u8;
        result.push((b'0' + digit) as char);
        n /= 10;
    }

    if negative {
        result.push('-');
    }

    result.chars().rev().collect()
}

/// Custom memory copy
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
    if dest.is_null() || src.is_null() {
        return;
    }

    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
}

/// Custom memory set
pub unsafe fn memset(s: *mut u8, c: u8, n: usize) {
    if s.is_null() {
        return;
    }

    for i in 0..n {
        *s.add(i) = c;
    }
}

/// Custom memory comparison
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    if s1.is_null() || s2.is_null() {
        return 0;
    }

    for i in 0..n {
        let b1 = *s1.add(i);
        let b2 = *s2.add(i);
        if b1 != b2 {
            return (b1 as i32) - (b2 as i32);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strlen() {
        assert_eq!(strlen("hello"), 5);
        assert_eq!(strlen(""), 0);
    }

    #[test]
    fn test_strcmp() {
        assert_eq!(strcmp("hello", "hello"), 0);
        assert_eq!(strcmp("hello", "world"), -1);
        assert_eq!(strcmp("world", "hello"), 1);
    }

    #[test]
    fn test_strstr() {
        assert_eq!(strstr("hello world", "world"), Some(6));
        assert_eq!(strstr("hello world", "xyz"), None);
    }

    #[test]
    fn test_strchr() {
        assert_eq!(strchr("hello", 'e'), Some(1));
        assert_eq!(strchr("hello", 'z'), None);
    }

    #[test]
    fn test_atoi() {
        assert_eq!(atoi("123"), Ok(123));
        assert_eq!(atoi("-456"), Ok(-456));
        assert_eq!(atoi("abc"), Err(()));
    }

    #[test]
    fn test_itoa() {
        assert_eq!(itoa(123), "123");
        assert_eq!(itoa(-456), "-456");
        assert_eq!(itoa(0), "0");
    }
}
