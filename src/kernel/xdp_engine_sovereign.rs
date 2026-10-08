//! SigmaOS eBPF XDP Zero-Copy Network Engine
//!
//! Sovereign implementation of Linux XDP (eXpress Data Path) for sub-microsecond
//! packet processing without kernel network stack overhead.
//!
//! Inspired by:
//! - Linux XDP (io_uring author Jesper Dangaard Brouer, Daniel Borkmann)
//! - AF_XDP zero-copy socket map redirection (sockmap/DEVMAP)
//! - Cloudflare's goroutine-free DDoS mitigation using XDP DROP
//! - Cilium eBPF-based Kubernetes networking
//! - FreeBSD netmap — user-space zero-copy packet I/O
//!
//! Provides:
//! - XDP action dispatch (PASS / DROP / REDIRECT / TX / ABORTED)
//! - Sockmap zero-copy socket redirect
//! - Packet parser for Ethernet/IPv4/IPv6/TCP/UDP frames
//! - Per-interface statistics counters
//! - BPF map emulation (array, hash, LRU hash, sockmap)

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::format;
use std::string::String;
use std::vec::Vec;

// ─── XDP Action ───────────────────────────────────────────────────────────────

/// XDP program return codes (matches Linux include/uapi/linux/bpf.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    /// Pass packet to the kernel network stack (normal path)
    Pass,
    /// Drop packet immediately — zero-copy drop for DDoS mitigation
    Drop,
    /// Redirect to another interface index (zero-copy forwarding)
    Redirect(u32),
    /// Transmit packet back out the same interface (XDP_TX)
    Tx,
    /// Program error — fall back to kernel
    Aborted,
}

impl XdpAction {
    pub fn label(&self) -> &'static str {
        match self {
            XdpAction::Pass => "PASS",
            XdpAction::Drop => "DROP",
            XdpAction::Redirect(_) => "REDIRECT",
            XdpAction::Tx => "TX",
            XdpAction::Aborted => "ABORTED",
        }
    }
}

// ─── Ethernet Frame Parser ────────────────────────────────────────────────────

/// Ethernet II frame header (14 bytes)
#[derive(Debug, Clone)]
pub struct EthernetHeader {
    /// Destination MAC address
    pub dst_mac: [u8; 6],
    /// Source MAC address
    pub src_mac: [u8; 6],
    /// EtherType (0x0800 = IPv4, 0x86DD = IPv6, 0x0806 = ARP)
    pub ethertype: u16,
}

/// EtherType constants
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;
pub const ETHERTYPE_ARP: u16 = 0x0806;
pub const ETHERTYPE_VLAN: u16 = 0x8100;

impl EthernetHeader {
    /// Parse from raw bytes — returns None if fewer than 14 bytes
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 14 {
            return None;
        }
        let mut dst_mac = [0u8; 6];
        let mut src_mac = [0u8; 6];
        dst_mac.copy_from_slice(&data[0..6]);
        src_mac.copy_from_slice(&data[6..12]);
        let ethertype = ((data[12] as u16) << 8) | (data[13] as u16);
        Some(EthernetHeader {
            dst_mac,
            src_mac,
            ethertype,
        })
    }

    /// Returns true if this is an IPv4 packet
    pub fn is_ipv4(&self) -> bool {
        self.ethertype == ETHERTYPE_IPV4
    }
    /// Returns true if this is an IPv6 packet
    pub fn is_ipv6(&self) -> bool {
        self.ethertype == ETHERTYPE_IPV6
    }
    /// Returns true if dst MAC is the broadcast address
    pub fn is_broadcast(&self) -> bool {
        self.dst_mac == [0xFF; 6]
    }
    /// Returns true if dst MAC is multicast
    pub fn is_multicast(&self) -> bool {
        self.dst_mac[0] & 0x01 != 0
    }
}

