// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/sigma_libc.rs — Minimal libc Subset for SigmaOS
//
// Implements only the most widely used libc functions: string, memory, I/O, and math functions.
// Lightweight, modular, and Rust-safe.
// Allows developers to link against this "Sigma libc" for portability.
//
// Language: Rust (no_std for kernel compatibility)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Isize = isize;
type Usize = usize;

// ─── String Functions ───────────────────────────────────

/// Calculate string length
#[no_mangle]
pub extern "C" fn strlen(s: *const U8) -> Usize {
    if s.is_null() {
        return 0;
    }

    let mut len = 0;
    unsafe {
        while *s.add(len) != 0 {
            len += 1;
        }
    }
    len
}

/// Copy string
#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut U8, src: *const U8) -> *mut U8 {
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

/// Copy string with length limit
#[no_mangle]
pub unsafe extern "C" fn strncpy(dest: *mut U8, src: *const U8, n: Usize) -> *mut U8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    let mut i = 0;
    while i < n {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }

    // Pad with zeros if needed
    while i < n {
        *dest.add(i) = 0;
        i += 1;
    }

    dest
}

/// Compare strings
#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const U8, s2: *const U8) -> I32 {
    if s1.is_null() || s2.is_null() {
        if s1.is_null() && s2.is_null() {
            return 0;
        }
        return if s1.is_null() { -1 } else { 1 };
    }

    let mut i = 0;
    loop {
        let c1 = *s1.add(i) as I32;
        let c2 = *s2.add(i) as I32;
        
        if c1 != c2 {
            return c1 - c2;
        }
        
        if c1 == 0 {
            return 0;
        }
        
        i += 1;
    }
}

/// Compare strings with length limit
#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const U8, s2: *const U8, n: Usize) -> I32 {
    if s1.is_null() || s2.is_null() || n == 0 {
        return 0;
    }

    for i in 0..n {
        let c1 = *s1.add(i) as I32;
        let c2 = *s2.add(i) as I32;
        
        if c1 != c2 {
            return c1 - c2;
        }
        
        if c1 == 0 {
            return 0;
        }
    }

    0
}

/// Find character in string
#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const U8, c: I32) -> *const U8 {
    if s.is_null() {
        return 0 as *const U8;
    }

    let target = c as U8;
    let mut i = 0;
    loop {
        let ch = *s.add(i);
        if ch == target {
            return s.add(i);
        }
        if ch == 0 {
            return 0 as *const U8;
        }
        i += 1;
    }
}

/// Find substring in string
#[no_mangle]
pub unsafe extern "C" fn strstr(haystack: *const U8, needle: *const U8) -> *const U8 {
    if haystack.is_null() || needle.is_null() {
        return 0 as *const U8;
    }

    let needle_len = strlen(needle);
    if needle_len == 0 {
        return haystack;
    }

    let haystack_len = strlen(haystack);
    if needle_len > haystack_len {
        return 0 as *const U8;
    }

    for i in 0..=(haystack_len - needle_len) {
        let mut match = true;
        for j in 0..needle_len {
            if *haystack.add(i + j) != *needle.add(j) {
                match = false;
                break;
            }
        }
        if match {
            return haystack.add(i);
        }
    }

    0 as *const U8
}

/// Concatenate strings
#[no_mangle]
pub unsafe extern "C" fn strcat(dest: *mut U8, src: *const U8) -> *mut U8 {
    if dest.is_null() || src.is_null() {
        return dest;
    }

    let dest_len = strlen(dest);
    strcpy(dest.add(dest_len), src);
    dest
}

/// Concatenate strings with length limit
#[no_mangle]
pub unsafe extern "C" fn strncat(dest: *mut U8, src: *const U8, n: Usize) -> *mut U8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    let dest_len = strlen(dest);
    strncpy(dest.add(dest_len), src, n);
    dest
}

/// Duplicate string
#[no_mangle]
pub unsafe extern "C" fn strdup(s: *const U8) -> *mut U8 {
    if s.is_null() {
        return 0 as *mut U8;
    }

    let len = strlen(s);
    let new_str = sigma_malloc(len + 1);
    
    if !new_str.is_null() {
        strcpy(new_str, s);
    }

    new_str
}

