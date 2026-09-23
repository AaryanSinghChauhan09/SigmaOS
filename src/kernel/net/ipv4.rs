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

/// SigmaOS IPv4 Network Stack
/// ARP, IPv4 routing, ICMP echo, raw packet I/O
/// Absorbs Linux net/ipv4/: arp.c, ip_input.c, ip_output.c, icmp.c
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// ── Types ────────────────────────────────────────────────────────────────

pub type Ipv4Addr = [u8; 4];
pub type MacAddr = [u8; 6];

pub const IPV4_BROADCAST: Ipv4Addr = [255, 255, 255, 255];
pub const IPV4_LOOPBACK: Ipv4Addr = [127, 0, 0, 1];
pub const MAC_BROADCAST: MacAddr = [0xFF; 6];

/// IPv4 packet header (RFC 791)
#[derive(Debug, Clone)]
pub struct Ipv4Header {
    pub version_ihl: u8, // version=4, IHL=5 (no options)
    pub dscp_ecn: u8,
    pub total_len: u16,
    pub id: u16,
    pub flags_frag: u16,
    pub ttl: u8,
    pub protocol: u8, // 1=ICMP, 6=TCP, 17=UDP
    pub checksum: u16,
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
}

impl Ipv4Header {
    pub fn new(src: Ipv4Addr, dst: Ipv4Addr, protocol: u8, payload_len: u16) -> Self {
        Ipv4Header {
            version_ihl: 0x45, // version 4, IHL 5 (20 bytes)
            dscp_ecn: 0,
            total_len: 20 + payload_len,
            id: 0,
            flags_frag: 0x4000, // DF bit set
            ttl: 64,
            protocol,
            checksum: 0, // filled after serialisation
            src,
            dst,
        }
    }

    pub fn header_len(&self) -> usize {
        ((self.version_ihl & 0x0F) as usize) * 4
    }
    pub fn version(&self) -> u8 {
        self.version_ihl >> 4
    }
}

/// ICMP message (RFC 792)
#[derive(Debug, Clone)]
pub struct IcmpMessage {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub id: u16,
    pub seq: u16,
    pub data: Vec<u8>,
}

impl IcmpMessage {
    pub fn echo_request(id: u16, seq: u16, data: Vec<u8>) -> Self {
        IcmpMessage {
            icmp_type: 8,
            code: 0,
            checksum: 0,
            id,
            seq,
            data,
        }
    }
    pub fn echo_reply(id: u16, seq: u16, data: Vec<u8>) -> Self {
        IcmpMessage {
            icmp_type: 0,
            code: 0,
            checksum: 0,
            id,
            seq,
            data,
        }
    }
    pub fn is_echo_request(&self) -> bool {
        self.icmp_type == 8
    }
    pub fn is_echo_reply(&self) -> bool {
        self.icmp_type == 0
    }
}

// ── ARP table ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpState {
    Incomplete,
    Reachable,
    Stale,
    Delay,
    Probe,
    Permanent,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub ip: Ipv4Addr,
    pub mac: MacAddr,
    pub state: ArpState,
    pub created_at_ticks: u64,
    pub ttl_ticks: u64,
    pub is_static: bool,
}

pub struct ArpTable {
    entries: BTreeMap<[u8; 4], ArpEntry>,
    lookups: AtomicUsize,
    miss_count: AtomicUsize,
}

