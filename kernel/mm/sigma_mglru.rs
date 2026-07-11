// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/mm/sigma_mglru.rs — Multi-Generation LRU (MGLRU) Implementation
//
// MGLRU is Linux's modern page replacement algorithm that improves upon traditional
// LRU by tracking access patterns across multiple generations. This implementation
// follows the Linux 6.x MGLRU design with OOP principles and no external dependencies.
//
// Key features:
// - Multi-generation tracking (young, middle, old generations)
// - Aging mechanism for page access patterns
// - Working set detection
// - O(1) access and O(log n) eviction
// - No external dependencies, pure Rust implementation

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Constants (Linux-inspired values)
// ─────────────────────────────────────────────────────────────────────────────

pub const MGLRU_MAX_PAGES: usize = 262144; // 1GB with 4KB pages
pub const MGLRU_NR_GENS: usize = 3; // Young, Middle, Old
pub const MGLRU_YOUNG_GEN: usize = 0;
pub const MGLRU_MIDDLE_GEN: usize = 1;
pub const MGLRU_OLD_GEN: usize = 2;
pub const MGLRU_MIN_AGE: u64 = 1000; // 1ms in nanoseconds
pub const MGLRU_MAX_AGE: u64 = 10_000_000_000; // 10 seconds
pub const MGLRU_BATCH_SIZE: usize = 32;
pub const NIL: u32 = u32::MAX;

// ─────────────────────────────────────────────────────────────────────────────
// Page flags
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PageFlags {
    None = 0,
    Referenced = 1,
    Dirty = 2,
    Active = 4,
    WorkingSet = 8,
}

impl PageFlags {
    pub fn has(&self, flag: PageFlags) -> bool {
        (*self as u8) & (flag as u8) != 0
    }

    pub fn set(&mut self, flag: PageFlags) {
        *self = unsafe { core::mem::transmute((*self as u8) | (flag as u8)) };
    }

