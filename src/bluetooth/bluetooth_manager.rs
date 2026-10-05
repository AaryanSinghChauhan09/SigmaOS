//! Bluetooth Manager
//!
//! Bluetooth management inspired by Linux Mint's Bluetooth settings and Omarchy's
//! Bluetooth utilities, supporting device discovery, pairing, and connection management.

use std::collections::HashMap;

/// Bluetooth device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothDeviceType {
    Unknown,
    Phone,
    Computer,
    Headset,
    Headphones,
    AudioVideo,
    Peripheral,
    Keyboard,
    Mouse,
    Joystick,
    Tablet,
}

impl BluetoothDeviceType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "phone" => Some(BluetoothDeviceType::Phone),
            "computer" => Some(BluetoothDeviceType::Computer),
            "headset" => Some(BluetoothDeviceType::Headset),
            "headphones" => Some(BluetoothDeviceType::Headphones),
            "audio" | "audiovideo" => Some(BluetoothDeviceType::AudioVideo),
            "peripheral" => Some(BluetoothDeviceType::Peripheral),
            "keyboard" => Some(BluetoothDeviceType::Keyboard),
            "mouse" => Some(BluetoothDeviceType::Mouse),
            "joystick" => Some(BluetoothDeviceType::Joystick),
            "tablet" => Some(BluetoothDeviceType::Tablet),
            _ => Some(BluetoothDeviceType::Unknown),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BluetoothDeviceType::Unknown => "Unknown",
            BluetoothDeviceType::Phone => "Phone",
            BluetoothDeviceType::Computer => "Computer",
            BluetoothDeviceType::Headset => "Headset",
            BluetoothDeviceType::Headphones => "Headphones",
            BluetoothDeviceType::AudioVideo => "Audio/Video",
            BluetoothDeviceType::Peripheral => "Peripheral",
            BluetoothDeviceType::Keyboard => "Keyboard",
            BluetoothDeviceType::Mouse => "Mouse",
            BluetoothDeviceType::Joystick => "Joystick",
            BluetoothDeviceType::Tablet => "Tablet",
        }
    }
}

/// Bluetooth device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothDeviceStatus {
    Disconnected,
    Connecting,
    Connected,
    Paired,
}

impl BluetoothDeviceStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "disconnected" => Some(BluetoothDeviceStatus::Disconnected),
            "connecting" => Some(BluetoothDeviceStatus::Connecting),
            "connected" => Some(BluetoothDeviceStatus::Connected),
            "paired" => Some(BluetoothDeviceStatus::Paired),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            BluetoothDeviceStatus::Disconnected => "Disconnected",
            BluetoothDeviceStatus::Connecting => "Connecting",
            BluetoothDeviceStatus::Connected => "Connected",
            BluetoothDeviceStatus::Paired => "Paired",
        }
    }
}

/// Bluetooth device
#[derive(Debug, Clone)]
pub struct BLEDevice {
    pub address: String,
    pub name: String,
    pub device_type: BluetoothDeviceType,
    pub status: BluetoothDeviceStatus,
    pub rssi: i32,
    pub is_trusted: bool,
    pub is_blocked: bool,
}

impl BLEDevice {
    pub fn new(address: String, name: String, device_type: BluetoothDeviceType) -> Self {
        Self {
            address,
            name,
            device_type,
            status: BluetoothDeviceStatus::Disconnected,
            rssi: 0,
            is_trusted: false,
            is_blocked: false,
        }
    }

    pub fn set_status(&mut self, status: BluetoothDeviceStatus) {
        self.status = status;
    }

    pub fn set_rssi(&mut self, rssi: i32) {
        self.rssi = rssi;
    }

    pub fn set_trusted(&mut self, trusted: bool) {
        self.is_trusted = trusted;
    }

    pub fn set_blocked(&mut self, blocked: bool) {
        self.is_blocked = blocked;
    }

    pub fn is_connected(&self) -> bool {
        self.status == BluetoothDeviceStatus::Connected || self.status == BluetoothDeviceStatus::Paired
    }
}

/// Bluetooth adapter
#[derive(Debug, Clone)]
pub struct BluetoothAdapter {
    pub name: String,
    pub address: String,
    pub is_powered: bool,
    pub is_discoverable: bool,
    pub is_pairable: bool,
}

impl BluetoothAdapter {
    pub fn new(name: String, address: String) -> Self {
        Self {
            name,
            address,
            is_powered: true,
            is_discoverable: false,
            is_pairable: false,
        }
    }

