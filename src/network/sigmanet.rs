// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaNet - Network Configuration Daemon
//!
//! SigmaNet provides network management for SigmaOS, inspired by NetworkManager from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Network interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    /// Ethernet
    Ethernet,
    /// WiFi
    WiFi,
    /// Loopback
    Loopback,
    /// Unknown
    Unknown,
}

impl fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceType::Ethernet => write!(f, "ethernet"),
            InterfaceType::WiFi => write!(f, "wifi"),
            InterfaceType::Loopback => write!(f, "loopback"),
            InterfaceType::Unknown => write!(f, "unknown"),
        }
    }
}

/// Network interface status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceStatus {
    /// Down
    Down,
    /// Up
    Up,
    /// Connected
    Connected,
    /// Disconnected
    Disconnected,
}

impl fmt::Display for InterfaceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceStatus::Down => write!(f, "down"),
            InterfaceStatus::Up => write!(f, "up"),
            InterfaceStatus::Connected => write!(f, "connected"),
            InterfaceStatus::Disconnected => write!(f, "disconnected"),
        }
    }
}

/// IP address type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpType {
    /// IPv4
    IPv4,
    /// IPv6
    IPv6,
}

impl fmt::Display for IpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpType::IPv4 => write!(f, "ipv4"),
            IpType::IPv6 => write!(f, "ipv6"),
        }
    }
}

/// IP configuration method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpMethod {
    /// Static
    Static,
    /// DHCP
    DHCP,
    /// Auto
    Auto,
}

impl fmt::Display for IpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpMethod::Static => write!(f, "static"),
            IpMethod::DHCP => write!(f, "dhcp"),
            IpMethod::Auto => write!(f, "auto"),
        }
    }
}

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    /// Interface name
    pub name: String,
    /// Interface type
    pub interface_type: InterfaceType,
    /// Interface status
    pub status: InterfaceStatus,
    /// MAC address
    pub mac_address: String,
    /// IP address
    pub ip_address: Option<String>,
    /// IP type
    pub ip_type: Option<IpType>,
    /// IP method
    pub ip_method: IpMethod,
    /// Gateway
    pub gateway: Option<String>,
    /// DNS servers
    pub dns_servers: Vec<String>,
}

impl NetworkInterface {
    /// Create a new network interface
    pub fn new(name: String, interface_type: InterfaceType, mac_address: String) -> Self {
        Self {
            name,
            interface_type,
            status: InterfaceStatus::Down,
            mac_address,
            ip_address: None,
            ip_type: None,
            ip_method: IpMethod::DHCP,
            gateway: None,
            dns_servers: Vec::new(),
        }
    }

    /// Set status
    pub fn with_status(mut self, status: InterfaceStatus) -> Self {
        self.status = status;
        self
    }

    /// Set IP address
    pub fn with_ip(mut self, ip_address: String, ip_type: IpType) -> Self {
        self.ip_address = Some(ip_address);
        self.ip_type = Some(ip_type);
        self
    }

    /// Set IP method
    pub fn with_ip_method(mut self, ip_method: IpMethod) -> Self {
        self.ip_method = ip_method;
        self
    }

    /// Set gateway
    pub fn with_gateway(mut self, gateway: String) -> Self {
        self.gateway = Some(gateway);
        self
    }

    /// Add DNS server
    pub fn with_dns(mut self, dns: String) -> Self {
        self.dns_servers.push(dns);
        self
    }
}

/// SigmaNet - Network Configuration Daemon
pub struct SigmaNet {
    /// Network interfaces (name -> interface)
    interfaces: BTreeMap<String, NetworkInterface>,
    /// Running state
    running: bool,
    /// Interface counter
    interface_counter: u32,
}

