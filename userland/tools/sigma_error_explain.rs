// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS AI Error Explainer
//! Maps common error codes (errnos, signals) to human-readable explanations.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

/// A simple dictionary mapping error codes to text.
/// Full system would use an LLM for contextual explanations.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorExpl {
    pub code: SigmaI32,
    pub short_msg: [u8; 32],
    pub explanation: [u8; 128],
    pub fix_hint: [u8; 64],
}

static EXPLANATIONS: [ErrorExpl; 4] = [
    ErrorExpl {
        code: 1, // EPERM
        short_msg: *b"Operation not permitted\0\0\0\0\0\0\0\0\0",
        explanation: *b"You tried to perform an action that requires elevated privileges (e.g., modifying system files or binding to a low port).\0\0\0\0\0\0\0\0\0",
        fix_hint: *b"Try running the command with 'sudo' or as the root user.\0\0\0\0\0\0\0",
    },
    ErrorExpl {
        code: 2, // ENOENT
        short_msg: *b"No such file or directory\0\0\0\0\0\0\0",
        explanation: *b"The path you specified does not exist. This could be a typo in the file name or the directory might have been deleted.\0\0\0\0\0\0\0\0\0",
        fix_hint: *b"Check your spelling and use 'ls' to verify the path exists.\0\0\0",
    },
    ErrorExpl {
        code: 11, // EAGAIN / EWOULDBLOCK
        short_msg: *b"Resource temporarily unavailable\0",
        explanation: *b"The system resource is currently locked or busy. A non-blocking operation could not be completed immediately.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        fix_hint: *b"Try the operation again in a few moments.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    },
    ErrorExpl {
        code: 13, // EACCES
        short_msg: *b"Permission denied\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        explanation: *b"You do not have the required read, write, or execute permissions for this file or directory based on its ownership and mode.\0\0\0",
        fix_hint: *b"Check permissions with 'ls -l' and modify with 'chmod'.\0\0\0\0\0\0\0",
    },
];

unsafe fn copy_bytes(dst: *mut u8, dst_len: usize, src: *const u8, src_max: usize) {
    let mut i = 0;
    while i < src_max && i < (dst_len - 1) && *src.add(i) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    *dst.add(i) = 0;
}

/// Looks up an error code and returns its explanation.
/// Returns 0 on success (found), -1 if not found.
#[no_mangle]
pub unsafe extern "C" fn sigma_error_explain(
    error_code: SigmaI32, 
    out_msg: *mut u8, msg_len: usize,
    out_expl: *mut u8, expl_len: usize,
    out_hint: *mut u8, hint_len: usize
) -> SigmaI32 {
    
    if out_msg.is_null() || out_expl.is_null() || out_hint.is_null() { return -1; }
    if msg_len == 0 || expl_len == 0 || hint_len == 0 { return -1; }

    for entry in &EXPLANATIONS {
        if entry.code == error_code {
            copy_bytes(out_msg, msg_len, entry.short_msg.as_ptr(), 32);
            copy_bytes(out_expl, expl_len, entry.explanation.as_ptr(), 128);
            copy_bytes(out_hint, hint_len, entry.fix_hint.as_ptr(), 64);
            return 0;
        }
    }
    
    // Generic fallback
    copy_bytes(out_msg, msg_len, b"Unknown Error\0".as_ptr(), 14);
    copy_bytes(out_expl, expl_len, b"The system encountered an error code it does not recognize.\0".as_ptr(), 60);
    copy_bytes(out_hint, hint_len, b"Check system logs or search the error code online.\0".as_ptr(), 51);
    
    -1
}
