//! SigmaNetwork - Network Configuration Management
//!
//! Provides comprehensive network configuration inspired by Linux NetworkManager and BSD networking.
//! Supports network interfaces, connections, and DNS configuration.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Interface state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    Up,
    Down,
    Unknown,
;

/// Connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    WiFi,
    VPN,
    Unknown,
;

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String,
    pub state: InterfaceState,
    pub ip_address: Option<String>,
    pub netmask: Option<String>,
    pub gateway: Option<String>,
    pub mac_address: Option<String>,
;

impl NetworkInterface {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            state: InterfaceState::Down,
            ip_address: None,
            netmask: None,
            gateway: None,
            mac_address: None,
        }
    }

    pub fn with_ip(mut self, ip: String, netmask: String) -> Self {
        self.ip_address = Some(ip);
        self.netmask = Some(netmask);
        self
    }

    pub fn with_gateway(mut self, gateway: String) -> Self {
        self.gateway = Some(gateway);
        self
    }

    pub fn with_mac(mut self, mac: String) -> Self {
        self.mac_address = Some(mac);
        self
    }

    pub fn up(&mut self) {
        self.state = InterfaceState::Up;
    }

    pub fn down(&mut self) {
        self.state = InterfaceState::Down;
    }
;

/// Network connection
#[derive(Debug, Clone)]
pub struct NetworkConnection {
    pub id: String,
    pub name: String,
    pub connection_type: ConnectionType,
    pub auto_connect: bool,
    pub active: bool,
;

impl NetworkConnection {
    pub fn new(id: String, name: String, connection_type: ConnectionType) -> Self {
        Self {
            id,
            name,
            connection_type,
            auto_connect: false,
            active: false,
        }
    }

