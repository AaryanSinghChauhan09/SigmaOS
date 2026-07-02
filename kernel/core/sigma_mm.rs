// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_mm.rs — Sovereign Memory Manager
// Replaces: sigma_mm.cpp (C++ stub, removed)
//
// Implements: Buddy allocator (2^n pages) + Slab allocator (kmalloc)
// Language: Rust #![no_std] — no libc, no alloc crate, no third-party
// Pattern: OOP via Traits (Allocator trait) + concrete structs

#![no_std]

use core::mem::size_of;
use core::ptr;

// ── Constants ────────────────────────────────────────────────────────────────

pub const PAGE_SIZE:       usize = 4096;
pub const PAGE_SHIFT:      usize = 12;
pub const MAX_ORDER:       usize = 11;        // up to 2^11 = 2048 pages = 8 MB
pub const BUDDY_POOL_PAGES: usize = 1 << 20; // 4 GB addressable (4096 * 1M)
pub const SLAB_MAX_SIZE:   usize = 2048;      // max kmalloc size (bytes)
pub const SLAB_CACHES:     usize = 12;        // 8,16,32,...,2048

// ── Allocator Trait (OOP interface) ─────────────────────────────────────────

pub trait Allocator {
    /// Allocate `size` bytes, returns physical address or 0 on failure
    unsafe fn alloc(&mut self, size: usize) -> usize;
    /// Free previously allocated block
    unsafe fn free(&mut self, addr: usize, size: usize);
    /// Available free bytes
    fn available(&self) -> usize;
}

// ── Buddy Allocator ──────────────────────────────────────────────────────────

/// Free list node embedded at the start of each free block
#[repr(C)]
struct FreeNode {
    next: usize, // physical address of next free node, 0 = end
}

pub struct BuddyAllocator {
    /// free_lists[order] = head physical address of free blocks of 2^order pages
    free_lists: [usize; MAX_ORDER + 1],
    /// Bitmap: 1 bit per page frame, 1 = free
    bitmap: [u64; BUDDY_POOL_PAGES / 64 + 1],
    base_phys:  usize, // physical base address of managed memory
    total_pages: usize,
    free_pages:  usize,
}

impl BuddyAllocator {
    /// Create a new buddy allocator managing [base, base + total_pages * PAGE_SIZE)
    pub const fn new(base_phys: usize, total_pages: usize) -> Self {
        Self {
            free_lists:  [0usize; MAX_ORDER + 1],
            bitmap:      [0u64; BUDDY_POOL_PAGES / 64 + 1],
            base_phys,
            total_pages,
            free_pages:  0,
        }
    }

    /// Initialise: mark all pages as free in the highest-order buckets
    pub fn init(&mut self) {
        let mut remaining = self.total_pages;
        let mut pfn = 0usize;
        while remaining > 0 {
            let order = remaining.next_power_of_two().trailing_zeros() as usize;
            let order = order.min(MAX_ORDER);
            let count = 1usize << order;
            if count > remaining { break; }
            self.push_free(pfn, order);
            pfn      += count;
            remaining -= count;
        }
        self.free_pages = self.total_pages;
    }

    fn push_free(&mut self, pfn: usize, order: usize) {
        let phys = self.base_phys + pfn * PAGE_SIZE;
        // Write next pointer into the block
        unsafe {
            let node = phys as *mut FreeNode;
            (*node).next = self.free_lists[order];
        }
        self.free_lists[order] = phys;
        self.set_bit(pfn, true);
    }

    fn pop_free(&mut self, order: usize) -> Option<usize> {
        let phys = self.free_lists[order];
        if phys == 0 { return None; }
        unsafe {
            let node = phys as *mut FreeNode;
            self.free_lists[order] = (*node).next;
        }
        let pfn = (phys - self.base_phys) / PAGE_SIZE;
        self.set_bit(pfn, false);
        Some(phys)
    }

    fn set_bit(&mut self, pfn: usize, val: bool) {
        let word = pfn / 64;
        let bit  = pfn % 64;
        if val { self.bitmap[word] |=   1 << bit; }
        else   { self.bitmap[word] &= !(1 << bit); }
    }

    fn buddy_pfn(pfn: usize, order: usize) -> usize {
        pfn ^ (1 << order)
    }
}

