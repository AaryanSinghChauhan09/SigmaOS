// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/posix_signal.rs — POSIX Signals & IPC System
//
// Implements POSIX signal handling and IPC: sigaction, sigprocmask, pipe
// Redesigned with OOP principles to coexist with SigmaOS native message-passing.
//
// Language: Rust (no_std for kernel compatibility)

#![no_std]

use super::posix_base::{
    set_errno_and_return, clear_errno, EINVAL, EFAULT, ENOMEM,
    SIGINT, SIGTERM, SIGKILL, SIGCHLD, SIGSTOP, SIGCONT, SIGHUP, SIGQUIT, SIGILL,
    SIGTRAP, SIGABRT, SIGBUS, SIGFPE, SIGUSR1, SIGSEGV, SIGUSR2, SIGPIPE, SIGALRM,
};

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type Isize = isize;
type Usize = usize;

// ─── Signal Action Flags ─────────────────────────────────

pub const SA_NOCLDSTOP: U32 = 0x00000001;
pub const SA_NOCLDWAIT: U32 = 0x00000002;
pub const SA_SIGINFO: U32 = 0x00000004;
pub const SA_ONSTACK: U32 = 0x08000000;
pub const SA_RESTART: U32 = 0x10000000;
pub const SA_RESETHAND: U32 = 0x80000000;
pub const SA_NODEFER: U32 = 0x40000000;

// ─── Signal Mask ─────────────────────────────────────────

pub const SIG_BLOCK: I32 = 0;
pub const SIG_UNBLOCK: I32 = 1;
pub const SIG_SETMASK: I32 = 2;

// ─── Signal Handler Types ───────────────────────────────

pub type SigHandler = extern "C" fn(I32);
pub type SigActionHandler = extern "C" fn(I32, *mut SigInfo, *mut U8);

// ─── SigInfo Structure ───────────────────────────────────

#[repr(C)]
pub struct SigInfo {
    pub si_signo: I32,
    pub si_errno: I32,
    pub si_code: I32,
    pub si_pid: I32,
    pub si_uid: U32,
    pub si_addr: U64,
    pub si_status: I32,
    pub si_value: I32,
}

impl SigInfo {
    pub const fn new() -> Self {
        SigInfo {
            si_signo: 0,
            si_errno: 0,
            si_code: 0,
            si_pid: 0,
            si_uid: 0,
            si_addr: 0,
            si_status: 0,
            si_value: 0,
        }
    }
}

// ─── SigAction Structure ────────────────────────────────

#[repr(C)]
pub struct SigAction {
    pub sa_handler: SigHandler,
    pub sa_sigaction: SigActionHandler,
    pub sa_mask: U64,
    pub sa_flags: U32,
    pub sa_restorer: extern "C" fn(),
}

impl SigAction {
    pub const fn new() -> Self {
        SigAction {
            sa_handler: default_sig_handler,
            sa_sigaction: default_sigaction_handler,
            sa_mask: 0,
            sa_flags: 0,
            sa_restorer: default_sig_restorer,
        }
    }
}

// ─── Default Signal Handlers ───────────────────────────

extern "C" fn default_sig_handler(sig: I32) {
    // Default signal handler - terminate process
    unsafe { super::posix_process::posix_exit(128 + sig); }
}

extern "C" fn default_sigaction_handler(sig: I32, info: *mut SigInfo, context: *mut U8) {
    // Default sigaction handler - terminate process
    unsafe { super::posix_process::posix_exit(128 + sig); }
}

extern "C" fn default_sig_restorer() {
    // Default signal restorer
}

// ─── Signal Action Table ───────────────────────────────

pub const MAX_SIGNALS: Usize = 32;

pub struct SignalActionTable {
    pub actions: [SigAction; MAX_SIGNALS],
    pub signal_mask: U64,
}

impl SignalActionTable {
    pub const fn new() -> Self {
        SignalActionTable {
            actions: [SigAction::new(); MAX_SIGNALS],
            signal_mask: 0,
        }
    }

    pub fn get_action(&mut self, sig: I32) -> Option<&mut SigAction> {
        if sig > 0 && (sig as Usize) < MAX_SIGNALS {
            Some(&mut self.actions[sig as Usize])
        } else {
            None
        }
    }

    pub fn set_signal_mask(&mut self, mask: U64) {
        self.signal_mask = mask;
    }

    pub fn get_signal_mask(&self) -> U64 {
        self.signal_mask
    }

    pub fn block_signal(&mut self, sig: I32) {
        if sig > 0 && (sig as Usize) < 64 {
            self.signal_mask |= 1u64 << (sig as Usize);
        }
    }

    pub fn unblock_signal(&mut self, sig: I32) {
        if sig > 0 && (sig as Usize) < 64 {
            self.signal_mask &= !(1u64 << (sig as Usize));
        }
    }