    pub fn with_auto_connect(mut self, auto: bool) -> Self {
        self.auto_connect = auto;
        self
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
;

/// Network manager
pub struct SigmaNetwork {
    interfaces: BTreeMap<String, NetworkInterface>,
    connections: BTreeMap<String, NetworkConnection>,
    next_interface_id: u64,
    next_connection_id: u64,
    dns_servers: Vec<String>,
;

impl SigmaNetwork {
    pub fn new() -> Self {
        Self {
            interfaces: BTreeMap::new(),
            connections: BTreeMap::new(),
            next_interface_id: 1,
            next_connection_id: 1,
            dns_servers: Vec::new(),
        }
    }

    pub fn add_interface(&mut self, name: String) -> String {
        let id = format!("iface-{}", self.next_interface_id);
        self.next_interface_id += 1;
        let iface = NetworkInterface::new(id.clone(), name);
        self.interfaces.insert(id.clone(), iface);
        id
    }

    pub fn get_interface(&self, id: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(id)
    }

    pub fn get_interface_mut(&mut self, id: &str) -> Option<&mut NetworkInterface> {
        self.interfaces.get_mut(id)
    }

    pub fn list_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values().collect()
    }

    pub fn remove_interface(&mut self, id: &str) -> bool {
        self.interfaces.remove(id).is_some()
    }

    pub fn add_connection(&mut self, name: String, connection_type: ConnectionType) -> String {
        let id = format!("conn-{}", self.next_connection_id);
        self.next_connection_id += 1;
        let conn = NetworkConnection::new(id.clone(), name, connection_type);
        self.connections.insert(id.clone(), conn);
        id
    }

    pub fn get_connection(&self, id: &str) -> Option<&NetworkConnection> {
        self.connections.get(id)
    }

    pub fn get_connection_mut(&mut self, id: &str) -> Option<&mut NetworkConnection> {
        self.connections.get_mut(id)
    }

    pub fn list_connections(&self) -> Vec<&NetworkConnection> {
        self.connections.values().collect()
    }

    pub fn remove_connection(&mut self, id: &str) -> bool {
        self.connections.remove(id).is_some()
    }

    pub fn add_dns_server(&mut self, server: String) {
        self.dns_servers.push(server);
    }

    pub fn get_dns_servers(&self) -> &[String] {
        &self.dns_servers
    }

    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
;

impl Default for SigmaNetwork {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMANETWORK_INSTANCE: Option<SigmaNetwork> = None;

pub fn init_sigmanetwork() {
    unsafe {
        SIGMANETWORK_INSTANCE = Some(SigmaNetwork::new());
    }
;

pub fn get_sigmanetwork() -> &'static SigmaNetwork {
    unsafe {
        SIGMANETWORK_INSTANCE
            .as_ref()
            .expect("SigmaNetwork not initialized")
    }
;

pub fn get_sigmanetwork_mut() -> &'static mut SigmaNetwork {
    unsafe {
        SIGMANETWORK_INSTANCE
            .as_mut()
            .expect("SigmaNetwork not initialized")
    }
;

pub fn add_interface(name: String) -> String {
    let manager = get_sigmanetwork_mut();
    manager.add_interface(name)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_creation() {
        let iface = NetworkInterface::new("iface-1".to_string(), "eth0".to_string());
        assert_eq!(iface.id, "iface-1");
        assert_eq!(iface.name, "eth0");
        assert_eq!(iface.state, InterfaceState::Down);
    }

    #[test]
    fn test_interface_builder() {
        let iface = NetworkInterface::new("iface-1".to_string(), "eth0".to_string())
            .with_ip("192.168.1.100".to_string(), "255.255.255.0".to_string())
            .with_gateway("192.168.1.1".to_string())
            .with_mac("00:11:22:33:44:55".to_string());

        assert_eq!(iface.ip_address, Some("192.168.1.100".to_string()));
        assert_eq!(iface.gateway, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_interface_state() {
        let mut iface = NetworkInterface::new("iface-1".to_string(), "eth0".to_string());
        assert_eq!(iface.state, InterfaceState::Down);

        iface.up();
        assert_eq!(iface.state, InterfaceState::Up);

        iface.down();
        assert_eq!(iface.state, InterfaceState::Down);
    }

    #[test]
    fn test_connection_creation() {
        let conn = NetworkConnection::new(
            "conn-1".to_string(),
            "HomeWiFi".to_string(),
            ConnectionType::WiFi,
        );
        assert_eq!(conn.id, "conn-1");
        assert_eq!(conn.name, "HomeWiFi");
        assert_eq!(conn.connection_type, ConnectionType::WiFi);
    }

    #[test]
    fn test_connection_state() {
        let mut conn = NetworkConnection::new(
            "conn-1".to_string(),
            "HomeWiFi".to_string(),
            ConnectionType::WiFi,
        );
        assert!(!conn.active);

        conn.activate();
        assert!(conn.active);

        conn.deactivate();
        assert!(!conn.active);
    }

    #[test]
    fn test_network_manager() {
        let manager = SigmaNetwork::new();
        assert_eq!(manager.interface_count(), 0);
        assert_eq!(manager.connection_count(), 0);
    }

    #[test]
    fn test_add_interface() {
        let mut manager = SigmaNetwork::new();
        let id = manager.add_interface("eth0".to_string());
        assert_eq!(id, "iface-1");
        assert_eq!(manager.interface_count(), 1);
    }

    #[test]
    fn test_add_connection() {
        let mut manager = SigmaNetwork::new();
        let id = manager.add_connection("HomeWiFi".to_string(), ConnectionType::WiFi);
        assert_eq!(id, "conn-1");
        assert_eq!(manager.connection_count(), 1);
    }

    #[test]
    fn test_dns_servers() {
        let mut manager = SigmaNetwork::new();
        manager.add_dns_server("8.8.8.8".to_string());
        manager.add_dns_server("8.8.4.4".to_string());

        assert_eq!(manager.get_dns_servers().len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmanetwork();
        let manager = get_sigmanetwork();
        assert_eq!(manager.interface_count(), 0);

        let manager_mut = get_sigmanetwork_mut();
        manager_mut.add_interface("eth0".to_string());
        assert_eq!(manager.interface_count(), 1);
    }

    #[test]
    fn test_add_interface_convenience() {
        init_sigmanetwork();
        let id = add_interface("eth0".to_string());
        assert_eq!(id, "iface-1");

        let manager = get_sigmanetwork();
        assert!(manager.get_interface(&id).is_some());
    }
;
