// POSIX Signal Delivery Infrastructure
// Implements POSIX.1-2017 signal handling (sigaction, sigprocmask, signal delivery)

use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::vec::Vec;

/// POSIX signal numbers (following Linux/BSD conventions)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PosixSignal {
    SIGHUP = 1,    // Hangup detected on controlling terminal
    SIGINT = 2,    // Interrupt from keyboard (Ctrl+C)
    SIGQUIT = 3,   // Quit from keyboard (Ctrl+\)
    SIGILL = 4,    // Illegal Instruction
    SIGTRAP = 5,   // Trace/breakpoint trap
    SIGABRT = 6,   // Abort (usually SIGIOT)
    SIGBUS = 7,    // Bus error (bad memory access)
    SIGFPE = 8,    // Floating point exception
    SIGKILL = 9,   // Kill signal (cannot be caught or ignored)
    SIGUSR1 = 10,  // User-defined signal 1
    SIGSEGV = 11,  // Invalid memory reference
    SIGUSR2 = 12,  // User-defined signal 2
    SIGPIPE = 13,  // Broken pipe: write to pipe with no readers
    SIGALRM = 14,  // Timer signal from alarm(2)
    SIGTERM = 15,  // Termination signal
    SIGSTKFLT = 16, // Stack fault on coprocessor (unused)
    SIGCHLD = 17,  // Child stopped or terminated
    SIGCONT = 18,  // Continue if stopped
    SIGSTOP = 19,  // Stop process (cannot be caught or ignored)
    SIGTSTP = 20,  // Stop typed at terminal (Ctrl+Z)
    SIGTTIN = 21,  // Background read from tty
    SIGTTOU = 22,  // Background write to tty
    SIGURG = 23,   // Urgent condition on socket
    SIGXCPU = 24,  // CPU time limit exceeded
    SIGXFSZ = 25,  // File size limit exceeded
    SIGVTALRM = 26, // Virtual timer expired
    SIGPROF = 27,  // Profiling timer expired
    SIGWINCH = 28, // Window size change
    SIGIO = 29,    // I/O now possible
    SIGPWR = 30,   // Power failure restart
    SIGSYS = 31,   // Bad system call (SVr4)
}

impl PosixSignal {
    /// Convert from signal number
    pub fn from_num(num: u32) -> Option<Self> {
        match num {
            1 => Some(PosixSignal::SIGHUP),
            2 => Some(PosixSignal::SIGINT),
            3 => Some(PosixSignal::SIGQUIT),
            4 => Some(PosixSignal::SIGILL),
            5 => Some(PosixSignal::SIGTRAP),
            6 => Some(PosixSignal::SIGABRT),
            7 => Some(PosixSignal::SIGBUS),
            8 => Some(PosixSignal::SIGFPE),
            9 => Some(PosixSignal::SIGKILL),
            10 => Some(PosixSignal::SIGUSR1),
            11 => Some(PosixSignal::SIGSEGV),
            12 => Some(PosixSignal::SIGUSR2),
            13 => Some(PosixSignal::SIGPIPE),
            14 => Some(PosixSignal::SIGALRM),
            15 => Some(PosixSignal::SIGTERM),
            16 => Some(PosixSignal::SIGSTKFLT),
            17 => Some(PosixSignal::SIGCHLD),
            18 => Some(PosixSignal::SIGCONT),
            19 => Some(PosixSignal::SIGSTOP),
            20 => Some(PosixSignal::SIGTSTP),
            21 => Some(PosixSignal::SIGTTIN),
            22 => Some(PosixSignal::SIGTTOU),
            23 => Some(PosixSignal::SIGURG),
            24 => Some(PosixSignal::SIGXCPU),
            25 => Some(PosixSignal::SIGXFSZ),
            26 => Some(PosixSignal::SIGVTALRM),
            27 => Some(PosixSignal::SIGPROF),
            28 => Some(PosixSignal::SIGWINCH),
            29 => Some(PosixSignal::SIGIO),
            30 => Some(PosixSignal::SIGPWR),
            31 => Some(PosixSignal::SIGSYS),
            _ => None,
        }
    }

