extern crate alloc;

// SigmaOS klib: Slab Allocator (like Linux SLUB/SLAB, FreeBSD UMA)
// Custom memory allocator for fixed-size object allocation pools
// No external dependencies - fully sovereign implementation

#![allow(dead_code)]

use core::alloc::Layout;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A slab cache for fixed-size allocations.
/// Inspired by Linux's SLAB/SLUB allocator and FreeBSD's UMA (Universal Memory Allocator).
/// 
/// Key design choices from Linux SLUB:
/// - Per-cache object size (no wasted padding beyond alignment)
/// - Free list stored inside free objects (no metadata overhead per object)
/// - Batch allocation from buddy allocator
pub struct SlabCache {
    /// Size of each object in bytes
    object_size: usize,
    /// Alignment requirement
    align: usize,
    /// Total objects in the pool
    total: usize,
    /// Free list head (index into pool, or usize::MAX for end)
    free_head: AtomicUsize,
    /// The backing memory
    pool: *mut u8,
    /// Number of allocated objects
    allocated: AtomicUsize,
    /// Cache name (for debugging, like Linux's kmem_cache_create name param)
    name: &'static str,
}

// SAFETY: The SlabCache manages memory exclusively through its interface
unsafe impl Send for SlabCache {}
unsafe impl Sync for SlabCache {}

const FREE_END: usize = usize::MAX;

impl SlabCache {
    /// Create a new slab cache.
    /// `object_size`: size of each object, must be >= size_of::<usize>()
    /// `capacity`: maximum number of objects
    /// `name`: debugging name (like Linux kmem_cache_create)
    pub unsafe fn new(object_size: usize, align: usize, capacity: usize, name: &'static str) -> Option<Self> {
        let obj_size = object_size.max(core::mem::size_of::<usize>());
        // Align the object size to the required alignment
        let obj_size = (obj_size + align - 1) & !(align - 1);
        let total_bytes = obj_size * capacity;

        let layout = Layout::from_size_align(total_bytes, align).ok()?;
        let pool = alloc::alloc::alloc(layout);
        if pool.is_null() {
            return None;
        }

        // Initialize free list - each free slot stores the index of the next free slot
        for i in 0..capacity {
            let slot_ptr = pool.add(i * obj_size) as *mut usize;
            *slot_ptr = if i + 1 < capacity { i + 1 } else { FREE_END };
        }

        Some(Self {
            object_size: obj_size,
            align,
            total: capacity,
            free_head: AtomicUsize::new(0),
            pool,
            allocated: AtomicUsize::new(0),
            name,
        })
    }

    /// Allocate one object from the slab cache.
    /// Returns None if the cache is full.
    /// Like Linux's kmem_cache_alloc().
    pub fn alloc(&self) -> Option<NonNull<u8>> {
        loop {
            let head = self.free_head.load(Ordering::Acquire);
            if head == FREE_END {
                return None; // Cache exhausted
            }

            // Read the next free index from the free object
            let slot_ptr = unsafe { self.pool.add(head * self.object_size) as *mut usize };
            let next = unsafe { *slot_ptr };

            // CAS to claim this slot
            match self.free_head.compare_exchange_weak(
                head, next, Ordering::AcqRel, Ordering::Relaxed
            ) {
                Ok(_) => {
                    self.allocated.fetch_add(1, Ordering::Relaxed);
                    let ptr = unsafe { NonNull::new_unchecked(self.pool.add(head * self.object_size)) };
                    // Zero-initialize (like Linux's kmem_cache_zalloc)
                    unsafe { core::ptr::write_bytes(ptr.as_ptr(), 0, self.object_size); }
                    return Some(ptr);
                }
                Err(_) => continue, // Retry on contention
            }
        }
    }

