#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// S-IPC Transaction Bus (Microkernel Inter-Process Communication)
// Sovereign AI-Native zero-dependency implementation

use std::vec::Vec;

pub const MAX_IPC_MESSAGE_SIZE: usize = 64;
pub const IPC_QUEUE_CAPACITY: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcMessage {
    pub sender_pid: u32,
    pub receiver_pid: u32,
    pub payload: [u8; MAX_IPC_MESSAGE_SIZE],
    pub size: usize,
}

pub struct SovereignIpcBus {
    pub queue: [Option<IpcMessage>; IPC_QUEUE_CAPACITY],
    pub read_idx: usize,
    pub write_idx: usize,
    pub count: usize,
}

impl SovereignIpcBus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            queue: [None; IPC_QUEUE_CAPACITY],
            read_idx: 0,
            write_idx: 0,
            count: 0,
        }
    }

    pub fn send_message(
        &mut self,
        sender_pid: u32,
        receiver_pid: u32,
        data: &[u8],
        has_ipc_capability: bool,
    ) -> Result<(), &'static str> {
        if !has_ipc_capability {
            return Err("Sender lacks S-SEC capability token to write to IPC bus");
        }
        if self.count >= IPC_QUEUE_CAPACITY {
            return Err("Sovereign IPC bus queue is full");
        }
        if data.len() > MAX_IPC_MESSAGE_SIZE {
            return Err("Message payload exceeds maximum transaction limit");
        }

        let mut payload = [0u8; MAX_IPC_MESSAGE_SIZE];
        payload[..data.len()].copy_from_slice(data);

        let msg = IpcMessage {
            sender_pid,
            receiver_pid,
            payload,
            size: data.len(),
        };

        self.queue[self.write_idx] = Some(msg);
        self.write_idx = (self.write_idx + 1) % IPC_QUEUE_CAPACITY;
        self.count += 1;
        Ok(())
    }

    pub fn receive_message(&mut self, receiver_pid: u32) -> Option<IpcMessage> {
        if self.count == 0 {
            return None;
        }
        let current_msg_opt = self.queue[self.read_idx];
        if let Some(msg) = current_msg_opt {
            if msg.receiver_pid == receiver_pid {
                self.queue[self.read_idx] = None;
                self.read_idx = (self.read_idx + 1) % IPC_QUEUE_CAPACITY;
                self.count -= 1;
                return Some(msg);
            }
        }
        None
    }

    pub fn message_count(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count >= IPC_QUEUE_CAPACITY
    }

    pub fn clear(&mut self) {
        self.queue = [None; IPC_QUEUE_CAPACITY];
        self.read_idx = 0;
        self.write_idx = 0;
        self.count = 0;
    }
}

impl Default for SovereignIpcBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_bus_creation() {
        let bus = SovereignIpcBus::new();
        assert_eq!(bus.message_count(), 0);
        assert!(bus.is_empty());
        assert!(!bus.is_full());
    }

    #[test]
    fn test_send_receive_message() {
        let mut bus = SovereignIpcBus::new();
        let data = b"Hello, World!";
        
        assert!(bus.send_message(1, 2, data, true).is_ok());
        assert_eq!(bus.message_count(), 1);
        
        let received = bus.receive_message(2);
        assert!(received.is_some());
        let msg = received.unwrap();
        assert_eq!(msg.sender_pid, 1);
        assert_eq!(msg.receiver_pid, 2);
        assert_eq!(&msg.payload[..data.len()], data);
    }

    #[test]
    fn test_ipc_capability_check() {
        let mut bus = SovereignIpcBus::new();
        let data = b"Test message";
        
        // Should fail without capability
        assert!(bus.send_message(1, 2, data, false).is_err());
        
        // Should succeed with capability
        assert!(bus.send_message(1, 2, data, true).is_ok());
    }

    #[test]
    fn test_message_size_limit() {
        let mut bus = SovereignIpcBus::new();
        let large_data = [0u8; 100]; // Exceeds MAX_IPC_MESSAGE_SIZE
        
        assert!(bus.send_message(1, 2, &large_data, true).is_err());
    }

    #[test]
    fn test_queue_full() {
        let mut bus = SovereignIpcBus::new();
        let data = b"Test";
        
        // Fill the queue
        for i in 0..IPC_QUEUE_CAPACITY {
            assert!(bus.send_message(1, (i + 1) as u32, data, true).is_ok());
        }
        
        assert!(bus.is_full());
        
        // Should fail when queue is full
        assert!(bus.send_message(1, 100, data, true).is_err());
    }

    #[test]
    fn test_clear_bus() {
        let mut bus = SovereignIpcBus::new();
        let data = b"Test";
        
        bus.send_message(1, 2, data, true).unwrap();
        assert_eq!(bus.message_count(), 1);
        
        bus.clear();
        assert!(bus.is_empty());
        assert_eq!(bus.message_count(), 0);
    }

    #[test]
    fn test_receive_wrong_pid() {
        let mut bus = SovereignIpcBus::new();
        let data = b"Test";
        
        bus.send_message(1, 2, data, true).unwrap();
        
        // Process 3 should not receive message meant for process 2
        assert!(bus.receive_message(3).is_none());
        
        // Process 2 should receive the message
        assert!(bus.receive_message(2).is_some());
    }
}
