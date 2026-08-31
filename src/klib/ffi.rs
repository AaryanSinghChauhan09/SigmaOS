use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Custom FFI Library
// Reduces dependency on std::ffi by providing custom implementations

/// Custom C string to Rust string conversion
pub unsafe fn cstr_to_rust_string(ptr: *const i8) -> Result<String, &'static str> {
    if ptr.is_null() {
        return Err("Null pointer");
    }

    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }

    let slice = core::slice::from_raw_parts(ptr as *const u8, len);
    String::from_utf8(slice.to_vec()).map_err(|_| "Invalid UTF-8")
}

/// Custom Rust string to C string conversion
pub fn rust_string_to_cstr(s: &str) -> Vec<u8> {
    let mut result = s.as_bytes().to_vec();
    result.push(0); // Null terminator
    result
}

/// Custom C string length calculation
pub unsafe fn cstrlen(ptr: *const i8) -> usize {
    if ptr.is_null() {
        return 0;
    }

    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    len
}

/// Custom C string comparison
pub unsafe fn cstrcmp(s1: *const i8, s2: *const i8) -> i32 {
    if s1.is_null() || s2.is_null() {
        return if s1 == s2 { 0 } else { -1 };
    }

    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);

        if c1 == 0 && c2 == 0 {
            return 0;
        }
        if c1 == 0 {
            return -1;
        }
        if c2 == 0 {
            return 1;
        }
        if c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }
        i += 1;
    }
}

/// Custom C string copy
pub unsafe fn cstrncpy_safe_safe_safe_safe_safe(dest: *mut i8, src: *const i8) -> *mut i8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }

    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    dest
}

/// Custom C string concatenation
pub unsafe fn cstrcat(dest: *mut i8, src: *const i8) -> *mut i8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }

    // Find end of dest
    let mut dest_len = 0;
    while *dest.add(dest_len) != 0 {
        dest_len += 1;
    }

    // Append src
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(dest_len + i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
    dest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cstrlen() {
        let s = b"Hello\0";
        unsafe {
            assert_eq!(cstrlen(s.as_ptr() as *const i8), 5);
        }
    }

    #[test]
    fn test_cstrcmp() {
        let s1 = b"Hello\0";
        let s2 = b"Hello\0";
        let s3 = b"World\0";
        unsafe {
            assert_eq!(
                cstrcmp(s1.as_ptr() as *const i8, s2.as_ptr() as *const i8),
                0
            );
            assert!(cstrcmp(s1.as_ptr() as *const i8, s3.as_ptr() as *const i8) != 0);
        }
    }

    #[test]
    fn test_cstr_to_rust_string() {
        let s = b"Hello\0";
        unsafe {
            let result = cstr_to_rust_string(s.as_ptr() as *const i8);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello");
        }
    }

    #[test]
    fn test_rust_string_to_cstr() {
        let s = "Hello";
        let result = rust_string_to_cstr(s);
        assert_eq!(result.len(), 6); // 5 + null terminator
        assert_eq!(result[5], 0);
    }
}
