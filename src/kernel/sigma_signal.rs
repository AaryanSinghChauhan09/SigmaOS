#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! SigmaOS Signal Subsystem
//!
//! Sovereign POSIX-compatible signal handling. Inspired by:
//! - Linux signal handling (kernel/signal.c)
//! - FreeBSD sigaction(2), sigprocmask(2)
//! - OpenBSD signal(3) with pledge integration
//!
//! Signals are asynchronous notifications sent between processes
//! or from kernel to process (e.g., SIGSEGV on fault).

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================
// Signal Numbers (POSIX + Linux-compatible)
// ============================================================

/// POSIX signal numbers. Values match Linux x86_64 signal numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Signal {
    /// Hangup (terminal disconnect)
    SIGHUP  = 1,
    /// Interrupt (Ctrl+C)
    SIGINT  = 2,
    /// Quit (core dump)
    SIGQUIT = 3,
    /// Illegal instruction
    SIGILL  = 4,
    /// Trap (debug breakpoint)
    SIGTRAP = 5,
    /// Abort
    SIGABRT = 6,
    /// Bus error (misaligned access)
    SIGBUS  = 7,
    /// Floating-point exception
    SIGFPE  = 8,
    /// Kill (cannot be caught or ignored)
    SIGKILL = 9,
    /// User-defined signal 1
    SIGUSR1 = 10,
    /// Segmentation fault
    SIGSEGV = 11,
    /// User-defined signal 2
    SIGUSR2 = 12,
    /// Broken pipe
    SIGPIPE = 13,
    /// Alarm (timer expiry)
    SIGALRM = 14,
    /// Termination request
    SIGTERM = 15,
    /// Stack fault
    SIGSTKFLT = 16,
    /// Child stopped or terminated
    SIGCHLD = 17,
    /// Continue paused process
    SIGCONT = 18,
    /// Stop (cannot be caught)
    SIGSTOP = 19,
    /// Stop from terminal
    SIGTSTP = 20,
    /// Terminal input from background process
    SIGTTIN = 21,
    /// Terminal output from background process
    SIGTTOU = 22,
    /// Urgent I/O condition
    SIGURG  = 23,
    /// CPU time limit exceeded
    SIGXCPU = 24,
    /// File size limit exceeded
    SIGXFSZ = 25,
    /// Virtual alarm timer
    SIGVTALRM = 26,
    /// Profiling timer
    SIGPROF = 27,
    /// Window resize
    SIGWINCH = 28,
    /// I/O possible
    SIGIO   = 29,
    /// Power failure
    SIGPWR  = 30,
    /// Bad system call
    SIGSYS  = 31,
}

impl Signal {
    /// Returns the signal number (1-31).
    pub fn number(self) -> u8 { self as u8 }

    /// Returns the signal name as a string.
    pub fn name(self) -> &'static str {
        match self {
            Self::SIGHUP => "SIGHUP", Self::SIGINT => "SIGINT",
            Self::SIGQUIT => "SIGQUIT", Self::SIGILL => "SIGILL",
            Self::SIGTRAP => "SIGTRAP", Self::SIGABRT => "SIGABRT",
            Self::SIGBUS => "SIGBUS", Self::SIGFPE => "SIGFPE",
            Self::SIGKILL => "SIGKILL", Self::SIGUSR1 => "SIGUSR1",
            Self::SIGSEGV => "SIGSEGV", Self::SIGUSR2 => "SIGUSR2",
            Self::SIGPIPE => "SIGPIPE", Self::SIGALRM => "SIGALRM",
            Self::SIGTERM => "SIGTERM", Self::SIGCHLD => "SIGCHLD",
            Self::SIGCONT => "SIGCONT", Self::SIGSTOP => "SIGSTOP",
            Self::SIGTSTP => "SIGTSTP", Self::SIGIO => "SIGIO",
            Self::SIGWINCH => "SIGWINCH", Self::SIGSYS => "SIGSYS",
            _ => "SIGUNKNOWN",
        }
    }

    /// Returns true if this signal cannot be caught or ignored.
    pub fn is_unblockable(self) -> bool {
        matches!(self, Self::SIGKILL | Self::SIGSTOP)
    }

    /// Returns the default disposition for this signal.
    pub fn default_action(self) -> SignalDisposition {
        match self {
            Self::SIGCHLD | Self::SIGURG | Self::SIGWINCH => SignalDisposition::Ignore,
            Self::SIGSTOP | Self::SIGTSTP | Self::SIGTTIN | Self::SIGTTOU => SignalDisposition::Stop,
            Self::SIGCONT => SignalDisposition::Continue,
            Self::SIGABRT | Self::SIGBUS | Self::SIGFPE | Self::SIGILL |
            Self::SIGQUIT | Self::SIGSEGV | Self::SIGSYS | Self::SIGTRAP |
            Self::SIGXCPU | Self::SIGXFSZ => SignalDisposition::CoreDump,
            _ => SignalDisposition::Terminate,
        }
    }
}