impl SigmaNet {
    /// Create a new SigmaNet instance
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            running: false,
            interface_counter: 0,
        }
    }

    /// Start the network daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the network daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add an interface
    pub fn add_interface(&mut self, interface: NetworkInterface) -> Result<(), String> {
        if self.interfaces.contains_key(&interface.name) {
            return Err(format!("Interface '{}' already exists", interface.name));
        }
        
        self.interfaces.insert(interface.name.clone(), interface);
        self.interface_counter += 1;
        Ok(())
    }

    /// Remove an interface
    pub fn remove_interface(&mut self, name: &str) -> Result<(), String> {
        if self.interfaces.remove(name).is_none() {
            return Err(format!("Interface '{}' not found", name));
        }
        Ok(())
    }

    /// Get an interface
    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    /// Bring up an interface
    pub fn bring_up(&mut self, name: &str) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.status = InterfaceStatus::Up;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Bring down an interface
    pub fn bring_down(&mut self, name: &str) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.status = InterfaceStatus::Down;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Connect an interface
    pub fn connect(&mut self, name: &str) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.status = InterfaceStatus::Connected;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Disconnect an interface
    pub fn disconnect(&mut self, name: &str) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.status = InterfaceStatus::Disconnected;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Set IP configuration
    pub fn set_ip(&mut self, name: &str, ip_address: String, ip_type: IpType) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.ip_address = Some(ip_address);
                interface.ip_type = Some(ip_type);
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Set IP method
    pub fn set_ip_method(&mut self, name: &str, ip_method: IpMethod) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.ip_method = ip_method;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Set gateway
    pub fn set_gateway(&mut self, name: &str, gateway: String) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.gateway = Some(gateway);
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Add DNS server
    pub fn add_dns(&mut self, name: &str, dns: String) -> Result<(), String> {
        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.dns_servers.push(dns);
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// List all interfaces
    pub fn list_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values().collect()
    }

    /// List interfaces by type
    pub fn list_by_type(&self, interface_type: InterfaceType) -> Vec<&NetworkInterface> {
        self.interfaces.values()
            .filter(|i| i.interface_type == interface_type)
            .collect()
    }

    /// List connected interfaces
    pub fn list_connected(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values()
            .filter(|i| i.status == InterfaceStatus::Connected)
            .collect()
    }

    /// Get interface count
    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    /// Get network statistics
    pub fn network_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_interfaces".to_string(), self.interface_count().to_string());
        stats.insert("connected_interfaces".to_string(), self.list_connected().len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaNet {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNet instance (placeholder for actual global state)
static mut SIGMANET_INSTANCE: Option<SigmaNet> = None;

/// Initialize the global SigmaNet instance
pub fn init_sigmanet() {
    unsafe {
        SIGMANET_INSTANCE = Some(SigmaNet::new());
    }
}

/// Get the global SigmaNet instance
pub fn get_sigmanet() -> Option<&'static mut SigmaNet> {
    unsafe { SIGMANET_INSTANCE.as_mut() }
}

/// Convenience function to bring up an interface
pub fn bring_up_interface(name: &str) -> Result<(), String> {
    if let Some(net) = get_sigmanet() {
        net.bring_up(name)
    } else {
        Err("SigmaNet not initialized".to_string())
    }
}

/// Convenience function to connect an interface
pub fn connect_interface(name: &str) -> Result<(), String> {
    if let Some(net) = get_sigmanet() {
        net.connect(name)
    } else {
        Err("SigmaNet not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_type_display() {
        assert_eq!(InterfaceType::Ethernet.to_string(), "ethernet");
        assert_eq!(InterfaceType::WiFi.to_string(), "wifi");
    }

    #[test]
    fn test_interface_status_display() {
        assert_eq!(InterfaceStatus::Up.to_string(), "up");
        assert_eq!(InterfaceStatus::Connected.to_string(), "connected");
    }

    #[test]
    fn test_ip_type_display() {
        assert_eq!(IpType::IPv4.to_string(), "ipv4");
        assert_eq!(IpType::IPv6.to_string(), "ipv6");
    }

    #[test]
    fn test_ip_method_display() {
        assert_eq!(IpMethod::Static.to_string(), "static");
        assert_eq!(IpMethod::DHCP.to_string(), "dhcp");
    }

    #[test]
    fn test_network_interface() {
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        )
        .with_status(InterfaceStatus::Connected)
        .with_ip("192.168.1.100".to_string(), IpType::IPv4)
        .with_gateway("192.168.1.1".to_string())
        .with_dns("8.8.8.8".to_string());

        assert_eq!(interface.name, "eth0");
        assert_eq!(interface.status, InterfaceStatus::Connected);
        assert_eq!(interface.ip_address, Some("192.168.1.100".to_string()));
    }

    #[test]
    fn test_sigmanet() {
        let mut net = SigmaNet::new();
        assert!(!net.is_running());
        net.start();
        assert!(net.is_running());
        net.stop();
        assert!(!net.is_running());
    }

    #[test]
    fn test_sigmanet_add_interface() {
        let mut net = SigmaNet::new();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        assert!(net.add_interface(interface).is_ok());
        assert_eq!(net.interface_count(), 1);
    }

    #[test]
    fn test_sigmanet_bring_up() {
        let mut net = SigmaNet::new();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        net.add_interface(interface).unwrap();
        assert!(net.bring_up("eth0").is_ok());
        
        let interface = net.get_interface("eth0").unwrap();
        assert_eq!(interface.status, InterfaceStatus::Up);
    }

    #[test]
    fn test_sigmanet_connect() {
        let mut net = SigmaNet::new();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        net.add_interface(interface).unwrap();
        assert!(net.connect("eth0").is_ok());
        
        let interface = net.get_interface("eth0").unwrap();
        assert_eq!(interface.status, InterfaceStatus::Connected);
    }

    #[test]
    fn test_sigmanet_set_ip() {
        let mut net = SigmaNet::new();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        net.add_interface(interface).unwrap();
        assert!(net.set_ip("eth0", "192.168.1.100".to_string(), IpType::IPv4).is_ok());
        
        let interface = net.get_interface("eth0").unwrap();
        assert_eq!(interface.ip_address, Some("192.168.1.100".to_string()));
    }

    #[test]
    fn test_sigmanet_list_by_type() {
        let mut net = SigmaNet::new();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        net.add_interface(interface).unwrap();
        
        let interfaces = net.list_by_type(InterfaceType::Ethernet);
        assert_eq!(interfaces.len(), 1);
    }

    #[test]
    fn test_sigmanet_stats() {
        let mut net = SigmaNet::new();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        )
        .with_status(InterfaceStatus::Connected);
        
        net.add_interface(interface).unwrap();
        
        let stats = net.network_stats();
        assert_eq!(stats.get("total_interfaces"), Some(&"1".to_string()));
        assert_eq!(stats.get("connected_interfaces"), Some(&"1".to_string()));
    }

    #[test]
    fn test_bring_up_interface_convenience() {
        init_sigmanet();
        
        let mut net = get_sigmanet().unwrap();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        net.add_interface(interface).unwrap();
        
        let result = bring_up_interface("eth0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_connect_interface_convenience() {
        init_sigmanet();
        
        let mut net = get_sigmanet().unwrap();
        
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            InterfaceType::Ethernet,
            "00:11:22:33:44:55".to_string()
        );
        
        net.add_interface(interface).unwrap();
        
        let result = connect_interface("eth0");
        assert!(result.is_ok());
    }
}
