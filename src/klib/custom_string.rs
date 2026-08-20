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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

impl Default for SigmaString {
    fn default() -> Self {
        Self::new()
    }
}
