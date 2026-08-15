// SigmaOS Network Configuration Management
// Linux distro-inspired network configuration
// Handles network interfaces, routing, DNS, and network settings

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Network interface configuration
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: InterfaceType,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub netmask: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
    pub dhcp_enabled: bool,
    pub mtu: u32,
    pub status: InterfaceStatus,
}

/// Interface types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    WiFi,
    Loopback,
    Bridge,
    VLAN,
    Bond,
}

/// Interface status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceStatus {
    Up,
    Down,
    Configuring,
    Error,
}

/// Network configuration manager
pub struct NetworkConfigManager {
    pub interfaces: BTreeMap<String, NetworkInterface>,
    pub routing_table: Vec<RouteEntry>,
    pub dns_config: DnsConfig,
    pub config_dir: String,
}

impl NetworkConfigManager {
    pub fn new(config_dir: &str) -> Self {
        Self {
            interfaces: BTreeMap::new(),
            routing_table: Vec::new(),
            dns_config: DnsConfig::default(),
            config_dir: String::from(config_dir),
        }
    }

    /// Initialize network configuration
    pub fn initialize(&mut self) -> Result<(), NetworkError> {
        // Create loopback interface
        let loopback = NetworkInterface {
            name: String::from("lo"),
            interface_type: InterfaceType::Loopback,
            mac_address: String::from("00:00:00:00:00:00"),
            ip_address: Some(String::from("127.0.0.1")),
            netmask: Some(String::from("255.0.0.0")),
            gateway: None,
            dns_servers: Vec::new(),
            dhcp_enabled: false,
            mtu: 65536,
            status: InterfaceStatus::Up,
        };
        self.interfaces.insert(String::from("lo"), loopback);

        // Detect network interfaces
        self.detect_interfaces()?;

        Ok(())
    }

    /// Detect network interfaces
    fn detect_interfaces(&mut self) -> Result<(), NetworkError> {
        // Simulated interface detection
        let eth0 = NetworkInterface {
            name: String::from("eth0"),
            interface_type: InterfaceType::Ethernet,
            mac_address: String::from("00:11:22:33:44:55"),
            ip_address: None,
            netmask: None,
            gateway: None,
            dns_servers: Vec::new(),
            dhcp_enabled: true,
            mtu: 1500,
            status: InterfaceStatus::Down,
        };
        self.interfaces.insert(String::from("eth0"), eth0);

        Ok(())
    }

    /// Configure interface with static IP
    pub fn configure_static_ip(&mut self, interface: &str, ip: &str, netmask: &str, gateway: &str) -> Result<(), NetworkError> {
        if let Some(iface) = self.interfaces.get_mut(interface) {
            iface.ip_address = Some(String::from(ip));
            iface.netmask = Some(String::from(netmask));
            iface.gateway = Some(String::from(gateway));
            iface.dhcp_enabled = false;
            iface.status = InterfaceStatus::Up;
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound(String::from(interface)))
        }
    }

    /// Enable DHCP on interface
    pub fn enable_dhcp(&mut self, interface: &str) -> Result<(), NetworkError> {
        if let Some(iface) = self.interfaces.get_mut(interface) {
            iface.dhcp_enabled = true;
            iface.status = InterfaceStatus::Configuring;
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound(String::from(interface)))
        }
    }

    /// Set DNS servers
    pub fn set_dns_servers(&mut self, servers: Vec<String>) {
        self.dns_config.servers = servers;
    }

    /// Add route to routing table
    pub fn add_route(&mut self, destination: &str, gateway: &str, interface: &str) -> Result<(), NetworkError> {
        let route = RouteEntry {
            destination: String::from(destination),
            gateway: String::from(gateway),
            interface: String::from(interface),
            metric: 100,
        };
        self.routing_table.push(route);
        Ok(())
    }

    /// Get interface by name
    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    /// Get all interfaces
    pub fn get_all_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values().collect()
    }

    /// Bring interface up
    pub fn bring_up(&mut self, interface: &str) -> Result<(), NetworkError> {
        if let Some(iface) = self.interfaces.get_mut(interface) {
            iface.status = InterfaceStatus::Up;
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound(String::from(interface)))
        }
    }

    /// Bring interface down
    pub fn bring_down(&mut self, interface: &str) -> Result<(), NetworkError> {
        if let Some(iface) = self.interfaces.get_mut(interface) {
            iface.status = InterfaceStatus::Down;
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound(String::from(interface)))
        }
    }

    /// Save network configuration
    pub fn save_config(&self) -> Result<(), NetworkError> {
        // In real implementation, save to configuration files
        Ok(())
    }

    /// Load network configuration
    pub fn load_config(&mut self) -> Result<(), NetworkError> {
        // In real implementation, load from configuration files
        Ok(())
    }
}

/// DNS configuration
#[derive(Debug, Clone)]
pub struct DnsConfig {
    pub servers: Vec<String>,
    pub search_domains: Vec<String>,
    pub timeout: u32,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            servers: vec![String::from("8.8.8.8"), String::from("8.8.4.4")],
            search_domains: Vec::new(),
            timeout: 5,
        }
    }
}

/// Routing table entry
#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub destination: String,
    pub gateway: String,
    pub interface: String,
    pub metric: u32,
}

/// Network errors
#[derive(Debug)]
pub enum NetworkError {
    InterfaceNotFound(String),
    ConfigurationError(String),
    DhcpError(String),
    RoutingError(String),
    DnsError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_manager() {
        let mut manager = NetworkConfigManager::new("/etc/network");
        assert!(manager.initialize().is_ok());
        assert!(manager.get_interface("lo").is_some());
    }

    #[test]
    fn test_static_ip_configuration() {
        let mut manager = NetworkConfigManager::new("/etc/network");
        manager.initialize().unwrap();
        
        assert!(manager.configure_static_ip("eth0", "192.168.1.100", "255.255.255.0", "192.168.1.1").is_ok());
        
        let iface = manager.get_interface("eth0").unwrap();
        assert_eq!(iface.ip_address, Some(String::from("192.168.1.100")));
    }

    #[test]
    fn test_dhcp_configuration() {
        let mut manager = NetworkConfigManager::new("/etc/network");
        manager.initialize().unwrap();
        
        assert!(manager.enable_dhcp("eth0").is_ok());
        
        let iface = manager.get_interface("eth0").unwrap();
        assert!(iface.dhcp_enabled);
    }

    #[test]
    fn test_interface_status() {
        let mut manager = NetworkConfigManager::new("/etc/network");
        manager.initialize().unwrap();
        
        assert!(manager.bring_up("eth0").is_ok());
        assert!(manager.bring_down("eth0").is_ok());
    }

    #[test]
    fn test_dns_configuration() {
        let mut manager = NetworkConfigManager::new("/etc/network");
        manager.initialize().unwrap();
        
        manager.set_dns_servers(vec![String::from("1.1.1.1"), String::from("1.0.0.1")]);
        assert_eq!(manager.dns_config.servers.len(), 2);
    }
}
