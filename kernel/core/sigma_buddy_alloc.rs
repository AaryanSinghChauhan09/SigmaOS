//! SigmaOS — Buddy Physical Page Frame Allocator
//! Classic buddy-system allocator for physical pages.
//! No std, no allocator — self-contained with bitmaps.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── Constants ───────────────────────────────────────────────────────────────
const PAGE_SIZE:     Usize = 4096;        // 4 KB pages
const MAX_ORDER:     usize = 11;          // 2^10 = 1024 pages = 4MB max block
const MAX_PAGES:     usize = 65536;       // Track up to 256 MB of RAM
const BITMAP_SIZE:   usize = MAX_PAGES / 8;

// ── Free List Node ──────────────────────────────────────────────────────────
const MAX_FREE_BLOCKS: usize = 1024;

#[derive(Copy, Clone)]
struct FreeBlock {
    base_pfn: U32,  // Page Frame Number
    next: i32,      // Index of next block (-1 = end)
}

impl FreeBlock {
    const fn empty() -> Self {
        FreeBlock { base_pfn: 0, next: -1 }
    }
}

// ── Buddy Allocator State ───────────────────────────────────────────────────
pub struct BuddyAllocator {
    // Free lists per order (head index into free_pool, -1 = empty)
    free_heads:  [i32; MAX_ORDER],
    free_counts: [U32; MAX_ORDER],
    // Pool of FreeBlock nodes
    free_pool:   [FreeBlock; MAX_FREE_BLOCKS],
    pool_next:   usize,  // Next available pool slot
    // Allocation bitmap: 1 = allocated, 0 = free
    bitmap:      [U8; BITMAP_SIZE],
    // Statistics
    total_pages: U32,
    free_pages:  U32,
    alloc_count: U64,
    free_count:  U64,
    base_addr:   U64,    // Physical base address of managed memory
}

static mut BUDDY: BuddyAllocator = BuddyAllocator {
    free_heads:  [-1i32; MAX_ORDER],
    free_counts: [0u32; MAX_ORDER],
    free_pool:   [FreeBlock::empty(); MAX_FREE_BLOCKS],
    pool_next:   0,
    bitmap:      [0u8; BITMAP_SIZE],
    total_pages: 0,
    free_pages:  0,
    alloc_count: 0,
    free_count:  0,
    base_addr:   0,
};

// ── Internal Helpers ────────────────────────────────────────────────────────

unsafe fn bitmap_set(pfn: U32) {
    let idx = pfn as usize / 8;
    let bit = pfn as usize % 8;
    if idx < BITMAP_SIZE {
        BUDDY.bitmap[idx] |= 1 << bit;
    }
}

unsafe fn bitmap_clear(pfn: U32) {
    let idx = pfn as usize / 8;
    let bit = pfn as usize % 8;
    if idx < BITMAP_SIZE {
        BUDDY.bitmap[idx] &= !(1 << bit);
    }
}

unsafe fn bitmap_test(pfn: U32) -> bool {
    let idx = pfn as usize / 8;
    let bit = pfn as usize % 8;
    if idx >= BITMAP_SIZE { return true; }
    (BUDDY.bitmap[idx] & (1 << bit)) != 0
}

unsafe fn alloc_free_block() -> Option<usize> {
    if BUDDY.pool_next >= MAX_FREE_BLOCKS { return None; }
    let idx = BUDDY.pool_next;
    BUDDY.pool_next += 1;
    Some(idx)
}

unsafe fn list_push(order: usize, pfn: U32) {
    if let Some(idx) = alloc_free_block() {
        BUDDY.free_pool[idx].base_pfn = pfn;
        BUDDY.free_pool[idx].next = BUDDY.free_heads[order];
        BUDDY.free_heads[order] = idx as i32;
        BUDDY.free_counts[order] += 1;
    }
}

unsafe fn list_pop(order: usize) -> Option<U32> {
    let head = BUDDY.free_heads[order];
    if head < 0 { return None; }
    let pfn = BUDDY.free_pool[head as usize].base_pfn;
    BUDDY.free_heads[order] = BUDDY.free_pool[head as usize].next;
    BUDDY.free_counts[order] -= 1;
    Some(pfn)
}

