// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaNIC - Network Interface Manager
//!
//! SigmaNIC provides network interface management functionality for SigmaOS, inspired by NetworkManager/ifconfig from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Interface state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    /// Down
    Down,
    /// Up
    Up,
    /// Unknown
    Unknown,
}

impl fmt::Display for InterfaceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceState::Down => write!(f, "down"),
            InterfaceState::Up => write!(f, "up"),
            InterfaceState::Unknown => write!(f, "unknown"),
        }
    }
}

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    /// Interface name
    pub name: String,
    /// MAC address
    pub mac_address: String,
    /// IP address
    pub ip_address: String,
    /// Netmask
    pub netmask: String,
    /// State
    pub state: InterfaceState,
    /// MTU
    pub mtu: u32,
}

impl NetworkInterface {
    /// Create a new network interface
    pub fn new(name: String, mac_address: String, ip_address: String, netmask: String) -> Self {
        Self {
            name,
            mac_address,
            ip_address,
            netmask,
            state: InterfaceState::Down,
            mtu: 1500,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: InterfaceState) -> Self {
        self.state = state;
        self
    }

    /// Set MTU
    pub fn with_mtu(mut self, mtu: u32) -> Self {
        self.mtu = mtu;
        self
    }
}

/// SigmaNIC - Network Interface Manager
pub struct SigmaNIC {
    /// Network interfaces (name -> interface)
    interfaces: BTreeMap<String, NetworkInterface>,
    /// Running state
    running: bool,
    /// Interface counter
    interface_counter: u32,
}

impl SigmaNIC {
    /// Create a new SigmaNIC instance
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            running: false,
            interface_counter: 0,
        }
    }

    /// Start the interface manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the interface manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a network interface
    pub fn add_interface(&mut self, interface: NetworkInterface) -> Result<(), String> {
        if self.interfaces.contains_key(&interface.name) {
            return Err(format!("Interface '{}' already exists", interface.name));
        }

        self.interfaces.insert(interface.name.clone(), interface);
        self.interface_counter += 1;
        Ok(())
    }

    /// Remove a network interface
    pub fn remove_interface(&mut self, name: &str) -> Result<(), String> {
        if self.interfaces.remove(name).is_none() {
            return Err(format!("Interface '{}' not found", name));
        }
        Ok(())
    }

    /// Bring up an interface
    pub fn bring_up(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaNIC not running".to_string());
        }

        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.state = InterfaceState::Up;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Bring down an interface
    pub fn bring_down(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaNIC not running".to_string());
        }

        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.state = InterfaceState::Down;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Set MTU for an interface
    pub fn set_mtu(&mut self, name: &str, mtu: u32) -> Result<(), String> {
        if !self.running {
            return Err("SigmaNIC not running".to_string());
        }

        match self.interfaces.get_mut(name) {
            Some(interface) => {
                interface.mtu = mtu;
                Ok(())
            }
            None => Err(format!("Interface '{}' not found", name)),
        }
    }

    /// Get a network interface
    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    /// List all interfaces
    pub fn list_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values().collect()
    }

    /// List interfaces by state
    pub fn list_by_state(&self, state: InterfaceState) -> Vec<&NetworkInterface> {
        self.interfaces
            .values()
            .filter(|i| i.state == state)
            .collect()
    }

    /// Get interface count
    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    /// Get interface manager statistics
    pub fn nic_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_interfaces".to_string(),
            self.interface_count().to_string(),
        );
        stats.insert(
            "up_interfaces".to_string(),
            self.list_by_state(InterfaceState::Up).len().to_string(),
        );
        stats.insert(
            "down_interfaces".to_string(),
            self.list_by_state(InterfaceState::Down).len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaNIC {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNIC instance (placeholder for actual global state)
static mut SIGMANIC_INSTANCE: Option<SigmaNIC> = None;

/// Initialize the global SigmaNIC instance
pub fn init_sigmanic() {
    unsafe {
        SIGMANIC_INSTANCE = Some(SigmaNIC::new());
    }
}

/// Get the global SigmaNIC instance
pub fn get_sigmanic() -> Option<&'static mut SigmaNIC> {
    unsafe { SIGMANIC_INSTANCE.as_mut() }
}

/// Convenience function to bring up an interface
pub fn bring_up_interface(name: &str) -> Result<(), String> {
    if let Some(nic) = get_sigmanic() {
        nic.bring_up(name)
    } else {
        Err("SigmaNIC not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_state_display() {
        assert_eq!(InterfaceState::Down.to_string(), "down");
        assert_eq!(InterfaceState::Up.to_string(), "up");
    }

    #[test]
    fn test_network_interface() {
        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        )
        .with_state(InterfaceState::Up)
        .with_mtu(9000);

        assert_eq!(interface.name, "eth0");
        assert_eq!(interface.state, InterfaceState::Up);
        assert_eq!(interface.mtu, 9000);
    }

    #[test]
    fn test_sigmanic() {
        let mut nic = SigmaNIC::new();
        assert!(!nic.is_running());
        nic.start();
        assert!(nic.is_running());
        nic.stop();
        assert!(!nic.is_running());
    }

    #[test]
    fn test_sigmanic_add_interface() {
        let mut nic = SigmaNIC::new();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        );

        assert!(nic.add_interface(interface).is_ok());
        assert_eq!(nic.interface_count(), 1);
    }

    #[test]
    fn test_sigmanic_bring_up() {
        let mut nic = SigmaNIC::new();
        nic.start();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        );

        nic.add_interface(interface).unwrap();
        assert!(nic.bring_up("eth0").is_ok());

        let iface = nic.get_interface("eth0").unwrap();
        assert_eq!(iface.state, InterfaceState::Up);
    }

    #[test]
    fn test_sigmanic_bring_down() {
        let mut nic = SigmaNIC::new();
        nic.start();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        )
        .with_state(InterfaceState::Up);

        nic.add_interface(interface).unwrap();
        assert!(nic.bring_down("eth0").is_ok());

        let iface = nic.get_interface("eth0").unwrap();
        assert_eq!(iface.state, InterfaceState::Down);
    }

    #[test]
    fn test_sigmanic_set_mtu() {
        let mut nic = SigmaNIC::new();
        nic.start();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        );

        nic.add_interface(interface).unwrap();
        assert!(nic.set_mtu("eth0", 9000).is_ok());

        let iface = nic.get_interface("eth0").unwrap();
        assert_eq!(iface.mtu, 9000);
    }

    #[test]
    fn test_sigmanic_list_by_state() {
        let mut nic = SigmaNIC::new();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        )
        .with_state(InterfaceState::Up);

        nic.add_interface(interface).unwrap();

        let interfaces = nic.list_by_state(InterfaceState::Up);
        assert_eq!(interfaces.len(), 1);
    }

    #[test]
    fn test_sigmanic_stats() {
        let mut nic = SigmaNIC::new();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        )
        .with_state(InterfaceState::Up);

        nic.add_interface(interface).unwrap();

        let stats = nic.nic_stats();
        assert_eq!(stats.get("total_interfaces"), Some(&"1".to_string()));
        assert_eq!(stats.get("up_interfaces"), Some(&"1".to_string()));
    }

    #[test]
    fn test_bring_up_interface_convenience() {
        init_sigmanic();

        let mut nic = get_sigmanic().unwrap();
        nic.start();

        let interface = NetworkInterface::new(
            "eth0".to_string(),
            "00:11:22:33:44:55".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
        );

        nic.add_interface(interface).unwrap();

        let result = bring_up_interface("eth0");
        assert!(result.is_ok());
    }
}