    pub fn clear(&mut self, flag: PageFlags) {
        *self = unsafe { core::mem::transmute((*self as u8) & !(flag as u8)) };
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Page entity with OOP-style encapsulation
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct MglruPage {
    pub pfn: u32, // Physical frame number
    pub gen: usize, // Current generation
    pub ref_count: u32,
    pub age: u64, // Time since last access
    pub flags: u8,
    pub access_count: u32,
    pub last_access: u64,
    pub next: u32, // Linked list for generation
    pub prev: u32,
}

impl MglruPage {
    pub const fn empty() -> Self {
        Self {
            pfn: NIL,
            gen: MGLRU_YOUNG_GEN,
            ref_count: 0,
            age: 0,
            flags: 0,
            access_count: 0,
            last_access: 0,
            next: NIL,
            prev: NIL,
        }
    }

    pub fn is_referenced(&self) -> bool {
        self.flags & (PageFlags::Referenced as u8) != 0
    }

    pub fn is_dirty(&self) -> bool {
        self.flags & (PageFlags::Dirty as u8) != 0
    }

    pub fn is_active(&self) -> bool {
        self.flags & (PageFlags::Active as u8) != 0
    }

    pub fn is_working_set(&self) -> bool {
        self.flags & (PageFlags::WorkingSet as u8) != 0
    }

    pub fn set_referenced(&mut self) {
        self.flags |= PageFlags::Referenced as u8;
    }

    pub fn set_dirty(&mut self) {
        self.flags |= PageFlags::Dirty as u8;
    }

    pub fn set_active(&mut self) {
        self.flags |= PageFlags::Active as u8;
    }

    pub fn set_working_set(&mut self) {
        self.flags |= PageFlags::WorkingSet as u8;
    }

    pub fn clear_referenced(&mut self) {
        self.flags &= !(PageFlags::Referenced as u8);
    }

    pub fn clear_dirty(&mut self) {
        self.flags &= !(PageFlags::Dirty as u8);
    }

    pub fn clear_active(&mut self) {
        self.flags &= !(PageFlags::Active as u8);
    }

    pub fn clear_working_set(&mut self) {
        self.flags &= !(PageFlags::WorkingSet as u8);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Generation list (doubly-linked list per generation)
// ─────────────────────────────────────────────────────────────────────────────

pub struct MglruGenList {
    head: u32,
    tail: u32,
    count: usize,
    total_age: u64,
}

impl MglruGenList {
    pub const fn new() -> Self {
        Self {
            head: NIL,
            tail: NIL,
            count: 0,
            total_age: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == NIL
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn avg_age(&self) -> u64 {
        if self.count == 0 { return 0; }
        self.total_age / self.count as u64
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MGLRU Manager with OOP principles
// ─────────────────────────────────────────────────────────────────────────────

pub struct MglruManager {
    pages: [MglruPage; MGLRU_MAX_PAGES],
    gen_lists: [MglruGenList; MGLRU_NR_GENS],
    free_list: MglruGenList,
    current_time: u64,
    nr_pages: usize,
    next_pfn: u32,
    working_set_size: usize,
    min_gen_age: [u64; MGLRU_NR_GENS],
}

impl MglruManager {
    pub const fn new() -> Self {
        Self {
            pages: [MglruPage::empty(); MGLRU_MAX_PAGES],
            gen_lists: [MglruGenList::new(); MGLRU_NR_GENS],
            free_list: MglruGenList::new(),
            current_time: 0,
            nr_pages: 0,
            next_pfn: 0,
            working_set_size: 0,
            min_gen_age: [0; MGLRU_NR_GENS],
        }
    }

    // Initialize the MGLRU manager
    pub fn init(&mut self, nr_pages: usize) {
        self.nr_pages = nr_pages.min(MGLRU_MAX_PAGES);
        self.working_set_size = self.nr_pages / 2; // 50% of pages for working set
        
        // Initialize free list
        for i in 0..self.nr_pages {
            let pfn = i as u32;
            self.pages[pfn as usize].pfn = pfn;
            self.add_to_free_list(pfn);
        }
    }

    // Add page to free list
    fn add_to_free_list(&mut self, pfn: u32) {
        let page = &mut self.pages[pfn as usize];
        page.next = self.free_list.head;
        page.prev = NIL;
        
        if self.free_list.head != NIL {
            self.pages[self.free_list.head as usize].prev = pfn;
        } else {
            self.free_list.tail = pfn;
        }
        
        self.free_list.head = pfn;
        self.free_list.count += 1;
    }

    // Remove page from free list
    fn remove_from_free_list(&mut self, pfn: u32) {
        let page = &mut self.pages[pfn as usize];
        
        if page.prev != NIL {
            self.pages[page.prev as usize].next = page.next;
        } else {
            self.free_list.head = page.next;
        }
        
        if page.next != NIL {
            self.pages[page.next as usize].prev = page.prev;
        } else {
            self.free_list.tail = page.prev;
        }
        
        page.next = NIL;
        page.prev = NIL;
        self.free_list.count -= 1;
    }

    // Allocate a page
    pub fn alloc_page(&mut self) -> Option<u32> {
        if self.free_list.is_empty() {
            // Try to evict from old generation
            if let Some(pfn) = self.evict_from_gen(MGLRU_OLD_GEN) {
                return Some(pfn);
            }
            return None;
        }
        
        let pfn = self.free_list.head;
        self.remove_from_free_list(pfn);
        
        // Initialize page in young generation
        self.pages[pfn as usize].gen = MGLRU_YOUNG_GEN;
        self.pages[pfn as usize].age = 0;
        self.pages[pfn as usize].last_access = self.current_time;
        self.pages[pfn as usize].access_count = 0;
        self.pages[pfn as usize].set_active();
        
        self.add_to_gen_list(MGLRU_YOUNG_GEN, pfn);
        Some(pfn)
    }

    // Free a page
    pub fn free_page(&mut self, pfn: u32) {
        if (pfn as usize) >= self.nr_pages { return; }
        
        let page = &mut self.pages[pfn as usize];
        if !page.is_active() { return; }
        
        self.remove_from_gen_list(page.gen, pfn);
        page.clear_active();
        page.clear_referenced();
        page.clear_dirty();
        page.clear_working_set();
        
        self.add_to_free_list(pfn);
    }

    // Add page to generation list
    fn add_to_gen_list(&mut self, gen: usize, pfn: u32) {
        let page = &mut self.pages[pfn as usize];
        page.gen = gen;
        page.next = self.gen_lists[gen].head;
        page.prev = NIL;
        
        if self.gen_lists[gen].head != NIL {
            self.pages[self.gen_lists[gen].head as usize].prev = pfn;
        } else {
            self.gen_lists[gen].tail = pfn;
        }
        
        self.gen_lists[gen].head = pfn;
        self.gen_lists[gen].count += 1;
    }

    // Remove page from generation list
    fn remove_from_gen_list(&mut self, gen: usize, pfn: u32) {
        let page = &mut self.pages[pfn as usize];
        
        if page.prev != NIL {
            self.pages[page.prev as usize].next = page.next;
        } else {
            self.gen_lists[gen].head = page.next;
        }
        
        if page.next != NIL {
            self.pages[page.next as usize].prev = page.prev;
        } else {
            self.gen_lists[gen].tail = page.prev;
        }
        
        page.next = NIL;
        page.prev = NIL;
        self.gen_lists[gen].count -= 1;
    }

    // Access a page (promote if needed)
    pub fn access_page(&mut self, pfn: u32) {
        if (pfn as usize) >= self.nr_pages { return; }
        
        let page = &mut self.pages[pfn as usize];
        if !page.is_active() { return; }
        
        page.set_referenced();
        page.access_count += 1;
        page.last_access = self.current_time;
        
        // Promote to younger generation if accessed frequently
        let old_gen = page.gen;
        if page.access_count > 2 && old_gen > MGLRU_YOUNG_GEN {
            self.remove_from_gen_list(old_gen, pfn);
            self.add_to_gen_list(old_gen - 1, pfn);
        }
    }

    // Age pages and promote/demote generations
    pub fn age_pages(&mut self, delta_ns: u64) {
        self.current_time += delta_ns;
        
        for gen in 0..MGLRU_NR_GENS {
            let mut current = self.gen_lists[gen].head;
            while current != NIL {
                let page = &mut self.pages[current as usize];
                page.age += delta_ns;
                current = page.next;
            }
            
            // Update min age for this generation
            if let Some(min_age) = self.get_min_gen_age(gen) {
                self.min_gen_age[gen] = min_age;
            }
        }
        
        // Promote/demote pages based on age
        self.promote_demote_pages();
    }

    // Get minimum age in a generation
    fn get_min_gen_age(&self, gen: usize) -> Option<u64> {
        let mut min_age = u64::MAX;
        let mut current = self.gen_lists[gen].head;
        
        while current != NIL {
            let page = &self.pages[current as usize];
            min_age = min_age.min(page.age);
            current = page.next;
        }
        
        if min_age == u64::MAX { None } else { Some(min_age) }
    }

    // Promote/demote pages between generations
    fn promote_demote_pages(&mut self) {
        // Promote young pages that are frequently accessed
        let mut current = self.gen_lists[MGLRU_YOUNG_GEN].head;
        while current != NIL {
            let next = self.pages[current as usize].next;
            let page = &self.pages[current as usize];
            
            if page.access_count > 3 && page.age > MGLRU_MIN_AGE {
                self.remove_from_gen_list(MGLRU_YOUNG_GEN, current);
                self.add_to_gen_list(MGLRU_MIDDLE_GEN, current);
                self.pages[current as usize].access_count = 0;
            }
            
            current = next;
        }
        
        // Promote middle pages to old generation
        let mut current = self.gen_lists[MGLRU_MIDDLE_GEN].head;
        while current != NIL {
            let next = self.pages[current as usize].next;
            let page = &self.pages[current as usize];
            
            if page.age > MGLRU_MAX_AGE / 2 {
                self.remove_from_gen_list(MGLRU_MIDDLE_GEN, current);
                self.add_to_gen_list(MGLRU_OLD_GEN, current);
            }
            
            current = next;
        }
        
        // Demote old pages that are accessed
        let mut current = self.gen_lists[MGLRU_OLD_GEN].head;
        while current != NIL {
            let next = self.pages[current as usize].next;
            let page = &self.pages[current as usize];
            
            if page.is_referenced() {
                page.clear_referenced();
                self.remove_from_gen_list(MGLRU_OLD_GEN, current);
                self.add_to_gen_list(MGLRU_MIDDLE_GEN, current);
                self.pages[current as usize].age = 0;
            }
            
            current = next;
        }
    }

    // Evict a page from a specific generation
    pub fn evict_from_gen(&mut self, gen: usize) -> Option<u32> {
        if self.gen_lists[gen].is_empty() { return None; }
        
        // Evict from tail (oldest in generation)
        let pfn = self.gen_lists[gen].tail;
        
        if pfn != NIL {
            let page = &mut self.pages[pfn as usize];
            
            // Skip if dirty (should be written back first)
            if page.is_dirty() {
                // In a real implementation, write back here
                page.clear_dirty();
            }
            
            self.remove_from_gen_list(gen, pfn);
            page.clear_active();
            page.clear_referenced();
            page.clear_working_set();
            
            return Some(pfn);
        }
        
        None
    }

    // Evict best candidate page
    pub fn evict_best_candidate(&mut self) -> Option<u32> {
        // Try old generation first
        if let Some(pfn) = self.evict_from_gen(MGLRU_OLD_GEN) {
            return Some(pfn);
        }
        
        // Try middle generation
        if let Some(pfn) = self.evict_from_gen(MGLRU_MIDDLE_GEN) {
            return Some(pfn);
        }
        
        // Try young generation (last resort)
        self.evict_from_gen(MGLRU_YOUNG_GEN)
    }

    // Update working set
    pub fn update_working_set(&mut self) {
        let mut working_set_count = 0usize;
        
        for gen in 0..MGLRU_NR_GENS {
            let mut current = self.gen_lists[gen].head;
            while current != NIL {
                let page = &self.pages[current as usize];
                if page.is_working_set() {
                    working_set_count += 1;
                }
                current = page.next;
            }
        }
        
        self.working_set_size = working_set_count;
    }

    // Mark page as working set
    pub fn mark_working_set(&mut self, pfn: u32) {
        if (pfn as usize) >= self.nr_pages { return; }
        self.pages[pfn as usize].set_working_set();
    }

    // Get statistics
    pub fn get_stats(&self) -> MglruStats {
        let mut gen_counts = [0usize; MGLRU_NR_GENS];
        let mut total_age = [0u64; MGLRU_NR_GENS];
        
        for gen in 0..MGLRU_NR_GENS {
            gen_counts[gen] = self.gen_lists[gen].count;
            total_age[gen] = self.gen_lists[gen].total_age;
        }
        
        MglruStats {
            total_pages: self.nr_pages,
            free_pages: self.free_list.count,
            gen_counts,
            total_age,
            working_set_size: self.working_set_size,
            current_time: self.current_time,
        }
    }

    // Get page info
    pub fn get_page_info(&self, pfn: u32) -> Option<MglruPageInfo> {
        if (pfn as usize) >= self.nr_pages { return None; }
        
        let page = &self.pages[pfn as usize];
        if !page.is_active() { return None; }
        
        Some(MglruPageInfo {
            pfn: page.pfn,
            gen: page.gen,
            age: page.age,
            ref_count: page.ref_count,
            access_count: page.access_count,
            last_access: page.last_access,
            is_referenced: page.is_referenced(),
            is_dirty: page.is_dirty(),
            is_working_set: page.is_working_set(),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Statistics structure
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct MglruStats {
    pub total_pages: usize,
    pub free_pages: usize,
    pub gen_counts: [usize; MGLRU_NR_GENS],
    pub total_age: [u64; MGLRU_NR_GENS],
    pub working_set_size: usize,
    pub current_time: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct MglruPageInfo {
    pub pfn: u32,
    pub gen: usize,
    pub age: u64,
    pub ref_count: u32,
    pub access_count: u32,
    pub last_access: u64,
    pub is_referenced: bool,
    pub is_dirty: bool,
    pub is_working_set: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut MGLRU: MglruManager = MglruManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_init(nr_pages: usize) {
    MGLRU.init(nr_pages);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_alloc_page() -> u32 {
    MGLRU.alloc_page().unwrap_or(NIL)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_free_page(pfn: u32) {
    MGLRU.free_page(pfn);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_access_page(pfn: u32) {
    MGLRU.access_page(pfn);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_age_pages(delta_ns: u64) {
    MGLRU.age_pages(delta_ns);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_evict_page() -> u32 {
    MGLRU.evict_best_candidate().unwrap_or(NIL)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_mark_working_set(pfn: u32) {
    MGLRU.mark_working_set(pfn);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_get_working_set_size() -> usize {
    MGLRU.working_set_size
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_get_free_pages() -> usize {
    MGLRU.free_list.count
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mglru_get_gen_count(gen: usize) -> usize {
    if gen < MGLRU_NR_GENS { MGLRU.gen_lists[gen].count } else { 0 }
}
