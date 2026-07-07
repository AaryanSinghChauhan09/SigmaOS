//! sigma_memory — Custom Allocator + Physical Memory Manager
//! Implements a buddy allocator over physical frames (no_std Rust).
//! This is the foundational step toward eliminating all libc malloc/free
//! dependencies from SigmaOS kernel code.

#![no_std]
#![allow(dead_code)]

// ── Constants ─────────────────────────────────────────────────────────────

pub const PAGE_SIZE:       usize = 4096;        // 4 KiB
pub const MAX_ORDER:       usize = 11;           // 2^11 * 4K = 8 MiB max allocation
pub const BITMAP_WORDS:    usize = 1024;         // Tracks up to 65,536 pages (256 MiB)

// ── Physical Frame Bitmap ─────────────────────────────────────────────────

pub struct FrameBitmap {
    bits: [u64; BITMAP_WORDS],
}

impl FrameBitmap {
    pub const fn new() -> Self {
        Self { bits: [0u64; BITMAP_WORDS] }
    }

    #[inline(always)]
    pub fn set_used(&mut self, frame: usize) {
        self.bits[frame / 64] |= 1u64 << (frame % 64);
    }

    #[inline(always)]
    pub fn set_free(&mut self, frame: usize) {
        self.bits[frame / 64] &= !(1u64 << (frame % 64));
    }

    #[inline(always)]
    pub fn is_used(&self, frame: usize) -> bool {
        self.bits[frame / 64] & (1u64 << (frame % 64)) != 0
    }

    /// Find first free frame using a fast u64 leading-zeros scan.
    pub fn find_free(&self) -> Option<usize> {
        for (word_idx, &word) in self.bits.iter().enumerate() {
            if word != u64::MAX {
                let bit = (!word).trailing_zeros() as usize;
                return Some(word_idx * 64 + bit);
            }
        }
        None
    }
}

// ── Buddy Allocator ───────────────────────────────────────────────────────

/// Order-based buddy allocator.  Each order tracks a free-list head (frame index).
pub struct BuddyAllocator {
    pub bitmap:        FrameBitmap,
    pub base_phys:     usize,        // Physical base address of managed memory
    pub total_frames:  usize,
    free_list:         [Option<usize>; MAX_ORDER + 1],
}

impl BuddyAllocator {
    /// Initialise the allocator over a physical memory range.
    pub const fn new(base_phys: usize, total_frames: usize) -> Self {
        Self {
            bitmap:       FrameBitmap::new(),
            base_phys,
            total_frames,
            free_list:    [None; MAX_ORDER + 1],
        }
    }

    /// Allocate 2^order contiguous physical frames.
    /// Returns the starting physical address or None if OOM.
    pub fn alloc(&mut self, order: usize) -> Option<usize> {
        if order > MAX_ORDER { return None; }

        // Try to find a free block at this order or higher
        for current_order in order..=MAX_ORDER {
            if let Some(frame) = self.free_list[current_order] {
                // Remove from free list
                self.free_list[current_order] = None; // simplified single-entry list
                // Mark frames as used
                let count = 1usize << order;
                for i in 0..count {
                    self.bitmap.set_used(frame + i);
                }
                // Split remaining blocks back (buddy splitting)
                let mut remaining_order = current_order;
                let mut remaining_frame = frame;
                while remaining_order > order {
                    remaining_order -= 1;
                    let buddy = remaining_frame + (1usize << remaining_order);
                    self.free_list[remaining_order] = Some(buddy);
                }
                return Some(self.base_phys + frame * PAGE_SIZE);
            }
        }
        None
    }

    /// Free 2^order contiguous physical frames at `phys_addr`.
    pub fn free(&mut self, phys_addr: usize, order: usize) {
        let frame = (phys_addr - self.base_phys) / PAGE_SIZE;
        let count = 1usize << order;
        for i in 0..count {
            self.bitmap.set_free(frame + i);
        }
        // Return to free list (simplified; full impl merges buddies)
        self.free_list[order] = Some(frame);
    }
}

// ── Slab Allocator for small fixed-size objects ───────────────────────────

pub const SLAB_SIZES:  [usize; 5] = [16, 32, 64, 128, 256];
pub const SLAB_SLOTS:  usize      = 64;

pub struct SlabCache {
    pub object_size: usize,
    storage:         [u8; 256 * 64],
    free_mask:       u64,            // Bitmask of free slots (up to 64)
}

impl SlabCache {
    pub const fn new(object_size: usize) -> Self {
        Self {
            object_size,
            storage:   [0u8; 256 * 64],
            free_mask: u64::MAX,     // All slots free
        }
    }

    pub fn alloc_slot(&mut self) -> Option<*mut u8> {
        if self.free_mask == 0 { return None; }
        let slot = self.free_mask.trailing_zeros() as usize;
        self.free_mask &= !(1u64 << slot);
        let offset = slot * self.object_size.min(256);
        unsafe { Some(self.storage.as_mut_ptr().add(offset)) }
    }

    pub fn free_slot(&mut self, ptr: *mut u8) {
        let base = self.storage.as_ptr() as usize;
        let addr  = ptr as usize;
        if addr < base { return; }
        let slot = (addr - base) / self.object_size.min(256);
        if slot < SLAB_SLOTS {
            self.free_mask |= 1u64 << slot;
        }
    }
}
