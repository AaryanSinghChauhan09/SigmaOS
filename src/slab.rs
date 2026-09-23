//! # SigmaOS Slab Allocator
//!
//! A SLUB-inspired slab allocator for fixed-size kernel objects.
//! Inspired by the Linux SLUB allocator (mm/slub.c) and FreeBSD UMA.
//!
//! ## Design Goals
//! - Zero dependency on `std` — uses only `core` and raw pointers
//! - O(1) allocation and deallocation for known object sizes
//! - Per-CPU slab caches to reduce lock contention
//! - Reduced internal fragmentation vs buddy allocator for small objects
//!
//! ## References
//! - Linux SLUB: `mm/slub.c` (Christoph Lameter, 2007)
//! - FreeBSD UMA: `sys/vm/uma_core.c`
//! - NetBSD pool allocator: `sys/kern/subr_pool.c`


use core::ptr::NonNull;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use core::mem;

/// Maximum object size handled by the slab allocator.
/// Objects larger than this are delegated to the buddy allocator.
pub const SLAB_MAX_OBJECT_SIZE: usize = 4096;

/// Number of objects per slab page (4 KiB page, minimum 8 objects).
pub const MIN_OBJECTS_PER_SLAB: usize = 8;

/// Slab state flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlabState {
    /// All objects free
    Empty,
    /// Some objects allocated
    Partial,
    /// All objects allocated
    Full,
}

/// A single slab — one or more pages holding fixed-size objects.
/// The freelist is embedded directly into free objects (like Linux SLUB).
#[repr(C)]
pub struct Slab {
    /// Base address of the slab's memory region
    pub base: usize,
    /// Size of each object in bytes (rounded to alignment)
    pub object_size: usize,
    /// Total number of objects in this slab
    pub total_objects: usize,
    /// Number of free objects
    pub free_count: AtomicUsize,
    /// Head of the embedded free list (offset within slab)
    pub freelist: AtomicPtr<u8>,
    /// Link to next slab in list
    pub next: AtomicPtr<Slab>,
}

impl Slab {
    /// Initialize a new slab over a pre-allocated memory region.
    ///
    /// # Safety
    /// `base` must point to at least `page_size` bytes of writable memory,
    /// aligned to `align_of::<usize>()`. The memory must not be aliased.
    pub unsafe fn init(base: *mut u8, page_size: usize, object_size: usize) -> &'static mut Self {
        // Embed the Slab header at the start of the region
        let slab = &mut *(base as *mut Slab);

        let header_size = Self::aligned_size(mem::size_of::<Slab>(), mem::align_of::<usize>());
        let usable = page_size.saturating_sub(header_size);
        let obj_size = core::cmp::max(object_size, mem::size_of::<*mut u8>());
        let obj_size = Self::aligned_size(obj_size, mem::align_of::<usize>());
        let n_objects = usable / obj_size;

        slab.base = base as usize;
        slab.object_size = obj_size;
        slab.total_objects = n_objects;
        slab.free_count = AtomicUsize::new(n_objects);
        slab.freelist = AtomicPtr::new(core::ptr::null_mut());
        slab.next = AtomicPtr::new(core::ptr::null_mut());

        // Build embedded freelist: each free slot holds a pointer to the next free slot
        let data_start = base.add(header_size);
        let mut prev: *mut u8 = core::ptr::null_mut();
        // Walk backwards to build freelist
        let mut i = n_objects;
        while i > 0 {
            i -= 1;
            let slot = data_start.add(i * obj_size);
            // Write pointer to previous free slot into this slot
            *(slot as *mut *mut u8) = prev;
            prev = slot;
        }
        slab.freelist.store(prev, Ordering::Release);

        slab
    }

    /// Allocate one object from this slab.
    /// Returns `None` if slab is full.
    ///
    /// # Safety
    /// The caller must ensure no concurrent unsynchronized access.
    pub unsafe fn allocate(&self) -> Option<NonNull<u8>> {
        // Pop from freelist
        let ptr = self.freelist.load(Ordering::Acquire);
        if ptr.is_null() {
            return None;
        }
        // Read the next pointer stored in the free slot
        let next = *(ptr as *const *mut u8);
        self.freelist.store(next, Ordering::Release);
        // Zero the object before returning (security: prevent info leak)
        core::ptr::write_bytes(ptr, 0, self.object_size);
        self.free_count.fetch_sub(1, Ordering::Relaxed);
        Some(NonNull::new_unchecked(ptr))
    }

    /// Deallocate one object back to the slab.
    ///
    /// # Safety
    /// `ptr` must be a pointer previously returned by `allocate()` from this slab.
    /// The object must not be used after calling this function.
    pub unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        let p = ptr.as_ptr();
        debug_assert!(self.owns(p), "ptr does not belong to this slab");
        // Push onto freelist
        let old_head = self.freelist.load(Ordering::Acquire);
        *(p as *mut *mut u8) = old_head;
        self.freelist.store(p, Ordering::Release);
        self.free_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns true if `ptr` falls within this slab's memory range.
    pub fn owns(&self, ptr: *const u8) -> bool {
        let addr = ptr as usize;
        let header_size = Self::aligned_size(mem::size_of::<Slab>(), mem::align_of::<usize>());
        let start = self.base.saturating_add(header_size);
        let end = self.base.saturating_add(self.total_objects.saturating_mul(self.object_size).saturating_add(header_size));
        addr >= start && addr < end
    }

    /// Returns the current state of this slab.
    pub fn state(&self) -> SlabState {
        let free = self.free_count.load(Ordering::Relaxed);
        if free == 0 {
            SlabState::Full
        } else if free == self.total_objects {
            SlabState::Empty
        } else {
            SlabState::Partial
        }
    }

    /// Align `size` up to the given `align` (must be power of two).
    #[inline]
    pub const fn aligned_size(size: usize, align: usize) -> usize {
        (size.wrapping_add(align).wrapping_sub(1)) & !(align.wrapping_sub(1))
    }
}

