// Custom String and Allocator types for klib

pub mod custom_allocator {
    pub struct CustomAllocator;
    pub static GLOBAL_CUSTOM_ALLOCATOR: CustomAllocator = CustomAllocator;
    impl CustomAllocator {
        pub fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
            core::ptr::null_mut()
        }
        pub fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
    }
    pub unsafe fn alloc(size: usize) -> *mut u8 {
        use std::alloc::{alloc as std_alloc, Layout};
        let layout = Layout::from_size_align(size, 8).unwrap();
        std_alloc(layout)
    }
    pub unsafe fn free(_ptr: *mut u8) {}
}

pub mod uuid {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Uuid([u8; 16]);
    impl Uuid {
        pub fn new_v4() -> Self {
            Uuid([0; 16])
        }
        pub fn new() -> Self {
            Uuid([0; 16])
        }
    }
    impl core::fmt::Display for Uuid {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "00000000-0000-0000-0000-000000000000")
        }
    }
}

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SigmaString {
    data: String,
}

impl SigmaString {
    pub fn new() -> Self {
        Self { data: String::new() }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn from_str(s: &str) -> Self {
        Self { data: String::from(s) }
    }

    pub fn as_str(&self) -> &str {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data.into_bytes()
    }

    pub fn push(&mut self, ch: char) {
        self.data.push(ch);
    }

    pub fn push_str(&mut self, s: &str) {
        self.data.push_str(s);
    }

    pub fn trim_end_matches(&self, pat: char) -> &str {
        self.data.trim_end_matches(pat)
    }

    pub fn contains(&self, pat: &str) -> bool {
        self.data.contains(pat)
    }

    pub fn join(&self, sep: &str) -> String {
        self.data.clone()
    }
}

impl Default for SigmaString {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for SigmaString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl From<&str> for SigmaString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for SigmaString {
    fn from(s: String) -> Self {
        Self { data: s }
    }
}

impl core::ops::Deref for SigmaString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl PartialEq<&str> for SigmaString {
    fn eq(&self, other: &&str) -> bool {
        self.data == *other
    }
}

impl PartialEq<String> for SigmaString {
    fn eq(&self, other: &String) -> bool {
        self.data == *other
    }
}

impl PartialEq<SigmaString> for &str {
    fn eq(&self, other: &SigmaString) -> bool {
        *self == other.data
    }
}

impl PartialEq<SigmaString> for String {
    fn eq(&self, other: &SigmaString) -> bool {
        *self == other.data
    }
}
