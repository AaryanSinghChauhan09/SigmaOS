/// SigmaOS POSIX Signal Handling Engine
/// Manages sigaction disposition tables, RT signal queuing, and per-thread masks.
///
/// Standard library (std, libc) are fully authorized per SigmaOS policy.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Signal disposition types
#[derive(Debug, Clone, PartialEq)]
pub enum SigDisposition {
    Default,
    Ignore,
    Handler(u64), // function pointer address
}

/// Real-time signal queue entry
#[derive(Debug, Clone)]
pub struct SigQueueEntry {
    pub signo: i32,
    pub priority: i32, // lower = higher priority for RT signals
    pub sender_pid: u32,
    pub value: i64,
}

/// Per-process signal state
#[derive(Debug)]
pub struct SigmaSignalState {
    pub dispositions: HashMap<i32, SigDisposition>,
    pub pending_mask: u64,    // bitmask of pending signals
    pub blocked_mask: u64,    // sigprocmask
    pub rt_queue: Vec<SigQueueEntry>,
    pub sa_restart_flags: u64, // bitmask of signals with SA_RESTART
}

impl SigmaSignalState {
    pub fn new() -> Self {
        let mut dispositions = HashMap::new();
        // Standard default dispositions
        for sig in 1..=64i32 {
            dispositions.insert(sig, SigDisposition::Default);
        }
        // SIGCHLD and SIGURG are ignored by default in many contexts
        dispositions.insert(17, SigDisposition::Ignore); // SIGCHLD -> SIG_DFL (wait)
        dispositions.insert(23, SigDisposition::Ignore); // SIGURG

        Self {
            dispositions,
            pending_mask: 0,
            blocked_mask: 0,
            rt_queue: Vec::new(),
            sa_restart_flags: 0,
        }
    }

    /// Install a signal disposition (analogous to sigaction(2))
    pub fn set_disposition(&mut self, signo: i32, disposition: SigDisposition, sa_restart: bool) -> Result<(), &'static str> {
        if signo < 1 || signo > 64 {
            return Err("invalid signal number");
        }
        // SIGKILL (9) and SIGSTOP (19) cannot be caught or ignored
        if signo == 9 || signo == 19 {
            return Err("SIGKILL and SIGSTOP cannot be caught or ignored");
        }
        self.dispositions.insert(signo, disposition);
        if sa_restart {
            self.sa_restart_flags |= 1u64 << (signo - 1);
        } else {
            self.sa_restart_flags &= !(1u64 << (signo - 1));
        }
        Ok(())
    }

    /// Block/unblock signals (analogous to sigprocmask(2))
    pub fn update_mask(&mut self, how: u8, mask: u64) -> u64 {
        let old = self.blocked_mask;
        match how {
            0 => self.blocked_mask |= mask,   // SIG_BLOCK
            1 => self.blocked_mask &= !mask,  // SIG_UNBLOCK
            2 => self.blocked_mask = mask,    // SIG_SETMASK
            _ => {}
        }
        old
    }

    /// Enqueue an RT signal (mq_notify style)
    pub fn enqueue_rt_signal(&mut self, entry: SigQueueEntry) -> Result<(), &'static str> {
        if entry.signo < 32 || entry.signo > 64 {
            return Err("not an RT signal");
        }
        if self.rt_queue.len() >= 8192 {
            return Err("RT signal queue full (rtsig-max)");
        }
        self.rt_queue.push(entry);
        self.rt_queue.sort_by_key(|e| e.priority);
        Ok(())
    }

    /// Dequeue the highest priority pending RT signal
    pub fn dequeue_rt_signal(&mut self) -> Option<SigQueueEntry> {
        if self.rt_queue.is_empty() { return None; }
        Some(self.rt_queue.remove(0))
    }

    /// Check if signal is currently blocked
    pub fn is_blocked(&self, signo: i32) -> bool {
        if signo < 1 || signo > 64 { return false; }
        (self.blocked_mask >> (signo - 1)) & 1 == 1
    }

    /// Raise a signal (mark as pending)
    pub fn raise_signal(&mut self, signo: i32) -> Result<(), &'static str> {
        if signo < 1 || signo > 64 {
            return Err("invalid signal number");
        }
        self.pending_mask |= 1u64 << (signo - 1);
        Ok(())
    }

    /// Get pending but unblocked signals
    pub fn deliverable_signals(&self) -> u64 {
        self.pending_mask & !self.blocked_mask
    }
}

/// Global signal manager (Arc<Mutex<>> for thread safety)
pub struct SigmaSignalManager {
    states: Arc<Mutex<HashMap<u32, SigmaSignalState>>>,
}

impl SigmaSignalManager {
    pub fn new() -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_process(&self, pid: u32) {
        let mut map = self.states.lock().unwrap();
        map.insert(pid, SigmaSignalState::new());
    }

    pub fn send_signal(&self, pid: u32, signo: i32) -> Result<(), &'static str> {
        let mut map = self.states.lock().unwrap();
        let state = map.get_mut(&pid).ok_or("process not found")?;
        state.raise_signal(signo)
    }

    pub fn get_deliverable(&self, pid: u32) -> u64 {
        let map = self.states.lock().unwrap();
        map.get(&pid).map(|s| s.deliverable_signals()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state_has_64_dispositions() {
        let state = SigmaSignalState::new();
        assert_eq!(state.dispositions.len(), 64);
    }

    #[test]
    fn test_set_handler_disposition() {
        let mut state = SigmaSignalState::new();
        let result = state.set_disposition(10, SigDisposition::Handler(0xdeadbeef), true);
        assert!(result.is_ok());
        assert_eq!(*state.dispositions.get(&10).unwrap(), SigDisposition::Handler(0xdeadbeef));
        assert!(state.sa_restart_flags & (1 << 9) != 0);
    }

    #[test]
    fn test_sigkill_cannot_be_caught() {
        let mut state = SigmaSignalState::new();
        let result = state.set_disposition(9, SigDisposition::Ignore, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_signal_masking() {
        let mut state = SigmaSignalState::new();
        state.raise_signal(1).unwrap(); // SIGHUP
        assert_eq!(state.deliverable_signals() & 1, 1);
        state.update_mask(0, 1); // SIG_BLOCK SIGHUP
        assert_eq!(state.deliverable_signals() & 1, 0);
    }

    #[test]
    fn test_rt_signal_queue_ordering() {
        let mut state = SigmaSignalState::new();
        state.enqueue_rt_signal(SigQueueEntry { signo: 34, priority: 2, sender_pid: 100, value: 0 }).unwrap();
        state.enqueue_rt_signal(SigQueueEntry { signo: 35, priority: 1, sender_pid: 101, value: 0 }).unwrap();
        let first = state.dequeue_rt_signal().unwrap();
        assert_eq!(first.priority, 1); // higher priority first
    }
}
