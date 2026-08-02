// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::klib::buddy_allocator::{BuddyAllocator, SimpleBuddyAllocator};
use crate::kernel::memory::{BuddyAllocator as KernelBuddyAllocator, MemoryBlock, PAGE_SIZE};

/// SigmaOS Buddy Allocator Wrapper
///
/// Wraps the klib buddy allocator and exposes a kernel-friendly interface.
/// Integrates with the existing memory subsystem.
pub struct SigmaBuddyAllocator {
    pub inner: SimpleBuddyAllocator,
    pub base_addr: usize,
    pub total_size: usize,
    pub allocated: AtomicUsize,
}

impl SigmaBuddyAllocator {
    pub fn new(base_addr: usize, total_size: usize, max_order: usize) -> Self {
        SigmaBuddyAllocator {
            inner: SimpleBuddyAllocator::new(max_order, total_size / PAGE_SIZE),
            base_addr,
            total_size,
            allocated: AtomicUsize::new(0),
        }
    }

    pub fn init(&mut self) {
        self.inner = SimpleBuddyAllocator::new(10, self.total_size / PAGE_SIZE);
    }

    pub fn allocate(&mut self, size: usize) -> Option<MemoryBlock> {
        if size == 0 || size > self.total_size {
            return None;
        }
        let pages = size.div_ceil(PAGE_SIZE);
        let order = Self::calculate_order(pages);
        match self.inner.allocate(order) {
            Ok(block_id) => {
                let addr = self.base_addr + (block_id * PAGE_SIZE);
                let actual_size = (1 << order) * PAGE_SIZE;
                self.allocated.fetch_add(actual_size, Ordering::SeqCst);
                use core::ptr::NonNull;
                NonNull::new(addr as *mut u8).map(|addr| MemoryBlock {
                    addr,
                    size: actual_size,
                })
            }
            Err(_) => None,
        }
    }

    pub fn free(&mut self, block: &MemoryBlock) {
        let addr = block.addr.as_ptr() as usize;
        let block_id = (addr - self.base_addr) / PAGE_SIZE;
        let pages = block.size / PAGE_SIZE;
        let order = Self::calculate_order(pages);
        let _ = self.inner.free(block_id, order);
        self.allocated.fetch_sub(block.size, Ordering::SeqCst);
    }

    pub fn get_free_memory(&self) -> usize {
        self.total_size - self.allocated.load(Ordering::SeqCst)
    }

    pub fn get_total_memory(&self) -> usize {
        self.total_size
    }

    pub fn get_used_memory(&self) -> usize {
        self.allocated.load(Ordering::SeqCst)
    }

    fn calculate_order(pages: usize) -> usize {
        if pages <= 1 {
            0
        } else {
            pages.next_power_of_two().trailing_zeros() as usize
        }
    }
}

impl Default for SigmaBuddyAllocator {
    fn default() -> Self {
        Self {
            inner: SimpleBuddyAllocator::new(10, 1024),
            base_addr: 0,
            total_size: 0,
            allocated: AtomicUsize::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddy_wrapper_allocation() {
        let mut allocator = SigmaBuddyAllocator::new(0x1000_0000, 4 * 1024 * 1024, 12);
        allocator.init();
        let block = allocator.allocate(4096);
        assert!(block.is_some());
    }

    #[test]
    fn test_buddy_wrapper_stats() {
        let mut allocator = SigmaBuddyAllocator::new(0x1000_0000, 4 * 1024 * 1024, 12);
        allocator.init();
        let total = allocator.get_total_memory();
        assert_eq!(total, 4 * 1024 * 1024);
    }
}
