#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// SigmaOS POSIX signals implementation
/// Based on early and modern Linux signals design
extern crate alloc;
use crate::klib::BTreeMap;
use crate::kernel::proc::process_lifecycle::{ProcessLifecycleManager};
extern crate alloc;
use alloc::vec::Vec;

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
    SIGSTOP = 17,
    SIGTSTP = 18,
    SIGCONT = 19,
    SIGCHLD = 20,
}

/// Detailed siginfo_t structure inspired by POSIX/FreeBSD SA_SIGINFO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigInfo {
    pub signo: Signal,
    pub errno: i32,
    pub code: i32,
    pub sender_pid: u64,
    pub sender_uid: u32,
    pub fault_addr: u64,
}

impl SigInfo {
    pub fn simple(signo: Signal, sender_pid: u64) -> Self {
        Self {
            signo,
            errno: 0,
            code: 0,
            sender_pid,
            sender_uid: 0,
            fault_addr: 0,
        }
    }
}

/// Alternate signal stack configuration (POSIX sigaltstack)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigAltStack {
    pub ss_sp: u64,
    pub ss_flags: u32,
    pub ss_size: usize,
}

/// Linux OOM-killer score adjustment (-1000 to +1000)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OomScoreAdjustment(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalHandler {
    Default,
    Ignore,
    Custom(extern "C" fn(u32)),
    SigInfoCustom(extern "C" fn(u32, *const SigInfo, *const u8)),
}

impl PartialEq for SignalHandler {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SignalHandler::Default, SignalHandler::Default) => true,
            (SignalHandler::Ignore, SignalHandler::Ignore) => true,
            // Custom handlers: compare by raw function pointer address (cast to usize).
            // This avoids direct function pointer comparison (which is unpredictable)
            // and instead compares the underlying address values, which is deterministic.
            (SignalHandler::Custom(a), SignalHandler::Custom(b)) => (*a as usize) == (*b as usize),
            _ => false,
        }
    }
}

impl Eq for SignalHandler {}

pub struct SignalManager {
    pending_signals: BTreeMap<u64, Vec<Signal>>,
    pending_siginfo: BTreeMap<u64, Vec<SigInfo>>,
    signal_actions: BTreeMap<u64, BTreeMap<Signal, SignalHandler>>,
    signal_masks: BTreeMap<u64, Vec<Signal>>,
    alt_stacks: BTreeMap<u64, SigAltStack>,
    oom_score_adj: BTreeMap<u64, OomScoreAdjustment>,
    pub sigterm_escalation_tracker: BTreeMap<u64, bool>, // Track SIGTERM sent but pending SIGKILL escalation
}