impl Allocator for BuddyAllocator {
    unsafe fn alloc(&mut self, size: usize) -> usize {
        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let order = pages.next_power_of_two().trailing_zeros() as usize;
        let order = order.min(MAX_ORDER);

        // Find smallest available order >= requested
        for o in order..=MAX_ORDER {
            if let Some(phys) = self.pop_free(o) {
                // Split excess buddies back into free lists
                let mut curr_phys = phys;
                let mut curr_ord  = o;
                while curr_ord > order {
                    curr_ord -= 1;
                    let buddy_phys = curr_phys + (1 << curr_ord) * PAGE_SIZE;
                    let buddy_pfn  = (buddy_phys - self.base_phys) / PAGE_SIZE;
                    self.push_free(buddy_pfn, curr_ord);
                }
                self.free_pages -= 1 << order;
                return curr_phys;
            }
        }
        0 // OOM
    }

    unsafe fn free(&mut self, addr: usize, size: usize) {
        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let order = pages.next_power_of_two().trailing_zeros() as usize;
        let order = order.min(MAX_ORDER);
        let mut pfn = (addr - self.base_phys) / PAGE_SIZE;

        // Coalesce with buddy
        let mut o = order;
        while o < MAX_ORDER {
            let buddy = Self::buddy_pfn(pfn, o);
            // Check if buddy is free (bit set)
            let word = buddy / 64; let bit = buddy % 64;
            if (self.bitmap[word] >> bit) & 1 == 0 { break; }
            // Remove buddy from free list
            self.pop_free(o);
            pfn = pfn.min(buddy);
            o  += 1;
        }
        self.push_free(pfn, o);
        self.free_pages += 1 << order;
    }

    fn available(&self) -> usize {
        self.free_pages * PAGE_SIZE
    }
}

// ── Slab Allocator (kmalloc) ─────────────────────────────────────────────────

struct SlabCache {
    obj_size:  usize,
    free_head: usize, // physical address of first free object
    alloc_cnt: usize,
}

pub struct SlabAllocator {
    caches: [SlabCache; SLAB_CACHES],
    buddy:  *mut BuddyAllocator, // back-pointer to page allocator
}

impl SlabAllocator {
    const SIZES: [usize; SLAB_CACHES] = [8,16,32,64,128,256,512,1024,1280,1536,1792,2048];

    pub fn new(buddy: *mut BuddyAllocator) -> Self {
        let caches = Self::SIZES.map(|s| SlabCache { obj_size: s, free_head: 0, alloc_cnt: 0 });
        Self { caches, buddy }
    }

    fn cache_index(size: usize) -> Option<usize> {
        Self::SIZES.iter().position(|&s| size <= s)
    }
}

impl Allocator for SlabAllocator {
    unsafe fn alloc(&mut self, size: usize) -> usize {
        let idx = match Self::cache_index(size) {
            Some(i) => i,
            None    => return 0, // too large — use buddy directly
        };
        let cache = &mut self.caches[idx];
        if cache.free_head == 0 {
            // Refill: allocate one page from buddy
            let page = (*self.buddy).alloc(PAGE_SIZE);
            if page == 0 { return 0; }
            // Carve page into objects
            let count = PAGE_SIZE / cache.obj_size;
            let mut prev = 0usize;
            for i in 0..count {
                let obj = page + i * cache.obj_size;
                let next_ptr = obj as *mut usize;
                *next_ptr = prev;
                prev = obj;
            }
            cache.free_head = prev;
        }
        let obj = cache.free_head;
        let next = *(obj as *const usize);
        cache.free_head = next;
        cache.alloc_cnt += 1;
        // Zero the object before returning
        ptr::write_bytes(obj as *mut u8, 0, cache.obj_size);
        obj
    }

    unsafe fn free(&mut self, addr: usize, size: usize) {
        if let Some(idx) = Self::cache_index(size) {
            let cache = &mut self.caches[idx];
            let next_ptr = addr as *mut usize;
            *next_ptr = cache.free_head;
            cache.free_head = addr;
            cache.alloc_cnt -= 1;
        }
    }

    fn available(&self) -> usize {
        // Return total free across all caches (approximate)
        self.caches.iter().map(|c| {
            let mut count = 0;
            let mut node = c.free_head;
            while node != 0 && count < 1024 {
                node = unsafe { *(node as *const usize) };
                count += 1;
            }
            count * c.obj_size
        }).sum()
    }
}

/// Public kmalloc / kfree interface
///
/// # Safety
/// Caller must ensure size matches on free
pub unsafe fn kmalloc(slab: &mut SlabAllocator, size: usize) -> usize {
    slab.alloc(size)
}

pub unsafe fn kfree(slab: &mut SlabAllocator, addr: usize, size: usize) {
    slab.free(addr, size);
}
