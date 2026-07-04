// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/memory/sigma_buddy.rs — Buddy allocator + Slab allocator
// Physical page frame management. Alloc/free in O(log n).
// Slab: object caches for kmalloc() efficiency.
//
// Inspired by: Linux mm/buddy.c, mm/slab.c
// Language: Rust (#![no_std])

#![no_std]
#![allow(dead_code)]

// ── Constants ──────────────────────────────────────────────────────────────
pub const PAGE_SIZE:      usize = 4096;       // 4KB pages
pub const MAX_ORDER:      usize = 11;          // 2^10 = 4MB max block
pub const MAX_PAGES:      usize = 1 << 20;    // 4GB physical @ 4KB pages
pub const PAGE_FREE:      u8    = 0;
pub const PAGE_USED:      u8    = 1;

// ── Physical page frame descriptor ────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PageFrame {
    pub pfn:      u32,   // page frame number
    pub order:    u8,    // block order (0 = single 4K page)
    pub flags:    u8,    // PAGE_FREE / PAGE_USED
    pub next_pfn: u32,   // next in free list (0 = end)
    pub prev_pfn: u32,
}

impl PageFrame {
    pub const fn new(pfn: u32) -> Self {
        Self { pfn, order: 0, flags: PAGE_FREE, next_pfn: 0, prev_pfn: 0 }
    }
    pub fn is_free(&self) -> bool { self.flags == PAGE_FREE }
}

// ── Buddy Allocator ────────────────────────────────────────────────────────
pub struct BuddyAllocator {
    /// free_lists[order] = head PFN of free blocks at that order
    pub free_lists:  [u32; MAX_ORDER],
    pub free_counts: [u32; MAX_ORDER],
    pub frames:      [PageFrame; MAX_PAGES],
    pub total_pages: u32,
    pub free_pages:  u32,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            free_lists:  [0u32; MAX_ORDER],
            free_counts: [0u32; MAX_ORDER],
            frames:      [PageFrame { pfn:0, order:0, flags:0, next_pfn:0, prev_pfn:0 }; MAX_PAGES],
            total_pages: 0,
            free_pages:  0,
        }
    }

    /// Initialize with a contiguous memory range
    pub fn init(&mut self, start_pfn: u32, end_pfn: u32) {
        self.total_pages = end_pfn - start_pfn;
        // Add all pages as max-order blocks
        let mut pfn = start_pfn;
        while pfn + (1 << (MAX_ORDER - 1)) as u32 <= end_pfn {
            let order = MAX_ORDER - 1;
            self.free_add(pfn, order as u8);
            pfn += 1 << order;
        }
        // Handle remainder with smaller blocks
        let mut remaining = end_pfn - pfn;
        let mut cur_pfn = pfn;
        let mut order = MAX_ORDER - 1;
        while remaining > 0 && order < MAX_ORDER {
            if remaining >= (1u32 << order) {
                self.free_add(cur_pfn, order as u8);
                cur_pfn   += 1 << order;
                remaining -= 1 << order;
            }
            if order == 0 { break; }
            order -= 1;
        }
    }

    fn free_add(&mut self, pfn: u32, order: u8) {
        let idx = pfn as usize;
        if idx >= MAX_PAGES { return; }
        self.frames[idx].pfn      = pfn;
        self.frames[idx].order    = order;
        self.frames[idx].flags    = PAGE_FREE;
        self.frames[idx].next_pfn = self.free_lists[order as usize];
        self.frames[idx].prev_pfn = 0;
        if self.free_lists[order as usize] != 0 {
            let next = self.free_lists[order as usize] as usize;
            if next < MAX_PAGES { self.frames[next].prev_pfn = pfn; }
        }
        self.free_lists[order as usize] = pfn;
        self.free_counts[order as usize] += 1;
        self.free_pages += 1 << order;
    }

    fn free_remove(&mut self, pfn: u32, order: u8) {
        let idx = pfn as usize;
        if idx >= MAX_PAGES { return; }
        let prev = self.frames[idx].prev_pfn;
        let next = self.frames[idx].next_pfn;
        if prev == 0 { self.free_lists[order as usize] = next; }
        else if prev as usize < MAX_PAGES { self.frames[prev as usize].next_pfn = next; }
        if next as usize < MAX_PAGES { self.frames[next as usize].prev_pfn = prev; }
        self.frames[idx].flags = PAGE_USED;
        self.free_counts[order as usize] -= 1;
    }

    /// Allocate 2^order pages. Returns base PFN or 0 on failure.
    pub fn alloc(&mut self, order: u8) -> u32 {
        let order = order as usize;
        if order >= MAX_ORDER { return 0; }
        // Find smallest order with a free block
        let mut found_order = MAX_ORDER;
        for o in order..MAX_ORDER {
            if self.free_lists[o] != 0 { found_order = o; break; }
        }
        if found_order == MAX_ORDER { return 0; }
        // Remove block from found_order
        let pfn = self.free_lists[found_order];
        self.free_remove(pfn, found_order as u8);
        self.free_pages -= 1 << found_order;
        // Split down to requested order
        let mut cur_order = found_order;
        while cur_order > order {
            cur_order -= 1;
            // Buddy is at pfn XOR (1 << cur_order)
            let buddy_pfn = pfn ^ (1u32 << cur_order);
            self.free_add(buddy_pfn, cur_order as u8);
            self.free_pages += 1 << cur_order;
        }
        self.frames[pfn as usize].flags = PAGE_USED;
        self.frames[pfn as usize].order = order as u8;
        pfn
    }

    /// Free pages starting at pfn with given order. Coalesces buddies.
    pub fn free(&mut self, mut pfn: u32, mut order: u8) {
        while (order as usize) < MAX_ORDER - 1 {
            let buddy_pfn = pfn ^ (1u32 << order);
            let buddy_idx = buddy_pfn as usize;
            if buddy_idx >= MAX_PAGES { break; }
            let buddy = &self.frames[buddy_idx];
            if !buddy.is_free() || buddy.order != order { break; }
            // Coalesce: remove buddy, move to next order
            self.free_remove(buddy_pfn, order);
            self.free_pages -= 1 << order;
            // Aligned block is the lower of the two
            if buddy_pfn < pfn { pfn = buddy_pfn; }
            order += 1;
        }
        self.free_add(pfn, order);
    }

    pub fn free_bytes(&self) -> u64 { self.free_pages as u64 * PAGE_SIZE as u64 }
    pub fn total_bytes(&self) -> u64 { self.total_pages as u64 * PAGE_SIZE as u64 }
}

