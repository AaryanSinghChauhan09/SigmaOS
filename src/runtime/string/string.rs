#![no_std]
#![no_main]

/// Custom String Handling for SigmaOS
/// Implements string operations without relying on std::string
/// Supports UTF-8 encoding and common string operations

use core::ptr::{self, NonNull};
use core::mem;

/// Custom string structure
#[repr(C)]
pub struct SigmaString {
    data: NonNull<u8>,
    length: usize,
    capacity: usize,
}

impl SigmaString {
    /// Create a new empty string
    pub fn new() -> Self {
        SigmaString {
            data: NonNull::dangling(),
            length: 0,
            capacity: 0,
        }
    }

    /// Create a string from a byte slice
    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        let len = bytes.len();
        let capacity = len.next_power_of_two();
        
        let data = alloc(capacity);
        if data.is_null() {
            return SigmaString::new();
        }

        ptr::copy_nonoverlapping(bytes.as_ptr(), data, len);
        
        SigmaString {
            data: NonNull::new_unchecked(data),
            length: len,
            capacity,
        }
    }

    /// Create a string from a C-style string
    pub unsafe fn from_c_str(ptr: *const i8) -> Self {
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }

        let bytes = core::slice::from_raw_parts(ptr as *const u8, len);
        Self::from_bytes(bytes)
    }

    /// Get string length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if string is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Get string capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get string as bytes
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.data.as_ptr(), self.length)
        }
    }

    /// Append bytes to string
    pub unsafe fn push_bytes(&mut self, bytes: &[u8]) {
        let new_len = self.length + bytes.len();
        
        if new_len > self.capacity {
            self.reallocate(new_len.next_power_of_two());
        }

        if self.capacity >= new_len {
            let dest = self.data.as_ptr().add(self.length);
            ptr::copy_nonoverlapping(bytes.as_ptr(), dest, bytes.len());
            self.length = new_len;
        }
    }

    /// Append a character to string
    pub unsafe fn push_char(&mut self, c: char) {
        let mut buf = [0u8; 4];
        let len = c.encode_utf8(&mut buf).len();
        self.push_bytes(&buf[..len]);
    }

    /// Append another string
    pub unsafe fn push_str(&mut self, other: &SigmaString) {
        self.push_bytes(other.as_bytes());
    }

    /// Reallocate string buffer
    unsafe fn reallocate(&mut self, new_capacity: usize) {
        if new_capacity <= self.capacity {
            return;
        }

        let new_data = alloc(new_capacity);
        if new_data.is_null() {
            return;
        }

        if self.capacity > 0 {
            ptr::copy_nonoverlapping(self.data.as_ptr(), new_data, self.length);
            free(self.data.as_ptr());
        }

        self.data = NonNull::new_unchecked(new_data);
        self.capacity = new_capacity;
    }

    /// Clear string
    pub unsafe fn clear(&mut self) {
        self.length = 0;
    }

    /// Truncate string to new length
    pub unsafe fn truncate(&mut self, new_len: usize) {
        if new_len < self.length {
            self.length = new_len;
        }
    }

    /// Remove last character
    pub unsafe fn pop(&mut self) -> Option<char> {
        if self.length == 0 {
            return None;
        }

        // Find last UTF-8 character boundary
        let mut len = 1;
        while len <= self.length {
            let byte = *self.data.as_ptr().add(self.length - len);
            if (byte & 0xC0) != 0x80 {
                break;
            }
            len += 1;
        }

        let bytes = &self.as_bytes()[self.length - len..];
        let c = core::str::from_utf8_unchecked(bytes).chars().next();
        self.length -= len;
        c
    }

    /// Compare two strings
    pub fn compare(&self, other: &SigmaString) -> i32 {
        let self_bytes = self.as_bytes();
        let other_bytes = other.as_bytes();
        let min_len = self_bytes.len().min(other_bytes.len());

        for i in 0..min_len {
            if self_bytes[i] < other_bytes[i] {
                return -1;
            } else if self_bytes[i] > other_bytes[i] {
                return 1;
            }
        }

        if self_bytes.len() < other_bytes.len() {
            -1
        } else if self_bytes.len() > other_bytes.len() {
            1
        } else {
            0
        }
    }

    /// Check if strings are equal
    pub fn equals(&self, other: &SigmaString) -> bool {
        self.compare(other) == 0
    }

    /// Find substring
    pub fn find(&self, pattern: &SigmaString) -> Option<usize> {
        let self_bytes = self.as_bytes();
        let pattern_bytes = pattern.as_bytes();

        if pattern_bytes.is_empty() {
            return Some(0);
        }

        if pattern_bytes.len() > self_bytes.len() {
            return None;
        }

        for i in 0..=(self_bytes.len() - pattern_bytes.len()) {
            if &self_bytes[i..i + pattern_bytes.len()] == pattern_bytes {
                return Some(i);
            }
        }

        None
    }

    /// Split string by delimiter
    pub unsafe fn split(&self, delimiter: u8) -> SigmaStringArray {
        let mut array = SigmaStringArray::new();
        let mut start = 0;
        let bytes = self.as_bytes();

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == delimiter {
                let slice = &bytes[start..i];
                array.push(SigmaString::from_bytes(slice));
                start = i + 1;
            }
        }

        if start < bytes.len() {
            let slice = &bytes[start..];
            array.push(SigmaString::from_bytes(slice));
        }

        array
    }

    /// Trim whitespace from string
    pub unsafe fn trim(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut start = 0;
        let mut end = bytes.len();

        while start < end && (bytes[start] == b' ' || bytes[start] == b'\t' || bytes[start] == b'\n' || bytes[start] == b'\r') {
            start += 1;
        }

        while end > start && (bytes[end - 1] == b' ' || bytes[end - 1] == b'\t' || bytes[end - 1] == b'\n' || bytes[end - 1] == b'\r') {
            end -= 1;
        }

        SigmaString::from_bytes(&bytes[start..end])
    }

    /// Convert to lowercase
    pub unsafe fn to_lowercase(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut result = SigmaString::new();

        for &byte in bytes {
            if byte >= b'A' && byte <= b'Z' {
                result.push_bytes(&[byte + 32]);
            } else {
                result.push_bytes(&[byte]);
            }
        }

        result
    }

    /// Convert to uppercase
    pub unsafe fn to_uppercase(&self) -> SigmaString {
        let bytes = self.as_bytes();
        let mut result = SigmaString::new();

        for &byte in bytes {
            if byte >= b'a' && byte <= b'z' {
                result.push_bytes(&[byte - 32]);
            } else {
                result.push_bytes(&[byte]);
            }
        }

        result
    }
}

