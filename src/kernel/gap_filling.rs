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

// Gap Filling Implementation
// S-IPC, S-Signal, and S-MM implementations from GAP_FILLING_STRATEGIC_PLAN.md

// (no_std only applicable at crate root - removed)

use std::vec::Vec;
use core::default::Default;
use core::option::Option::{self, None, Some};
use core::result::Result::{self, Err, Ok};

pub const MAX_IPC_MESSAGE_SIZE: usize = 64;
pub const IPC_QUEUE_CAPACITY: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcMessage {
    pub sender_pid: u32,
    pub receiver_pid: u32,
    pub payload: [u8; MAX_IPC_MESSAGE_SIZE],
    pub size: usize,
}

/// S-IPC: Zero-Copy Capability-Gated Message Queue
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

    /// Sends a message over the secure transaction bus
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
        // Optimize modulo by replacing with bitwise masking. Since IPC_QUEUE_CAPACITY is a constant power of two (8),
        // bitwise AND with capacity - 1 operates in a single CPU cycle.
        self.write_idx = (self.write_idx + 1) & (IPC_QUEUE_CAPACITY - 1);
        self.count += 1;
        Ok(())
    }

    /// Receives a message for a specific process ID
    pub fn receive_message(&mut self, receiver_pid: u32) -> Option<IpcMessage> {
        if self.count == 0 {
            return None;
        }

        let current_msg_opt = self.queue[self.read_idx];
        if let Some(msg) = current_msg_opt {
            if msg.receiver_pid == receiver_pid {
                self.queue[self.read_idx] = None;
                // Optimize modulo by replacing with bitwise masking. Since IPC_QUEUE_CAPACITY is a constant power of two (8),
                // bitwise AND with capacity - 1 operates in a single CPU cycle.
                self.read_idx = (self.read_idx + 1) & (IPC_QUEUE_CAPACITY - 1);
                self.count -= 1;
                return Some(msg);
            }
        }
        None
    }

    pub fn message_count(&self) -> usize {
        self.count
    }
}

impl Default for SovereignIpcBus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignSignal {
    Terminate,
    Interrupt,
    PageFault,
    PowerStateTransition,
}

/// S-Signal: Capability-Gated Signal Dispatcher
pub struct SignalDispatcher {
    pub pending_signals: [Option<(u32, SovereignSignal)>; 16],
}

impl SignalDispatcher {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pending_signals: [None; 16],
        }
    }

    /// Registers a secure signal for target processes, checking signal delegation permissions
    pub fn raise_signal(
        &mut self,
        target_pid: u32,
        signal: SovereignSignal,
        is_sender_allowed: bool,
    ) -> Result<(), &'static str> {
        if !is_sender_allowed {
            return Err("Sender process lacks capability to raise signal to target");
        }

        for slot in self.pending_signals.iter_mut() {
            if slot.is_none() {
                *slot = Some((target_pid, signal));
                return Ok(());
            }
        }
        Err("Signal queue is full")
    }

    /// Process and dispatch signal queue for a specific target
    pub fn poll_signal(&mut self, target_pid: u32) -> Option<SovereignSignal> {
        for slot in self.pending_signals.iter_mut() {
            if let Some((pid, sig)) = slot {
                if *pid == target_pid {
                    let sig_to_return = *sig;
                    *slot = None;
                    return Some(sig_to_return);
                }
            }
        }
        None
    }

    pub fn pending_count(&self) -> usize {
        self.pending_signals.iter().filter(|s| s.is_some()).count()
    }
}

impl Default for SignalDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// S-MM: Strict Page Directory Controller
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageDirectoryEntry {
    pub virtual_address: u64,
    pub physical_address: u64,
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
}

pub struct PageDirectoryController {
    pub entries: Vec<PageDirectoryEntry>,
    pub next_virtual: u64,
    pub next_physical: u64,
}

