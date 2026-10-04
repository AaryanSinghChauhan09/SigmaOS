//! # ICMP Protocol
//!
//! Internet Control Message Protocol (RFC 792) for network diagnostics and error reporting.
//! Inspired by Linux net/ipv4/icmp.c and FreeBSD netinet/ip_icmp.c.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// ICMP message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpType {
    EchoReply = 0,
    DestinationUnreachable = 3,
    SourceQuench = 4,
    Redirect = 5,
    EchoRequest = 8,
    RouterAdvertisement = 9,
    RouterSolicitation = 10,
    TimeExceeded = 11,
    ParameterProblem = 12,
    Timestamp = 13,
    TimestampReply = 14,
    InformationRequest = 15,
    InformationReply = 16,
    AddressMaskRequest = 17,
    AddressMaskReply = 18,
}

/// ICMP destination unreachable codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpUnreachableCode {
    NetUnreachable = 0,
    HostUnreachable = 1,
    ProtocolUnreachable = 2,
    PortUnreachable = 3,
    FragmentationNeeded = 4,
    SourceRouteFailed = 5,
    NetUnknown = 6,
    HostUnknown = 7,
    SourceHostIsolated = 8,
    NetProhibited = 9,
    HostProhibited = 10,
    NetUnreachableForTOS = 11,
    HostUnreachableForTOS = 12,
    CommunicationProhibited = 13,
    HostPrecedenceViolation = 14,
    PrecedenceCutoff = 15,
}

/// ICMP time exceeded codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpTimeExceededCode {
    TTLExceeded = 0,
    FragmentReassemblyTimeExceeded = 1,
}

/// ICMP redirect codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpRedirectCode {
    RedirectForNetwork = 0,
    RedirectForHost = 1,
    RedirectForTOSAndNetwork = 2,
    RedirectForTOSAndHost = 3,
}

/// ICMP header (8 bytes minimum)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IcmpHeader {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16, // Big-endian
    pub rest: [u8; 4], // Type-specific data
}

impl IcmpHeader {
    /// Create echo request
    pub fn echo_request(id: u16, seq: u16) -> Self {
        let mut header = Self {
            icmp_type: IcmpType::EchoRequest as u8,
            code: 0,
            checksum: 0,
            rest: [0; 4],
        };

        // Store ID and sequence in rest field
        header.rest[0..2].copy_from_slice(&id.to_be_bytes());
        header.rest[2..4].copy_from_slice(&seq.to_be_bytes());

        header
    }

    /// Create echo reply
    pub fn echo_reply(id: u16, seq: u16) -> Self {
        let mut header = Self {
            icmp_type: IcmpType::EchoReply as u8,
            code: 0,
            checksum: 0,
            rest: [0; 4],
        };

        header.rest[0..2].copy_from_slice(&id.to_be_bytes());
        header.rest[2..4].copy_from_slice(&seq.to_be_bytes());

        header
    }

    /// Create destination unreachable
    pub fn dest_unreachable(code: IcmpUnreachableCode, next_hop_mtu: u16) -> Self {
        let mut header = Self {
            icmp_type: IcmpType::DestinationUnreachable as u8,
            code: code as u8,
            checksum: 0,
            rest: [0; 4],
        };

        // For fragmentation needed, include MTU
        if code == IcmpUnreachableCode::FragmentationNeeded {
            header.rest[2..4].copy_from_slice(&next_hop_mtu.to_be_bytes());
        }

        header
    }

    /// Create time exceeded
    pub fn time_exceeded(code: IcmpTimeExceededCode) -> Self {
        Self {
            icmp_type: IcmpType::TimeExceeded as u8,
            code: code as u8,
            checksum: 0,
            rest: [0; 4],
        }
    }

    /// Parse ICMP header from bytes
    pub fn parse(data: &[u8]) -> Result<Self, IcmpError> {
        if data.len() < 8 {
            return Err(IcmpError::PacketTooSmall);
        }

        Ok(Self {
            icmp_type: data[0],
            code: data[1],
            checksum: u16::from_be_bytes([data[2], data[3]]),
            rest: [data[4], data[5], data[6], data[7]],
        })
    }

