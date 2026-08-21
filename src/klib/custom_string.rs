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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SigmaString {
    data: String,
}

impl PartialEq<&str> for SigmaString {
    fn eq(&self, other: &&str) -> bool {
        self.data == *other
    }
}

impl From<&str> for SigmaString {
    fn from(s: &str) -> Self {
        SigmaString::from_str(s)
    }
}

impl From<String> for SigmaString {
    fn from(s: String) -> Self {
        Self { data: s }
    }
}

impl PartialEq<str> for SigmaString {
    fn eq(&self, other: &str) -> bool {
        self.data == *other
    }
}

impl PartialEq<String> for SigmaString {
    fn eq(&self, other: &String) -> bool {
        self.data == *other
    }
}

impl core::fmt::Display for SigmaString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl SigmaString {
    pub fn new() -> Self {
        Self { data: String::new() }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data.into_bytes()
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

    pub fn push(&mut self, ch: char) {
        self.data.push(ch);
    }

    pub fn push_str(&mut self, s: &str) {
        self.data.push_str(s);
    }

    pub fn join(&self, path: &str) -> SigmaString {
        let mut new_path = self.data.clone();
        if !new_path.ends_with('/') && !path.starts_with('/') {
            new_path.push('/');
        }
        new_path.push_str(path);
        SigmaString { data: new_path }
    }
}

impl Default for SigmaString {
    fn default() -> Self {
        Self::new()
    }
}
