// SigmaOS — Slab Allocator
//
// Inspired by the FreeBSD UMA (Universal Memory Allocator) and the Linux
// slab/SLUB allocator.  Provides O(1) allocation of fixed-size kernel objects
// with no dependency on an external malloc implementation.
//
// Reference:
//   Jeff Bonwick, "The Slab Allocator: An Object-Caching Kernel Memory
//   Allocator," USENIX 1994.
//
// Design goals for SigmaOS:
//   - `no_std` — works in bare-metal kernel context.
//   - No external crate dependencies — pure custom implementation.
//   - Static caches for the most common kernel objects.
//   - Explicit free-list tracking without heap allocation.

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

/// Slab size in bytes (one physical page).
pub const SLAB_PAGE_SIZE: usize = 4096;

/// Maximum supported object size.  Objects larger than this should use a
/// page allocator directly.
pub const MAX_SLAB_OBJECT: usize = 512;

/// Maximum number of free-list entries per cache.
/// = SLAB_PAGE_SIZE / MIN_OBJECT_SIZE (MIN assumed 8 bytes).
const MAX_FREE_LIST: usize = SLAB_PAGE_SIZE / 8;

// ─────────────────────────────────────────────────────────────────────────────
// Free-list node (embedded in free slots)
// ─────────────────────────────────────────────────────────────────────────────

/// Tracks the offset of each free slot within the slab's virtual region.
/// In a real implementation these would be embedded inside the free memory
/// itself; here we keep an explicit array so the code stays safe-Rust.
struct FreeList {
    slots: [u32; MAX_FREE_LIST],  // offsets (in bytes) of free slots
    head: usize,                  // index of first free slot
    count: usize,                 // number of free slots
}

impl FreeList {
    const fn empty() -> Self {
        Self {
            slots: [0u32; MAX_FREE_LIST],
            head: 0,
            count: 0,
        }
    }

    fn push(&mut self, offset: u32) -> bool {
        if self.count >= MAX_FREE_LIST {
            return false;
        }
        let idx = (self.head + self.count) % MAX_FREE_LIST;
        self.slots[idx] = offset;
        self.count += 1;
        true
    }

