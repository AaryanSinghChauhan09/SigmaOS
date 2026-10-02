//! # Internet Protocol (IPv4/IPv6)
//!
//! IP layer implementation inspired by Linux network stack.
//! Handles packet routing, fragmentation, and addressing.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::net::{Ipv4Addr, Ipv6Addr};

/// IP protocol numbers (IANA assigned)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IpProtocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
    Icmpv6 = 58,
}

/// IPv4 header (RFC 791)
#[derive(Debug, Clone)]
pub struct Ipv4Header {
    pub version: u8,         // 4
    pub ihl: u8,             // Header length in 32-bit words (5-15)
    pub dscp: u8,            // Differentiated Services Code Point
    pub ecn: u8,             // Explicit Congestion Notification
    pub total_length: u16,   // Total packet length
    pub identification: u16, // Fragment identification
    pub flags: Ipv4Flags,
    pub fragment_offset: u16, // 13 bits
    pub ttl: u8,              // Time to live
    pub protocol: u8,         // Upper layer protocol
    pub checksum: u16,
    pub src_addr: Ipv4Addr,
    pub dst_addr: Ipv4Addr,
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv4Flags {
    pub dont_fragment: bool,
    pub more_fragments: bool,
}

impl Ipv4Header {
    pub fn new(src: Ipv4Addr, dst: Ipv4Addr, protocol: IpProtocol) -> Self {
        Self {
            version: 4,
            ihl: 5, // 20 bytes (no options)
            dscp: 0,
            ecn: 0,
            total_length: 20,
            identification: 0,
            flags: Ipv4Flags {
                dont_fragment: true,
                more_fragments: false,
            },
            fragment_offset: 0,
            ttl: 64,
            protocol: protocol as u8,
            checksum: 0,
            src_addr: src,
            dst_addr: dst,
        }
    }

    /// Calculate header checksum
    pub fn calculate_checksum(&self) -> u16 {
        // Simplified checksum calculation
        let mut sum: u32 = 0;

        // Add header fields (16-bit words)
        sum += ((self.version as u32) << 12)
            | ((self.ihl as u32) << 8)
            | ((self.dscp as u32) << 2)
            | (self.ecn as u32);
        sum += self.total_length as u32;
        sum += self.identification as u32;
        sum += self.ttl as u32 | ((self.protocol as u32) << 8);

        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }
}

/// IPv6 header (RFC 2460)
#[derive(Debug, Clone)]
pub struct Ipv6Header {
    pub version: u8,         // 6
    pub traffic_class: u8,   // 8 bits
    pub flow_label: u32,     // 20 bits
    pub payload_length: u16, // Excludes header
    pub next_header: u8,     // Protocol
    pub hop_limit: u8,       // Like TTL
    pub src_addr: Ipv6Addr,
    pub dst_addr: Ipv6Addr,
}

impl Ipv6Header {
    pub fn new(src: Ipv6Addr, dst: Ipv6Addr, protocol: IpProtocol) -> Self {
        Self {
            version: 6,
            traffic_class: 0,
            flow_label: 0,
            payload_length: 0,
            next_header: protocol as u8,
            hop_limit: 64,
            src_addr: src,
            dst_addr: dst,
        }
    }
}

/// IP packet (either IPv4 or IPv6)
#[derive(Debug, Clone)]
pub enum IpPacket {
    V4 {
        header: Ipv4Header,
        payload: Vec<u8>,
    },
    V6 {
        header: Ipv6Header,
        payload: Vec<u8>,
    },
}

impl IpPacket {
    pub fn new_v4(src: Ipv4Addr, dst: Ipv4Addr, protocol: IpProtocol, payload: Vec<u8>) -> Self {
        let mut header = Ipv4Header::new(src, dst, protocol);
        header.total_length = 20 + payload.len() as u16;
        header.checksum = header.calculate_checksum();

        IpPacket::V4 { header, payload }
    }

    pub fn new_v6(src: Ipv6Addr, dst: Ipv6Addr, protocol: IpProtocol, payload: Vec<u8>) -> Self {
        let mut header = Ipv6Header::new(src, dst, protocol);
        header.payload_length = payload.len() as u16;

        IpPacket::V6 { header, payload }
    }

    pub fn protocol(&self) -> u8 {
        match self {
            IpPacket::V4 { header, .. } => header.protocol,
            IpPacket::V6 { header, .. } => header.next_header,
        }
    }

    pub fn payload(&self) -> &[u8] {
        match self {
            IpPacket::V4 { payload, .. } => payload,
            IpPacket::V6 { payload, .. } => payload,
        }
    }
}

/// Routing table entry
#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub destination: IpNetwork,
    pub gateway: Option<Ipv4Addr>,
    pub interface: u32,
    pub metric: u32,
}

/// IP network (CIDR notation)
#[derive(Debug, Clone, Copy)]
pub struct IpNetwork {
    pub addr: Ipv4Addr,
    pub prefix_len: u8,
}

impl IpNetwork {
    pub fn new(addr: Ipv4Addr, prefix_len: u8) -> Self {
        Self { addr, prefix_len }
    }

    /// Check if address is in this network
    pub fn contains(&self, addr: Ipv4Addr) -> bool {
        let mask = !((1u32 << (32 - self.prefix_len)) - 1);
        let network = u32::from(self.addr) & mask;
        let test = u32::from(addr) & mask;
        network == test
    }
}

