// SPDX-License-Identifier: MIT
// SigmaOS - Network Manager
// Inspired by NetworkManager for Linux
// Provides network connection management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    WiFi,
    Bluetooth,
    VPN,
    Bridge,
    Bond,
    VLAN,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Unknown,
    Activating,
    Activated,
    Deactivating,
    Deactivated,
    Failed,
}

/// IP configuration method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPMethod {
    Auto,
    Manual,
    LinkLocal,
    Disabled,
}

/// WiFi security type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WiFiSecurity {
    Open,
    WEP,
    WPA_PSK,
    WPA_EAP,
    WPA2_PSK,
    WPA2_EAP,
    WPA3_PSK,
    WPA3_EAP,
}

/// IP address
#[derive(Debug, Clone)]
pub struct IPAddress {
    pub address: String,
    pub prefix: u8,
    pub gateway: Option<String>,
}

/// DNS server
#[derive(Debug, Clone)]
pub struct DNSServer {
    pub address: String,
    pub search_domains: Vec<String>,
}

/// Connection
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: u32,
    pub name: String,
    pub connection_type: ConnectionType,
    pub state: ConnectionState,
    pub auto_connect: bool,
    pub ip_method: IPMethod,
    pub ip_address: Option<IPAddress>,
    pub dns_servers: Vec<DNSServer>,
    pub wifi_security: Option<WiFiSecurity>,
    pub wifi_ssid: Option<String>,
}

/// Network device
#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub id: u32,
    pub name: String,
    pub device_type: ConnectionType,
    pub state: ConnectionState,
    pub mac_address: String,
    pub is_up: bool,
    pub mtu: u32,
}

/// Network Manager
pub struct NetworkManager {
    pub connections: BTreeMap<u32, Connection>,
    pub devices: BTreeMap<u32, NetworkDevice>,
    pub active_connection: Option<u32>,
    next_connection_id: AtomicU32,
    next_device_id: AtomicU32,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            devices: BTreeMap::new(),
            active_connection: None,
            next_connection_id: AtomicU32::new(1),
            next_device_id: AtomicU32::new(1),
        }
    }

    /// Add connection
    pub fn add_connection(&mut self, name: String, connection_type: ConnectionType) -> u32 {
        let id = self.next_connection_id.fetch_add(1, Ordering::Relaxed);
        let connection = Connection {
            id,
            name,
            connection_type,
            state: ConnectionState::Deactivated,
            auto_connect: false,
            ip_method: IPMethod::Auto,
            ip_address: None,
            dns_servers: Vec::new(),
            wifi_security: None,
            wifi_ssid: None,
        };
        self.connections.insert(id, connection);
        id
    }

    /// Remove connection
    pub fn remove_connection(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.connections.contains_key(&id) {
            return Err("Connection not found");
        }
        if self.active_connection == Some(id) {
            self.active_connection = None;
        }
        self.connections.remove(&id);
        Ok(())
    }

    /// Activate connection
    pub fn activate_connection(&mut self, id: u32) -> Result<(), &'static str> {
        let connection = self
            .connections
            .get_mut(&id)
            .ok_or("Connection not found")?;
        connection.state = ConnectionState::Activating;
        connection.state = ConnectionState::Activated;
        self.active_connection = Some(id);
        Ok(())
    }

    /// Deactivate connection
    pub fn deactivate_connection(&mut self, id: u32) -> Result<(), &'static str> {
        let connection = self
            .connections
            .get_mut(&id)
            .ok_or("Connection not found")?;
        connection.state = ConnectionState::Deactivating;
        connection.state = ConnectionState::Deactivated;
        if self.active_connection == Some(id) {
            self.active_connection = None;
        }
        Ok(())
    }

    /// Set IP configuration
    pub fn set_ip_config(
        &mut self,
        id: u32,
        ip_method: IPMethod,
        ip_address: Option<IPAddress>,
    ) -> Result<(), &'static str> {
        let connection = self
            .connections
            .get_mut(&id)
            .ok_or("Connection not found")?;
        connection.ip_method = ip_method;
        connection.ip_address = ip_address;
        Ok(())
    }

    /// Set DNS servers
    pub fn set_dns_servers(
        &mut self,
        id: u32,
        dns_servers: Vec<DNSServer>,
    ) -> Result<(), &'static str> {
        let connection = self
            .connections
            .get_mut(&id)
            .ok_or("Connection not found")?;
        connection.dns_servers = dns_servers;
        Ok(())
    }

    /// Set WiFi configuration
    pub fn set_wifi_config(
        &mut self,
        id: u32,
        ssid: String,
        security: WiFiSecurity,
    ) -> Result<(), &'static str> {
        let connection = self
            .connections
            .get_mut(&id)
            .ok_or("Connection not found")?;
        connection.wifi_ssid = Some(ssid);
        connection.wifi_security = Some(security);
        Ok(())
    }

    /// Set auto connect
    pub fn set_auto_connect(&mut self, id: u32, auto_connect: bool) -> Result<(), &'static str> {
        let connection = self
            .connections
            .get_mut(&id)
            .ok_or("Connection not found")?;
        connection.auto_connect = auto_connect;
        Ok(())
    }

    /// Add device
    pub fn add_device(
        &mut self,
        name: String,
        device_type: ConnectionType,
        mac_address: String,
    ) -> u32 {
        let id = self.next_device_id.fetch_add(1, Ordering::Relaxed);
        let device = NetworkDevice {
            id,
            name,
            device_type,
            state: ConnectionState::Deactivated,
            mac_address,
            is_up: false,
            mtu: 1500,
        };
        self.devices.insert(id, device);
        id
    }

    /// Set device state
    pub fn set_device_state(&mut self, id: u32, is_up: bool) -> Result<(), &'static str> {
        let device = self.devices.get_mut(&id).ok_or("Device not found")?;
        device.is_up = is_up;
        device.state = if is_up {
            ConnectionState::Activated
        } else {
            ConnectionState::Deactivated
        };
        Ok(())
    }

    /// Get active connection
    pub fn get_active_connection(&self) -> Option<&Connection> {
        self.active_connection
            .and_then(|id| self.connections.get(&id))
    }

    /// Scan WiFi networks
    pub fn scan_wifi(&self) -> Vec<String> {
        // Simplified - in real implementation would scan actual networks
        Vec::new()
    }

    /// Get connection list
    pub fn list_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }

    /// Get device list
    pub fn list_devices(&self) -> Vec<&NetworkDevice> {
        self.devices.values().collect()
    }

    /// Get connection status
    pub fn get_connection_status(&self, id: u32) -> Result<ConnectionState, &'static str> {
        let connection = self.connections.get(&id).ok_or("Connection not found")?;
        Ok(connection.state)
    }
}