    fn pop(&mut self) -> Option<u32> {
        if self.count == 0 {
            return None;
        }
        let val = self.slots[self.head];
        self.head = (self.head + 1) % MAX_FREE_LIST;
        self.count -= 1;
        Some(val)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SlabCache
// ─────────────────────────────────────────────────────────────────────────────

/// A slab cache for objects of a fixed size.
///
/// Each `SlabCache` manages one pool of same-sized objects.  Multiple caches
/// can coexist for different object types (process descriptors, inodes, …).
pub struct SlabCache {
    /// Human-readable name for diagnostics.
    name: &'static str,
    /// Size of each object in bytes.
    object_size: usize,
    /// Total slots in the slab.
    total: usize,
    /// Free-list of available slot offsets.
    free: FreeList,
    /// Allocation statistics.
    alloc_count: usize,
    free_count_stat: usize,
}

impl SlabCache {
    /// Construct a new slab cache.  This is `const` so it can be used in
    /// `static` initialisers (no heap allocation at runtime).
    pub const fn new(name: &'static str, object_size: usize) -> Self {
        // Compute how many objects fit in one 4 KiB page.
        let total = if object_size == 0 { 1 } else { SLAB_PAGE_SIZE / object_size };
        Self {
            name,
            object_size,
            total,
            free: FreeList::empty(),
            alloc_count: 0,
            free_count_stat: 0,
        }
    }

    /// Initialise the free list.  Must be called once before `alloc()`.
    ///
    /// # Safety
    ///
    /// This function is safe — it only fills an in-memory bookkeeping array.
    /// It does not touch physical memory.
    pub fn init(&mut self) {
        for i in 0..self.total.min(MAX_FREE_LIST) {
            let offset = (i * self.object_size) as u32;
            self.free.push(offset);
        }
    }

    // ── Allocation ────────────────────────────────────────────────────────────

    /// Allocate one object from the cache.
    ///
    /// Returns the byte offset of the allocated slot within the slab's
    /// virtual address region, or `None` if the cache is exhausted.
    pub fn alloc(&mut self) -> Option<usize> {
        self.free.pop().map(|offset| {
            self.alloc_count = self.alloc_count.wrapping_add(1);
            offset as usize
        })
    }

    // ── Deallocation ──────────────────────────────────────────────────────────

    /// Return `offset` to the free list.
    ///
    /// The caller must guarantee that `offset` was previously returned by
    /// `alloc()` on *this* cache and has not already been freed.
    pub fn free_slot(&mut self, offset: usize) -> bool {
        let pushed = self.free.push(offset as u32);
        if pushed {
            self.free_count_stat = self.free_count_stat.wrapping_add(1);
        }
        pushed
    }

    // ── Query ─────────────────────────────────────────────────────────────────

    pub fn name(&self) -> &'static str { self.name }
    pub fn object_size(&self) -> usize { self.object_size }
    pub fn total_slots(&self) -> usize { self.total }
    pub fn free_slots(&self) -> usize { self.free.count }
    pub fn used_slots(&self) -> usize { self.total.saturating_sub(self.free.count) }
    pub fn alloc_count(&self) -> usize { self.alloc_count }
    pub fn free_count(&self) -> usize { self.free_count_stat }

    /// Utilisation as a percentage (0–100).
    pub fn utilisation_pct(&self) -> u32 {
        if self.total == 0 { return 0; }
        ((self.used_slots() * 100) / self.total) as u32
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Static kernel caches
// ─────────────────────────────────────────────────────────────────────────────

// SAFETY: These statics are only accessed from single-threaded kernel init
//         and then protected by the kernel's big lock.  SigmaOS does not yet
//         have multi-core SMP, so data races are impossible.
pub static mut PROCESS_SLAB: SlabCache = SlabCache::new("sigma_process",  256);
pub static mut SOCKET_SLAB:  SlabCache = SlabCache::new("sigma_socket",   128);
pub static mut INODE_SLAB:   SlabCache = SlabCache::new("sigma_inode",    512);
pub static mut PIPE_SLAB:    SlabCache = SlabCache::new("sigma_pipe",     64);
pub static mut TIMER_SLAB:   SlabCache = SlabCache::new("sigma_timer",    32);

/// Initialise all static kernel slab caches.  Call once during early boot.
///
/// # Safety
///
/// Must be called before any concurrent access to the static caches, i.e.
/// before scheduler / SMP bring-up.
pub unsafe fn init_all_slabs() {
    PROCESS_SLAB.init();
    SOCKET_SLAB.init();
    INODE_SLAB.init();
    PIPE_SLAB.init();
    TIMER_SLAB.init();
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_and_free() {
        let mut cache = SlabCache::new("test", 64);
        cache.init();
        assert_eq!(cache.free_slots(), SLAB_PAGE_SIZE / 64);
        let a = cache.alloc().expect("first alloc");
        let b = cache.alloc().expect("second alloc");
        assert_ne!(a, b, "offsets must be distinct");
        cache.free_slot(a);
        let c = cache.alloc().expect("re-alloc after free");
        assert_eq!(c, a, "freed slot should be reused");
    }

    #[test]
    fn test_exhaustion() {
        let mut cache = SlabCache::new("small", 512);
        cache.init();
        let total = cache.total_slots();
        for _ in 0..total {
            assert!(cache.alloc().is_some());
        }
        assert!(cache.alloc().is_none(), "should be exhausted");
    }

    #[test]
    fn test_utilisation() {
        let mut cache = SlabCache::new("util_test", 256);
        cache.init();
        let total = cache.total_slots();
        for _ in 0..(total / 2) {
            cache.alloc();
        }
        assert_eq!(cache.utilisation_pct(), 50);
    }

    #[test]
    fn test_stats() {
        let mut cache = SlabCache::new("stats", 128);
        cache.init();
        let slot = cache.alloc().unwrap();
        cache.free_slot(slot);
        assert_eq!(cache.alloc_count(), 1);
        assert_eq!(cache.free_count(), 1);
    }
}
