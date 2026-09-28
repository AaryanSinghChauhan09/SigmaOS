// Network Namespace Isolation
// Inspired by Linux network namespaces for network stack isolation

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Network namespace ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkNamespaceId(pub u64);

/// Network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub index: u32,
    pub mtu: u32,
    pub state: InterfaceState,
    pub addresses: Vec<InterfaceAddress>,
}

/// Interface state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    Up,
    Down,
    Unknown,
}

/// Interface address
#[derive(Debug, Clone)]
pub struct InterfaceAddress {
    pub address: String,
    pub netmask: String,
    pub broadcast: String,
}

/// Network route
#[derive(Debug, Clone)]
pub struct NetworkRoute {
    pub destination: String,
    pub gateway: String,
    pub metric: u32,
    pub interface: String,
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub source: String,
    pub destination: String,
    pub action: FirewallAction,
}

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
}

/// Network namespace
pub struct NetworkNamespace {
    pub id: NetworkNamespaceId,
    pub interfaces: HashMap<String, NetworkInterface>,
    pub routes: Vec<NetworkRoute>,
    pub firewall_rules: Vec<FirewallRule>,
    pub default_route: Option<NetworkRoute>,
}

impl NetworkNamespace {
    pub fn new(id: NetworkNamespaceId) -> Self {
        Self {
            id,
            interfaces: HashMap::new(),
            routes: Vec::new(),
            firewall_rules: Vec::new(),
            default_route: None,
        }
    }

    /// Add an interface
    pub fn add_interface(&mut self, interface: NetworkInterface) {
        self.interfaces.insert(interface.name.clone(), interface);
    }

    /// Remove an interface
    pub fn remove_interface(&mut self, name: &str) -> Option<NetworkInterface> {
        self.interfaces.remove(name)
    }

    /// Get an interface
    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    /// Add a route
    pub fn add_route(&mut self, route: NetworkRoute) {
        self.routes.push(route);
    }

    /// Remove a route
    pub fn remove_route(&mut self, index: usize) -> Option<NetworkRoute> {
        if index < self.routes.len() {
            Some(self.routes.remove(index))
        } else {
            None
        }
    }

    /// Add a firewall rule
    pub fn add_firewall_rule(&mut self, rule: FirewallRule) {
        self.firewall_rules.push(rule);
    }

    /// Remove a firewall rule
    pub fn remove_firewall_rule(&mut self, index: usize) -> Option<FirewallRule> {
        if index < self.firewall_rules.len() {
            Some(self.firewall_rules.remove(index))
        } else {
            None
        }
    }

    /// Set default route
    pub fn set_default_route(&mut self, route: NetworkRoute) {
        self.default_route = Some(route);
    }

    /// Get interface count
    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }

    /// Get route count
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Get firewall rule count
    pub fn firewall_rule_count(&self) -> usize {
        self.firewall_rules.len()
    }
}

/// Network namespace manager
pub struct NetworkNamespaceManager {
    namespaces: HashMap<NetworkNamespaceId, NetworkNamespace>,
    next_id: AtomicU64,
    initial_namespace: NetworkNamespaceId,
}

impl NetworkNamespaceManager {
    pub fn new() -> Self {
        let initial_id = NetworkNamespaceId(1);
        let mut manager = Self {
            namespaces: HashMap::new(),
            next_id: AtomicU64::new(2),
            initial_namespace: initial_id,
        };
        
        // Create initial namespace
        let initial_ns = NetworkNamespace::new(initial_id);
        manager.namespaces.insert(initial_id, initial_ns);
        
        manager
    }

    /// Create a new network namespace
    pub fn create_namespace(&mut self) -> NetworkNamespaceId {
        let id = NetworkNamespaceId(self.next_id.fetch_add(1, Ordering::SeqCst));
        
        let namespace = NetworkNamespace::new(id);
        self.namespaces.insert(id, namespace);
        
        id
    }

