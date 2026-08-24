//! # SigmaOS Buddy Allocator
//!
//! A binary buddy allocator for physical memory frame management.
//! Inspired by Linux's `mm/page_alloc.c` buddy system and FreeBSD's VM subsystem.
//!
//! ## Design
//! - Tracks free blocks of sizes 2^0 through 2^MAX_ORDER pages
//! - O(log n) allocation and deallocation
//! - Coalescing: freed blocks are merged with their buddy when both are free
//! - No `std` dependency — all state held in static bitmaps and free lists
//!
//! ## References
//! - Linux buddy allocator: Knuth Vol. 1, §2.5
//! - FreeBSD `vm_phys_alloc_pages`
//! - NetBSD `uvm_pglistalloc`

// #![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

/// Maximum order (2^MAX_ORDER pages per block = 4MB with 4KB pages)
pub const MAX_ORDER: usize = 10;
/// Page size in bytes
pub const PAGE_SIZE: usize = 4096;

/// A physical frame number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameNum(pub usize);

impl FrameNum {
    /// Convert frame number to physical address
    #[inline]
    pub const fn to_phys(self) -> usize {
        self.0.wrapping_mul(PAGE_SIZE)
    }

    /// Convert physical address to frame number
    #[inline]
    pub const fn from_phys(addr: usize) -> Self {
        Self(addr / PAGE_SIZE)
    }

    /// Compute the buddy frame number for a block of 2^order pages
    #[inline]
    pub const fn buddy(self, order: usize) -> Self {
        Self(self.0 ^ (1 << order))
    }

    /// Check if this frame is aligned to 2^order
    #[inline]
    pub const fn is_aligned(self, order: usize) -> bool {
        self.0 & ((1 << order).wrapping_sub(1)) == 0
    }
}

/// Bitmap tracking free status for up to 2^16 = 65536 frames per order
/// Each bit = one block at that order. 1 = free, 0 = allocated.
struct OrderBitmap {
    // 1024 u64s = 65536 bits = 65536 frames max per order
    bits: [AtomicU64; 1024],
}

impl OrderBitmap {
    const fn new() -> Self {
        // SAFETY: AtomicU64 has the same in-memory representation as u64,
        // and 0 is a valid value (all bits allocated = safe initial state).
        Self {
            bits: [const { AtomicU64::new(0) }; 1024],
        }
    }

    /// Set the bit for frame `idx` (mark as free)
    fn set(&self, idx: usize) {
        let word = idx / 64;
        let bit = idx % 64;
        if word < 1024 {
            self.bits[word].fetch_or(1u64 << bit, Ordering::Relaxed);
        }
    }

    /// Clear the bit for frame `idx` (mark as allocated)
    fn clear(&self, idx: usize) {
        let word = idx / 64;
        let bit = idx % 64;
        if word < 1024 {
            self.bits[word].fetch_and(!(1u64 << bit), Ordering::Relaxed);
        }
    }

    /// Test the bit for frame `idx`
    fn test(&self, idx: usize) -> bool {
        let word = idx / 64;
        let bit = idx % 64;
        if word < 1024 {
            (self.bits[word].load(Ordering::Relaxed) >> bit) & 1 == 1
        } else {
            false
        }
    }

    /// Toggle the bit (XOR) — used for buddy coalescing
    fn toggle(&self, idx: usize) {
        let word = idx / 64;
        let bit = idx % 64;
        if word < 1024 {
            self.bits[word].fetch_xor(1u64 << bit, Ordering::Relaxed);
        }
    }
}

/// Free list for one order — a simple stack of free block frame numbers.
/// Holds up to 1024 blocks per order. For a real OS this would be larger.
struct FreeList {
    frames: [AtomicU64; 1024],
    top: AtomicU64,
}

impl FreeList {
    const fn new() -> Self {
        Self {
            frames: [const { AtomicU64::new(u64::MAX) }; 1024],
            top: AtomicU64::new(0),
        }
    }

    /// Push a frame onto the free list. Returns false if full.
    fn push(&self, frame: FrameNum) -> bool {
        let idx = self.top.load(Ordering::Acquire);
        if idx >= 1024 {
            return false;
        }
        self.frames[idx as usize].store(frame.0 as u64, Ordering::Release);
        self.top.fetch_add(1, Ordering::Release);
        true
    }

    /// Pop a frame from the free list. Returns `None` if empty.
    fn pop(&self) -> Option<FrameNum> {
        let idx = self.top.load(Ordering::Acquire);
        if idx == 0 {
            return None;
        }
        let new_top = idx.saturating_sub(1);
        self.top.store(new_top, Ordering::Release);
        let f = self.frames[new_top as usize].load(Ordering::Acquire);
        if f == u64::MAX {
            None
        } else {
            Some(FrameNum(f as usize))
        }
    }

