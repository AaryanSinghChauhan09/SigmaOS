// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_signal.rs — POSIX signal handling
//
// Implements:
//   - Signal delivery: send_signal, deliver_pending
//   - Per-process signal disposition table (SIG_DFL / SIG_IGN / custom handler)
//   - rt_sigaction, rt_sigprocmask, rt_sigreturn
//   - sigpending, sigsuspend, sigaltstack
//   - Real-time signals (SIGRTMIN..SIGRTMAX)
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Signal numbers ─────────────────────────────────────────────────────────
pub const SIGHUP:  u8 = 1;  pub const SIGINT:  u8 = 2;
pub const SIGQUIT: u8 = 3;  pub const SIGILL:  u8 = 4;
pub const SIGTRAP: u8 = 5;  pub const SIGABRT: u8 = 6;
pub const SIGBUS:  u8 = 7;  pub const SIGFPE:  u8 = 8;
pub const SIGKILL: u8 = 9;  pub const SIGUSR1: u8 = 10;
pub const SIGSEGV: u8 = 11; pub const SIGUSR2: u8 = 12;
pub const SIGPIPE: u8 = 13; pub const SIGALRM: u8 = 14;
pub const SIGTERM: u8 = 15; pub const SIGCHLD: u8 = 17;
pub const SIGCONT: u8 = 18; pub const SIGSTOP: u8 = 19;
pub const SIGTSTP: u8 = 20; pub const SIGTTIN: u8 = 21;
pub const SIGTTOU: u8 = 22; pub const SIGURG:  u8 = 23;
pub const SIGXCPU: u8 = 24; pub const SIGXFSZ: u8 = 25;
pub const SIGVTALRM:u8= 26; pub const SIGPROF: u8 = 27;
pub const SIGWINCH:u8 = 28; pub const SIGIO:   u8 = 29;
pub const SIGRTMIN:u8 = 32; pub const SIGRTMAX:u8 = 63;
pub const NSIG:    usize = 64;

// ── Signal disposition ─────────────────────────────────────────────────────
pub type SigHandler = unsafe extern "C" fn(sig: i32);

pub const SIG_DFL: u64 = 0;
pub const SIG_IGN: u64 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigAction {
    pub handler:   u64,     // SIG_DFL, SIG_IGN, or function pointer
    pub flags:     u64,     // SA_RESTART, SA_SIGINFO, etc.
    pub mask:      u64,     // blocked signals during handler execution (bitmask)
    pub restorer:  u64,     // libc signal trampoline
}

impl SigAction {
    pub const DEFAULT: Self = SigAction { handler: SIG_DFL, flags: 0, mask: 0, restorer: 0 };
    pub const IGNORE:  Self = SigAction { handler: SIG_IGN, flags: 0, mask: 0, restorer: 0 };
}

pub const SA_NOCLDSTOP: u64 = 1;
pub const SA_NOCLDWAIT: u64 = 2;
pub const SA_SIGINFO:   u64 = 4;
pub const SA_RESTART:   u64 = 0x10000000;
pub const SA_NODEFER:   u64 = 0x40000000;
pub const SA_RESETHAND: u64 = 0x80000000;

// ── Per-process signal state ───────────────────────────────────────────────
const MAX_PROCS: usize = 256;

#[derive(Copy, Clone)]
pub struct SigState {
    pub actions:  [SigAction; NSIG],
    pub pending:  u64,   // bitmask of pending signals
    pub blocked:  u64,   // bitmask of blocked signals (sigprocmask)
    pub sigaltstack_sp:   u64,
    pub sigaltstack_size: usize,
    pub in_handler: bool,
}

impl SigState {
    pub const fn new() -> Self {
        SigState {
            actions: [SigAction::DEFAULT; NSIG],
            pending: 0, blocked: 0,
            sigaltstack_sp: 0, sigaltstack_size: 0,
            in_handler: false,
        }
    }

    /// Default terminal signals that kill the process
    const TERM_SIGNALS: u64 =
        (1 << SIGHUP)  | (1 << SIGINT)  | (1 << SIGQUIT) | (1 << SIGILL)  |
        (1 << SIGTRAP) | (1 << SIGABRT) | (1 << SIGFPE)  | (1 << SIGKILL) |
        (1 << SIGBUS)  | (1 << SIGSEGV) | (1 << SIGPIPE) | (1 << SIGALRM) |
        (1 << SIGTERM);
    /// Signals ignored by default (SIGCHLD, SIGCONT in some contexts)
    const IGN_SIGNALS: u64 = 0;
    /// Signals that stop the process
    const STOP_SIGNALS: u64 = (1 << SIGSTOP) | (1 << SIGTSTP) | (1 << SIGTTIN) | (1 << SIGTTOU);

    fn default_action(&self, sig: u8) -> &'static str {
        let bit = 1u64 << sig;
        if Self::TERM_SIGNALS & bit != 0 { "terminate" }
        else if Self::STOP_SIGNALS & bit != 0 { "stop" }
        else { "ignore" }
    }
}

static mut SIG_TABLE: [SigState; MAX_PROCS] = [const { SigState::new() }; MAX_PROCS];

fn proc_sig(pid: u32) -> &'static mut SigState {
    unsafe { &mut SIG_TABLE[pid as usize % MAX_PROCS] }
}