impl ArpTable {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ArpTable {
            entries: BTreeMap::new(),
            lookups: AtomicUsize::new(0),
            miss_count: AtomicUsize::new(0),
        }
    }

    pub fn insert(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        self.insert_with_ttl(ip, mac, 300, false);
    }

    pub fn insert_static(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        self.insert_with_ttl(ip, mac, u64::MAX, true);
    }

    pub fn insert_with_ttl(&mut self, ip: Ipv4Addr, mac: MacAddr, ttl_ticks: u64, is_static: bool) {
        let state = if is_static { ArpState::Permanent } else { ArpState::Reachable };
        self.entries.insert(
            ip,
            ArpEntry {
                ip,
                mac,
                state,
                created_at_ticks: 0,
                ttl_ticks,
                is_static,
            },
        );
    }

    pub fn flush_stale_entries(&mut self, current_ticks: u64) -> usize {
        let mut to_remove = Vec::new();
        for (ip, entry) in self.entries.iter_mut() {
            if entry.is_static || entry.state == ArpState::Permanent {
                continue;
            }
            if current_ticks.saturating_sub(entry.created_at_ticks) >= entry.ttl_ticks {
                if entry.state == ArpState::Reachable {
                    entry.state = ArpState::Stale;
                } else if entry.state == ArpState::Stale {
                    entry.state = ArpState::Failed;
                    to_remove.push(*ip);
                }
            }
        }
        let removed_count = to_remove.len();
        for ip in to_remove {
            self.entries.remove(&ip);
        }
        removed_count
    }

    /// Construct a Gratuitous ARP packet (ARP Request or Reply announcing IP/MAC mapping to LAN)
    pub fn build_gratuitous_arp_packet(sender_ip: Ipv4Addr, sender_mac: MacAddr, is_reply: bool) -> Vec<u8> {
        let mut packet = Vec::with_capacity(42);
        // Ethernet Header
        packet.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]); // Broadcast dst
        packet.extend_from_slice(&sender_mac);                           // Src MAC
        packet.extend_from_slice(&[0x08, 0x06]);                         // EtherType ARP (0x0806)

        // ARP Payload
        packet.extend_from_slice(&[0x00, 0x01]);                         // Hardware Type: Ethernet (1)
        packet.extend_from_slice(&[0x08, 0x00]);                         // Protocol Type: IPv4 (0x0800)
        packet.push(6);                                                   // HW Size: 6
        packet.push(4);                                                   // Proto Size: 4
        let opcode: u16 = if is_reply { 2 } else { 1 };
        packet.extend_from_slice(&opcode.to_be_bytes());                 // Opcode
        packet.extend_from_slice(&sender_mac);                           // Sender HW Addr
        packet.extend_from_slice(&sender_ip);                            // Sender Proto Addr
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // Target HW Addr (unknown / zero)
        packet.extend_from_slice(&sender_ip);                            // Target Proto Addr (own IP for gratuitous ARP)

        packet
    }

    pub fn lookup(&self, ip: &Ipv4Addr) -> Option<&ArpEntry> {
        self.lookups.fetch_add(1, Ordering::Relaxed);
        let entry = self.entries.get(ip);
        if entry.is_none() {
            self.miss_count.fetch_add(1, Ordering::Relaxed);
        }
        entry
    }

    pub fn remove(&mut self, ip: &Ipv4Addr) {
        self.entries.remove(ip);
    }
    pub fn lookups(&self) -> usize {
        self.lookups.load(Ordering::Relaxed)
    }
    pub fn miss_count(&self) -> usize {
        self.miss_count.load(Ordering::Relaxed)
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for ArpTable {
    fn default() -> Self {
        Self::new()
    }
}

// ── IPv4 Routing table ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Route {
    pub network: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub gateway: Option<Ipv4Addr>,
    pub iface: String,
    pub metric: u32,
}

impl Route {
    pub fn matches(&self, dst: &Ipv4Addr) -> bool {
        for i in 0..4 {
            if dst[i] & self.netmask[i] != self.network[i] {
                return false;
            }
        }
        true
    }

    pub fn prefix_len(&self) -> u8 {
        let mut bits = 0u8;
        for b in &self.netmask {
            bits += b.count_ones() as u8;
        }
        bits
    }
}

pub struct RoutingTable {
    routes: Vec<Route>,
    lookups: AtomicUsize,
}