// ── Slab Allocator ─────────────────────────────────────────────────────────
/// Object cache for fixed-size kernel allocations (kmalloc)
pub const SLAB_SIZES: [usize; 8] = [8, 16, 32, 64, 128, 256, 512, 1024];
pub const SLAB_CACHE_COUNT: usize = SLAB_SIZES.len();
const SLAB_OBJ_PER_PAGE: usize = PAGE_SIZE / 8;   // worst case (8 byte objects)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SlabObject {
    pub next: u64,  // next free object pointer (0 = end of list)
}

pub struct SlabCache {
    pub obj_size:    usize,
    pub free_list:   u64,    // pointer to first free object
    pub total_objs:  u32,
    pub free_objs:   u32,
    pub page_pfn:    u32,    // backing pages (simplified: single page slab)
}

impl SlabCache {
    pub const fn new(obj_size: usize) -> Self {
        Self { obj_size, free_list: 0, total_objs: 0, free_objs: 0, page_pfn: 0 }
    }

    /// Initialize slab from a page PFN
    pub fn init_from_page(&mut self, page_vaddr: u64) {
        let per_page = PAGE_SIZE / self.obj_size;
        self.total_objs = per_page as u32;
        self.free_objs  = per_page as u32;
        self.free_list  = page_vaddr;
        // Link objects into free list
        for i in 0..per_page {
            let obj_ptr = (page_vaddr + (i * self.obj_size) as u64) as *mut u64;
            let next_ptr = if i + 1 < per_page {
                page_vaddr + ((i + 1) * self.obj_size) as u64
            } else { 0 };
            unsafe { *obj_ptr = next_ptr; }
        }
    }

    pub fn alloc(&mut self) -> u64 {
        if self.free_list == 0 || self.free_objs == 0 { return 0; }
        let obj = self.free_list;
        let next = unsafe { *(obj as *const u64) };
        self.free_list = next;
        self.free_objs -= 1;
        obj
    }

    pub fn free(&mut self, ptr: u64) {
        if ptr == 0 { return; }
        unsafe { *(ptr as *mut u64) = self.free_list; }
        self.free_list = ptr;
        self.free_objs += 1;
    }
}

pub struct SlabAllocator {
    pub caches: [SlabCache; SLAB_CACHE_COUNT],
}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self { caches: [
            SlabCache::new(SLAB_SIZES[0]),
            SlabCache::new(SLAB_SIZES[1]),
            SlabCache::new(SLAB_SIZES[2]),
            SlabCache::new(SLAB_SIZES[3]),
            SlabCache::new(SLAB_SIZES[4]),
            SlabCache::new(SLAB_SIZES[5]),
            SlabCache::new(SLAB_SIZES[6]),
            SlabCache::new(SLAB_SIZES[7]),
        ]}
    }

    /// kmalloc: allocate at least `size` bytes from appropriate slab cache
    pub fn kmalloc(&mut self, size: usize) -> u64 {
        for i in 0..SLAB_CACHE_COUNT {
            if SLAB_SIZES[i] >= size {
                let ptr = self.caches[i].alloc();
                if ptr != 0 { return ptr; }
            }
        }
        0   // OOM
    }

    /// kfree: return allocation to correct slab cache
    pub fn kfree(&mut self, ptr: u64, size: usize) {
        for i in 0..SLAB_CACHE_COUNT {
            if SLAB_SIZES[i] >= size {
                self.caches[i].free(ptr);
                return;
            }
        }
    }
}

// ── ASLR: 42-bit entropy for VMA randomization ─────────────────────────────
pub fn aslr_offset(entropy_source: u64, vma_type: u8) -> u64 {
    // Simple PRNG for address randomization — production: use hardware RNG
    let mixed = entropy_source
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        ^ (entropy_source >> 33);
    // 42-bit mask for user-space VMA randomization
    let mask: u64 = (1 << 42) - 1;
    (mixed & mask) & !0xFFF  // page-align
}

// ── W^X enforcement: no page is both writable and executable ───────────────
pub const PROT_READ:    u32 = 1 << 0;
pub const PROT_WRITE:   u32 = 1 << 1;
pub const PROT_EXEC:    u32 = 1 << 2;

pub fn wxor_check(prot: u32) -> bool {
    // W^X: write and exec cannot both be set
    (prot & PROT_WRITE) == 0 || (prot & PROT_EXEC) == 0
}

// ── C-ABI exports ──────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn buddy_alloc_pages(order: u8) -> u64 {
    // In production: call global BUDDY_ALLOC instance
    // Returns physical address of allocated pages
    let _ = order;
    0
}

#[no_mangle]
pub extern "C" fn buddy_free_pages(pfn: u32, order: u8) {
    let _ = (pfn, order);
}

#[no_mangle]
pub extern "C" fn kmalloc(size: usize) -> u64 {
    let _ = size;
    0
}

#[no_mangle]
pub extern "C" fn kfree(ptr: u64, size: usize) {
    let _ = (ptr, size);
}
