// Linux-inspired Network Namespace
// Provides per-process network stack isolation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Network device
#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub name: String,
    pub index: u32,
    pub mac_address: String,
    pub mtu: u32,
}

impl NetworkDevice {
    pub fn new(name: String, index: u32, mac_address: String, mtu: u32) -> Self {
        Self {
            name,
            index,
            mac_address,
            mtu,
        }
    }
}

/// Network namespace
#[derive(Debug, Clone)]
pub struct NetworkNamespace {
    pub id: u64,
    pub devices: HashMap<String, NetworkDevice>,
    pub parent_id: Option<u64>,
}

impl NetworkNamespace {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            devices: HashMap::new(),
            parent_id: None,
        }
    }

    pub fn with_parent(mut self, parent_id: u64) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Add a network device
    pub fn add_device(&mut self, device: NetworkDevice) -> Result<(), String> {
        if self.devices.contains_key(&device.name) {
            return Err(format!("Device {} already exists", device.name));
        }
        self.devices.insert(device.name.clone(), device);
        Ok(())
    }

    /// Remove a network device
    pub fn remove_device(&mut self, name: &str) -> Result<(), String> {
        match self.devices.remove(name) {
            Some(_) => Ok(()),
            None => Err(format!("Device {} not found", name)),
        }
    }

    /// Get a device by name
    pub fn get_device(&self, name: &str) -> Option<&NetworkDevice> {
        self.devices.get(name)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&NetworkDevice> {
        self.devices.values().collect()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

/// Network namespace manager for system-wide namespace management
pub struct NetworkNamespaceManager {
    namespaces: Arc<Mutex<HashMap<u64, NetworkNamespace>>>,
    next_namespace_id: Arc<Mutex<u64>>,
}

impl NetworkNamespaceManager {
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(Mutex::new(HashMap::new())),
            next_namespace_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new network namespace
    pub fn create_namespace(&self, parent_id: Option<u64>) -> u64 {
        let mut next_id = self.next_namespace_id.lock().unwrap();
        let namespace_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let namespace = match parent_id {
            Some(pid) => NetworkNamespace::new(namespace_id).with_parent(pid),
            None => NetworkNamespace::new(namespace_id),
        };

        let mut namespaces = self.namespaces.lock().unwrap();
        namespaces.insert(namespace_id, namespace);

        namespace_id
    }

    /// Get a namespace by ID
    pub fn get_namespace(&self, namespace_id: u64) -> Option<NetworkNamespace> {
        let namespaces = self.namespaces.lock().unwrap();
        namespaces.get(&namespace_id).cloned()
    }

    /// Remove a namespace
    pub fn remove_namespace(&self, namespace_id: u64) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.remove(&namespace_id) {
            Some(_) => Ok(()),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Add device to a namespace
    pub fn add_device(&self, namespace_id: u64, device: NetworkDevice) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.get_mut(&namespace_id) {
            Some(namespace) => namespace.add_device(device),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Remove device from a namespace
    pub fn remove_device(&self, namespace_id: u64, name: &str) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.get_mut(&namespace_id) {
            Some(namespace) => namespace.remove_device(name),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Get namespace count
    pub fn namespace_count(&self) -> usize {
        let namespaces = self.namespaces.lock().unwrap();
        namespaces.len()
    }
}

impl Default for NetworkNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_device() {
        let device = NetworkDevice::new("eth0".to_string(), 1, "00:11:22:33:44:55".to_string(), 1500);
        assert_eq!(device.name, "eth0");
        assert_eq!(device.index, 1);
        assert_eq!(device.mtu, 1500);
    }

    #[test]
    fn test_network_namespace() {
        let namespace = NetworkNamespace::new(1);
        assert_eq!(namespace.id, 1);
        assert_eq!(namespace.device_count(), 0);
    }

    #[test]
    fn test_network_namespace_with_parent() {
        let namespace = NetworkNamespace::new(1).with_parent(0);
        assert_eq!(namespace.parent_id, Some(0));
    }

    #[test]
    fn test_network_namespace_add_device() {
        let mut namespace = NetworkNamespace::new(1);

        let device = NetworkDevice::new("eth0".to_string(), 1, "00:11:22:33:44:55".to_string(), 1500);
        namespace.add_device(device).unwrap();

        assert_eq!(namespace.device_count(), 1);
    }

    #[test]
    fn test_network_namespace_duplicate_device() {
        let mut namespace = NetworkNamespace::new(1);

        let device = NetworkDevice::new("eth0".to_string(), 1, "00:11:22:33:44:55".to_string(), 1500);
        namespace.add_device(device.clone()).unwrap();
        assert!(namespace.add_device(device).is_err());
    }

    #[test]
    fn test_network_namespace_remove_device() {
        let mut namespace = NetworkNamespace::new(1);

        let device = NetworkDevice::new("eth0".to_string(), 1, "00:11:22:33:44:55".to_string(), 1500);
        namespace.add_device(device).unwrap();

        namespace.remove_device("eth0").unwrap();
        assert_eq!(namespace.device_count(), 0);
    }

    #[test]
    fn test_network_namespace_manager() {
        let manager = NetworkNamespaceManager::new();

        let namespace_id = manager.create_namespace(None);
        assert_eq!(namespace_id, 1);

        let device = NetworkDevice::new("eth0".to_string(), 1, "00:11:22:33:44:55".to_string(), 1500);
        manager.add_device(namespace_id, device).unwrap();

        assert_eq!(manager.namespace_count(), 1);
    }

    #[test]
    fn test_network_namespace_manager_with_parent() {
        let manager = NetworkNamespaceManager::new();

        let parent_id = manager.create_namespace(None);
        let child_id = manager.create_namespace(Some(parent_id));

        let namespace = manager.get_namespace(child_id).unwrap();
        assert_eq!(namespace.parent_id, Some(parent_id));
    }

    #[test]
    fn test_network_namespace_manager_multiple_namespaces() {
        let manager = NetworkNamespaceManager::new();

        let _ns_id1 = manager.create_namespace(None);
        let _ns_id2 = manager.create_namespace(None);

        assert_eq!(manager.namespace_count(), 2);
    }

    #[test]
    fn test_network_namespace_manager_remove() {
        let manager = NetworkNamespaceManager::new();

        let namespace_id = manager.create_namespace(None);
        manager.remove_namespace(namespace_id).unwrap();

        assert_eq!(manager.namespace_count(), 0);
    }

    #[test]
    fn test_network_namespace_manager_invalid() {
        let manager = NetworkNamespaceManager::new();
        assert!(manager.add_device(999, NetworkDevice::new("eth0".to_string(), 1, "00:11:22:33:44:55".to_string(), 1500)).is_err());
    }
}
