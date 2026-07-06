// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS coreutils - awk
//! Basic text processing and field extraction.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_AWK_FIELDS: usize = 16;
pub const MAX_FIELD_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AwkField {
    pub data: [u8; MAX_FIELD_LEN],
    pub len: usize,
}

#[no_mangle]
pub unsafe extern "C" fn sigma_awk_split(
    line: *const u8, line_len: usize, 
    delim: u8, 
    out_fields: *mut AwkField, out_count: *mut usize
) -> SigmaI32 {
    if line.is_null() || out_fields.is_null() || out_count.is_null() || line_len == 0 { return -1; }
    
    let mut count = 0;
    let mut current_field = &mut (*out_fields.add(count));
    current_field.len = 0;
    
    for i in 0..line_len {
        let c = *line.add(i);
        if c == delim {
            current_field.data[current_field.len] = 0;
            count += 1;
            if count >= MAX_AWK_FIELDS { break; }
            current_field = &mut (*out_fields.add(count));
            current_field.len = 0;
        } else {
            if current_field.len < MAX_FIELD_LEN - 1 {
                current_field.data[current_field.len] = c;
                current_field.len += 1;
            }
        }
    }
    
    // Finish last field
    if count < MAX_AWK_FIELDS {
        current_field.data[current_field.len] = 0;
        count += 1;
    }
    
    *out_count = count;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_awk_print_field(
    fields: *const AwkField, count: usize, index: usize, 
    out_buf: *mut u8, out_max: usize, out_len: *mut usize
) -> SigmaI32 {
    if fields.is_null() || out_buf.is_null() || out_len.is_null() || out_max == 0 { return -1; }
    
    // 1-based index (0 means entire line usually, but we assume 1-based here)
    if index == 0 || index > count {
        *out_len = 0;
        if out_max > 0 { *out_buf = 0; }
        return -1;
    }
    
    let field = &*fields.add(index - 1);
    let copy_len = field.len.min(out_max - 1);
    
    for i in 0..copy_len {
        *out_buf.add(i) = field.data[i];
    }
    *out_buf.add(copy_len) = 0;
    *out_len = copy_len;
    
    0
}
