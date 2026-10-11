// SigmaOS Sovereign Kernel Signal Interruption & Delivery Suite
// (`src/kernel/sovereign_kernel_signal_interruption_suite.rs`)
//
// Safe, zero-dependency Rust kernel implementation for POSIX.1-2024 signal processing,
// kernel-to-user return frame delivery, and blocking syscall interruption semantics.
// Inspired by Linux kernel 6.12+ signal dispatch and FreeBSD process descriptor signals.

#![allow(clippy::new_without_default)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::signal::sovereign_signal_development_and_pr_suite::{
    ProcessSignalState, SigAction, SigActionFlags, SigHandler, SigInfo, SigSet,
    SignalDefaultAction, SignalNumber, ThreadSignalState, UserSignalFrame,
};

/// Kernel Signal Interruption Result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelSignalDispatchResult {
    NoSignalPending,
    DeliveredHandlerFrame(u64), // Frame pointer on user stack
    ExecutedDefaultAction(SignalDefaultAction),
    BlockedByMask,
    IgnoredByDisposition,
}

/// Kernel Syscall Interruption Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallInterruptionAction {
    NoInterruption,
    ReturnEintr,
    RestartSyscall,
}

/// Sovereign Kernel Signal Interruption Engine
pub struct SovereignKernelSignalInterruptionEngine {
    pub processes: BTreeMap<u32, ProcessSignalState>,
    pub threads: BTreeMap<u32, ThreadSignalState>,
    pub active_signalfds: BTreeMap<i32, (u32, SigSet)>, // fd -> (pid, mask)
}

impl SovereignKernelSignalInterruptionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            processes: BTreeMap::new(),
            threads: BTreeMap::new(),
            active_signalfds: BTreeMap::new(),
        };

        // Seed initial kernel init process (PID 1) and main kernel thread (TID 1)
        engine.processes.insert(1, ProcessSignalState::new(1, 1, 0, 0));
        engine.threads.insert(1, ThreadSignalState::new(1, 1));
        engine
    }

    /// Enqueues a signal into a target process or thread queue with permission check
    pub fn send_signal(
        &mut self,
        sender_uid: u32,
        target_pid: u32,
        target_tid: Option<u32>,
        sig: SignalNumber,
        info: SigInfo,
    ) -> Result<(), &'static str> {
        let target_proc = self.processes.get_mut(&target_pid).ok_or("ESRCH: Target process not found")?;

        // Permission check: Root (UID 0) or matching UID
        if sender_uid != 0 && sender_uid != target_proc.uid {
            return Err("EPERM: Permission denied to send signal");
        }

        // Real-time signal queue limit check
        if sig.is_realtime() && target_proc.pending_process_signals.len() >= target_proc.real_time_queue_limit {
            return Err("EAGAIN: Real-time signal queue limit exceeded");
        }

        if let Some(tid) = target_tid {
            let thread = self.threads.get_mut(&tid).ok_or("ESRCH: Target thread not found")?;
            thread.pending_thread_signals.push((sig, info));
        } else {
            target_proc.pending_process_signals.push((sig, info));
        }

        Ok(())
    }

    /// Evaluates and selects pending signals for delivery at kernel-to-user return boundary
    pub fn process_kernel_to_user_return(
        &mut self,
        pid: u32,
        tid: u32,
    ) -> Result<KernelSignalDispatchResult, &'static str> {
        let (blocked, alt_stack) = {
            let thread = self.threads.get(&tid).ok_or("Thread not found")?;
            (thread.blocked_mask, thread.alt_stack)
        };

        let mut selected_sig: Option<(SignalNumber, usize)> = None;
        let mut selected_info = None;
        let mut is_thread_directed = false;

        if let Some(thread) = self.threads.get(&tid) {
            for (idx, (sig, info)) in thread.pending_thread_signals.iter().enumerate() {
                if !blocked.is_member(*sig) || sig.is_uncatchable() {
                    selected_sig = Some((*sig, idx));
                    selected_info = Some(*info);
                    is_thread_directed = true;
                    break;
                }
            }
        }

        if selected_sig.is_none() {
            if let Some(proc_state) = self.processes.get(&pid) {
                for (idx, (sig, info)) in proc_state.pending_process_signals.iter().enumerate() {
                    if !blocked.is_member(*sig) || sig.is_uncatchable() {
                        selected_sig = Some((*sig, idx));
                        selected_info = Some(*info);
                        break;
                    }
                }
            }
        }

        let (sig, idx) = match selected_sig {
            Some(pair) => pair,
            None => return Ok(KernelSignalDispatchResult::NoSignalPending),
        };

        if is_thread_directed {
            if let Some(thread_mut) = self.threads.get_mut(&tid) {
                thread_mut.pending_thread_signals.remove(idx);
            }
        } else {
            if let Some(proc_mut) = self.processes.get_mut(&pid) {
                proc_mut.pending_process_signals.remove(idx);
            }
        }

        let act = self
            .processes
            .get(&pid)
            .and_then(|p| p.dispositions.get(&sig))
            .cloned()
            .unwrap_or_default();

        match act.sa_handler {
            SigHandler::Ignore => {
                if sig.is_uncatchable() {
                    let default_act = sig.default_action();
                    if let Some(proc_mut) = self.processes.get_mut(&pid) {
                        proc_mut.is_terminated = true;
                    }
                    Ok(KernelSignalDispatchResult::ExecutedDefaultAction(default_act))
                } else {
                    Ok(KernelSignalDispatchResult::IgnoredByDisposition)
                }
            }
            SigHandler::Default => {
                let default_act = sig.default_action();
                if let Some(proc_mut) = self.processes.get_mut(&pid) {
                    match default_act {
                        SignalDefaultAction::Terminate | SignalDefaultAction::CoreDump => {
                            proc_mut.is_terminated = true;
                        }
                        SignalDefaultAction::Stop => {
                            proc_mut.is_stopped = true;
                        }
                        SignalDefaultAction::Continue => {
                            proc_mut.is_stopped = false;
                        }
                        SignalDefaultAction::Ignore => {}
                    }
                }
                Ok(KernelSignalDispatchResult::ExecutedDefaultAction(default_act))
            }
            SigHandler::UserHandler(handler_addr) => {
                let _frame = UserSignalFrame {
                    frame_version: 1,
                    saved_registers: vec![0x1000, 0x2000, 0x3000, 0x4000],
                    saved_mask: blocked,
                    handler_ptr: handler_addr,
                    trampoline_ptr: act.sa_restorer,
                    siginfo: selected_info.unwrap_or_default(),
                    is_valid: true,
                };

                let user_sp = if alt_stack.is_enabled {
                    alt_stack.ss_sp + alt_stack.ss_size - 128
                } else {
                    0x7fff0000 - 128
                };

                if let Some(thread_mut) = self.threads.get_mut(&tid) {
                    thread_mut.blocked_mask.block_mask(act.sa_mask);
                    if !act.sa_flags.sa_nodefer {
                        thread_mut.blocked_mask.add_member(sig);
                    }
                }

                Ok(KernelSignalDispatchResult::DeliveredHandlerFrame(user_sp))
            }
        }
    }

    /// Evaluates syscall interruption behavior when a signal arrives while blocked
    pub fn evaluate_syscall_interruption(
        &mut self,
        pid: u32,
        tid: u32,
        sig: SignalNumber,
    ) -> Result<SyscallInterruptionAction, &'static str> {
        let thread = self.threads.get_mut(&tid).ok_or("Thread not found")?;
        if !thread.is_blocked_on_syscall {
            return Ok(SyscallInterruptionAction::NoInterruption);
        }

        let proc_state = self.processes.get(&pid).ok_or("Process not found")?;
        let act = proc_state.dispositions.get(&sig).cloned().unwrap_or_default();

        thread.is_blocked_on_syscall = false;

        if act.sa_flags.sa_restart {
            Ok(SyscallInterruptionAction::RestartSyscall)
        } else {
            Ok(SyscallInterruptionAction::ReturnEintr)
        }
    }

    /// Validates a user signal frame upon `sigreturn` syscall invocation
    pub fn validate_sigreturn_frame(
        &mut self,
        tid: u32,
        frame: &UserSignalFrame,
    ) -> Result<(), &'static str> {
        if !frame.is_valid || frame.frame_version != 1 {
            return Err("EFAULT: Invalid or corrupted signal return frame");
        }

        let thread = self.threads.get_mut(&tid).ok_or("Thread not found")?;
        thread.blocked_mask = frame.saved_mask;
        thread.blocked_mask.sanitize_unblockable();
        Ok(())
    }
}

impl Default for SovereignKernelSignalInterruptionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_and_process_handler_frame() {
        let mut engine = SovereignKernelSignalInterruptionEngine::new();

        // Install user handler for SIGUSR1 on PID 1
        if let Some(p) = engine.processes.get_mut(&1) {
            p.dispositions.insert(
                SignalNumber::SIGUSR1,
                SigAction {
                    sa_handler: SigHandler::UserHandler(0x8048000),
                    sa_mask: SigSet::empty(),
                    sa_flags: SigActionFlags::default(),
                    sa_restorer: 0x8049000,
                },
            );
        }

