// SigmaOS Minix 3 Synchronous IPC Rendez-vous & Microkernel Guard
// Reference: Minix 3 IPC architecture (send, receive, sendrec, notify)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const IPC_ENDPOINT_ANY: i32 = -1;
pub const IPC_ENDPOINT_NONE: i32 = -2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcCallType {
    Send,
    Receive,
    SendRec,
    Notify,
}

/// Standard Minix 3 IPC Message Envelope (64 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinixMessage {
    pub m_source: i32,
    pub m_type: i32,
    pub m_u: [u8; 56],
}

impl MinixMessage {
    pub fn new(source: i32, msg_type: i32, payload: &[u8]) -> Self {
        let mut u = [0u8; 56];
        let len = payload.len().min(56);
        u[..len].copy_from_slice(&payload[..len]);
        Self {
            m_source: source,
            m_type: msg_type,
            m_u: u,
        }
    }
}

/// Rendez-vous Process Wait State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RendezvousState {
    Ready,
    BlockedOnSend { target: i32, message: MinixMessage },
    BlockedOnReceive { from: i32 },
}

/// Minix 3 Synchronous Rendez-vous IPC Engine
pub struct MinixRendezvousEngine {
    process_states: BTreeMap<i32, RendezvousState>,
    notifications: BTreeMap<i32, Vec<MinixMessage>>,
}

impl MinixRendezvousEngine {
    pub fn new() -> Self {
        Self {
            process_states: BTreeMap::new(),
            notifications: BTreeMap::new(),
        }
    }

    pub fn register_process(&mut self, endpoint: i32) {
        self.process_states.insert(endpoint, RendezvousState::Ready);
        self.notifications.insert(endpoint, Vec::new());
    }

    /// Asynchronous non-blocking hardware/kernel interrupt notification
    pub fn notify(&mut self, src: i32, dst: i32, msg_type: i32) -> Result<(), &'static str> {
        let notifs = self
            .notifications
            .get_mut(&dst)
            .ok_or("Destination endpoint not found")?;
        notifs.push(MinixMessage::new(src, msg_type, &[]));
        Ok(())
    }

    /// Synchronous rendezvous: sender transfers message directly to receiver
    pub fn send_rendezvous(
        &mut self,
        src: i32,
        dst: i32,
        msg: MinixMessage,
    ) -> Result<Option<MinixMessage>, &'static str> {
        let dst_state = self
            .process_states
            .get(&dst)
            .cloned()
            .ok_or("Target endpoint not registered")?;

        match dst_state {
            RendezvousState::BlockedOnReceive { from }
                if from == src || from == IPC_ENDPOINT_ANY =>
            {
                // Rendez-vous achieved! Unblock receiver and deliver message immediately
                self.process_states.insert(dst, RendezvousState::Ready);
                self.process_states.insert(src, RendezvousState::Ready);
                Ok(Some(msg))
            }
            _ => {
                // Receiver not waiting -> Sender blocks
                self.process_states.insert(
                    src,
                    RendezvousState::BlockedOnSend {
                        target: dst,
                        message: msg,
                    },
                );
                Ok(None)
            }
        }
    }

    /// Synchronous rendezvous: receiver waits for sender
    pub fn receive_rendezvous(
        &mut self,
        dst: i32,
        from: i32,
    ) -> Result<Option<MinixMessage>, &'static str> {
        // First check pending async notifications
        if let Some(notifs) = self.notifications.get_mut(&dst) {
            if !notifs.is_empty() {
                return Ok(Some(notifs.remove(0)));
            }
        }

        // Check if any sender is already blocked waiting for this receiver
        for (&src_ep, state) in self.process_states.iter() {
            if let RendezvousState::BlockedOnSend { target, message } = state {
                if *target == dst && (from == src_ep || from == IPC_ENDPOINT_ANY) {
                    let msg = *message;
                    // Unblock sender
                    self.process_states.insert(src_ep, RendezvousState::Ready);
                    self.process_states.insert(dst, RendezvousState::Ready);
                    return Ok(Some(msg));
                }
            }
        }

        // No sender waiting -> Receiver blocks
        self.process_states
            .insert(dst, RendezvousState::BlockedOnReceive { from });
        Ok(None)
    }

    pub fn is_blocked(&self, endpoint: i32) -> bool {
        match self.process_states.get(&endpoint) {
            Some(RendezvousState::Ready) | None => false,
            _ => true,
        }
    }
}
