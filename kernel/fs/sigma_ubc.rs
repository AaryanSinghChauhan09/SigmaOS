//! SigmaOS — Unified Buffer Cache (UBC)
//! Caches file blocks in physical memory to speed up I/O.
//! Uses a simple LRU replacement policy.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;

const PAGE_SIZE: usize = 4096;
const MAX_CACHE_PAGES: usize = 1024; // 4MB cache

#[derive(Copy, Clone)]
pub struct CachePage {
    pub dev_id:  U32,
    pub block:   U64,
    pub pfn:     U32,  // Physical Frame Number backing this block
    pub dirty:   bool,
    pub valid:   bool,
    pub prev:    i32,
    pub next:    i32,
}

impl CachePage {
    pub const fn empty() -> Self {
        CachePage {
            dev_id: 0, block: 0, pfn: 0, dirty: false, valid: false,
            prev: -1, next: -1,
        }
    }
}

pub struct UbcState {
    pub pages: [CachePage; MAX_CACHE_PAGES],
    pub lru_head: i32,
    pub lru_tail: i32,
    pub cache_hits: U64,
    pub cache_misses: U64,
}

static mut UBC: UbcState = UbcState {
    pages: [CachePage::empty(); MAX_CACHE_PAGES],
    lru_head: -1,
    lru_tail: -1,
    cache_hits: 0,
    cache_misses: 0,
};

// ── External Dependencies ───────────────────────────────────────────────────
extern "C" {
    fn sigma_buddy_alloc(order: U32) -> U32;
    fn sigma_buddy_free(pfn: U32, order: U32);
}

// ── Internal Helpers ────────────────────────────────────────────────────────

unsafe fn lru_remove(idx: i32) {
    if idx < 0 || idx as usize >= MAX_CACHE_PAGES { return; }
    let prev = UBC.pages[idx as usize].prev;
    let next = UBC.pages[idx as usize].next;

    if prev >= 0 {
        UBC.pages[prev as usize].next = next;
    } else {
        UBC.lru_head = next;
    }

    if next >= 0 {
        UBC.pages[next as usize].prev = prev;
    } else {
        UBC.lru_tail = prev;
    }

    UBC.pages[idx as usize].prev = -1;
    UBC.pages[idx as usize].next = -1;
}

unsafe fn lru_append(idx: i32) {
    if idx < 0 || idx as usize >= MAX_CACHE_PAGES { return; }
    
    UBC.pages[idx as usize].prev = UBC.lru_tail;
    UBC.pages[idx as usize].next = -1;

    if UBC.lru_tail >= 0 {
        UBC.pages[UBC.lru_tail as usize].next = idx;
    } else {
        UBC.lru_head = idx;
    }
    UBC.lru_tail = idx;
}

// ── Public API ──────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_init() -> i32 {
    for i in 0..MAX_CACHE_PAGES {
        UBC.pages[i].valid = false;
        // Initially, link all pages in LRU list
        lru_append(i as i32);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_lookup(dev_id: U32, block: U64) -> U32 {
    // Linear scan (could be optimized with a hash table)
    for i in 0..MAX_CACHE_PAGES {
        if UBC.pages[i].valid && UBC.pages[i].dev_id == dev_id && UBC.pages[i].block == block {
            // Hit! Move to tail (most recently used)
            lru_remove(i as i32);
            lru_append(i as i32);
            UBC.cache_hits += 1;
            return UBC.pages[i].pfn;
        }
    }
    UBC.cache_misses += 1;
    U32::MAX
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_insert(dev_id: U32, block: U64, pfn: U32) -> i32 {
    // Evict the least recently used page (head)
    let victim_idx = UBC.lru_head;
    if victim_idx < 0 { return -1; }

    let victim = &mut UBC.pages[victim_idx as usize];

    // If victim was valid and dirty, we would normally flush it to disk here
    // For now, we just overwrite it
    if victim.valid && victim.pfn != 0 && victim.pfn != pfn {
        sigma_buddy_free(victim.pfn, 0);
    }

    victim.dev_id = dev_id;
    victim.block = block;
    victim.pfn = pfn;
    victim.dirty = false;
    victim.valid = true;

    // Move to MRU position
    lru_remove(victim_idx);
    lru_append(victim_idx);

    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_mark_dirty(dev_id: U32, block: U64) {
    for i in 0..MAX_CACHE_PAGES {
        if UBC.pages[i].valid && UBC.pages[i].dev_id == dev_id && UBC.pages[i].block == block {
            UBC.pages[i].dirty = true;
            break;
        }
    }
}