// ─── Memory Functions ─────────────────────────────────

/// Allocate memory
#[no_mangle]
pub extern "C" fn malloc(size: Usize) -> *mut U8 {
    sigma_malloc(size)
}

/// Free memory
#[no_mangle]
pub extern "C" fn free(ptr: *mut U8) {
    sigma_free(ptr)
}

/// Reallocate memory
#[no_mangle]
pub extern "C" fn realloc(ptr: *mut U8, size: Usize) -> *mut U8 {
    sigma_realloc(ptr, size)
}

/// Allocate and zero memory
#[no_mangle]
pub extern "C" fn calloc(nmemb: Usize, size: Usize) -> *mut U8 {
    let total = nmemb * size;
    let ptr = sigma_malloc(total);
    
    if !ptr.is_null() {
        unsafe {
            memset(ptr, 0, total);
        }
    }

    ptr
}

/// Copy memory
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut U8, src: *const U8, n: Usize) -> *mut U8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }

    dest
}

/// Move memory (handles overlapping regions)
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut U8, src: *const U8, n: Usize) -> *mut U8 {
    if dest.is_null() || src.is_null() || n == 0 {
        return dest;
    }

    let src_addr = src as Usize;
    let dest_addr = dest as Usize;

    if src_addr > dest_addr && src_addr < dest_addr + n {
        // Copy backwards to handle overlap
        for i in (0..n).rev() {
            *dest.add(i) = *src.add(i);
        }
    } else {
        // Copy forwards
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }

    dest
}

/// Set memory
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut U8, c: I32, n: Usize) -> *mut U8 {
    if s.is_null() || n == 0 {
        return s;
    }

    let value = c as U8;
    for i in 0..n {
        *s.add(i) = value;
    }

    s
}

/// Compare memory
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const U8, s2: *const U8, n: Usize) -> I32 {
    if s1.is_null() || s2.is_null() || n == 0 {
        return 0;
    }

    for i in 0..n {
        let c1 = *s1.add(i) as I32;
        let c2 = *s2.add(i) as I32;
        
        if c1 != c2 {
            return c1 - c2;
        }
    }

    0
}

// ─── SigmaOS Memory Operations (stubs) ───────────────

// These would call into SigmaOS's memory manager
// For now, we provide stub implementations

extern "C" {
    fn sigma_malloc(size: Usize) -> *mut U8;
    fn sigma_free(ptr: *mut U8);
    fn sigma_realloc(ptr: *mut U8, size: Usize) -> *mut U8;
}

// ─── I/O Functions ───────────────────────────────────

/// Print formatted output to stdout
#[no_mangle]
pub extern "C" fn printf(format: *const U8, ...) -> I32 {
    // Stub: In real implementation, this would format and print
    0
}

/// Print formatted output to file
#[no_mangle]
pub extern "C" fn fprintf(stream: *mut U8, format: *const U8, ...) -> I32 {
    // Stub: In real implementation, this would format and print to stream
    0
}

/// Print formatted output to string
#[no_mangle]
pub extern "C" fn sprintf(s: *mut U8, format: *const U8, ...) -> I32 {
    // Stub: In real implementation, this would format to string
    0
}

/// Print formatted output to string with length limit
#[no_mangle]
pub extern "C" fn snprintf(s: *mut U8, size: Usize, format: *const U8, ...) -> I32 {
    // Stub: In real implementation, this would format to string with limit
    0
}

/// Print string to stdout
#[no_mangle]
pub extern "C" fn puts(s: *const U8) -> I32 {
    // Stub: In real implementation, this would print string
    0
}

/// Print character to stdout
#[no_mangle]
pub extern "C" fn putchar(c: I32) -> I32 {
    // Stub: In real implementation, this would print character
    c
}

/// Get character from stdin
#[no_mangle]
pub extern "C" fn getchar() -> I32 {
    // Stub: In real implementation, this would read character
    0
}

