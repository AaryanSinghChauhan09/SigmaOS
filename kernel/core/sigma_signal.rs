// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_signal.rs — POSIX signal delivery
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Signal numbers (Linux-compatible) ────────────────────────────────────
pub const SIGHUP:  u8 = 1;   pub const SIGINT:  u8 = 2;
pub const SIGQUIT: u8 = 3;   pub const SIGILL:  u8 = 4;
pub const SIGTRAP: u8 = 5;   pub const SIGABRT: u8 = 6;
pub const SIGBUS:  u8 = 7;   pub const SIGFPE:  u8 = 8;
pub const SIGKILL: u8 = 9;   pub const SIGUSR1: u8 = 10;
pub const SIGSEGV: u8 = 11;  pub const SIGUSR2: u8 = 12;
pub const SIGPIPE: u8 = 13;  pub const SIGALRM: u8 = 14;
pub const SIGTERM: u8 = 15;  pub const SIGCHLD: u8 = 17;
pub const SIGCONT: u8 = 18;  pub const SIGSTOP: u8 = 19;
pub const SIGTSTP: u8 = 20;  pub const SIGTTIN: u8 = 21;
pub const SIGTTOU: u8 = 22;

pub const SIG_DFL: u64 = 0;  // default action
pub const SIG_IGN: u64 = 1;  // ignore

// ── Signal handler entry ──────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct SigAction {
    pub handler: u64,   // SIG_DFL / SIG_IGN / function pointer
    pub mask:    u64,   // signals to block while handler runs
    pub flags:   u32,
}

impl SigAction {
    pub const fn default() -> Self {
        Self { handler: SIG_DFL, mask: 0, flags: 0 }
    }
}

const MAX_SIGS:     usize = 64;
const MAX_SIG_PROCS:usize = 256;

#[derive(Clone)]
struct SigState {
    pid:      u32,
    pending:  u64,        // bitmask of pending signals
    blocked:  u64,        // bitmask of blocked signals (sigprocmask)
    actions:  [SigAction; MAX_SIGS],
    active:   bool,
}

impl SigState {
    const fn new() -> Self {
        Self {
            pid: 0, pending: 0, blocked: 0,
            actions: [const { SigAction { handler: SIG_DFL, mask: 0, flags: 0 } }; MAX_SIGS],
            active: false,
        }
    }
}

pub struct SignalManager {
    procs: [SigState; MAX_SIG_PROCS],
}

impl SignalManager {
    pub const fn new() -> Self {
        Self { procs: [const { SigState::new() }; MAX_SIG_PROCS] }
    }

    fn find(&mut self, pid: u32) -> Option<&mut SigState> {
        self.procs.iter_mut().find(|s| s.active && s.pid == pid)
    }

    pub fn register_process(&mut self, pid: u32) {
        for s in &mut self.procs {
            if !s.active { *s = SigState::new(); s.pid = pid; s.active = true; return; }
        }
    }

    pub fn remove_process(&mut self, pid: u32) {
        if let Some(s) = self.find(pid) { s.active = false; }
    }

    pub fn send_signal(&mut self, pid: u32, sig: u8) -> i32 {
        if sig == 0 || sig as usize >= MAX_SIGS { return -22; }
        let bit = 1u64 << sig;
        match self.find(pid) {
            Some(s) => { s.pending |= bit; 0 }
            None    => -3, // ESRCH
        }
    }

    /// Check for pending (unblocked) signals; return lowest-priority pending signal
    pub fn dequeue_signal(&mut self, pid: u32) -> Option<u8> {
        let s = self.find(pid)?;
        let deliverable = s.pending & !s.blocked;
        if deliverable == 0 { return None; }
        let sig = deliverable.trailing_zeros() as u8;
        s.pending &= !(1u64 << sig);
        Some(sig)
    }

    pub fn sigaction(&mut self, pid: u32, sig: u8, action: SigAction) -> i32 {
        if sig == 0 || sig == SIGKILL || sig == SIGSTOP { return -22; }
        match self.find(pid) {
            Some(s) => { s.actions[sig as usize] = action; 0 }
            None    => -3,
        }
    }

    pub fn sigprocmask(&mut self, pid: u32, how: u8, new_mask: u64) -> u64 {
        const SIG_BLOCK: u8 = 0; const SIG_UNBLOCK: u8 = 1; const SIG_SETMASK: u8 = 2;
        let s = match self.find(pid) { Some(s) => s, None => return 0 };
        let old = s.blocked;
        match how {
            SIG_BLOCK   => s.blocked |= new_mask,
            SIG_UNBLOCK => s.blocked &= !new_mask,
            SIG_SETMASK => s.blocked = new_mask,
            _ => {}
        }
        old
    }

    /// Deliver pending signals for a process; returns action to take
    pub unsafe fn deliver(&mut self, pid: u32) -> SignalAction {
        let sig = match self.dequeue_signal(pid) { Some(s) => s, None => return SignalAction::None };
        // SIGKILL and SIGSTOP cannot be caught/ignored
        if sig == SIGKILL { return SignalAction::Kill; }
        if sig == SIGSTOP { return SignalAction::Stop; }
        let action = match self.find(pid) {
            Some(s) => s.actions[sig as usize],
            None    => return SignalAction::Kill,
        };
        if action.handler == SIG_DFL {
            // Default actions
            match sig {
                SIGTERM | SIGHUP | SIGINT | SIGQUIT | SIGPIPE => SignalAction::Kill,
                SIGCHLD | SIGCONT | SIGWINCH => SignalAction::Ignore,
                _ => SignalAction::Kill,
            }
        } else if action.handler == SIG_IGN {
            SignalAction::Ignore
        } else {
            SignalAction::Call { handler: action.handler, sig }
        }
    }
}

#[derive(Debug)]
pub enum SignalAction {
    None,
    Kill,
    Stop,
    Ignore,
    Call { handler: u64, sig: u8 },
}

static mut G_SIGNALS: SignalManager = SignalManager::new();

#[no_mangle] pub unsafe extern "C" fn sigma_signal_register(pid: u32) { G_SIGNALS.register_process(pid); }
#[no_mangle] pub unsafe extern "C" fn sigma_signal_remove(pid: u32) { G_SIGNALS.remove_process(pid); }
#[no_mangle] pub unsafe extern "C" fn sigma_kill(pid: u32, sig: u8) -> i32 { G_SIGNALS.send_signal(pid, sig) }
#[no_mangle] pub unsafe extern "C" fn sigma_sigaction(pid: u32, sig: u8, handler: u64, mask: u64, flags: u32) -> i32 {
    G_SIGNALS.sigaction(pid, sig, SigAction { handler, mask, flags })
}
#[no_mangle] pub unsafe extern "C" fn sigma_sigprocmask(pid: u32, how: u8, mask: u64) -> u64 {
    G_SIGNALS.sigprocmask(pid, how, mask)
}
