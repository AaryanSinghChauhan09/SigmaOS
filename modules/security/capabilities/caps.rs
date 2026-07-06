#![no_std]
#![allow(dead_code)]

/// SigmaOS Security Capabilities (Pledge-inspired)
/// Restricts process capabilities at runtime similar to OpenBSD's pledge(2).

use core::sync::atomic::{AtomicU32, Ordering};

// ── Capability Flags (Promises) ───────────────────────────────────────────
pub const PROMISE_STDIO: u32 = 1 << 0;  // Basic I/O (read/write on fd 0,1,2)
pub const PROMISE_RPATH: u32 = 1 << 1;  // Read files
pub const PROMISE_WPATH: u32 = 1 << 2;  // Write files
pub const PROMISE_CPATH: u32 = 1 << 3;  // Create/delete files
pub const PROMISE_DPATH: u32 = 1 << 4;  // Create/delete directories
pub const PROMISE_INET:  u32 = 1 << 5;  // IPv4/IPv6 sockets
pub const PROMISE_UNIX:  u32 = 1 << 6;  // Unix sockets
pub const PROMISE_EXEC:  u32 = 1 << 7;  // execve()
pub const PROMISE_PROC:  u32 = 1 << 8;  // fork(), waitpid()
pub const PROMISE_FATAL: u32 = 1 << 31; // Kill process on violation

const MAX_TASKS: usize = 512;

/// Global capability tracking array.
/// Maps PID to current capability mask.
static PLEDGE_TABLE: [AtomicU32; MAX_TASKS] = {
    // Rust 1.x allows initializing array of atomics like this
    const INIT_ATOMIC: AtomicU32 = AtomicU32::new(0xFFFF_FFFF); // All caps by default
    [INIT_ATOMIC; 512] // Workaround for older rust const eval
};

// Custom initialization for PLEDGE_TABLE in no_std without const array repetition.
// The above works in modern rust.

#[no_mangle]
pub unsafe extern "C" fn sigma_pledge_init() {
    for i in 0..MAX_TASKS {
        PLEDGE_TABLE[i].store(0xFFFF_FFFF, Ordering::Relaxed);
    }
}

/// Apply a pledge mask to the given PID.
/// A process can only *reduce* its capabilities, never gain them back.
#[no_mangle]
pub unsafe extern "C" fn sigma_pledge(pid: u32, promises: u32) -> i32 {
    if pid as usize >= MAX_TASKS {
        return -1; // EINVAL
    }
    
    let current = PLEDGE_TABLE[pid as usize].load(Ordering::Relaxed);
    
    // You can only drop privileges, you cannot add ones you don't have.
    // So new promises must be a subset of current promises.
    if (promises & !current) != 0 {
        return -13; // EACCES
    }
    
    PLEDGE_TABLE[pid as usize].store(promises, Ordering::Relaxed);
    0
}

/// Check if a process has a specific capability.
/// Returns 1 if permitted, 0 if denied.
#[no_mangle]
pub unsafe extern "C" fn sigma_pledge_check(pid: u32, required_promise: u32) -> u32 {
    if pid as usize >= MAX_TASKS {
        return 0; // Denied by default if invalid PID
    }
    
    let current = PLEDGE_TABLE[pid as usize].load(Ordering::Relaxed);
    if (current & required_promise) == required_promise {
        1
    } else {
        0
    }
}