    /// Check if signal can be caught
    pub fn can_catch(self) -> bool {
        !matches!(self, PosixSignal::SIGKILL | PosixSignal::SIGSTOP)
    }

    /// Get signal name as string
    pub fn name(self) -> &'static str {
        match self {
            PosixSignal::SIGHUP => "SIGHUP",
            PosixSignal::SIGINT => "SIGINT",
            PosixSignal::SIGQUIT => "SIGQUIT",
            PosixSignal::SIGILL => "SIGILL",
            PosixSignal::SIGTRAP => "SIGTRAP",
            PosixSignal::SIGABRT => "SIGABRT",
            PosixSignal::SIGBUS => "SIGBUS",
            PosixSignal::SIGFPE => "SIGFPE",
            PosixSignal::SIGKILL => "SIGKILL",
            PosixSignal::SIGUSR1 => "SIGUSR1",
            PosixSignal::SIGSEGV => "SIGSEGV",
            PosixSignal::SIGUSR2 => "SIGUSR2",
            PosixSignal::SIGPIPE => "SIGPIPE",
            PosixSignal::SIGALRM => "SIGALRM",
            PosixSignal::SIGTERM => "SIGTERM",
            PosixSignal::SIGSTKFLT => "SIGSTKFLT",
            PosixSignal::SIGCHLD => "SIGCHLD",
            PosixSignal::SIGCONT => "SIGCONT",
            PosixSignal::SIGSTOP => "SIGSTOP",
            PosixSignal::SIGTSTP => "SIGTSTP",
            PosixSignal::SIGTTIN => "SIGTTIN",
            PosixSignal::SIGTTOU => "SIGTTOU",
            PosixSignal::SIGURG => "SIGURG",
            PosixSignal::SIGXCPU => "SIGXCPU",
            PosixSignal::SIGXFSZ => "SIGXFSZ",
            PosixSignal::SIGVTALRM => "SIGVTALRM",
            PosixSignal::SIGPROF => "SIGPROF",
            PosixSignal::SIGWINCH => "SIGWINCH",
            PosixSignal::SIGIO => "SIGIO",
            PosixSignal::SIGPWR => "SIGPWR",
            PosixSignal::SIGSYS => "SIGSYS",
        }
    }
}

/// Signal mask for sigprocmask
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalMask {
    bits: u64,
}

impl SignalMask {
    pub const fn empty() -> Self {
        SignalMask { bits: 0 }
    }

    pub const fn full() -> Self {
        SignalMask { bits: u64::MAX }
    }

    pub fn add(&mut self, signal: PosixSignal) {
        self.bits |= 1u64 << (signal as u32);
    }

    pub fn remove(&mut self, signal: PosixSignal) {
        self.bits &= !(1u64 << (signal as u32));
    }

    pub fn contains(&self, signal: PosixSignal) -> bool {
        self.bits & (1u64 << (signal as u32)) != 0
    }

    pub fn bits(&self) -> u64 {
        self.bits
    }
}

/// Signal action flags (sigaction sa_flags)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalFlags {
    pub no_cld_stop: bool,    // No SIGCHLD when child stops
    pub no_cld_wait: bool,    // No SIGCHLD when child terminates
    pub sig_info: bool,      // Provide signal info to handler
    pub restart_sys: bool,    // Restart interrupted syscalls
    pub on_stack: bool,       // Use alternate signal stack
    pub def_handler: bool,    // Use default handler
    pub no_defer: bool,       // Don't defer signals
    pub resume: bool,          // Resume if stopped
}

impl Default for SignalFlags {
    fn default() -> Self {
        SignalFlags {
            no_cld_stop: false,
            no_cld_wait: false,
            sig_info: false,
            restart_sys: true,
            on_stack: false,
            def_handler: false,
            no_defer: false,
            resume: false,
        }
    }
}

/// Signal handler function pointer (placeholder for actual function address)
#[derive(Debug, Clone, Copy)]
pub enum SignalHandler {
    /// Default handler (terminate, ignore, stop, or continue)
    Default,
    /// Ignore signal
    Ignore,
    /// Custom handler (function address)
    Handler(u64),
}

