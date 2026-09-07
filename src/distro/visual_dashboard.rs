// SigmaOS Visual-First Management Dashboard
// Implements intuitive GUI system controls for firewall, VPN, capabilities, and telemetry
// Inspired by Linux Mint and Clear Linux visual management paradigms

use std::string::String;
use std::vec::Vec;

/// Firewall policy configuration
#[derive(Debug, Clone)]
pub struct FirewallPolicy {
    pub name: String,
    pub action: FirewallAction,
    pub source: String,
    pub destination: String,
    pub port: Option<u16>,
    pub protocol: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
    Log,
}

/// VPN tunnel configuration
#[derive(Debug, Clone)]
pub struct VpnTunnel {
    pub name: String,
    pub vpn_type: VpnType,
    pub server_address: String,
    pub local_address: String,
    pub status: VpnStatus,
    pub connected_since: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnType {
    WireGuard,
    OpenVPN,
    IPSec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// Process capability entry
#[derive(Debug, Clone)]
pub struct ProcessCapability {
    pub pid: u32,
    pub name: String,
    pub capabilities: Vec<String>,
    pub sandbox_policy: String,
}

/// Hardware telemetry data
#[derive(Debug, Clone)]
pub struct HardwareTelemetry {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_in: u64,
    pub network_out: u64,
    pub temperature: Option<f32>,
}

/// Visual dashboard manager
pub struct VisualDashboardManager {
    pub firewall_policies: Vec<FirewallPolicy>,
    pub vpn_tunnels: Vec<VpnTunnel>,
    pub process_capabilities: Vec<ProcessCapability>,
    pub telemetry: HardwareTelemetry,
}

impl VisualDashboardManager {
    pub fn new() -> Self {
        Self {
            firewall_policies: Vec::new(),
            vpn_tunnels: Vec::new(),
            process_capabilities: Vec::new(),
            telemetry: HardwareTelemetry {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                network_in: 0,
                network_out: 0,
                temperature: None,
            },
        }
    }

    /// Add firewall policy
    pub fn add_firewall_policy(&mut self, policy: FirewallPolicy) {
        self.firewall_policies.push(policy);
    }

    /// Enable/disable firewall policy
    pub fn toggle_firewall_policy(&mut self, index: usize) {
        if index < self.firewall_policies.len() {
            self.firewall_policies[index].enabled = !self.firewall_policies[index].enabled;
        }
    }

    /// Add VPN tunnel
    pub fn add_vpn_tunnel(&mut self, tunnel: VpnTunnel) {
        self.vpn_tunnels.push(tunnel);
    }

    /// Connect VPN tunnel
    pub fn connect_vpn(&mut self, index: usize) {
        if index < self.vpn_tunnels.len() {
            self.vpn_tunnels[index].status = VpnStatus::Connecting;
            // In real implementation, would establish connection
            self.vpn_tunnels[index].status = VpnStatus::Connected;
            self.vpn_tunnels[index].connected_since = Some(1234567890);
        }
    }

    /// Disconnect VPN tunnel
    pub fn disconnect_vpn(&mut self, index: usize) {
        if index < self.vpn_tunnels.len() {
            self.vpn_tunnels[index].status = VpnStatus::Disconnected;
            self.vpn_tunnels[index].connected_since = None;
        }
    }

    /// Update process capabilities
    pub fn update_process_capabilities(&mut self, capabilities: Vec<ProcessCapability>) {
        self.process_capabilities = capabilities;
    }

    /// Update telemetry
    pub fn update_telemetry(&mut self, telemetry: HardwareTelemetry) {
        self.telemetry = telemetry;
    }

    /// Get dashboard summary
    pub fn get_summary(&self) -> String {
        format!(
            "Dashboard Summary\nFirewall Policies: {}\nVPN Tunnels: {}\nProcesses: {}\nCPU: {:.1}%\nMemory: {:.1}%",
            self.firewall_policies.len(),
            self.vpn_tunnels.len(),
            self.process_capabilities.len(),
            self.telemetry.cpu_usage,
            self.telemetry.memory_usage
        )
    }
}

impl Default for VisualDashboardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_policy() {
        let mut dashboard = VisualDashboardManager::new();
        let policy = FirewallPolicy {
            name: "Allow SSH".to_string(),
            action: FirewallAction::Allow,
            source: "0.0.0.0/0".to_string(),
            destination: "any".to_string(),
            port: Some(22),
            protocol: Some("tcp".to_string()),
            enabled: true,
        };

        dashboard.add_firewall_policy(policy);
        assert_eq!(dashboard.firewall_policies.len(), 1);
    }

    #[test]
    fn test_vpn_tunnel() {
        let mut dashboard = VisualDashboardManager::new();
        let tunnel = VpnTunnel {
            name: "Home VPN".to_string(),
            vpn_type: VpnType::WireGuard,
            server_address: "192.168.1.1".to_string(),
            local_address: "10.0.0.2".to_string(),
            status: VpnStatus::Disconnected,
            connected_since: None,
        };

        dashboard.add_vpn_tunnel(tunnel);
        dashboard.connect_vpn(0);
        assert_eq!(dashboard.vpn_tunnels[0].status, VpnStatus::Connected);
    }
}
