// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Pre-emptive Read-Ahead Engine
//! Detects sequential access patterns and issues speculative block prefetches.
//! no_std, no alloc. Integrates with sigma_ubc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;

pub const RA_MAX_FILES: usize = 32;
pub const RA_WINDOW_MAX: SigmaU64 = 128;  // max blocks to prefetch ahead

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReadAheadState {
    pub dev_id:       SigmaU32,
    pub file_id:      SigmaU64,
    pub last_block:   SigmaU64,
    pub window_size:  SigmaU64,  // current adaptive window (blocks)
    pub sequential:   bool,      // detected sequential pattern
    pub active:       bool,
}

static mut RA_TABLE: [ReadAheadState; RA_MAX_FILES] = [ReadAheadState {
    dev_id: 0, file_id: 0, last_block: 0, window_size: 4,
    sequential: false, active: false,
}; RA_MAX_FILES];

unsafe fn ra_find(dev_id: SigmaU32, file_id: SigmaU64) -> usize {
    for i in 0..RA_MAX_FILES {
        if RA_TABLE[i].active && RA_TABLE[i].dev_id == dev_id && RA_TABLE[i].file_id == file_id {
            return i;
        }
    }
    usize::MAX
}

unsafe fn ra_alloc(dev_id: SigmaU32, file_id: SigmaU64) -> usize {
    for i in 0..RA_MAX_FILES {
        if !RA_TABLE[i].active {
            RA_TABLE[i] = ReadAheadState {
                dev_id, file_id, last_block: 0, window_size: 4,
                sequential: false, active: true,
            };
            return i;
        }
    }
    0  // wrap to slot 0 on overflow
}

/// Called on each block read. Returns the starting block of the prefetch window.
/// The caller should issue async reads for [prefetch_start, prefetch_start + window) blocks.
#[no_mangle]
pub unsafe extern "C" fn sigma_readahead_on_access(
    dev_id:   SigmaU32,
    file_id:  SigmaU64,
    block_no: SigmaU64,
    prefetch_start: *mut SigmaU64,
    prefetch_len:   *mut SigmaU64,
) {
    let idx = {
        let i = ra_find(dev_id, file_id);
        if i == usize::MAX { ra_alloc(dev_id, file_id) } else { i }
    };
    let st = &mut RA_TABLE[idx];

    let sequential = block_no == st.last_block + 1;
    st.sequential = sequential;

    if sequential {
        // Grow window exponentially up to max
        if st.window_size < RA_WINDOW_MAX {
            st.window_size = (st.window_size * 2).min(RA_WINDOW_MAX);
        }
    } else {
        // Random access: shrink window
        st.window_size = 4;
    }

    st.last_block = block_no;

    if !prefetch_start.is_null() { *prefetch_start = block_no + 1; }
    if !prefetch_len.is_null()   { *prefetch_len   = if sequential { st.window_size } else { 0 }; }
}

/// Close the read-ahead state for a file (file closed).
#[no_mangle]
pub unsafe extern "C" fn sigma_readahead_close(dev_id: SigmaU32, file_id: SigmaU64) {
    let idx = ra_find(dev_id, file_id);
    if idx != usize::MAX { RA_TABLE[idx].active = false; }
}
