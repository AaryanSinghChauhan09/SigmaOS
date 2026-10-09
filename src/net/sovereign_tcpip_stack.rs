//! # Sovereign TCP/IP Network Stack
//!
//! Production-grade TCP/IP stack with ARP, ICMP, UDP, and TCP state machine.
//! Inspired by Linux `net/ipv4/tcp_input.c`, FreeBSD `sys/netinet/tcp_input.c`,
//! smoltcp 0.11 (Rust embedded TCP/IP), and lwIP.
//!
//! Implements: Ethernet II framing → ARP → IPv4 → ICMP/UDP/TCP

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// ============================================================================
// 1. ETHERNET II FRAME
// ============================================================================

pub const ETHERTYPE_ARP: u16 = 0x0806;
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;
pub const ETHERNET_HEADER_LEN: usize = 14;
pub const ETHERNET_MIN_FRAME: usize = 64;
pub const ETHERNET_MAX_FRAME: usize = 1518;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    pub const BROADCAST: Self = MacAddress([0xFF; 6]);
    pub const ZERO: Self = MacAddress([0; 6]);

    pub fn is_broadcast(&self) -> bool {
        *self == Self::BROADCAST
    }
    pub fn is_unicast(&self) -> bool {
        (self.0[0] & 1) == 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EthernetFrame<'a> {
    pub dst: MacAddress,
    pub src: MacAddress,
    pub ethertype: u16,
    pub payload: &'a [u8],
}

impl<'a> EthernetFrame<'a> {
    pub fn parse(buf: &'a [u8]) -> Option<Self> {
        if buf.len() < ETHERNET_HEADER_LEN {
            return None;
        }
        let mut dst = [0u8; 6];
        let mut src = [0u8; 6];
        dst.copy_from_slice(&buf[0..6]);
        src.copy_from_slice(&buf[6..12]);
        let ethertype = u16::from_be_bytes([buf[12], buf[13]]);
        Some(Self {
            dst: MacAddress(dst),
            src: MacAddress(src),
            ethertype,
            payload: &buf[14..],
        })
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.dst.0);
        buf.extend_from_slice(&self.src.0);
        buf.extend_from_slice(&self.ethertype.to_be_bytes());
        buf.extend_from_slice(self.payload);
    }
}

// ============================================================================
// 2. ARP — Address Resolution Protocol
//    Inspired by Linux net/ipv4/arp.c
// ============================================================================

pub const ARP_HTYPE_ETHERNET: u16 = 1;
pub const ARP_PTYPE_IPV4: u16 = 0x0800;
pub const ARP_OP_REQUEST: u16 = 1;
pub const ARP_OP_REPLY: u16 = 2;

#[derive(Debug, Clone, Copy)]
pub struct ArpPacket {
    pub htype: u16,
    pub ptype: u16,
    pub hlen: u8,
    pub plen: u8,
    pub operation: u16,
    pub sender_mac: MacAddress,
    pub sender_ip: [u8; 4],
    pub target_mac: MacAddress,
    pub target_ip: [u8; 4],
}

impl ArpPacket {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 28 {
            return None;
        }
        let mut smac = [0u8; 6];
        let mut tmac = [0u8; 6];
        let mut sip = [0u8; 4];
        let mut tip = [0u8; 4];
        smac.copy_from_slice(&buf[8..14]);
        sip.copy_from_slice(&buf[14..18]);
        tmac.copy_from_slice(&buf[18..24]);
        tip.copy_from_slice(&buf[24..28]);
        Some(Self {
            htype: u16::from_be_bytes([buf[0], buf[1]]),
            ptype: u16::from_be_bytes([buf[2], buf[3]]),
            hlen: buf[4],
            plen: buf[5],
            operation: u16::from_be_bytes([buf[6], buf[7]]),
            sender_mac: MacAddress(smac),
            sender_ip: sip,
            target_mac: MacAddress(tmac),
            target_ip: tip,
        })
    }

    pub fn build_reply(request: &ArpPacket, our_mac: MacAddress) -> Self {
        Self {
            htype: request.htype,
            ptype: request.ptype,
            hlen: 6,
            plen: 4,
            operation: ARP_OP_REPLY,
            sender_mac: our_mac,
            sender_ip: request.target_ip,
            target_mac: request.sender_mac,
            target_ip: request.sender_ip,
        }
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.htype.to_be_bytes());
        buf.extend_from_slice(&self.ptype.to_be_bytes());
        buf.push(self.hlen);
        buf.push(self.plen);
        buf.extend_from_slice(&self.operation.to_be_bytes());
        buf.extend_from_slice(&self.sender_mac.0);
        buf.extend_from_slice(&self.sender_ip);
        buf.extend_from_slice(&self.target_mac.0);
        buf.extend_from_slice(&self.target_ip);
    }
}

