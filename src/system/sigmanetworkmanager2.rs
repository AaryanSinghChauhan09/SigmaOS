//! SigmaNetworkManager - Network Configuration Management System
//!
//! Provides comprehensive network configuration management inspired by Arch, Debian, and CachyOS.
//! Supports network interfaces, connections, DNS, DHCP, and network profiles.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    WiFi,
    VPN,
    Bluetooth,
    Other,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Connecting,
    Failed,
}

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String,
    pub connection_type: ConnectionType,
    pub state: ConnectionState,
    pub ip_address: Option<String>,
    pub netmask: Option<String>,
    pub gateway: Option<String>,
    pub mac_address: String,
    pub is_up: bool,
}

impl NetworkInterface {
    pub fn new(id: String, name: String, connection_type: ConnectionType, mac_address: String) -> Self {
        Self {
            id,
            name,
            connection_type,
            state: ConnectionState::Disconnected,
            ip_address: None,
            netmask: None,
            gateway: None,
            mac_address,
            is_up: false,
        }
    }

    pub fn set_state(&mut self, state: ConnectionState) {
        self.state = state;
    }

    pub fn set_ip_address(&mut self, ip: Option<String>) {
        self.ip_address = ip;
    }

    pub fn set_netmask(&mut self, netmask: Option<String>) {
        self.netmask = netmask;
    }

    pub fn set_gateway(&mut self, gateway: Option<String>) {
        self.gateway = gateway;
    }

    pub fn set_up(&mut self, up: bool) {
        self.is_up = up;
    }

    pub fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }
}

/// DNS server
#[derive(Debug, Clone)]
pub struct DNSServer {
    pub id: String,
    pub address: String,
    pub is_primary: bool,
}

impl DNSServer {
    pub fn new(id: String, address: String, is_primary: bool) -> Self {
        Self {
            id,
            address,
            is_primary,
        }
    }
}

/// Network profile
#[derive(Debug, Clone)]
pub struct NetworkProfile {
    pub id: String,
    pub name: String,
    pub connection_type: ConnectionType,
    pub auto_connect: bool,
    pub use_dhcp: bool,
    pub static_ip: Option<String>,
    pub static_netmask: Option<String>,
    pub static_gateway: Option<String>,
}

impl NetworkProfile {
    pub fn new(id: String, name: String, connection_type: ConnectionType) -> Self {
        Self {
            id,
            name,
            connection_type,
            auto_connect: false,
            use_dhcp: true,
            static_ip: None,
            static_netmask: None,
            static_gateway: None,
        }
    }

    pub fn set_auto_connect(&mut self, auto: bool) {
        self.auto_connect = auto;
    }

    pub fn set_use_dhcp(&mut self, use_dhcp: bool) {
        self.use_dhcp = use_dhcp;
    }

    pub fn set_static_config(&mut self, ip: String, netmask: String, gateway: String) {
        self.static_ip = Some(ip);
        self.static_netmask = Some(netmask);
        self.static_gateway = Some(gateway);
        self.use_dhcp = false;
    }
}

/// SigmaNetworkManager - Network configuration management
#[derive(Debug)]
pub struct SigmaNetworkManager {
    interfaces: BTreeMap<String, NetworkInterface>,
    dns_servers: BTreeMap<String, DNSServer>,
    profiles: BTreeMap<String, NetworkProfile>,
}

impl SigmaNetworkManager {
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            dns_servers: BTreeMap::new(),
            profiles: BTreeMap::new(),
        }
    }

    pub fn add_interface(&mut self, interface: NetworkInterface) {
        self.interfaces.insert(interface.id.clone(), interface);
    }

    pub fn remove_interface(&mut self, id: &str) -> bool {
        self.interfaces.remove(id).is_some()
    }

    pub fn get_interface(&self, id: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(id)
    }

    pub fn get_interface_mut(&mut self, id: &str) -> Option<&mut NetworkInterface> {
        self.interfaces.get_mut(id)
    }

    pub fn add_dns_server(&mut self, dns: DNSServer) {
        self.dns_servers.insert(dns.id.clone(), dns);
    }

    pub fn remove_dns_server(&mut self, id: &str) -> bool {
        self.dns_servers.remove(id).is_some()
    }

    pub fn get_dns_server(&self, id: &str) -> Option<&DNSServer> {
        self.dns_servers.get(id)
    }

    pub fn add_profile(&mut self, profile: NetworkProfile) {
        self.profiles.insert(profile.id.clone(), profile);
    }

    pub fn remove_profile(&mut self, id: &str) -> bool {
        self.profiles.remove(id).is_some()
    }

    pub fn get_profile(&self, id: &str) -> Option<&NetworkProfile> {
        self.profiles.get(id)
    }

    pub fn list_connected(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values()
            .filter(|iface| iface.is_connected())
            .collect()
    }

    pub fn list_by_type(&self, conn_type: ConnectionType) -> Vec<&NetworkInterface> {
        self.interfaces.values()
            .filter(|iface| iface.connection_type == conn_type)
            .collect()
    }

    pub fn list_primary_dns(&self) -> Vec<&DNSServer> {
        self.dns_servers.values()
            .filter(|dns| dns.is_primary)
            .collect()
    }

    pub fn list_auto_connect_profiles(&self) -> Vec<&NetworkProfile> {
        self.profiles.values()
            .filter(|profile| profile.auto_connect)
            .collect()
    }

    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    pub fn dns_count(&self) -> usize {
        self.dns_servers.len()
    }

    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }
}