/// Minimal IPv4 header fields needed for XDP decisions
#[derive(Debug, Clone)]
pub struct Ipv4Header {
    pub src_addr: [u8; 4],
    pub dst_addr: [u8; 4],
    pub protocol: u8, // 6=TCP, 17=UDP, 1=ICMP
    pub ttl: u8,
    pub total_len: u16,
}

impl Ipv4Header {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 20 || data[0] >> 4 != 4 {
            return None;
        }
        let ihl = (data[0] & 0x0F) as usize * 4;
        if ihl < 20 || data.len() < ihl {
            return None;
        }
        let total_len = u16::from_be_bytes([data[2], data[3]]);
        if (total_len as usize) < ihl || (total_len as usize) > data.len() {
            return None;
        }
        let ttl = data[8];
        let protocol = data[9];
        let mut src_addr = [0u8; 4];
        let mut dst_addr = [0u8; 4];
        src_addr.copy_from_slice(&data[12..16]);
        dst_addr.copy_from_slice(&data[16..20]);
        Some(Ipv4Header {
            src_addr,
            dst_addr,
            protocol,
            ttl,
            total_len,
        })
    }

    pub fn src_str(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.src_addr[0], self.src_addr[1], self.src_addr[2], self.src_addr[3]
        )
    }
}

// ─── XDP Packet ──────────────────────────────────────────────────────────────

/// An XDP packet with metadata
#[derive(Debug, Clone)]
pub struct XdpPacket {
    /// Raw packet bytes (starting from Ethernet header)
    pub data: Vec<u8>,
    /// RX queue index
    pub rx_queue: u32,
    /// Ingress interface index
    pub ingress_ifindex: u32,
    /// Hardware timestamp (nanoseconds)
    pub timestamp_ns: u64,
}

impl XdpPacket {
    pub fn new(data: Vec<u8>, ingress_ifindex: u32) -> Self {
        XdpPacket {
            data,
            rx_queue: 0,
            ingress_ifindex,
            timestamp_ns: 0,
        }
    }

    /// Parse Ethernet header from packet data
    pub fn eth_header(&self) -> Option<EthernetHeader> {
        EthernetHeader::parse(&self.data)
    }

    /// Parse IPv4 header from an untagged Ethernet II IPv4 frame
    pub fn ipv4_header(&self) -> Option<Ipv4Header> {
        if !self.eth_header()?.is_ipv4() {
            return None;
        }
        Ipv4Header::parse(&self.data[14..])
    }

    // Ports must be inside the IPv4 total length, not just Ethernet padding.
    // Later fragments carry payload bytes in place of the transport header.
    fn transport_ports(&self) -> Option<(u16, u16)> {
        let ip = self.ipv4_header()?;
        if ip.protocol != 6 && ip.protocol != 17 {
            return None;
        }
        let fragment = u16::from_be_bytes([self.data[20], self.data[21]]);
        if fragment & 0x1FFF != 0 {
            return None;
        }
        let ihl = ((self.data[14] & 0x0F) as usize) * 4;
        if (ip.total_len as usize) < ihl + 4 {
            return None;
        }
        let port_off = 14 + ihl;
        let src = u16::from_be_bytes([self.data[port_off], self.data[port_off + 1]]);
        let dst = u16::from_be_bytes([self.data[port_off + 2], self.data[port_off + 3]]);
        Some((src, dst))
    }

    /// Returns TCP/UDP source port from a first IPv4 fragment
    pub fn src_port(&self) -> Option<u16> {
        self.transport_ports().map(|(src, _)| src)
    }

    /// Returns TCP/UDP destination port from a first IPv4 fragment
    pub fn dst_port(&self) -> Option<u16> {
        self.transport_ports().map(|(_, dst)| dst)
    }
}

// ─── XDP Statistics ───────────────────────────────────────────────────────────