unsafe fn list_remove(order: usize, pfn: U32) -> bool {
    let mut prev: i32 = -1;
    let mut cur = BUDDY.free_heads[order];
    while cur >= 0 {
        if BUDDY.free_pool[cur as usize].base_pfn == pfn {
            let nxt = BUDDY.free_pool[cur as usize].next;
            if prev < 0 {
                BUDDY.free_heads[order] = nxt;
            } else {
                BUDDY.free_pool[prev as usize].next = nxt;
            }
            BUDDY.free_counts[order] -= 1;
            return true;
        }
        prev = cur;
        cur = BUDDY.free_pool[cur as usize].next;
    }
    false
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the buddy allocator with a contiguous physical memory region.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_init(base_phys: U64, num_pages: U32) -> i32 {
    if num_pages == 0 || num_pages > MAX_PAGES as U32 {
        return -1;
    }

    BUDDY.base_addr = base_phys;
    BUDDY.total_pages = num_pages;
    BUDDY.free_pages = num_pages;
    BUDDY.alloc_count = 0;
    BUDDY.free_count = 0;
    BUDDY.pool_next = 0;

    // Clear bitmap
    for i in 0..BITMAP_SIZE {
        BUDDY.bitmap[i] = 0;
    }

    // Reset free lists
    for o in 0..MAX_ORDER {
        BUDDY.free_heads[o] = -1;
        BUDDY.free_counts[o] = 0;
    }

    // Add all pages as largest possible blocks
    let mut pfn: U32 = 0;
    while pfn < num_pages {
        // Find the largest order that fits
        let mut order = MAX_ORDER - 1;
        loop {
            let block_size = 1u32 << order;
            if pfn + block_size <= num_pages && (pfn % block_size) == 0 {
                list_push(order, pfn);
                pfn += block_size;
                break;
            }
            if order == 0 {
                list_push(0, pfn);
                pfn += 1;
                break;
            }
            order -= 1;
        }
    }

    0
}

/// Allocate 2^order contiguous pages. Returns PFN or U32::MAX on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_alloc(order: U32) -> U32 {
    let order = order as usize;
    if order >= MAX_ORDER {
        return U32::MAX;
    }

    // Try to find a block at the requested order or higher
    let mut found_order = order;
    while found_order < MAX_ORDER {
        if BUDDY.free_counts[found_order] > 0 {
            break;
        }
        found_order += 1;
    }
    if found_order >= MAX_ORDER {
        return U32::MAX; // Out of memory
    }

    // Pop a block from the found order
    let pfn = match list_pop(found_order) {
        Some(p) => p,
        None => return U32::MAX,
    };

    // Split down to requested order
    while found_order > order {
        found_order -= 1;
        let buddy_pfn = pfn + (1u32 << found_order);
        list_push(found_order, buddy_pfn);
    }

    // Mark pages as allocated
    let pages = 1u32 << order;
    for i in 0..pages {
        bitmap_set(pfn + i);
    }
    BUDDY.free_pages -= pages;
    BUDDY.alloc_count += 1;

    pfn
}

/// Free 2^order contiguous pages starting at the given PFN.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_free(pfn: U32, order: U32) {
    let mut pfn = pfn;
    let mut order = order as usize;

    if order >= MAX_ORDER { return; }

    // Clear bitmap
    let pages = 1u32 << order;
    for i in 0..pages {
        bitmap_clear(pfn + i);
    }
    BUDDY.free_pages += pages;
    BUDDY.free_count += 1;

    // Coalesce with buddies
    while order < MAX_ORDER - 1 {
        let buddy_pfn = pfn ^ (1u32 << order);
        // Check if buddy is free (not allocated)
        let buddy_free = !bitmap_test(buddy_pfn);
        if !buddy_free {
            break;
        }
        // Remove buddy from its free list
        if !list_remove(order, buddy_pfn) {
            break;
        }
        // Merge: use the lower address
        if buddy_pfn < pfn {
            pfn = buddy_pfn;
        }
        order += 1;
    }

    list_push(order, pfn);
}

/// Convert a PFN to a physical address.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_pfn_to_phys(pfn: U32) -> U64 {
    BUDDY.base_addr + (pfn as U64) * (PAGE_SIZE as U64)
}

/// Convert a physical address to a PFN.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_phys_to_pfn(phys: U64) -> U32 {
    ((phys - BUDDY.base_addr) / PAGE_SIZE as U64) as U32
}

/// Get the total number of managed pages.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_total_pages() -> U32 {
    BUDDY.total_pages
}

/// Get the number of free pages.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_free_pages() -> U32 {
    BUDDY.free_pages
}

/// Get the allocation count.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_alloc_count() -> U64 {
    BUDDY.alloc_count
}

/// Get the number of free blocks at a given order.
#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_free_blocks(order: U32) -> U32 {
    if (order as usize) < MAX_ORDER {
        BUDDY.free_counts[order as usize]
    } else {
        0
    }
}