    /// Calculate checksum
    pub fn calculate_checksum(&self, payload: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        // Add header (excluding checksum field)
        sum += (self.icmp_type as u32) << 8;
        sum += self.code as u32;
        // Skip checksum field
        sum += u32::from_be_bytes([0, 0, self.rest[0], self.rest[1]]);
        sum += u32::from_be_bytes([self.rest[2], self.rest[3], 0, 0]);

        // Add payload
        let mut i = 0;
        while i + 1 < payload.len() {
            sum += ((payload[i] as u32) << 8) | (payload[i + 1] as u32);
            i += 2;
        }

        // Add last byte if odd length
        if i < payload.len() {
            sum += (payload[i] as u32) << 8;
        }

        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    /// Set checksum
    pub fn set_checksum(&mut self, payload: &[u8]) {
        self.checksum = 0;
        let checksum = self.calculate_checksum(payload);
        self.checksum = checksum.to_be();
    }

    /// Verify checksum
    pub fn verify_checksum(&self, payload: &[u8]) -> bool {
        let calculated = self.calculate_checksum(payload);
        u16::from_be(self.checksum) == calculated
    }

    /// Get echo ID
    pub fn get_echo_id(&self) -> u16 {
        u16::from_be_bytes([self.rest[0], self.rest[1]])
    }

    /// Get echo sequence
    pub fn get_echo_seq(&self) -> u16 {
        u16::from_be_bytes([self.rest[2], self.rest[3]])
    }
}

/// ICMP packet
#[derive(Debug, Clone)]
pub struct IcmpPacket {
    pub header: IcmpHeader,
    pub payload: Vec<u8>,
}

impl IcmpPacket {
    /// Create new ICMP packet
    pub fn new(header: IcmpHeader, payload: Vec<u8>) -> Self {
        Self { header, payload }
    }

    /// Create echo request packet
    pub fn echo_request(id: u16, seq: u16, payload: Vec<u8>) -> Self {
        let mut header = IcmpHeader::echo_request(id, seq);
        header.set_checksum(&payload);
        Self::new(header, payload)
    }

    /// Create echo reply packet
    pub fn echo_reply(id: u16, seq: u16, payload: Vec<u8>) -> Self {
        let mut header = IcmpHeader::echo_reply(id, seq);
        header.set_checksum(&payload);
        Self::new(header, payload)
    }

    /// Parse ICMP packet from bytes
    pub fn parse(data: &[u8]) -> Result<Self, IcmpError> {
        if data.len() < 8 {
            return Err(IcmpError::PacketTooSmall);
        }

        let header = IcmpHeader::parse(&data[..8])?;
        let payload = data[8..].to_vec();

        Ok(Self { header, payload })
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len());

        bytes.push(self.header.icmp_type);
        bytes.push(self.header.code);
        bytes.extend_from_slice(&self.header.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.header.rest);
        bytes.extend_from_slice(&self.payload);

        bytes
    }
}

/// ICMP handler
pub struct IcmpHandler {
    echo_id: u16,
    next_seq: AtomicU64,
    stats: IcmpStats,
}

impl IcmpHandler {
    pub fn new(echo_id: u16) -> Self {
        Self {
            echo_id,
            next_seq: AtomicU64::new(0),
            stats: IcmpStats::default(),
        }
    }