impl SignalManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SignalManager {
            pending_signals: BTreeMap::new(),
            pending_siginfo: BTreeMap::new(),
            signal_actions: BTreeMap::new(),
            signal_masks: BTreeMap::new(),
            alt_stacks: BTreeMap::new(),
            oom_score_adj: BTreeMap::new(),
            sigterm_escalation_tracker: BTreeMap::new(),
        }
    }

    pub fn send_signal(&mut self, target_pid: u64, sig: Signal) {
        self.send_siginfo(target_pid, SigInfo::simple(sig, 0));
    }

    pub fn send_siginfo(&mut self, target_pid: u64, info: SigInfo) {
        let sig = info.signo;
        if sig != Signal::SIGKILL && sig != Signal::SIGSTOP {
            if let Some(mask) = self.signal_masks.get(&target_pid) {
                if mask.contains(&sig) {
                    // Blocked
                    return;
                }
            }
        }

        let handler = self
            .signal_actions
            .get(&target_pid)
            .and_then(|m| m.get(&sig).copied())
            .unwrap_or(SignalHandler::Default);

        if sig == Signal::SIGKILL && handler == SignalHandler::Default {
            self.pending_signals
                .entry(target_pid)
                .or_default()
                .push(sig);
            self.pending_siginfo
                .entry(target_pid)
                .or_default()
                .push(info);
            return;
        }

        if sig == Signal::SIGTERM {
            self.sigterm_escalation_tracker.insert(target_pid, true);
        }

        self.pending_signals
            .entry(target_pid)
            .or_default()
            .push(sig);
        self.pending_siginfo
            .entry(target_pid)
            .or_default()
            .push(info);
    }

    pub fn set_handler(&mut self, pid: u64, sig: Signal, handler: SignalHandler) {
        if sig == Signal::SIGKILL || sig == Signal::SIGSTOP {
            // SIGKILL and SIGSTOP cannot be caught or ignored
            return;
        }
        self.signal_actions
            .entry(pid)
            .or_default()
            .insert(sig, handler);
    }

    pub fn set_alt_stack(&mut self, pid: u64, stack: SigAltStack) {
        self.alt_stacks.insert(pid, stack);
    }

    pub fn get_alt_stack(&self, pid: u64) -> Option<&SigAltStack> {
        self.alt_stacks.get(&pid)
    }

    pub fn set_oom_score_adj(&mut self, pid: u64, adj: OomScoreAdjustment) {
        let clamped_val = adj.0.clamp(-1000, 1000);
        self.oom_score_adj.insert(pid, OomScoreAdjustment(clamped_val));
    }

    pub fn get_oom_score_adj(&self, pid: u64) -> OomScoreAdjustment {
        self.oom_score_adj.get(&pid).copied().unwrap_or(OomScoreAdjustment(0))
    }

    /// Trigger Linux-style Out-Of-Memory (OOM) killer to select and terminate the highest score process
    pub fn trigger_oom_killer(&mut self, process_memory_usage: &BTreeMap<u64, usize>) -> Option<u64> {
        let mut highest_pid = None;
        let mut highest_score: i64 = -10000;

        for (&pid, &mem_bytes) in process_memory_usage {
            let adj = self.get_oom_score_adj(pid).0 as i64;
            if adj == -1000 {
                // -1000 disables OOM killer targeting (OOM_SCORE_ADJ_MIN)
                continue;
            }
            let base_score = (mem_bytes / (1024 * 1024)) as i64; // MB used
            let final_score = base_score + (adj * 10);
            if final_score > highest_score {
                highest_score = final_score;
                highest_pid = Some(pid);
            }
        }

        if let Some(target_pid) = highest_pid {
            self.send_signal(target_pid, Signal::SIGKILL);
        }

        highest_pid
    }

    pub fn get_pending_signals(&self, pid: u64) -> Option<&Vec<Signal>> {
        self.pending_signals.get(&pid)
    }

    pub fn get_pending_siginfo(&self, pid: u64) -> Option<&Vec<SigInfo>> {
        self.pending_siginfo.get(&pid)
    }

    pub fn clear_pending(&mut self, pid: u64) {
        self.pending_signals.remove(&pid);
        self.pending_siginfo.remove(&pid);
        self.sigterm_escalation_tracker.remove(&pid);
    }

    pub fn block_signal(&mut self, pid: u64, sig: Signal) {
        if sig == Signal::SIGKILL || sig == Signal::SIGSTOP {
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
    use crate::kernel::proc::process_lifecycle::mock_scheduler::{Priority, Process};

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

    #[test]
    fn test_siginfo_and_sigaltstack() {
        let mut sm = SignalManager::new();
        let info = SigInfo {
            signo: Signal::SIGSEGV,
            errno: 0,
            code: 1, // SEGV_MAPERR
            sender_pid: 1001,
            sender_uid: 100,
            fault_addr: 0xDEADBEEF,
        };

        sm.send_siginfo(101, info);

        let pending_info = sm.get_pending_siginfo(101).unwrap();
        assert_eq!(pending_info[0].signo, Signal::SIGSEGV);
        assert_eq!(pending_info[0].fault_addr, 0xDEADBEEF);

        let stack = SigAltStack {
            ss_sp: 0x8000000,
            ss_flags: 0,
            ss_size: 16384,
        };
        sm.set_alt_stack(101, stack);
        assert_eq!(sm.get_alt_stack(101), Some(&stack));
    }

    #[test]
    fn test_oom_killer_selection() {
        let mut sm = SignalManager::new();
        sm.set_oom_score_adj(101, OomScoreAdjustment(0));
        sm.set_oom_score_adj(102, OomScoreAdjustment(500));
        sm.set_oom_score_adj(103, OomScoreAdjustment(-1000)); // Protected from OOM killer

        let mut mem_usage = BTreeMap::new();
        mem_usage.insert(101, 500 * 1024 * 1024); // 500 MB
        mem_usage.insert(102, 200 * 1024 * 1024); // 200 MB
        mem_usage.insert(103, 1000 * 1024 * 1024); // 1000 MB (protected)

        let killed_pid = sm.trigger_oom_killer(&mem_usage).unwrap();
        assert_eq!(killed_pid, 102); // 102 has score 200 + 5000 = 5200 vs 101 score 500

        let pending = sm.get_pending_signals(102).unwrap();
        assert!(pending.contains(&Signal::SIGKILL));
    }

    #[test]
    fn test_sigstop_non_overrideable() {
        let mut sm = SignalManager::new();
        sm.set_handler(101, Signal::SIGSTOP, SignalHandler::Ignore); // Should be ignored
        sm.block_signal(101, Signal::SIGSTOP); // Should not block SIGSTOP

        sm.send_signal(101, Signal::SIGSTOP);
        let pending = sm.get_pending_signals(101).unwrap();
        assert_eq!(pending[0], Signal::SIGSTOP);
    }
}
