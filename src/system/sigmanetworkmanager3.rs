// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Network interface state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
    Connected,
    Disconnected,
}

/// Network interface type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    WiFi,
    Bluetooth,
    Virtual,
}

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub interface_type: InterfaceType,
    pub state: InterfaceState,
    pub is_default: bool,
}

/// Network manager for managing network interfaces
/// Inspired by Omarchy's hw- commands and NetworkManager
#[derive(Debug, Clone)]
pub struct SigmaNetworkManager3 {
    interfaces: Vec<NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
        }
    }

    /// Register a network interface
    pub fn register_interface(&mut self, interface: NetworkInterface) {
        self.interfaces.push(interface);
    }

    /// Get interface by ID
    pub fn get_interface(&self, id: &str) -> Option<&NetworkInterface> {
        self.interfaces.iter().find(|i| i.id == id)
    }

    /// Get all interfaces
    pub fn get_interfaces(&self) -> &[NetworkInterface] {
        &self.interfaces
    }

    /// Bring interface up
    pub fn bring_up(&mut self, id: &str) -> Result<String, String> {
        if let Some(interface) = self.interfaces.iter_mut().find(|i| i.id == id) {
            interface.state = InterfaceState::Up;
            Ok(format!("Brought up interface: {}", interface.name))
        } else {
            Err(format!("Interface not found: {}", id))
        }
    }

    /// Bring interface down
    pub fn bring_down(&mut self, id: &str) -> Result<String, String> {
        if let Some(interface) = self.interfaces.iter_mut().find(|i| i.id == id) {
            interface.state = InterfaceState::Down;
            Ok(format!("Brought down interface: {}", interface.name))
        } else {
            Err(format!("Interface not found: {}", id))
        }
    }

    /// Connect interface
    pub fn connect(&mut self, id: &str, ip: &str) -> Result<String, String> {
        if let Some(interface) = self.interfaces.iter_mut().find(|i| i.id == id) {
            interface.ip_address = Some(ip.to_string());
            interface.state = InterfaceState::Connected;
            Ok(format!("Connected interface: {}", interface.name))
        } else {
            Err(format!("Interface not found: {}", id))
        }
    }

    /// Disconnect interface
    pub fn disconnect(&mut self, id: &str) -> Result<String, String> {
        if let Some(interface) = self.interfaces.iter_mut().find(|i| i.id == id) {
            interface.ip_address = None;
            interface.state = InterfaceState::Disconnected;
            Ok(format!("Disconnected interface: {}", interface.name))
        } else {
            Err(format!("Interface not found: {}", id))
        }
    }

    /// Set as default interface
    pub fn set_default(&mut self, id: &str) -> Result<String, String> {
        for interface in &mut self.interfaces {
            interface.is_default = false;
        }
        if let Some(interface) = self.interfaces.iter_mut().find(|i| i.id == id) {
            interface.is_default = true;
            Ok(format!("Set default interface: {}", interface.name))
        } else {
            Err(format!("Interface not found: {}", id))
        }
    }

    /// Get connected interfaces
    pub fn get_connected_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces
            .iter()
            .filter(|i| i.state == InterfaceState::Connected)
            .collect()
    }

    /// Get default interface
    pub fn get_default_interface(&self) -> Option<&NetworkInterface> {
        self.interfaces.iter().find(|i| i.is_default)
    }

    /// Get interfaces by type
    pub fn get_interfaces_by_type(&self, interface_type: InterfaceType) -> Vec<&NetworkInterface> {
        self.interfaces
            .iter()
            .filter(|i| i.interface_type == interface_type)
            .collect()
    }
}

impl Default for SigmaNetworkManager3 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_interface() {
        let mut manager = SigmaNetworkManager3::new();
        let interface = NetworkInterface {
            id: "eth0".to_string(),
            name: "Ethernet 0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: None,
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Down,
            is_default: false,
        };
        manager.register_interface(interface);
        assert_eq!(manager.get_interfaces().len(), 1);
    }

    #[test]
    fn test_bring_up() {
        let mut manager = SigmaNetworkManager3::new();
        let interface = NetworkInterface {
            id: "eth0".to_string(),
            name: "Ethernet 0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: None,
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Down,
            is_default: false,
        };
        manager.register_interface(interface);
        manager.bring_up("eth0").unwrap();
        assert_eq!(
            manager.get_interface("eth0").unwrap().state,
            InterfaceState::Up
        );
    }

    #[test]
    fn test_connect() {
        let mut manager = SigmaNetworkManager3::new();
        let interface = NetworkInterface {
            id: "eth0".to_string(),
            name: "Ethernet 0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: None,
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Up,
            is_default: false,
        };
        manager.register_interface(interface);
        manager.connect("eth0", "192.168.1.100").unwrap();
        assert_eq!(
            manager.get_interface("eth0").unwrap().state,
            InterfaceState::Connected
        );
        assert_eq!(
            manager.get_interface("eth0").unwrap().ip_address,
            Some("192.168.1.100".to_string())
        );
    }

    #[test]
    fn test_set_default() {
        let mut manager = SigmaNetworkManager3::new();
        let interface = NetworkInterface {
            id: "eth0".to_string(),
            name: "Ethernet 0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: None,
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Connected,
            is_default: false,
        };
        manager.register_interface(interface);
        manager.set_default("eth0").unwrap();
        assert!(manager.get_interface("eth0").unwrap().is_default);
    }

    #[test]
    fn test_get_connected_interfaces() {
        let mut manager = SigmaNetworkManager3::new();
        let interface1 = NetworkInterface {
            id: "eth0".to_string(),
            name: "Ethernet 0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: Some("192.168.1.100".to_string()),
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Connected,
            is_default: false,
        };
        let interface2 = NetworkInterface {
            id: "eth1".to_string(),
            name: "Ethernet 1".to_string(),
            mac_address: "00:11:22:33:44:56".to_string(),
            ip_address: None,
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Down,
            is_default: false,
        };
        manager.register_interface(interface1);
        manager.register_interface(interface2);
        assert_eq!(manager.get_connected_interfaces().len(), 1);
    }

    #[test]
    fn test_get_interfaces_by_type() {
        let mut manager = SigmaNetworkManager3::new();
        let eth = NetworkInterface {
            id: "eth0".to_string(),
            name: "Ethernet 0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: None,
            interface_type: InterfaceType::Ethernet,
            state: InterfaceState::Down,
            is_default: false,
        };
        let wifi = NetworkInterface {
            id: "wlan0".to_string(),
            name: "WiFi 0".to_string(),
            mac_address: "00:11:22:33:44:56".to_string(),
            ip_address: None,
            interface_type: InterfaceType::WiFi,
            state: InterfaceState::Down,
            is_default: false,
        };
        manager.register_interface(eth);
        manager.register_interface(wifi);
        assert_eq!(manager.get_interfaces_by_type(InterfaceType::WiFi).len(), 1);
    }
}
