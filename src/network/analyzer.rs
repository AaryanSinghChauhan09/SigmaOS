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
}