    /// Remove a specific frame from the free list (O(n) — used during coalescing)
    fn remove(&self, target: FrameNum) -> bool {
        let top = self.top.load(Ordering::Acquire) as usize;
        for i in 0..top {
            let f = self.frames[i].load(Ordering::Relaxed);
            if f == target.0 as u64 {
                // Swap with last element
                let last_idx = top.saturating_sub(1);
                let last = self.frames[last_idx].load(Ordering::Relaxed);
                self.frames[i].store(last, Ordering::Relaxed);
                self.frames[last_idx].store(u64::MAX, Ordering::Relaxed);
                self.top.fetch_sub(1, Ordering::Release);
                return true;
            }
        }
        false
    }
}

/// The buddy allocator — one per NUMA node (only one node in this implementation)
pub struct BuddyAllocator {
    /// Free lists indexed by order [0..MAX_ORDER]
    free_lists: [FreeList; MAX_ORDER + 1],
    /// Buddy bitmaps for coalescing detection
    bitmaps: [OrderBitmap; MAX_ORDER + 1],
    /// Total free pages
    free_pages: AtomicU64,
    /// Total managed pages
    total_pages: AtomicU64,
}

impl BuddyAllocator {
    /// Create a new, empty buddy allocator.
    pub const fn new() -> Self {
        Self {
            free_lists: [const { FreeList::new() }; MAX_ORDER + 1],
            bitmaps: [const { OrderBitmap::new() }; MAX_ORDER + 1],
            free_pages: AtomicU64::new(0),
            total_pages: AtomicU64::new(0),
        }
    }

    /// Add a contiguous range of physical frames to this allocator.
    ///
    /// # Safety
    /// The frames `[start, start + count)` must be physically present,
    /// not in use, and not aliased by any other allocator.
    pub unsafe fn add_range(&self, start: FrameNum, count: usize) {
        self.total_pages.fetch_add(count as u64, Ordering::Relaxed);
        // Free each page using the standard free path to trigger coalescing
        for i in 0..count {
            self.free_frame(FrameNum(start.0.wrapping_add(i)), 0);
        }
    }

    /// Allocate `2^order` contiguous pages.
    /// Returns the first frame number, or `None` if out of memory.
    pub fn alloc(&self, order: usize) -> Option<FrameNum> {
        if order > MAX_ORDER {
            return None;
        }
        // Try to allocate from the requested order, splitting larger blocks if needed
        for o in order..=MAX_ORDER {
            if let Some(frame) = self.free_lists[o].pop() {
                self.bitmaps[o].toggle(frame.0 >> o);
                self.free_pages.fetch_sub(1u64 << (o as u64), Ordering::Relaxed);

                // Split block if we got a larger one than needed
                let mut remaining_order = o;
                let mut remaining_frame = frame;
                while remaining_order > order {
                    remaining_order = remaining_order.saturating_sub(1);
                    let buddy = FrameNum(remaining_frame.0.wrapping_add(1 << remaining_order));
                    self.free_lists[remaining_order].push(buddy);
                    self.bitmaps[remaining_order].toggle(buddy.0 >> remaining_order);
                    self.free_pages.fetch_add(1u64 << remaining_order as u64, Ordering::Relaxed);
                }
                return Some(remaining_frame);
            }
        }
        None
    }

    /// Free a block of `2^order` pages starting at `frame`.
    ///
    /// # Safety
    /// `frame` must have been allocated by `alloc(order)` and must not be used after freeing.
    pub unsafe fn free(&self, frame: FrameNum, order: usize) {
        self.free_frame(frame, order);
    }

    fn free_frame(&self, mut frame: FrameNum, mut order: usize) {
        // Coalesce with buddy if possible — walk up the order tree
        while order < MAX_ORDER {
            let buddy = frame.buddy(order);
            let bitmap_idx = frame.0 >> order;
            // Toggle this block's bit in the bitmap
            self.bitmaps[order].toggle(bitmap_idx);
            // If buddy's bit is now 0, buddy was free and we can coalesce
            let buddy_idx = buddy.0 >> order;
            if !self.bitmaps[order].test(buddy_idx) {
                // Buddy is free — remove it from free list and coalesce
                if self.free_lists[order].remove(buddy) {
                    // Merge: use the lower-addressed block as the new block
                    frame = FrameNum(core::cmp::min(frame.0, buddy.0));
                    order = order.saturating_add(1);
                    continue;
                }
            }
            break;
        }
        // Insert the (possibly coalesced) block into the free list
        self.free_lists[order].push(frame);
        self.free_pages.fetch_add(1u64 << order as u64, Ordering::Relaxed);
    }

    /// Returns the number of free pages currently managed.
    pub fn free_pages(&self) -> usize {
        self.free_pages.load(Ordering::Relaxed) as usize
    }

    /// Returns the total number of managed pages.
    pub fn total_pages(&self) -> usize {
        self.total_pages.load(Ordering::Relaxed) as usize
    }

    /// Returns free memory in bytes.
    pub fn free_bytes(&self) -> usize {
        self.free_pages().wrapping_mul(PAGE_SIZE)
    }
}

// SAFETY: BuddyAllocator uses atomics for all shared mutable state.
unsafe impl Sync for BuddyAllocator {}
unsafe impl Send for BuddyAllocator {}

/// Global physical memory allocator
pub static BUDDY: BuddyAllocator = BuddyAllocator::new();
