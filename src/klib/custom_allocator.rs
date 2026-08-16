#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Custom Memory Allocator
// A bump allocator with a simple recycle list, replacing std::alloc::System.
// Designed to minimize dependency on predefined library allocators.

#[allow(dead_code)]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

// ============================================================================
// Constants
// ============================================================================

/// Total heap size managed by the bump allocator (128 MB default).
const HEAP_SIZE: usize = 128 * 1024 * 1024;

/// Maximum number of free-list entries in the recycle bin.
const RECYCLE_BIN_CAPACITY: usize = 256;

/// Minimum alignment for all allocations (matches most hardware requirements).
const MIN_ALIGN: usize = 16;

// ============================================================================
// Heap backing store
// ============================================================================

/// Raw backing heap — statically allocated to avoid OS dependency.
#[repr(align(16))]
struct HeapStorage {
    data: UnsafeCell<[u8; HEAP_SIZE]>,
}

unsafe impl Sync for HeapStorage {}

static HEAP_STORAGE: HeapStorage = HeapStorage {
    data: UnsafeCell::new([0u8; HEAP_SIZE]),
};

// ============================================================================
// Recycle bin — free-list for reuse
// ============================================================================

/// A single entry in the recycle bin.
#[derive(Copy, Clone)]
struct RecycleEntry {
    ptr: *mut u8,
    size: usize,
    align: usize,
}

unsafe impl Send for RecycleEntry {}
unsafe impl Sync for RecycleEntry {}

/// Simple ring-buffer free list. Not thread-safe without a lock; for a kernel
/// you'd protect this with a spinlock.
struct RecycleBin {
    entries: UnsafeCell<[RecycleEntry; RECYCLE_BIN_CAPACITY]>,
    head: AtomicUsize,
    tail: AtomicUsize,
    count: AtomicUsize,
}

unsafe impl Sync for RecycleBin {}

impl RecycleBin {
    const fn new() -> Self {
        let empty = RecycleEntry { ptr: ptr::null_mut(), size: 0, align: 0 };
        RecycleBin {
            entries: UnsafeCell::new([empty; RECYCLE_BIN_CAPACITY]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            count: AtomicUsize::new(0),
        }
    }

    /// Try to insert a freed block into the recycle bin.
    fn insert(&self, ptr: *mut u8, size: usize, align: usize) -> bool {
        if self.count.load(Ordering::Acquire) >= RECYCLE_BIN_CAPACITY {
            return false;
        }
        let tail = self.tail.load(Ordering::Acquire);
        let next = (tail + 1) % RECYCLE_BIN_CAPACITY;
        unsafe {
            (*self.entries.get())[tail] = RecycleEntry { ptr, size, align };
        }
        self.tail.store(next, Ordering::Release);
        self.count.fetch_add(1, Ordering::AcqRel);
        true
    }

    /// Try to find a recycled block that satisfies the requested layout.
    fn take(&self, layout: Layout) -> Option<*mut u8> {
        let count = self.count.load(Ordering::Acquire);
        if count == 0 {
            return None;
        }
        // Linear scan through the live entries
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        let mut idx = head;
        let entries = unsafe { &mut *self.entries.get() };
        let live = if tail >= head { tail - head } else { RECYCLE_BIN_CAPACITY - head + tail };
        for _ in 0..live {
            let entry = entries[idx];
            if entry.size >= layout.size() && entry.align >= layout.align() {
                // Remove by swapping with head entry
                let head_entry = entries[head];
                entries[idx] = head_entry;
                let new_head = (head + 1) % RECYCLE_BIN_CAPACITY;
                self.head.store(new_head, Ordering::Release);
                self.count.fetch_sub(1, Ordering::AcqRel);
                return Some(entry.ptr);
            }
            idx = (idx + 1) % RECYCLE_BIN_CAPACITY;
        }
        None
    }
}

// ============================================================================
// SigmaBumpAllocator
// ============================================================================

/// A bump-pointer allocator with a recycle bin.
///
/// Allocation strategy:
/// 1. Check the recycle bin for a suitable freed block.
/// 2. If not found, advance the bump pointer.
/// 3. If the heap is exhausted, return null.
///
/// Deallocation strategy:
/// 1. Insert the block into the recycle bin.
/// 2. If the bin is full, the memory is lost (acceptable for early-stage OS).
pub struct SigmaBumpAllocator {
    /// Next free position in the heap (byte offset from HEAP_STORAGE.data).
    bump: AtomicUsize,
    /// Recycle bin for freed blocks.
    recycle: RecycleBin,
    /// How many allocations are currently live.
    live_count: AtomicUsize,
    /// Total bytes allocated (for diagnostics).
    total_allocated: AtomicUsize,
    /// Total bytes deallocated (for diagnostics).
    total_deallocated: AtomicUsize,
    /// Pseudo-random seed for ASLR guard offset
    random_seed: AtomicUsize,
}

impl SigmaBumpAllocator {
    pub const fn new() -> Self {
        SigmaBumpAllocator {
            bump: AtomicUsize::new(0),
            recycle: RecycleBin::new(),
            live_count: AtomicUsize::new(0),
            total_allocated: AtomicUsize::new(0),
            total_deallocated: AtomicUsize::new(0),
            random_seed: AtomicUsize::new(0x1337_55AA),
        }
    }