/// Routing table
pub struct RoutingTable {
    entries: Vec<RouteEntry>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_route(&mut self, entry: RouteEntry) {
        self.entries.push(entry);
        // Sort by metric (lower is better)
        self.entries.sort_by_key(|e| e.metric);
    }

    pub fn lookup(&self, dst: Ipv4Addr) -> Option<&RouteEntry> {
        // Find most specific matching route (longest prefix match)
        self.entries
            .iter()
            .filter(|e| e.destination.contains(dst))
            .max_by_key(|e| e.destination.prefix_len)
    }

    pub fn remove_route(&mut self, dst: IpNetwork) {
        self.entries.retain(|e| {
            e.destination.addr != dst.addr || e.destination.prefix_len != dst.prefix_len
        });
    }
}

/// IP fragmentation support
pub struct IpFragmenter {
    next_id: u16,
    mtu: usize,
}

impl IpFragmenter {
    pub fn new(mtu: usize) -> Self {
        Self { next_id: 1, mtu }
    }

    /// Fragment packet if needed
    pub fn fragment(&mut self, packet: &IpPacket) -> Vec<IpPacket> {
        match packet {
            IpPacket::V4 { header, payload } => {
                let max_payload = self.mtu - 20; // IP header size

                if payload.len() <= max_payload {
                    // No fragmentation needed
                    return alloc::vec![packet.clone()];
                }

                let id = self.next_id;
                self.next_id = self.next_id.wrapping_add(1);

                let mut fragments = Vec::new();
                let mut offset = 0;

                while offset < payload.len() {
                    let remaining = payload.len() - offset;
                    let frag_size = remaining.min(max_payload);
                    let more_fragments = offset + frag_size < payload.len();

                    let mut frag_header = header.clone();
                    frag_header.identification = id;
                    frag_header.fragment_offset = (offset / 8) as u16;
                    frag_header.flags.more_fragments = more_fragments;
                    frag_header.total_length = 20 + frag_size as u16;
                    frag_header.checksum = frag_header.calculate_checksum();

                    let frag_payload = payload[offset..offset + frag_size].to_vec();
                    fragments.push(IpPacket::V4 {
                        header: frag_header,
                        payload: frag_payload,
                    });

                    offset += frag_size;
                }

                fragments
            }
            IpPacket::V6 { .. } => {
                // IPv6 uses Path MTU Discovery, no fragmentation by routers
                alloc::vec![packet.clone()]
            }
        }
    }
}

/// IP layer manager
pub struct IpLayer {
    pub routing_table: RoutingTable,
    pub fragmenter: IpFragmenter,
}

impl IpLayer {
    pub fn new(mtu: usize) -> Self {
        Self {
            routing_table: RoutingTable::new(),
            fragmenter: IpFragmenter::new(mtu),
        }
    }

    /// Send packet (routes and fragments if needed)
    pub fn send(&mut self, packet: IpPacket) -> Result<Vec<IpPacket>, IpError> {
        // Check destination and route
        let dst = match &packet {
            IpPacket::V4 { header, .. } => header.dst_addr,
            IpPacket::V6 { .. } => return Err(IpError::Unsupported), // Simplified
        };

        if self.routing_table.lookup(dst).is_none() {
            return Err(IpError::NoRoute);
        }

        // Fragment if needed
        Ok(self.fragmenter.fragment(&packet))
    }
}

/// IP errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpError {
    NoRoute,
    TtlExpired,
    FragmentationNeeded,
    Unsupported,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_header() {
        let src = Ipv4Addr::new(192, 168, 1, 1);
        let dst = Ipv4Addr::new(192, 168, 1, 2);
        let header = Ipv4Header::new(src, dst, IpProtocol::Tcp);

        assert_eq!(header.version, 4);
        assert_eq!(header.ttl, 64);
        assert_eq!(header.protocol, 6);
    }

    #[test]
    fn test_ip_network() {
        let network = IpNetwork::new(Ipv4Addr::new(192, 168, 1, 0), 24);
        assert!(network.contains(Ipv4Addr::new(192, 168, 1, 100)));
        assert!(!network.contains(Ipv4Addr::new(192, 168, 2, 1)));
    }

    #[test]
    fn test_routing_table() {
        let mut table = RoutingTable::new();
        table.add_route(RouteEntry {
            destination: IpNetwork::new(Ipv4Addr::new(192, 168, 1, 0), 24),
            gateway: None,
            interface: 0,
            metric: 0,
        });

        let route = table.lookup(Ipv4Addr::new(192, 168, 1, 50));
        assert!(route.is_some());
    }

    #[test]
    fn test_ip_packet() {
        let src = Ipv4Addr::new(10, 0, 0, 1);
        let dst = Ipv4Addr::new(10, 0, 0, 2);
        let payload = alloc::vec![1, 2, 3, 4, 5];

        let packet = IpPacket::new_v4(src, dst, IpProtocol::Tcp, payload.clone());
        assert_eq!(packet.protocol(), 6);
        assert_eq!(packet.payload(), &payload[..]);
    }

    #[test]
    fn test_fragmentation() {
        let mut fragmenter = IpFragmenter::new(100);
        let src = Ipv4Addr::new(10, 0, 0, 1);
        let dst = Ipv4Addr::new(10, 0, 0, 2);
        let payload = alloc::vec![0u8; 200];

        let packet = IpPacket::new_v4(src, dst, IpProtocol::Tcp, payload);
        let fragments = fragmenter.fragment(&packet);

        assert!(fragments.len() > 1);
    }
}
