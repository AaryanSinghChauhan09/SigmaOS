// Linux-inspired signalfd for signal-based file descriptor notifications
// Signal management through file descriptor interface for SigmaOS

use std::collections::HashSet;

/// Standard POSIX signals
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
    pub fn from_num(num: i32) -> Option<Self> {
        match num {
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

    pub fn to_num(&self) -> i32 {
        *self as i32
    }
}

/// Signal information structure
#[derive(Debug, Clone)]
pub struct SigInfo {
    pub signo: i32,
    pub errno: i32,
    pub code: i32,
    pub pid: u32,
    pub uid: u32,
    pub value: i32,
}

impl SigInfo {
    pub fn new(signo: i32, pid: u32, uid: u32) -> Self {
        SigInfo {
            signo,
            errno: 0,
            code: 0,
            pid,
            uid,
            value: 0,
        }
    }
}

/// signalfd configuration flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalFdFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
}

impl SignalFdFlags {
    pub fn new() -> Self {
        SignalFdFlags {
            non_blocking: false,
            close_on_exec: false,
        }
    }
}

impl Default for SignalFdFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// signalfd mask
#[derive(Debug, Clone)]
pub struct SignalMask {
    signals: HashSet<Signal>,
}

impl SignalMask {
    pub fn new() -> Self {
        SignalMask {
            signals: HashSet::new(),
        }
    }

    pub fn add(&mut self, signal: Signal) {
        self.signals.insert(signal);
    }

    pub fn remove(&mut self, signal: Signal) {
        self.signals.remove(&signal);
    }

    pub fn contains(&self, signal: Signal) -> bool {
        self.signals.contains(&signal)
    }

    pub fn clear(&mut self) {
        self.signals.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    pub fn count(&self) -> usize {
        self.signals.len()
    }
}

impl Default for SignalMask {
    fn default() -> Self {
        Self::new()
    }
}

/// signalfd instance
pub struct SignalFd {
    flags: SignalFdFlags,
    mask: SignalMask,
    pending_signals: Vec<SigInfo>,
}

impl SignalFd {
    pub fn new(flags: SignalFdFlags, mask: SignalMask) -> Self {
        SignalFd {
            flags,
            mask,
            pending_signals: Vec::new(),
        }
    }

    /// Update signal mask
    pub fn set_mask(&mut self, mask: SignalMask) {
        self.mask = mask;
    }

    /// Get current signal mask
    pub fn get_mask(&self) -> SignalMask {
        self.mask.clone()
    }

    /// Simulate signal delivery
    pub fn deliver_signal(&mut self, siginfo: SigInfo) {
        if let Some(signal) = Signal::from_num(siginfo.signo) {
            if self.mask.contains(signal) {
                self.pending_signals.push(siginfo);
            }
        }
    }

    /// Read pending signals
    pub fn read(&mut self) -> Result<Vec<SigInfo>, String> {
        if self.pending_signals.is_empty() {
            return Err("No pending signals".to_string());
        }

        let signals = self.pending_signals.clone();
        self.pending_signals.clear();
        Ok(signals)
    }

    /// Get pending signal count
    pub fn pending_count(&self) -> usize {
        self.pending_signals.len()
    }

    /// Check if signal is in mask
    pub fn is_masked(&self, signal: Signal) -> bool {
        self.mask.contains(signal)
    }
}

impl Default for SignalFd {
    fn default() -> Self {
        Self::new(SignalFdFlags::new(), SignalMask::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_from_num() {
        assert_eq!(Signal::from_num(2), Some(Signal::SIGINT));
        assert_eq!(Signal::from_num(15), Some(Signal::SIGTERM));
        assert_eq!(Signal::from_num(99), None);
    }

    #[test]
    fn test_signal_to_num() {
        assert_eq!(Signal::SIGINT.to_num(), 2);
        assert_eq!(Signal::SIGTERM.to_num(), 15);
    }

    #[test]
    fn test_sig_info_creation() {
        let siginfo = SigInfo::new(2, 100, 1000);
        assert_eq!(siginfo.signo, 2);
        assert_eq!(siginfo.pid, 100);
        assert_eq!(siginfo.uid, 1000);
    }

    #[test]
    fn test_signal_mask() {
        let mut mask = SignalMask::new();
        
        mask.add(Signal::SIGINT);
        mask.add(Signal::SIGTERM);
        
        assert_eq!(mask.count(), 2);
        assert!(mask.contains(Signal::SIGINT));
        assert!(!mask.contains(Signal::SIGKILL));
    }

    #[test]
    fn test_signal_mask_remove() {
        let mut mask = SignalMask::new();
        
        mask.add(Signal::SIGINT);
        mask.remove(Signal::SIGINT);
        
        assert!(mask.is_empty());
    }

    #[test]
    fn test_signal_fd_creation() {
        let mask = SignalMask::new();
        let sigfd = SignalFd::new(SignalFdFlags::new(), mask);
        
        assert_eq!(sigfd.pending_count(), 0);
    }

    #[test]
    fn test_signal_fd_set_mask() {
        let mut mask = SignalMask::new();
        mask.add(Signal::SIGINT);
        
        let mut sigfd = SignalFd::new(SignalFdFlags::new(), SignalMask::new());
        sigfd.set_mask(mask);
        
        assert!(sigfd.is_masked(Signal::SIGINT));
    }

    #[test]
    fn test_signal_fd_deliver() {
        let mut mask = SignalMask::new();
        mask.add(Signal::SIGINT);
        
        let mut sigfd = SignalFd::new(SignalFdFlags::new(), mask);
        let siginfo = SigInfo::new(2, 100, 1000);
        
        sigfd.deliver_signal(siginfo);
        
        assert_eq!(sigfd.pending_count(), 1);
    }

    #[test]
    fn test_signal_fd_deliver_masked() {
        let mask = SignalMask::new();
        // Don't add SIGINT to mask
        
        let mut sigfd = SignalFd::new(SignalFdFlags::new(), mask);
        let siginfo = SigInfo::new(2, 100, 1000);
        
        sigfd.deliver_signal(siginfo);
        
        assert_eq!(sigfd.pending_count(), 0);
    }

    #[test]
    fn test_signal_fd_read() {
        let mut mask = SignalMask::new();
        mask.add(Signal::SIGINT);
        
        let mut sigfd = SignalFd::new(SignalFdFlags::new(), mask);
        let siginfo = SigInfo::new(2, 100, 1000);
        
        sigfd.deliver_signal(siginfo);
        
        let signals = sigfd.read().unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signo, 2);
    }

    #[test]
    fn test_signal_fd_read_empty() {
        let mut sigfd = SignalFd::new(SignalFdFlags::new(), SignalMask::new());
        
        let result = sigfd.read();
        assert!(result.is_err());
    }
}
