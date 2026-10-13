// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaProxy - System Proxy Server
//!
//! SigmaProxy provides system proxy server functionality for SigmaOS, inspired by Squid from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Proxy connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyConnectionState {
    /// Connected
    Connected,
    /// Request Received
    RequestReceived,
    /// Forwarding
    Forwarding,
    /// Responding
    Responding,
    /// Closed
    Closed,
}

impl fmt::Display for ProxyConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyConnectionState::Connected => write!(f, "connected"),
            ProxyConnectionState::RequestReceived => write!(f, "request_received"),
            ProxyConnectionState::Forwarding => write!(f, "forwarding"),
            ProxyConnectionState::Responding => write!(f, "responding"),
            ProxyConnectionState::Closed => write!(f, "closed"),
        }
    }
}

/// Proxy protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyProtocol {
    /// HTTP
    Http,
    /// HTTPS
    Https,
    /// FTP
    Ftp,
}

impl fmt::Display for ProxyProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyProtocol::Http => write!(f, "HTTP"),
            ProxyProtocol::Https => write!(f, "HTTPS"),
            ProxyProtocol::Ftp => write!(f, "FTP"),
        }
    }
}

/// Proxy connection
#[derive(Debug, Clone)]
pub struct ProxyConnection {
    /// Connection ID
    pub connection_id: String,
    /// Client address
    pub client_address: String,
    /// Client port
    pub client_port: u16,
    /// Protocol
    pub protocol: ProxyProtocol,
    /// Target host
    pub target_host: String,
    /// Target port
    pub target_port: u16,
    /// State
    pub state: ProxyConnectionState,
}

impl ProxyConnection {
    /// Create a new proxy connection
    pub fn new(connection_id: String, client_address: String, client_port: u16) -> Self {
        Self {
            connection_id,
            client_address,
            client_port,
            protocol: ProxyProtocol::Http,
            target_host: String::new(),
            target_port: 80,
            state: ProxyConnectionState::Connected,
        }
    }