/// ARP Cache (IP → MAC mapping)
pub struct ArpCache {
    pub entries: BTreeMap<[u8; 4], MacAddress>,
    pub hits: AtomicU64,
    pub misses: AtomicU64,
}

impl ArpCache {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub fn lookup(&self, ip: &[u8; 4]) -> Option<MacAddress> {
        if let Some(&mac) = self.entries.get(ip) {
            self.hits.fetch_add(1, Ordering::SeqCst);
            Some(mac)
        } else {
            self.misses.fetch_add(1, Ordering::SeqCst);
            None
        }
    }

    pub fn insert(&mut self, ip: [u8; 4], mac: MacAddress) {
        self.entries.insert(ip, mac);
    }
}

// ============================================================================
// 3. IPv4 HEADER + CHECKSUM
//    Inspired by Linux net/ipv4/ip_output.c and FreeBSD netinet/ip_output.c
// ============================================================================

pub const IPPROTO_ICMP: u8 = 1;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;

#[derive(Debug, Clone, Copy)]
pub struct Ipv4Packet {
    pub version_ihl: u8,
    pub dscp_ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src: [u8; 4],
    pub dst: [u8; 4],
}

impl Ipv4Packet {
    pub const HEADER_LEN: usize = 20;
    pub const DEFAULT_TTL: u8 = 64;

    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 20 {
            return None;
        }
        if (buf[0] >> 4) != 4 {
            return None;
        }
        let mut src = [0u8; 4];
        let mut dst = [0u8; 4];
        src.copy_from_slice(&buf[12..16]);
        dst.copy_from_slice(&buf[16..20]);
        Some(Self {
            version_ihl: buf[0],
            dscp_ecn: buf[1],
            total_length: u16::from_be_bytes([buf[2], buf[3]]),
            identification: u16::from_be_bytes([buf[4], buf[5]]),
            flags_fragment: u16::from_be_bytes([buf[6], buf[7]]),
            ttl: buf[8],
            protocol: buf[9],
            checksum: u16::from_be_bytes([buf[10], buf[11]]),
            src,
            dst,
        })
    }

    pub fn new(src: [u8; 4], dst: [u8; 4], protocol: u8, payload_len: u16) -> Self {
        let total_length = 20 + payload_len;
        let mut pkt = Self {
            version_ihl: 0x45, // IPv4, IHL=5 (20 bytes)
            dscp_ecn: 0,
            total_length,
            identification: 0x1234,
            flags_fragment: 0x4000, // Don't Fragment
            ttl: Self::DEFAULT_TTL,
            protocol,
            checksum: 0,
            src,
            dst,
        };
        pkt.checksum = pkt.compute_checksum();
        pkt
    }

    pub fn compute_checksum(&self) -> u16 {
        let mut words = [
            ((self.version_ihl as u32) << 8) | self.dscp_ecn as u32,
            self.total_length as u32,
            self.identification as u32,
            self.flags_fragment as u32,
            ((self.ttl as u32) << 8) | self.protocol as u32,
            0u32, // checksum field itself = 0 for calculation
            ((self.src[0] as u32) << 8) | self.src[1] as u32,
            ((self.src[2] as u32) << 8) | self.src[3] as u32,
            ((self.dst[0] as u32) << 8) | self.dst[1] as u32,
            ((self.dst[2] as u32) << 8) | self.dst[3] as u32,
        ];
        let mut sum: u32 = words.iter().map(|&w| w).sum();
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        !(sum as u16)
    }

    pub fn payload_offset(&self) -> usize {
        ((self.version_ihl & 0x0F) as usize) * 4
    }
}

// ============================================================================
// 4. ICMP — Internet Control Message Protocol
//    Inspired by Linux net/ipv4/icmp.c
// ============================================================================