// ============================================================
// Signal Disposition
// ============================================================

/// What happens when a signal is delivered to a process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalDisposition {
    /// Terminate the process
    Terminate,
    /// Terminate + generate core dump
    CoreDump,
    /// Ignore the signal
    Ignore,
    /// Stop the process (SIGSTOP)
    Stop,
    /// Continue stopped process (SIGCONT)
    Continue,
    /// Call a user-installed handler
    Handler,
}

// ============================================================
// Signal Action (sigaction)
// ============================================================

/// Signal action configuration (analogous to POSIX sigaction struct).
#[derive(Clone)]
pub struct SigAction {
    /// How to handle the signal
    pub disposition: SignalDisposition,
    /// Handler address (opaque; actual dispatch is platform-specific)
    pub handler_addr: u64,
    /// Signals to block during handler execution
    pub mask: SigSet,
    /// Flags (SA_RESTART, SA_SIGINFO, etc.)
    pub flags: SigActionFlags,
}

impl Default for SigAction {
    fn default() -> Self {
        Self {
            disposition: SignalDisposition::Terminate,
            handler_addr: 0,
            mask: SigSet::empty(),
            flags: SigActionFlags::empty(),
        }
    }
}

// ============================================================
// Signal Set (sigset_t)
// ============================================================

/// A bitmask of signals (analogous to POSIX sigset_t).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SigSet(u64);

impl SigSet {
    pub fn empty() -> Self { Self(0) }
    pub fn full() -> Self { Self(u64::MAX) }

    pub fn add(&mut self, sig: Signal) { self.0 |= 1u64 << (sig.number() - 1); }
    pub fn remove(&mut self, sig: Signal) { self.0 &= !(1u64 << (sig.number() - 1)); }
    pub fn contains(&self, sig: Signal) -> bool { (self.0 >> (sig.number() - 1)) & 1 == 1 }
    pub fn is_empty(&self) -> bool { self.0 == 0 }

    pub fn union(self, other: Self) -> Self { Self(self.0 | other.0) }
    pub fn intersection(self, other: Self) -> Self { Self(self.0 & other.0) }
}

// ============================================================
// Signal Action Flags
// ============================================================

/// Flags for sigaction (SA_* constants).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SigActionFlags(u32);

impl SigActionFlags {
    pub fn empty() -> Self { Self(0) }
    /// Restart syscall interrupted by signal
    pub const SA_RESTART:  u32 = 0x10000000;
    /// Provide siginfo_t to handler
    pub const SA_SIGINFO:  u32 = 0x00000004;
    /// Clear handler after delivery (one-shot)
    pub const SA_RESETHAND: u32 = 0x80000000;
    /// Don't generate SIGCHLD when child stops
    pub const SA_NOCLDSTOP: u32 = 0x00000001;

    pub fn has_restart(&self) -> bool { (self.0 & Self::SA_RESTART) != 0 }
    pub fn has_siginfo(&self) -> bool { (self.0 & Self::SA_SIGINFO) != 0 }
    pub fn has_resethand(&self) -> bool { (self.0 & Self::SA_RESETHAND) != 0 }
}

// ============================================================
// Pending Signal
// ============================================================

/// A signal pending delivery to a process.
#[derive(Debug, Clone)]
pub struct PendingSignal {
    pub signal: Signal,
    /// Sender PID (0 = kernel)
    pub sender_pid: u32,
    /// Additional info (si_code, si_addr, etc.)
    pub si_code: i32,
    pub si_addr: u64,
}

// ============================================================
// SigmaSignalState — Per-Process Signal State
// ============================================================

