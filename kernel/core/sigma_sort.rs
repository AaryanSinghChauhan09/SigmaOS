// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS coreutils - sort
//! In-memory string sort for coreutils.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_SORT_LINES: usize = 128;
pub const MAX_LINE_LEN: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SortLine {
    pub data: [u8; MAX_LINE_LEN],
    pub len: usize,
}

static mut SORT_LINES: [SortLine; MAX_SORT_LINES] = [SortLine { data: [0; MAX_LINE_LEN], len: 0 }; MAX_SORT_LINES];
static mut LINE_COUNT: usize = 0;

unsafe fn strcmp(a: *const u8, b: *const u8, a_len: usize, b_len: usize) -> i32 {
    let min_len = if a_len < b_len { a_len } else { b_len };
    for i in 0..min_len {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca < cb { return -1; }
        if ca > cb { return 1; }
    }
    if a_len < b_len { return -1; }
    if a_len > b_len { return 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sort_init() {
    LINE_COUNT = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sort_add_line(line: *const u8, len: usize) -> SigmaI32 {
    if line.is_null() || len == 0 || LINE_COUNT >= MAX_SORT_LINES { return -1; }
    
    let copy_len = len.min(MAX_LINE_LEN - 1);
    for i in 0..copy_len {
        SORT_LINES[LINE_COUNT].data[i] = *line.add(i);
    }
    SORT_LINES[LINE_COUNT].data[copy_len] = 0;
    SORT_LINES[LINE_COUNT].len = copy_len;
    LINE_COUNT += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sort_execute() {
    if LINE_COUNT <= 1 { return; }
    
    // Simple Bubble Sort (O(N^2) but N is small (128))
    for i in 0..LINE_COUNT {
        for j in 0..(LINE_COUNT - i - 1) {
            let p1 = &SORT_LINES[j] as *const SortLine;
            let p2 = &SORT_LINES[j + 1] as *const SortLine;
            
            if strcmp((*p1).data.as_ptr(), (*p2).data.as_ptr(), (*p1).len, (*p2).len) > 0 {
                let temp = SORT_LINES[j];
                SORT_LINES[j] = SORT_LINES[j + 1];
                SORT_LINES[j + 1] = temp;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sort_get_line(index: usize, out_line: *mut u8, out_len: *mut usize) -> SigmaI32 {
    if out_line.is_null() || out_len.is_null() || index >= LINE_COUNT { return -1; }
    
    let len = SORT_LINES[index].len;
    for i in 0..len {
        *out_line.add(i) = SORT_LINES[index].data[i];
    }
    *out_line.add(len) = 0;
    *out_len = len;
    0
}