/// A slab cache managing slabs of a fixed object size.
/// Inspired by Linux `kmem_cache` and FreeBSD `uma_zone`.
pub struct SlabCache {
    pub name: &'static str,
    pub object_size: usize,
    /// Linked list of partial slabs (head)
    pub partial: AtomicPtr<Slab>,
    /// Total allocations served
    pub alloc_count: AtomicUsize,
    /// Total deallocations
    pub free_count: AtomicUsize,
}

impl SlabCache {
    /// Create a new slab cache for objects of `object_size` bytes.
    pub const fn new(name: &'static str, object_size: usize) -> Self {
        Self {
            name,
            object_size,
            partial: AtomicPtr::new(core::ptr::null_mut()),
            alloc_count: AtomicUsize::new(0),
            free_count: AtomicUsize::new(0),
        }
    }

    /// Allocate one object. Returns `None` if no slab memory is available.
    ///
    /// # Safety
    /// A page-allocator callback must have pre-populated partial slabs,
    /// or the caller must call `add_slab()` first.
    pub unsafe fn alloc(&self) -> Option<NonNull<u8>> {
        let mut slab_ptr = self.partial.load(Ordering::Acquire);
        while !slab_ptr.is_null() {
            let slab = &*slab_ptr;
            if let Some(obj) = slab.allocate() {
                self.alloc_count.fetch_add(1, Ordering::Relaxed);
                return Some(obj);
            }
            // Move to next slab
            slab_ptr = slab.next.load(Ordering::Acquire);
        }
        None
    }

    /// Return an object to the cache.
    ///
    /// # Safety
    /// `ptr` must have been allocated by this cache.
    pub unsafe fn dealloc(&self, ptr: NonNull<u8>) {
        // Find the owning slab
        let mut slab_ptr = self.partial.load(Ordering::Acquire);
        while !slab_ptr.is_null() {
            let slab = &*slab_ptr;
            if slab.owns(ptr.as_ptr()) {
                slab.deallocate(ptr);
                self.free_count.fetch_add(1, Ordering::Relaxed);
                return;
            }
            slab_ptr = slab.next.load(Ordering::Acquire);
        }
        // Should not reach here — mismatched deallocation
        #[cfg(debug_assertions)]
        panic!("SlabCache::dealloc: ptr does not belong to any slab in cache '{}'", self.name);
    }

    /// Add a pre-initialized slab to this cache's partial list.
    ///
    /// # Safety
    /// `slab` must be initialized via `Slab::init()` and not already in another cache.
    pub unsafe fn add_slab(&self, slab: &'static mut Slab) {
        let old_head = self.partial.load(Ordering::Acquire);
        slab.next.store(old_head, Ordering::Relaxed);
        self.partial.store(slab as *mut Slab, Ordering::Release);
    }

    /// Statistics: total outstanding allocations
    pub fn outstanding(&self) -> usize {
        self.alloc_count
            .load(Ordering::Relaxed)
            .saturating_sub(self.free_count.load(Ordering::Relaxed))
    }
}

// SAFETY: SlabCache uses atomic operations for all shared state.
// Raw pointer access is guarded by the callers' safety invariants.
unsafe impl Sync for SlabCache {}
unsafe impl Send for SlabCache {}

/// Global slab caches for common kernel object sizes.
/// Inspired by Linux's `kmalloc` size classes.
pub static SLAB_8:    SlabCache = SlabCache::new("slab-8",    8);
pub static SLAB_16:   SlabCache = SlabCache::new("slab-16",   16);
pub static SLAB_32:   SlabCache = SlabCache::new("slab-32",   32);
pub static SLAB_64:   SlabCache = SlabCache::new("slab-64",   64);
pub static SLAB_128:  SlabCache = SlabCache::new("slab-128",  128);
pub static SLAB_256:  SlabCache = SlabCache::new("slab-256",  256);
pub static SLAB_512:  SlabCache = SlabCache::new("slab-512",  512);
pub static SLAB_1024: SlabCache = SlabCache::new("slab-1024", 1024);
pub static SLAB_2048: SlabCache = SlabCache::new("slab-2048", 2048);
pub static SLAB_4096: SlabCache = SlabCache::new("slab-4096", 4096);

/// Select the appropriate slab cache for a given size.
/// Returns `None` if size exceeds `SLAB_MAX_OBJECT_SIZE`.
pub fn cache_for_size(size: usize) -> Option<&'static SlabCache> {
    match size {
        0..=8    => Some(&SLAB_8),
        9..=16   => Some(&SLAB_16),
        17..=32  => Some(&SLAB_32),
        33..=64  => Some(&SLAB_64),
        65..=128 => Some(&SLAB_128),
        129..=256  => Some(&SLAB_256),
        257..=512  => Some(&SLAB_512),
        513..=1024 => Some(&SLAB_1024),
        1025..=2048 => Some(&SLAB_2048),
        2049..=4096 => Some(&SLAB_4096),
        _ => None,
    }
}