/// Per-process signal state.
///
/// Tracks pending signals, signal actions, and blocked signal mask.
/// Follows the Linux sighand_struct + signal_struct design.
pub struct SigmaSignalState {
    /// Owner process PID
    pid: u32,
    /// Signal actions (one per signal, 1-31)
    actions: [SigAction; 32],
    /// Blocked signals (signals not delivered while blocked)
    blocked: SigSet,
    /// Queue of pending signals
    pending: VecDeque<PendingSignal>,
    /// Whether the process is stopped
    is_stopped: bool,
    /// Exit code if terminated by signal
    exit_code: Option<u8>,
}

impl SigmaSignalState {
    /// Create a new signal state for the given PID.
    pub fn new(pid: u32) -> Self {
        // Initialise all actions to default
        let actions: [SigAction; 32] = core::array::from_fn(|_| SigAction::default());
        Self {
            pid,
            actions,
            blocked: SigSet::empty(),
            pending: VecDeque::new(),
            is_stopped: false,
            exit_code: None,
        }
    }

    /// Install a new signal action (sigaction).
    pub fn sigaction(&mut self, sig: Signal, action: SigAction) -> Result<SigAction, &'static str> {
        if sig.is_unblockable() { return Err("SIGKILL and SIGSTOP cannot be caught"); }
        let old = self.actions[sig.number() as usize].clone();
        self.actions[sig.number() as usize] = action;
        Ok(old)
    }

    /// Get current signal action.
    pub fn get_action(&self, sig: Signal) -> &SigAction {
        &self.actions[sig.number() as usize]
    }

    /// Set signal mask (sigprocmask SIG_SETMASK).
    pub fn set_mask(&mut self, mask: SigSet) {
        // SIGKILL and SIGSTOP cannot be blocked
        let mut m = mask;
        m.remove(Signal::SIGKILL);
        m.remove(Signal::SIGSTOP);
        self.blocked = m;
    }

    /// Block additional signals (sigprocmask SIG_BLOCK).
    pub fn block(&mut self, mask: SigSet) {
        let new_mask = self.blocked.union(mask);
        self.set_mask(new_mask);
    }

    /// Unblock signals (sigprocmask SIG_UNBLOCK).
    pub fn unblock(&mut self, mask: SigSet) {
        self.blocked.0 &= !mask.0;
    }

    /// Send a signal to this process.
    pub fn send_signal(&mut self, sig: Signal, sender_pid: u32) {
        // Drop signal if blocked and not RT signal
        if self.blocked.contains(sig) && !sig.is_unblockable() { return; }

        // Don't queue duplicate non-RT signals
        if self.pending.iter().any(|p| p.signal == sig) { return; }

        self.pending.push_back(PendingSignal {
            signal: sig, sender_pid, si_code: 0, si_addr: 0,
        });
    }

    /// Dequeue the next deliverable signal.
    ///
    /// Returns None if no unblocked signal is pending.
    pub fn dequeue_signal(&mut self) -> Option<PendingSignal> {
        let pos = self.pending.iter().position(|p| {
            !self.blocked.contains(p.signal) || p.signal.is_unblockable()
        })?;
        self.pending.remove(pos)
    }

    /// Process all pending signals. Returns the disposition of the last-processed signal.
    pub fn process_pending(&mut self) -> Vec<(Signal, SignalDisposition)> {
        let mut results = Vec::new();
        while let Some(pending) = self.dequeue_signal() {
            let sig = pending.signal;
            let action = &self.actions[sig.number() as usize];
            let disposition = match action.disposition {
                SignalDisposition::Handler => {
                    // In a real system, we'd set up the signal frame here
                    SignalDisposition::Handler
                }
                d => {
                    // Apply default disposition effects
                    match d {
                        SignalDisposition::Terminate | SignalDisposition::CoreDump => {
                            self.exit_code = Some(sig.number());
                        }
                        SignalDisposition::Stop => { self.is_stopped = true; }
                        SignalDisposition::Continue => { self.is_stopped = false; }
                        _ => {}
                    }
                    d
                }
            };
            // SA_RESETHAND: reset handler to default after delivery
            if action.flags.has_resethand() {
                self.actions[sig.number() as usize] = SigAction::default();
            }
            results.push((sig, disposition));
        }
        results
    }

    pub fn pid(&self) -> u32 { self.pid }
    pub fn is_stopped(&self) -> bool { self.is_stopped }
    pub fn exit_code(&self) -> Option<u8> { self.exit_code }
    pub fn pending_count(&self) -> usize { self.pending.len() }
    pub fn blocked_mask(&self) -> SigSet { self.blocked }
}

// ============================================================
// SigmaSignalManager — System-Wide Signal Dispatcher
// ============================================================

