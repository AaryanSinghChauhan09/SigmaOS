// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS coreutils - xargs
//! Builds and executes command lines from standard input.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_ARGS: usize = 16;
pub const MAX_ARG_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XargsState {
    pub base_cmd: [u8; 32],
    pub args: [[u8; MAX_ARG_LEN]; MAX_ARGS],
    pub arg_count: usize,
}

#[no_mangle]
pub unsafe extern "C" fn sigma_xargs_init(state: *mut XargsState, base_cmd: *const u8) -> SigmaI32 {
    if state.is_null() || base_cmd.is_null() { return -1; }
    
    let st = &mut *state;
    st.arg_count = 0;
    
    let mut i = 0;
    while i < 31 && *base_cmd.add(i) != 0 {
        st.base_cmd[i] = *base_cmd.add(i);
        i += 1;
    }
    st.base_cmd[i] = 0;
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_xargs_add_arg(state: *mut XargsState, arg: *const u8, arg_len: usize) -> SigmaI32 {
    if state.is_null() || arg.is_null() || arg_len == 0 { return -1; }
    
    let st = &mut *state;
    if st.arg_count >= MAX_ARGS { return -1; } // Flush needed
    
    let copy_len = arg_len.min(MAX_ARG_LEN - 1);
    for i in 0..copy_len {
        st.args[st.arg_count][i] = *arg.add(i);
    }
    st.args[st.arg_count][copy_len] = 0;
    st.arg_count += 1;
    
    0
}

/// Formats the final command string into `out_cmd`
#[no_mangle]
pub unsafe extern "C" fn sigma_xargs_build_cmd(state: *mut XargsState, out_cmd: *mut u8, max_len: usize) -> SigmaI32 {
    if state.is_null() || out_cmd.is_null() || max_len == 0 { return -1; }
    
    let st = &mut *state;
    let mut offset = 0;
    
    // Copy base cmd
    let mut i = 0;
    while i < 32 && st.base_cmd[i] != 0 && offset < max_len - 1 {
        *out_cmd.add(offset) = st.base_cmd[i];
        offset += 1;
        i += 1;
    }
    
    // Copy args
    for a in 0..st.arg_count {
        if offset < max_len - 1 {
            *out_cmd.add(offset) = b' ';
            offset += 1;
        }
        let mut j = 0;
        while j < MAX_ARG_LEN && st.args[a][j] != 0 && offset < max_len - 1 {
            *out_cmd.add(offset) = st.args[a][j];
            offset += 1;
            j += 1;
        }
    }
    
    *out_cmd.add(offset) = 0;
    st.arg_count = 0; // Reset for next batch
    
    0
}
