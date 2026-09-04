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

// OOP-based IPC Message System for SigmaOS
// Implements Direct Addressing (1-to-1 explicit Process IDs),
// Indirect Addressing (Mailboxes/Ports with 1-to-1, 1-to-N, N-to-N relationships),
// Structured Message Headers with payload validation, sequence numbers, and delivery modes.

use std::boxed::Box;
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ChannelID = usize;
pub type Pid = u32;
pub type MailboxId = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPCError {
    Success = 0,
    ChannelFull = 1,
    ChannelEmpty = 2,
    InvalidChannel = 3,
    InvalidRecipient = 4,
    PayloadTooLarge = 5,
    RelationshipViolation = 6,
    Timeout = 7,
}

/// Communication addressing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Direct process-to-process addressing (explicit sender Pid & recipient Pid)
    Direct { sender: Pid, recipient: Pid },
    /// Indirect addressing via shared Mailbox/Port ID
    Indirect { mailbox_id: MailboxId },
}

/// Process relationship cardinality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessRelationship {
    /// 1-to-1: Exactly one sender process communicating with one receiver process
    OneToOne,
    /// 1-to-N: One sender process broadcasting to multiple receiver processes
    OneToMany,
    /// N-to-N: Multiple sender processes communicating with multiple receiver processes
    ManyToMany,
}

/// Delivery mode requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryMode {
    SynchronousBlocking,
    AsynchronousNonBlocking,
}

/// Message header containing requirements, sequence ID, and payload validation metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageHeader {
    pub sequence_number: u64,
    pub sender_pid: Pid,
    pub recipient_pid: Option<Pid>,
    pub mailbox_id: Option<MailboxId>,
    pub payload_len: usize,
    pub delivery_mode: DeliveryMode,
    pub timeout_ticks: u64,
}

impl MessageHeader {
    pub fn new(
        sequence_number: u64,
        sender_pid: Pid,
        payload_len: usize,
        delivery_mode: DeliveryMode,
    ) -> Self {
        Self {
            sequence_number,
            sender_pid,
            recipient_pid: None,
            mailbox_id: None,
            payload_len,
            delivery_mode,
            timeout_ticks: 1000,
        }
    }
}