/// Global network manager
static mut NETWORK_MANAGER: Option<NetworkManager> = None;

/// Initialize network manager
pub fn init_network_manager() {
    unsafe {
        NETWORK_MANAGER = Some(NetworkManager::new());
    }
}

/// Get network manager
pub fn get_network_manager() -> &'static mut NetworkManager {
    unsafe {
        NETWORK_MANAGER
            .as_mut()
            .expect("Network manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_manager_creation() {
        let nm = NetworkManager::new();
        assert_eq!(nm.connections.len(), 0);
    }

    #[test]
    fn test_add_connection() {
        let mut nm = NetworkManager::new();
        let id = nm.add_connection("Ethernet".to_string(), ConnectionType::Ethernet);
        assert!(nm.connections.contains_key(&id));
    }

    #[test]
    fn test_activate_connection() {
        let mut nm = NetworkManager::new();
        let id = nm.add_connection("WiFi".to_string(), ConnectionType::WiFi);
        nm.activate_connection(id).unwrap();
        assert_eq!(nm.active_connection, Some(id));
    }

    #[test]
    fn test_deactivate_connection() {
        let mut nm = NetworkManager::new();
        let id = nm.add_connection("WiFi".to_string(), ConnectionType::WiFi);
        nm.activate_connection(id).unwrap();
        nm.deactivate_connection(id).unwrap();
        assert_eq!(nm.active_connection, None);
    }

    #[test]
    fn test_set_ip_config() {
        let mut nm = NetworkManager::new();
        let id = nm.add_connection("Ethernet".to_string(), ConnectionType::Ethernet);
        let ip = IPAddress {
            address: "192.168.1.100".to_string(),
            prefix: 24,
            gateway: Some("192.168.1.1".to_string()),
        };
        nm.set_ip_config(id, IPMethod::Manual, Some(ip)).unwrap();
    }

    #[test]
    fn test_add_device() {
        let mut nm = NetworkManager::new();
        let id = nm.add_device(
            "eth0".to_string(),
            ConnectionType::Ethernet,
            "00:11:22:33:44:55".to_string(),
        );
        assert!(nm.devices.contains_key(&id));
    }
}
