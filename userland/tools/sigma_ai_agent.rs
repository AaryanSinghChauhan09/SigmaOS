// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS AI Agent
//! Natural Language to CLI command translator stub.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

/// A simple rule-based intent matcher since we don't have a real LLM in the kernel.
/// In the full system, this would interface with a local llama.cpp instance via IPC.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IntentRule {
    pub keyword: [u8; 16],
    pub command: [u8; 32],
}

static RULES: [IntentRule; 4] = [
    IntentRule {
        keyword: *b"list files\0\0\0\0\0\0",
        command: *b"ls -la\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    },
    IntentRule {
        keyword: *b"ip address\0\0\0\0\0\0",
        command: *b"ifconfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    },
    IntentRule {
        keyword: *b"free memory\0\0\0\0\0",
        command: *b"free -h\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    },
    IntentRule {
        keyword: *b"running process\0",
        command: *b"ps aux\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    },
];

unsafe fn contains_keyword(query: *const u8, qlen: usize, keyword: *const u8) -> bool {
    let mut klen = 0;
    while klen < 16 && *keyword.add(klen) != 0 { klen += 1; }
    
    if qlen < klen { return false; }
    
    for i in 0..=(qlen - klen) {
        let mut matches = true;
        for j in 0..klen {
            // Very naive case-insensitive comparison (works for ascii lower/upper)
            let mut c1 = *query.add(i + j);
            let c2 = *keyword.add(j);
            if c1 >= b'A' && c1 <= b'Z' { c1 += 32; }
            if c1 != c2 {
                matches = false;
                break;
            }
        }
        if matches { return true; }
    }
    false
}

unsafe fn copy_cmd(dst: *mut u8, dst_len: usize, src: *const u8) {
    let mut i = 0;
    while i < 32 && i < (dst_len - 1) && *src.add(i) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    *dst.add(i) = 0;
}

/// Translates a natural language query into a CLI command.
/// Returns 0 on success (match found), -1 on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_ai_agent_translate(query: *const u8, qlen: usize, out_cmd: *mut u8, cmd_len: usize) -> SigmaI32 {
    if query.is_null() || out_cmd.is_null() || qlen == 0 || cmd_len == 0 { return -1; }
    
    for rule in &RULES {
        if contains_keyword(query, qlen, rule.keyword.as_ptr()) {
            copy_cmd(out_cmd, cmd_len, rule.command.as_ptr());
            return 0;
        }
    }
    
    // Fallback if no match
    let fallback = b"echo 'Sorry, I did not understand that.'\0";
    let mut i = 0;
    while i < fallback.len() && i < (cmd_len - 1) {
        *out_cmd.add(i) = fallback[i];
        i += 1;
    }
    *out_cmd.add(i) = 0;
    -1
}
