#![no_std]
#![no_main]

/// Custom Memory Allocator for SigmaOS
/// Implements memory allocation without relying on std::alloc
/// Uses buddy system algorithm for efficient memory management

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Block size for buddy system (4KB)
const BLOCK_SIZE: usize = 4096;

/// Maximum order for buddy system (2^MAX_ORDER * BLOCK_SIZE)
const MAX_ORDER: usize = 10;

/// Memory block structure
#[repr(C)]
struct MemoryBlock {
    size: usize,
    used: AtomicBool,
    next: Option<NonNull<MemoryBlock>>,
    prev: Option<NonNull<MemoryBlock>>,
}

/// Buddy system memory allocator
pub struct BuddyAllocator {
    free_lists: [Option<NonNull<MemoryBlock>>; MAX_ORDER + 1],
    total_memory: AtomicUsize,
    used_memory: AtomicUsize,
    heap_start: *mut u8,
    heap_size: usize,
}

impl BuddyAllocator {
    /// Create a new buddy allocator
    pub unsafe fn new(heap_start: *mut u8, heap_size: usize) -> Self {
        let mut allocator = BuddyAllocator {
            free_lists: [None; MAX_ORDER + 1],
            total_memory: AtomicUsize::new(heap_size),
            used_memory: AtomicUsize::new(0),
            heap_start,
            heap_size,
        };

        // Initialize free lists
        let mut remaining = heap_size;
        let mut current = heap_start;

        while remaining >= BLOCK_SIZE {
            let order = Self::calculate_order(remaining);
            let block_size = BLOCK_SIZE * (1 << order);
            
            let block = current as *mut MemoryBlock;
            (*block).size = block_size;
            (*block).used.store(false, Ordering::SeqCst);
            (*block).next = None;
            (*block).prev = None;

            allocator.add_to_free_list(block, order);

            current = current.add(block_size);
            remaining -= block_size;
        }

        allocator
    }

    /// Calculate order for a given size
    fn calculate_order(size: usize) -> usize {
        let mut order = 0;
        let mut block_size = BLOCK_SIZE;
        while block_size < size && order < MAX_ORDER {
            block_size *= 2;
            order += 1;
        }
        order
    }

    /// Add block to free list
    unsafe fn add_to_free_list(&mut self, block: *mut MemoryBlock, order: usize) {
        (*block).next = self.free_lists[order];
        if let Some(mut head) = self.free_lists[order] {
            (*head.as_ptr()).prev = Some(NonNull::new_unchecked(block));
        }
        self.free_lists[order] = Some(NonNull::new_unchecked(block));
    }

    /// Remove block from free list
    unsafe fn remove_from_free_list(&mut self, block: *mut MemoryBlock, order: usize) {
        let prev = (*block).prev;
        let next = (*block).next;

        match prev {
            Some(mut prev_block) => {
                (*prev_block.as_ptr()).next = next;
            }
            None => {
                self.free_lists[order] = next;
            }
        }

        if let Some(mut next_block) = next {
            (*next_block.as_ptr()).prev = prev;
        }

        (*block).prev = None;
        (*block).next = None;
    }

    /// Allocate memory of given size
    pub unsafe fn allocate(&mut self, size: usize) -> *mut u8 {
        let aligned_size = (size + core::mem::size_of::<MemoryBlock>() + 15) & !15;
        let order = Self::calculate_order(aligned_size);

        // Find a free block of sufficient size
        let mut current_order = order;
        let mut block = None;

        while current_order <= MAX_ORDER {
            if let Some(free_block) = self.free_lists[current_order] {
                block = Some(free_block);
                break;
            }
            current_order += 1;
        }

        if block.is_none() {
            return ptr::null_mut();
        }

        let mut block = block.unwrap();
        self.remove_from_free_list(block.as_ptr(), current_order);

        // Split block if necessary
        while current_order > order {
            current_order -= 1;
            let split_size = BLOCK_SIZE * (1 << current_order);
            let buddy = (block.as_ptr() as *mut u8).add(split_size) as *mut MemoryBlock;

            (*buddy).size = split_size;
            (*buddy).used.store(false, Ordering::SeqCst);
            (*buddy).next = None;
            (*buddy).prev = None;

            self.add_to_free_list(buddy, current_order);

            (*block.as_ptr()).size = split_size;
        }

        (*block.as_ptr()).used.store(true, Ordering::SeqCst);
        self.used_memory.fetch_add((*block.as_ptr()).size, Ordering::SeqCst);

        (block.as_ptr() as *mut u8).add(core::mem::size_of::<MemoryBlock>())
    }

    /// Deallocate memory
    pub unsafe fn deallocate(&mut self, ptr: *mut u8) {
        let block = (ptr as *mut u8).sub(core::mem::size_of::<MemoryBlock>()) as *mut MemoryBlock;
        
        if !(*block).used.load(Ordering::SeqCst) {
            return; // Already freed
        }

        (*block).used.store(false, Ordering::SeqCst);
        self.used_memory.fetch_sub((*block).size, Ordering::SeqCst);

        let size = (*block).size;
        let order = Self::calculate_order(size);

        // Try to merge with buddy
        let mut current_block = block;
        let mut current_order = order;

        while current_order < MAX_ORDER {
            let buddy = Self::get_buddy(current_block, current_order);
            
            if (*buddy).used.load(Ordering::SeqCst) || (*buddy).size != (*current_block).size {
                break;
            }

            // Remove buddy from free list
            self.remove_from_free_list(buddy, current_order);

            // Merge blocks
            if current_block < buddy {
                (*current_block).size *= 2;
            } else {
                (*buddy).size *= 2;
                current_block = buddy;
            }

            current_order += 1;
        }

        self.add_to_free_list(current_block, current_order);
    }

    /// Get buddy block
    unsafe fn get_buddy(block: *mut MemoryBlock, order: usize) -> *mut MemoryBlock {
        let block_addr = block as usize;
        let size = BLOCK_SIZE * (1 << order);
        let buddy_addr = block_addr ^ size;
        buddy_addr as *mut MemoryBlock
    }

    /// Get memory statistics
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            total: self.total_memory.load(Ordering::SeqCst),
            used: self.used_memory.load(Ordering::SeqCst),
            free: self.total_memory.load(Ordering::SeqCst) - self.used_memory.load(Ordering::SeqCst),
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total: usize,
    pub used: usize,
    pub free: usize,
}

/// Global allocator instance
static mut GLOBAL_ALLOCATOR: Option<BuddyAllocator> = None;

/// Initialize global allocator
pub unsafe fn init_allocator(heap_start: *mut u8, heap_size: usize) {
    GLOBAL_ALLOCATOR = Some(BuddyAllocator::new(heap_start, heap_size));
}

/// Allocate memory using global allocator
pub unsafe fn alloc(size: usize) -> *mut u8 {
    if let Some(ref mut allocator) = GLOBAL_ALLOCATOR {
        allocator.allocate(size)
    } else {
        ptr::null_mut()
    }
}

/// Deallocate memory using global allocator
pub unsafe fn free(ptr: *mut u8) {
    if let Some(ref mut allocator) = GLOBAL_ALLOCATOR {
        allocator.deallocate(ptr);
    }
}

/// Get memory statistics
pub unsafe fn get_stats() -> MemoryStats {
    if let Some(ref allocator) = GLOBAL_ALLOCATOR {
        allocator.stats()
    } else {
        MemoryStats {
            total: 0,
            used: 0,
            free: 0,
        }
    }
}
