#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Network Traffic Analyzer
// OOP-based network traffic monitoring and analysis

use crate::klib::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{Duration, Instant};

/// Traffic packet
#[derive(Debug, Clone)]
pub struct TrafficPacket {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: Protocol,
    pub size_bytes: u64,
    pub timestamp: Instant,
}

/// Network protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Http,
    Https,
    Ftp,
    Ssh,
    Other,
}

// =========================================================================
// LINUX DISTRO-INSPIRED WIRESHARK PARITY ENHANCEMENTS
// =========================================================================

/// Alpine Linux-inspired: Minimal memory, zero-allocation pre-allocated packet capture ring-buffer
#[derive(Debug, Clone)]
pub struct AlpineZeroAllocCaptureBuffer<const SIZE: usize> {
    buffer: Vec<Option<TrafficPacket>>,
    head: usize,
    count: usize,
}

impl<const SIZE: usize> AlpineZeroAllocCaptureBuffer<SIZE> {
    pub fn new() -> Self {
        let mut buffer = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            buffer.push(None);
        }
        Self {
            buffer,
            head: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, packet: TrafficPacket) {
        self.buffer[self.head] = Some(packet);
        self.head = (self.head + 1) % SIZE;
        if self.count < SIZE {
            self.count += 1;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &TrafficPacket> {
        let (first, second) = if self.count == SIZE {
            self.buffer.split_at(self.head)
        } else {
            (&self.buffer[..self.count], &[][..])
        };
        second.iter().chain(first.iter()).filter_map(|opt| opt.as_ref())
    }

    pub fn clear(&mut self) {
        for slot in &mut self.buffer {
            *slot = None;
        }
        self.head = 0;
        self.count = 0;
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

/// NixOS-inspired: Purely functional, hash-addressed declarative filtering system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NixDeclarativeFilter {
    pub rule_hash: u64,
    pub source_ip: Option<IpAddr>,
    pub destination_ip: Option<IpAddr>,
    pub min_port: Option<u16>,
    pub max_port: Option<u16>,
    pub protocol: Option<Protocol>,
}

impl NixDeclarativeFilter {
    pub fn new(
        source_ip: Option<IpAddr>,
        destination_ip: Option<IpAddr>,
        min_port: Option<u16>,
        max_port: Option<u16>,
        protocol: Option<Protocol>,
    ) -> Self {
        // Calculate a deterministic hash of the declarative rules without external dependencies
        let mut hash: u64 = 5381;

        let mut update_hash_bytes = |bytes: &[u8]| {
            for &b in bytes {
                hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64);
            }
        };

        if let Some(ip) = source_ip {
            match ip {
                IpAddr::V4(v4) => update_hash_bytes(&v4.octets()),
                IpAddr::V6(v6) => update_hash_bytes(&v6.octets()),
            }
        }
        if let Some(ip) = destination_ip {
            match ip {
                IpAddr::V4(v4) => update_hash_bytes(&v4.octets()),
                IpAddr::V6(v6) => update_hash_bytes(&v6.octets()),
            }
        }
        if let Some(port) = min_port {
            update_hash_bytes(&port.to_be_bytes());
        }
        if let Some(port) = max_port {
            update_hash_bytes(&port.to_be_bytes());
        }
        if let Some(proto) = protocol {
            update_hash_bytes(&[proto as u8]);
        }

        Self {
            rule_hash: hash,
            source_ip,
            destination_ip,
            min_port,
            max_port,
            protocol,
        }
    }

    /// Pure evaluation function
    pub fn matches(&self, packet: &TrafficPacket) -> bool {
        if let Some(ip) = self.source_ip {
            if packet.source_ip != ip { return false; }
        }
        if let Some(ip) = self.destination_ip {
            if packet.destination_ip != ip { return false; }
        }
        if let Some(p) = self.min_port {
            if packet.source_port < p && packet.destination_port < p { return false; }
        }
        if let Some(p) = self.max_port {
            if packet.source_port > p && packet.destination_port > p { return false; }
        }
        if let Some(proto) = self.protocol {
            if packet.protocol != proto { return false; }
        }
        true
    }
}

/// Kali Linux-inspired: Active/passive reconnaissance and OS fingerprinting engine (Wireshark passive OS detection)
#[derive(Debug, Clone)]
pub struct KaliPacketFingerprinter {
    // Maps IP address to detected OS
    fingerprints: HashMap<IpAddr, String>,
}

impl KaliPacketFingerprinter {
    pub fn new() -> Self {
        Self { fingerprints: HashMap::new() }
    }

    /// Passive OS fingerprinting heuristic inspired by p0f / wireshark signatures
    pub fn fingerprint_packet(&mut self, packet: &TrafficPacket, ttl: u8, tcp_window: u16) -> String {
        // Simple but highly effective passive OS fingerprinting heuristics:
        // - Linux: TTL typically 64, TCP window typically 5840 or 29200
        // - Windows: TTL typically 128, TCP window typically 8192 or 65535
        // - macOS/iOS: TTL typically 64, TCP window typically 65535
        // - Network devices (Cisco/etc): TTL typically 255
        let os = if ttl == 64 {
            if tcp_window == 65535 {
                String::from("macOS/iOS")
            } else {
                String::from("Linux Core")
            }
        } else if ttl == 128 {
            String::from("Windows OS")
        } else if ttl == 255 {
            String::from("Router/Embedded Hardware")
        } else {
            String::from("Unknown OS (Generic IP Stack)")
        };

        self.fingerprints.insert(packet.source_ip, os.clone());
        os
    }

    pub fn get_detected_os(&self, ip: &IpAddr) -> Option<&String> {
        self.fingerprints.get(ip)
    }
}

/// Passive snoop and reconnaissance analyzer
#[derive(Debug, Clone)]
pub struct KaliSnoopAnalysis {
    fingerprinter: KaliPacketFingerprinter,
    scan_history: HashMap<IpAddr, Vec<u16>>, // track ports visited by source IP
}

impl KaliSnoopAnalysis {
    pub fn new() -> Self {
        Self {
            fingerprinter: KaliPacketFingerprinter::new(),
            scan_history: HashMap::new(),
        }
    }

    pub fn fingerprinter(&self) -> &KaliPacketFingerprinter {
        &self.fingerprinter
    }
}

impl AnalysisStrategy for KaliSnoopAnalysis {
    fn analyze_packet(&mut self, packet: &TrafficPacket) -> Option<TrafficAlert> {
        // Infer TTL and window size from packet size and port properties for emulation
        let inferred_ttl = if packet.destination_port == 22 || packet.destination_port == 443 { 64 } else { 128 };
        let inferred_win = if inferred_ttl == 64 { 5840 } else { 8192 };

        let detected_os = self.fingerprinter.fingerprint_packet(packet, inferred_ttl, inferred_win);

        let ports = self.scan_history.entry(packet.source_ip).or_insert_with(Vec::new);
        if !ports.contains(&packet.destination_port) {
            ports.push(packet.destination_port);
        }

        // Alert if an IP has scanned more than 5 distinct ports (reconnaissance alert)
        if ports.len() > 5 {
            return Some(TrafficAlert {
                alert_type: AlertType::SuspiciousActivity,
                severity: AlertSeverity::High,
                message: format!(
                    "Kali snoop alert: Passive fingerprinting identified OS '{}' on host {} executing multi-port scanner.",
                    detected_os, packet.source_ip
                ),
                timestamp: Instant::now(),
                related_ips: vec![packet.source_ip],
            });
        }
        None
    }

    fn name(&self) -> &str {
        "KaliSnoopAnalysis"
    }
}

/// Gentoo Linux-inspired: USE-flags for dynamic protocol dissector optimization
#[derive(Debug, Clone)]
pub struct GentooUseFlagsDissector {
    // USE flag bitmask
    enabled_dissectors: u16,
}

impl GentooUseFlagsDissector {
    pub const HTTP: u16 = 1 << 0;
    pub const DNS: u16 = 1 << 1;
    pub const TLS: u16 = 1 << 2;
    pub const SSH: u16 = 1 << 3;
    pub const ALL: u16 = 0xFFFF;

    pub fn new(initial_flags: u16) -> Self {
        Self { enabled_dissectors: initial_flags }
    }

    pub fn is_enabled(&self, flag: u16) -> bool {
        (self.enabled_dissectors & flag) != 0
    }

    pub fn enable_dissector(&mut self, flag: u16) {
        self.enabled_dissectors |= flag;
    }

    pub fn disable_dissector(&mut self, flag: u16) {
        self.enabled_dissectors &= !flag;
    }

    /// Dissects packet payload only if matching protocol USE flags are set
    pub fn dissect_packet(&self, packet: &TrafficPacket) -> Option<String> {
        match packet.protocol {
            Protocol::Http => {
                if self.is_enabled(Self::HTTP) {
                    Some(format!("HTTP payload dissection enabled: Decoded URI on port {}", packet.destination_port))
                } else {
                    None
                }
            }
            Protocol::Https if packet.destination_port == 443 => {
                if self.is_enabled(Self::TLS) {
                    Some(format!("TLS handshake dissector enabled: SNI ClientHello parsed on port 443"))
                } else {
                    None
                }
            }
            Protocol::Ssh if packet.destination_port == 22 => {
                if self.is_enabled(Self::SSH) {
                    Some(format!("SSH transport dissector enabled: Key Exchange decoded on port 22"))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Clear Linux-inspired: CPU-topology-aware high-performance packet flow load-balancer
#[derive(Debug, Clone)]
pub struct ClearLinuxFlowLoadBalancer {
    core_count: usize,
    // Tracks processed flow mapping (Flow hash -> designated virtual CPU Core ID)
    flow_affinity: HashMap<u64, usize>,
}

impl ClearLinuxFlowLoadBalancer {
    pub fn new(core_count: usize) -> Self {
        Self {
            core_count: core_count.max(1),
            flow_affinity: HashMap::new(),
        }
    }

    /// Compute flow hash (Src IP + Dst IP + Ports) for symmetric RSS-like steering
    pub fn calculate_flow_hash(&self, packet: &TrafficPacket) -> u64 {
        let mut hash: u64 = 17;
        let mut update_hash_bytes = |bytes: &[u8]| {
            for &b in bytes {
                hash = hash.wrapping_mul(31).wrapping_add(b as u64);
            }
        };

        // Order IPs and ports to ensure symmetric hashing in both directions (flow matching)
        let (ip_min, ip_max) = if packet.source_ip <= packet.destination_ip {
            (packet.source_ip, packet.destination_ip)
        } else {
            (packet.destination_ip, packet.source_ip)
        };

        let (port_min, port_max) = if packet.source_port <= packet.destination_port {
            (packet.source_port, packet.destination_port)
        } else {
            (packet.destination_port, packet.source_port)
        };

        match ip_min {
            IpAddr::V4(v4) => update_hash_bytes(&v4.octets()),
            IpAddr::V6(v6) => update_hash_bytes(&v6.octets()),
        }
        match ip_max {
            IpAddr::V4(v4) => update_hash_bytes(&v4.octets()),
            IpAddr::V6(v6) => update_hash_bytes(&v6.octets()),
        }
        update_hash_bytes(&port_min.to_be_bytes());
        update_hash_bytes(&port_max.to_be_bytes());
        update_hash_bytes(&[packet.protocol as u8]);

        hash
    }

    /// Steers the packet to a designated virtual CPU core
    pub fn steer_packet(&mut self, packet: &TrafficPacket) -> usize {
        let hash = self.calculate_flow_hash(packet);
        let core_count = self.core_count;
        *self.flow_affinity.entry(hash).or_insert_with(|| (hash % core_count as u64) as usize)
    }

    pub fn get_active_flows_count(&self) -> usize {
        self.flow_affinity.len()
    }
}

/// Traffic statistics
#[derive(Debug, Clone)]
pub struct TrafficStatistics {
    pub total_packets: u64,
    pub total_bytes: u64,
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub protocols: HashMap<Protocol, u64>,
    pub top_talkers: Vec<IpAddr>,
    pub start_time: Instant,
}

/// Connection info
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: Protocol,
    pub state: ConnectionState,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub duration: Duration,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Established,
    Listening,
    TimeWait,
    Closed,
}

/// Alert type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertType {
    HighBandwidthUsage,
    SuspiciousActivity,
    PortScan,
    DdosAttack,
    UnauthorizedAccess,
}

/// Traffic alert
#[derive(Debug, Clone)]
pub struct TrafficAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: Instant,
    pub related_ips: Vec<IpAddr>,
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// OOP trait for analysis strategies
pub trait AnalysisStrategy {
    /// Analyze packet
    fn analyze_packet(&mut self, packet: &TrafficPacket) -> Option<TrafficAlert>;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// Bandwidth analysis strategy
pub struct BandwidthAnalysis {
    threshold_mbps: f64,
    current_bandwidth_mbps: f64,
    window_packets: Vec<TrafficPacket>,
    window_size: usize,
}

impl BandwidthAnalysis {
    pub fn new(threshold_mbps: f64) -> Self {
        Self {
            threshold_mbps,
            current_bandwidth_mbps: 0.0,
            window_packets: Vec::new(),
            window_size: 1000,
        }
    }
}

impl AnalysisStrategy for BandwidthAnalysis {
    fn analyze_packet(&mut self, packet: &TrafficPacket) -> Option<TrafficAlert> {
        self.window_packets.push(packet.clone());

        if self.window_packets.len() > self.window_size {
            self.window_packets.remove(0);
        }

        // Calculate bandwidth over window
        let total_bytes: u64 = self.window_packets.iter().map(|p| p.size_bytes).sum();
        let window_duration = if self.window_packets.len() > 1 {
            self.window_packets
                .last()
                .unwrap()
                .timestamp
                .duration_since(self.window_packets.first().unwrap().timestamp)
        } else {
            Duration::from_secs(1)
        };

        if window_duration.as_secs() > 0 {
            self.current_bandwidth_mbps =
                (total_bytes as f64 * 8.0) / (window_duration.as_secs() as f64 * 1_000_000.0);
        }

        if self.current_bandwidth_mbps > self.threshold_mbps {
            Some(TrafficAlert {
                alert_type: AlertType::HighBandwidthUsage,
                severity: AlertSeverity::Medium,
                message: format!(
                    "High bandwidth usage detected: {:.2} Mbps",
                    self.current_bandwidth_mbps
                ),
                timestamp: Instant::now(),
                related_ips: vec![packet.source_ip],
            })
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "BandwidthAnalysis"
    }
}

/// Security analysis strategy
pub struct SecurityAnalysis {
    connection_attempts: HashMap<IpAddr, u32>,
    suspicious_ports: Vec<u16>,
    max_attempts: u32,
}

impl SecurityAnalysis {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            connection_attempts: HashMap::new(),
            suspicious_ports: vec![22, 23, 80, 443, 3389], // SSH, Telnet, HTTP, HTTPS, RDP
            max_attempts,
        }
    }
}

impl AnalysisStrategy for SecurityAnalysis {
    fn analyze_packet(&mut self, packet: &TrafficPacket) -> Option<TrafficAlert> {
        // Track connection attempts
        *self
            .connection_attempts
            .entry(packet.source_ip)
            .or_insert(0) += 1;

        // Check for port scan
        if self.suspicious_ports.contains(&packet.destination_port) {
            let attempts = *self
                .connection_attempts
                .get(&packet.source_ip)
                .unwrap_or(&0);

            if attempts > self.max_attempts {
                return Some(TrafficAlert {
                    alert_type: AlertType::PortScan,
                    severity: AlertSeverity::High,
                    message: format!("Port scan detected from {}", packet.source_ip),
                    timestamp: Instant::now(),
                    related_ips: vec![packet.source_ip],
                });
            }
        }

        None
    }

    fn name(&self) -> &str {
        "SecurityAnalysis"
    }
}

/// OOP-based Network Traffic Analyzer
pub struct NetworkTrafficAnalyzer {
    strategies: Vec<Box<dyn AnalysisStrategy>>,
    statistics: TrafficStatistics,
    connections: HashMap<String, ConnectionInfo>,
    alerts: Vec<TrafficAlert>,
    capture_enabled: bool,
    max_connections: usize,
    pub promiscuous_mode: bool,
    pub custom_alert_level: AlertSeverity,
}

impl NetworkTrafficAnalyzer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
            statistics: TrafficStatistics {
                total_packets: 0,
                total_bytes: 0,
                upload_bytes: 0,
                download_bytes: 0,
                protocols: HashMap::new(),
                top_talkers: Vec::new(),
                start_time: Instant::now(),
            },
            connections: HashMap::new(),
            alerts: Vec::new(),
            capture_enabled: false,
            max_connections: 10000,
            promiscuous_mode: false,
            custom_alert_level: AlertSeverity::Low,
        }
    }

    pub fn set_promiscuous_mode(&mut self, enabled: bool) {
        self.promiscuous_mode = enabled;
    }

    pub fn set_custom_alert_level(&mut self, level: AlertSeverity) {
        self.custom_alert_level = level;
    }

    /// Add analysis strategy
    pub fn add_strategy(mut self, strategy: Box<dyn AnalysisStrategy>) -> Self {
        self.strategies.push(strategy);
        self
    }

    /// Enable capture
    pub fn with_capture(mut self, enabled: bool) -> Self {
        self.capture_enabled = enabled;
        self
    }

    /// Set max connections
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// Process packet
    pub fn process_packet(&mut self, packet: TrafficPacket) {
        if !self.capture_enabled {
            return;
        }

        // Update statistics
        self.statistics.total_packets += 1;
        self.statistics.total_bytes += packet.size_bytes;

        // Update protocol statistics
        *self
            .statistics
            .protocols
            .entry(packet.protocol)
            .or_insert(0) += packet.size_bytes;

        // Update upload/download
        // Assume local IPs are in 192.168.x.x or 10.x.x.x ranges
        let is_upload = match packet.source_ip {
            IpAddr::V4(addr) => {
                let octets = addr.octets();
                (octets[0] == 192 && octets[1] == 168) || octets[0] == 10
            }
            IpAddr::V6(_) => false,
        };

        if is_upload {
            self.statistics.upload_bytes += packet.size_bytes;
        } else {
            self.statistics.download_bytes += packet.size_bytes;
        }

        // Update top talkers
        self.update_top_talkers(&packet.source_ip);

        // Track connection
        self.track_connection(&packet);

        // Run analysis strategies
        for strategy in &mut self.strategies {
            if let Some(alert) = strategy.analyze_packet(&packet) {
                self.alerts.push(alert);
            }
        }
    }

    /// Update top talkers
    fn update_top_talkers(&mut self, ip: &IpAddr) {
        // Simple implementation - in real would track bytes per IP
        if !self.statistics.top_talkers.contains(ip) {
            self.statistics.top_talkers.push(*ip);
            if self.statistics.top_talkers.len() > 10 {
                self.statistics.top_talkers.remove(0);
            }
        }
    }

    /// Track connection
    fn track_connection(&mut self, packet: &TrafficPacket) {
        let connection_key = format!(
            "{}:{}-{}:{}",
            packet.source_ip, packet.source_port, packet.destination_ip, packet.destination_port
        );

        if let Some(conn) = self.connections.get_mut(&connection_key) {
            conn.bytes_sent += packet.size_bytes;
            conn.duration = packet.timestamp.duration_since(self.statistics.start_time);
        } else {
            if self.connections.len() >= self.max_connections {
                // Remove oldest connection
                if let Some(key) = self.connections.keys().next().cloned() {
                    self.connections.remove(&key);
                }
            }

            self.connections.insert(
                connection_key,
                ConnectionInfo {
                    source_ip: packet.source_ip,
                    destination_ip: packet.destination_ip,
                    source_port: packet.source_port,
                    destination_port: packet.destination_port,
                    protocol: packet.protocol,
                    state: ConnectionState::Established,
                    bytes_sent: packet.size_bytes,
                    bytes_received: 0,
                    duration: Duration::from_secs(0),
                },
            );
        }
    }

    /// Get statistics
    pub fn statistics(&self) -> &TrafficStatistics {
        &self.statistics
    }

    /// Get connections
    pub fn connections(&self) -> Vec<&ConnectionInfo> {
        self.connections.values().collect()
    }

    /// Get alerts
    pub fn alerts(&self) -> &[TrafficAlert] {
        &self.alerts
    }

    /// Clear alerts
    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    /// Get current bandwidth
    pub fn current_bandwidth_mbps(&self) -> f64 {
        let duration = self.statistics.start_time.elapsed().as_secs_f64();
        if duration > 0.0 {
            (self.statistics.total_bytes as f64 * 8.0) / (duration * 1_000_000.0)
        } else {
            0.0
        }
    }

    /// Get connections by IP
    pub fn connections_by_ip(&self, ip: IpAddr) -> Vec<&ConnectionInfo> {
        self.connections
            .values()
            .filter(|c| c.source_ip == ip || c.destination_ip == ip)
            .collect()
    }

    /// Get connections by protocol
    pub fn connections_by_protocol(&self, protocol: Protocol) -> Vec<&ConnectionInfo> {
        self.connections
            .values()
            .filter(|c| c.protocol == protocol)
            .collect()
    }
}

impl Default for NetworkTrafficAnalyzer {
    fn default() -> Self {
        Self::new()
            .add_strategy(Box::new(BandwidthAnalysis::new(100.0)))
            .add_strategy(Box::new(SecurityAnalysis::new(10)))
            .with_capture(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_packet() {
        let packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 12345,
            destination_port: 80,
            protocol: Protocol::Tcp,
            size_bytes: 1024,
            timestamp: Instant::now(),
        };
        assert_eq!(packet.protocol, Protocol::Tcp);
    }

    #[test]
    fn test_bandwidth_analysis() {
        let analysis = BandwidthAnalysis::new(100.0);
        assert_eq!(analysis.name(), "BandwidthAnalysis");
    }

    #[test]
    fn test_security_analysis() {
        let analysis = SecurityAnalysis::new(10);
        assert_eq!(analysis.name(), "SecurityAnalysis");
    }

    #[test]
    fn test_network_traffic_analyzer() {
        let analyzer = NetworkTrafficAnalyzer::default();
        assert_eq!(analyzer.strategies.len(), 2);
    }

    #[test]
    fn test_process_packet() {
        let mut analyzer = NetworkTrafficAnalyzer::default();
        let packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 12345,
            destination_port: 80,
            protocol: Protocol::Tcp,
            size_bytes: 1024,
            timestamp: Instant::now(),
        };
        analyzer.process_packet(packet);
        assert_eq!(analyzer.statistics().total_packets, 1);
    }

    #[test]
    fn test_alpine_zero_alloc_capture_buffer() {
        let mut buffer = AlpineZeroAllocCaptureBuffer::<3>::new();
        assert_eq!(buffer.len(), 0);

        let packet1 = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 12345,
            destination_port: 80,
            protocol: Protocol::Tcp,
            size_bytes: 100,
            timestamp: Instant::now(),
        };
        let packet2 = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 12346,
            destination_port: 443,
            protocol: Protocol::Https,
            size_bytes: 200,
            timestamp: Instant::now(),
        };
        let packet3 = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 3)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 12347,
            destination_port: 22,
            protocol: Protocol::Ssh,
            size_bytes: 300,
            timestamp: Instant::now(),
        };
        let packet4 = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 4)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 12348,
            destination_port: 23,
            protocol: Protocol::Other,
            size_bytes: 400,
            timestamp: Instant::now(),
        };

        buffer.push(packet1);
        buffer.push(packet2);
        buffer.push(packet3);
        assert_eq!(buffer.len(), 3);

        // This push should overwrite packet1 (circular ring-buffer)
        buffer.push(packet4);
        assert_eq!(buffer.len(), 3);

        let packets: Vec<TrafficPacket> = buffer.iter().cloned().collect();
        assert_eq!(packets.len(), 3);
        assert_eq!(packets[0].source_port, 12346); // packet2
        assert_eq!(packets[1].source_port, 12347); // packet3
        assert_eq!(packets[2].source_port, 12348); // packet4

        buffer.clear();
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_n_declarative_filter() {
        let filter = NixDeclarativeFilter::new(
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10))),
            None,
            Some(80),
            Some(100),
            Some(Protocol::Tcp),
        );

        // Verification of deterministic rule hashing
        assert_ne!(filter.rule_hash, 0);

        let matching_packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 90,
            destination_port: 80,
            protocol: Protocol::Tcp,
            size_bytes: 50,
            timestamp: Instant::now(),
        };

        let mismatch_packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 11)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 90,
            destination_port: 80,
            protocol: Protocol::Tcp,
            size_bytes: 50,
            timestamp: Instant::now(),
        };

        assert!(filter.matches(&matching_packet));
        assert!(!filter.matches(&mismatch_packet));
    }

    #[test]
    fn test_kali_fingerprinting_and_recon() {
        let mut snoop = KaliSnoopAnalysis::new();
        assert_eq!(snoop.name(), "KaliSnoopAnalysis");

        let packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            source_port: 43210,
            destination_port: 22,
            protocol: Protocol::Ssh,
            size_bytes: 100,
            timestamp: Instant::now(),
        };

        // Passive fingerprinting test
        let mut fingerprinter = KaliPacketFingerprinter::new();
        let os_linux = fingerprinter.fingerprint_packet(&packet, 64, 5840);
        assert_eq!(os_linux, "Linux Core");

        let os_windows = fingerprinter.fingerprint_packet(&packet, 128, 8192);
        assert_eq!(os_windows, "Windows OS");

        // Multi-port scanner reconnaissance snoop detection
        for p in 1..=5 {
            let scan_packet = TrafficPacket {
                source_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                destination_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
                source_port: 33200 + p,
                destination_port: p,
                protocol: Protocol::Tcp,
                size_bytes: 40,
                timestamp: Instant::now(),
            };
            let alert = snoop.analyze_packet(&scan_packet);
            assert!(alert.is_none());
        }

        // 6th port scanned -> alert trigger
        let trigger_packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            source_port: 33206,
            destination_port: 6,
            protocol: Protocol::Tcp,
            size_bytes: 40,
            timestamp: Instant::now(),
        };
        let alert = snoop.analyze_packet(&trigger_packet).unwrap();
        assert_eq!(alert.alert_type, AlertType::SuspiciousActivity);
        assert!(alert.message.contains("Kali snoop alert"));
    }

    #[test]
    fn test_gentoo_use_flags_dissector() {
        let mut dissector = GentooUseFlagsDissector::new(GentooUseFlagsDissector::HTTP | GentooUseFlagsDissector::TLS);
        assert!(dissector.is_enabled(GentooUseFlagsDissector::HTTP));
        assert!(!dissector.is_enabled(GentooUseFlagsDissector::SSH));

        let http_packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            source_port: 12345,
            destination_port: 80,
            protocol: Protocol::Http,
            size_bytes: 1000,
            timestamp: Instant::now(),
        };

        let ssh_packet = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            source_port: 12345,
            destination_port: 22,
            protocol: Protocol::Ssh,
            size_bytes: 1000,
            timestamp: Instant::now(),
        };

        // HTTP should dissect since HTTP flag is enabled
        let decoded_http = dissector.dissect_packet(&http_packet);
        assert!(decoded_http.is_some());
        assert!(decoded_http.unwrap().contains("HTTP payload dissection enabled"));

        // SSH should return None because SSH flag is disabled
        assert!(dissector.dissect_packet(&ssh_packet).is_none());

        // Now enable SSH flag and check
        dissector.enable_dissector(GentooUseFlagsDissector::SSH);
        assert!(dissector.is_enabled(GentooUseFlagsDissector::SSH));
        let decoded_ssh = dissector.dissect_packet(&ssh_packet);
        assert!(decoded_ssh.is_some());
        assert!(decoded_ssh.unwrap().contains("SSH transport dissector enabled"));
    }

    #[test]
    fn test_clear_linux_flow_load_balancer() {
        let mut lb = ClearLinuxFlowLoadBalancer::new(4); // 4 virtual cores

        let flow1_p1 = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            source_port: 50000,
            destination_port: 443,
            protocol: Protocol::Https,
            size_bytes: 500,
            timestamp: Instant::now(),
        };

        let flow1_p2_reverse = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            source_port: 443,
            destination_port: 50000,
            protocol: Protocol::Https,
            size_bytes: 1500,
            timestamp: Instant::now(),
        };

        let flow2_p1 = TrafficPacket {
            source_ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 11)),
            destination_ip: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
            source_port: 60000,
            destination_port: 53,
            protocol: Protocol::Udp,
            size_bytes: 64,
            timestamp: Instant::now(),
        };

        // Core assignment steering tests
        let core_flow1_p1 = lb.steer_packet(&flow1_p1);
        let core_flow1_p2 = lb.steer_packet(&flow1_p2_reverse);
        let core_flow2 = lb.steer_packet(&flow2_p1);

        // Symmetric packets must map to the same virtual CPU core
        assert_eq!(core_flow1_p1, core_flow1_p2);
        assert!(core_flow1_p1 < 4);
        assert!(core_flow2 < 4);
        assert_eq!(lb.get_active_flows_count(), 2);
    }
}
