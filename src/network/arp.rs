//! # ARP Protocol
//!
//! Address Resolution Protocol (RFC 826) for mapping IP addresses to MAC addresses.
//! Inspired by Linux net/ipv4/arp.c and FreeBSD netinet/if_ether.c.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// ARP hardware types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpHardwareType {
    Ethernet = 1,
    IEEE802 = 6,
    Arcnet = 7,
    FrameRelay = 15,
    ATM = 16,
    HDLC = 17,
    FibreChannel = 18,
    InfiniBand = 32,
}

/// ARP operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpOperation {
    Request = 1,
    Reply = 2,
    ReverseRequest = 3,
    ReverseReply = 4,
    InverseRequest = 8,
    InverseReply = 9,
}

/// ARP packet structure (for Ethernet/IPv4)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ArpPacket {
    pub hardware_type: u16,         // Hardware type (big-endian)
    pub protocol_type: u16,         // Protocol type (0x0800 for IPv4)
    pub hardware_addr_len: u8,      // Hardware address length (6 for Ethernet)
    pub protocol_addr_len: u8,      // Protocol address length (4 for IPv4)
    pub operation: u16,             // Operation (big-endian)
    pub sender_hw_addr: [u8; 6],    // Sender hardware address
    pub sender_proto_addr: [u8; 4], // Sender protocol address
    pub target_hw_addr: [u8; 6],    // Target hardware address
    pub target_proto_addr: [u8; 4], // Target protocol address
}

impl ArpPacket {
    /// Create ARP request
    pub fn request(sender_mac: [u8; 6], sender_ip: [u8; 4], target_ip: [u8; 4]) -> Self {
        Self {
            hardware_type: (ArpHardwareType::Ethernet as u16).to_be(),
            protocol_type: 0x0800u16.to_be(), // IPv4
            hardware_addr_len: 6,
            protocol_addr_len: 4,
            operation: (ArpOperation::Request as u16).to_be(),
            sender_hw_addr: sender_mac,
            sender_proto_addr: sender_ip,
            target_hw_addr: [0; 6],
            target_proto_addr: target_ip,
        }
    }

    /// Create ARP reply
    pub fn reply(
        sender_mac: [u8; 6],
        sender_ip: [u8; 4],
        target_mac: [u8; 6],
        target_ip: [u8; 4],
    ) -> Self {
        Self {
            hardware_type: (ArpHardwareType::Ethernet as u16).to_be(),
            protocol_type: 0x0800u16.to_be(),
            hardware_addr_len: 6,
            protocol_addr_len: 4,
            operation: (ArpOperation::Reply as u16).to_be(),
            sender_hw_addr: sender_mac,
            sender_proto_addr: sender_ip,
            target_hw_addr: target_mac,
            target_proto_addr: target_ip,
        }
    }

    /// Parse ARP packet from bytes
    pub fn parse(data: &[u8]) -> Result<Self, ArpError> {
        if data.len() < core::mem::size_of::<Self>() {
            return Err(ArpError::PacketTooSmall);
        }

        Ok(unsafe { core::ptr::read_unaligned(data.as_ptr() as *const Self) })
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        };
        bytes.to_vec()
    }

    /// Get operation
    pub fn get_operation(&self) -> ArpOperation {
        let op = u16::from_be(self.operation);
        unsafe { core::mem::transmute(op) }
    }
}

/// ARP cache entry
#[derive(Debug, Clone, Copy)]
pub struct ArpCacheEntry {
    pub mac_addr: [u8; 6],
    pub ip_addr: [u8; 4],
    pub timestamp: u64,
    pub is_static: bool,
    pub incomplete: bool,
}

impl ArpCacheEntry {
    pub fn new(mac: [u8; 6], ip: [u8; 4], timestamp: u64) -> Self {
        Self {
            mac_addr: mac,
            ip_addr: ip,
            timestamp,
            is_static: false,
            incomplete: false,
        }
    }