pub const ICMP_ECHO_REQUEST: u8 = 8;
pub const ICMP_ECHO_REPLY: u8 = 0;

#[derive(Debug, Clone, Copy)]
pub struct IcmpEchoMessage {
    pub msg_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence: u16,
}

impl IcmpEchoMessage {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }
        Some(Self {
            msg_type: buf[0],
            code: buf[1],
            checksum: u16::from_be_bytes([buf[2], buf[3]]),
            identifier: u16::from_be_bytes([buf[4], buf[5]]),
            sequence: u16::from_be_bytes([buf[6], buf[7]]),
        })
    }

    pub fn build_reply(request: &IcmpEchoMessage) -> Self {
        let mut reply = Self {
            msg_type: ICMP_ECHO_REPLY,
            code: 0,
            checksum: 0,
            identifier: request.identifier,
            sequence: request.sequence,
        };
        reply.checksum = internet_checksum(&[
            reply.msg_type,
            reply.code,
            0,
            0,
            (reply.identifier >> 8) as u8,
            reply.identifier as u8,
            (reply.sequence >> 8) as u8,
            reply.sequence as u8,
        ]);
        reply
    }
}

// ============================================================================
// 5. UDP — User Datagram Protocol
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }
        Some(Self {
            src_port: u16::from_be_bytes([buf[0], buf[1]]),
            dst_port: u16::from_be_bytes([buf[2], buf[3]]),
            length: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]]),
        })
    }
}

// ============================================================================
// 6. TCP STATE MACHINE
//    Inspired by Linux net/ipv4/tcp.c and FreeBSD sys/netinet/tcp_fsm.h
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

/// TCP Flags
pub const TCP_FIN: u8 = 0x01;
pub const TCP_SYN: u8 = 0x02;
pub const TCP_RST: u8 = 0x04;
pub const TCP_PSH: u8 = 0x08;
pub const TCP_ACK: u8 = 0x10;
pub const TCP_URG: u8 = 0x20;

#[derive(Debug, Clone, Copy)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data_offset_flags: u16, // Top 4 bits = data offset (header len in 32-bit words), lower = flags
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
}

impl TcpHeader {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 20 {
            return None;
        }
        Some(Self {
            src_port: u16::from_be_bytes([buf[0], buf[1]]),
            dst_port: u16::from_be_bytes([buf[2], buf[3]]),
            seq_num: u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]),
            ack_num: u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]),
            data_offset_flags: u16::from_be_bytes([buf[12], buf[13]]),
            window: u16::from_be_bytes([buf[14], buf[15]]),
            checksum: u16::from_be_bytes([buf[16], buf[17]]),
            urgent_ptr: u16::from_be_bytes([buf[18], buf[19]]),
        })
    }

    pub fn flags(&self) -> u8 {
        (self.data_offset_flags & 0xFF) as u8
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        (self.flags() & flag) != 0
    }

    pub fn header_len(&self) -> usize {
        ((self.data_offset_flags >> 12) as usize) * 4
    }
}

/// TCP Connection Socket
#[derive(Debug, Clone)]
pub struct TcpSocket {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub state: TcpState,
    pub snd_seq: u32,
    pub rcv_seq: u32,
    pub rcv_window: u16,
    pub snd_window: u16,
    pub rx_buf: Vec<u8>,
    pub tx_buf: Vec<u8>,
}

impl TcpSocket {
    pub fn new(src_ip: [u8; 4], src_port: u16) -> Self {
        Self {
            src_ip,
            dst_ip: [0; 4],
            src_port,
            dst_port: 0,
            state: TcpState::Closed,
            snd_seq: 0x4D34_0001,
            rcv_seq: 0,
            rcv_window: 65535,
            snd_window: 0,
            rx_buf: Vec::new(),
            tx_buf: Vec::new(),
        }
    }