        // Send SIGUSR1 to PID 1
        assert!(engine
            .send_signal(0, 1, None, SignalNumber::SIGUSR1, SigInfo::default())
            .is_ok());

        // Process kernel-to-user return
        let result = engine.process_kernel_to_user_return(1, 1).unwrap();
        match result {
            KernelSignalDispatchResult::DeliveredHandlerFrame(sp) => {
                assert_eq!(sp, 0x7fff0000 - 128);
            }
            _ => panic!("Expected DeliveredHandlerFrame"),
        }
    }

    #[test]
    fn test_permission_denied_kill() {
        let mut engine = SovereignKernelSignalInterruptionEngine::new();
        engine.processes.insert(2, ProcessSignalState::new(2, 2, 1000, 1000));

        // Sending signal from UID 1001 to UID 1000 process must fail
        let res = engine.send_signal(1001, 2, None, SignalNumber::SIGTERM, SigInfo::default());
        assert!(res.is_err());
    }

    #[test]
    fn test_uncatchable_signals_bypass_mask() {
        let mut engine = SovereignKernelSignalInterruptionEngine::new();

        // Block SIGKILL in thread mask (kernel must ignore this block)
        if let Some(t) = engine.threads.get_mut(&1) {
            t.blocked_mask.add_member(SignalNumber::SIGKILL);
        }

        // Send SIGKILL to PID 1
        assert!(engine
            .send_signal(0, 1, None, SignalNumber::SIGKILL, SigInfo::default())
            .is_ok());

        // SIGKILL must be delivered and execute default Terminate action despite blocked mask
        let res = engine.process_kernel_to_user_return(1, 1).unwrap();
        assert_eq!(
            res,
            KernelSignalDispatchResult::ExecutedDefaultAction(SignalDefaultAction::Terminate)
        );
        assert!(engine.processes.get(&1).unwrap().is_terminated);
    }

    #[test]
    fn test_syscall_interruption_and_restart() {
        let mut engine = SovereignKernelSignalInterruptionEngine::new();

        if let Some(t) = engine.threads.get_mut(&1) {
            t.is_blocked_on_syscall = true;
        }

        // Test without SA_RESTART -> Returns ReturnEintr
        let action1 = engine
            .evaluate_syscall_interruption(1, 1, SignalNumber::SIGINT)
            .unwrap();
        assert_eq!(action1, SyscallInterruptionAction::ReturnEintr);

        // Test with SA_RESTART
        if let Some(p) = engine.processes.get_mut(&1) {
            p.dispositions.insert(
                SignalNumber::SIGUSR1,
                SigAction {
                    sa_handler: SigHandler::UserHandler(0x8048000),
                    sa_mask: SigSet::empty(),
                    sa_flags: SigActionFlags {
                        sa_restart: true,
                        ..Default::default()
                    },
                    sa_restorer: 0,
                },
            );
        }

        if let Some(t) = engine.threads.get_mut(&1) {
            t.is_blocked_on_syscall = true;
        }

        let action2 = engine
            .evaluate_syscall_interruption(1, 1, SignalNumber::SIGUSR1)
            .unwrap();
        assert_eq!(action2, SyscallInterruptionAction::RestartSyscall);
    }

    #[test]
    fn test_sigreturn_frame_validation() {
        let mut engine = SovereignKernelSignalInterruptionEngine::new();

        let valid_frame = UserSignalFrame {
            frame_version: 1,
            saved_registers: vec![1, 2, 3],
            saved_mask: SigSet::empty(),
            handler_ptr: 0x8048000,
            trampoline_ptr: 0x8049000,
            siginfo: SigInfo::default(),
            is_valid: true,
        };

        assert!(engine.validate_sigreturn_frame(1, &valid_frame).is_ok());

        let invalid_frame = UserSignalFrame {
            is_valid: false,
            ..valid_frame
        };

        assert!(engine.validate_sigreturn_frame(1, &invalid_frame).is_err());
    }

    #[test]
    fn test_realtime_signal_queue_bounds() {
        let mut engine = SovereignKernelSignalInterruptionEngine::new();
        if let Some(p) = engine.processes.get_mut(&1) {
            p.real_time_queue_limit = 2;
        }

        assert!(engine
            .send_signal(0, 1, None, SignalNumber::SIGRTMIN, SigInfo::default())
            .is_ok());
        assert!(engine
            .send_signal(0, 1, None, SignalNumber::SIGRT1, SigInfo::default())
            .is_ok());

        // Exceeding queue limit must fail with EAGAIN
        assert!(engine
            .send_signal(0, 1, None, SignalNumber::SIGRT2, SigInfo::default())
            .is_err());
    }
}
