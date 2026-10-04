// IPC Bus System
// D-Bus-inspired inter-process communication bus

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    MethodCall,
    MethodReturn,
    Error,
    Signal,
}

/// IPC message
#[derive(Debug, Clone)]
pub struct IpcMessage {
    pub id: u64,
    pub message_type: MessageType,
    pub sender: u64,
    pub destination: Option<u64>,
    pub interface: String,
    pub member: String,
    pub parameters: Vec<String>,
}

/// Bus name
#[derive(Debug, Clone)]
pub struct BusName {
    pub name: String,
    pub owner: u64,
    pub unique: bool,
}

/// Bus endpoint
#[derive(Debug, Clone)]
pub struct BusEndpoint {
    pub id: u64,
    pub owner: u64,
    pub interfaces: Vec<String>,
}

/// IPC bus
pub struct IpcBus {
    next_message_id: AtomicU64,
    next_endpoint_id: AtomicU64,
    messages: Vec<IpcMessage>,
    endpoints: HashMap<u64, BusEndpoint>,
    bus_names: HashMap<String, BusName>,
    message_count: AtomicU64,
}

impl IpcBus {
    pub fn new() -> Self {
        Self {
            next_message_id: AtomicU64::new(1),
            next_endpoint_id: AtomicU64::new(1),
            messages: Vec::new(),
            endpoints: HashMap::new(),
            bus_names: HashMap::new(),
            message_count: AtomicU64::new(0),
        }
    }

    /// Register a bus endpoint
    pub fn register_endpoint(&mut self, owner: u64, interfaces: Vec<String>) -> BusEndpoint {
        let id = self.next_endpoint_id.fetch_add(1, Ordering::SeqCst);

        let endpoint = BusEndpoint {
            id,
            owner,
            interfaces,
        };

        self.endpoints.insert(id, endpoint.clone());
        endpoint
    }

    /// Unregister a bus endpoint
    pub fn unregister_endpoint(&mut self, id: u64) -> Result<(), &'static str> {
        if self.endpoints.remove(&id).is_some() {
            // Remove bus names owned by this endpoint
            self.bus_names.retain(|_, name| name.owner != id);
            Ok(())
        } else {
            Err("Endpoint not found")
        }
    }

    /// Request a bus name
    pub fn request_name(&mut self, name: String, owner: u64) -> Result<(), &'static str> {
        if self.bus_names.contains_key(&name) {
            return Err("Name already requested");
        }

        let bus_name = BusName {
            name: name.clone(),
            owner,
            unique: false,
        };

        self.bus_names.insert(name, bus_name);
        Ok(())
    }

    /// Release a bus name
    pub fn release_name(&mut self, name: String, owner: u64) -> Result<(), &'static str> {
        if let Some(bus_name) = self.bus_names.get(&name) {
            if bus_name.owner == owner {
                self.bus_names.remove(&name);
                Ok(())
            } else {
                Err("Not the owner of this name")
            }
        } else {
            Err("Name not found")
        }
    }

    /// Send a message
    pub fn send_message(&mut self, message: IpcMessage) -> Result<u64, &'static str> {
        let id = self.next_message_id.fetch_add(1, Ordering::SeqCst);
        let mut new_message = message;
        new_message.id = id;

        self.messages.push(new_message);
        self.message_count.fetch_add(1, Ordering::SeqCst);

        Ok(id)
    }

    /// Get messages for endpoint
    pub fn get_messages(&self, endpoint_id: u64) -> Vec<&IpcMessage> {
        self.messages
            .iter()
            .filter(|m| m.destination == Some(endpoint_id) || m.destination.is_none())
            .collect()
    }

    /// Get endpoint by ID
    pub fn get_endpoint(&self, id: u64) -> Option<&BusEndpoint> {
        self.endpoints.get(&id)
    }

    /// Get bus name owner
    pub fn get_name_owner(&self, name: &str) -> Option<u64> {
        self.bus_names.get(name).map(|n| n.owner)
    }

    /// Get all bus names
    pub fn get_all_names(&self) -> Vec<&BusName> {
        self.bus_names.values().collect()
    }

    /// Get message count
    pub fn message_count(&self) -> u64 {
        self.message_count.load(Ordering::SeqCst)
    }

    /// Get endpoint count
    pub fn endpoint_count(&self) -> usize {
        self.endpoints.len()
    }

    /// Clear all messages
    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.message_count.store(0, Ordering::SeqCst);
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_register_endpoint() {
        let mut bus = IpcBus::new();

        let endpoint = bus.register_endpoint(1, vec!["test.interface".to_string()]);
        assert_eq!(endpoint.id, 1);
        assert_eq!(bus.endpoint_count(), 1);
    }

    #[test]
    fn test_request_name() {
        let mut bus = IpcBus::new();

        assert!(bus.request_name("org.test".to_string(), 1).is_ok());
        assert_eq!(bus.get_name_owner("org.test"), Some(1));
    }

    #[test]
    fn test_send_message() {
        let mut bus = IpcBus::new();

        let message = IpcMessage {
            id: 0,
            message_type: MessageType::MethodCall,
            sender: 1,
            destination: Some(2),
            interface: "org.test".to_string(),
            member: "TestMethod".to_string(),
            parameters: vec!["param1".to_string()],
        };

        let id = bus.send_message(message).unwrap();
        assert_eq!(id, 1);
        assert_eq!(bus.message_count(), 1);
    }

    #[test]
    fn test_get_messages() {
        let mut bus = IpcBus::new();

        let message = IpcMessage {
            id: 0,
            message_type: MessageType::MethodCall,
            sender: 1,
            destination: Some(2),
            interface: "org.test".to_string(),
            member: "TestMethod".to_string(),
            parameters: vec!["param1".to_string()],
        };

        bus.send_message(message);

        let messages = bus.get_messages(2);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_release_name() {
        let mut bus = IpcBus::new();

        bus.request_name("org.test".to_string(), 1).unwrap();
        assert!(bus.release_name("org.test".to_string(), 1).is_ok());
        assert!(bus.get_name_owner("org.test").is_none());
    }

    #[test]
    fn test_unregister_endpoint() {
        let mut bus = IpcBus::new();

        let endpoint = bus.register_endpoint(1, vec!["test.interface".to_string()]);
        bus.request_name("org.test".to_string(), endpoint.id)
            .unwrap();

        assert!(bus.unregister_endpoint(endpoint.id).is_ok());
        assert_eq!(bus.endpoint_count(), 0);
        assert!(bus.get_name_owner("org.test").is_none());
    }
}
