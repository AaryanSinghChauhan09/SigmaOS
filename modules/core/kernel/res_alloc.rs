/// SigmaOS — modules/core/kernel/res_alloc.rs
/// Sovereign Physical Page Allocator: bitmap + buddy zones.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU64   = u64;
type SigmaU32   = u32;
type SigmaUsize = usize;
type SigmaI32   = i32;
type SigmaBool  = bool;

// ─── Configuration ────────────────────────────────────────────────────────────

/// Page size: 4 KiB
pub const PAGE_SIZE: SigmaUsize = 4096;

/// Page size shift (log2 of PAGE_SIZE)
pub const PAGE_SHIFT: SigmaU32 = 12;

/// Maximum number of physical pages tracked (4 GiB / 4 KiB = 1 M pages)
pub const MAX_PAGES: SigmaUsize = 1_048_576;

/// Number of u64 words in the bitmap
pub const BITMAP_WORDS: SigmaUsize = MAX_PAGES / 64;

/// Maximum buddy order (2^11 = 2048 pages = 8 MiB contiguous)
pub const BUDDY_ORDERS: SigmaUsize = 12;

// ─── Page State Bitmap ────────────────────────────────────────────────────────
// Bit = 0 → page is FREE, bit = 1 → page is ALLOCATED.

static mut PAGE_BITMAP: [SigmaU64; BITMAP_WORDS] = [0u64; BITMAP_WORDS];

// ─── Allocator Stats ─────────────────────────────────────────────────────────

#[repr(C)]
pub struct AllocStats {
    pub total_pages:  SigmaU64,
    pub free_pages:   SigmaU64,
    pub alloc_calls:  SigmaU64,
    pub free_calls:   SigmaU64,
    pub alloc_fails:  SigmaU64,
}

static mut ALLOC_STATS: AllocStats = AllocStats {
    total_pages: 0,
    free_pages:  0,
    alloc_calls: 0,
    free_calls:  0,
    alloc_fails: 0,
};

// ─── Bitmap Helpers ───────────────────────────────────────────────────────────

#[inline]
unsafe fn bitmap_set(pfn: SigmaUsize) {
    PAGE_BITMAP[pfn / 64] |= 1u64 << (pfn % 64);
}

#[inline]
unsafe fn bitmap_clear(pfn: SigmaUsize) {
    PAGE_BITMAP[pfn / 64] &= !(1u64 << (pfn % 64));
}

#[inline]
unsafe fn bitmap_test(pfn: SigmaUsize) -> SigmaBool {
    (PAGE_BITMAP[pfn / 64] >> (pfn % 64)) & 1 == 1
}

// ─── First-Fit Search ────────────────────────────────────────────────────────

/// Find the first run of `count` contiguous free pages.
/// Returns the starting PFN or `usize::MAX` if not found.
unsafe fn find_free_run(count: SigmaUsize) -> SigmaUsize {
    let total = ALLOC_STATS.total_pages as SigmaUsize;
    let mut start = 0usize;
    let mut run   = 0usize;
    let mut pfn   = 0usize;

    while pfn < total {
        if !bitmap_test(pfn) {
            // Free page
            if run == 0 { start = pfn; }
            run += 1;
            if run == count { return start; }
        } else {
            run = 0;
        }
        pfn += 1;
    }
    SigmaUsize::MAX
}

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

/// Initialise the allocator. `mem_start` and `mem_size` define usable RAM.
/// Called once from `kernel_main` after HAL reports the memory map.
#[no_mangle]
pub unsafe extern "C" fn res_alloc_init() -> SigmaI32 {
    // Zero the bitmap — all pages start as FREE
    for word in PAGE_BITMAP.iter_mut() { *word = 0; }

    // Hard-coded: assume 1 GiB for now.
    // In production, fill from E820 / UEFI memory map.
    let total = (1u64 << 30) / PAGE_SIZE as SigmaU64;  // 262144 pages
    ALLOC_STATS.total_pages = total;
    ALLOC_STATS.free_pages  = total;
    ALLOC_STATS.alloc_calls = 0;
    ALLOC_STATS.free_calls  = 0;
    ALLOC_STATS.alloc_fails = 0;

    // Mark the first 4 MiB as reserved (kernel image + stack)
    let reserved_pages = (4 * 1024 * 1024) / PAGE_SIZE;
    for p in 0..reserved_pages {
        bitmap_set(p);
    }
    ALLOC_STATS.free_pages -= reserved_pages as SigmaU64;
    0
}

/// Allocate `2^order` contiguous pages. Returns the physical address or 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_alloc_pages(order: SigmaU32) -> SigmaU64 {
    ALLOC_STATS.alloc_calls = ALLOC_STATS.alloc_calls.wrapping_add(1);

    if order as SigmaUsize >= BUDDY_ORDERS {
        ALLOC_STATS.alloc_fails = ALLOC_STATS.alloc_fails.wrapping_add(1);
        return 0;
    }

    let count = 1usize << order;
    let pfn   = find_free_run(count);

    if pfn == SigmaUsize::MAX {
        ALLOC_STATS.alloc_fails = ALLOC_STATS.alloc_fails.wrapping_add(1);
        return 0;
    }

    // Mark pages allocated
    for i in pfn..(pfn + count) { bitmap_set(i); }
    ALLOC_STATS.free_pages = ALLOC_STATS.free_pages.saturating_sub(count as SigmaU64);

    (pfn * PAGE_SIZE) as SigmaU64
}

/// Free `2^order` contiguous pages starting at physical address `phys`.
#[no_mangle]
pub unsafe extern "C" fn sigma_free_pages(phys: SigmaU64, order: SigmaU32) {
    ALLOC_STATS.free_calls = ALLOC_STATS.free_calls.wrapping_add(1);

    let pfn   = (phys as SigmaUsize) / PAGE_SIZE;
    let count = 1usize << order as SigmaUsize;

    for i in pfn..(pfn + count) {
        bitmap_clear(i);
    }
    ALLOC_STATS.free_pages = ALLOC_STATS.free_pages.saturating_add(count as SigmaU64);
}

/// Query free page count.
#[no_mangle]
pub unsafe extern "C" fn res_alloc_free_pages() -> SigmaU64 {
    ALLOC_STATS.free_pages
}

/// Query total page count.
#[no_mangle]
pub unsafe extern "C" fn res_alloc_total_pages() -> SigmaU64 {
    ALLOC_STATS.total_pages
}

/// Rebalance: compact free runs and merge adjacent free blocks (buddy coalescing).
/// Called by the scheduler when memory pressure is high.
#[no_mangle]
pub unsafe extern "C" fn res_alloc_rebalance() {
    // Phase 2: implement buddy coalescing.
    // For now, just recalculate free_pages from the bitmap.
    let total = ALLOC_STATS.total_pages as SigmaUsize;
    let mut free = 0u64;
    for pfn in 0..total {
        if !bitmap_test(pfn) { free += 1; }
    }
    ALLOC_STATS.free_pages = free;
}

/// Read allocator statistics.
#[no_mangle]
pub unsafe extern "C" fn res_alloc_stats() -> *const AllocStats {
    &ALLOC_STATS as *const AllocStats
}