// ─── Math Functions ───────────────────────────────────

/// Convert string to integer
#[no_mangle]
pub unsafe extern "C" fn atoi(s: *const U8) -> I32 {
    if s.is_null() {
        return 0;
    }

    let mut result: I32 = 0;
    let mut sign: I32 = 1;
    let mut i = 0;

    // Skip whitespace
    while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' || *s.add(i) == b'\r' {
        i += 1;
    }

    // Handle sign
    if *s.add(i) == b'-' {
        sign = -1;
        i += 1;
    } else if *s.add(i) == b'+' {
        i += 1;
    }

    // Convert digits
    while *s.add(i) >= b'0' && *s.add(i) <= b'9' {
        result = result * 10 + (*s.add(i) - b'0') as I32;
        i += 1;
    }

    sign * result
}

/// Convert string to long
#[no_mangle]
pub unsafe extern "C" fn atol(s: *const U8) -> I64 {
    if s.is_null() {
        return 0;
    }

    let mut result: I64 = 0;
    let mut sign: I64 = 1;
    let mut i = 0;

    // Skip whitespace
    while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' || *s.add(i) == b'\r' {
        i += 1;
    }

    // Handle sign
    if *s.add(i) == b'-' {
        sign = -1;
        i += 1;
    } else if *s.add(i) == b'+' {
        i += 1;
    }

    // Convert digits
    while *s.add(i) >= b'0' && *s.add(i) <= b'9' {
        result = result * 10 + (*s.add(i) - b'0') as I64;
        i += 1;
    }

    sign * result
}

/// Convert string to long with base
#[no_mangle]
pub unsafe extern "C" fn strtol(s: *const U8, endptr: *mut *const U8, base: I32) -> I64 {
    if s.is_null() {
        return 0;
    }

    let mut result: I64 = 0;
    let mut sign: I64 = 1;
    let mut i = 0;
    let actual_base = if base == 0 { 10 } else { base };

    // Skip whitespace
    while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' || *s.add(i) == b'\r' {
        i += 1;
    }

    // Handle sign
    if *s.add(i) == b'-' {
        sign = -1;
        i += 1;
    } else if *s.add(i) == b'+' {
        i += 1;
    }

    // Handle hex prefix
    if actual_base == 16 && *s.add(i) == b'0' && (*s.add(i + 1) == b'x' || *s.add(i + 1) == b'X') {
        i += 2;
    }

    // Convert digits
    while *s.add(i) != 0 {
        let digit = if *s.add(i) >= b'0' && *s.add(i) <= b'9' {
            (*s.add(i) - b'0') as I64
        } else if *s.add(i) >= b'a' && *s.add(i) <= b'f' {
            (*s.add(i) - b'a' + 10) as I64
        } else if *s.add(i) >= b'A' && *s.add(i) <= b'F' {
            (*s.add(i) - b'A' + 10) as I64
        } else {
            break;
        };

        if digit >= actual_base as I64 {
            break;
        }

        result = result * actual_base as I64 + digit;
        i += 1;
    }

    if !endptr.is_null() {
        *endptr = s.add(i);
    }

    sign * result
}

/// Convert string to unsigned long with base
#[no_mangle]
pub unsafe extern "C" fn strtoul(s: *const U8, endptr: *mut *const U8, base: I32) -> U64 {
    if s.is_null() {
        return 0;
    }

    let mut result: U64 = 0;
    let mut i = 0;
    let actual_base = if base == 0 { 10 } else { base };

    // Skip whitespace
    while *s.add(i) == b' ' || *s.add(i) == b'\t' || *s.add(i) == b'\n' || *s.add(i) == b'\r' {
        i += 1;
    }

    // Handle sign (ignore for unsigned)
    if *s.add(i) == b'-' || *s.add(i) == b'+' {
        i += 1;
    }

    // Handle hex prefix
    if actual_base == 16 && *s.add(i) == b'0' && (*s.add(i + 1) == b'x' || *s.add(i + 1) == b'X') {
        i += 2;
    }

    // Convert digits
    while *s.add(i) != 0 {
        let digit = if *s.add(i) >= b'0' && *s.add(i) <= b'9' {
            (*s.add(i) - b'0') as U64
        } else if *s.add(i) >= b'a' && *s.add(i) <= b'f' {
            (*s.add(i) - b'a' + 10) as U64
        } else if *s.add(i) >= b'A' && *s.add(i) <= b'F' {
            (*s.add(i) - b'A' + 10) as U64
        } else {
            break;
        };

        if digit >= actual_base as U64 {
            break;
        }

        result = result * actual_base as U64 + digit;
        i += 1;
    }

    if !endptr.is_null() {
        *endptr = s.add(i);
    }

    result
}