impl Drop for SigmaString {
    fn drop(&mut self) {
        unsafe {
            if self.capacity > 0 {
                free(self.data.as_ptr());
            }
        }
    }
}

/// String array for split operations
#[repr(C)]
pub struct SigmaStringArray {
    data: *mut SigmaString,
    length: usize,
    capacity: usize,
}

impl SigmaStringArray {
    pub fn new() -> Self {
        SigmaStringArray {
            data: ptr::null_mut(),
            length: 0,
            capacity: 0,
        }
    }

    pub unsafe fn push(&mut self, string: SigmaString) {
        if self.length >= self.capacity {
            self.reallocate(if self.capacity == 0 { 4 } else { self.capacity * 2 });
        }

        if self.capacity > self.length {
            ptr::write(self.data.add(self.length), string);
            self.length += 1;
        }
    }

    unsafe fn reallocate(&mut self, new_capacity: usize) {
        let new_data = alloc(new_capacity * mem::size_of::<SigmaString>());
        if new_data.is_null() {
            return;
        }

        if self.capacity > 0 {
            for i in 0..self.length {
                let string = ptr::read(self.data.add(i));
                ptr::write(new_data.add(i), string);
            }
            free(self.data);
        }

        self.data = new_data as *mut SigmaString;
        self.capacity = new_capacity;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub unsafe fn get(&self, index: usize) -> Option<&SigmaString> {
        if index < self.length {
            Some(&*self.data.add(index))
        } else {
            None
        }
    }
}

impl Drop for SigmaStringArray {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.length {
                ptr::drop_in_place(self.data.add(i));
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
        }
    }
}

/// String comparison functions
pub unsafe fn strlen(ptr: *const i8) -> usize {
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    len
}

pub unsafe fn strcpy(dest: *mut i8, src: *const i8) -> *mut i8 {
    let mut i = 0;
    loop {
        let byte = *src.add(i);
        *dest.add(i) = byte;
        if byte == 0 {
            break;
        }
        i += 1;
    }
    dest
}

pub unsafe fn strcmp(s1: *const i8, s2: *const i8) -> i32 {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i) as u8;
        let c2 = *s2.add(i) as u8;
        
        if c1 < c2 {
            return -1;
        } else if c1 > c2 {
            return 1;
        } else if c1 == 0 {
            return 0;
        }
        i += 1;
    }
}

pub unsafe fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32 {
    for i in 0..n {
        let c1 = *s1.add(i) as u8;
        let c2 = *s2.add(i) as u8;
        
        if c1 < c2 {
            return -1;
        } else if c1 > c2 {
            return 1;
        } else if c1 == 0 {
            return 0;
        }
    }
    0
}

pub unsafe fn strcat(dest: *mut i8, src: *const i8) -> *mut i8 {
    let dest_len = strlen(dest);
    strcpy(dest.add(dest_len), src);
    dest
}

// External allocator functions (from allocator.rs)
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