/// Per-interface XDP packet statistics
#[derive(Debug, Clone, Default)]
pub struct XdpIfaceStats {
    pub passed: u64,
    pub dropped: u64,
    pub redirected: u64,
    pub tx_looped: u64,
    pub aborted: u64,
    pub bytes_passed: u64,
    pub bytes_dropped: u64,
}

impl XdpIfaceStats {
    pub fn total_packets(&self) -> u64 {
        self.passed + self.dropped + self.redirected + self.tx_looped + self.aborted
    }

    pub fn drop_rate(&self) -> f64 {
        let total = self.total_packets();
        if total == 0 {
            0.0
        } else {
            self.dropped as f64 / total as f64
        }
    }
}

// ─── XDP Filter Rule ─────────────────────────────────────────────────────────

/// An XDP filter rule (matches Ethernet/IP/port fields → action)
#[derive(Debug, Clone)]
pub struct XdpFilterRule {
    /// Rule priority (lower = evaluated first)
    pub priority: u32,
    /// Optional source IPv4 address to match (None = any)
    pub src_ip: Option<[u8; 4]>,
    /// Optional destination port to match (None = any)
    pub dst_port: Option<u16>,
    /// Optional EtherType to match (None = any)
    pub ethertype: Option<u16>,
    /// Action to take when rule matches
    pub action: XdpAction,
    /// Human-readable rule label
    pub label: String,
}

impl XdpFilterRule {
    /// Check if this rule matches the given packet
    pub fn matches(&self, pkt: &XdpPacket) -> bool {
        // Check EtherType
        if let Some(eth_type) = self.ethertype {
            match pkt.eth_header() {
                Some(eth) if eth.ethertype == eth_type => {}
                _ => return false,
            }
        }
        // Check source IP
        if let Some(src_ip) = self.src_ip {
            match pkt.ipv4_header() {
                Some(ip) if ip.src_addr == src_ip => {}
                _ => return false,
            }
        }
        // Check destination port
        if let Some(port) = self.dst_port {
            match pkt.dst_port() {
                Some(p) if p == port => {}
                _ => return false,
            }
        }
        true
    }
}

// ─── XDP Engine ──────────────────────────────────────────────────────────────

/// SigmaOS XDP Engine — zero-copy packet processor
///
/// Manages filter rules, sockmap redirect table, and per-interface statistics.
pub struct XdpEngine {
    /// Sockmap: source interface index → redirect target fd/interface
    pub sockmap: BTreeMap<u32, u32>,
    /// Filter rules sorted by priority
    pub rules: Vec<XdpFilterRule>,
    /// Per-interface statistics
    pub iface_stats: BTreeMap<u32, XdpIfaceStats>,
    /// DEVMAP: ifindex → redirect target ifindex
    pub devmap: BTreeMap<u32, u32>,
    /// Whether promiscuous mode is enabled (receive all packets)
    pub promiscuous: bool,
}

impl XdpEngine {
    /// Create a new XDP engine
    pub fn new() -> Self {
        XdpEngine {
            sockmap: BTreeMap::new(),
            rules: Vec::new(),
            iface_stats: BTreeMap::new(),
            devmap: BTreeMap::new(),
            promiscuous: false,
        }
    }

    // ── Rule Management ───────────────────────────────────────────────────────

    /// Add a filter rule. Rules are evaluated in priority order (lowest first).
    pub fn add_rule(&mut self, rule: XdpFilterRule) {
        self.rules.push(rule);
        // Sort by priority ascending
        self.rules.sort_by_key(|r| r.priority);
    }

    /// Remove rules matching a label
    pub fn remove_rules_by_label(&mut self, label: &str) {
        self.rules.retain(|r| r.label != label);
    }

    // ── Sockmap ───────────────────────────────────────────────────────────────

    /// Add a sockmap entry: packets from src_ifindex are redirected to dst_ifindex
    pub fn add_sockmap_entry(&mut self, src_ifindex: u32, dst_ifindex: u32) {
        self.sockmap.insert(src_ifindex, dst_ifindex);
    }

