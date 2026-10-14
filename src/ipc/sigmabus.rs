// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaBus - System Message Bus
//!
//! SigmaBus provides a system message bus for SigmaOS, inspired by D-Bus.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Method call
    MethodCall,
    /// Method return
    MethodReturn,
    /// Error message
    Error,
    /// Signal message
    Signal,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::MethodCall => write!(f, "method_call"),
            MessageType::MethodReturn => write!(f, "method_return"),
            MessageType::Error => write!(f, "error"),
            MessageType::Signal => write!(f, "signal"),
        }
    }
}

/// Bus message
#[derive(Debug, Clone)]
pub struct BusMessage {
    /// Message type
    pub message_type: MessageType,
    /// Sender
    pub sender: String,
    /// Destination
    pub destination: Option<String>,
    /// Path
    pub path: String,
    /// Interface
    pub interface: String,
    /// Member
    pub member: String,
    /// Message body
    pub body: Vec<u8>,
    /// Message ID
    pub message_id: u32,
    /// Reply serial
    pub reply_serial: Option<u32>,
}

impl BusMessage {
    /// Create a new bus message
    pub fn new(message_type: MessageType, sender: String) -> Self {
        Self {
            message_type,
            sender,
            destination: None,
            path: String::new(),
            interface: String::new(),
            member: String::new(),
            body: Vec::new(),
            message_id: 0,
            reply_serial: None,
        }
    }

    /// Set destination
    pub fn with_destination(mut self, dest: String) -> Self {
        self.destination = Some(dest);
        self
    }

    /// Set path
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Set interface
    pub fn with_interface(mut self, interface: String) -> Self {
        self.interface = interface;
        self
    }

    /// Set member
    pub fn with_member(mut self, member: String) -> Self {
        self.member = member;
        self
    }

    /// Set body
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    /// Set message ID
    pub fn with_message_id(mut self, id: u32) -> Self {
        self.message_id = id;
        self
    }

    /// Set reply serial
    pub fn with_reply_serial(mut self, serial: u32) -> Self {
        self.reply_serial = Some(serial);
        self
    }
}

/// Service endpoint
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    /// Service name
    pub name: String,
    /// Object path
    pub path: String,
    /// Interface
    pub interface: String,
    /// Methods
    pub methods: Vec<String>,
    /// Signals
    pub signals: Vec<String>,
}

impl ServiceEndpoint {
    /// Create a new service endpoint
    pub fn new(name: String, path: String, interface: String) -> Self {
        Self {
            name,
            path,
            interface,
            methods: Vec::new(),
            signals: Vec::new(),
        }
    }

    /// Add method
    pub fn with_method(mut self, method: String) -> Self {
        self.methods.push(method);
        self
    }

    /// Add signal
    pub fn with_signal(mut self, signal: String) -> Self {
        self.signals.push(signal);
        self
    }
}

/// SigmaBus - System Message Bus
pub struct SigmaBus {
    /// Service endpoints (name -> endpoint)
    endpoints: BTreeMap<String, ServiceEndpoint>,
    /// Running state
    running: bool,
    /// Message counter
    message_counter: u32,
}

impl SigmaBus {
    /// Create a new SigmaBus instance
    pub fn new() -> Self {
        Self {
            endpoints: BTreeMap::new(),
            running: false,
            message_counter: 0,
        }
    }

    /// Start the message bus
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the message bus
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the bus is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Register a service endpoint
    pub fn register_endpoint(&mut self, endpoint: ServiceEndpoint) -> Result<(), String> {
        if self.endpoints.contains_key(&endpoint.name) {
            return Err(format!("Endpoint '{}' already registered", endpoint.name));
        }
        self.endpoints.insert(endpoint.name.clone(), endpoint);
        Ok(())
    }

    /// Unregister a service endpoint
    pub fn unregister_endpoint(&mut self, name: &str) -> Result<(), String> {
        if self.endpoints.remove(name).is_none() {
            return Err(format!("Endpoint '{}' not found", name));
        }
        Ok(())
    }

    /// Get an endpoint
    pub fn get_endpoint(&self, name: &str) -> Option<&ServiceEndpoint> {
        self.endpoints.get(name)
    }

    /// List all endpoints
    pub fn list_endpoints(&self) -> Vec<&ServiceEndpoint> {
        self.endpoints.values().collect()
    }

    /// Send a message
    pub fn send_message(&mut self, message: BusMessage) -> Result<(), String> {
        if !self.running {
            return Err("SigmaBus not running".to_string());
        }

        self.message_counter += 1;
        // In a real implementation, this would route the message to the destination
        Ok(())
    }

    /// Get message count
    pub fn message_count(&self) -> u32 {
        self.message_counter
    }

    /// Get endpoint count
    pub fn endpoint_count(&self) -> usize {
        self.endpoints.len()
    }