    pub fn incomplete(ip: [u8; 4], timestamp: u64) -> Self {
        Self {
            mac_addr: [0; 6],
            ip_addr: ip,
            timestamp,
            is_static: false,
            incomplete: true,
        }
    }
}

/// ARP cache
pub struct ArpCache {
    entries: BTreeMap<u32, ArpCacheEntry>,
    max_entries: usize,
    timeout_seconds: u64,
    current_time: AtomicU64,
}

impl ArpCache {
    pub fn new(max_entries: usize, timeout_seconds: u64) -> Self {
        Self {
            entries: BTreeMap::new(),
            max_entries,
            timeout_seconds,
            current_time: AtomicU64::new(0),
        }
    }

    /// Convert IP address to u32 for indexing
    fn ip_to_u32(ip: [u8; 4]) -> u32 {
        u32::from_be_bytes(ip)
    }

    /// Add or update entry
    pub fn insert(&mut self, ip: [u8; 4], mac: [u8; 6]) {
        let key = Self::ip_to_u32(ip);
        let timestamp = self.current_time.load(Ordering::Relaxed);

        // Evict old entries if cache is full
        if self.entries.len() >= self.max_entries {
            self.evict_oldest();
        }

        let entry = ArpCacheEntry::new(mac, ip, timestamp);
        self.entries.insert(key, entry);
    }

    /// Lookup MAC address for IP
    pub fn lookup(&self, ip: [u8; 4]) -> Option<[u8; 6]> {
        let key = Self::ip_to_u32(ip);
        let entry = self.entries.get(&key)?;

        if entry.incomplete {
            return None;
        }

        // Check if entry is expired
        let current = self.current_time.load(Ordering::Relaxed);
        if !entry.is_static && current - entry.timestamp > self.timeout_seconds {
            return None;
        }

        Some(entry.mac_addr)
    }

    /// Mark entry as incomplete (waiting for response)
    pub fn mark_incomplete(&mut self, ip: [u8; 4]) {
        let key = Self::ip_to_u32(ip);
        let timestamp = self.current_time.load(Ordering::Relaxed);
        let entry = ArpCacheEntry::incomplete(ip, timestamp);
        self.entries.insert(key, entry);
    }

    /// Add static entry (never expires)
    pub fn add_static(&mut self, ip: [u8; 4], mac: [u8; 6]) {
        let key = Self::ip_to_u32(ip);
        let timestamp = self.current_time.load(Ordering::Relaxed);
        let mut entry = ArpCacheEntry::new(mac, ip, timestamp);
        entry.is_static = true;
        self.entries.insert(key, entry);
    }

    /// Remove entry
    pub fn remove(&mut self, ip: [u8; 4]) {
        let key = Self::ip_to_u32(ip);
        self.entries.remove(&key);
    }

    /// Clear all dynamic entries
    pub fn clear_dynamic(&mut self) {
        self.entries.retain(|_, entry| entry.is_static);
    }

    /// Evict oldest entry
    fn evict_oldest(&mut self) {
        if let Some((&oldest_key, _)) = self
            .entries
            .iter()
            .filter(|(_, e)| !e.is_static)
            .min_by_key(|(_, e)| e.timestamp)
        {
            self.entries.remove(&oldest_key);
        }
    }

    /// Update current time
    pub fn update_time(&self, time: u64) {
        self.current_time.store(time, Ordering::Relaxed);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> ArpStats {
        let total = self.entries.len();
        let static_count = self.entries.values().filter(|e| e.is_static).count();
        let incomplete = self.entries.values().filter(|e| e.incomplete).count();

        ArpStats {
            total_entries: total,
            static_entries: static_count,
            dynamic_entries: total - static_count,
            incomplete_entries: incomplete,
        }
    }
}

/// ARP statistics
#[derive(Debug, Clone, Copy)]
pub struct ArpStats {
    pub total_entries: usize,
    pub static_entries: usize,
    pub dynamic_entries: usize,
    pub incomplete_entries: usize,
}

/// ARP handler
pub struct ArpHandler {
    cache: ArpCache,
    local_ip: [u8; 4],
    local_mac: [u8; 6],
    pending_requests: BTreeMap<u32, u64>,
}

impl ArpHandler {
    pub fn new(local_ip: [u8; 4], local_mac: [u8; 6]) -> Self {
        Self {
            cache: ArpCache::new(256, 300), // 256 entries, 5 min timeout
            local_ip,
            local_mac,
            pending_requests: BTreeMap::new(),
        }
    }