    /// Add a DEVMAP entry for hardware-level redirect
    pub fn add_devmap_entry(&mut self, src_ifindex: u32, dst_ifindex: u32) {
        self.devmap.insert(src_ifindex, dst_ifindex);
    }

    // ── Packet Processing ─────────────────────────────────────────────────────

    /// Process a single packet through the XDP filter pipeline.
    ///
    /// Evaluates filter rules in priority order. If no rule matches,
    /// checks sockmap for redirect, otherwise returns Pass.
    ///
    /// This is the hot path — must be sub-microsecond.
    pub fn process_packet(&mut self, pkt: &XdpPacket) -> XdpAction {
        let iface = pkt.ingress_ifindex;
        let pkt_len = pkt.data.len() as u64;

        // Evaluate filter rules (O(N) scan — in production, use BPF maps)
        for rule in &self.rules {
            if rule.matches(pkt) {
                let action = rule.action;
                self.record_action(iface, action, pkt_len);
                return action;
            }
        }

        // Check sockmap redirect
        if let Some(&dst) = self.sockmap.get(&iface) {
            self.record_action(iface, XdpAction::Redirect(dst), pkt_len);
            return XdpAction::Redirect(dst);
        }

        // Default: pass to kernel network stack
        self.record_action(iface, XdpAction::Pass, pkt_len);
        XdpAction::Pass
    }

    /// Process a batch of packets — optimized for cache efficiency
    pub fn process_batch(&mut self, pkts: &[XdpPacket]) -> Vec<XdpAction> {
        pkts.iter().map(|pkt| self.process_packet(pkt)).collect()
    }

    /// Record an action in per-interface statistics
    fn record_action(&mut self, iface: u32, action: XdpAction, pkt_len: u64) {
        let stats = self
            .iface_stats
            .entry(iface)
            .or_insert_with(XdpIfaceStats::default);
        match action {
            XdpAction::Pass => {
                stats.passed += 1;
                stats.bytes_passed += pkt_len;
            }
            XdpAction::Drop => {
                stats.dropped += 1;
                stats.bytes_dropped += pkt_len;
            }
            XdpAction::Redirect(_) => stats.redirected += 1,
            XdpAction::Tx => stats.tx_looped += 1,
            XdpAction::Aborted => stats.aborted += 1,
        }
    }

    // ── Statistics ────────────────────────────────────────────────────────────

    /// Returns stats for a specific interface
    pub fn stats(&self, ifindex: u32) -> Option<&XdpIfaceStats> {
        self.iface_stats.get(&ifindex)
    }

    /// Returns total packets processed across all interfaces
    pub fn total_packets(&self) -> u64 {
        self.iface_stats.values().map(|s| s.total_packets()).sum()
    }

    /// Returns aggregate stats summary string
    pub fn stats_summary(&self) -> String {
        let total = self.total_packets();
        let dropped: u64 = self.iface_stats.values().map(|s| s.dropped).sum();
        let passed: u64 = self.iface_stats.values().map(|s| s.passed).sum();
        let redirected: u64 = self.iface_stats.values().map(|s| s.redirected).sum();
        format!(
            "XDP Engine | {} ifaces | {} total pkts | {} passed | {} dropped | {} redirected | {} rules",
            self.iface_stats.len(), total, passed, dropped, redirected, self.rules.len()
        )
    }
}

// ─── Convenience Builders ─────────────────────────────────────────────────────

impl XdpFilterRule {
    /// Create a DROP rule for a specific source IP (DDoS mitigation)
    pub fn block_src_ip(priority: u32, src_ip: [u8; 4]) -> Self {
        XdpFilterRule {
            priority,
            src_ip: Some(src_ip),
            dst_port: None,
            ethertype: None,
            action: XdpAction::Drop,
            label: format!(
                "block-{}.{}.{}.{}",
                src_ip[0], src_ip[1], src_ip[2], src_ip[3]
            ),
        }
    }

