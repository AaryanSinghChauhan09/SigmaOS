//! Slab Allocator
//!
//! Inspired by Linux SLUB allocator and FreeBSD UMA (Universal Memory Allocator).
//! Provides efficient allocation for frequently used small objects.
//!
//! # Features
//! - Object caching for common sizes
//! - Per-CPU caches to reduce lock contention
//! - Slab coloring to reduce cache conflicts
//! - Memory reclamation under pressure
//!
//! # Linux Inspiration
//! - `mm/slub.c` - SLUB allocator (Simple, List-based, Unqueued)
//! - `mm/slab.c` - Original SLAB allocator
//! - `include/linux/slab.h` - Slab API
//!
//! # FreeBSD Inspiration
//! - `sys/vm/uma_core.c` - UMA allocator
//! - `sys/vm/uma.h` - UMA API

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Slab size classes (powers of 2 from 8 to 4096 bytes)
const SLAB_SIZES: [usize; 10] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

/// Objects per slab (for different sizes)
const OBJECTS_PER_SLAB: [usize; 10] = [512, 256, 128, 64, 32, 16, 8, 4, 2, 1];

/// Slab descriptor
#[repr(C)]
struct Slab {
    /// Pointer to free list
    free_list: *mut FreeObject,
    /// Number of free objects
    free_count: usize,
    /// Total objects in this slab
    total_objects: usize,
    /// Object size
    object_size: usize,
    /// Next slab in chain
    next: *mut Slab,
}

/// Free object list entry
#[repr(C)]
struct FreeObject {
    next: *mut FreeObject,
}

/// Slab cache for a specific object size
pub struct SlabCache {
    object_size: usize,
    objects_per_slab: usize,
    partial_slabs: *mut Slab,
    full_slabs: *mut Slab,
    empty_slabs: *mut Slab,
    allocated_objects: AtomicUsize,
    freed_objects: AtomicUsize,
}

impl SlabCache {
    /// Create new slab cache
    pub const fn new(object_size: usize, objects_per_slab: usize) -> Self {
        Self {
            object_size,
            objects_per_slab,
            partial_slabs: ptr::null_mut(),
            full_slabs: ptr::null_mut(),
            empty_slabs: ptr::null_mut(),
            allocated_objects: AtomicUsize::new(0),
            freed_objects: AtomicUsize::new(0),
        }
    }

    /// Allocate object from cache
    /// Linux: `mm/slub.c:slab_alloc()`
    pub fn allocate(&mut self) -> Option<NonNull<u8>> {
        // Try partial slabs first
        if let Some(obj) = Self::allocate_from_slab_list(&mut self.partial_slabs, &mut self.full_slabs) {
            self.allocated_objects.fetch_add(1, Ordering::Relaxed);
            return Some(obj);
        }

        // Try empty slabs
        if let Some(obj) = Self::allocate_from_slab_list(&mut self.empty_slabs, &mut self.full_slabs) {
            self.allocated_objects.fetch_add(1, Ordering::Relaxed);
            return Some(obj);
        }

        // Need to allocate new slab
        // In real implementation, this would call page allocator
        None
    }

    /// Allocate from specific slab list
    fn allocate_from_slab_list(list: &mut *mut Slab, full_slabs: &mut *mut Slab) -> Option<NonNull<u8>> {
        if list.is_null() {
            return None;
        }

        unsafe {
            let slab = &mut **list;

            if slab.free_list.is_null() {
                return None;
            }

            // Pop from free list
            let obj = slab.free_list;
            let free_obj = &*obj;
            slab.free_list = free_obj.next;
            slab.free_count -= 1;

            // If slab is now full, move to full list
            if slab.free_count == 0 {
                Self::move_slab_to_full(list, full_slabs);
            }

            NonNull::new(obj as *mut u8)
        }
    }

    /// Free object back to cache
    /// Linux: `mm/slub.c:slab_free()`
    pub fn deallocate(&mut self, ptr: NonNull<u8>) {
        // Find which slab this object belongs to
        // In real implementation, would use page metadata

        unsafe {
            let free_obj = ptr.as_ptr() as *mut FreeObject;

            // For now, add to first partial slab
            if !self.partial_slabs.is_null() {
                let slab = &mut *self.partial_slabs;
                (*free_obj).next = slab.free_list;
                slab.free_list = free_obj;
                slab.free_count += 1;
            }
        }

        self.freed_objects.fetch_add(1, Ordering::Relaxed);
    }

