// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Shell Environment Manager
//! Env vars + command history.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_ENV_VARS: usize = 64;
pub const MAX_HISTORY: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EnvVar {
    pub key: [u8; 32],
    pub val: [u8; 128],
    pub active: bool,
}

static mut ENV_VARS: [EnvVar; MAX_ENV_VARS] = [EnvVar {
    key: [0; 32], val: [0; 128], active: false
}; MAX_ENV_VARS];

static mut HISTORY: [[u8; 128]; MAX_HISTORY] = [[0; 128]; MAX_HISTORY];
static mut HISTORY_COUNT: usize = 0;

unsafe fn copy_bytes(dst: *mut u8, dst_len: usize, src: *const u8, src_max: usize) {
    let mut i = 0;
    while i < src_max && i < (dst_len - 1) && *src.add(i) != 0 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    *dst.add(i) = 0;
}

unsafe fn c_str_match(s1: *const u8, s2: *const u8) -> bool {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 { return false; }
        if c1 == 0 { return true; }
        i += 1;
        if i >= 32 { return true; }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_env_set(key: *const u8, val: *const u8) -> SigmaI32 {
    if key.is_null() || val.is_null() { return -1; }
    
    // Check if exists
    for i in 0..MAX_ENV_VARS {
        if ENV_VARS[i].active && c_str_match(ENV_VARS[i].key.as_ptr(), key) {
            copy_bytes(ENV_VARS[i].val.as_mut_ptr(), 128, val, 128);
            return 0;
        }
    }
    
    // Add new
    for i in 0..MAX_ENV_VARS {
        if !ENV_VARS[i].active {
            copy_bytes(ENV_VARS[i].key.as_mut_ptr(), 32, key, 32);
            copy_bytes(ENV_VARS[i].val.as_mut_ptr(), 128, val, 128);
            ENV_VARS[i].active = true;
            return 0;
        }
    }
    -1 // Table full
}

#[no_mangle]
pub unsafe extern "C" fn sigma_env_get(key: *const u8, out_val: *mut u8, val_len: usize) -> SigmaI32 {
    if key.is_null() || out_val.is_null() || val_len == 0 { return -1; }
    
    for i in 0..MAX_ENV_VARS {
        if ENV_VARS[i].active && c_str_match(ENV_VARS[i].key.as_ptr(), key) {
            copy_bytes(out_val, val_len, ENV_VARS[i].val.as_ptr(), 128);
            return 0;
        }
    }
    -1 // Not found
}

#[no_mangle]
pub unsafe extern "C" fn sigma_history_add(cmd: *const u8) {
    if cmd.is_null() { return; }
    
    // Shift history back
    for i in (1..MAX_HISTORY).rev() {
        HISTORY[i] = HISTORY[i - 1];
    }
    
    // Insert at front
    copy_bytes(HISTORY[0].as_mut_ptr(), 128, cmd, 128);
    
    if HISTORY_COUNT < MAX_HISTORY {
        HISTORY_COUNT += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_history_get(index: usize, out_cmd: *mut u8, cmd_len: usize) -> SigmaI32 {
    if out_cmd.is_null() || cmd_len == 0 || index >= HISTORY_COUNT { return -1; }
    
    copy_bytes(out_cmd, cmd_len, HISTORY[index].as_ptr(), 128);
    0
}