/// Signal information passed to handler (siginfo_t)
#[derive(Debug, Clone)]
pub struct SignalInfo {
    pub signo: PosixSignal,
    pub errno: i32,
    pub code: i32,
    pub pid: u32,
    pub uid: u32,
    pub value: i32,
    pub addr: u64,
}

impl SignalInfo {
    pub fn new(signo: PosixSignal) -> Self {
        SignalInfo {
            signo,
            errno: 0,
            code: 0,
            pid: 0,
            uid: 0,
            value: 0,
            addr: 0,
        }
    }
}

/// Signal action descriptor (struct sigaction)
#[derive(Debug, Clone, Copy)]
pub struct SignalAction {
    pub handler: SignalHandler,
    pub mask: SignalMask,
    pub flags: SignalFlags,
    pub restorer: Option<u64>,
}

impl Default for SignalAction {
    fn default() -> Self {
        SignalAction {
            handler: SignalHandler::Default,
            mask: SignalMask::empty(),
            flags: SignalFlags::default(),
            restorer: None,
        }
    }
}

/// Per-process signal disposition table
#[derive(Debug)]
pub struct SignalDispositionTable {
    actions: BTreeMap<PosixSignal, SignalAction>,
    pending: AtomicU64,
    delivered: AtomicU64,
}

impl Clone for SignalDispositionTable {
    fn clone(&self) -> Self {
        SignalDispositionTable {
            actions: self.actions.clone(),
            pending: AtomicU64::new(self.pending.load(Ordering::SeqCst)),
            delivered: AtomicU64::new(self.delivered.load(Ordering::SeqCst)),
        }
    }
}

impl SignalDispositionTable {
    pub fn new() -> Self {
        let mut actions = BTreeMap::new();

        // Set default dispositions (following Linux defaults)
        for sig in 1..=31u32 {
            if let Some(signal) = PosixSignal::from_num(sig) {
                let action = SignalAction::default();
                actions.insert(signal, action);
            }
        }

        SignalDispositionTable {
            actions,
            pending: AtomicU64::new(0),
            delivered: AtomicU64::new(0),
        }
    }

    /// Set signal action (sigaction system call)
    pub fn set_action(&mut self, signal: PosixSignal, action: SignalAction) -> Result<(), &'static str> {
        if !signal.can_catch() {
            return Err("Cannot set handler for SIGKILL or SIGSTOP");
        }
        self.actions.insert(signal, action);
        Ok(())
    }

    /// Get signal action
    pub fn get_action(&self, signal: PosixSignal) -> Option<&SignalAction> {
        self.actions.get(&signal)
    }

    /// Reset signal to default
    pub fn reset_to_default(&mut self, signal: PosixSignal) {
        self.actions.insert(signal, SignalAction::default());
    }

    /// Mark signal as pending
    pub fn mark_pending(&self, _signal: PosixSignal) {
        self.pending.fetch_add(1, Ordering::SeqCst);
    }

    /// Mark signal as delivered
    pub fn mark_delivered(&self, _signal: PosixSignal) {
        self.delivered.fetch_add(1, Ordering::SeqCst);
    }

    /// Get pending count
    pub fn get_pending_count(&self) -> u64 {
        self.pending.load(Ordering::SeqCst)
    }

    /// Get delivered count
    pub fn get_delivered_count(&self) -> u64 {
        self.delivered.load(Ordering::SeqCst)
    }
}

impl Default for SignalDispositionTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-thread signal mask (sigprocmask)
#[derive(Debug, Clone)]
pub struct ThreadSignalMask {
    current: SignalMask,
    blocked: SignalMask,
}

impl ThreadSignalMask {
    pub fn new() -> Self {
        ThreadSignalMask {
            current: SignalMask::empty(),
            blocked: SignalMask::empty(),
        }
    }

    /// Set signal mask (sigprocmask SIG_SETMASK)
    pub fn set_mask(&mut self, new_mask: SignalMask) {
        self.current = new_mask;
    }

