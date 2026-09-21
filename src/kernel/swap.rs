// Linux-inspired Swap Management
// Provides swap space management for virtual memory

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Swap priority (higher value = higher priority)
pub type SwapPriority = i32;

/// Swap device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapDeviceType {
    Partition,
    File,
}

/// Swap device
#[derive(Debug, Clone)]
pub struct SwapDevice {
    pub id: u64,
    pub device_type: SwapDeviceType,
    pub path: String,
    pub size: u64,
    pub priority: SwapPriority,
    pub active: bool,
}

impl SwapDevice {
    pub fn new(id: u64, device_type: SwapDeviceType, path: String, size: u64) -> Self {
        Self {
            id,
            device_type,
            path,
            size,
            priority: -1, // Default priority
            active: false,
        }
    }

    pub fn with_priority(mut self, priority: SwapPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Activate the swap device
    pub fn activate(&mut self) -> Result<(), String> {
        if self.active {
            return Err("Swap device already active".to_string());
        }
        self.active = true;
        Ok(())
    }

    /// Deactivate the swap device
    pub fn deactivate(&mut self) -> Result<(), String> {
        if !self.active {
            return Err("Swap device not active".to_string());
        }
        self.active = false;
        Ok(())
    }

    /// Check if active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Swap statistics
#[derive(Debug, Clone, Copy)]
pub struct SwapStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

impl SwapStats {
    pub fn new(total: u64, used: u64) -> Self {
        Self {
            total,
            used,
            free: total - used,
        }
    }

    /// Calculate usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.used as f64 / self.total as f64) * 100.0
        }
    }
}

/// Swap manager for system-wide swap management
pub struct SwapManager {
    devices: Arc<Mutex<HashMap<u64, SwapDevice>>>,
    next_device_id: Arc<Mutex<u64>>,
    swap_enabled: Arc<Mutex<bool>>,
}

impl SwapManager {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(Mutex::new(HashMap::new())),
            next_device_id: Arc::new(Mutex::new(1)),
            swap_enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Create a new swap device
    pub fn create_device(&self, device_type: SwapDeviceType, path: String, size: u64) -> u64 {
        let mut next_id = self.next_device_id.lock().unwrap();
        let device_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let device = SwapDevice::new(device_id, device_type, path, size);
        let mut devices = self.devices.lock().unwrap();
        devices.insert(device_id, device);

        device_id
    }

    /// Get a device by ID
    pub fn get_device(&self, device_id: u64) -> Option<SwapDevice> {
        let devices = self.devices.lock().unwrap();
        devices.get(&device_id).cloned()
    }

    /// Remove a device
    pub fn remove_device(&self, device_id: u64) -> Result<(), String> {
        let mut devices = self.devices.lock().unwrap();
        match devices.remove(&device_id) {
            Some(device) => {
                if device.active {
                    return Err("Cannot remove active swap device".to_string());
                }
                Ok(())
            }
            None => Err(format!("Swap device {} not found", device_id)),
        }
    }

    /// Activate a swap device
    pub fn activate_device(&self, device_id: u64) -> Result<(), String> {
        let mut devices = self.devices.lock().unwrap();
        match devices.get_mut(&device_id) {
            Some(device) => device.activate(),
            None => Err(format!("Swap device {} not found", device_id)),
        }
    }

    /// Deactivate a swap device
    pub fn deactivate_device(&self, device_id: u64) -> Result<(), String> {
        let mut devices = self.devices.lock().unwrap();
        match devices.get_mut(&device_id) {
            Some(device) => device.deactivate(),
            None => Err(format!("Swap device {} not found", device_id)),
        }
    }

    /// Set swap device priority
    pub fn set_priority(&self, device_id: u64, priority: SwapPriority) -> Result<(), String> {
        let mut devices = self.devices.lock().unwrap();
        match devices.get_mut(&device_id) {
            Some(device) => {
                device.priority = priority;
                Ok(())
            }
            None => Err(format!("Swap device {} not found", device_id)),
        }
    }

    /// Get all active devices
    pub fn get_active_devices(&self) -> Vec<SwapDevice> {
        let devices = self.devices.lock().unwrap();
        devices.values().filter(|d| d.is_active()).cloned().collect()
    }

