// Network Diagnostics for SigmaOS
// Network diagnostics per Wiki 06-Networking.md
// Provides connectivity testing, monitoring, and diagnostics

use std::string::{String, ToString};
use std::vec::Vec;

/// Ping result
#[derive(Debug, Clone)]
pub struct PingResult {
    pub target: String,
    pub packets_sent: u32,
    pub packets_received: u32,
    pub packet_loss_percent: f32,
    pub min_rtt_ms: f32,
    pub max_rtt_ms: f32,
    pub avg_rtt_ms: f32,
    pub success: bool,
}

impl PingResult {
    pub fn new(target: String) -> Self {
        PingResult {
            target,
            packets_sent: 0,
            packets_received: 0,
            packet_loss_percent: 0.0,
            min_rtt_ms: 0.0,
            max_rtt_ms: 0.0,
            avg_rtt_ms: 0.0,
            success: false,
        }
    }

    pub fn get_summary(&self) -> String {
        if self.success {
            format!(
                "Ping {}: {} packets transmitted, {} received, {:.1}% packet loss, rtt min/avg/max = {:.2}/{:.2}/{:.2} ms",
                self.target,
                self.packets_sent,
                self.packets_received,
                self.packet_loss_percent,
                self.min_rtt_ms,
                self.avg_rtt_ms,
                self.max_rtt_ms
            )
        } else {
            format!("Ping {}: failed", self.target)
        }
    }
}

/// Trace route hop
#[derive(Debug, Clone)]
pub struct TraceRouteHop {
    pub hop_number: u32,
    pub hostname: String,
    pub ip_address: String,
    pub rtt_ms: Vec<f32>,
}

impl TraceRouteHop {
    pub fn new(hop_number: u32, hostname: String, ip_address: String) -> Self {
        TraceRouteHop {
            hop_number,
            hostname,
            ip_address,
            rtt_ms: Vec::new(),
        }
    }

    pub fn add_rtt(&mut self, rtt: f32) {
        self.rtt_ms.push(rtt);
    }

    pub fn get_avg_rtt(&self) -> f32 {
        if self.rtt_ms.is_empty() {
            0.0
        } else {
            let sum: f32 = self.rtt_ms.iter().sum();
            sum / self.rtt_ms.len() as f32
        }
    }
}

/// Trace route result
#[derive(Debug, Clone)]
pub struct TraceRouteResult {
    pub target: String,
    pub hops: Vec<TraceRouteHop>,
    pub success: bool,
}

impl TraceRouteResult {
    pub fn new(target: String) -> Self {
        TraceRouteResult {
            target,
            hops: Vec::new(),
            success: false,
        }
    }

    pub fn add_hop(&mut self, hop: TraceRouteHop) {
        self.hops.push(hop);
    }

    pub fn get_summary(&self) -> String {
        if self.success {
            let mut summary = format!(
                "Trace route to {} ({} hops):\n",
                self.target,
                self.hops.len()
            );
            for hop in &self.hops {
                summary.push_str(&format!(
                    "  {}: {} ({}) - {:.2} ms\n",
                    hop.hop_number,
                    hop.hostname,
                    hop.ip_address,
                    hop.get_avg_rtt()
                ));
            }
            summary
        } else {
            format!("Trace route to {}: failed", self.target)
        }
    }
}

/// DNS lookup result
#[derive(Debug, Clone)]
pub struct DnsLookupResult {
    pub hostname: String,
    pub ip_addresses: Vec<String>,
    pub query_time_ms: f32,
    pub success: bool,
}

impl DnsLookupResult {
    pub fn new(hostname: String) -> Self {
        DnsLookupResult {
            hostname,
            ip_addresses: Vec::new(),
            query_time_ms: 0.0,
            success: false,
        }
    }

    pub fn add_ip(&mut self, ip: String) {
        self.ip_addresses.push(ip);
    }

    pub fn get_summary(&self) -> String {
        if self.success {
            format!(
                "DNS lookup for {}: {} addresses found in {:.2} ms - {}",
                self.hostname,
                self.ip_addresses.len(),
                self.query_time_ms,
                self.ip_addresses.join(", ")
            )
        } else {
            format!("DNS lookup for {}: failed", self.hostname)
        }
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub interface: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors_send: u64,
    pub errors_receive: u64,
    pub drops_send: u64,
    pub drops_receive: u64,
}

impl NetworkStats {
    pub fn new(interface: String) -> Self {
        NetworkStats {
            interface,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            errors_send: 0,
            errors_receive: 0,
            drops_send: 0,
            drops_receive: 0,
        }
    }

