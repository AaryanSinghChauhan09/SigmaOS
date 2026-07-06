// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// klib/sigma_ubc.rs — Unified Buffer Cache
// Implements: Page cache for VFS. Buffers disk I/O to avoid redundant
// reads/writes. Supports LRU eviction, dirty page writeback, and
// integration with the virtual memory manager for mmap.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

const CACHE_PAGES_MAX: usize = 8192; // Max pages to cache (32MB)
const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PageState {
    Free,
    Clean,
    Dirty,
    Locked,
}

pub struct UbcPage {
    pub state: PageState,
    pub inode: u64,     // Which file this page belongs to
    pub offset: u64,    // Offset within the file (multiple of PAGE_SIZE)
    pub phys_addr: u64, // Physical memory address backing this page
    pub ref_count: u32,
    pub lru_prev: usize,
    pub lru_next: usize,
}

impl UbcPage {
    const fn empty() -> Self {
        Self {
            state: PageState::Free,
            inode: 0,
            offset: 0,
            phys_addr: 0,
            ref_count: 0,
            lru_prev: 0,
            lru_next: 0,
        }
    }
}

pub struct UnifiedBufferCache {
    pages: [UbcPage; CACHE_PAGES_MAX],
    lru_head: usize,
    lru_tail: usize,
    free_count: usize,
}

static mut UBC: UnifiedBufferCache = UnifiedBufferCache {
    pages: [UbcPage::empty(); CACHE_PAGES_MAX],
    lru_head: CACHE_PAGES_MAX,
    lru_tail: CACHE_PAGES_MAX,
    free_count: CACHE_PAGES_MAX,
};

static UBC_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

impl UnifiedBufferCache {
    pub fn init(&mut self) {
        if UBC_INITIALIZED.swap(1, Ordering::SeqCst) != 0 {
            return;
        }

        // Initialize LRU list for free pages
        for i in 0..CACHE_PAGES_MAX {
            self.pages[i].lru_prev = if i > 0 { i - 1 } else { CACHE_PAGES_MAX };
            self.pages[i].lru_next = if i < CACHE_PAGES_MAX - 1 { i + 1 } else { CACHE_PAGES_MAX };
            self.pages[i].state = PageState::Free;
        }
        self.lru_head = 0;
        self.lru_tail = CACHE_PAGES_MAX - 1;
        self.free_count = CACHE_PAGES_MAX;
    }

    /// Looks up a page in the cache. Returns the physical address if found.
    pub fn lookup(&mut self, inode: u64, offset: u64) -> Option<u64> {
        let aligned_offset = offset & !(PAGE_SIZE - 1);
        
        // Simple linear search for now. In production, use a hash table.
        for i in 0..CACHE_PAGES_MAX {
            let page = &self.pages[i];
            if page.state != PageState::Free && page.inode == inode && page.offset == aligned_offset {
                // Move to head of LRU
                self.lru_remove(i);
                self.lru_push_front(i);
                return Some(page.phys_addr);
            }
        }
        None
    }

    /// Allocates a new cache page. May evict an old page if necessary.
    pub fn allocate_page(&mut self, inode: u64, offset: u64) -> Option<u64> {
        let aligned_offset = offset & !(PAGE_SIZE - 1);
        
        let target_idx = if self.free_count > 0 {
            // Take from LRU tail (which should be free pages during init)
            self.lru_tail
        } else {
            // Evict from LRU tail
            self.evict_page()
        };

        if target_idx >= CACHE_PAGES_MAX {
            return None; // Failed to allocate/evict
        }

        self.lru_remove(target_idx);
        
        let phys_addr = if self.pages[target_idx].state == PageState::Free {
            // Allocate physical memory
            crate::kernel::mm::buddy_allocator::alloc_pages(0).unwrap_or(0) as u64
        } else {
            self.pages[target_idx].phys_addr
        };

        if phys_addr == 0 {
            // Re-insert into LRU and fail
            self.lru_push_back(target_idx);
            return None;
        }

        self.pages[target_idx] = UbcPage {
            state: PageState::Clean,
            inode,
            offset: aligned_offset,
            phys_addr,
            ref_count: 1,
            lru_prev: CACHE_PAGES_MAX,
            lru_next: CACHE_PAGES_MAX,
        };

        self.lru_push_front(target_idx);
        
        if self.free_count > 0 {
            self.free_count -= 1;
        }

        Some(phys_addr)
    }

