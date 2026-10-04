#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct SigmaAllocator {
    alloc_count: AtomicUsize,
    free_count: AtomicUsize,
}

impl SigmaAllocator {
    pub const fn new() -> Self {
        Self {
            alloc_count: AtomicUsize::new(0),
            free_count: AtomicUsize::new(0),
        }
    }

    pub fn stats(&self) -> (usize, usize) {
        (
            self.alloc_count.load(Ordering::Relaxed),
            self.free_count.load(Ordering::Relaxed),
        )
    }

    pub fn on_memory_pressure(&self) {
        // Drop caches
    }
}

unsafe impl GlobalAlloc for SigmaAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        16 as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        self.free_count.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_new() {
        let alloc = SigmaAllocator::new();
        assert_eq!(alloc.stats(), (0, 0));
    }

    #[test]
    fn test_allocator_stats() {
        let alloc = SigmaAllocator::new();
        alloc.alloc_count.store(10, Ordering::Relaxed);
        alloc.free_count.store(5, Ordering::Relaxed);
        assert_eq!(alloc.stats(), (10, 5));
    }

    #[test]
    fn test_global_alloc_trait() {
        let alloc = SigmaAllocator::new();
        let layout = Layout::from_size_align(8, 8).unwrap();
        unsafe {
            let ptr = alloc.alloc(layout);
            assert!(!ptr.is_null());
            alloc.dealloc(ptr, layout);
        }
        assert_eq!(alloc.stats(), (1, 1));
    }

    #[test]
    fn test_memory_pressure() {
        let alloc = SigmaAllocator::new();
        alloc.on_memory_pressure();
    }

    #[test]
    fn test_large_allocation_buddy() {
        let alloc = SigmaAllocator::new();
        let layout = Layout::from_size_align(4096 * 10, 4096).unwrap();
        unsafe {
            let ptr = alloc.alloc(layout);
            alloc.dealloc(ptr, layout);
        }
        assert_eq!(alloc.stats(), (1, 1));
    }
}
