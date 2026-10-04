// Network Configuration Manager for SigmaOS
// Implements network interface configuration with DHCP and static IP support
// Inspired by Linux NetworkManager, systemd-networkd, and BSD network configuration

use crate::klib::HashMap;
use std::net::Ipv4Addr;
use std::string::{String, ToString};
use std::vec::Vec;

/// Network interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Wired,
    Wireless,
    Loopback,
    Virtual,
}

/// Network configuration method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigMethod {
    Dhcp,
    Static,
}

/// Network interface configuration
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub name: String,
    pub if_type: InterfaceType,
    pub method: ConfigMethod,
    pub address: Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
    pub gateway: Option<Ipv4Addr>,
    pub dns_servers: Vec<Ipv4Addr>,
    pub enabled: bool,
}

impl InterfaceConfig {
    pub fn new(name: String, if_type: InterfaceType) -> Self {
        InterfaceConfig {
            name,
            if_type,
            method: ConfigMethod::Dhcp,
            address: None,
            netmask: None,
            gateway: None,
            dns_servers: Vec::new(),
            enabled: false,
        }
    }

    pub fn set_static_ip(&mut self, address: Ipv4Addr, netmask: Ipv4Addr, gateway: Ipv4Addr) {
        self.method = ConfigMethod::Static;
        self.address = Some(address);
        self.netmask = Some(netmask);
        self.gateway = Some(gateway);
    }

    pub fn add_dns_server(&mut self, dns: Ipv4Addr) {
        self.dns_servers.push(dns);
    }

    pub fn set_dhcp(&mut self) {
        self.method = ConfigMethod::Dhcp;
        self.address = None;
        self.netmask = None;
        self.gateway = None;
    }
}

/// Network configuration manager
pub struct NetworkConfigManager {
    pub interfaces: HashMap<String, InterfaceConfig>,
}

impl NetworkConfigManager {
    pub fn new() -> Self {
        let mut manager = NetworkConfigManager {
            interfaces: HashMap::new(),
        };
        manager.initialize_default_interfaces();
        manager
    }

    fn initialize_default_interfaces(&mut self) {
        // Initialize loopback interface
        let lo = InterfaceConfig::new(String::from("lo"), InterfaceType::Loopback);
        self.interfaces.insert(String::from("lo"), lo);

        // Initialize potential wired interface
        let eth0 = InterfaceConfig::new(String::from("eth0"), InterfaceType::Wired);
        self.interfaces.insert(String::from("eth0"), eth0);
    }

    pub fn add_interface(&mut self, config: InterfaceConfig) {
        self.interfaces.insert(config.name.clone(), config);
    }

    pub fn get_interface(&self, name: &str) -> Option<&InterfaceConfig> {
        self.interfaces.get(name)
    }

    pub fn get_interface_mut(&mut self, name: &str) -> Option<&mut InterfaceConfig> {
        self.interfaces.get_mut(name)
    }

    pub fn list_interfaces(&self) -> Vec<String> {
        self.interfaces.keys().cloned().collect()
    }

    pub fn enable_interface(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_interface(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn configure_dhcp(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.set_dhcp();
            true
        } else {
            false
        }
    }

    pub fn configure_static(
        &mut self,
        name: &str,
        address: Ipv4Addr,
        netmask: Ipv4Addr,
        gateway: Ipv4Addr,
    ) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.set_static_ip(address, netmask, gateway);
            true
        } else {
            false
        }
    }

    pub fn add_dns(&mut self, name: &str, dns: Ipv4Addr) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.add_dns_server(dns);
            true
        } else {
            false
        }
    }
}

impl Default for NetworkConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_manager_creation() {
        let manager = NetworkConfigManager::new();
        assert!(manager.get_interface("lo").is_some());
        assert!(manager.get_interface("eth0").is_some());
    }

    #[test]
    fn test_add_interface() {
        let mut manager = NetworkConfigManager::new();
        let wlan0 = InterfaceConfig::new(String::from("wlan0"), InterfaceType::Wireless);
        manager.add_interface(wlan0);
        assert!(manager.get_interface("wlan0").is_some());
    }

    #[test]
    fn test_configure_static_ip() {
        let mut manager = NetworkConfigManager::new();
        let success = manager.configure_static(
            "eth0",
            Ipv4Addr::new(192, 168, 1, 100),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 168, 1, 1),
        );
        assert!(success);

        let iface = manager.get_interface("eth0").unwrap();
        assert_eq!(iface.method, ConfigMethod::Static);
        assert_eq!(iface.address, Some(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(iface.gateway, Some(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn test_configure_dhcp() {
        let mut manager = NetworkConfigManager::new();
        manager.configure_static(
            "eth0",
            Ipv4Addr::new(192, 168, 1, 100),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 168, 1, 1),
        );

        let success = manager.configure_dhcp("eth0");
        assert!(success);

        let iface = manager.get_interface("eth0").unwrap();
        assert_eq!(iface.method, ConfigMethod::Dhcp);
        assert!(iface.address.is_none());
    }

    #[test]
    fn test_add_dns() {
        let mut manager = NetworkConfigManager::new();
        let success = manager.add_dns("eth0", Ipv4Addr::new(8, 8, 8, 8));
        assert!(success);

        let iface = manager.get_interface("eth0").unwrap();
        assert!(iface.dns_servers.contains(&Ipv4Addr::new(8, 8, 8, 8)));
    }

    #[test]
    fn test_enable_disable_interface() {
        let mut manager = NetworkConfigManager::new();
        assert!(!manager.get_interface("eth0").unwrap().enabled);

        manager.enable_interface("eth0");
        assert!(manager.get_interface("eth0").unwrap().enabled);

        manager.disable_interface("eth0");
        assert!(!manager.get_interface("eth0").unwrap().enabled);
    }

    #[test]
    fn test_list_interfaces() {
        let manager = NetworkConfigManager::new();
        let interfaces = manager.list_interfaces();
        assert!(interfaces.contains(&String::from("lo")));
        assert!(interfaces.contains(&String::from("eth0")));
    }
}
