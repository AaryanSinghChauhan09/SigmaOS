#![allow(clippy::new_without_default)]
#![allow(dead_code)]
//! High-performance BSD & Linux inspired Signal Subsystem for SigmaOS
//! Implements standard signals, custom signal actions, signal masking,
//! real-time queueable signals with custom payloads, and capability-gated signal delivery.



extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use crate::ipc::ipc::{IPCError, IPCCapability};

/// Unix/BSD standard and custom signal numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignalType {
    SigInt = 2,
    SigKill = 9,
    SigUsr1 = 10,
    SigSegv = 11,
    SigUsr2 = 12,
    SigTerm = 15,
}

impl SignalType {
    pub fn to_bit(&self) -> u32 {
        1 << (*self as u32)
    }
}

/// Signal Actions (equivalent to sigaction disposition)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalDisposition {
    Default,
    Ignore,
    CustomHandler(usize), // Userland function pointer/handler ID
}

/// Represent a signal pending delivery
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingSignal {
    pub signal_type: SignalType,
    pub sender_pid: u32,
    pub payload: Option<Vec<u8>>, // Real-time signal user data payload
}

/// Represent the signal state for a single process
pub struct ProcessSignalState {
    pub pid: u32,
    pub signal_mask: u32, // Bitmask of blocked signals (sigprocmask)
    pub pending_signals: Vec<PendingSignal>,
    pub dispositions: BTreeMap<SignalType, SignalDisposition>,
    pub capability: IPCCapability,
}

impl ProcessSignalState {
    pub fn new(pid: u32, capability: IPCCapability) -> Self {
        Self {
            pid,
            signal_mask: 0,
            pending_signals: Vec::new(),
            dispositions: BTreeMap::new(),
            capability,
        }
    }

    /// Set signal action (sigaction)
    pub fn set_action(&mut self, signal: SignalType, disposition: SignalDisposition) -> Result<(), IPCError> {
        // Cannot ignore SIGKILL (standard Unix/BSD rule)
        if signal == SignalType::SigKill && disposition == SignalDisposition::Ignore {
            return Err(IPCError::PermissionDenied);
        }
        self.dispositions.insert(signal, disposition);
        Ok(())
    }

    /// Set signal mask (sigprocmask)
    pub fn set_mask(&mut self, mask: u32) {
        // Cannot block SIGKILL (standard Unix/BSD rule)
        let sigkill_bit = SignalType::SigKill.to_bit();
        self.signal_mask = mask & !sigkill_bit;
    }

    /// Queue a pending signal
    pub fn queue_signal(&mut self, pending: PendingSignal) -> Result<(), IPCError> {
        self.pending_signals.push(pending);
        Ok(())
    }

