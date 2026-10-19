// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaHTTP - System HTTP Server
//!
//! SigmaHTTP provides system HTTP server functionality for SigmaOS, inspired by Apache/nginx from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// HTTP connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpConnectionState {
    /// Connected
    Connected,
    /// Request Received
    RequestReceived,
    /// Processing
    Processing,
    /// Responding
    Responding,
    /// Closed
    Closed,
;

impl fmt::Display for HttpConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpConnectionState::Connected => write!(f, "connected"),
            HttpConnectionState::RequestReceived => write!(f, "request_received"),
            HttpConnectionState::Processing => write!(f, "processing"),
            HttpConnectionState::Responding => write!(f, "responding"),
            HttpConnectionState::Closed => write!(f, "closed"),
        }
    }
;

/// HTTP method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// GET
    Get,
    /// POST
    Post,
    /// PUT
    Put,
    /// DELETE
    Delete,
    /// HEAD
    Head,
    /// OPTIONS
    Options,
;

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
        }
    }
;

/// HTTP connection
#[derive(Debug, Clone)]
pub struct HttpConnection {
    /// Connection ID
    pub connection_id: String,
    /// Client address
    pub client_address: String,
    /// Client port
    pub client_port: u16,
    /// HTTP method
    pub method: HttpMethod,
    /// Request path
    pub path: String,
    /// HTTP version
    pub version: String,
    /// State
    pub state: HttpConnectionState,
;

impl HttpConnection {
    /// Create a new HTTP connection
    pub fn new(connection_id: String, client_address: String, client_port: u16) -> Self {
        Self {
            connection_id,
            client_address,
            client_port,
            method: HttpMethod::Get,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            state: HttpConnectionState::Connected,
        }
    }

    /// Set HTTP method
    pub fn with_method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    /// Set path
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Set version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// Set state
    pub fn with_state(mut self, state: HttpConnectionState) -> Self {
        self.state = state;
        self
    }
;

/// SigmaHTTP - System HTTP Server
pub struct SigmaHTTP {
    /// HTTP connections (connection_id -> connection)
    connections: BTreeMap<String, HttpConnection>,
    /// Running state
    running: bool,
    /// Connection counter
    connection_counter: u32,
;

impl SigmaHTTP {
    /// Create a new SigmaHTTP instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            running: false,
            connection_counter: 0,
        }
    }

    /// Start the HTTP server
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the HTTP server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add an HTTP connection
    pub fn add_connection(&mut self, connection: HttpConnection) -> Result<(), String> {
        if self.connections.contains_key(&connection.connection_id) {
            return Err(format!(
                "Connection '{}' already exists",
                connection.connection_id
            ));
        }

        self.connections
            .insert(connection.connection_id.clone(), connection);
        self.connection_counter += 1;
        Ok(())
    }

    /// Remove an HTTP connection
    pub fn remove_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if self.connections.remove(connection_id).is_none() {
            return Err(format!("Connection '{}' not found", connection_id));
        }
        Ok(())
    }

    /// Process a request
    pub fn process_request(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaHTTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = HttpConnectionState::Processing;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Send response
    pub fn send_response(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaHTTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = HttpConnectionState::Responding;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Close a connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaHTTP not running".to_string());
        }

        match self.connections.get_mut(connection_id) {
            Some(connection) => {
                connection.state = HttpConnectionState::Closed;
                Ok(())
            }
            None => Err(format!("Connection '{}' not found", connection_id)),
        }
    }

