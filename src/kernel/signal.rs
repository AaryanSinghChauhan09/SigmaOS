// Linux-inspired Signal Management
// Provides POSIX signal handling and dispatching

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// POSIX signal numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGSTKFLT = 16,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGURG = 23,
    SIGXCPU = 24,
    SIGXFSZ = 25,
    SIGVTALRM = 26,
    SIGPROF = 27,
    SIGWINCH = 28,
    SIGIO = 29,
    SIGPWR = 30,
    SIGSYS = 31,
}

impl Signal {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Signal::SIGHUP),
            2 => Some(Signal::SIGINT),
            3 => Some(Signal::SIGQUIT),
            4 => Some(Signal::SIGILL),
            5 => Some(Signal::SIGTRAP),
            6 => Some(Signal::SIGABRT),
            7 => Some(Signal::SIGBUS),
            8 => Some(Signal::SIGFPE),
            9 => Some(Signal::SIGKILL),
            10 => Some(Signal::SIGUSR1),
            11 => Some(Signal::SIGSEGV),
            12 => Some(Signal::SIGUSR2),
            13 => Some(Signal::SIGPIPE),
            14 => Some(Signal::SIGALRM),
            15 => Some(Signal::SIGTERM),
            16 => Some(Signal::SIGSTKFLT),
            17 => Some(Signal::SIGCHLD),
            18 => Some(Signal::SIGCONT),
            19 => Some(Signal::SIGSTOP),
            20 => Some(Signal::SIGTSTP),
            21 => Some(Signal::SIGTTIN),
            22 => Some(Signal::SIGTTOU),
            23 => Some(Signal::SIGURG),
            24 => Some(Signal::SIGXCPU),
            25 => Some(Signal::SIGXFSZ),
            26 => Some(Signal::SIGVTALRM),
            27 => Some(Signal::SIGPROF),
            28 => Some(Signal::SIGWINCH),
            29 => Some(Signal::SIGIO),
            30 => Some(Signal::SIGPWR),
            31 => Some(Signal::SIGSYS),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Signal::SIGHUP => "SIGHUP",
            Signal::SIGINT => "SIGINT",
            Signal::SIGQUIT => "SIGQUIT",
            Signal::SIGILL => "SIGILL",
            Signal::SIGTRAP => "SIGTRAP",
            Signal::SIGABRT => "SIGABRT",
            Signal::SIGBUS => "SIGBUS",
            Signal::SIGFPE => "SIGFPE",
            Signal::SIGKILL => "SIGKILL",
            Signal::SIGUSR1 => "SIGUSR1",
            Signal::SIGSEGV => "SIGSEGV",
            Signal::SIGUSR2 => "SIGUSR2",
            Signal::SIGPIPE => "SIGPIPE",
            Signal::SIGALRM => "SIGALRM",
            Signal::SIGTERM => "SIGTERM",
            Signal::SIGSTKFLT => "SIGSTKFLT",
            Signal::SIGCHLD => "SIGCHLD",
            Signal::SIGCONT => "SIGCONT",
            Signal::SIGSTOP => "SIGSTOP",
            Signal::SIGTSTP => "SIGTSTP",
            Signal::SIGTTIN => "SIGTTIN",
            Signal::SIGTTOU => "SIGTTOU",
            Signal::SIGURG => "SIGURG",
            Signal::SIGXCPU => "SIGXCPU",
            Signal::SIGXFSZ => "SIGXFSZ",
            Signal::SIGVTALRM => "SIGVTALRM",
            Signal::SIGPROF => "SIGPROF",
            Signal::SIGWINCH => "SIGWINCH",
            Signal::SIGIO => "SIGIO",
            Signal::SIGPWR => "SIGPWR",
            Signal::SIGSYS => "SIGSYS",
        }
    }

    /// Check if signal can be caught
    pub fn can_catch(&self) -> bool {
        match self {
            Signal::SIGKILL | Signal::SIGSTOP => false,
            _ => true,
        }
    }

    /// Check if signal can be ignored
    pub fn can_ignore(&self) -> bool {
        match self {
            Signal::SIGKILL | Signal::SIGSTOP => false,
            _ => true,
        }
    }
}

