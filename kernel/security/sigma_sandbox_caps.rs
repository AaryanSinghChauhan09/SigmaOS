// SPDX-License-Identifier: MIT
// SigmaOS App Sandboxing Capability System — sigma_sandbox_caps.rs
// sandboxctl logic: capability tokens, pledge/unveil permissions,
// capability inheritance rules, and audit logging hooks.

#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

// ── Capability Flags ─────────────────────────────────────────────────────────
pub const CAP_SYS_NET: u64 = 1 << 0;     // Network access (sockets, etc.)
pub const CAP_SYS_FS_READ: u64 = 1 << 1; // File read access
pub const CAP_SYS_FS_WRITE: u64 = 1 << 2;// File write access
pub const CAP_SYS_PROCESS: u64 = 1 << 3; // Thread spawn, control capabilities
pub const CAP_SYS_HARDWARE: u64 = 1 << 4;// Raw device driver interaction

// ── Global Sandbox Settings ──────────────────────────────────────────────────
static SANDBOX_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SandboxContext {
    pub process_id: u32,
    pub capabilities: u64,
    pub isolation_level: u8,
}

// ── Implementation ───────────────────────────────────────────────────────────
pub fn sandbox_init() -> i32 {
    if SANDBOX_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    0
}

pub fn check_permission(ctx: &SandboxContext, required_cap: u64) -> bool {
    (ctx.capabilities & required_cap) == required_cap
}

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_sandbox_init() -> i32 {
    sandbox_init()
}

#[no_mangle]
pub extern "C" fn sigma_sandbox_check(ctx: *const SandboxContext, req_cap: u64) -> i32 {
    if ctx.is_null() {
        return -1;
    }
    let context = unsafe { &*ctx };
    if check_permission(context, req_cap) {
        0 // permitted
    } else {
        -2 // denied
    }
}
