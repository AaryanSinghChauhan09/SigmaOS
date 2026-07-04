
//! SigmaOS Buddy Allocator
//! A simple buddy memory allocator for kernel use.

#![no_std]
#![allow(unused)]

use core::ptr::null_mut;

/// Order represents the power-of-two size of a block.
pub type Order = usize;

/// The maximum order we can allocate.
pub const MAX_ORDER: Order = 20; // 1MB blocks

/// A block in the buddy allocator's free list.
#[repr(C)]
struct Block {
    next: *mut Block,
}

/// The buddy allocator itself.
pub struct BuddyAllocator {
    free_lists: [*mut Block; MAX_ORDER + 1],
    base_addr: usize,
    total_size: usize,
    initialized: bool,
}

impl BuddyAllocator {
    /// Creates a new, uninitialized BuddyAllocator.
    pub const fn new() -> Self {
        BuddyAllocator {
            free_lists: [null_mut(); MAX_ORDER + 1],
            base_addr: 0,
            total_size: 0,
            initialized: false,
        }
    }

    /// Initializes the buddy allocator with the given memory range.
    pub fn init(&mut self, base: usize, size: usize) {
        self.base_addr = base;
        self.total_size = size;
        self.initialized = true;

        // Find the largest order that fits in our size
        let mut order = MAX_ORDER;
        while order > 0 && (1 << order) > size {
            order -= 1;
        }

        // Add the initial block to the free list
        let initial_block = base as *mut Block;
        unsafe {
            (*initial_block).next = null_mut();
        }
        self.free_lists[order] = initial_block;
    }

    /// Allocates a block of the given order.
    pub fn alloc(&mut self, order: Order) -> *mut u8 {
        if order > MAX_ORDER || !self.initialized {
            return null_mut();
        }

        // Find the smallest order with a free block
        let mut current_order = order;
        while current_order <= MAX_ORDER && self.free_lists[current_order].is_null() {
            current_order += 1;
        }

        if current_order > MAX_ORDER {
            return null_mut(); // No memory available
        }

        // Take the block from the free list
        let block = self.free_lists[current_order];
        unsafe {
            self.free_lists[current_order] = (*block).next;
        }

        // Split blocks down to the requested order
        while current_order > order {
            current_order -= 1;
            let buddy_addr = (block as usize) ^ (1 << current_order);
            let buddy_block = buddy_addr as *mut Block;
            unsafe {
                (*buddy_block).next = self.free_lists[current_order];
            }
            self.free_lists[current_order] = buddy_block;
        }

        block as *mut u8
    }

    /// Deallocates a block of the given order.
    pub fn dealloc(&mut self, addr: *mut u8, order: Order) {
        if order > MAX_ORDER || !self.initialized || addr.is_null() {
            return;
        }

        let mut current_addr = addr as usize;
        let mut current_order = order;

        while current_order <= MAX_ORDER {
            let buddy_addr = current_addr ^ (1 << current_order);
            let mut found_buddy = false;

            // Check if the buddy is in the free list
            let mut prev = null_mut::<*mut Block>();
            let mut curr = self.free_lists[current_order];

            while !curr.is_null() {
                unsafe {
                    if curr as usize == buddy_addr {
                        // Remove buddy from free list
                        if prev.is_null() {
                            self.free_lists[current_order] = (*curr).next;
                        } else {
                            (**prev).next = (*curr).next;
                        }
                        found_buddy = true;
                        break;
                    }
                    prev = &mut curr;
                    curr = (*curr).next;
                }
            }

            if found_buddy {
                // Merge blocks
                current_addr = core::cmp::min(current_addr, buddy_addr);
                current_order += 1;
            } else {
                // Add the block to the free list and exit
                let block = current_addr as *mut Block;
                unsafe {
                    (*block).next = self.free_lists[current_order];
                }
                self.free_lists[current_order] = block;
                break;
            }
        }
    }
}

// Safe because we manage memory internally
unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}
