#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Network monitor for network traffic monitoring
/// Inspired by Linux tools like iftop, nethogs, and tcpdump
#[derive(Debug, Clone)]
pub struct NetworkMonitor {
    interfaces: Vec<NetworkMonitorInterface>,
    connections: Vec<NetworkMonitorConnection>,
}

/// Network interface statistics
#[derive(Debug, Clone)]
pub struct NetworkMonitorInterface {
    name: String,
    bytes_sent: u64,
    bytes_received: u64,
    packets_sent: u64,
    packets_received: u64,
    state: NetworkMonitorInterfaceState,
}

/// Network connection
#[derive(Debug, Clone)]
pub struct NetworkMonitorConnection {
    id: String,
    local_address: String,
    remote_address: String,
    protocol: ConnectionProtocol,
    state: NetworkMonitorConnectionState,
    bytes_sent: u64,
    bytes_received: u64,
}

/// Connection protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionProtocol {
    Tcp,
    Udp,
    Icmp,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMonitorConnectionState {
    Established,
    Listening,
    TimeWait,
    CloseWait,
    Closed,
}

/// Interface state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMonitorInterfaceState {
    Up,
    Down,
    Monitoring,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Add an interface to monitor
    pub fn add_interface(&mut self, name: String) -> Result<String, String> {
        if self.interfaces.iter().any(|i| i.name == name) {
            return Err(format!("Interface {} already exists", name));
        }
        
        let interface = NetworkMonitorInterface {
            name: name.clone(),
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            state: NetworkMonitorInterfaceState::Monitoring,
        };
        self.interfaces.push(interface);
        Ok(name)
    }

    /// Remove an interface
    pub fn remove_interface(&mut self, name: &str) -> Result<(), String> {
        let pos = self.interfaces.iter().position(|i| i.name == name)
            .ok_or_else(|| format!("Interface {} not found", name))?;
        self.interfaces.remove(pos);
        Ok(())
    }

    /// Update interface statistics
    pub fn update_interface_stats(&mut self, name: &str, bytes_sent: u64, bytes_received: u64, packets_sent: u64, packets_received: u64) -> Result<(), String> {
        let interface = self.interfaces.iter_mut().find(|i| i.name == name)
            .ok_or_else(|| format!("Interface {} not found", name))?;
        interface.bytes_sent = bytes_sent;
        interface.bytes_received = bytes_received;
        interface.packets_sent = packets_sent;
        interface.packets_received = packets_received;
        Ok(())
    }

    /// Set interface state
    pub fn set_interface_state(&mut self, name: &str, state: NetworkMonitorInterfaceState) -> Result<(), String> {
        let interface = self.interfaces.iter_mut().find(|i| i.name == name)
            .ok_or_else(|| format!("Interface {} not found", name))?;
        interface.state = state;
        Ok(())
    }

    /// Add a connection
    pub fn add_connection(&mut self, id: String, local_address: String, remote_address: String, protocol: ConnectionProtocol) -> Result<String, String> {
        if self.connections.iter().any(|c| c.id == id) {
            return Err(format!("Connection with id {} already exists", id));
        }
        
        let connection = NetworkMonitorConnection {
            id: id.clone(),
            local_address,
            remote_address,
            protocol,
            state: NetworkMonitorConnectionState::Established,
            bytes_sent: 0,
            bytes_received: 0,
        };
        self.connections.push(connection);
        Ok(id)
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, id: &str) -> Result<(), String> {
        let pos = self.connections.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Connection with id {} not found", id))?;
        self.connections.remove(pos);
        Ok(())
    }

    /// Update connection statistics
    pub fn update_connection_stats(&mut self, id: &str, bytes_sent: u64, bytes_received: u64) -> Result<(), String> {
        let connection = self.connections.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Connection with id {} not found", id))?;
        connection.bytes_sent = bytes_sent;
        connection.bytes_received = bytes_received;
        Ok(())
    }

    /// Set connection state
    pub fn set_connection_state(&mut self, id: &str, state: NetworkMonitorConnectionState) -> Result<(), String> {
        let connection = self.connections.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Connection with id {} not found", id))?;
        connection.state = state;
        Ok(())
    }

    /// Get all interfaces
    pub fn get_interfaces(&self) -> &[NetworkMonitorInterface] {
        &self.interfaces
    }

    /// Get monitoring interfaces
    pub fn get_monitoring_interfaces(&self) -> Vec<&NetworkMonitorInterface> {
        self.interfaces.iter().filter(|i| i.state == NetworkMonitorInterfaceState::Monitoring).collect()
    }

    /// Get interface by name
    pub fn get_interface(&self, name: &str) -> Option<&NetworkMonitorInterface> {
        self.interfaces.iter().find(|i| i.name == name)
    }

    /// Get all connections
    pub fn get_connections(&self) -> &[NetworkMonitorConnection] {
        &self.connections
    }

    /// Get connections by protocol
    pub fn get_connections_by_protocol(&self, protocol: ConnectionProtocol) -> Vec<&NetworkMonitorConnection> {
        self.connections.iter().filter(|c| c.protocol == protocol).collect()
    }

    /// Get connection by ID
    pub fn get_connection(&self, id: &str) -> Option<&NetworkMonitorConnection> {
        self.connections.iter().find(|c| c.id == id)
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_interface() {
        let mut monitor = NetworkMonitor::new();
        let result = monitor.add_interface("eth0".to_string());
        assert!(result.is_ok());
        assert_eq!(monitor.get_interfaces().len(), 1);
    }

    #[test]
    fn test_duplicate_interface() {
        let mut monitor = NetworkMonitor::new();
        monitor.add_interface("eth0".to_string()).unwrap();
        let result = monitor.add_interface("eth0".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_update_interface_stats() {
        let mut monitor = NetworkMonitor::new();
        monitor.add_interface("eth0".to_string()).unwrap();
        
        monitor.update_interface_stats("eth0", 1000, 2000, 100, 200).unwrap();
        assert_eq!(monitor.get_interface("eth0").unwrap().bytes_sent, 1000);
        assert_eq!(monitor.get_interface("eth0").unwrap().bytes_received, 2000);
    }

    #[test]
    fn test_add_connection() {
        let mut monitor = NetworkMonitor::new();
        let result = monitor.add_connection("conn1".to_string(), "192.168.1.1:1234".to_string(), "10.0.0.1:80".to_string(), ConnectionProtocol::Tcp);
        assert!(result.is_ok());
        assert_eq!(monitor.get_connections().len(), 1);
    }

    #[test]
    fn test_update_connection_stats() {
        let mut monitor = NetworkMonitor::new();
        monitor.add_connection("conn1".to_string(), "192.168.1.1:1234".to_string(), "10.0.0.1:80".to_string(), ConnectionProtocol::Tcp).unwrap();
        
        monitor.update_connection_stats("conn1", 500, 1000).unwrap();
        assert_eq!(monitor.get_connection("conn1").unwrap().bytes_sent, 500);
        assert_eq!(monitor.get_connection("conn1").unwrap().bytes_received, 1000);
    }
}