/// Signal disposition (how to handle a signal)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalDisposition {
    Default,  // Default handler
    Ignore,   // Ignore signal
    Catch,    // Catch with handler
}

/// Signal handler
#[derive(Debug, Clone)]
pub struct SignalHandler {
    pub signal: Signal,
    pub disposition: SignalDisposition,
    pub handler_address: Option<u64>,
}

impl SignalHandler {
    pub fn new(signal: Signal, disposition: SignalDisposition) -> Self {
        Self {
            signal,
            disposition,
            handler_address: None,
        }
    }

    pub fn with_handler(mut self, address: u64) -> Self {
        self.handler_address = Some(address);
        self
    }
}

/// Signal info (additional information about signal)
#[derive(Debug, Clone)]
pub struct SignalInfo {
    pub signal: Signal,
    pub sender_pid: u32,
    pub sender_uid: u32,
    pub value: i32,
    pub errno: i32,
}

impl SignalInfo {
    pub fn new(signal: Signal, sender_pid: u32, sender_uid: u32) -> Self {
        Self {
            signal,
            sender_pid,
            sender_uid,
            value: 0,
            errno: 0,
        }
    }

    pub fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }

    pub fn with_errno(mut self, errno: i32) -> Self {
        self.errno = errno;
        self
    }
}

/// Signal mask (blocked signals)
#[derive(Debug, Clone)]
pub struct SignalMask {
    pub blocked: u64, // Bitmask of blocked signals
}

impl SignalMask {
    pub fn new() -> Self {
        Self { blocked: 0 }
    }

    /// Block a signal
    pub fn block(&mut self, signal: Signal) {
        self.blocked |= 1 << (signal as u8);
    }

    /// Unblock a signal
    pub fn unblock(&mut self, signal: Signal) {
        self.blocked &= !(1 << (signal as u8));
    }

    /// Check if signal is blocked
    pub fn is_blocked(&self, signal: Signal) -> bool {
        (self.blocked & (1 << (signal as u8))) != 0
    }

    /// Block all signals
    pub fn block_all(&mut self) {
        self.blocked = u64::MAX;
    }

    /// Unblock all signals
    pub fn unblock_all(&mut self) {
        self.blocked = 0;
    }
}

impl Default for SignalMask {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal manager for system-wide signal management
pub struct SignalManager {
    handlers: Arc<Mutex<HashMap<u32, HashMap<Signal, SignalHandler>>>>,
    signal_masks: Arc<Mutex<HashMap<u32, SignalMask>>>,
    pending_signals: Arc<Mutex<HashMap<u32, Vec<SignalInfo>>>>,
    next_pid: Arc<Mutex<u32>>,
}

impl SignalManager {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            signal_masks: Arc::new(Mutex::new(HashMap::new())),
            pending_signals: Arc::new(Mutex::new(HashMap::new())),
            next_pid: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new process (returns PID)
    pub fn create_process(&self) -> u32 {
        let mut next_pid = self.next_pid.lock().unwrap();
        let pid = *next_pid;
        *next_pid += 1;
        drop(next_pid);

        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(pid, HashMap::new());

        let mut signal_masks = self.signal_masks.lock().unwrap();
        signal_masks.insert(pid, SignalMask::new());

        let mut pending = self.pending_signals.lock().unwrap();
        pending.insert(pid, Vec::new());

        pid
    }

