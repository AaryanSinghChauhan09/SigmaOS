// SPDX-License-Identifier: MIT OR Apache-2.0
// SigmaOS klib::slab - Slab Allocator (zero external dependencies)
// Inspired by Linux's SLAB/SLUB allocator and FreeBSD's UMA (Universal Memory Allocator)
// Uses only core, no std or alloc required

use core::alloc::Layout;
use core::cell::UnsafeCell;
use core::mem::{self, MaybeUninit};
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Maximum number of object sizes a slab cache can hold
const MAX_SLAB_PAGES: usize = 16;

/// A slab cache for a fixed-size object type.
/// Provides O(1) alloc/free with minimal fragmentation.
/// Inspired by Linux `kmem_cache_t` and FreeBSD `uma_zone_t`.
pub struct SlabCache {
    object_size: usize,
    objects_per_slab: usize,
    free_count: AtomicUsize,
    total_count: AtomicUsize,
    /// Free-list head: index into slab backing store
    free_head: AtomicUsize,
    /// Slab backing memory (statically sized for no_std)
    backing: UnsafeCell<[MaybeUninit<u8>; 65536]>, // 64KB per slab cache
    initialized: AtomicUsize, // 0 = not init, 1 = initialized
}

unsafe impl Sync for SlabCache {}
unsafe impl Send for SlabCache {}

/// Free object header embedded in free objects (intrusive free list)
#[repr(C)]
struct FreeNode {
    next: usize, // Index to next free node, or usize::MAX if last
}

impl SlabCache {
    /// Create a new slab cache (const for static initialization)
    pub const fn new(object_size: usize) -> Self {
        let aligned_size = if object_size < mem::size_of::<FreeNode>() {
            mem::size_of::<FreeNode>()
        } else {
            // Round up to pointer alignment
            (object_size + mem::align_of::<usize>() - 1) & !(mem::align_of::<usize>() - 1)
        };

        let objects_per_slab = if aligned_size > 0 { 65536 / aligned_size } else { 0 };

        Self {
            object_size: aligned_size,
            objects_per_slab,
            free_count: AtomicUsize::new(0),
            total_count: AtomicUsize::new(0),
            free_head: AtomicUsize::new(usize::MAX),
            backing: UnsafeCell::new([MaybeUninit::uninit(); 65536]),
            initialized: AtomicUsize::new(0),
        }
    }

    /// Initialize the slab cache. Must be called before first alloc.
    pub fn init(&self) {
        if self.initialized.load(Ordering::Relaxed) != 0 {
            return; // Already initialized
        }

        if self.object_size == 0 || self.objects_per_slab == 0 {
            return;
        }

        // Build the free list by writing FreeNode headers into each slot
        // SAFETY: We have exclusive access during initialization
        let backing = unsafe { &mut *self.backing.get() };

        for i in 0..self.objects_per_slab {
            let offset = i * self.object_size;
            let next = if i + 1 < self.objects_per_slab { i + 1 } else { usize::MAX };

            // Write FreeNode into the backing buffer
            let node = FreeNode { next };
            let node_bytes = unsafe {
                core::slice::from_raw_parts(
                    &node as *const FreeNode as *const MaybeUninit<u8>,
                    mem::size_of::<FreeNode>(),
                )
            };
            backing[offset..offset + mem::size_of::<FreeNode>()].copy_from_slice(node_bytes);
        }

        self.free_head.store(0, Ordering::Release);
        self.free_count.store(self.objects_per_slab, Ordering::Release);
        self.total_count.store(self.objects_per_slab, Ordering::Release);
        self.initialized.store(1, Ordering::Release);
    }

    /// Allocate an object from this slab cache. Returns byte offset into backing store.
    pub fn alloc(&self) -> Option<usize> {
        let head = self.free_head.load(Ordering::Acquire);
        if head == usize::MAX {
            return None; // No free objects
        }

        let offset = head * self.object_size;

        // Read next free node from the current head
        // SAFETY: head is a valid index into backing store
        let next = unsafe {
            let backing = &*self.backing.get();
            let node_ptr = backing[offset..].as_ptr() as *const FreeNode;
            (*node_ptr).next
        };

        self.free_head.store(next, Ordering::Release);
        self.free_count.fetch_sub(1, Ordering::Relaxed);

        Some(offset)
    }

    /// Free an object by its byte offset into the backing store.
    pub fn free(&self, offset: usize) {
        // Push onto free list head
        let old_head = self.free_head.load(Ordering::Acquire);

        let node = FreeNode { next: old_head };

        // SAFETY: offset is within backing store bounds
        unsafe {
            let backing = &mut *self.backing.get();
            let node_bytes = core::slice::from_raw_parts(
                &node as *const FreeNode as *const MaybeUninit<u8>,
                mem::size_of::<FreeNode>(),
            );
            backing[offset..offset + mem::size_of::<FreeNode>()].copy_from_slice(node_bytes);
        }

        let slot_index = offset / self.object_size;
        self.free_head.store(slot_index, Ordering::Release);
        self.free_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get a mutable pointer to an object at offset
    /// SAFETY: Caller must ensure the offset was returned by alloc() and not freed
    pub unsafe fn get_ptr(&self, offset: usize) -> *mut u8 {
        let backing = &mut *self.backing.get();
        backing[offset].as_mut_ptr()
    }

    /// Statistics
    pub fn free_count(&self) -> usize {
        self.free_count.load(Ordering::Relaxed)
    }

    pub fn allocated_count(&self) -> usize {
        self.total_count.load(Ordering::Relaxed)
            .saturating_sub(self.free_count())
    }

    pub fn utilization_percent(&self) -> u8 {
        let total = self.total_count.load(Ordering::Relaxed);
        if total == 0 { return 0; }
        let used = self.allocated_count();
        ((used * 100) / total) as u8
    }
}

/// A global slab cache registry inspired by Linux's kmem_cache
/// Provides named caches for common kernel object sizes
pub struct SlabRegistry {
    caches: [Option<&'static SlabCache>; 16],
    count: usize,
}

impl SlabRegistry {
    pub const fn new() -> Self {
        Self {
            caches: [None; 16],
            count: 0,
        }
    }

    pub fn register(&mut self, cache: &'static SlabCache) -> Option<usize> {
        if self.count >= 16 {
            return None;
        }
        let idx = self.count;
        self.caches[idx] = Some(cache);
        self.count += 1;
        Some(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&'static SlabCache> {
        self.caches.get(idx)?.copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_alloc_free() {
        static CACHE: SlabCache = SlabCache::new(64);
        CACHE.init();

        let offset1 = CACHE.alloc().expect("Should allocate");
        let offset2 = CACHE.alloc().expect("Should allocate");

        assert_ne!(offset1, offset2);
        assert_eq!(CACHE.allocated_count(), 2);

        CACHE.free(offset1);
        assert_eq!(CACHE.allocated_count(), 1);

        let offset3 = CACHE.alloc().expect("Should reuse freed slot");
        assert_eq!(offset3, offset1); // LIFO: freed slot is reused first

        CACHE.free(offset2);
        CACHE.free(offset3);
        assert_eq!(CACHE.allocated_count(), 0);
    }

    #[test]
    fn test_slab_capacity() {
        static CACHE: SlabCache = SlabCache::new(1024);
        CACHE.init();

        let capacity = CACHE.objects_per_slab;
        assert!(capacity > 0);
        assert!(CACHE.free_count() == capacity);
    }
}