    /// Handle incoming ICMP packet
    pub fn handle_packet(&mut self, packet: &IcmpPacket) -> Option<IcmpPacket> {
        match packet.header.icmp_type {
            t if t == IcmpType::EchoRequest as u8 => {
                self.stats.echo_requests.fetch_add(1, Ordering::Relaxed);

                // Create echo reply
                let reply = IcmpPacket::echo_reply(
                    packet.header.get_echo_id(),
                    packet.header.get_echo_seq(),
                    packet.payload.clone(),
                );

                self.stats.echo_replies.fetch_add(1, Ordering::Relaxed);
                Some(reply)
            }
            t if t == IcmpType::EchoReply as u8 => {
                self.stats.echo_replies_recv.fetch_add(1, Ordering::Relaxed);
                None
            }
            t if t == IcmpType::DestinationUnreachable as u8 => {
                self.stats.dest_unreachable.fetch_add(1, Ordering::Relaxed);
                None
            }
            t if t == IcmpType::TimeExceeded as u8 => {
                self.stats.time_exceeded.fetch_add(1, Ordering::Relaxed);
                None
            }
            _ => {
                self.stats.unknown_type.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }

    /// Create echo request (ping)
    pub fn create_ping(&self, payload: Vec<u8>) -> IcmpPacket {
        let seq = self.next_seq.fetch_add(1, Ordering::Relaxed) as u16;
        IcmpPacket::echo_request(self.echo_id, seq, payload)
    }

    /// Get statistics
    pub fn get_stats(&self) -> IcmpStats {
        self.stats.clone()
    }
}

/// ICMP statistics
#[derive(Debug, Default)]
pub struct IcmpStats {
    pub echo_requests: AtomicU64,
    pub echo_replies: AtomicU64,
    pub echo_replies_recv: AtomicU64,
    pub dest_unreachable: AtomicU64,
    pub time_exceeded: AtomicU64,
    pub unknown_type: AtomicU64,
}

impl Clone for IcmpStats {
    fn clone(&self) -> Self {
        Self {
            echo_requests: AtomicU64::new(
                self.echo_requests
                    .load(core::sync::atomic::Ordering::Relaxed),
            ),
            echo_replies: AtomicU64::new(
                self.echo_replies
                    .load(core::sync::atomic::Ordering::Relaxed),
            ),
            echo_replies_recv: AtomicU64::new(
                self.echo_replies_recv
                    .load(core::sync::atomic::Ordering::Relaxed),
            ),
            dest_unreachable: AtomicU64::new(
                self.dest_unreachable
                    .load(core::sync::atomic::Ordering::Relaxed),
            ),
            time_exceeded: AtomicU64::new(
                self.time_exceeded
                    .load(core::sync::atomic::Ordering::Relaxed),
            ),
            unknown_type: AtomicU64::new(
                self.unknown_type
                    .load(core::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

/// ICMP errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpError {
    PacketTooSmall,
    InvalidChecksum,
    InvalidType,
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_icmp_echo_request() {
        let header = IcmpHeader::echo_request(123, 456);
        assert_eq!(header.icmp_type, IcmpType::EchoRequest as u8);
        assert_eq!(header.get_echo_id(), 123);
        assert_eq!(header.get_echo_seq(), 456);
    }

    #[test]
    fn test_icmp_checksum() {
        let payload = b"Hello, ICMP!";
        let mut header = IcmpHeader::echo_request(1, 1);
        header.set_checksum(payload);

        assert!(header.verify_checksum(payload));
    }

    #[test]
    fn test_icmp_packet() {
        let payload = b"Test payload".to_vec();
        let packet = IcmpPacket::echo_request(100, 200, payload.clone());

        assert_eq!(packet.header.icmp_type, IcmpType::EchoRequest as u8);
        assert_eq!(packet.header.get_echo_id(), 100);
        assert_eq!(packet.header.get_echo_seq(), 200);
        assert_eq!(packet.payload, payload);
    }

    #[test]
    fn test_icmp_handler() {
        let mut handler = IcmpHandler::new(1);

        let request = IcmpPacket::echo_request(1, 1, b"ping".to_vec());
        let reply = handler.handle_packet(&request);

        assert!(reply.is_some());
        let reply = reply.unwrap();
        assert_eq!(reply.header.icmp_type, IcmpType::EchoReply as u8);
        assert_eq!(reply.payload, b"ping");
    }
}