    /// Get bus statistics
    pub fn bus_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("endpoints".to_string(), self.endpoint_count().to_string());
        stats.insert("messages_sent".to_string(), self.message_count.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaBus instance (placeholder for actual global state)
static mut SIGMABUS_INSTANCE: Option<SigmaBus> = None;

/// Initialize the global SigmaBus instance
pub fn init_sigmabus() {
    unsafe {
        SIGMABUS_INSTANCE = Some(SigmaBus::new());
    }
}

/// Get the global SigmaBus instance
pub fn get_sigmabus() -> Option<&'static mut SigmaBus> {
    unsafe { SIGMABUS_INSTANCE.as_mut() }
}

/// Convenience function to register an endpoint
pub fn register_service_endpoint(endpoint: ServiceEndpoint) -> Result<(), String> {
    if let Some(bus) = get_sigmabus() {
        bus.register_endpoint(endpoint)
    } else {
        Err("SigmaBus not initialized".to_string())
    }
}

/// Convenience function to send a message
pub fn send_bus_message(message: BusMessage) -> Result<(), String> {
    if let Some(bus) = get_sigmabus() {
        bus.send_message(message)
    } else {
        Err("SigmaBus not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_display() {
        assert_eq!(MessageType::MethodCall.to_string(), "method_call");
        assert_eq!(MessageType::Signal.to_string(), "signal");
    }

    #[test]
    fn test_bus_message() {
        let message = BusMessage::new(MessageType::MethodCall, "sender".to_string())
            .with_destination("destination".to_string())
            .with_path("/org/example".to_string())
            .with_interface("org.example.Interface".to_string())
            .with_member("Method".to_string())
            .with_message_id(1);

        assert_eq!(message.message_type, MessageType::MethodCall);
        assert_eq!(message.sender, "sender");
        assert_eq!(message.destination, Some("destination".to_string()));
        assert_eq!(message.path, "/org/example");
    }

    #[test]
    fn test_service_endpoint() {
        let endpoint = ServiceEndpoint::new(
            "org.example.Service".to_string(),
            "/org/example/Service".to_string(),
            "org.example.Interface".to_string()
        )
        .with_method("Method1".to_string())
        .with_method("Method2".to_string())
        .with_signal("Signal1".to_string());

        assert_eq!(endpoint.name, "org.example.Service");
        assert_eq!(endpoint.methods.len(), 2);
        assert_eq!(endpoint.signals.len(), 1);
    }

    #[test]
    fn test_sigmabus() {
        let mut bus = SigmaBus::new();
        assert!(!bus.is_running());
        bus.start();
        assert!(bus.is_running());
        bus.stop();
        assert!(!bus.is_running());
    }

    #[test]
    fn test_sigmabus_register_endpoint() {
        let mut bus = SigmaBus::new();
        
        let endpoint = ServiceEndpoint::new(
            "org.example.Service".to_string(),
            "/org/example/Service".to_string(),
            "org.example.Interface".to_string()
        );
        
        assert!(bus.register_endpoint(endpoint).is_ok());
        assert_eq!(bus.endpoint_count(), 1);
    }

    #[test]
    fn test_sigmabus_duplicate_endpoint() {
        let mut bus = SigmaBus::new();
        
        let endpoint1 = ServiceEndpoint::new(
            "org.example.Service".to_string(),
            "/org/example/Service".to_string(),
            "org.example.Interface".to_string()
        );
        
        let endpoint2 = ServiceEndpoint::new(
            "org.example.Service".to_string(),
            "/org/example/Service2".to_string(),
            "org.example.Interface2".to_string()
        );
        
        assert!(bus.register_endpoint(endpoint1).is_ok());
        assert!(bus.register_endpoint(endpoint2).is_err());
    }

    #[test]
    fn test_sigmabus_send_message() {
        let mut bus = SigmaBus::new();
        bus.start();
        
        let message = BusMessage::new(MessageType::MethodCall, "sender".to_string());
        assert!(bus.send_message(message).is_ok());
        assert_eq!(bus.message_count(), 1);
    }

    #[test]
    fn test_sigmabus_send_message_not_running() {
        let mut bus = SigmaBus::new();
        
        let message = BusMessage::new(MessageType::MethodCall, "sender".to_string());
        assert!(bus.send_message(message).is_err());
    }

    #[test]
    fn test_sigmabus_stats() {
        let mut bus = SigmaBus::new();
        bus.start();
        
        let endpoint = ServiceEndpoint::new(
            "org.example.Service".to_string(),
            "/org/example/Service".to_string(),
            "org.example.Interface".to_string()
        );
        
        bus.register_endpoint(endpoint).unwrap();
        
        let message = BusMessage::new(MessageType::MethodCall, "sender".to_string());
        bus.send_message(message).unwrap();
        
        let stats = bus.bus_stats();
        assert_eq!(stats.get("endpoints"), Some(&"1".to_string()));
        assert_eq!(stats.get("messages_sent"), Some(&"1".to_string()));
        assert_eq!(stats.get("running"), Some(&"true".to_string()));
    }

    #[test]
    fn test_register_service_endpoint_convenience() {
        init_sigmabus();
        
        let endpoint = ServiceEndpoint::new(
            "org.example.Service".to_string(),
            "/org/example/Service".to_string(),
            "org.example.Interface".to_string()
        );
        
        let result = register_service_endpoint(endpoint);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_bus_message_convenience() {
        init_sigmabus();
        
        let mut bus = get_sigmabus().unwrap();
        bus.start();
        
        let message = BusMessage::new(MessageType::MethodCall, "sender".to_string());
        let result = send_bus_message(message);
        
        assert!(result.is_ok());
    }
}