    /// Delete a network namespace
    pub fn delete_namespace(&mut self, id: NetworkNamespaceId) -> Result<(), &'static str> {
        if id == self.initial_namespace {
            return Err("Cannot delete initial namespace");
        }
        
        if self.namespaces.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Namespace not found")
        }
    }

    /// Get a namespace
    pub fn get_namespace(&self, id: NetworkNamespaceId) -> Option<&NetworkNamespace> {
        self.namespaces.get(&id)
    }

    /// Get mutable namespace
    pub fn get_namespace_mut(&mut self, id: NetworkNamespaceId) -> Option<&mut NetworkNamespace> {
        self.namespaces.get_mut(&id)
    }

    /// Get initial namespace
    pub fn initial_namespace(&self) -> NetworkNamespaceId {
        self.initial_namespace
    }

    /// Get namespace count
    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }

    /// List all namespaces
    pub fn list_namespaces(&self) -> Vec<NetworkNamespaceId> {
        self.namespaces.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_namespace() {
        let mut manager = NetworkNamespaceManager::new();
        
        let id = manager.create_namespace();
        assert_eq!(id.0, 2);
        assert_eq!(manager.namespace_count(), 2);
    }

    #[test]
    fn test_delete_namespace() {
        let mut manager = NetworkNamespaceManager::new();
        
        let id = manager.create_namespace();
        assert!(manager.delete_namespace(id).is_ok());
        assert_eq!(manager.namespace_count(), 1);
    }

    #[test]
    fn test_delete_initial_namespace() {
        let mut manager = NetworkNamespaceManager::new();
        
        let initial = manager.initial_namespace();
        assert!(manager.delete_namespace(initial).is_err());
    }

    #[test]
    fn test_add_interface() {
        let mut manager = NetworkNamespaceManager::new();
        
        let id = manager.create_namespace();
        let namespace = manager.get_namespace_mut(id).unwrap();
        
        let interface = NetworkInterface {
            name: "eth0".to_string(),
            index: 1,
            mtu: 1500,
            state: InterfaceState::Up,
            addresses: Vec::new(),
        };
        
        namespace.add_interface(interface);
        assert_eq!(namespace.interface_count(), 1);
    }

    #[test]
    fn test_add_route() {
        let mut manager = NetworkNamespaceManager::new();
        
        let id = manager.create_namespace();
        let namespace = manager.get_namespace_mut(id).unwrap();
        
        let route = NetworkRoute {
            destination: "192.168.1.0/24".to_string(),
            gateway: "192.168.1.1".to_string(),
            metric: 100,
            interface: "eth0".to_string(),
        };
        
        namespace.add_route(route);
        assert_eq!(namespace.route_count(), 1);
    }

    #[test]
    fn test_firewall_rule() {
        let mut manager = NetworkNamespaceManager::new();
        
        let id = manager.create_namespace();
        let namespace = manager.get_namespace_mut(id).unwrap();
        
        let rule = FirewallRule {
            source: "0.0.0.0/0".to_string(),
            destination: "192.168.1.0/24".to_string(),
            action: FirewallAction::Accept,
        };
        
        namespace.add_firewall_rule(rule);
        assert_eq!(namespace.firewall_rule_count(), 1);
    }

    #[test]
    fn test_default_route() {
        let mut manager = NetworkNamespaceManager::new();
        
        let id = manager.create_namespace();
        let namespace = manager.get_namespace_mut(id).unwrap();
        
        let route = NetworkRoute {
            destination: "default".to_string(),
            gateway: "192.168.1.1".to_string(),
            metric: 100,
            interface: "eth0".to_string(),
        };
        
        namespace.set_default_route(route);
        assert!(namespace.default_route.is_some());
    }

    #[test]
    fn test_list_namespaces() {
        let mut manager = NetworkNamespaceManager::new();
        
        manager.create_namespace();
        manager.create_namespace();
        
        let list = manager.list_namespaces();
        assert_eq!(list.len(), 3);
    }
}