    /// Get swap statistics
    pub fn get_stats(&self) -> SwapStats {
        let devices = self.devices.lock().unwrap();
        let total: u64 = devices.values().filter(|d| d.is_active()).map(|d| d.size).sum();
        // In a real implementation, used would be tracked
        let used = total / 2; // Placeholder: 50% usage
        SwapStats::new(total, used)
    }

    /// Enable/disable swap globally
    pub fn set_swap_enabled(&self, enabled: bool) {
        let mut swap_enabled = self.swap_enabled.lock().unwrap();
        *swap_enabled = enabled;
    }

    /// Check if swap is enabled
    pub fn is_swap_enabled(&self) -> bool {
        let swap_enabled = self.swap_enabled.lock().unwrap();
        *swap_enabled
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        let devices = self.devices.lock().unwrap();
        devices.len()
    }

    /// Get active device count
    pub fn active_device_count(&self) -> usize {
        let devices = self.devices.lock().unwrap();
        devices.values().filter(|d| d.is_active()).count()
    }
}

impl Default for SwapManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_device_creation() {
        let device = SwapDevice::new(1, SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);
        assert_eq!(device.id, 1);
        assert_eq!(device.path, "/dev/sda2");
        assert!(!device.is_active());
    }

    #[test]
    fn test_swap_device_with_priority() {
        let device = SwapDevice::new(1, SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024)
            .with_priority(10);
        assert_eq!(device.priority, 10);
    }

    #[test]
    fn test_swap_device_lifecycle() {
        let mut device = SwapDevice::new(1, SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);

        device.activate().unwrap();
        assert!(device.is_active());

        device.deactivate().unwrap();
        assert!(!device.is_active());
    }

    #[test]
    fn test_swap_device_double_activate() {
        let mut device = SwapDevice::new(1, SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);
        device.activate().unwrap();
        assert!(device.activate().is_err());
    }

    #[test]
    fn test_swap_stats() {
        let stats = SwapStats::new(1024 * 1024 * 1024, 512 * 1024 * 1024);
        assert_eq!(stats.total, 1024 * 1024 * 1024);
        assert_eq!(stats.used, 512 * 1024 * 1024);
        assert_eq!(stats.free, 512 * 1024 * 1024);
        assert_eq!(stats.usage_percent(), 50.0);
    }

    #[test]
    fn test_swap_manager() {
        let manager = SwapManager::new();

        let device_id = manager.create_device(SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);
        assert_eq!(device_id, 1);

        manager.activate_device(device_id).unwrap();
        assert_eq!(manager.active_device_count(), 1);

        let stats = manager.get_stats();
        assert_eq!(stats.total, 1024 * 1024 * 1024);

        manager.deactivate_device(device_id).unwrap();
        manager.remove_device(device_id).unwrap();

        assert_eq!(manager.device_count(), 0);
    }

    #[test]
    fn test_swap_manager_multiple_devices() {
        let manager = SwapManager::new();

        let device_id1 = manager.create_device(SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);
        let device_id2 = manager.create_device(SwapDeviceType::File, "/swapfile".to_string(), 512 * 1024 * 1024);

        manager.activate_device(device_id1).unwrap();
        manager.activate_device(device_id2).unwrap();

        assert_eq!(manager.active_device_count(), 2);

        let stats = manager.get_stats();
        assert_eq!(stats.total, 1536 * 1024 * 1024);
    }

    #[test]
    fn test_swap_manager_set_priority() {
        let manager = SwapManager::new();

        let device_id = manager.create_device(SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);
        manager.set_priority(device_id, 10).unwrap();

        let device = manager.get_device(device_id).unwrap();
        assert_eq!(device.priority, 10);
    }

    #[test]
    fn test_swap_manager_swap_enabled() {
        let manager = SwapManager::new();

        assert!(manager.is_swap_enabled());

        manager.set_swap_enabled(false);
        assert!(!manager.is_swap_enabled());

        manager.set_swap_enabled(true);
        assert!(manager.is_swap_enabled());
    }

    #[test]
    fn test_swap_manager_remove_active() {
        let manager = SwapManager::new();

        let device_id = manager.create_device(SwapDeviceType::Partition, "/dev/sda2".to_string(), 1024 * 1024 * 1024);
        manager.activate_device(device_id).unwrap();

        assert!(manager.remove_device(device_id).is_err());

        manager.deactivate_device(device_id).unwrap();
        assert!(manager.remove_device(device_id).is_ok());
    }
}