    /// Set signal handler for a process
    pub fn set_handler(&self, pid: u32, handler: SignalHandler) -> Result<(), String> {
        let mut handlers = self.handlers.lock().unwrap();
        match handlers.get_mut(&pid) {
            Some(process_handlers) => {
                process_handlers.insert(handler.signal, handler);
                Ok(())
            }
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// Get signal handler for a process
    pub fn get_handler(&self, pid: u32, signal: Signal) -> Option<SignalHandler> {
        let handlers = self.handlers.lock().unwrap();
        handlers.get(&pid).and_then(|h| h.get(&signal).cloned())
    }

    /// Set signal mask for a process
    pub fn set_signal_mask(&self, pid: u32, mask: SignalMask) -> Result<(), String> {
        let mut signal_masks = self.signal_masks.lock().unwrap();
        match signal_masks.get_mut(&pid) {
            Some(process_mask) => {
                *process_mask = mask;
                Ok(())
            }
            None => Err(format!("Process {} not found", pid)),
        }
    }

    /// Get signal mask for a process
    pub fn get_signal_mask(&self, pid: u32) -> Option<SignalMask> {
        let signal_masks = self.signal_masks.lock().unwrap();
        signal_masks.get(&pid).cloned()
    }

    /// Send a signal to a process
    pub fn send_signal(&self, info: SignalInfo) -> Result<(), String> {
        let pid = info.sender_pid;
        let sig = info.signal;

        let signal_masks = self.signal_masks.lock().unwrap();
        let blocked = signal_masks.get(&pid)
            .map(|mask| mask.is_blocked(sig))
            .unwrap_or(false);
        drop(signal_masks);

        if blocked {
            // Add to pending signals
            let mut pending = self.pending_signals.lock().unwrap();
            match pending.get_mut(&pid) {
                Some(signals) => {
                    signals.push(info);
                }
                None => return Err(format!("Process {} not found", pid)),
            }
        } else {
            // Deliver immediately (simplified)
            let handlers = self.handlers.lock().unwrap();
            if let Some(process_handlers) = handlers.get(&pid) {
                if let Some(handler) = process_handlers.get(&sig) {
                    match handler.disposition {
                        SignalDisposition::Ignore => {
                            // Do nothing
                        }
                        SignalDisposition::Default => {
                            // Default action
                        }
                        SignalDisposition::Catch => {
                            // Call handler (simplified)
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get pending signals for a process
    pub fn get_pending_signals(&self, pid: u32) -> Vec<SignalInfo> {
        let pending = self.pending_signals.lock().unwrap();
        pending.get(&pid).cloned().unwrap_or_default()
    }

    /// Clear pending signals for a process
    pub fn clear_pending_signals(&self, pid: u32) {
        let mut pending = self.pending_signals.lock().unwrap();
        if let Some(signals) = pending.get_mut(&pid) {
            signals.clear();
        }
    }

    /// Remove a process
    pub fn remove_process(&self, pid: u32) -> Result<(), String> {
        let mut handlers = self.handlers.lock().unwrap();
        let mut signal_masks = self.signal_masks.lock().unwrap();
        let mut pending = self.pending_signals.lock().unwrap();

        match (handlers.remove(&pid), signal_masks.remove(&pid), pending.remove(&pid)) {
            (Some(_), Some(_), Some(_)) => Ok(()),
            _ => Err(format!("Process {} not found", pid)),
        }
    }
}

impl Default for SignalManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_from_u8() {
        assert_eq!(Signal::from_u8(1), Some(Signal::SIGHUP));
        assert_eq!(Signal::from_u8(9), Some(Signal::SIGKILL));
        assert_eq!(Signal::from_u8(15), Some(Signal::SIGTERM));
        assert_eq!(Signal::from_u8(32), None);
    }

    #[test]
    fn test_signal_as_str() {
        assert_eq!(Signal::SIGKILL.as_str(), "SIGKILL");
        assert_eq!(Signal::SIGTERM.as_str(), "SIGTERM");
        assert_eq!(Signal::SIGINT.as_str(), "SIGINT");
    }

    #[test]
    fn test_signal_can_catch() {
        assert!(Signal::SIGTERM.can_catch());
        assert!(!Signal::SIGKILL.can_catch());
        assert!(!Signal::SIGSTOP.can_catch());
    }

    #[test]
    fn test_signal_can_ignore() {
        assert!(Signal::SIGTERM.can_ignore());
        assert!(!Signal::SIGKILL.can_ignore());
        assert!(!Signal::SIGSTOP.can_ignore());
    }

    #[test]
    fn test_signal_handler() {
        let handler = SignalHandler::new(Signal::SIGTERM, SignalDisposition::Catch)
            .with_handler(0x12345678);

        assert_eq!(handler.signal, Signal::SIGTERM);
        assert_eq!(handler.disposition, SignalDisposition::Catch);
        assert_eq!(handler.handler_address, Some(0x12345678));
    }

    #[test]
    fn test_signal_mask() {
        let mut mask = SignalMask::new();
        assert!(!mask.is_blocked(Signal::SIGTERM));

        mask.block(Signal::SIGTERM);
        assert!(mask.is_blocked(Signal::SIGTERM));

        mask.unblock(Signal::SIGTERM);
        assert!(!mask.is_blocked(Signal::SIGTERM));
    }

    #[test]
    fn test_signal_mask_block_all() {
        let mut mask = SignalMask::new();
        mask.block_all();
        assert!(mask.is_blocked(Signal::SIGTERM));
        assert!(mask.is_blocked(Signal::SIGINT));

        mask.unblock_all();
        assert!(!mask.is_blocked(Signal::SIGTERM));
    }

    #[test]
    fn test_signal_info() {
        let info = SignalInfo::new(Signal::SIGTERM, 100, 1000)
            .with_value(42)
            .with_errno(1);

        assert_eq!(info.signal, Signal::SIGTERM);
        assert_eq!(info.sender_pid, 100);
        assert_eq!(info.sender_uid, 1000);
        assert_eq!(info.value, 42);
        assert_eq!(info.errno, 1);
    }

    #[test]
    fn test_signal_manager() {
        let manager = SignalManager::new();

        let pid = manager.create_process();
        assert_eq!(pid, 1);

        let handler = SignalHandler::new(Signal::SIGTERM, SignalDisposition::Catch);
        manager.set_handler(pid, handler).unwrap();

        let retrieved = manager.get_handler(pid, Signal::SIGTERM).unwrap();
        assert_eq!(retrieved.disposition, SignalDisposition::Catch);
    }

    #[test]
    fn test_signal_manager_signal_mask() {
        let manager = SignalManager::new();

        let pid = manager.create_process();

        let mut mask = SignalMask::new();
        mask.block(Signal::SIGTERM);
        manager.set_signal_mask(pid, mask).unwrap();

        let retrieved = manager.get_signal_mask(pid).unwrap();
        assert!(retrieved.is_blocked(Signal::SIGTERM));
    }

    #[test]
    fn test_signal_manager_send_signal() {
        let manager = SignalManager::new();

        let pid = manager.create_process();

        let info = SignalInfo::new(Signal::SIGTERM, pid, 1000);
        manager.send_signal(info).unwrap();
    }

    #[test]
    fn test_signal_manager_pending_signals() {
        let manager = SignalManager::new();

        let pid = manager.create_process();

        let mut mask = SignalMask::new();
        mask.block(Signal::SIGTERM);
        manager.set_signal_mask(pid, mask).unwrap();

        let info = SignalInfo::new(Signal::SIGTERM, pid, 1000);
        manager.send_signal(info).unwrap();

        let pending = manager.get_pending_signals(pid);
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_signal_manager_remove_process() {
        let manager = SignalManager::new();

        let pid = manager.create_process();
        manager.remove_process(pid).unwrap();

        assert!(manager.set_handler(pid, SignalHandler::new(Signal::SIGTERM, SignalDisposition::Default)).is_err());
    }
}