    /// Create a PASS rule for a specific destination port
    pub fn allow_dst_port(priority: u32, port: u16) -> Self {
        XdpFilterRule {
            priority,
            src_ip: None,
            dst_port: Some(port),
            ethertype: Some(ETHERTYPE_IPV4),
            action: XdpAction::Pass,
            label: format!("allow-port-{}", port),
        }
    }

    /// Create a DROP rule for all ARP packets (anti-spoofing)
    pub fn drop_arp(priority: u32) -> Self {
        XdpFilterRule {
            priority,
            src_ip: None,
            dst_port: None,
            ethertype: Some(ETHERTYPE_ARP),
            action: XdpAction::Drop,
            label: String::from("drop-arp"),
        }
    }
}

// ─── Helper: Build Test Ethernet+IPv4+UDP Packet ─────────────────────────────

/// Build a minimal Ethernet+IPv4+UDP packet for testing
pub fn build_test_packet(
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
    src_port: u16,
    dst_port: u16,
) -> XdpPacket {
    let mut data = Vec::with_capacity(42);
    // Ethernet header (14 bytes)
    data.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]); // dst MAC broadcast
    data.extend_from_slice(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]); // src MAC
    data.extend_from_slice(&[0x08, 0x00]); // EtherType = IPv4
                                           // IPv4 header (20 bytes)
    data.push(0x45); // version=4, IHL=5
    data.push(0x00); // DSCP
    data.extend_from_slice(&[0x00, 0x1C]); // total length = 28
    data.extend_from_slice(&[0x00, 0x01]); // identification
    data.extend_from_slice(&[0x00, 0x00]); // flags + fragment offset
    data.push(64); // TTL
    data.push(17); // protocol = UDP
    data.extend_from_slice(&[0x00, 0x00]); // checksum (0 for test)
    data.extend_from_slice(&src_ip);
    data.extend_from_slice(&dst_ip);
    // UDP header (8 bytes)
    data.extend_from_slice(&[(src_port >> 8) as u8, src_port as u8]);
    data.extend_from_slice(&[(dst_port >> 8) as u8, dst_port as u8]);
    data.extend_from_slice(&[0x00, 0x08]); // UDP length
    data.extend_from_slice(&[0x00, 0x00]); // UDP checksum
    XdpPacket::new(data, 1)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod xdp_tests {
    use super::*;

    #[test]
    fn test_ethernet_parse() {
        let pkt = build_test_packet([10, 0, 0, 1], [10, 0, 0, 2], 12345, 80);
        let eth = pkt.eth_header().expect("should parse eth header");
        assert_eq!(eth.ethertype, ETHERTYPE_IPV4);
        assert!(eth.is_ipv4());
        assert!(!eth.is_ipv6());
    }

    #[test]
    fn test_ipv4_parse() {
        let pkt = build_test_packet([192, 168, 1, 1], [10, 0, 0, 1], 1234, 443);
        let ip = pkt.ipv4_header().expect("should parse ipv4 header");
        assert_eq!(ip.src_addr, [192, 168, 1, 1]);
        assert_eq!(ip.dst_addr, [10, 0, 0, 1]);
        assert_eq!(ip.protocol, 17); // UDP
        assert_eq!(ip.ttl, 64);
    }

    #[test]
    fn test_port_extraction() {
        let pkt = build_test_packet([1, 2, 3, 4], [5, 6, 7, 8], 54321, 8080);
        assert_eq!(pkt.src_port(), Some(54321));
        assert_eq!(pkt.dst_port(), Some(8080));
    }

    #[test]
    fn test_ipv4_validation_and_transport_port_bounds() {
        // Each case starts with a valid Ethernet/IPv4/UDP frame. The last two
        // booleans indicate whether packet-level IPv4 and port parsing succeed.
        let cases: &[(&str, fn(&mut Vec<u8>), bool, bool)] = &[
            ("valid UDP", |_| {}, true, true),
            ("valid TCP", |d| d[23] = 6, true, true),
            ("not a transport protocol", |d| d[23] = 1, true, false),
            (
                "ARP EtherType",
                |d| d[12..14].copy_from_slice(&[0x08, 0x06]),
                false,
                false,
            ),
            (
                "IPv6 EtherType",
                |d| d[12..14].copy_from_slice(&[0x86, 0xdd]),
                false,
                false,
            ),
            (
                "VLAN EtherType",
                |d| d[12..14].copy_from_slice(&[0x81, 0x00]),
                false,
                false,
            ),
            ("short Ethernet header", |d| d.truncate(13), false, false),
            ("short IPv4 header", |d| d.truncate(33), false, false),
            ("wrong IP version", |d| d[14] = 0x65, false, false),
            ("IHL below minimum", |d| d[14] = 0x44, false, false),
            ("IHL exceeds frame", |d| d[14] = 0x4f, false, false),
            (
                "total length below IHL",
                |d| d[16..18].copy_from_slice(&19u16.to_be_bytes()),
                false,
                false,
            ),
            (
                "options exceed total length",
                |d| {
                    d[14] = 0x46;
                    d[16..18].copy_from_slice(&20u16.to_be_bytes());
                },
                false,
                false,
            ),
            (
                "total length exceeds frame",
                |d| d[16..18].copy_from_slice(&43u16.to_be_bytes()),
                false,
                false,
            ),
            (
                "frame truncated below total length",
                |d| d.truncate(41),
                false,
                false,
            ),
            (
                "valid IP options",
                |d| {
                    d[14] = 0x46;
                    d[16..18].copy_from_slice(&32u16.to_be_bytes());
                    d.splice(34..34, [0, 0, 0, 0]);
                },
                true,
                true,
            ),
            (
                "IP payload is only padding",
                |d| d[16..18].copy_from_slice(&20u16.to_be_bytes()),
                true,
                false,
            ),
            (
                "partial port pair in padding",
                |d| d[16..18].copy_from_slice(&22u16.to_be_bytes()),
                true,
                false,
            ),
            (
                "four real port bytes",
                |d| d[16..18].copy_from_slice(&24u16.to_be_bytes()),
                true,
                true,
            ),
            ("non-first fragment", |d| d[21] = 1, true, false),
            (
                "non-first fragment with MF",
                |d| {
                    d[20] = 0x20;
                    d[21] = 1;
                },
                true,
                false,
            ),
            ("first fragment with MF", |d| d[20] = 0x20, true, true),
            ("don't fragment flag", |d| d[20] = 0x40, true, true),
        ];
        for &(name, modify, has_ip, has_ports) in cases {
            let mut pkt = build_test_packet([1, 2, 3, 4], [5, 6, 7, 8], 54321, 8080);
            modify(&mut pkt.data);
            assert_eq!(pkt.ipv4_header().is_some(), has_ip, "{name}: IPv4");
            assert_eq!(
                pkt.src_port(),
                has_ports.then_some(54321),
                "{name}: source port"
            );
            assert_eq!(
                pkt.dst_port(),
                has_ports.then_some(8080),
                "{name}: destination port"
            );
            assert_eq!(
                XdpFilterRule::block_src_ip(1, [1, 2, 3, 4]).matches(&pkt),
                has_ip,
                "{name}: IP rule"
            );
            assert_eq!(
                XdpFilterRule::allow_dst_port(1, 8080).matches(&pkt),
                has_ports,
                "{name}: port rule"
            );
        }
    }

    #[test]
    fn test_default_action_is_pass() {
        let mut engine = XdpEngine::new();
        let pkt = build_test_packet([10, 0, 0, 1], [10, 0, 0, 2], 1111, 80);
        assert_eq!(engine.process_packet(&pkt), XdpAction::Pass);
        assert_eq!(engine.total_packets(), 1);
    }

    #[test]
    fn test_drop_rule_blocks_src_ip() {
        let mut engine = XdpEngine::new();
        engine.add_rule(XdpFilterRule::block_src_ip(10, [1, 2, 3, 4]));
        let pkt = build_test_packet([1, 2, 3, 4], [10, 0, 0, 1], 9999, 80);
        assert_eq!(engine.process_packet(&pkt), XdpAction::Drop);
        let stats = engine.stats(1).unwrap();
        assert_eq!(stats.dropped, 1);
    }

    #[test]
    fn test_allow_rule_passes() {
        let mut engine = XdpEngine::new();
        engine.add_rule(XdpFilterRule::allow_dst_port(5, 443));
        let pkt = build_test_packet([1, 2, 3, 4], [10, 0, 0, 1], 5000, 443);
        assert_eq!(engine.process_packet(&pkt), XdpAction::Pass);
    }

    #[test]
    fn test_sockmap_redirect() {
        let mut engine = XdpEngine::new();
        engine.add_sockmap_entry(1, 2); // redirect from iface 1 → iface 2
        let pkt = build_test_packet([1, 2, 3, 4], [5, 6, 7, 8], 100, 200);
        let action = engine.process_packet(&pkt);
        assert_eq!(action, XdpAction::Redirect(2));
    }

    #[test]
    fn test_rule_priority_ordering() {
        let mut engine = XdpEngine::new();
        // Lower priority (10) PASS rule and higher priority (1) DROP rule
        engine.add_rule(XdpFilterRule::allow_dst_port(10, 80));
        engine.add_rule(XdpFilterRule::block_src_ip(1, [1, 2, 3, 4]));
        // Packet from blocked IP to port 80 — DROP should win (priority 1 < 10)
        let pkt = build_test_packet([1, 2, 3, 4], [10, 0, 0, 1], 5000, 80);
        assert_eq!(engine.process_packet(&pkt), XdpAction::Drop);
    }

    #[test]
    fn test_batch_processing() {
        let mut engine = XdpEngine::new();
        engine.add_rule(XdpFilterRule::block_src_ip(1, [10, 0, 0, 1]));
        let pkts = vec![
            build_test_packet([10, 0, 0, 1], [10, 0, 0, 2], 100, 80), // DROP
            build_test_packet([192, 168, 0, 1], [10, 0, 0, 2], 200, 80), // PASS
            build_test_packet([10, 0, 0, 1], [10, 0, 0, 3], 300, 443), // DROP
        ];
        let actions = engine.process_batch(&pkts);
        assert_eq!(actions[0], XdpAction::Drop);
        assert_eq!(actions[1], XdpAction::Pass);
        assert_eq!(actions[2], XdpAction::Drop);
    }

    #[test]
    fn test_stats_summary() {
        let mut engine = XdpEngine::new();
        let pkt = build_test_packet([1, 2, 3, 4], [5, 6, 7, 8], 100, 200);
        engine.process_packet(&pkt);
        let summary = engine.stats_summary();
        assert!(summary.contains("XDP Engine"));
        assert!(summary.contains("1 total pkts"));
    }

    #[test]
    fn test_drop_rate() {
        let mut engine = XdpEngine::new();
        engine.add_rule(XdpFilterRule::block_src_ip(1, [1, 2, 3, 4]));
        let pkts = vec![
            build_test_packet([1, 2, 3, 4], [10, 0, 0, 1], 1, 80), // DROP
            build_test_packet([1, 2, 3, 4], [10, 0, 0, 1], 2, 80), // DROP
            build_test_packet([9, 9, 9, 9], [10, 0, 0, 1], 3, 80), // PASS
            build_test_packet([9, 9, 9, 9], [10, 0, 0, 1], 4, 80), // PASS
        ];
        engine.process_batch(&pkts);
        let stats = engine.stats(1).unwrap();
        assert!((stats.drop_rate() - 0.5).abs() < 1e-9);
    }
}
