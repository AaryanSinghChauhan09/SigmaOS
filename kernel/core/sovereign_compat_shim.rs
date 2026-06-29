// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Compatibility Shim (Rust, no_std)
//! Replaces: kernel/core/SovereignCompatShim.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const LINUX_SYS_READ: u32 = 0;
pub const LINUX_SYS_WRITE: u32 = 1;
pub const LINUX_SYS_OPEN: u32 = 2;
pub const LINUX_SYS_CLOSE: u32 = 3;
pub const LINUX_SYS_MMAP: u32 = 9;

pub const K_ERR_PERM: i64 = 1; // Used positively inside negative returns
pub const SIGMA_ERROR: i64 = 2;

extern "C" {
    fn sandbox_check_syscall(syscall_no: u32) -> bool;
    fn sigma_log(s: *const u8);
    fn sigma_log_info(fmt: *const u8, val1: u32, val2: *const u8, val3: u32, val4: u32);
}

pub struct SovereignCompatShim;

impl SovereignCompatShim {
    pub const fn new() -> Self {
        Self
    }
}

static COMPAT_SHIM: SovereignCompatShim = SovereignCompatShim::new();

#[no_mangle]
pub unsafe extern "C" fn compat_shim_init() {
    sigma_log(b"[OMNIPKG-SHIM] POSIX Compatibility Layer initialized (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn compat_shim_execute_syscall(
    container_id: u32,
    sys_no: u32,
    a1: u64,
    a2: u64,
    a3: u64,
) -> i64 {
    if !sandbox_check_syscall(sys_no) {
        return -K_ERR_PERM;
    }

    match sys_no {
        LINUX_SYS_READ => a3 as i64, // Mock read bytes
        LINUX_SYS_WRITE => a3 as i64, // Mock written bytes
        LINUX_SYS_OPEN => 4, // Mock file descriptor
        LINUX_SYS_CLOSE => 0, // Success
        LINUX_SYS_MMAP => 0x7FFFF7A00000i64, // Mock address
        _ => -SIGMA_ERROR, // Deny unsupported syscalls
    }
}