    /// Block signals (sigprocmask SIG_BLOCK)
    pub fn block(&mut self, mask: SignalMask) {
        self.current.bits |= mask.bits;
    }

    /// Unblock signals (sigprocmask SIG_UNBLOCK)
    pub fn unblock(&mut self, mask: SignalMask) {
        self.current.bits &= !mask.bits;
    }

    /// Get current mask
    pub fn get_mask(&self) -> SignalMask {
        self.current
    }

    /// Check if signal is blocked
    pub fn is_blocked(&self, signal: PosixSignal) -> bool {
        self.current.contains(signal)
    }
}

impl Default for ThreadSignalMask {
    fn default() -> Self {
        Self::new()
    }
}

/// POSIX signal delivery engine
#[derive(Debug)]
pub struct PosixSignalDeliveryEngine {
    disposition_table: Arc<SignalDispositionTable>,
    thread_masks: Vec<ThreadSignalMask>,
    total_signals_sent: AtomicU64,
    total_signals_handled: AtomicU64,
}

impl PosixSignalDeliveryEngine {
    pub fn new() -> Self {
        PosixSignalDeliveryEngine {
            disposition_table: Arc::new(SignalDispositionTable::new()),
            thread_masks: Vec::new(),
            total_signals_sent: AtomicU64::new(0),
            total_signals_handled: AtomicU64::new(0),
        }
    }