pub trait MessageChannel {
    fn id(&self) -> ChannelID;
    fn capacity(&self) -> usize;
    fn send(&mut self, header: MessageHeader, message: &[u8]) -> Result<(), IPCError>;
    fn receive(&mut self) -> Result<(MessageHeader, Vec<u8>), IPCError>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

pub struct SimpleMessageChannel {
    pub id: ChannelID,
    pub capacity: AtomicUsize,
    pub addressing_mode: AddressingMode,
    pub relationship: ProcessRelationship,
    pub allowed_senders: Vec<Pid>,
    pub allowed_receivers: Vec<Pid>,
    pub headers: Vec<MessageHeader>,
    pub messages: Vec<[u8; 256]>,
}

impl SimpleMessageChannel {
    pub fn new_direct(id: ChannelID, capacity: usize, sender: Pid, recipient: Pid) -> Self {
        let mut senders = Vec::new();
        senders.push(sender);
        let mut receivers = Vec::new();
        receivers.push(recipient);

        SimpleMessageChannel {
            id,
            capacity: AtomicUsize::new(capacity),
            addressing_mode: AddressingMode::Direct { sender, recipient },
            relationship: ProcessRelationship::OneToOne,
            allowed_senders: senders,
            allowed_receivers: receivers,
            headers: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn new_indirect(
        id: ChannelID,
        capacity: usize,
        mailbox_id: MailboxId,
        relationship: ProcessRelationship,
    ) -> Self {
        SimpleMessageChannel {
            id,
            capacity: AtomicUsize::new(capacity),
            addressing_mode: AddressingMode::Indirect { mailbox_id },
            relationship,
            allowed_senders: Vec::new(),
            allowed_receivers: Vec::new(),
            headers: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn register_sender(&mut self, pid: Pid) -> Result<(), IPCError> {
        match self.relationship {
            ProcessRelationship::OneToOne | ProcessRelationship::OneToMany => {
                if !self.allowed_senders.is_empty() && !self.allowed_senders.contains(&pid) {
                    return Err(IPCError::RelationshipViolation);
                }
            }
            ProcessRelationship::ManyToMany => {}
        }
        if !self.allowed_senders.contains(&pid) {
            self.allowed_senders.push(pid);
        }
        Ok(())
    }

    pub fn register_receiver(&mut self, pid: Pid) -> Result<(), IPCError> {
        match self.relationship {
            ProcessRelationship::OneToOne => {
                if !self.allowed_receivers.is_empty() && !self.allowed_receivers.contains(&pid) {
                    return Err(IPCError::RelationshipViolation);
                }
            }
            ProcessRelationship::OneToMany | ProcessRelationship::ManyToMany => {}
        }
        if !self.allowed_receivers.contains(&pid) {
            self.allowed_receivers.push(pid);
        }
        Ok(())
    }
}

impl MessageChannel for SimpleMessageChannel {
    fn id(&self) -> ChannelID {
        self.id
    }
    fn capacity(&self) -> usize {
        self.capacity.load(Ordering::SeqCst)
    }

    fn send(&mut self, mut header: MessageHeader, message: &[u8]) -> Result<(), IPCError> {
        if self.messages.len() >= self.capacity() {
            return Err(IPCError::ChannelFull);
        }

        if message.len() > 255 {
            return Err(IPCError::PayloadTooLarge);
        }

        // Validate sender relationship
        if !self.allowed_senders.is_empty() && !self.allowed_senders.contains(&header.sender_pid) {
            return Err(IPCError::RelationshipViolation);
        }

        // Direct addressing validation
        if let AddressingMode::Direct { sender, recipient } = self.addressing_mode {
            if header.sender_pid != sender {
                return Err(IPCError::InvalidRecipient);
            }
            header.recipient_pid = Some(recipient);
        } else if let AddressingMode::Indirect { mailbox_id } = self.addressing_mode {
            header.mailbox_id = Some(mailbox_id);
        }

        header.payload_len = message.len();

        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        for i in 0..msg_len {
            msg_array[i] = message[i];
        }

        self.headers.push(header);
        self.messages.push(msg_array);
        Ok(())
    }

    fn receive(&mut self) -> Result<(MessageHeader, Vec<u8>), IPCError> {
        if self.messages.is_empty() {
            return Err(IPCError::ChannelEmpty);
        }

        let header = self.headers.remove(0);
        let msg_array = self.messages.remove(0);

        let mut result = Vec::new();
        for i in 0..header.payload_len {
            result.push(msg_array[i]);
        }

        Ok((header, result))
    }

    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    fn is_full(&self) -> bool {
        self.messages.len() >= self.capacity()
    }
}

pub trait IPCManager {
    fn create_channel(&mut self, capacity: usize) -> Result<ChannelID, IPCError>;
    fn destroy_channel(&mut self, id: ChannelID) -> Result<(), IPCError>;
    fn get_channel(&mut self, id: ChannelID) -> Option<&mut dyn MessageChannel>;
}

pub struct SimpleIPCManager {
    pub channels: Vec<Option<Box<dyn MessageChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIPCManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleIPCManager {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IPCManager for SimpleIPCManager {
    fn create_channel(&mut self, capacity: usize) -> Result<ChannelID, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let channel = SimpleMessageChannel::new_indirect(
            id,
            capacity,
            id as MailboxId,
            ProcessRelationship::ManyToMany,
        );
        self.channels.push(Some(Box::new(channel)));
        Ok(id)
    }

    fn destroy_channel(&mut self, id: ChannelID) -> Result<(), IPCError> {
        for channel_option in &mut self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == id {
                    return Ok(());
                }
            }
        }
        Err(IPCError::InvalidChannel)
    }

    fn get_channel(&mut self, id: ChannelID) -> Option<&mut dyn MessageChannel> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == id {
                    return Some(channel.as_mut());
                }
            }
        }
        None
    }
}

pub trait SharedMemory {
    fn allocate(&mut self, size: usize) -> Result<usize, IPCError>;
    fn deallocate(&mut self, id: usize) -> Result<(), IPCError>;
    fn write(&mut self, id: usize, offset: usize, data: &[u8]) -> Result<(), IPCError>;
    fn read(&self, id: usize, offset: usize, buffer: &mut [u8]) -> Result<(), IPCError>;
}

pub struct SimpleSharedMemory {
    pub regions: Vec<(usize, Vec<u8>)>,
    pub next_id: AtomicUsize,
}

impl SimpleSharedMemory {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSharedMemory {
            regions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SharedMemory for SimpleSharedMemory {
    fn allocate(&mut self, size: usize) -> Result<usize, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(0u8);
        }
        self.regions.push((id, data));
        Ok(id)
    }