    pub fn is_signal_blocked(&self, sig: I32) -> bool {
        if sig > 0 && (sig as Usize) < 64 {
            (self.signal_mask & (1u64 << (sig as Usize))) != 0
        } else {
            false
        }
    }
}

// ─── Global Signal Action Table ───────────────────────

static mut SIGNAL_ACTION_TABLE: SignalActionTable = SignalActionTable::new();

// ─── SigmaOS Signal Operations (stubs) ─────────────────

// These would call into SigmaOS's signal manager
// For now, we provide stub implementations

unsafe fn sigma_signal_send(pid: I32, sig: I32) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS signal manager
    Ok(())
}

unsafe fn sigma_signal_register(sig: I32, action: *const SigAction) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS signal manager
    Ok(())
}

// ─── POSIX sigaction() ─────────────────────────────────

/// Set signal handler
#[no_mangle]
pub unsafe extern "C" fn posix_sigaction(sig: I32, act: *const SigAction, oldact: *mut SigAction) -> I32 {
    clear_errno();

    // Validate signal number
    if sig <= 0 || sig >= MAX_SIGNALS as I32 {
        return set_errno_and_return(EINVAL);
    }

    let signal_table = &mut SIGNAL_ACTION_TABLE;

    // Get old action if requested
    if !oldact.is_null() {
        if let Some(action) = signal_table.get_action(sig) {
            *oldact = *action;
        }
    }

    // Set new action if provided
    if !act.is_null() {
        if let Some(action) = signal_table.get_action(sig) {
            *action = *act;
        }
    }

    0
}

// ─── POSIX signal() ───────────────────────────────────

/// Set signal handler (simplified version of sigaction)
#[no_mangle]
pub unsafe extern "C" fn posix_signal(sig: I32, handler: SigHandler) -> SigHandler {
    let mut old_action = SigAction::new();
    let mut new_action = SigAction::new();
    
    new_action.sa_handler = handler;
    
    posix_sigaction(sig, &new_action, &mut old_action);
    
    old_action.sa_handler
}

// ─── POSIX sigprocmask() ───────────────────────────────

/// Set signal mask
#[no_mangle]
pub unsafe extern "C" fn posix_sigprocmask(how: I32, set: *const U64, oldset: *mut U64) -> I32 {
    clear_errno();

    let signal_table = &mut SIGNAL_ACTION_TABLE;

    // Get old mask if requested
    if !oldset.is_null() {
        *oldset = signal_table.get_signal_mask();
    }

    // Set new mask if provided
    if !set.is_null() {
        let mask = *set;
        
        match how {
            SIG_BLOCK => {
                signal_table.signal_mask |= mask;
            }
            SIG_UNBLOCK => {
                signal_table.signal_mask &= !mask;
            }
            SIG_SETMASK => {
                signal_table.signal_mask = mask;
            }
            _ => {
                return set_errno_and_return(EINVAL);
            }
        }
    }

    0
}

// ─── POSIX sigemptyset() ───────────────────────────────

/// Initialize signal set to empty
#[no_mangle]
pub unsafe extern "C" fn posix_sigemptyset(set: *mut U64) -> I32 {
    if set.is_null() {
        return set_errno_and_return(EFAULT);
    }

    *set = 0;
    0
}

// ─── POSIX sigfillset() ────────────────────────────────

/// Initialize signal set to full
#[no_mangle]
pub unsafe extern "C" fn posix_sigfillset(set: *mut U64) -> I32 {
    if set.is_null() {
        return set_errno_and_return(EFAULT);
    }

    *set = 0xFFFFFFFFFFFFFFFF;
    0
}

// ─── POSIX sigaddset() ────────────────────────────────

/// Add signal to set
#[no_mangle]
pub unsafe extern "C" fn posix_sigaddset(set: *mut U64, sig: I32) -> I32 {
    if set.is_null() {
        return set_errno_and_return(EFAULT);
    }

    if sig <= 0 || sig >= 64 {
        return set_errno_and_return(EINVAL);
    }

    *set |= 1u64 << (sig as Usize);
    0
}

// ─── POSIX sigdelset() ────────────────────────────────

/// Remove signal from set
#[no_mangle]
pub unsafe extern "C" fn posix_sigdelset(set: *mut U64, sig: I32) -> I32 {
    if set.is_null() {
        return set_errno_and_return(EFAULT);
    }

    if sig <= 0 || sig >= 64 {
        return set_errno_and_return(EINVAL);
    }

    *set &= !(1u64 << (sig as Usize));
    0
}

// ─── POSIX sigismember() ───────────────────────────────

/// Check if signal is in set
#[no_mangle]
pub unsafe extern "C" fn posix_sigismember(set: *const U64, sig: I32) -> I32 {
    if set.is_null() {
        return set_errno_and_return(EFAULT) as I32;
    }

    if sig <= 0 || sig >= 64 {
        return set_errno_and_return(EINVAL) as I32;
    }

    if (*set & (1u64 << (sig as Usize))) != 0 {
        1
    } else {
        0
    }
}

