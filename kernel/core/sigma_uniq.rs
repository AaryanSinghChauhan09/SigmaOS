// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS coreutils - uniq
//! Deduplicates adjacent identical lines.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_UNIQ_LINES: usize = 128;
pub const MAX_LINE_LEN: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UniqLine {
    pub data: [u8; MAX_LINE_LEN],
    pub len: usize,
    pub count: usize,
}

static mut UNIQ_LINES: [UniqLine; MAX_UNIQ_LINES] = [UniqLine { data: [0; MAX_LINE_LEN], len: 0, count: 0 }; MAX_UNIQ_LINES];
static mut OUTPUT_COUNT: usize = 0;

unsafe fn strcmp(a: *const u8, b: *const u8, a_len: usize, b_len: usize) -> bool {
    if a_len != b_len { return false; }
    for i in 0..a_len {
        if *a.add(i) != *b.add(i) { return false; }
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn sigma_uniq_init() {
    OUTPUT_COUNT = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_uniq_process_line(line: *const u8, len: usize) -> SigmaI32 {
    if line.is_null() || len == 0 { return -1; }
    
    let copy_len = len.min(MAX_LINE_LEN - 1);
    
    if OUTPUT_COUNT > 0 {
        let last_idx = OUTPUT_COUNT - 1;
        if strcmp(UNIQ_LINES[last_idx].data.as_ptr(), line, UNIQ_LINES[last_idx].len, copy_len) {
            UNIQ_LINES[last_idx].count += 1;
            return 0; // successfully deduped
        }
    }
    
    if OUTPUT_COUNT >= MAX_UNIQ_LINES { return -1; }
    
    for i in 0..copy_len {
        UNIQ_LINES[OUTPUT_COUNT].data[i] = *line.add(i);
    }
    UNIQ_LINES[OUTPUT_COUNT].data[copy_len] = 0;
    UNIQ_LINES[OUTPUT_COUNT].len = copy_len;
    UNIQ_LINES[OUTPUT_COUNT].count = 1;
    OUTPUT_COUNT += 1;
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_uniq_get_line(index: usize, out_line: *mut u8, out_len: *mut usize, out_count: *mut usize) -> SigmaI32 {
    if out_line.is_null() || out_len.is_null() || out_count.is_null() || index >= OUTPUT_COUNT { return -1; }
    
    let len = UNIQ_LINES[index].len;
    for i in 0..len {
        *out_line.add(i) = UNIQ_LINES[index].data[i];
    }
    *out_line.add(len) = 0;
    *out_len = len;
    *out_count = UNIQ_LINES[index].count;
    0
}