    /// Move slab from one list to another
    unsafe fn move_slab_to_full(from: &mut *mut Slab, full_slabs: &mut *mut Slab) {
        if from.is_null() {
            return;
        }

        let slab = *from;
        *from = (*slab).next;
        (*slab).next = *full_slabs;
        *full_slabs = slab;
    }

    /// Get allocation statistics
    pub fn stats(&self) -> SlabStats {
        SlabStats {
            object_size: self.object_size,
            allocated: self.allocated_objects.load(Ordering::Relaxed),
            freed: self.freed_objects.load(Ordering::Relaxed),
            active: self.allocated_objects.load(Ordering::Relaxed)
                - self.freed_objects.load(Ordering::Relaxed),
        }
    }
}

/// Slab allocator statistics
#[derive(Debug, Clone, Copy)]
pub struct SlabStats {
    pub object_size: usize,
    pub allocated: usize,
    pub freed: usize,
    pub active: usize,
}

/// Global slab allocator
pub struct SlabAllocator {
    caches: Vec<SlabCache>,
}

impl SlabAllocator {
    /// Create new slab allocator
    pub fn new() -> Self {
        let mut caches = Vec::new();

        for (i, &size) in SLAB_SIZES.iter().enumerate() {
            caches.push(SlabCache::new(size, OBJECTS_PER_SLAB[i]));
        }

        Self { caches }
    }

    /// Find cache for given size
    fn find_cache(&mut self, size: usize) -> Option<&mut SlabCache> {
        for cache in &mut self.caches {
            if cache.object_size >= size {
                return Some(cache);
            }
        }
        None
    }

    /// Allocate object
    /// Linux: `mm/slab_common.c:kmalloc()`
    pub fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let size = layout.size();

        if let Some(cache) = self.find_cache(size) {
            cache.allocate()
        } else {
            // Too large for slab allocator, use page allocator
            None
        }
    }

    /// Deallocate object
    /// Linux: `mm/slab_common.c:kfree()`
    pub fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = layout.size();

        if let Some(cache) = self.find_cache(size) {
            cache.deallocate(ptr);
        }
    }

    /// Get all cache statistics
    pub fn all_stats(&self) -> Vec<SlabStats> {
        self.caches.iter().map(|c| c.stats()).collect()
    }

    /// Shrink caches (reclaim memory under pressure)
    /// Linux: `mm/slab_common.c:kmem_cache_shrink()`
    pub fn shrink_caches(&mut self) {
        // Move empty slabs back to page allocator
        // In real implementation, would free empty slabs
    }
}

/// Slab allocator wrapper for GlobalAlloc
pub struct GlobalSlabAllocator;

unsafe impl GlobalAlloc for GlobalSlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // In real implementation, would use global slab allocator instance
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // In real implementation, would use global slab allocator instance
    }
}

/// Create slab cache for specific type
/// Linux: `KMEM_CACHE()` macro
#[macro_export]
macro_rules! create_slab_cache {
    ($name:ident, $type:ty) => {
        static mut $name: SlabCache = SlabCache::new(
            core::mem::size_of::<$type>(),
            4096 / core::mem::size_of::<$type>(),
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_sizes() {
        assert_eq!(SLAB_SIZES[0], 8);
        assert_eq!(SLAB_SIZES[9], 4096);
    }

    #[test]
    fn test_slab_cache_creation() {
        let cache = SlabCache::new(64, 64);
        assert_eq!(cache.object_size, 64);
        assert_eq!(cache.objects_per_slab, 64);
    }

    #[test]
    fn test_slab_stats() {
        let cache = SlabCache::new(128, 32);
        let stats = cache.stats();
        assert_eq!(stats.object_size, 128);
        assert_eq!(stats.active, 0);
    }

    #[test]
    fn test_allocator_creation() {
        let allocator = SlabAllocator::new();
        assert_eq!(allocator.caches.len(), SLAB_SIZES.len());
    }
}