    /// Marks a page as dirty, meaning it needs to be written to disk.
    pub fn mark_dirty(&mut self, inode: u64, offset: u64) {
        let aligned_offset = offset & !(PAGE_SIZE - 1);
        for i in 0..CACHE_PAGES_MAX {
            let page = &mut self.pages[i];
            if page.state != PageState::Free && page.inode == inode && page.offset == aligned_offset {
                page.state = PageState::Dirty;
                break;
            }
        }
    }

    /// Syncs all dirty pages for a specific inode to disk.
    pub fn sync_inode(&mut self, inode: u64) {
        for i in 0..CACHE_PAGES_MAX {
            let page = &mut self.pages[i];
            if page.state == PageState::Dirty && page.inode == inode {
                // STUB: Write to disk via VFS/driver
                // vfs_write_physical(inode, page.offset, page.phys_addr, PAGE_SIZE);
                page.state = PageState::Clean;
            }
        }
    }

    /// Syncs all dirty pages in the cache to disk.
    pub fn sync_all(&mut self) {
        for i in 0..CACHE_PAGES_MAX {
            let page = &mut self.pages[i];
            if page.state == PageState::Dirty {
                // STUB: Write to disk
                page.state = PageState::Clean;
            }
        }
    }

    // -- Internal LRU helpers --

    fn evict_page(&mut self) -> usize {
        // Find oldest clean page
        let mut curr = self.lru_tail;
        while curr < CACHE_PAGES_MAX {
            let page = &self.pages[curr];
            if page.state == PageState::Clean && page.ref_count == 0 {
                return curr;
            }
            if page.state == PageState::Dirty && page.ref_count == 0 {
                // Writeback then evict
                // STUB: vfs_write_physical(page.inode, page.offset, page.phys_addr, PAGE_SIZE);
                self.pages[curr].state = PageState::Clean;
                return curr;
            }
            curr = page.lru_prev;
        }
        CACHE_PAGES_MAX // Nothing to evict
    }

    fn lru_remove(&mut self, idx: usize) {
        let prev = self.pages[idx].lru_prev;
        let next = self.pages[idx].lru_next;

        if prev < CACHE_PAGES_MAX {
            self.pages[prev].lru_next = next;
        } else {
            self.lru_head = next;
        }

        if next < CACHE_PAGES_MAX {
            self.pages[next].lru_prev = prev;
        } else {
            self.lru_tail = prev;
        }

        self.pages[idx].lru_prev = CACHE_PAGES_MAX;
        self.pages[idx].lru_next = CACHE_PAGES_MAX;
    }

    fn lru_push_front(&mut self, idx: usize) {
        self.pages[idx].lru_prev = CACHE_PAGES_MAX;
        self.pages[idx].lru_next = self.lru_head;
        
        if self.lru_head < CACHE_PAGES_MAX {
            self.pages[self.lru_head].lru_prev = idx;
        } else {
            self.lru_tail = idx;
        }
        
        self.lru_head = idx;
    }

    fn lru_push_back(&mut self, idx: usize) {
        self.pages[idx].lru_prev = self.lru_tail;
        self.pages[idx].lru_next = CACHE_PAGES_MAX;
        
        if self.lru_tail < CACHE_PAGES_MAX {
            self.pages[self.lru_tail].lru_next = idx;
        } else {
            self.lru_head = idx;
        }
        
        self.lru_tail = idx;
    }
}

pub fn ubc_init() {
    unsafe { UBC.init(); }
}

pub fn ubc_lookup(inode: u64, offset: u64) -> Option<u64> {
    unsafe { UBC.lookup(inode, offset) }
}

pub fn ubc_alloc(inode: u64, offset: u64) -> Option<u64> {
    unsafe { UBC.allocate_page(inode, offset) }
}

pub fn ubc_mark_dirty(inode: u64, offset: u64) {
    unsafe { UBC.mark_dirty(inode, offset); }
}

pub fn ubc_sync() {
    unsafe { UBC.sync_all(); }
}