impl Default for SigmaNetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNetworkManager instance
static mut SIGMANETWORKMANAGER_INSTANCE: Option<SigmaNetworkManager> = None;

/// Initialize the global SigmaNetworkManager instance
pub fn init_sigmanetworkmanager() {
    unsafe {
        SIGMANETWORKMANAGER_INSTANCE = Some(SigmaNetworkManager::new());
    }
}

/// Get the global SigmaNetworkManager instance
pub fn get_sigmanetworkmanager() -> Option<&'static SigmaNetworkManager> {
    unsafe {
        SIGMANETWORKMANAGER_INSTANCE.as_ref()
    }
}

/// Get the global SigmaNetworkManager instance (mutable)
pub fn get_sigmanetworkmanager_mut() -> Option<&'static mut SigmaNetworkManager> {
    unsafe {
        SIGMANETWORKMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add an interface
pub fn add_interface(interface: NetworkInterface) -> bool {
    if let Some(manager) = get_sigmanetworkmanager_mut() {
        manager.add_interface(interface);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_interface_creation() {
        let iface = NetworkInterface::new(
            "eth0".to_string(),
            "eth0".to_string(),
            ConnectionType::Ethernet,
            "00:11:22:33:44:55".to_string(),
        );
        assert_eq!(iface.id, "eth0");
        assert_eq!(iface.connection_type, ConnectionType::Ethernet);
    }

    #[test]
    fn test_network_interface_modification() {
        let mut iface = NetworkInterface::new(
            "wlan0".to_string(),
            "wlan0".to_string(),
            ConnectionType::WiFi,
            "00:11:22:33:44:56".to_string(),
        );
        iface.set_state(ConnectionState::Connected);
        iface.set_ip_address(Some("192.168.1.100".to_string()));
        iface.set_up(true);
        assert!(iface.is_connected());
        assert!(iface.is_up);
    }

    #[test]
    fn test_dns_server_creation() {
        let dns = DNSServer::new(
            "dns1".to_string(),
            "8.8.8.8".to_string(),
            true,
        );
        assert_eq!(dns.id, "dns1");
        assert!(dns.is_primary);
    }

    #[test]
    fn test_network_profile_creation() {
        let profile = NetworkProfile::new(
            "profile1".to_string(),
            "Home WiFi".to_string(),
            ConnectionType::WiFi,
        );
        assert_eq!(profile.id, "profile1");
        assert!(profile.use_dhcp);
    }

    #[test]
    fn test_network_profile_modification() {
        let mut profile = NetworkProfile::new(
            "profile1".to_string(),
            "Test".to_string(),
            ConnectionType::Ethernet,
        );
        profile.set_auto_connect(true);
        profile.set_static_config("192.168.1.50".to_string(), "255.255.255.0".to_string(), "192.168.1.1".to_string());
        assert!(profile.auto_connect);
        assert!(!profile.use_dhcp);
    }

    #[test]
    fn test_sigmanetworkmanager_creation() {
        let manager = SigmaNetworkManager::new();
        assert_eq!(manager.interface_count(), 0);
        assert_eq!(manager.dns_count(), 0);
    }

    #[test]
    fn test_sigmanetworkmanager_management() {
        let mut manager = SigmaNetworkManager::new();
        let iface = NetworkInterface::new(
            "eth0".to_string(),
            "eth0".to_string(),
            ConnectionType::Ethernet,
            "00:11:22:33:44:55".to_string(),
        );
        manager.add_interface(iface);
        assert_eq!(manager.interface_count(), 1);
    }

    #[test]
    fn test_connected_filtering() {
        let mut manager = SigmaNetworkManager::new();
        let mut iface = NetworkInterface::new(
            "eth0".to_string(),
            "eth0".to_string(),
            ConnectionType::Ethernet,
            "00:11:22:33:44:55".to_string(),
        );
        iface.set_state(ConnectionState::Connected);
        manager.add_interface(iface);
        let connected = manager.list_connected();
        assert_eq!(connected.len(), 1);
    }
}