    /// Send signal to process (kill system call)
    pub fn send_signal(&self, signal: PosixSignal, _target_pid: u32) -> Result<(), &'static str> {
        self.total_signals_sent.fetch_add(1, Ordering::SeqCst);
        self.disposition_table.mark_pending(signal);
        Ok(())
    }

    /// Set signal action (sigaction system call)
    pub fn sigaction(&mut self, signal: PosixSignal, action: SignalAction) -> Result<(), &'static str> {
        let mut table = (*self.disposition_table).clone();
        let result = table.set_action(signal, action);
        self.disposition_table = Arc::new(table);
        result
    }

    /// Get signal action
    pub fn get_sigaction(&self, signal: PosixSignal) -> Option<SignalAction> {
        self.disposition_table.get_action(signal).copied()
    }

    /// Set signal mask (sigprocmask system call)
    pub fn sigprocmask(&mut self, thread_id: usize, operation: SigprocmaskOp, mask: SignalMask) -> Result<SignalMask, &'static str> {
        if thread_id >= self.thread_masks.len() {
            self.thread_masks.resize(thread_id + 1, ThreadSignalMask::default());
        }

        let old_mask = self.thread_masks[thread_id].get_mask();
        
        match operation {
            SigprocmaskOp::Block => {
                self.thread_masks[thread_id].block(mask);
            }
            SigprocmaskOp::Unblock => {
                self.thread_masks[thread_id].unblock(mask);
            }
            SigprocmaskOp::SetMask => {
                self.thread_masks[thread_id].set_mask(mask);
            }
        }

        Ok(old_mask)
    }

    /// Process pending signals (called during kernel context switch)
    pub fn process_pending_signals(&mut self, thread_id: usize) -> Vec<SignalInfo> {
        let mut delivered = Vec::new();
        
        if thread_id >= self.thread_masks.len() {
            return delivered;
        }

        let mask = self.thread_masks[thread_id].get_mask();
        
        // Check all signals (simplified - in real implementation would check pending queue)
        for sig_num in 1..=31u32 {
            if let Some(signal) = PosixSignal::from_num(sig_num) {
                if !mask.contains(signal) {
                    if let Some(action) = self.disposition_table.get_action(signal) {
                        match action.handler {
                            SignalHandler::Default => {
                                // Default action: terminate, ignore, stop, or continue
                                delivered.push(SignalInfo::new(signal));
                            }
                            SignalHandler::Ignore => {
                                // Signal ignored
                            }
                            SignalHandler::Handler(_addr) => {
                                // Call custom handler (placeholder)
                                delivered.push(SignalInfo::new(signal));
                            }
                        }
                        self.disposition_table.mark_delivered(signal);
                        self.total_signals_handled.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        }

        delivered
    }

    /// Get statistics
    pub fn get_stats(&self) -> SignalStats {
        SignalStats {
            total_sent: self.total_signals_sent.load(Ordering::SeqCst),
            total_handled: self.total_signals_handled.load(Ordering::SeqCst),
            pending: self.disposition_table.get_pending_count(),
            delivered: self.disposition_table.get_delivered_count(),
        }
    }
}

impl Default for PosixSignalDeliveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// sigprocmask operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigprocmaskOp {
    Block,      // SIG_BLOCK: add signals to block set
    Unblock,    // SIG_UNBLOCK: remove signals from block set
    SetMask,    // SIG_SETMASK: set block set to given mask
}

/// Signal statistics
#[derive(Debug, Clone)]
pub struct SignalStats {
    pub total_sent: u64,
    pub total_handled: u64,
    pub pending: u64,
    pub delivered: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_from_num() {
        assert_eq!(PosixSignal::from_num(1), Some(PosixSignal::SIGHUP));
        assert_eq!(PosixSignal::from_num(9), Some(PosixSignal::SIGKILL));
        assert_eq!(PosixSignal::from_num(11), Some(PosixSignal::SIGSEGV));
        assert_eq!(PosixSignal::from_num(15), Some(PosixSignal::SIGTERM));
        assert_eq!(PosixSignal::from_num(99), None);
    }

    #[test]
    fn test_signal_can_catch() {
        assert!(PosixSignal::SIGINT.can_catch());
        assert!(PosixSignal::SIGTERM.can_catch());
        assert!(!PosixSignal::SIGKILL.can_catch());
        assert!(!PosixSignal::SIGSTOP.can_catch());
    }

    #[test]
    fn test_signal_mask() {
        let mut mask = SignalMask::empty();
        mask.add(PosixSignal::SIGINT);
        mask.add(PosixSignal::SIGTERM);
        
        assert!(mask.contains(PosixSignal::SIGINT));
        assert!(mask.contains(PosixSignal::SIGTERM));
        assert!(!mask.contains(PosixSignal::SIGKILL));
        
        mask.remove(PosixSignal::SIGINT);
        assert!(!mask.contains(PosixSignal::SIGINT));
    }

    #[test]
    fn test_signal_disposition_table() {
        let mut table = SignalDispositionTable::new();

        let action = SignalAction {
            handler: SignalHandler::Ignore,
            mask: SignalMask::empty(),
            flags: SignalFlags::default(),
            restorer: None,
        };

        assert!(table.set_action(PosixSignal::SIGINT, action.clone()).is_ok());
        assert!(table.set_action(PosixSignal::SIGKILL, action).is_err());

        table.mark_pending(PosixSignal::SIGINT);
        assert_eq!(table.get_pending_count(), 1);
    }

    #[test]
    fn test_thread_signal_mask() {
        let mut mask = ThreadSignalMask::new();
        
        let block_mask = SignalMask { bits: 1u64 << (PosixSignal::SIGINT as u32) };
        mask.block(block_mask);
        
        assert!(mask.is_blocked(PosixSignal::SIGINT));
        assert!(!mask.is_blocked(PosixSignal::SIGTERM));
        
        mask.unblock(block_mask);
        assert!(!mask.is_blocked(PosixSignal::SIGINT));
    }

    #[test]
    fn test_signal_delivery_engine() {
        let mut engine = PosixSignalDeliveryEngine::new();
        
        // Send signal
        assert!(engine.send_signal(PosixSignal::SIGINT, 1).is_ok());
        
        // Set action
        let action = SignalAction {
            handler: SignalHandler::Ignore,
            mask: SignalMask::empty(),
            flags: SignalFlags::default(),
            restorer: None,
        };
        assert!(engine.sigaction(PosixSignal::SIGINT, action).is_ok());
        
        // Set mask
        let mask = SignalMask::empty();
        assert!(engine.sigprocmask(0, SigprocmaskOp::SetMask, mask).is_ok());
        
        // Check stats
        let stats = engine.get_stats();
        assert_eq!(stats.total_sent, 1);
    }
}
