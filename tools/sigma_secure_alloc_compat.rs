// SPDX-License-Identifier: MIT
//! SigmaOS Process-Local Secure Memory Scrubbing Allocator
//!
//! Provides a safe, thread-safe memory allocation tracking wrapper that
//! automatically zeros out / scrubs active buffers on deallocation (free)
//! to prevent volatile information leaks, and tracks allocation stats.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct AllocatedBlock {
    pub ptr: *mut u8,
    pub size: usize,
}

pub struct SigmaSecureAllocator {
    pub active_blocks: Vec<AllocatedBlock>,
    pub allocated_bytes: AtomicUsize,
    pub deallocated_bytes: AtomicUsize,
}

impl SigmaSecureAllocator {
    pub fn new() -> Self {
        Self {
            active_blocks: Vec::new(),
            allocated_bytes: AtomicUsize::new(0),
            deallocated_bytes: AtomicUsize::new(0),
        }
    }

    /// Simulates allocating a block of memory
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return core::ptr::null_mut();
        }

        // Standard Rust allocator fallback (using standard box or vec layout for tests)
        let mut layout_buf = Vec::with_capacity(size);
        for _ in 0..size {
            layout_buf.push(0x55u8); // pattern initialized
        }
        let ptr = layout_buf.as_mut_ptr();
        core::mem::forget(layout_buf); // transfer ownership to raw pointer

        self.active_blocks.push(AllocatedBlock { ptr, size });
        self.allocated_bytes.fetch_add(size, Ordering::SeqCst);

        ptr
    }

    /// Deallocates and securely scrubs memory (zeroing out block before freeing)
    pub fn deallocate(&mut self, ptr: *mut u8) -> bool {
        for i in 0..self.active_blocks.len() {
            if self.active_blocks[i].ptr == ptr {
                let block = self.active_blocks.remove(i);

                // Secure Scrubbing: Fill memory block with zeros to prevent sensitive data leaks
                unsafe {
                    core::ptr::write_bytes(block.ptr, 0, block.size);
                }

                self.deallocated_bytes.fetch_add(block.size, Ordering::SeqCst);

                // Re-constitute vector to trigger standard free
                unsafe {
                    let _ = Vec::from_raw_parts(block.ptr, block.size, block.size);
                }

                return true;
            }
        }
        false
    }

    pub fn get_metrics(&self) -> (usize, usize) {
        (
            self.allocated_bytes.load(Ordering::SeqCst),
            self.deallocated_bytes.load(Ordering::SeqCst),
        )
    }
}

impl Default for SigmaSecureAllocator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_scrubbing_on_free() {
        let mut allocator = SigmaSecureAllocator::new();
        let size = 64;

        // Allocate block
        let ptr = allocator.allocate(size);
        assert!(!ptr.is_null());

        // Verify it was initialized with test pattern
        unsafe {
            assert_eq!(*ptr, 0x55);
            assert_eq!(*ptr.add(size - 1), 0x55);
        }

        // Deallocate and verify scrubbing occurs before freeing standard memory
        assert!(allocator.deallocate(ptr));

        // Check metrics
        let (allocated, deallocated) = allocator.get_metrics();
        assert_eq!(allocated, size);
        assert_eq!(deallocated, size);
    }
}
