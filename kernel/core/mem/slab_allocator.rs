//! SigmaOS Slab Allocator
//! Optimized for fixed-size small allocations, implementing the Allocator trait

#![no_std]

use core::ptr::null_mut;
use super::Allocator;

/// Slab Config
pub struct SlabConfig {
    pub object_size: usize,
    pub objects_per_slab: usize,
}

/// A single slab that holds multiple objects
#[repr(C)]
struct Slab {
    next: *mut Slab,
    free_list: *mut u8,
    in_use: usize,
}

/// Slab Allocator - optimized for fixed-size allocations
pub struct SlabAllocator {
    config: SlabConfig,
    slabs: *mut Slab,
    empty_slabs: *mut Slab,
    // For no_std (we'll handle memory manually
}

impl SlabAllocator {
    /// Create a new slab allocator
    pub const fn new(object_size: usize, objects_per_slab: usize) -> Self {
        SlabAllocator {
            config: SlabConfig {
                object_size,
                objects_per_slab,
            },
            slabs: null_mut(),
            empty_slabs: null_mut(),
        }
        // For no_std
    }

    /// Allocator for this allocator, you need to pass a Slab and use a
}

// Implement the Allocator trait (OOP: Polymorphism)
impl Allocator for SlabAllocator {
    fn alloc(&mut self, size: usize) -> *mut u8 {
        // If size doesn't match our object_size, return null (we only handle fixed size!
        if size != self.config.object_size {
            return null_mut();
        }

        // Find a slab with free space
        let mut curr_slab = self.slabs;
        while !curr_slab.is_null() {
            unsafe {
                if !(*curr_slab).free_list.is_null() {
                    let obj = (*curr_slab).free_list;
                    (*curr_slab).free_list = *(obj as *const *mut u8).read(); // Get next free
                    (*curr_slab).in_use += 1;
                    return obj;
                }
            }
            curr_slab = (*curr_slab).next;
        }

        null_mut() // No free slabs or no memory!
    }

    fn dealloc(&mut self, ptr: *mut u8, size: usize) {
        if size != self.config.object_size || ptr.is_null() {
            return;
        }

        // Find the slab for this ptr
        let mut curr_slab = self.slabs;
        let mut prev_slab = null_mut::<*mut Slab>();
        while !curr_slab.is_null() {
            unsafe {
                // Check if ptr is in this slab's memory range
                // We need to know the slab's start and end for this
                // For now, add ptr to the free_list
                // This is a simple implementation
                // Add ptr to the free_list
                *(ptr as *mut *mut u8) = (*curr_slab).free_list;
                (*curr_slab).free_list = ptr;
                (*curr_slab).in_use -= 1;
                break;
            }
            prev_slab = &mut curr_slab;
            curr_slab = (*curr_slab).next;
        }
    }
}

// Safe because we manage memory internally
unsafe impl Send for SlabAllocator {}
unsafe impl Sync for SlabAllocator {}
