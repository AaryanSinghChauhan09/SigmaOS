// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS coreutils - sed
//! Basic stream editor (s/target/replacement/ logic).
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_LINE_LEN: usize = 256;

#[no_mangle]
pub unsafe extern "C" fn sigma_sed_replace(
    line: *const u8, line_len: usize,
    target: *const u8, target_len: usize,
    repl: *const u8, repl_len: usize,
    out_buf: *mut u8, out_max: usize,
    out_len: *mut usize
) -> SigmaI32 {
    if line.is_null() || target.is_null() || repl.is_null() || out_buf.is_null() || out_len.is_null() { return -1; }
    if line_len == 0 || target_len == 0 || out_max == 0 { return -1; }

    let mut i = 0;
    let mut o = 0;

    while i < line_len {
        // Check for match
        let mut matches = false;
        if i + target_len <= line_len {
            matches = true;
            for j in 0..target_len {
                if *line.add(i + j) != *target.add(j) {
                    matches = false;
                    break;
                }
            }
        }

        if matches {
            // Write replacement
            for j in 0..repl_len {
                if o < out_max - 1 {
                    *out_buf.add(o) = *repl.add(j);
                    o += 1;
                }
            }
            i += target_len;
        } else {
            // Write original
            if o < out_max - 1 {
                *out_buf.add(o) = *line.add(i);
                o += 1;
            }
            i += 1;
        }
    }

    *out_buf.add(o) = 0;
    *out_len = o;
    0
}