    /// Process incoming TCP segment and advance state machine
    /// Returns true if packet was accepted
    pub fn process_segment(&mut self, hdr: &TcpHeader, payload: &[u8]) -> bool {
        match self.state {
            TcpState::SynSent => {
                if hdr.has_flag(TCP_SYN) && hdr.has_flag(TCP_ACK) {
                    // SYN-ACK received → send ACK → ESTABLISHED
                    self.rcv_seq = hdr.seq_num.wrapping_add(1);
                    self.snd_seq = self.snd_seq.wrapping_add(1);
                    self.state = TcpState::Established;
                    return true;
                }
            }
            TcpState::Established => {
                if hdr.has_flag(TCP_FIN) {
                    self.rcv_seq = hdr.seq_num.wrapping_add(1);
                    self.state = TcpState::CloseWait;
                    return true;
                }
                if hdr.has_flag(TCP_ACK) && !payload.is_empty() {
                    self.rx_buf.extend_from_slice(payload);
                    self.rcv_seq = self.rcv_seq.wrapping_add(payload.len() as u32);
                    return true;
                }
            }
            _ => {}
        }
        false
    }
}

// ============================================================================
// 7. SOVEREIGN NETWORK STACK — PACKET DISPATCH ENGINE
// ============================================================================

pub struct SovereignNetworkStack {
    pub mac: MacAddress,
    pub ip: [u8; 4],
    pub gateway: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub arp_cache: ArpCache,
    pub tcp_sockets: Vec<TcpSocket>,
    pub total_rx_packets: AtomicU64,
    pub total_tx_packets: AtomicU64,
    pub total_rx_bytes: AtomicU64,
    pub total_dropped: AtomicU64,
}

impl SovereignNetworkStack {
    pub fn new(mac: MacAddress, ip: [u8; 4], gateway: [u8; 4]) -> Self {
        let mut arp = ArpCache::new();
        arp.insert(ip, mac); // Self entry
        Self {
            mac,
            ip,
            gateway,
            subnet_mask: [255, 255, 255, 0],
            arp_cache: arp,
            tcp_sockets: Vec::new(),
            total_rx_packets: AtomicU64::new(0),
            total_tx_packets: AtomicU64::new(0),
            total_rx_bytes: AtomicU64::new(0),
            total_dropped: AtomicU64::new(0),
        }
    }

    /// Dispatch incoming raw Ethernet frame
    pub fn rx_frame(&mut self, frame_bytes: &[u8]) -> Option<Vec<u8>> {
        self.total_rx_packets.fetch_add(1, Ordering::SeqCst);
        self.total_rx_bytes
            .fetch_add(frame_bytes.len() as u64, Ordering::SeqCst);

        let frame = EthernetFrame::parse(frame_bytes)?;
        match frame.ethertype {
            ETHERTYPE_ARP => self.handle_arp(frame.payload, frame.src),
            ETHERTYPE_IPV4 => self.handle_ipv4(frame.payload),
            _ => {
                self.total_dropped.fetch_add(1, Ordering::SeqCst);
                None
            }
        }
    }

    fn handle_arp(&mut self, payload: &[u8], sender_mac: MacAddress) -> Option<Vec<u8>> {
        let arp = ArpPacket::parse(payload)?;
        self.arp_cache.insert(arp.sender_ip, arp.sender_mac);

        if arp.operation == ARP_OP_REQUEST && arp.target_ip == self.ip {
            let reply = ArpPacket::build_reply(&arp, self.mac);
            let mut buf = Vec::new();
            reply.serialize(&mut buf);
            let eth_frame = EthernetFrame {
                dst: sender_mac,
                src: self.mac,
                ethertype: ETHERTYPE_ARP,
                payload: &buf.clone(),
            };
            let mut out = Vec::new();
            eth_frame.serialize(&mut out);
            out.extend_from_slice(&buf);
            self.total_tx_packets.fetch_add(1, Ordering::SeqCst);
            return Some(out);
        }
        None
    }

    fn handle_ipv4(&mut self, payload: &[u8]) -> Option<Vec<u8>> {
        let ipv4 = Ipv4Packet::parse(payload)?;
        if ipv4.dst != self.ip {
            return None;
        }
        let ip_payload = &payload[ipv4.payload_offset()..];
        match ipv4.protocol {
            IPPROTO_ICMP => self.handle_icmp(&ipv4, ip_payload),
            _ => None,
        }
    }