    /// Handle incoming ARP packet
    pub fn handle_packet(&mut self, packet: &ArpPacket) -> Option<ArpPacket> {
        match packet.get_operation() {
            ArpOperation::Request => {
                // Update cache with sender's info
                self.cache
                    .insert(packet.sender_proto_addr, packet.sender_hw_addr);

                // If target is us, send reply
                if packet.target_proto_addr == self.local_ip {
                    Some(ArpPacket::reply(
                        self.local_mac,
                        self.local_ip,
                        packet.sender_hw_addr,
                        packet.sender_proto_addr,
                    ))
                } else {
                    None
                }
            }
            ArpOperation::Reply => {
                // Update cache with reply
                self.cache
                    .insert(packet.sender_proto_addr, packet.sender_hw_addr);

                // Remove from pending
                let key = ArpCache::ip_to_u32(packet.sender_proto_addr);
                self.pending_requests.remove(&key);

                None
            }
            _ => None,
        }
    }

    /// Resolve IP to MAC (may return None if needs ARP request)
    pub fn resolve(&mut self, target_ip: [u8; 4]) -> Option<[u8; 6]> {
        // Check cache first
        if let Some(mac) = self.cache.lookup(target_ip) {
            return Some(mac);
        }

        // Mark as incomplete and pending
        self.cache.mark_incomplete(target_ip);
        let key = ArpCache::ip_to_u32(target_ip);
        self.pending_requests.insert(key, 0);

        None
    }

    /// Create ARP request for IP
    pub fn create_request(&self, target_ip: [u8; 4]) -> ArpPacket {
        ArpPacket::request(self.local_mac, self.local_ip, target_ip)
    }

    /// Get cache reference
    pub fn cache(&self) -> &ArpCache {
        &self.cache
    }

    /// Get mutable cache reference
    pub fn cache_mut(&mut self) -> &mut ArpCache {
        &mut self.cache
    }
}

/// ARP errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpError {
    PacketTooSmall,
    InvalidOperation,
    CacheFull,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arp_request() {
        let sender_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let sender_ip = [192, 168, 1, 10];
        let target_ip = [192, 168, 1, 1];

        let packet = ArpPacket::request(sender_mac, sender_ip, target_ip);
        assert_eq!(packet.get_operation(), ArpOperation::Request);
        assert_eq!(packet.sender_hw_addr, sender_mac);
        assert_eq!(packet.sender_proto_addr, sender_ip);
        assert_eq!(packet.target_proto_addr, target_ip);
    }

    #[test]
    fn test_arp_cache() {
        let mut cache = ArpCache::new(10, 300);

        let ip = [192, 168, 1, 1];
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];

        cache.insert(ip, mac);

        assert_eq!(cache.lookup(ip), Some(mac));
    }

    #[test]
    fn test_arp_handler() {
        let local_ip = [192, 168, 1, 10];
        let local_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];

        let mut handler = ArpHandler::new(local_ip, local_mac);

        // Receive ARP request for us
        let request = ArpPacket::request(
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            [192, 168, 1, 1],
            local_ip,
        );

        let reply = handler.handle_packet(&request);
        assert!(reply.is_some());

        let reply = reply.unwrap();
        assert_eq!(reply.get_operation(), ArpOperation::Reply);
        assert_eq!(reply.sender_hw_addr, local_mac);
    }
}
