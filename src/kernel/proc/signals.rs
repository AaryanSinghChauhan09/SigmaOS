/// SigmaOS POSIX signals implementation
/// Based on early and modern Linux signals design
use crate::klib::HashMap;
use crate::kernel::proc::process_lifecycle::{ProcessLifecycleManager};
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
    pub sigterm_escalation_tracker: HashMap<u64, bool>, // Track SIGTERM sent but pending SIGKILL escalation
}

impl SignalManager {
    pub fn new() -> Self {
        SignalManager {
            pending_signals: HashMap::new(),
            signal_actions: HashMap::new(),
            signal_masks: HashMap::new(),
            sigterm_escalation_tracker: HashMap::new(),
        }
    }

    pub fn send_signal(&mut self, target_pid: u64, sig: Signal) {
        if let Some(mask) = self.signal_masks.get(&target_pid) {
            if mask.contains(&sig) {
                // Blocked
                return;
            }
        }

        // Handle SIGKILL and SIGTERM instantly if default handler is mapped
        let handler = self
            .signal_actions
            .get(&target_pid)
            .and_then(|m| m.get(&sig).copied())
            .unwrap_or(SignalHandler::Default);

        if sig == Signal::SIGKILL && handler == SignalHandler::Default {
            // Terminate instantly (forceful)
            return;
        }

        if sig == Signal::SIGTERM {
            self.sigterm_escalation_tracker.insert(target_pid, true);
        }

        self.pending_signals
            .entry(target_pid)
            .or_default()
            .push(sig);
    }

    pub fn set_handler(&mut self, pid: u64, sig: Signal, handler: SignalHandler) {
        if sig == Signal::SIGKILL {
            // SIGKILL cannot be caught or ignored
            return;
        }
        self.signal_actions
            .entry(pid)
            .or_default()
            .insert(sig, handler);
    }

    pub fn get_pending_signals(&self, pid: u64) -> Option<&Vec<Signal>> {
        self.pending_signals.get(&pid)
    }

    pub fn clear_pending(&mut self, pid: u64) {
        self.pending_signals.remove(&pid);
        self.sigterm_escalation_tracker.remove(&pid);
    }

    pub fn block_signal(&mut self, pid: u64, sig: Signal) {
        if sig == Signal::SIGKILL {
            return;
        }
        self.signal_masks.entry(pid).or_default().push(sig);
    }

    /// Propagates a signal to an entire Process Group (PGID), mimicking Linux signal groups
    pub fn propagate_group_signal(&mut self, pgid: u32, sig: Signal, lifecycle: &ProcessLifecycleManager) {
        for (&pid, &group) in &lifecycle.group_ids {
            if group == pgid {
                self.send_signal(pid, sig);
            }
        }
    }

    /// Escalates SIGTERM to SIGKILL if the process fails to terminate gracefully within the limit
    pub fn escalate_sigterm_to_sigkill(&mut self, pid: u64) -> Result<bool, &'static str> {
        if let Some(&is_pending_term) = self.sigterm_escalation_tracker.get(&pid) {
            if is_pending_term {
                // Escalate immediately to SIGKILL as done by systemd/sysvinit on lingering processes
                self.send_signal(pid, Signal::SIGKILL);
                self.sigterm_escalation_tracker.remove(&pid);
                return Ok(true); // Escalated
            }
        }
        Ok(false)
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
    use crate::kernel::proc::process_lifecycle::{ProcessLifecycleManager};
    use crate::kernel::scheduler::{Priority, Process};

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

    #[test]
    fn test_sigterm_group_propagation_and_escalation() {
        let mut sm = SignalManager::new();
        let mut pm = ProcessLifecycleManager::new();

        // Register two processes in process group 2000
        let p1 = Process::new(101, "proc1".to_string(), Priority::Normal);
        let p2 = Process::new(102, "proc2".to_string(), Priority::Normal);
        pm.register_process(p1);
        pm.register_process(p2);
        pm.group_ids.insert(101, 2000);
        pm.group_ids.insert(102, 2000);

        // Propagate SIGTERM to the process group
        sm.propagate_group_signal(2000, Signal::SIGTERM, &pm);

        // Verify SIGTERM was sent to both processes
        assert_eq!(sm.get_pending_signals(101).unwrap()[0], Signal::SIGTERM);
        assert_eq!(sm.get_pending_signals(102).unwrap()[0], Signal::SIGTERM);

        // Verify SIGTERM is tracked in the escalation tracker
        assert_eq!(sm.sigterm_escalation_tracker.get(&101), Some(&true));

        // Escalate proc1 (fails to terminate gracefully) to SIGKILL
        let escalated = sm.escalate_sigterm_to_sigkill(101).unwrap();
        assert!(escalated);

        // Verify proc1 now has SIGKILL pending
        let pending_101 = sm.get_pending_signals(101).unwrap();
        assert!(pending_101.contains(&Signal::SIGKILL));
    }
}