impl RoutingTable {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        RoutingTable {
            routes: Vec::new(),
            lookups: AtomicUsize::new(0),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Longest-prefix match
    pub fn lookup(&self, dst: &Ipv4Addr) -> Option<&Route> {
        self.lookups.fetch_add(1, Ordering::Relaxed);
        self.routes
            .iter()
            .filter(|r| r.matches(dst))
            .max_by_key(|r| r.prefix_len())
    }

    pub fn del_route(&mut self, network: &Ipv4Addr) {
        self.routes.retain(|r| &r.network != network);
    }

    pub fn lookups(&self) -> usize {
        self.lookups.load(Ordering::Relaxed)
    }
}

impl Default for RoutingTable {
    fn default() -> Self {
        Self::new()
    }
}

// ── IPv4 stack ────────────────────────────────────────────────────────────

pub struct Ipv4Stack {
    pub local_ip: Ipv4Addr,
    pub local_mac: MacAddr,
    pub arp_table: ArpTable,
    pub routing_table: RoutingTable,
    rx_packets: AtomicUsize,
    tx_packets: AtomicUsize,
    icmp_replies_sent: AtomicUsize,
}

impl Ipv4Stack {
    pub fn new(ip: Ipv4Addr, mac: MacAddr) -> Self {
        let mut stack = Ipv4Stack {
            local_ip: ip,
            local_mac: mac,
            arp_table: ArpTable::new(),
            routing_table: RoutingTable::new(),
            rx_packets: AtomicUsize::new(0),
            tx_packets: AtomicUsize::new(0),
            icmp_replies_sent: AtomicUsize::new(0),
        };
        // Default loopback route
        stack.routing_table.add_route(Route {
            network: [127, 0, 0, 0],
            netmask: [255, 0, 0, 0],
            gateway: None,
            iface: "lo".to_string(),
            metric: 0,
        });
        stack.arp_table.insert([127, 0, 0, 1], [0; 6]);
        stack
    }

    /// Process an incoming IPv4 packet
    pub fn receive(&mut self, hdr: &Ipv4Header, payload: &[u8]) -> Option<IcmpMessage> {
        self.rx_packets.fetch_add(1, Ordering::Relaxed);

        // Handle ICMP
        if hdr.protocol == 1 && payload.len() >= 8 {
            let icmp_type = payload[0];
            let id = u16::from_be_bytes([payload[4], payload[5]]);
            let seq = u16::from_be_bytes([payload[6], payload[7]]);
            if icmp_type == 8 {
                // Echo request
                self.icmp_replies_sent.fetch_add(1, Ordering::Relaxed);
                let mut data = Vec::with_capacity(payload.len().saturating_sub(8));
                if payload.len() > 8 {
                    for &b in &payload[8..] {
                        data.push(b);
                    }
                }
                return Some(IcmpMessage::echo_reply(id, seq, data));
            }
        }
        None
    }

    /// Build and "send" an IPv4 packet
    pub fn send(
        &mut self,
        dst: Ipv4Addr,
        protocol: u8,
        payload: &[u8],
    ) -> Result<Ipv4Header, &'static str> {
        let _route = self.routing_table.lookup(&dst).ok_or("No route to host")?;
        let hdr = Ipv4Header::new(self.local_ip, dst, protocol, payload.len() as u16);
        self.tx_packets.fetch_add(1, Ordering::Relaxed);
        Ok(hdr)
    }

