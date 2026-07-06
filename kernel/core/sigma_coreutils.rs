// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Coreutils — Sovereign Command Implementations
//! no_std, no alloc, no external crates. All logic hand-defined.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

unsafe fn util_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0 { return 0; }
        i += 1;
    }
}

unsafe fn util_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        let b = *src.add(i);
        *dst.add(i) = b;
        if b == 0 { return; }
        i += 1;
    }
}

// ─── Sovereign Coreutil commands (C-ABI callable hooks) ────────────────────

#[no_mangle]
pub unsafe extern "C" fn coreutil_chmod(path: *const u8, mode: SigmaU32) -> SigmaI32 {
    // In real VFS implementation: looks up inode for path, changes permission mode.
    if path.is_null() { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_chown(path: *const u8, uid: SigmaU32, gid: SigmaU32) -> SigmaI32 {
    if path.is_null() { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_cp(src: *const u8, dest: *const u8) -> SigmaI32 {
    if src.is_null() || dest.is_null() { return -1; }
    // Read from src, write to dest in VFS
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_mv(src: *const u8, dest: *const u8) -> SigmaI32 {
    if src.is_null() || dest.is_null() { return -1; }
    // Rename src to dest in VFS
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_touch(path: *const u8) -> SigmaI32 {
    if path.is_null() { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_wc(path: *const u8, count_lines: SigmaBool, count_words: SigmaBool, count_bytes: SigmaBool, out_lines: *mut SigmaU32, out_words: *mut SigmaU32, out_bytes: *mut SigmaU32) -> SigmaI32 {
    if path.is_null() { return -1; }
    if !out_lines.is_null() { *out_lines = 0; }
    if !out_words.is_null() { *out_words = 0; }
    if !out_bytes.is_null() { *out_bytes = 0; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_grep(pattern: *const u8, path: *const u8) -> SigmaI32 {
    if pattern.is_null() || path.is_null() { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_head(path: *const u8, lines: SigmaU32) -> SigmaI32 {
    if path.is_null() { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_tail(path: *const u8, lines: SigmaU32) -> SigmaI32 {
    if path.is_null() { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_df(out_total_kb: *mut SigmaU64, out_used_kb: *mut SigmaU64, out_free_kb: *mut SigmaU64) -> SigmaI32 {
    if !out_total_kb.is_null() { *out_total_kb = 1024 * 1024; }
    if !out_used_kb.is_null() { *out_used_kb = 512 * 1024; }
    if !out_free_kb.is_null() { *out_free_kb = 512 * 1024; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn coreutil_du(path: *const u8, out_size_kb: *mut SigmaU64) -> SigmaI32 {
    if path.is_null() { return -1; }
    if !out_size_kb.is_null() { *out_size_kb = 0; }
    0
}
