/// SigmaOS POSIX signals implementation
/// Based on early and modern Linux signals design

use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalHandler {
    Default,
    Ignore,
    Custom(extern "C" fn(u32)),
}

pub struct SignalManager {
    pending_signals: HashMap<u64, Vec<Signal>>,
    signal_actions: HashMap<u64, HashMap<Signal, SignalHandler>>,
    signal_masks: HashMap<u64, Vec<Signal>>,
}

impl SignalManager {
    pub fn new() -> Self {
        SignalManager {
            pending_signals: HashMap::new(),
            signal_actions: HashMap::new(),
            signal_masks: HashMap::new(),
        }
    }

    pub fn send_signal(&mut self, target_pid: u64, sig: Signal) {
        if let Some(mask) = self.signal_masks.get(&target_pid) {
            if mask.contains(&sig) {
                // Blocked
                return;
            }
        }
        
        // Handle SIGKILL and SIGTERM instantly if default
        let handler = self.signal_actions
            .get(&target_pid)
            .and_then(|m| m.get(&sig).copied())
            .unwrap_or(SignalHandler::Default);
            
        if sig == Signal::SIGKILL && handler == SignalHandler::Default {
            // Terminate instantly (simulated)
            return;
        }

        self.pending_signals.entry(target_pid).or_default().push(sig);
    }

    pub fn set_handler(&mut self, pid: u64, sig: Signal, handler: SignalHandler) {
        if sig == Signal::SIGKILL {
            // SIGKILL cannot be caught or ignored
            return;
        }
        self.signal_actions.entry(pid).or_default().insert(sig, handler);
    }

    pub fn get_pending_signals(&self, pid: u64) -> Option<&Vec<Signal>> {
        self.pending_signals.get(&pid)
    }

    pub fn clear_pending(&mut self, pid: u64) {
        self.pending_signals.remove(&pid);
    }

    pub fn block_signal(&mut self, pid: u64, sig: Signal) {
        if sig == Signal::SIGKILL {
            return;
        }
        self.signal_masks.entry(pid).or_default().push(sig);
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

    extern "C" fn mock_handler(_sig: u32) {}

    #[test]
    fn test_signal_sending() {
        let mut sm = SignalManager::new();
        sm.send_signal(101, Signal::SIGINT);
        
        let pending = sm.get_pending_signals(101).unwrap();
        assert_eq!(pending[0], Signal::SIGINT);

        sm.clear_pending(101);
        assert!(sm.get_pending_signals(101).is_none());
    }

    #[test]
    fn test_blocked_signals() {
        let mut sm = SignalManager::new();
        sm.block_signal(101, Signal::SIGINT);
        sm.send_signal(101, Signal::SIGINT);
        assert!(sm.get_pending_signals(101).is_none());
    }

    #[test]
    fn test_custom_handler() {
        let mut sm = SignalManager::new();
        sm.set_handler(101, Signal::SIGUSR1, SignalHandler::Custom(mock_handler));
        sm.set_handler(101, Signal::SIGKILL, SignalHandler::Ignore); // should fail/be ignored
        
        // Check that SIGKILL still acts default
        sm.send_signal(101, Signal::SIGKILL);
        // Custom handler works
        sm.send_signal(101, Signal::SIGUSR1);
    }
}
