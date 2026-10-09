// Desktop Bluetooth Manager
// Linux Mint & Omarchy inspiration for comprehensive Bluetooth management

use std::collections::HashMap;

/// Desktop Bluetooth Device Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopBluetoothDeviceType {
    Audio,
    Input,
    Network,
    Printer,
    Other,
}

impl DesktopBluetoothDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopBluetoothDeviceType::Audio => "audio",
            DesktopBluetoothDeviceType::Input => "input",
            DesktopBluetoothDeviceType::Network => "network",
            DesktopBluetoothDeviceType::Printer => "printer",
            DesktopBluetoothDeviceType::Other => "other",
        }
    }
}

/// Desktop Bluetooth Device Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopBluetoothDeviceStatus {
    Disconnected,
    Connected,
    Paired,
    Discovering,
}

impl DesktopBluetoothDeviceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopBluetoothDeviceStatus::Disconnected => "disconnected",
            DesktopBluetoothDeviceStatus::Connected => "connected",
            DesktopBluetoothDeviceStatus::Paired => "paired",
            DesktopBluetoothDeviceStatus::Discovering => "discovering",
        }
    }
}

/// Desktop Bluetooth Device
#[derive(Debug, Clone)]
pub struct DesktopBluetoothDevice {
    pub id: String,
    pub name: String,
    pub mac_address: String,
    pub device_type: DesktopBluetoothDeviceType,
    pub status: DesktopBluetoothDeviceStatus,
    pub paired: bool,
    pub trusted: bool,
    pub connected: bool,
}

impl DesktopBluetoothDevice {
    pub fn new(
        id: String,
        name: String,
        mac_address: String,
        device_type: DesktopBluetoothDeviceType,
    ) -> Self {
        Self {
            id,
            name,
            mac_address,
            device_type,
            status: DesktopBluetoothDeviceStatus::Disconnected,
            paired: false,
            trusted: false,
            connected: false,
        }
    }

    pub fn set_status(&mut self, status: DesktopBluetoothDeviceStatus) {
        self.status = status;
    }

    pub fn set_paired(&mut self, paired: bool) {
        self.paired = paired;
    }

    pub fn set_trusted(&mut self, trusted: bool) {
        self.trusted = trusted;
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
    }
}

/// Desktop Bluetooth Manager
pub struct DesktopBluetoothManager {
    devices: HashMap<String, DesktopBluetoothDevice>,
    adapter_enabled: bool,
    adapter_name: String,
    discoverable: bool,
    scanning: bool,
    counter: u32,
}

impl DesktopBluetoothManager {
    pub fn new() -> Self {
        let mut manager = Self {
            devices: HashMap::new(),
            adapter_enabled: true,
            adapter_name: "Default Adapter".to_string(),
            discoverable: false,
            scanning: false,
            counter: 1000,
        };

        // Add default devices
        manager.add_default_devices();

        manager
    }

    fn add_default_devices(&mut self) {
        let headphones = DesktopBluetoothDevice::new(
            "device_0".to_string(),
            "Wireless Headphones".to_string(),
            "00:11:22:33:44:55".to_string(),
            DesktopBluetoothDeviceType::Audio,
        );

        let keyboard = DesktopBluetoothDevice::new(
            "device_1".to_string(),
            "Bluetooth Keyboard".to_string(),
            "00:11:22:33:44:56".to_string(),
            DesktopBluetoothDeviceType::Input,
        );

        self.devices.insert(headphones.id.clone(), headphones);
        self.devices.insert(keyboard.id.clone(), keyboard);
    }

    pub fn add_device(&mut self, device: DesktopBluetoothDevice) -> String {
        let id = format!("device_{}", self.counter);
        self.counter += 1;

        let device = DesktopBluetoothDevice {
            id: id.clone(),
            ..device
        };

        self.devices.insert(id.clone(), device);
        id
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&DesktopBluetoothDevice> {
        self.devices.get(id)
    }

    pub fn get_devices(&self) -> Vec<&DesktopBluetoothDevice> {
        self.devices.values().collect()
    }

    pub fn get_devices_by_type(
        &self,
        device_type: DesktopBluetoothDeviceType,
    ) -> Vec<&DesktopBluetoothDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    pub fn get_devices_by_status(
        &self,
        status: DesktopBluetoothDeviceStatus,
    ) -> Vec<&DesktopBluetoothDevice> {
        self.devices
            .values()
            .filter(|d| d.status == status)
            .collect()
    }

    pub fn set_device_status(&mut self, id: &str, status: DesktopBluetoothDeviceStatus) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_status(status);
            true
        } else {
            false
        }
    }

    pub fn pair_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_paired(true);
            device.set_trusted(true);
            device.set_status(DesktopBluetoothDeviceStatus::Paired);
            true
        } else {
            false
        }
    }

    pub fn unpair_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_paired(false);
            device.set_trusted(false);
            device.set_status(DesktopBluetoothDeviceStatus::Disconnected);
            device.set_connected(false);
            true
        } else {
            false
        }
    }

    pub fn connect_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            if device.paired {
                device.set_connected(true);
                device.set_status(DesktopBluetoothDeviceStatus::Connected);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn disconnect_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_connected(false);
            device.set_status(DesktopBluetoothDeviceStatus::Disconnected);
            true
        } else {
            false
        }
    }

    pub fn set_adapter_enabled(&mut self, enabled: bool) {
        self.adapter_enabled = enabled;
    }

    pub fn is_adapter_enabled(&self) -> bool {
        self.adapter_enabled
    }

    pub fn set_adapter_name(&mut self, name: String) {
        self.adapter_name = name;
    }

    pub fn get_adapter_name(&self) -> &str {
        &self.adapter_name
    }

    pub fn set_discoverable(&mut self, discoverable: bool) {
        self.discoverable = discoverable;
    }

    pub fn is_discoverable(&self) -> bool {
        self.discoverable
    }

    pub fn set_scanning(&mut self, scanning: bool) {
        self.scanning = scanning;
    }

    pub fn is_scanning(&self) -> bool {
        self.scanning
    }

    pub fn get_statistics(&self) -> DesktopBluetoothManagerStatistics {
        DesktopBluetoothManagerStatistics {
            total_devices: self.devices.len(),
            paired_devices: self.devices.values().filter(|d| d.paired).count(),
            connected_devices: self.devices.values().filter(|d| d.connected).count(),
            audio_devices: self
                .get_devices_by_type(DesktopBluetoothDeviceType::Audio)
                .len(),
            input_devices: self
                .get_devices_by_type(DesktopBluetoothDeviceType::Input)
                .len(),
            adapter_enabled: self.adapter_enabled,
            scanning: self.scanning,
        }
    }
}