    pub fn get_total_bytes(&self) -> u64 {
        self.bytes_sent + self.bytes_received
    }

    pub fn get_total_packets(&self) -> u64 {
        self.packets_sent + self.packets_received
    }

    pub fn get_total_errors(&self) -> u64 {
        self.errors_send + self.errors_receive
    }

    pub fn get_total_drops(&self) -> u64 {
        self.drops_send + self.drops_receive
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Interface {}: TX: {} bytes/{} packets, RX: {} bytes/{} packets, Errors: {}, Drops: {}",
            self.interface,
            self.bytes_sent,
            self.packets_sent,
            self.bytes_received,
            self.packets_received,
            self.get_total_errors(),
            self.get_total_drops()
        )
    }
}

/// Network connection
#[derive(Debug, Clone)]
pub struct NetworkConnection {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub state: String,
    pub pid: Option<u32>,
}

impl NetworkConnection {
    pub fn new(
        protocol: String,
        local_address: String,
        remote_address: String,
        state: String,
    ) -> Self {
        NetworkConnection {
            protocol,
            local_address,
            remote_address,
            state,
            pid: None,
        }
    }

    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }
}

/// Bandwidth usage
#[derive(Debug, Clone)]
pub struct BandwidthUsage {
    pub interface: String,
    pub upload_mbps: f32,
    pub download_mbps: f32,
    pub total_mbps: f32,
}

impl BandwidthUsage {
    pub fn new(interface: String) -> Self {
        BandwidthUsage {
            interface,
            upload_mbps: 0.0,
            download_mbps: 0.0,
            total_mbps: 0.0,
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Bandwidth {}: Upload: {:.2} Mbps, Download: {:.2} Mbps, Total: {:.2} Mbps",
            self.interface, self.upload_mbps, self.download_mbps, self.total_mbps
        )
    }
}

/// Network diagnostics manager
#[derive(Debug, Clone)]
pub struct NetworkDiagnostics {
    pub enabled: bool,
}

impl Default for NetworkDiagnostics {
    fn default() -> Self {
        NetworkDiagnostics { enabled: true }
    }
}

impl NetworkDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn ping(&self, target: String, count: u32) -> PingResult {
        if !self.enabled {
            return PingResult::new(target);
        }

        let mut result = PingResult::new(target.clone());
        result.packets_sent = count;

        // Simulate ping result
        result.packets_received = count;
        result.packet_loss_percent = 0.0;
        result.min_rtt_ms = 10.0;
        result.max_rtt_ms = 20.0;
        result.avg_rtt_ms = 15.0;
        result.success = true;