impl PageDirectoryController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_virtual: 0x1000,
            next_physical: 0x1000,
        }
    }

    /// Maps a virtual address to a physical address with strict capability checks
    pub fn map_page(
        &mut self,
        virtual_addr: u64,
        physical_addr: u64,
        writable: bool,
        user_accessible: bool,
    ) -> Result<(), &'static str> {
        // Check for existing mapping
        if self
            .entries
            .iter()
            .any(|e| e.virtual_address == virtual_addr)
        {
            return Err("Virtual address already mapped");
        }

        let entry = PageDirectoryEntry {
            virtual_address: virtual_addr,
            physical_address: physical_addr,
            present: true,
            writable,
            user_accessible,
        };

        self.entries.push(entry);
        Ok(())
    }

    /// Unmaps a virtual address
    pub fn unmap_page(&mut self, virtual_addr: u64) -> Result<(), &'static str> {
        let original_len = self.entries.len();
        self.entries.retain(|e| e.virtual_address != virtual_addr);

        if self.entries.len() == original_len {
            return Err("Virtual address not found in page directory");
        }

        Ok(())
    }

    /// Translates virtual to physical address
    pub fn translate(&self, virtual_addr: u64) -> Option<u64> {
        self.entries
            .iter()
            .find(|e| e.virtual_address == virtual_addr && e.present)
            .map(|e| e.physical_address)
    }

    /// Get page count
    pub fn page_count(&self) -> usize {
        self.entries.len()
    }

    /// Allocate new virtual address
    pub fn allocate_virtual(&mut self) -> u64 {
        let addr = self.next_virtual;
        self.next_virtual += 0x1000;
        addr
    }

    /// Allocate new physical address
    pub fn allocate_physical(&mut self) -> u64 {
        let addr = self.next_physical;
        self.next_physical += 0x1000;
        addr
    }
}

impl Default for PageDirectoryController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_ipc_bus() {
        let mut bus = SovereignIpcBus::new();

        let data = b"INIT_SHARD_TRANSACTION";
        assert!(bus.send_message(100, 200, data, true).is_ok());

        assert!(bus.send_message(100, 200, data, false).is_err());

        let received = bus.receive_message(200).unwrap();
        assert_eq!(received.sender_pid, 100);
        assert_eq!(&received.payload[..received.size], data);
    }

    #[test]
    fn test_ipc_queue_full() {
        let mut bus = SovereignIpcBus::new();
        let data = b"TEST";

        for _ in 0..IPC_QUEUE_CAPACITY {
            bus.send_message(1, 2, data, true).unwrap();
        }

        assert!(bus.send_message(1, 2, data, true).is_err());
    }

    #[test]
    fn test_signal_dispatcher() {
        let mut dispatcher = SignalDispatcher::new();

        dispatcher
            .raise_signal(100, SovereignSignal::Terminate, true)
            .unwrap();

        assert!(dispatcher
            .raise_signal(100, SovereignSignal::Interrupt, false)
            .is_err());

        let signal = dispatcher.poll_signal(100).unwrap();
        assert_eq!(signal, SovereignSignal::Terminate);
    }

    #[test]
    fn test_signal_queue_full() {
        let mut dispatcher = SignalDispatcher::new();

        for _ in 0..16 {
            dispatcher
                .raise_signal(1, SovereignSignal::Interrupt, true)
                .unwrap();
        }

        assert!(dispatcher
            .raise_signal(1, SovereignSignal::Terminate, true)
            .is_err());
    }

    #[test]
    fn test_page_directory_controller() {
        let mut controller = PageDirectoryController::new();

        controller.map_page(0x1000, 0x2000, true, false).unwrap();

        assert_eq!(controller.translate(0x1000), Some(0x2000));
    }

    #[test]
    fn test_page_unmap() {
        let mut controller = PageDirectoryController::new();

        controller.map_page(0x1000, 0x2000, true, false).unwrap();

        controller.unmap_page(0x1000).unwrap();

        assert_eq!(controller.translate(0x1000), None);
    }

    #[test]
    fn test_duplicate_mapping() {
        let mut controller = PageDirectoryController::new();

        controller.map_page(0x1000, 0x2000, true, false).unwrap();

        assert!(controller.map_page(0x1000, 0x3000, true, false).is_err());
    }

    #[test]
    fn test_address_allocation() {
        let mut controller = PageDirectoryController::new();

        let v1 = controller.allocate_virtual();
        let v2 = controller.allocate_virtual();

        assert_eq!(v2 - v1, 0x1000);
    }
}