    /// Process/Retrieve pending signals that are not blocked
    pub fn dispatch_next_signal(&mut self) -> Option<(PendingSignal, SignalDisposition)> {
        let mut index = None;
        for (i, pending) in self.pending_signals.iter().enumerate() {
            let signal_bit = pending.signal_type.to_bit();
            if (self.signal_mask & signal_bit) == 0 {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            let pending = self.pending_signals.remove(i);
            let disp = self.dispositions.get(&pending.signal_type).copied().unwrap_or(SignalDisposition::Default);
            Some((pending, disp))
        } else {
            None
        }
    }
}

/// Dynamic manager for asynchronous signal delivery and capability enforcement
pub struct SignalDeliverySystem {
    pub processes: BTreeMap<u32, ProcessSignalState>,
}

impl SignalDeliverySystem {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
        }
    }

    /// Register a process with the system
    pub fn register_process(&mut self, pid: u32, capability: IPCCapability) {
        let state = ProcessSignalState::new(pid, capability);
        self.processes.insert(pid, state);
    }

    /// Send a signal (kill/sigqueue) from one process to another
    pub fn send_signal(
        &mut self,
        sender_pid: u32,
        receiver_pid: u32,
        signal_type: SignalType,
        payload: Option<Vec<u8>>,
    ) -> Result<(), IPCError> {
        // Capability-gated check
        {
            let sender = self.processes.get(&sender_pid).ok_or(IPCError::NotConnected)?;
            if !sender.capability.can_send {
                return Err(IPCError::PermissionDenied);
            }
        }

        let receiver = self.processes.get_mut(&receiver_pid).ok_or(IPCError::NotConnected)?;
        if !receiver.capability.can_receive {
            return Err(IPCError::PermissionDenied);
        }

        let pending = PendingSignal {
            signal_type,
            sender_pid,
            payload,
        };

        receiver.queue_signal(pending)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_delivery() {
        let mut sds = SignalDeliverySystem::new();
        let cap = IPCCapability::full();

        sds.register_process(100, cap);
        sds.register_process(101, cap);

        // Register custom handler on receiver
        {
            let p101 = sds.processes.get_mut(&101).unwrap();
            assert!(p101.set_action(SignalType::SigUsr1, SignalDisposition::CustomHandler(0x1234)).is_ok());
        }

        // Send Usr1 signal from PID 100 to PID 101
        assert!(sds.send_signal(100, 101, SignalType::SigUsr1, None).is_ok());

        // Dispatch Usr1 signal
        {
            let p101 = sds.processes.get_mut(&101).unwrap();
            let (pending, disp) = p101.dispatch_next_signal().unwrap();
            assert_eq!(pending.signal_type, SignalType::SigUsr1);
            assert_eq!(pending.sender_pid, 100);
            assert_eq!(disp, SignalDisposition::CustomHandler(0x1234));
        }
    }

    #[test]
    fn test_signal_masking() {
        let mut sds = SignalDeliverySystem::new();
        let cap = IPCCapability::full();

        sds.register_process(200, cap);

        let p200 = sds.processes.get_mut(&200).unwrap();
        // Mask (block) SIGUSR1
        p200.set_mask(SignalType::SigUsr1.to_bit());

        let pending = PendingSignal {
            signal_type: SignalType::SigUsr1,
            sender_pid: 201,
            payload: None,
        };
        p200.queue_signal(pending).unwrap();

        // Should not dispatch Usr1 because it is blocked/masked
        assert!(p200.dispatch_next_signal().is_none());

        // Queue SigKill
        let kill_sig = PendingSignal {
            signal_type: SignalType::SigKill,
            sender_pid: 201,
            payload: None,
        };
        p200.queue_signal(kill_sig).unwrap();

        // Cannot block SigKill; should dispatch SigKill even if we attempted to mask it
        p200.set_mask(0xFFFF_FFFF); // Try to block everything
        let (dispatched, disp) = p200.dispatch_next_signal().unwrap();
        assert_eq!(dispatched.signal_type, SignalType::SigKill);
        assert_eq!(disp, SignalDisposition::Default);
    }

    #[test]
    fn test_realtime_signals() {
        let mut sds = SignalDeliverySystem::new();
        let cap = IPCCapability::full();

        sds.register_process(300, cap);
        sds.register_process(301, cap);

        let payload_data = Vec::from(b"Realtime Payload");
        assert!(sds.send_signal(300, 301, SignalType::SigUsr2, Some(payload_data.clone())).is_ok());

        let p301 = sds.processes.get_mut(&301).unwrap();
        let (pending, _) = p301.dispatch_next_signal().unwrap();
        assert_eq!(pending.signal_type, SignalType::SigUsr2);
        assert_eq!(pending.payload, Some(payload_data));
    }

    #[test]
    fn test_signal_delivery_permissions() {
        let mut sds = SignalDeliverySystem::new();
        let mut cap_sender = IPCCapability::full();
        cap_sender.can_send = false; // No send permission

        sds.register_process(400, cap_sender);
        sds.register_process(401, IPCCapability::full());

        // Should fail because sender lacks send permission
        assert!(sds.send_signal(400, 401, SignalType::SigInt, None).is_err());
    }
}