        result
    }

    pub fn traceroute(&self, target: String, max_hops: u32) -> TraceRouteResult {
        if !self.enabled {
            return TraceRouteResult::new(target);
        }

        let mut result = TraceRouteResult::new(target.clone());

        // Simulate trace route
        for i in 1..=max_hops.min(10) {
            let mut hop =
                TraceRouteHop::new(i, format!("hop-{}", i), format!("192.168.{}.{}", i, 1));
            hop.add_rtt(10.0 + i as f32);
            hop.add_rtt(12.0 + i as f32);
            hop.add_rtt(11.0 + i as f32);
            result.add_hop(hop);
        }

        result.success = true;
        result
    }

    pub fn nslookup(&self, hostname: String) -> DnsLookupResult {
        if !self.enabled {
            return DnsLookupResult::new(hostname);
        }

        let mut result = DnsLookupResult::new(hostname.clone());

        // Simulate DNS lookup
        result.add_ip(String::from("192.168.1.1"));
        result.add_ip(String::from("192.168.1.2"));
        result.query_time_ms = 5.0;
        result.success = true;

        result
    }

    pub fn get_stats(&self, interface: String) -> NetworkStats {
        if !self.enabled {
            return NetworkStats::new(interface);
        }

        let mut stats = NetworkStats::new(interface);
        stats.bytes_sent = 1024000;
        stats.bytes_received = 2048000;
        stats.packets_sent = 1000;
        stats.packets_received = 2000;
        stats.errors_send = 0;
        stats.errors_receive = 1;
        stats.drops_send = 0;
        stats.drops_receive = 2;

        stats
    }

    pub fn get_connections(&self) -> Vec<NetworkConnection> {
        if !self.enabled {
            return Vec::new();
        }

        vec![
            NetworkConnection::new(
                String::from("tcp"),
                String::from("192.168.1.100:50000"),
                String::from("93.184.216.34:80"),
                String::from("ESTABLISHED"),
            )
            .with_pid(1234),
            NetworkConnection::new(
                String::from("tcp"),
                String::from("192.168.1.100:50001"),
                String::from("8.8.8.8:53"),
                String::from("ESTABLISHED"),
            )
            .with_pid(5678),
            NetworkConnection::new(
                String::from("udp"),
                String::from("192.168.1.100:50002"),
                String::from("8.8.8.8:53"),
                String::from("ESTABLISHED"),
            ),
        ]
    }

    pub fn get_bandwidth(&self, interface: String) -> BandwidthUsage {
        if !self.enabled {
            return BandwidthUsage::new(interface);
        }

        let mut bandwidth = BandwidthUsage::new(interface);
        bandwidth.upload_mbps = 50.0;
        bandwidth.download_mbps = 100.0;
        bandwidth.total_mbps = 150.0;

        bandwidth
    }

    pub fn diagnose_connectivity(&self, target: String) -> String {
        let mut diagnosis = String::new();

        diagnosis.push_str(&format!("Diagnosing connectivity to {}\n", target));

        // Ping test
        let ping_result = self.ping(target.clone(), 4);
        diagnosis.push_str(&format!("Ping: {}\n", ping_result.get_summary()));

        // DNS lookup
        let dns_result = self.nslookup(target.clone());
        diagnosis.push_str(&format!("DNS: {}\n", dns_result.get_summary()));

        // Trace route
        let trace_result = self.traceroute(target, 10);
        diagnosis.push_str(&format!("Trace route: {}\n", trace_result.get_summary()));

        diagnosis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_result_creation() {
        let result = PingResult::new(String::from("example.com"));
        assert_eq!(result.target, "example.com");
        assert!(!result.success);
    }

    #[test]
    fn test_ping_result_summary() {
        let mut result = PingResult::new(String::from("example.com"));
        result.packets_sent = 4;
        result.packets_received = 4;
        result.packet_loss_percent = 0.0;
        result.min_rtt_ms = 10.0;
        result.max_rtt_ms = 20.0;
        result.avg_rtt_ms = 15.0;
        result.success = true;

        let summary = result.get_summary();
        assert!(summary.contains("example.com"));
        assert!(summary.contains("4 packets"));
    }

    #[test]
    fn test_trace_route_hop_creation() {
        let hop = TraceRouteHop::new(1, String::from("hop-1"), String::from("192.168.1.1"));
        assert_eq!(hop.hop_number, 1);
        assert_eq!(hop.hostname, "hop-1");
    }

    #[test]
    fn test_trace_route_hop_add_rtt() {
        let mut hop = TraceRouteHop::new(1, String::from("hop-1"), String::from("192.168.1.1"));
        hop.add_rtt(10.0);
        hop.add_rtt(20.0);

        assert_eq!(hop.rtt_ms.len(), 2);
        assert_eq!(hop.get_avg_rtt(), 15.0);
    }

    #[test]
    fn test_trace_route_result_creation() {
        let result = TraceRouteResult::new(String::from("example.com"));
        assert_eq!(result.target, "example.com");
        assert!(!result.success);
    }

    #[test]
    fn test_trace_route_result_add_hop() {
        let mut result = TraceRouteResult::new(String::from("example.com"));
        let hop = TraceRouteHop::new(1, String::from("hop-1"), String::from("192.168.1.1"));
        result.add_hop(hop);

        assert_eq!(result.hops.len(), 1);
    }

    #[test]
    fn test_dns_lookup_result_creation() {
        let result = DnsLookupResult::new(String::from("example.com"));
        assert_eq!(result.hostname, "example.com");
        assert!(!result.success);
    }

    #[test]
    fn test_dns_lookup_result_add_ip() {
        let mut result = DnsLookupResult::new(String::from("example.com"));
        result.add_ip(String::from("192.168.1.1"));
        result.add_ip(String::from("192.168.1.2"));

        assert_eq!(result.ip_addresses.len(), 2);
    }

    #[test]
    fn test_network_stats_creation() {
        let stats = NetworkStats::new(String::from("eth0"));
        assert_eq!(stats.interface, "eth0");
        assert_eq!(stats.bytes_sent, 0);
    }

    #[test]
    fn test_network_stats_totals() {
        let mut stats = NetworkStats::new(String::from("eth0"));
        stats.bytes_sent = 1000;
        stats.bytes_received = 2000;
        stats.packets_sent = 10;
        stats.packets_received = 20;
        stats.errors_send = 1;
        stats.errors_receive = 2;
        stats.drops_send = 3;
        stats.drops_receive = 4;

        assert_eq!(stats.get_total_bytes(), 3000);
        assert_eq!(stats.get_total_packets(), 30);
        assert_eq!(stats.get_total_errors(), 3);
        assert_eq!(stats.get_total_drops(), 7);
    }

    #[test]
    fn test_network_connection_creation() {
        let conn = NetworkConnection::new(
            String::from("tcp"),
            String::from("192.168.1.100:50000"),
            String::from("93.184.216.34:80"),
            String::from("ESTABLISHED"),
        );
        assert_eq!(conn.protocol, "tcp");
        assert!(conn.pid.is_none());
    }

    #[test]
    fn test_network_connection_with_pid() {
        let conn = NetworkConnection::new(
            String::from("tcp"),
            String::from("192.168.1.100:50000"),
            String::from("93.184.216.34:80"),
            String::from("ESTABLISHED"),
        )
        .with_pid(1234);
        ).with_pid(1234);

        assert_eq!(conn.pid, Some(1234));
    }

    #[test]
    fn test_bandwidth_usage_creation() {
        let bandwidth = BandwidthUsage::new(String::from("eth0"));
        assert_eq!(bandwidth.interface, "eth0");
        assert_eq!(bandwidth.upload_mbps, 0.0);
    }

    #[test]
    fn test_network_diagnostics_creation() {
        let diag = NetworkDiagnostics::new();
        assert!(diag.enabled);
    }

    #[test]
    fn test_network_diagnostics_enable_disable() {
        let mut diag = NetworkDiagnostics::new();
        diag.disable();
        assert!(!diag.is_enabled());

        diag.enable();
        assert!(diag.is_enabled());
    }

    #[test]
    fn test_network_diagnostics_ping() {
        let diag = NetworkDiagnostics::new();
        let result = diag.ping(String::from("example.com"), 4);

        assert!(result.success);
        assert_eq!(result.packets_sent, 4);
    }

    #[test]
    fn test_network_diagnostics_traceroute() {
        let diag = NetworkDiagnostics::new();
        let result = diag.traceroute(String::from("example.com"), 5);

        assert!(result.success);
        assert_eq!(result.hops.len(), 5);
    }

    #[test]
    fn test_network_diagnostics_nslookup() {
        let diag = NetworkDiagnostics::new();
        let result = diag.nslookup(String::from("example.com"));

        assert!(result.success);
        assert_eq!(result.ip_addresses.len(), 2);
    }

    #[test]
    fn test_network_diagnostics_get_stats() {
        let diag = NetworkDiagnostics::new();
        let stats = diag.get_stats(String::from("eth0"));

        assert_eq!(stats.interface, "eth0");
        assert!(stats.bytes_sent > 0);
    }

    #[test]
    fn test_network_diagnostics_get_connections() {
        let diag = NetworkDiagnostics::new();
        let connections = diag.get_connections();

        assert!(!connections.is_empty());
        assert_eq!(connections.len(), 3);
    }

    #[test]
    fn test_network_diagnostics_get_bandwidth() {
        let diag = NetworkDiagnostics::new();
        let bandwidth = diag.get_bandwidth(String::from("eth0"));

        assert_eq!(bandwidth.interface, "eth0");
        assert!(bandwidth.download_mbps > 0);
    }

    #[test]
    fn test_network_diagnostics_diagnose_connectivity() {
        let diag = NetworkDiagnostics::new();
        let diagnosis = diag.diagnose_connectivity(String::from("example.com"));

        assert!(diagnosis.contains("Diagnosing connectivity"));
        assert!(diagnosis.contains("Ping"));
        assert!(diagnosis.contains("DNS"));
        assert!(diagnosis.contains("Trace route"));
    }

    #[test]
    fn test_network_diagnostics_disabled() {
        let mut diag = NetworkDiagnostics::new();
        diag.disable();

        let result = diag.ping(String::from("example.com"), 4);
        assert!(!result.success);

        let stats = diag.get_stats(String::from("eth0"));
        assert_eq!(stats.bytes_sent, 0);
    }
}