    /// Free an object back to the slab cache.
    /// Like Linux's kmem_cache_free().
    /// 
    /// # Safety
    /// `ptr` must have been allocated from this cache and not yet freed.
    pub unsafe fn free(&self, ptr: NonNull<u8>) {
        let offset = ptr.as_ptr().offset_from(self.pool) as usize;
        let index = offset / self.object_size;
        debug_assert!(index < self.total, "Invalid pointer freed to slab cache");

        loop {
            let head = self.free_head.load(Ordering::Acquire);
            // Store next-free index in the freed slot
            *(ptr.as_ptr() as *mut usize) = head;
            match self.free_head.compare_exchange_weak(
                head, index, Ordering::AcqRel, Ordering::Relaxed
            ) {
                Ok(_) => {
                    self.allocated.fetch_sub(1, Ordering::Relaxed);
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    /// Returns the number of currently allocated objects.
    pub fn allocated(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }

    /// Returns the total capacity.
    pub fn capacity(&self) -> usize {
        self.total
    }

    /// Returns the number of free slots.
    pub fn free_slots(&self) -> usize {
        self.total - self.allocated()
    }

    /// Returns the cache name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the object size (aligned).
    pub fn object_size(&self) -> usize {
        self.object_size
    }
}

impl Drop for SlabCache {
    fn drop(&mut self) {
        let total_bytes = self.object_size * self.total;
        let layout = unsafe {
            Layout::from_size_align_unchecked(total_bytes, self.align)
        };
        unsafe {
            alloc::alloc::dealloc(self.pool, layout);
        }
    }
}

/// A typed slab cache for allocating objects of type T.
/// Provides a safe, typed wrapper over SlabCache.
pub struct TypedSlabCache<T> {
    inner: SlabCache,
    _marker: core::marker::PhantomData<T>,
}

impl<T> TypedSlabCache<T> {
    /// Create a new typed slab cache.
    pub fn new(capacity: usize, name: &'static str) -> Option<Self> {
        let inner = unsafe {
            SlabCache::new(
                core::mem::size_of::<T>(),
                core::mem::align_of::<T>(),
                capacity,
                name,
            )?
        };
        Some(Self { inner, _marker: core::marker::PhantomData })
    }

    /// Allocate and initialize an object.
    pub fn alloc_with(&self, value: T) -> Option<NonNull<T>> {
        let ptr = self.inner.alloc()?;
        let typed_ptr = ptr.as_ptr() as *mut T;
        unsafe { typed_ptr.write(value); }
        Some(unsafe { NonNull::new_unchecked(typed_ptr) })
    }

    /// Free an object back to the cache.
    /// # Safety: ptr must come from this cache's alloc_with and not be freed twice.
    pub unsafe fn free(&self, ptr: NonNull<T>) {
        // Drop the object
        core::ptr::drop_in_place(ptr.as_ptr());
        self.inner.free(ptr.cast());
    }

    pub fn allocated(&self) -> usize { self.inner.allocated() }
    pub fn capacity(&self) -> usize { self.inner.capacity() }
    pub fn name(&self) -> &str { self.inner.name() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_cache_basic() {
        let cache = TypedSlabCache::<u64>::new(16, "test_u64_cache").unwrap();
        assert_eq!(cache.capacity(), 16);
        assert_eq!(cache.allocated(), 0);

        let ptr1 = cache.alloc_with(42u64).unwrap();
        let ptr2 = cache.alloc_with(99u64).unwrap();
        assert_eq!(cache.allocated(), 2);

        unsafe {
            assert_eq!(*ptr1.as_ptr(), 42);
            assert_eq!(*ptr2.as_ptr(), 99);
            cache.free(ptr1);
            cache.free(ptr2);
        }
        assert_eq!(cache.allocated(), 0);
    }

    #[test]
    fn test_slab_cache_exhaustion() {
        let cache = TypedSlabCache::<u32>::new(4, "tiny_cache").unwrap();
        let ptrs: alloc::vec::Vec<_> = (0..4).filter_map(|i| cache.alloc_with(i as u32)).collect();
        assert_eq!(ptrs.len(), 4);
        // Should be full now
        assert!(cache.alloc_with(99u32).is_none());
        // Free one
        unsafe { cache.free(ptrs[0]); }
        // Should work now
        let new_ptr = cache.alloc_with(100u32);
        assert!(new_ptr.is_some());
        unsafe {
            for p in ptrs.into_iter().skip(1) {
                cache.free(p);
            }
            cache.free(new_ptr.unwrap());
        }
    }

    #[test]
    fn test_slab_reuse() {
        let cache = TypedSlabCache::<i32>::new(8, "reuse_test").unwrap();
        let p = cache.alloc_with(777).unwrap();
        unsafe {
            assert_eq!(*p.as_ptr(), 777);
            cache.free(p);
        }
        // Reuse the slot - should be zero-initialized
        let p2 = cache.alloc_with(888).unwrap();
        unsafe {
            assert_eq!(*p2.as_ptr(), 888);
            cache.free(p2);
        }
    }
}