    pub fn set_powered(&mut self, powered: bool) {
        self.is_powered = powered;
    }

    pub fn set_discoverable(&mut self, discoverable: bool) {
        self.is_discoverable = discoverable;
    }

    pub fn set_pairable(&mut self, pairable: bool) {
        self.is_pairable = pairable;
    }
}

/// Bluetooth manager
#[derive(Debug)]
pub struct BluetoothDeviceManager {
    adapter: Option<BluetoothAdapter>,
    devices: HashMap<String, BLEDevice>,
}

impl BluetoothDeviceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            adapter: None,
            devices: HashMap::new(),
        };

        // Add default adapter
        let adapter = BluetoothAdapter::new(
            "SigmaOS Bluetooth".to_string(),
            "00:11:22:33:44:55".to_string(),
        );
        manager.adapter = Some(adapter);

        manager
    }

    /// Get adapter
    pub fn get_adapter(&self) -> Option<&BluetoothAdapter> {
        self.adapter.as_ref()
    }

    /// Get adapter mutably
    pub fn get_adapter_mut(&mut self) -> Option<&mut BluetoothAdapter> {
        self.adapter.as_mut()
    }

    /// Add a device
    pub fn add_device(&mut self, device: BLEDevice) {
        self.devices.insert(device.address.clone(), device);
    }

    /// Get a device
    pub fn get_device(&self, address: &str) -> Option<&BLEDevice> {
        self.devices.get(address)
    }

    /// Get a device mutably
    pub fn get_device_mut(&mut self, address: &str) -> Option<&mut BLEDevice> {
        self.devices.get_mut(address)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&BLEDevice> {
        self.devices.values().collect()
    }

    /// List devices by type
    pub fn list_by_type(&self, device_type: BluetoothDeviceType) -> Vec<&BLEDevice> {
        self.devices.values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    /// List connected devices
    pub fn list_connected(&self) -> Vec<&BLEDevice> {
        self.devices.values()
            .filter(|d| d.is_connected())
            .collect()
    }

    /// Scan for devices (simulated)
    pub fn scan(&mut self) -> Vec<&BLEDevice> {
        self.devices.clear();

        // Simulate device discovery
        let discovered_devices = vec![
            ("00:1A:2B:3C:4D:5E", "Headset Pro", BluetoothDeviceType::Headset),
            ("00:1A:2B:3C:4D:5F", "Wireless Mouse", BluetoothDeviceType::Mouse),
            ("00:1A:2B:3C:4D:60", "Bluetooth Keyboard", BluetoothDeviceType::Keyboard),
            ("00:1A:2B:3C:4D:61", "Phone", BluetoothDeviceType::Phone),
        ];

        for (address, name, device_type) in discovered_devices {
            let mut device = BLEDevice::new(
                address.to_string(),
                name.to_string(),
                device_type,
            );
            device.set_rssi(-60);
            self.devices.insert(address.to_string(), device);
        }

        self.devices.values().collect()
    }

    /// Connect to a device
    pub fn connect(&mut self, address: &str) -> Result<(), String> {
        let device = self.get_device_mut(address)
            .ok_or_else(|| format!("Device {} not found", address))?;

        if device.is_blocked {
            return Err(format!("Device {} is blocked", address));
        }

        device.set_status(BluetoothDeviceStatus::Connecting);

        // Simulate connection
        device.set_status(BluetoothDeviceStatus::Connected);
        device.set_trusted(true);

        Ok(())
    }

    /// Disconnect from a device
    pub fn disconnect(&mut self, address: &str) -> Result<(), String> {
        let device = self.get_device_mut(address)
            .ok_or_else(|| format!("Device {} not found", address))?;

        device.set_status(BluetoothDeviceStatus::Disconnected);
        Ok(())
    }

    /// Pair a device
    pub fn pair(&mut self, address: &str) -> Result<(), String> {
        let device = self.get_device_mut(address)
            .ok_or_else(|| format!("Device {} not found", address))?;

        device.set_status(BluetoothDeviceStatus::Paired);
        device.set_trusted(true);

        Ok(())
    }

    /// Unpair a device
    pub fn unpair(&mut self, address: &str) -> Result<(), String> {
        let device = self.get_device_mut(address)
            .ok_or_else(|| format!("Device {} not found", address))?;

        device.set_status(BluetoothDeviceStatus::Disconnected);
        device.set_trusted(false);

        Ok(())
    }

    /// Trust a device
    pub fn trust(&mut self, address: &str) -> Result<(), String> {
        let device = self.get_device_mut(address)
            .ok_or_else(|| format!("Device {} not found", address))?;

        device.set_trusted(true);
        Ok(())
    }

    /// Block a device
    pub fn block(&mut self, address: &str) -> Result<(), String> {
        let device = self.get_device_mut(address)
            .ok_or_else(|| format!("Device {} not found", address))?;

        device.set_blocked(true);
        if device.is_connected() {
            let _ = self.disconnect(address);
        }

        Ok(())
    }

    /// Remove a device
    pub fn remove(&mut self, address: &str) -> Result<(), String> {
        self.devices.remove(address)
            .ok_or_else(|| format!("Device {} not found", address))?;
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> BluetoothStatistics {
        let total_devices = self.devices.len();
        let connected_count = self.devices.values()
            .filter(|d| d.is_connected())
            .count();
        let paired_count = self.devices.values()
            .filter(|d| d.status == BluetoothDeviceStatus::Paired)
            .count();
        let trusted_count = self.devices.values()
            .filter(|d| d.is_trusted)
            .count();
        let blocked_count = self.devices.values()
            .filter(|d| d.is_blocked)
            .count();

        BluetoothStatistics {
            total_devices,
            connected_count,
            paired_count,
            trusted_count,
            blocked_count,
            adapter_powered: self.adapter.as_ref().map(|a| a.is_powered).unwrap_or(false),
        }
    }
}

impl Default for BluetoothDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Bluetooth statistics
#[derive(Debug, Clone)]
pub struct BluetoothStatistics {
    pub total_devices: usize,
    pub connected_count: usize,
    pub paired_count: usize,
    pub trusted_count: usize,
    pub blocked_count: usize,
    pub adapter_powered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_from_str() {
        assert_eq!(BluetoothDeviceType::from_str("headset"), Some(BluetoothDeviceType::Headset));
        assert_eq!(BluetoothDeviceType::from_str("mouse"), Some(BluetoothDeviceType::Mouse));
    }

    #[test]
    fn test_device_status_from_str() {
        assert_eq!(BluetoothDeviceStatus::from_str("connected"), Some(BluetoothDeviceStatus::Connected));
        assert_eq!(BluetoothDeviceStatus::from_str("paired"), Some(BluetoothDeviceStatus::Paired));
    }

    #[test]
    fn test_ble_device_creation() {
        let device = BLEDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Unknown,
        );
        assert_eq!(device.name, "Test Device");
    }

    #[test]
    fn test_bluetooth_manager_creation() {
        let manager = BluetoothDeviceManager::new();
        assert!(manager.get_adapter().is_some());
    }

    #[test]
    fn test_scan() {
        let mut manager = BluetoothDeviceManager::new();
        let devices = manager.scan();
        assert!(devices.len() >= 4);
    }

    #[test]
    fn test_connect() {
        let mut manager = BluetoothDeviceManager::new();
        manager.scan();
        assert!(manager.connect("00:1A:2B:3C:4D:5E").is_ok());
    }

    #[test]
    fn test_disconnect() {
        let mut manager = BluetoothDeviceManager::new();
        manager.scan();
        manager.connect("00:1A:2B:3C:4D:5E").ok();
        assert!(manager.disconnect("00:1A:2B:3C:4D:5E").is_ok());
    }

    #[test]
    fn test_pair() {
        let mut manager = BluetoothDeviceManager::new();
        manager.scan();
        assert!(manager.pair("00:1A:2B:3C:4D:5E").is_ok());
    }

    #[test]
    fn test_block() {
        let mut manager = BluetoothDeviceManager::new();
        manager.scan();
        assert!(manager.block("00:1A:2B:3C:4D:5E").is_ok());
    }

    #[test]
    fn test_connect_blocked_fails() {
        let mut manager = BluetoothDeviceManager::new();
        manager.scan();
        manager.block("00:1A:2B:3C:4D:5E").ok();
        assert!(manager.connect("00:1A:2B:3C:4D:5E").is_err());
    }

    #[test]
    fn test_statistics() {
        let mut manager = BluetoothDeviceManager::new();
        manager.scan();
        let stats = manager.get_statistics();
        assert!(stats.total_devices >= 4);
    }
}