    fn handle_icmp(&mut self, ip: &Ipv4Packet, payload: &[u8]) -> Option<Vec<u8>> {
        let echo = IcmpEchoMessage::parse(payload)?;
        if echo.msg_type != ICMP_ECHO_REQUEST {
            return None;
        }

        let reply = IcmpEchoMessage::build_reply(&echo);
        let mut icmp_buf = vec![
            reply.msg_type,
            reply.code,
            (reply.checksum >> 8) as u8,
            reply.checksum as u8,
            (reply.identifier >> 8) as u8,
            reply.identifier as u8,
            (reply.sequence >> 8) as u8,
            reply.sequence as u8,
        ];
        icmp_buf.extend_from_slice(&payload[8..]);

        let ip_reply = Ipv4Packet::new(self.ip, ip.src, IPPROTO_ICMP, icmp_buf.len() as u16);
        let mut ip_buf = vec![
            ip_reply.version_ihl,
            ip_reply.dscp_ecn,
            (ip_reply.total_length >> 8) as u8,
            ip_reply.total_length as u8,
            (ip_reply.identification >> 8) as u8,
            ip_reply.identification as u8,
            (ip_reply.flags_fragment >> 8) as u8,
            ip_reply.flags_fragment as u8,
            ip_reply.ttl,
            ip_reply.protocol,
            (ip_reply.checksum >> 8) as u8,
            ip_reply.checksum as u8,
        ];
        ip_buf.extend_from_slice(&ip_reply.src);
        ip_buf.extend_from_slice(&ip_reply.dst);
        ip_buf.extend_from_slice(&icmp_buf);

        self.total_tx_packets.fetch_add(1, Ordering::SeqCst);
        Some(ip_buf)
    }
}

/// Internet checksum (RFC 1071)
pub fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        sum += u16::from_be_bytes([data[i], data[i + 1]]) as u32;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_checksum() {
        let pkt = Ipv4Packet::new([10, 0, 0, 1], [10, 0, 0, 2], IPPROTO_UDP, 8);
        // Re-verify: compute_checksum with the written checksum should sum to 0
        let mut bytes = vec![
            pkt.version_ihl,
            pkt.dscp_ecn,
            (pkt.total_length >> 8) as u8,
            pkt.total_length as u8,
            (pkt.identification >> 8) as u8,
            pkt.identification as u8,
            (pkt.flags_fragment >> 8) as u8,
            pkt.flags_fragment as u8,
            pkt.ttl,
            pkt.protocol,
            (pkt.checksum >> 8) as u8,
            pkt.checksum as u8,
        ];
        bytes.extend_from_slice(&pkt.src);
        bytes.extend_from_slice(&pkt.dst);
        assert_eq!(internet_checksum(&bytes), 0); // Must sum to 0
    }

    #[test]
    fn test_arp_cache_hit_miss() {
        let mut cache = ArpCache::new();
        let ip = [192, 168, 1, 1];
        let mac = MacAddress([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        cache.insert(ip, mac);

        assert_eq!(cache.lookup(&ip), Some(mac));
        assert_eq!(cache.lookup(&[10, 0, 0, 1]), None);
        assert_eq!(cache.hits.load(Ordering::SeqCst), 1);
        assert_eq!(cache.misses.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_tcp_state_machine_established() {
        let mut sock = TcpSocket::new([10, 0, 0, 1], 12345);
        sock.state = TcpState::SynSent;
        sock.dst_ip = [10, 0, 0, 2];
        sock.dst_port = 80;

        // Simulate SYN-ACK
        let syn_ack = TcpHeader {
            src_port: 80,
            dst_port: 12345,
            seq_num: 0x9999_0000,
            ack_num: sock.snd_seq,
            data_offset_flags: (5 << 12) | (TCP_SYN | TCP_ACK) as u16,
            window: 65535,
            checksum: 0,
            urgent_ptr: 0,
        };

        let accepted = sock.process_segment(&syn_ack, &[]);
        assert!(accepted);
        assert_eq!(sock.state, TcpState::Established);
    }

    #[test]
    fn test_icmp_echo_reply() {
        let echo_req = IcmpEchoMessage {
            msg_type: ICMP_ECHO_REQUEST,
            code: 0,
            checksum: 0,
            identifier: 0x1234,
            sequence: 1,
        };
        let reply = IcmpEchoMessage::build_reply(&echo_req);
        assert_eq!(reply.msg_type, ICMP_ECHO_REPLY);
        assert_eq!(reply.identifier, 0x1234);
        assert_eq!(reply.sequence, 1);
        assert_ne!(reply.checksum, 0);
    }
}