    fn deallocate(&mut self, id: usize) -> Result<(), IPCError> {
        for i in 0..self.regions.len() {
            if self.regions[i].0 == id {
                self.regions.remove(i);
                return Ok(());
            }
        }
        Err(IPCError::InvalidChannel)
    }

    fn write(&mut self, id: usize, offset: usize, data: &[u8]) -> Result<(), IPCError> {
        for region in &mut self.regions {
            if region.0 == id {
                let region_data = &mut region.1;
                let end = (offset + data.len()).min(region_data.len());
                for i in 0..data.len() {
                    if offset + i < end {
                        region_data[offset + i] = data[i];
                    }
                }
                return Ok(());
            }
        }
        Err(IPCError::InvalidChannel)
    }

    fn read(&self, id: usize, offset: usize, buffer: &mut [u8]) -> Result<(), IPCError> {
        for region in &self.regions {
            if region.0 == id {
                let region_data = &region.1;
                let end = (offset + buffer.len()).min(region_data.len());
                for i in 0..buffer.len() {
                    if offset + i < end {
                        buffer[i] = region_data[offset + i];
                    }
                }
                return Ok(());
            }
        }
        Err(IPCError::InvalidChannel)
    }
}

pub trait Semaphore {
    fn acquire(&mut self) -> Result<(), IPCError>;
    fn release(&mut self) -> Result<(), IPCError>;
    fn count(&self) -> usize;
}

pub struct SimpleSemaphore {
    pub count: AtomicUsize,
    pub max_count: AtomicUsize,
}

impl SimpleSemaphore {
    pub fn new(initial_count: usize, max_count: usize) -> Self {
        SimpleSemaphore {
            count: AtomicUsize::new(initial_count),
            max_count: AtomicUsize::new(max_count),
        }
    }
}

impl Semaphore for SimpleSemaphore {
    fn acquire(&mut self) -> Result<(), IPCError> {
        let current = self.count.load(Ordering::SeqCst);
        if current > 0 {
            self.count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(IPCError::ChannelEmpty)
        }
    }

    fn release(&mut self) -> Result<(), IPCError> {
        let max = self.max_count.load(Ordering::SeqCst);
        let current = self.count.load(Ordering::SeqCst);
        if current < max {
            self.count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(IPCError::ChannelFull)
        }
    }

    fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_addressing_one_to_one() {
        let mut ch = SimpleMessageChannel::new_direct(1, 10, 100, 200);
        let header = MessageHeader::new(1, 100, 11, DeliveryMode::AsynchronousNonBlocking);

        // Sender 100 sends message to Recipient 200
        assert!(ch.send(header, b"hello direct").is_ok());

        // Attempt from unauthorized sender 101 fails with RelationshipViolation
        let bad_header = MessageHeader::new(2, 101, 11, DeliveryMode::AsynchronousNonBlocking);
        assert_eq!(ch.send(bad_header, b"fail"), Err(IPCError::RelationshipViolation));

        let (recv_header, payload) = ch.receive().unwrap();
        assert_eq!(recv_header.recipient_pid, Some(200));
        assert_eq!(&payload[..], b"hello direct");
    }

    #[test]
    fn test_indirect_addressing_one_to_many() {
        let mut ch = SimpleMessageChannel::new_indirect(2, 10, 5000, ProcessRelationship::OneToMany);
        ch.register_sender(100).unwrap();

        // Second sender registration fails due to 1-to-N constraint
        assert_eq!(ch.register_sender(101), Err(IPCError::RelationshipViolation));

        // Multiple receivers can register
        assert!(ch.register_receiver(201).is_ok());
        assert!(ch.register_receiver(202).is_ok());

        let header = MessageHeader::new(1, 100, 12, DeliveryMode::AsynchronousNonBlocking);
        assert!(ch.send(header, b"broadcast msg").is_ok());

        let (recv_header, payload) = ch.receive().unwrap();
        assert_eq!(recv_header.mailbox_id, Some(5000));
        assert_eq!(&payload[..], b"broadcast msg");
    }

    #[test]
    fn test_message_header_payload_limits() {
        let mut ch = SimpleMessageChannel::new_direct(3, 10, 10, 20);
        let header = MessageHeader::new(100, 10, 300, DeliveryMode::SynchronousBlocking);
        let oversized_payload = [0u8; 300];

        // Should return PayloadTooLarge error
        assert_eq!(ch.send(header, &oversized_payload), Err(IPCError::PayloadTooLarge));
    }
}