impl Default for DesktopBluetoothManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopBluetoothManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopBluetoothManagerStatistics {
    pub total_devices: usize,
    pub paired_devices: usize,
    pub connected_devices: usize,
    pub audio_devices: usize,
    pub input_devices: usize,
    pub adapter_enabled: bool,
    pub scanning: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopBluetoothManager::new();
        let stats = manager.get_statistics();

        assert!(stats.total_devices >= 2);
        assert!(stats.adapter_enabled);
        assert!(!stats.scanning);
    }

    #[test]
    fn test_add_device() {
        let mut manager = DesktopBluetoothManager::new();
        let initial_count = manager.get_devices().len();

        let device = DesktopBluetoothDevice::new(
            "custom".to_string(),
            "Custom Device".to_string(),
            "00:11:22:33:44:57".to_string(),
            DesktopBluetoothDeviceType::Audio,
        );

        let id = manager.add_device(device);
        assert!(manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_device() {
        let mut manager = DesktopBluetoothManager::new();

        let device = DesktopBluetoothDevice::new(
            "custom".to_string(),
            "Custom Device".to_string(),
            "00:11:22:33:44:57".to_string(),
            DesktopBluetoothDeviceType::Audio,
        );

        let id = manager.add_device(device);
        assert!(manager.remove_device(&id));
        assert!(manager.get_device(&id).is_none());
    }

    #[test]
    fn test_pair_device() {
        let mut manager = DesktopBluetoothManager::new();
        let device_id = "device_0";

        assert!(manager.pair_device(device_id));
        let device = manager.get_device(device_id).unwrap();
        assert!(device.paired);
        assert!(device.trusted);
    }

    #[test]
    fn test_unpair_device() {
        let mut manager = DesktopBluetoothManager::new();
        let device_id = "device_0";

        manager.pair_device(device_id);
        assert!(manager.unpair_device(device_id));

        let device = manager.get_device(device_id).unwrap();
        assert!(!device.paired);
        assert!(!device.trusted);
    }

    #[test]
    fn test_connect_device() {
        let mut manager = DesktopBluetoothManager::new();
        let device_id = "device_0";

        manager.pair_device(device_id);
        assert!(manager.connect_device(device_id));

        let device = manager.get_device(device_id).unwrap();
        assert!(device.connected);
    }

    #[test]
    fn test_connect_unpaired_device() {
        let mut manager = DesktopBluetoothManager::new();
        let device_id = "device_0";

        // Try to connect without pairing
        assert!(!manager.connect_device(device_id));
    }

    #[test]
    fn test_disconnect_device() {
        let mut manager = DesktopBluetoothManager::new();
        let device_id = "device_0";

        manager.pair_device(device_id);
        manager.connect_device(device_id);
        assert!(manager.disconnect_device(device_id));

        let device = manager.get_device(device_id).unwrap();
        assert!(!device.connected);
    }

    #[test]
    fn test_adapter_configuration() {
        let mut manager = DesktopBluetoothManager::new();

        manager.set_adapter_enabled(false);
        assert!(!manager.is_adapter_enabled());

        manager.set_adapter_name("My Adapter".to_string());
        assert_eq!(manager.get_adapter_name(), "My Adapter");

        manager.set_discoverable(true);
        assert!(manager.is_discoverable());

        manager.set_scanning(true);
        assert!(manager.is_scanning());
    }

    #[test]
    fn test_get_devices_by_type() {
        let manager = DesktopBluetoothManager::new();

        let audio = manager.get_devices_by_type(DesktopBluetoothDeviceType::Audio);
        let input = manager.get_devices_by_type(DesktopBluetoothDeviceType::Input);

        assert!(!audio.is_empty());
        assert!(!input.is_empty());

        for device in audio {
            assert_eq!(device.device_type, DesktopBluetoothDeviceType::Audio);
        }

        for device in input {
            assert_eq!(device.device_type, DesktopBluetoothDeviceType::Input);
        }
    }

    #[test]
    fn test_get_devices_by_status() {
        let mut manager = DesktopBluetoothManager::new();
        let device_id = "device_0";

        manager.pair_device(device_id);

        let paired = manager.get_devices_by_status(DesktopBluetoothDeviceStatus::Paired);
        assert!(!paired.is_empty());

        let disconnected =
            manager.get_devices_by_status(DesktopBluetoothDeviceStatus::Disconnected);
        assert!(!disconnected.is_empty());
    }
}