    /// Get an HTTP connection
    pub fn get_connection(&self, connection_id: &str) -> Option<&HttpConnection> {
        self.connections.get(connection_id)
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&HttpConnection> {
        self.connections.values().collect()
    }

    /// List connections by state
    pub fn list_by_state(&self, state: HttpConnectionState) -> Vec<&HttpConnection> {
        self.connections
            .values()
            .filter(|c| c.state == state)
            .collect()
    }

    /// List connections by method
    pub fn list_by_method(&self, method: HttpMethod) -> Vec<&HttpConnection> {
        self.connections
            .values()
            .filter(|c| c.method == method)
            .collect()
    }

    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get HTTP server statistics
    pub fn http_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_connections".to_string(),
            self.connection_count().to_string(),
        );
        stats.insert(
            "processing_connections".to_string(),
            self.list_by_state(HttpConnectionState::Processing)
                .len()
                .to_string(),
        );
        stats.insert(
            "responding_connections".to_string(),
            self.list_by_state(HttpConnectionState::Responding)
                .len()
                .to_string(),
        );
        stats.insert(
            "get_requests".to_string(),
            self.list_by_method(HttpMethod::Get).len().to_string(),
        );
        stats.insert(
            "post_requests".to_string(),
            self.list_by_method(HttpMethod::Post).len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaHTTP {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaHTTP instance (placeholder for actual global state)
static mut SIGMAHTTP_INSTANCE: Option<SigmaHTTP> = None;

/// Initialize the global SigmaHTTP instance
pub fn init_sigmahttp() {
    unsafe {
        SIGMAHTTP_INSTANCE = Some(SigmaHTTP::new());
    }
;

/// Get the global SigmaHTTP instance
pub fn get_sigmahttp() -> Option<&'static mut SigmaHTTP> {
    unsafe { SIGMAHTTP_INSTANCE.as_mut() }
;

/// Convenience function to process a request
pub fn process_http_request(connection_id: &str) -> Result<(), String> {
    if let Some(http) = get_sigmahttp() {
        http.process_request(connection_id)
    } else {
        Err("SigmaHTTP not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_connection_state_display() {
        assert_eq!(HttpConnectionState::Connected.to_string(), "connected");
        assert_eq!(HttpConnectionState::Processing.to_string(), "processing");
    }

    #[test]
    fn test_http_method_display() {
        assert_eq!(HttpMethod::Get.to_string(), "GET");
        assert_eq!(HttpMethod::Post.to_string(), "POST");
    }

    #[test]
    fn test_http_connection() {
        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345)
                .with_method(HttpMethod::Post)
                .with_path("/api/data".to_string())
                .with_version("HTTP/1.1".to_string())
                .with_state(HttpConnectionState::Processing);

        assert_eq!(connection.connection_id, "conn1");
        assert_eq!(connection.method, HttpMethod::Post);
        assert_eq!(connection.path, "/api/data");
        assert_eq!(connection.state, HttpConnectionState::Processing);
    }

    #[test]
    fn test_sigmahttp() {
        let mut http = SigmaHTTP::new();
        assert!(!http.is_running());
        http.start();
        assert!(http.is_running());
        http.stop();
        assert!(!http.is_running());
    }

    #[test]
    fn test_sigmahttp_add_connection() {
        let mut http = SigmaHTTP::new();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345);

        assert!(http.add_connection(connection).is_ok());
        assert_eq!(http.connection_count(), 1);
    }

    #[test]
    fn test_sigmahttp_process_request() {
        let mut http = SigmaHTTP::new();
        http.start();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345);

        http.add_connection(connection).unwrap();
        assert!(http.process_request("conn1").is_ok());

        let conn = http.get_connection("conn1").unwrap();
        assert_eq!(conn.state, HttpConnectionState::Processing);
    }

    #[test]
    fn test_sigmahttp_send_response() {
        let mut http = SigmaHTTP::new();
        http.start();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345)
                .with_state(HttpConnectionState::Processing);

        http.add_connection(connection).unwrap();
        assert!(http.send_response("conn1").is_ok());

        let conn = http.get_connection("conn1").unwrap();
        assert_eq!(conn.state, HttpConnectionState::Responding);
    }

    #[test]
    fn test_sigmahttp_list_by_state() {
        let mut http = SigmaHTTP::new();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345)
                .with_state(HttpConnectionState::Processing);

        http.add_connection(connection).unwrap();

        let connections = http.list_by_state(HttpConnectionState::Processing);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmahttp_list_by_method() {
        let mut http = SigmaHTTP::new();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345)
                .with_method(HttpMethod::Post);

        http.add_connection(connection).unwrap();

        let connections = http.list_by_method(HttpMethod::Post);
        assert_eq!(connections.len(), 1);
    }

    #[test]
    fn test_sigmahttp_stats() {
        let mut http = SigmaHTTP::new();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345)
                .with_method(HttpMethod::Get)
                .with_state(HttpConnectionState::Processing);

        http.add_connection(connection).unwrap();

        let stats = http.http_stats();
        assert_eq!(stats.get("total_connections"), Some(&"1".to_string()));
        assert_eq!(stats.get("processing_connections"), Some(&"1".to_string()));
        assert_eq!(stats.get("get_requests"), Some(&"1".to_string()));
    }

    #[test]
    fn test_process_http_request_convenience() {
        init_sigmahttp();

        let mut http = get_sigmahttp().unwrap();
        http.start();

        let connection =
            HttpConnection::new("conn1".to_string(), "192.168.1.100".to_string(), 12345);

        http.add_connection(connection).unwrap();

        let result = process_http_request("conn1");
        assert!(result.is_ok());
    }
;
