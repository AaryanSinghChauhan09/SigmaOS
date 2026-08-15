// SPDX-License-Identifier: MIT OR Apache-2.0
// SigmaOS SigmaBus - Zero-dependency message bus (D-Bus inspired)
// Provides inter-process communication without any external library dependencies
// Inspired by D-Bus, HeLin IPC (HeliOS), and Mach ports (macOS/GNU Mach)

#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

/// Maximum message payload size (inspired by D-Bus's 134MB, but kernel-safe at 4KB)
pub const SIGMA_BUS_MAX_PAYLOAD: usize = 4096;

/// Message types (inspired by D-Bus message types)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    MethodCall   = 1,  // Request from client to service
    MethodReturn = 2,  // Return value from method
    Error        = 3,  // Error reply
    Signal       = 4,  // Broadcast notification (no reply expected)
}

/// A SigmaBus message header (fixed-size, cache-line aligned)
#[derive(Debug, Clone)]
#[repr(C, align(64))]  // Cache-line aligned for performance
pub struct MessageHeader {
    pub msg_type: MessageType,
    pub serial: u64,        // Unique message ID (monotonically increasing)
    pub reply_serial: u64,  // Serial of the message we're replying to (0 if N/A)
    pub sender: u64,        // Sender's bus name hash
    pub destination: u64,   // Destination's bus name hash (0 = broadcast)
    pub interface_hash: u64, // Hash of interface name (e.g., "org.sigma.FileManager")
    pub member_hash: u64,   // Hash of method/signal name (e.g., "OpenFile")
    pub payload_len: u32,   // Length of payload in bytes
    pub flags: u16,         // Flags (e.g., NO_REPLY_EXPECTED, NO_AUTO_START)
    pub version: u8,        // Protocol version (currently 1)
    _pad: u8,
}

/// A complete SigmaBus message
#[derive(Debug, Clone)]
pub struct SigmaMessage {
    pub header: MessageHeader,
    pub payload: Vec<u8>,
}

impl SigmaMessage {
    pub fn new_method_call(
        sender: u64,
        destination: u64,
        interface: &str,
        member: &str,
        payload: Vec<u8>,
    ) -> Self {
        static SERIAL: AtomicU64 = AtomicU64::new(1);

        SigmaMessage {
            header: MessageHeader {
                msg_type: MessageType::MethodCall,
                serial: SERIAL.fetch_add(1, Ordering::Relaxed),
                reply_serial: 0,
                sender,
                destination,
                interface_hash: fnv1a_hash(interface),
                member_hash: fnv1a_hash(member),
                payload_len: payload.len().min(SIGMA_BUS_MAX_PAYLOAD) as u32,
                flags: 0,
                version: 1,
                _pad: 0,
            },
            payload,
        }
    }

    pub fn new_signal(sender: u64, interface: &str, member: &str, payload: Vec<u8>) -> Self {
        static SERIAL: AtomicU64 = AtomicU64::new(0x8000_0000_0000_0000); // High range for signals

        SigmaMessage {
            header: MessageHeader {
                msg_type: MessageType::Signal,
                serial: SERIAL.fetch_add(1, Ordering::Relaxed),
                reply_serial: 0,
                sender,
                destination: 0, // Broadcast
                interface_hash: fnv1a_hash(interface),
                member_hash: fnv1a_hash(member),
                payload_len: payload.len() as u32,
                flags: 0x01, // NO_REPLY_EXPECTED
                version: 1,
                _pad: 0,
            },
            payload,
        }
    }

    pub fn new_error(original: &SigmaMessage, error_name: &str) -> Self {
        static SERIAL: AtomicU64 = AtomicU64::new(0x4000_0000_0000_0000);

        SigmaMessage {
            header: MessageHeader {
                msg_type: MessageType::Error,
                serial: SERIAL.fetch_add(1, Ordering::Relaxed),
                reply_serial: original.header.serial,
                sender: original.header.destination,
                destination: original.header.sender,
                interface_hash: 0,
                member_hash: fnv1a_hash(error_name),
                payload_len: 0,
                flags: 0,
                version: 1,
                _pad: 0,
            },
            payload: Vec::new(),
        }
    }
}

/// Message subscription filter
#[derive(Debug, Clone)]
pub struct BusFilter {
    pub msg_type: Option<MessageType>,
    pub interface_hash: Option<u64>,
    pub member_hash: Option<u64>,
    pub sender: Option<u64>,
}

impl BusFilter {
    pub fn any() -> Self {
        Self { msg_type: None, interface_hash: None, member_hash: None, sender: None }
    }

    pub fn signal(interface: &str, member: &str) -> Self {
        Self {
            msg_type: Some(MessageType::Signal),
            interface_hash: Some(fnv1a_hash(interface)),
            member_hash: Some(fnv1a_hash(member)),
            sender: None,
        }
    }

    pub fn matches(&self, msg: &SigmaMessage) -> bool {
        if let Some(t) = self.msg_type {
            if msg.header.msg_type != t { return false; }
        }
        if let Some(ih) = self.interface_hash {
            if msg.header.interface_hash != ih { return false; }
        }
        if let Some(mh) = self.member_hash {
            if msg.header.member_hash != mh { return false; }
        }
        if let Some(s) = self.sender {
            if msg.header.sender != s { return false; }
        }
        true
    }
}