/// Absolute value
#[no_mangle]
pub extern "C" fn abs(x: I32) -> I32 {
    if x < 0 { -x } else { x }
}

/// Absolute value (long)
#[no_mangle]
pub extern "C" fn labs(x: I64) -> I64 {
    if x < 0 { -x } else { x }
}

/// Convert integer to string
#[no_mangle]
pub unsafe extern "C" fn itoa(value: I32, str: *mut U8, radix: I32) -> *mut U8 {
    if str.is_null() {
        return str;
    }

    let mut num = value;
    let mut i = 0;
    let is_negative = num < 0;

    if is_negative {
        num = -num;
    }

    // Handle zero
    if num == 0 {
        *str.add(i) = b'0';
        i += 1;
        *str.add(i) = 0;
        return str;
    }

    // Convert digits
    while num > 0 {
        let digit = (num % radix) as U8;
        *str.add(i) = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + (digit - 10)
        };
        num /= radix;
        i += 1;
    }

    // Add sign
    if is_negative {
        *str.add(i) = b'-';
        i += 1;
    }

    // Add null terminator
    *str.add(i) = 0;

    // Reverse string
    let len = i;
    for j in 0..(len / 2) {
        let temp = *str.add(j);
        *str.add(j) = *str.add(len - 1 - j);
        *str.add(len - 1 - j) = temp;
    }

    str
}

// ─── Error Handling ───────────────────────────────────

/// Get error number
#[no_mangle]
pub extern "C" fn __errno_location() -> *mut I32 {
    // This would point to the global errno in the full implementation
    0 as *mut I32
}

/// Get error string
#[no_mangle]
pub extern "C" fn strerror(errnum: I32) -> *const U8 {
    // Return static error messages
    static mut ERROR_MESSAGES: [&str; 35] = [
        "Success",
        "Operation not permitted",
        "No such file or directory",
        "No such process",
        "Interrupted system call",
        "I/O error",
        "No such device or address",
        "Argument list too long",
        "Exec format error",
        "Bad file number",
        "No child processes",
        "Try again",
        "Out of memory",
        "Permission denied",
        "Bad address",
        "Block device required",
        "Device or resource busy",
        "File exists",
        "Cross-device link",
        "No such device",
        "Not a directory",
        "Is a directory",
        "Invalid argument",
        "File table overflow",
        "Too many open files",
        "Not a typewriter",
        "Text file busy",
        "File too large",
        "No space left on device",
        "Illegal seek",
        "Read-only file system",
        "Too many links",
        "Broken pipe",
        "Math argument out of domain of func",
        "Math result not representable",
    ];

    if errnum >= 0 && (errnum as Usize) < ERROR_MESSAGES.len() {
        unsafe { ERROR_MESSAGES[errnum as Usize].as_ptr() as *const U8 }
    } else {
        unsafe { ERROR_MESSAGES[0].as_ptr() as *const U8 }
    }
}

// ─── Exit Functions ───────────────────────────────────

/// Exit normally
#[no_mangle]
pub extern "C" fn exit(status: I32) -> ! {
    // This would call the POSIX exit function
    loop {}
}

/// Exit quickly
#[no_mangle]
pub extern "C" fn _exit(status: I32) -> ! {
    // This would call the POSIX exit function without cleanup
    loop {}
}

/// Abort
#[no_mangle]
pub extern "C" fn abort() -> ! {
    // This would raise SIGABRT
    loop {}
}