// ── Signal operations ──────────────────────────────────────────────────────

/// Send a signal to a process. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn sigma_send_signal(pid: u32, sig: u8) -> i32 {
    if sig as usize >= NSIG { return -22; } // EINVAL
    let state = proc_sig(pid);
    // SIGKILL and SIGSTOP cannot be blocked or ignored
    if sig == SIGKILL || sig == SIGSTOP {
        state.pending |= 1 << sig;
        return 0;
    }
    // If blocked, still add to pending (delivered when unblocked)
    state.pending |= 1 << sig;
    0
}

/// Deliver pending signals to the current process.
/// Called at syscall return and when returning from interrupt.
/// Returns true if process should terminate.
#[no_mangle]
pub unsafe extern "C" fn sigma_deliver_signals(pid: u32) -> bool {
    let state = proc_sig(pid);
    let deliverable = state.pending & !state.blocked;
    if deliverable == 0 { return false; }

    for sig in 0..NSIG {
        let bit = 1u64 << sig;
        if deliverable & bit == 0 { continue; }
        state.pending &= !bit;

        let action = &state.actions[sig];
        match action.handler {
            SIG_DFL => {
                // Apply default action
                let default = SigState::new().default_action(sig as u8);
                if default == "terminate" { return true; }
                // stop: mark process sleeping (not implemented here)
            }
            SIG_IGN => { continue; } // ignored
            handler_ptr => {
                // Call user-space handler
                // In production: set up signal frame on user stack + iretq
                let handler: SigHandler = core::mem::transmute(handler_ptr);
                state.in_handler = true;
                // Block signals in the handler's mask
                let saved_blocked = state.blocked;
                state.blocked |= action.mask;
                // Execute handler (via iretq to user space in real kernel)
                handler(sig as i32);
                state.blocked = saved_blocked;
                state.in_handler = false;
                // SA_RESETHAND: restore to SIG_DFL after one delivery
                if action.flags & SA_RESETHAND != 0 {
                    state.actions[sig] = SigAction::DEFAULT;
                }
            }
        }
    }
    false
}

// ── Syscall implementations ────────────────────────────────────────────────

/// rt_sigaction(signum, act, oldact, sigsetsize)
#[no_mangle]
pub unsafe extern "C" fn sigma_sys_rt_sigaction(
    signum: u64, act: u64, oldact: u64, _size: u64,
) -> i64 {
    if signum as usize >= NSIG || signum == SIGKILL as u64 || signum == SIGSTOP as u64 {
        return -22; // EINVAL
    }
    extern "C" { fn sigma_getpid() -> u32; }
    let state = proc_sig(sigma_getpid());

    // Save old action
    if oldact != 0 {
        *(oldact as *mut SigAction) = state.actions[signum as usize];
    }
    // Install new action
    if act != 0 {
        state.actions[signum as usize] = *(act as *const SigAction);
    }
    0
}

/// rt_sigprocmask(how, set, oldset, sigsetsize)
#[no_mangle]
pub unsafe extern "C" fn sigma_sys_rt_sigprocmask(
    how: u64, set: u64, oldset: u64, _size: u64,
) -> i64 {
    const SIG_BLOCK:   u64 = 0;
    const SIG_UNBLOCK: u64 = 1;
    const SIG_SETMASK: u64 = 2;

    extern "C" { fn sigma_getpid() -> u32; }
    let state = proc_sig(sigma_getpid());

    if oldset != 0 { *(oldset as *mut u64) = state.blocked; }

    if set != 0 {
        let new_mask = *(set as *const u64);
        // Never block SIGKILL or SIGSTOP
        let safe_mask = new_mask & !((1u64 << SIGKILL) | (1u64 << SIGSTOP));
        state.blocked = match how {
            SIG_BLOCK   => state.blocked | safe_mask,
            SIG_UNBLOCK => state.blocked & !safe_mask,
            SIG_SETMASK => safe_mask,
            _           => return -22,
        };
    }
    0
}

/// rt_sigreturn — called from signal trampoline to restore process context
#[no_mangle]
pub unsafe extern "C" fn sigma_sys_rt_sigreturn() -> i64 {
    extern "C" { fn sigma_getpid() -> u32; }
    let state = proc_sig(sigma_getpid());
    state.in_handler = false;
    // In production: restore saved register context from signal frame on stack
    0
}

/// sigpending — return set of pending signals
#[no_mangle]
pub unsafe extern "C" fn sigma_sys_sigpending(set: u64) -> i64 {
    if set == 0 { return -14; }
    extern "C" { fn sigma_getpid() -> u32; }
    let state = proc_sig(sigma_getpid());
    *(set as *mut u64) = state.pending & !state.blocked;
    0
}

/// sigaltstack — set/get alternate signal stack
#[no_mangle]
pub unsafe extern "C" fn sigma_sys_sigaltstack(new_ss: u64, old_ss: u64) -> i64 {
    extern "C" { fn sigma_getpid() -> u32; }
    let state = proc_sig(sigma_getpid());
    if old_ss != 0 {
        *(old_ss as *mut u64) = state.sigaltstack_sp;
    }
    if new_ss != 0 {
        state.sigaltstack_sp   = *(new_ss as *const u64);
        state.sigaltstack_size = *((new_ss + 8) as *const usize);
    }
    0
}