    pub fn rx_packets(&self) -> usize {
        self.rx_packets.load(Ordering::Relaxed)
    }
    pub fn tx_packets(&self) -> usize {
        self.tx_packets.load(Ordering::Relaxed)
    }
    pub fn icmp_replies_sent(&self) -> usize {
        self.icmp_replies_sent.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_stack() -> Ipv4Stack {
        let mut stack = Ipv4Stack::new([192, 168, 1, 100], [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        stack
            .arp_table
            .insert([192, 168, 1, 1], [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        stack.routing_table.add_route(Route {
            network: [0, 0, 0, 0],
            netmask: [0, 0, 0, 0],
            gateway: Some([192, 168, 1, 1]),
            iface: "eth0".to_string(),
            metric: 100,
        });
        stack
    }

    #[test]
    fn test_arp_table() {
        let mut arp = ArpTable::new();
        arp.insert([10, 0, 0, 1], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        let entry = arp.lookup(&[10, 0, 0, 1]).unwrap();
        assert_eq!(entry.state, ArpState::Reachable);
        assert_eq!(arp.miss_count(), 0);
        arp.lookup(&[10, 0, 0, 2]);
        assert_eq!(arp.miss_count(), 1);
    }

    #[test]
    fn test_routing_longest_prefix() {
        let mut rt = RoutingTable::new();
        rt.add_route(Route {
            network: [0, 0, 0, 0],
            netmask: [0, 0, 0, 0],
            gateway: Some([192, 168, 1, 1]),
            iface: "eth0".to_string(),
            metric: 100,
        });
        rt.add_route(Route {
            network: [192, 168, 1, 0],
            netmask: [255, 255, 255, 0],
            gateway: None,
            iface: "eth0".to_string(),
            metric: 0,
        });
        let r = rt.lookup(&[192, 168, 1, 50]).unwrap();
        assert_eq!(r.prefix_len(), 24); // /24 should win over /0
    }

    #[test]
    fn test_icmp_ping() {
        let mut stack = local_stack();
        let icmp_req_payload = {
            let mut v = vec![8u8, 0, 0, 0]; // type=8, code=0, checksum=0
            v.extend_from_slice(&1u16.to_be_bytes()); // id=1
            v.extend_from_slice(&1u16.to_be_bytes()); // seq=1
            v.extend_from_slice(b"SigmaOS");
            v
        };
        let hdr = Ipv4Header::new(
            [10, 0, 0, 1],
            [192, 168, 1, 100],
            1,
            icmp_req_payload.len() as u16,
        );
        let reply = stack.receive(&hdr, &icmp_req_payload).unwrap();
        assert!(reply.is_echo_reply());
        assert_eq!(stack.icmp_replies_sent(), 1);
    }

    #[test]
    fn test_ipv4_send_route() {
        let mut stack = local_stack();
        let hdr = stack.send([8, 8, 8, 8], 17, b"hello udp").unwrap(); // UDP to Google DNS
        assert_eq!(hdr.ttl, 64);
        assert_eq!(hdr.protocol, 17);
        assert_eq!(stack.tx_packets(), 1);
    }

    #[test]
    fn test_no_route() {
        let mut stack = Ipv4Stack::new([192, 168, 1, 100], [0; 6]); // no routes added
                                                                    // Only loopback route exists
        let result = stack.send([8, 8, 8, 8], 1, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_arp_table_gratuitous_and_eviction() {
        let mut arp = ArpTable::new();
        arp.insert_static([192, 168, 1, 1], [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        arp.insert_with_ttl([192, 168, 1, 2], [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF], 10, false);

        assert_eq!(arp.lookup(&[192, 168, 1, 1]).unwrap().state, ArpState::Permanent);
        assert_eq!(arp.lookup(&[192, 168, 1, 2]).unwrap().state, ArpState::Reachable);

        // At 15 ticks, Reachable -> Stale
        arp.flush_stale_entries(15);
        assert_eq!(arp.lookup(&[192, 168, 1, 2]).unwrap().state, ArpState::Stale);

        // At 25 ticks, Stale -> Failed & Removed
        let removed = arp.flush_stale_entries(25);
        assert_eq!(removed, 1);
        assert!(arp.lookup(&[192, 168, 1, 2]).is_none());
        assert!(arp.lookup(&[192, 168, 1, 1]).is_some()); // Static retained

        let garp = ArpTable::build_gratuitous_arp_packet([10, 0, 0, 1], [0x00, 0x11, 0x22, 0x33, 0x44, 0x55], false);
        assert_eq!(garp.len(), 42);
        assert_eq!(&garp[0..6], &[0xFF; 6]); // Broadcast Ethernet
    }
}