/// System-wide signal manager.
///
/// Routes signals between processes and tracks per-process state.
pub struct SigmaSignalManager {
    states: BTreeMap<u32, SigmaSignalState>,
}

impl SigmaSignalManager {
    pub fn new() -> Self { Self { states: BTreeMap::new() } }

    /// Register a process.
    pub fn register_process(&mut self, pid: u32) {
        self.states.insert(pid, SigmaSignalState::new(pid));
    }

    /// Unregister a process (on exit).
    pub fn unregister_process(&mut self, pid: u32) { self.states.remove(&pid); }

    /// Send a signal to a process (kill(pid, sig)).
    pub fn kill(&mut self, target_pid: u32, sig: Signal, sender_pid: u32) -> Result<(), &'static str> {
        let state = self.states.get_mut(&target_pid).ok_or("no such process")?;
        state.send_signal(sig, sender_pid);
        Ok(())
    }

    /// Send signal to all processes (kill(-1, sig)).
    pub fn kill_all(&mut self, sig: Signal, sender_pid: u32) {
        let pids: Vec<u32> = self.states.keys().copied().collect();
        for pid in pids {
            if pid != sender_pid {
                if let Some(state) = self.states.get_mut(&pid) {
                    state.send_signal(sig, sender_pid);
                }
            }
        }
    }

    /// Process pending signals for a PID. Returns actions taken.
    pub fn deliver(&mut self, pid: u32) -> Vec<(Signal, SignalDisposition)> {
        self.states.get_mut(&pid)
            .map(|s| s.process_pending())
            .unwrap_or_default()
    }

    /// Get mutable signal state for a process.
    pub fn state_mut(&mut self, pid: u32) -> Option<&mut SigmaSignalState> {
        self.states.get_mut(&pid)
    }

    pub fn state(&self, pid: u32) -> Option<&SigmaSignalState> { self.states.get(&pid) }
    pub fn process_count(&self) -> usize { self.states.len() }
}

impl Default for SigmaSignalManager {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_send_and_receive() {
        let mut state = SigmaSignalState::new(100);
        state.send_signal(Signal::SIGTERM, 1);
        assert_eq!(state.pending_count(), 1);
        let results = state.process_pending();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, Signal::SIGTERM);
        assert_eq!(results[0].1, SignalDisposition::Terminate);
    }

    #[test]
    fn test_blocked_signal() {
        let mut state = SigmaSignalState::new(200);
        let mut mask = SigSet::empty();
        mask.add(Signal::SIGTERM);
        state.set_mask(mask);
        state.send_signal(Signal::SIGTERM, 1);
        // Blocked — should not be queued
        assert_eq!(state.pending_count(), 0);
    }

    #[test]
    fn test_sigkill_not_blockable() {
        let mut state = SigmaSignalState::new(300);
        let mut mask = SigSet::full();
        mask.remove(Signal::SIGKILL); // This would be a no-op anyway
        state.set_mask(mask);
        // SIGKILL should still be delivered even if in blocked set
        state.send_signal(Signal::SIGKILL, 1);
        assert_eq!(state.pending_count(), 1);
    }

    #[test]
    fn test_signal_manager_kill() {
        let mut mgr = SigmaSignalManager::new();
        mgr.register_process(1000);
        mgr.kill(1000, Signal::SIGINT, 0).unwrap();
        let results = mgr.deliver(1000);
        assert_eq!(results[0].0, Signal::SIGINT);
    }

    #[test]
    fn test_custom_handler() {
        let mut state = SigmaSignalState::new(400);
        let action = SigAction {
            disposition: SignalDisposition::Handler,
            handler_addr: 0xDEADBEEF,
            mask: SigSet::empty(),
            flags: SigActionFlags::empty(),
        };
        state.sigaction(Signal::SIGUSR1, action).unwrap();
        state.send_signal(Signal::SIGUSR1, 0);
        let results = state.process_pending();
        assert_eq!(results[0].1, SignalDisposition::Handler);
    }

    #[test]
    fn test_sigset_operations() {
        let mut set = SigSet::empty();
        set.add(Signal::SIGTERM);
        set.add(Signal::SIGINT);
        assert!(set.contains(Signal::SIGTERM));
        assert!(set.contains(Signal::SIGINT));
        assert!(!set.contains(Signal::SIGHUP));
        set.remove(Signal::SIGTERM);
        assert!(!set.contains(Signal::SIGTERM));
    }
}
