// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Unified Buffer Cache (UBC)
//! Page-granularity cache mapping (device, block) → cached page frames.
//! Eviction policy: Clock (second-chance). no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32  = u32;
type SigmaU64  = u64;
type SigmaUsize= usize;

pub const UBC_PAGE_SIZE: usize = 4096;
pub const UBC_MAX_PAGES: usize = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UbcPage {
    pub dev_id:    SigmaU32,
    pub block_no:  SigmaU64,
    pub data:      [u8; UBC_PAGE_SIZE],
    pub valid:     bool,
    pub dirty:     bool,
    pub referenced: bool,  // clock hand bit
}

static mut UBC_CACHE: [UbcPage; UBC_MAX_PAGES] = [UbcPage {
    dev_id: 0, block_no: 0, data: [0u8; UBC_PAGE_SIZE],
    valid: false, dirty: false, referenced: false,
}; UBC_MAX_PAGES];

static mut UBC_CLOCK_HAND: usize = 0;
static mut UBC_HIT_COUNT:  SigmaU64 = 0;
static mut UBC_MISS_COUNT: SigmaU64 = 0;

/// Find a cached page; returns its index or usize::MAX.
unsafe fn ubc_find(dev_id: SigmaU32, block_no: SigmaU64) -> usize {
    for i in 0..UBC_MAX_PAGES {
        if UBC_CACHE[i].valid && UBC_CACHE[i].dev_id == dev_id && UBC_CACHE[i].block_no == block_no {
            return i;
        }
    }
    usize::MAX
}

/// Clock eviction: advance hand, skip referenced pages (give second chance).
unsafe fn ubc_evict() -> usize {
    loop {
        let hand = UBC_CLOCK_HAND % UBC_MAX_PAGES;
        if !UBC_CACHE[hand].valid {
            UBC_CLOCK_HAND = (hand + 1) % UBC_MAX_PAGES;
            return hand;
        }
        if UBC_CACHE[hand].referenced {
            UBC_CACHE[hand].referenced = false;
            UBC_CLOCK_HAND = (hand + 1) % UBC_MAX_PAGES;
        } else {
            // Evict this page (caller handles writeback if dirty)
            UBC_CACHE[hand].valid = false;
            UBC_CLOCK_HAND = (hand + 1) % UBC_MAX_PAGES;
            return hand;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_init() {
    for i in 0..UBC_MAX_PAGES { UBC_CACHE[i].valid = false; }
    UBC_CLOCK_HAND = 0;
    UBC_HIT_COUNT  = 0;
    UBC_MISS_COUNT = 0;
}

/// Look up a block. Returns a pointer to cached data, or null on miss.
#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_lookup(dev_id: SigmaU32, block_no: SigmaU64) -> *mut u8 {
    let idx = ubc_find(dev_id, block_no);
    if idx != usize::MAX {
        UBC_HIT_COUNT += 1;
        UBC_CACHE[idx].referenced = true;
        UBC_CACHE[idx].data.as_mut_ptr()
    } else {
        UBC_MISS_COUNT += 1;
        core::ptr::null_mut()
    }
}

/// Insert a block into the cache, returning a pointer to where data should be written.
#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_insert(dev_id: SigmaU32, block_no: SigmaU64) -> *mut u8 {
    // Check if already present
    let existing = ubc_find(dev_id, block_no);
    if existing != usize::MAX {
        return UBC_CACHE[existing].data.as_mut_ptr();
    }
    let slot = ubc_evict();
    UBC_CACHE[slot] = UbcPage {
        dev_id, block_no,
        data: [0u8; UBC_PAGE_SIZE],
        valid: true, dirty: false, referenced: true,
    };
    UBC_CACHE[slot].data.as_mut_ptr()
}

/// Mark a cached page as dirty (needs writeback).
#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_mark_dirty(dev_id: SigmaU32, block_no: SigmaU64) {
    let idx = ubc_find(dev_id, block_no);
    if idx != usize::MAX { UBC_CACHE[idx].dirty = true; }
}

/// Returns hit rate as hits*100/(hits+misses), or 0.
#[no_mangle]
pub unsafe extern "C" fn sigma_ubc_hit_rate_pct() -> SigmaU32 {
    let total = UBC_HIT_COUNT + UBC_MISS_COUNT;
    if total == 0 { return 0; }
    (UBC_HIT_COUNT * 100 / total) as SigmaU32
}
