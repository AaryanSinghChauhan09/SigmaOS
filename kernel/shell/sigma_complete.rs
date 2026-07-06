// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Shell Tab Completion Engine
//! Minimal prefix matching for built-ins.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

// Hardcoded list of shell built-ins and coreutils for completion
static BUILTINS: [&[u8]; 14] = [
    b"cd\0", b"ls\0", b"cat\0", b"echo\0", b"pwd\0",
    b"grep\0", b"wc\0", b"df\0", b"du\0", b"cp\0",
    b"mv\0", b"touch\0", b"chmod\0", b"chown\0",
];

unsafe fn copy_bytes(dst: *mut u8, dst_len: usize, src: *const u8, src_max: usize) {
    let mut i = 0;
    while i < src_max && i < (dst_len - 1) && *src.add(i) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    *dst.add(i) = 0;
}

/// Autocompletes a command prefix. 
/// Returns 1 if a match is found, filling out_match. Returns 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn sigma_complete_cmd(prefix: *const u8, prefix_len: usize, out_match: *mut u8, match_len: usize) -> SigmaI32 {
    if prefix.is_null() || out_match.is_null() || prefix_len == 0 || match_len == 0 { return 0; }

    for cmd in &BUILTINS {
        // cmd.len() includes null terminator, so real length is cmd.len() - 1
        let real_len = cmd.len() - 1;
        if prefix_len <= real_len {
            let mut matches = true;
            for i in 0..prefix_len {
                if *prefix.add(i) != cmd[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                copy_bytes(out_match, match_len, cmd.as_ptr(), real_len);
                return 1;
            }
        }
    }
    0
}