    /// Set protocol
    pub fn with_protocol(mut self, protocol: ProxyProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Set target host
    pub fn with_target_host(mut self, target_host: String) -> Self {
        self.target_host = target_host;
        self
    }

    /// Set target port
    pub fn with_target_port(mut self, target_port: u16) -> Self {
        self.target_port = target_port;
        self
    }

    /// Set state
    pub fn with_state(mut self, state: ProxyConnectionState) -> Self {
        self.state = state;
        self
    }
}

/// SigmaProxy - System Proxy Server
pub struct SigmaProxy {
    /// Proxy connections (connection_id -> connection)
    connections: BTreeMap<String, ProxyConnection>,
    /// Running state
    running: bool,
    /// Connection counter
    connection_counter: u32,
}

impl SigmaProxy {
    /// Create a new SigmaProxy instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            running: false,
            connection_counter: 0,
        }
    }

    /// Start the proxy server
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the proxy server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a proxy connection
    pub fn add_connection(&mut self, connection: ProxyConnection) -> Result<(), String> {
        if self.connections.contains_key(&connection.connection_id) {
            return Err(format!("Connection '{}' already exists", connection.connection_id));
        }
        
        self.connections.insert(connection.connection_id.clone(), connection);
        self.connection_counter += 1;
        Ok(())
    }

    /// Remove a proxy connection
    pub fn remove_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if self.connections.remove(connection_id).is_none() {
            return Err(format!("Connection '{}' not found", connection_id));
        }
        Ok(())
    }

    /// Forward a request
    pub fn forward_request(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaProxy not running".to_string());
        }
        
        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = ProxyConnectionState::Forwarding;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Send response
    pub fn send_response(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaProxy not running".to_string());
        }
        
        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = ProxyConnectionState::Responding;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Close a connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaProxy not running".to_string());
        }
        
        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = ProxyConnectionState::Closed;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Get a proxy connection
    pub fn get_connection(&self, connection_id: &str) -> Option<&ProxyConnection> {
        self.connections.get(connection_id)
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&ProxyConnection> {
        self.connections.values().collect()
    }

    /// List connections by state
    pub fn list_by_state(&self, state: ProxyConnectionState) -> Vec<&ProxyConnection> {
        self.connections.values()
            .filter(|c| c.state == state)
            .collect()
    }

    /// List connections by protocol
    pub fn list_by_protocol(&self, protocol: ProxyProtocol) -> Vec<&ProxyConnection> {
        self.connections.values()
            .filter(|c| c.protocol == protocol)
            .collect()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get proxy server statistics
    pub fn proxy_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_connections".to_string(), self.connection_count().to_string());
        stats.insert("forwarding_connections".to_string(), self.list_by_state(ProxyConnectionState::Forwarding).len().to_string());
        stats.insert("responding_connections".to_string(), self.list_by_state(ProxyConnectionState::Responding).len().to_string());
        stats.insert("http_connections".to_string(), self.list_by_protocol(ProxyProtocol::Http).len().to_string());
        stats.insert("https_connections".to_string(), self.list_by_protocol(ProxyProtocol::Https).len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaProxy {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaProxy instance (placeholder for actual global state)
static unsafe mut SIGMAPROXY_INSTANCE: Option<SigmaProxy> = None;

/// Initialize the global SigmaProxy instance
pub fn init_sigmaproxy() {
    unsafe {
        SIGMAPROXY_INSTANCE = Some(SigmaProxy::new());
    }
}

/// Get the global SigmaProxy instance
pub fn get_sigmaproxy() -> Option<&'static mut SigmaProxy> {
    unsafe { SIGMAPROXY_INSTANCE.as_mut() }
}

/// Convenience function to forward a request
pub fn forward_proxy_request(connection_id: &str) -> Result<(), String> {
    if let Some(proxy) = get_sigmaproxy() {
        proxy.forward_request(connection_id)
    } else {
        Err("SigmaProxy not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_connection_state_display() {
        assert_eq!(ProxyConnectionState::Connected.to_string(), "connected");
        assert_eq!(ProxyConnectionState::Forwarding.to_string(), "forwarding");
    }

    #[test]
    fn test_proxy_protocol_display() {
        assert_eq!(ProxyProtocol::Http.to_string(), "HTTP");
        assert_eq!(ProxyProtocol::Https.to_string(), "HTTPS");
    }

    #[test]
    fn test_proxy_connection() {
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        )
        .with_protocol(ProxyProtocol::Https)
        .with_target_host("example.com".to_string())
        .with_target_port(443)
        .with_state(ProxyConnectionState::Forwarding);

        assert_eq!(connection.connection_id, "conn1");
        assert_eq!(connection.protocol, ProxyProtocol::Https);
        assert_eq!(connection.target_host, "example.com");
        assert_eq!(connection.state, ProxyConnectionState::Forwarding);
    }

    #[test]
    fn test_sigmaproxy() {
        let mut proxy = SigmaProxy::new();
        assert!(!proxy.is_running());
        proxy.start();
        assert!(proxy.is_running());
        proxy.stop();
        assert!(!proxy.is_running());
    }

    #[test]
    fn test_sigmaproxy_add_connection() {
        let mut proxy = SigmaProxy::new();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        );
        
        assert!(proxy.add_connection(connection).is_ok());
        assert_eq!(proxy.connection_count(), 1);
    }

    #[test]
    fn test_sigmaproxy_forward_request() {
        let mut proxy = SigmaProxy::new();
        proxy.start();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        );
        
        proxy.add_connection(connection).unwrap();
        assert!(proxy.forward_request("conn1").is_ok());
        
        let conn = proxy.get_connection("conn1").unwrap();
        assert_eq!(conn.state, ProxyConnectionState::Forwarding);
    }

    #[test]
    fn test_sigmaproxy_send_response() {
        let mut proxy = SigmaProxy::new();
        proxy.start();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        )
        .with_state(ProxyConnectionState::Forwarding);
        
        proxy.add_connection(connection).unwrap();
        assert!(proxy.send_response("conn1").is_ok());
        
        let conn = proxy.get_connection("conn1").unwrap();
        assert_eq!(conn.state, ProxyConnectionState::Responding);
    }

    #[test]
    fn test_sigmaproxy_list_by_state() {
        let mut proxy = SigmaProxy::new();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        )
        .with_state(ProxyConnectionState::Forwarding);
        
        proxy.add_connection(connection).unwrap();
        
        let connections = proxy.list_by_state(ProxyConnectionState::Forwarding);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmaproxy_list_by_protocol() {
        let mut proxy = SigmaProxy::new();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        )
        .with_protocol(ProxyProtocol::Https);
        
        proxy.add_connection(connection).unwrap();
        
        let connections = proxy.list_by_protocol(ProxyProtocol::Https);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmaproxy_stats() {
        let mut proxy = SigmaProxy::new();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        )
        .with_protocol(ProxyProtocol::Http)
        .with_state(ProxyConnectionState::Forwarding);
        
        proxy.add_connection(connection).unwrap();
        
        let stats = proxy.proxy_stats();
        assert_eq!(stats.get("total_connections"), Some(&"1".to_string()));
        assert_eq!(stats.get("forwarding_connections"), Some(&"1".to_string()));
        assert_eq!(stats.get("http_connections"), Some(&"1".to_string()));
    }

    #[test]
    fn test_forward_proxy_request_convenience() {
        init_sigmaproxy();
        
        let mut proxy = get_sigmaproxy().unwrap();
        proxy.start();
        
        let connection = ProxyConnection::new(
            "conn1".to_string(),
            "192.168.1.100".to_string(),
            12345
        );
        
        proxy.add_connection(connection).unwrap();
        
        let result = forward_proxy_request("conn1");
        assert!(result.is_ok());
    }
}