/// The SigmaBus router - routes messages between bus connections
/// Inspired by D-Bus daemon's message routing but without any external deps
pub struct SigmaBus {
    /// Registered services: bus name hash → service info
    services: BTreeMap<u64, BusServiceInfo>,
    /// Message queues: bus name hash → pending messages
    queues: BTreeMap<u64, Vec<SigmaMessage>>,
    /// Signal subscriptions: list of (subscriber, filter)
    subscriptions: Vec<(u64, BusFilter)>,
}

#[derive(Debug, Clone)]
pub struct BusServiceInfo {
    pub name: String,
    pub owner: u64,  // Process/task ID that owns this service name
    pub is_unique: bool, // Is this a unique name (like :1.42) vs. well-known?
}

impl SigmaBus {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            queues: BTreeMap::new(),
            subscriptions: Vec::new(),
        }
    }

    /// Register a well-known service name
    pub fn register_service(&mut self, name: &str, owner: u64) -> Result<(), BusError> {
        let hash = fnv1a_hash(name);
        if self.services.contains_key(&hash) {
            return Err(BusError::NameAlreadyRegistered);
        }
        self.services.insert(hash, BusServiceInfo {
            name: String::from(name),
            owner,
            is_unique: false,
        });
        Ok(())
    }

    /// Send a message - routes to the appropriate queue
    pub fn send(&mut self, msg: SigmaMessage) -> Result<u64, BusError> {
        let serial = msg.header.serial;

        if msg.header.msg_type == MessageType::Signal {
            // Broadcast to all matching subscriptions
            let matching: Vec<u64> = self.subscriptions.iter()
                .filter(|(_, filter)| filter.matches(&msg))
                .map(|(subscriber, _)| *subscriber)
                .collect();

            for subscriber in matching {
                self.queues
                    .entry(subscriber)
                    .or_insert_with(Vec::new)
                    .push(msg.clone());
            }
        } else {
            // Point-to-point routing
            let dest = msg.header.destination;
            if dest == 0 {
                return Err(BusError::NoDestination);
            }

            // Find the queue for the destination service
            let _service = self.services.get(&dest)
                .ok_or(BusError::ServiceNotFound)?;

            self.queues
                .entry(dest)
                .or_insert_with(Vec::new)
                .push(msg);
        }

        Ok(serial)
    }

    /// Receive the next message for a service
    pub fn recv(&mut self, service_hash: u64) -> Option<SigmaMessage> {
        let queue = self.queues.get_mut(&service_hash)?;
        if queue.is_empty() {
            return None;
        }
        Some(queue.remove(0)) // FIFO order (like D-Bus)
    }

    /// Subscribe to signals matching a filter
    pub fn subscribe(&mut self, subscriber: u64, filter: BusFilter) {
        self.subscriptions.push((subscriber, filter));
    }
}

impl Default for SigmaBus {
    fn default() -> Self { Self::new() }
}

/// Error types for SigmaBus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusError {
    NameAlreadyRegistered,
    ServiceNotFound,
    NoDestination,
    PayloadTooLarge,
    QueueFull,
}

/// FNV-1a hash function (zero dependency, used for string interning)
/// Inspired by the FNV hash used in LLVM, Go, and many OS kernels
pub const fn fnv1a_hash(s: &str) -> u64 {
    const FNV_PRIME: u64 = 0x00000100000001B3;
    const FNV_OFFSET: u64 = 0xCBF29CE484222325;

    let bytes = s.as_bytes();
    let mut hash = FNV_OFFSET;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv_hash_consistency() {
        assert_eq!(fnv1a_hash("hello"), fnv1a_hash("hello"));
        assert_ne!(fnv1a_hash("hello"), fnv1a_hash("world"));
    }

    #[test]
    fn test_bus_register_and_send() {
        let mut bus = SigmaBus::new();
        let service_hash = fnv1a_hash("org.sigma.FileManager");

        bus.register_service("org.sigma.FileManager", 1001).unwrap();

        let msg = SigmaMessage::new_method_call(
            1000,
            service_hash,
            "org.sigma.FileManager",
            "OpenFile",
            b"/home/user/doc.txt".to_vec(),
        );

        bus.send(msg).unwrap();

        let received = bus.recv(service_hash);
        assert!(received.is_some());
        let received = received.unwrap();
        assert_eq!(received.header.msg_type, MessageType::MethodCall);
        assert_eq!(&received.payload, b"/home/user/doc.txt");
    }

    #[test]
    fn test_signal_broadcast() {
        let mut bus = SigmaBus::new();
        let subscriber_a = fnv1a_hash("subscriber.A");
        let subscriber_b = fnv1a_hash("subscriber.B");

        bus.subscribe(subscriber_a, BusFilter::signal("org.sigma.SystemEvents", "Shutdown"));
        bus.subscribe(subscriber_b, BusFilter::signal("org.sigma.SystemEvents", "Shutdown"));

        let signal = SigmaMessage::new_signal(
            fnv1a_hash("org.sigma.Init"),
            "org.sigma.SystemEvents",
            "Shutdown",
            Vec::new(),
        );

        bus.send(signal).unwrap();

        assert!(bus.recv(subscriber_a).is_some());
        assert!(bus.recv(subscriber_b).is_some());
    }
}