// ─── POSIX sigpending() ────────────────────────────────

/// Get pending signals
#[no_mangle]
pub unsafe extern "C" fn posix_sigpending(set: *mut U64) -> I32 {
    if set.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Stub: In real implementation, return pending signals
    *set = 0;
    0
}

// ─── POSIX sigsuspend() ────────────────────────────────

/// Wait for signal
#[no_mangle]
pub unsafe extern "C" fn posix_sigsuspend(mask: *const U64) -> I32 {
    if mask.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Stub: In real implementation, wait for signal
    set_errno_and_return(EINTR)
}

// ─── POSIX sigwait() ───────────────────────────────────

/// Wait for signal synchronously
#[no_mangle]
pub unsafe extern "C" fn posix_sigwait(set: *const U64, sig: *mut I32) -> I32 {
    if set.is_null() || sig.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Stub: In real implementation, wait for signal
    *sig = 0;
    0
}

// ─── IPC: Pipe ─────────────────────────────────────────

#[repr(C)]
pub struct PipeFd {
    pub read_fd: I32,
    pub write_fd: I32,
}

// ─── SigmaOS Pipe Operations (stubs) ───────────────────

unsafe fn sigma_pipe_create() -> Result<(I32, I32), I32> {
    // Stub: In real implementation, this would call SigmaOS IPC manager
    Ok((3, 4)) // Return read and write file descriptors
}

unsafe fn sigma_pipe_close(fd: I32) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS IPC manager
    Ok(())
}

// ─── POSIX pipe() ─────────────────────────────────────

/// Create a pipe
#[no_mangle]
pub unsafe extern "C" fn posix_pipe(fds: *mut I32) -> I32 {
    clear_errno();

    if fds.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Call SigmaOS pipe create
    match sigma_pipe_create() {
        Ok((read_fd, write_fd)) => {
            *fds = read_fd;
            *fds.add(1) = write_fd;
            0
        }
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX pipe2() ───────────────────────────────────

/// Create a pipe with flags
#[no_mangle]
pub unsafe extern "C" fn posix_pipe2(fds: *mut I32, flags: I32) -> I32 {
    clear_errno();

    if fds.is_null() {
        return set_errno_and_return(EFAULT);
    }

    // Call SigmaOS pipe create (flags ignored in stub)
    match sigma_pipe_create() {
        Ok((read_fd, write_fd)) => {
            *fds = read_fd;
            *fds.add(1) = write_fd;
            0
        }
        Err(e) => set_errno_and_return(e),
    }
}

// ─── C-ABI Wrappers ───────────────────────────────────

#[no_mangle]
pub extern "C" fn sigaction(sig: I32, act: *const SigAction, oldact: *mut SigAction) -> I32 {
    unsafe { posix_sigaction(sig, act, oldact) }
}

#[no_mangle]
pub extern "C" fn signal(sig: I32, handler: SigHandler) -> SigHandler {
    unsafe { posix_signal(sig, handler) }
}

#[no_mangle]
pub extern "C" fn sigprocmask(how: I32, set: *const U64, oldset: *mut U64) -> I32 {
    unsafe { posix_sigprocmask(how, set, oldset) }
}

#[no_mangle]
pub extern "C" fn sigemptyset(set: *mut U64) -> I32 {
    unsafe { posix_sigemptyset(set) }
}

#[no_mangle]
pub extern "C" fn sigfillset(set: *mut U64) -> I32 {
    unsafe { posix_sigfillset(set) }
}

#[no_mangle]
pub extern "C" fn sigaddset(set: *mut U64, sig: I32) -> I32 {
    unsafe { posix_sigaddset(set, sig) }
}

#[no_mangle]
pub extern "C" fn sigdelset(set: *mut U64, sig: I32) -> I32 {
    unsafe { posix_sigdelset(set, sig) }
}

#[no_mangle]
pub extern "C" fn sigismember(set: *const U64, sig: I32) -> I32 {
    unsafe { posix_sigismember(set, sig) }
}

#[no_mangle]
pub extern "C" fn sigpending(set: *mut U64) -> I32 {
    unsafe { posix_sigpending(set) }
}

#[no_mangle]
pub extern "C" fn sigsuspend(mask: *const U64) -> I32 {
    unsafe { posix_sigsuspend(mask) }
}

#[no_mangle]
pub extern "C" fn sigwait(set: *const U64, sig: *mut I32) -> I32 {
    unsafe { posix_sigwait(set, sig) }
}

#[no_mangle]
pub extern "C" fn pipe(fds: *mut I32) -> I32 {
    unsafe { posix_pipe(fds) }
}

#[no_mangle]
pub extern "C" fn pipe2(fds: *mut I32, flags: I32) -> I32 {
    unsafe { posix_pipe2(fds, flags) }
}