    /// Pseudo-random LCG generator for ASLR heap layout randomization
    fn next_random(&self) -> usize {
        let current = self.random_seed.load(Ordering::Relaxed);
        let next = current.wrapping_mul(1103515245).wrapping_add(12345);
        self.random_seed.store(next, Ordering::Relaxed);
        next
    }

    /// Allocates memory with randomized ASLR guard padding (OpenBSD/Hardened Malloc inspired)
    pub unsafe fn alloc_randomized(&self, layout: Layout) -> *mut u8 {
        let align = layout.align().max(MIN_ALIGN);
        let size = layout.size();

        // Generate pseudo-random guard offset (0 to 112 bytes in multiples of MIN_ALIGN)
        let guard_offset = (self.next_random() % 8) * MIN_ALIGN;
        let total_size = size + guard_offset;

        let adjusted_layout = Layout::from_size_align(total_size, align).unwrap_or(layout);
        let ptr = self.alloc(adjusted_layout);
        if !ptr.is_null() && guard_offset > 0 {
            // Return pointer offset by randomized guard padding
            ptr.add(guard_offset)
        } else {
            ptr
        }
    }

    /// Return current heap usage statistics.
    pub fn stats(&self) -> AllocStats {
        AllocStats {
            heap_size: HEAP_SIZE,
            bump_offset: self.bump.load(Ordering::Relaxed),
            live_allocations: self.live_count.load(Ordering::Relaxed),
            total_allocated: self.total_allocated.load(Ordering::Relaxed),
            total_deallocated: self.total_deallocated.load(Ordering::Relaxed),
            recycle_bin_entries: self.recycle.count.load(Ordering::Relaxed),
        }
    }

    /// Attempt to reset the bump pointer (only safe when all allocations are freed).
    pub unsafe fn reset(&self) {
        self.bump.store(0, Ordering::SeqCst);
        self.live_count.store(0, Ordering::SeqCst);
    }
}

unsafe impl GlobalAlloc for SigmaBumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Enforce minimum alignment
        let align = layout.align().max(MIN_ALIGN);
        let size = layout.size();

        // Step 1: check recycle bin
        let adjusted_layout = Layout::from_size_align(size, align).unwrap_or(layout);
        if let Some(ptr) = self.recycle.take(adjusted_layout) {
            self.live_count.fetch_add(1, Ordering::Relaxed);
            self.total_allocated.fetch_add(size, Ordering::Relaxed);
            return ptr;
        }

        // Step 2: bump allocation
        let heap_start = (*HEAP_STORAGE.data.get()).as_mut_ptr();

        // Atomically reserve space
        loop {
            let current = self.bump.load(Ordering::Acquire);
            // Align up current offset
            let aligned = (current + align - 1) & !(align - 1);
            let end = aligned.checked_add(size).unwrap_or(usize::MAX);

            if end > HEAP_SIZE {
                // Out of memory
                return ptr::null_mut();
            }

            if self.bump.compare_exchange(current, end, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                let ptr = heap_start.add(aligned);
                self.live_count.fetch_add(1, Ordering::Relaxed);
                self.total_allocated.fetch_add(size, Ordering::Relaxed);
                return ptr;
            }
            // CAS failed: another thread advanced the bump; retry
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let align = layout.align().max(MIN_ALIGN);
        let size = layout.size();

        // Try to insert into the recycle bin
        self.recycle.insert(ptr, size, align);
        self.live_count.fetch_sub(1, Ordering::Relaxed);
        self.total_deallocated.fetch_add(size, Ordering::Relaxed);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
            ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_layout = match Layout::from_size_align(new_size, layout.align()) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };

        // Allocate new block
        let new_ptr = self.alloc(new_layout);
        if !new_ptr.is_null() {
            // Copy old data to new block
            let copy_size = layout.size().min(new_size);
            ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
            // Dealloc old block
            self.dealloc(ptr, layout);
        }
        new_ptr
    }
}

// ============================================================================
// Diagnostic types
// ============================================================================

/// Heap usage statistics.
pub struct AllocStats {
    pub heap_size: usize,
    pub bump_offset: usize,
    pub live_allocations: usize,
    pub total_allocated: usize,
    pub total_deallocated: usize,
    pub recycle_bin_entries: usize,
}

impl AllocStats {
    pub fn free_bytes(&self) -> usize {
        self.heap_size.saturating_sub(self.bump_offset)
    }

    pub fn utilization_percent(&self) -> u8 {
        if self.heap_size == 0 {
            return 0;
        }
        ((self.bump_offset * 100) / self.heap_size) as u8
    }
}

// ============================================================================
// Global allocator registration
// ============================================================================

/// The global instance of the SigmaOS bump allocator.
#[cfg(target_os = "none")]
#[global_allocator]
pub static SIGMA_ALLOCATOR: SigmaBumpAllocator = SigmaBumpAllocator::new();

/// The global instance of the SigmaOS bump allocator (not registered as global on host targets).
#[cfg(not(target_os = "none"))]
pub static SIGMA_ALLOCATOR: SigmaBumpAllocator = SigmaBumpAllocator::new();

// ============================================================================
// Out-of-memory handler
// ============================================================================

/// Called by the Rust runtime when allocation fails (requires nightly/alloc_error_handler).
#[cfg(feature = "custom_alloc_error_handler")]
#[alloc_error_handler]
fn sigma_oom(layout: Layout) -> ! {
    // In a real kernel this would trigger a kernel panic with diagnostics.
    let _ = layout;
    loop {
        core::hint::spin_loop();
    }
}

// ============================================================================
// Unit tests (no_std compatible via test harness)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::alloc::Layout;

    #[test]
    fn test_basic_alloc_dealloc() {
        let layout = Layout::from_size_align(64, 8).unwrap();
        let ptr = unsafe { SIGMA_ALLOCATOR.alloc(layout) };
        assert!(!ptr.is_null());
        unsafe { SIGMA_ALLOCATOR.dealloc(ptr, layout) };
    }

    #[test]
    fn test_zeroed_alloc() {
        let layout = Layout::from_size_align(128, 16).unwrap();
        let ptr = unsafe { SIGMA_ALLOCATOR.alloc_zeroed(layout) };
        assert!(!ptr.is_null());
        // All bytes should be zero
        unsafe {
            for i in 0..128 {
                assert_eq!(*ptr.add(i), 0);
            }
            SIGMA_ALLOCATOR.dealloc(ptr, layout);
        }
    }

    #[test]
    fn test_recycle_bin_reuse() {
        let layout = Layout::from_size_align(256, 16).unwrap();
        let ptr1 = unsafe { SIGMA_ALLOCATOR.alloc(layout) };
        assert!(!ptr1.is_null());
        unsafe { SIGMA_ALLOCATOR.dealloc(ptr1, layout) };

        // Next allocation of same size may come from recycle bin
        let ptr2 = unsafe { SIGMA_ALLOCATOR.alloc(layout) };
        assert!(!ptr2.is_null());
        unsafe { SIGMA_ALLOCATOR.dealloc(ptr2, layout) };
    }

    #[test]
    fn test_stats() {
        let stats = SIGMA_ALLOCATOR.stats();
        assert_eq!(stats.heap_size, HEAP_SIZE);
        assert!(stats.bump_offset <= HEAP_SIZE);
    }

    #[test]
    fn test_randomized_malloc_aslr_guard() {
        let layout = Layout::from_size_align(64, 16).unwrap();
        let ptr1 = unsafe { SIGMA_ALLOCATOR.alloc_randomized(layout) };
        let ptr2 = unsafe { SIGMA_ALLOCATOR.alloc_randomized(layout) };
        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert_ne!(ptr1, ptr2);
        unsafe {
            SIGMA_ALLOCATOR.dealloc(ptr1, layout);
            SIGMA_ALLOCATOR.dealloc(ptr2, layout);
        }
    }
}
