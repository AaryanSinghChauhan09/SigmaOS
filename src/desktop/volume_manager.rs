// Desktop Volume Manager
// Linux Mint & Omarchy inspiration for comprehensive audio volume management

use std::collections::HashMap;

/// Desktop Volume Device Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopVolumeDeviceType {
    Output,
    Input,
}

impl DesktopVolumeDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopVolumeDeviceType::Output => "output",
            DesktopVolumeDeviceType::Input => "input",
        }
    }
}

/// Desktop Volume Device Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopVolumeDeviceStatus {
    Active,
    Idle,
    Unplugged,
}

impl DesktopVolumeDeviceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopVolumeDeviceStatus::Active => "active",
            DesktopVolumeDeviceStatus::Idle => "idle",
            DesktopVolumeDeviceStatus::Unplugged => "unplugged",
        }
    }
}

/// Desktop Volume Device
#[derive(Debug, Clone)]
pub struct DesktopVolumeDevice {
    pub id: String,
    pub name: String,
    pub device_type: DesktopVolumeDeviceType,
    pub status: DesktopVolumeDeviceStatus,
    pub volume: u32,  // 0-100
    pub muted: bool,
    pub default: bool,
}

impl DesktopVolumeDevice {
    pub fn new(id: String, name: String, device_type: DesktopVolumeDeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            status: DesktopVolumeDeviceStatus::Idle,
            volume: 100,
            muted: false,
            default: false,
        }
    }

    pub fn set_volume(&mut self, volume: u32) {
        self.volume = volume.min(100);
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }
}

/// Desktop Volume Manager
pub struct DesktopVolumeManager {
    devices: HashMap<String, DesktopVolumeDevice>,
    default_output: Option<String>,
    default_input: Option<String>,
    system_volume: u32,  // 0-100
    system_muted: bool,
    counter: u32,
}

impl DesktopVolumeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            devices: HashMap::new(),
            default_output: None,
            default_input: None,
            system_volume: 100,
            system_muted: false,
            counter: 0,
        };

        // Add default devices
        manager.add_default_devices();

        manager
    }

    fn add_default_devices(&mut self) {
        // Add default output device
        let mut output = DesktopVolumeDevice::new(
            "output_0".to_string(),
            "Built-in Audio".to_string(),
            DesktopVolumeDeviceType::Output,
        );

        output.status = DesktopVolumeDeviceStatus::Active;
        output.default = true;

        let output_id = output.id.clone();
        self.devices.insert(output_id.clone(), output);
        self.default_output = Some(output_id);

        // Add default input device
        let mut input = DesktopVolumeDevice::new(
            "input_0".to_string(),
            "Built-in Microphone".to_string(),
            DesktopVolumeDeviceType::Input,
        );

        input.status = DesktopVolumeDeviceStatus::Idle;
        input.default = true;

        let input_id = input.id.clone();
        self.devices.insert(input_id.clone(), input);
        self.default_input = Some(input_id);
    }

    pub fn add_device(&mut self, device: DesktopVolumeDevice) -> String {
        let id = format!("device_{}", self.counter);
        self.counter += 1;

        let device = DesktopVolumeDevice {
            id: id.clone(),
            ..device
        };

        self.devices.insert(id.clone(), device);
        id
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.default_output ||
           Some(id.to_string()) == self.default_input {
            return false;
        }
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&DesktopVolumeDevice> {
        self.devices.get(id)
    }

    pub fn get_devices(&self) -> Vec<&DesktopVolumeDevice> {
        self.devices.values().collect()
    }

    pub fn set_default_output(&mut self, id: &str) -> bool {
        if !self.devices.contains_key(id) {
            return false;
        }

        // Unset default from previous
        if let Some(prev_id) = &self.default_output {
            if let Some(device) = self.devices.get_mut(prev_id) {
                device.default = false;
            }
        }

        // Set new default
        if let Some(device) = self.devices.get_mut(id) {
            device.default = true;
            self.default_output = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_default_output(&self) -> Option<&DesktopVolumeDevice> {
        self.default_output
            .as_ref()
            .and_then(|id| self.devices.get(id))
    }

    pub fn set_default_input(&mut self, id: &str) -> bool {
        if !self.devices.contains_key(id) {
            return false;
        }

        // Unset default from previous
        if let Some(prev_id) = &self.default_input {
            if let Some(device) = self.devices.get_mut(prev_id) {
                device.default = false;
            }
        }

        // Set new default
        if let Some(device) = self.devices.get_mut(id) {
            device.default = true;
            self.default_input = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_default_input(&self) -> Option<&DesktopVolumeDevice> {
        self.default_input
            .as_ref()
            .and_then(|id| self.devices.get(id))
    }

    pub fn set_device_volume(&mut self, id: &str, volume: u32) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_volume(volume);
            true
        } else {
            false
        }
    }

    pub fn set_device_muted(&mut self, id: &str, muted: bool) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_muted(muted);
            true
        } else {
            false
        }
    }

    pub fn set_system_volume(&mut self, volume: u32) {
        self.system_volume = volume.min(100);
    }

    pub fn get_system_volume(&self) -> u32 {
        self.system_volume
    }

    pub fn set_system_muted(&mut self, muted: bool) {
        self.system_muted = muted;
    }

    pub fn get_system_muted(&self) -> bool {
        self.system_muted
    }

    pub fn get_output_devices(&self) -> Vec<&DesktopVolumeDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == DesktopVolumeDeviceType::Output)
            .collect()
    }

    pub fn get_input_devices(&self) -> Vec<&DesktopVolumeDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == DesktopVolumeDeviceType::Input)
            .collect()
    }

    pub fn get_active_devices(&self) -> Vec<&DesktopVolumeDevice> {
        self.devices
            .values()
            .filter(|d| d.status == DesktopVolumeDeviceStatus::Active)
            .collect()
    }

    pub fn get_statistics(&self) -> DesktopVolumeManagerStatistics {
        DesktopVolumeManagerStatistics {
            total_devices: self.devices.len(),
            output_devices: self.get_output_devices().len(),
            input_devices: self.get_input_devices().len(),
            active_devices: self.get_active_devices().len(),
            default_output_set: self.default_output.is_some(),
            default_input_set: self.default_input.is_some(),
            system_volume: self.system_volume,
            system_muted: self.system_muted,
        }
    }
}

impl Default for DesktopVolumeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopVolumeManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopVolumeManagerStatistics {
    pub total_devices: usize,
    pub output_devices: usize,
    pub input_devices: usize,
    pub active_devices: usize,
    pub default_output_set: bool,
    pub default_input_set: bool,
    pub system_volume: u32,
    pub system_muted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopVolumeManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_devices, 2);
        assert_eq!(stats.output_devices, 1);
        assert_eq!(stats.input_devices, 1);
        assert_eq!(stats.active_devices, 1);
        assert!(stats.default_output_set);
        assert!(stats.default_input_set);
        assert_eq!(stats.system_volume, 100);
        assert!(!stats.system_muted);
    }

    #[test]
    fn test_add_device() {
        let mut manager = DesktopVolumeManager::new();
        let initial_count = manager.get_devices().len();

        let device = DesktopVolumeDevice::new(
            "Headphones".to_string(),
            "Headphones".to_string(),
            DesktopVolumeDeviceType::Output,
        );

        let id = manager.add_device(device);
        assert!(manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_device() {
        let mut manager = DesktopVolumeManager::new();

        let device = DesktopVolumeDevice::new(
            "Headphones".to_string(),
            "Headphones".to_string(),
            DesktopVolumeDeviceType::Output,
        );

        let id = manager.add_device(device);
        assert!(manager.remove_device(&id));
        assert!(manager.get_device(&id).is_none());
    }

    #[test]
    fn test_remove_default_device() {
        let mut manager = DesktopVolumeManager::new();

        // Try to remove default output (should fail)
        let default_output = manager.get_default_output().unwrap();
        let result = manager.remove_device(&default_output.id);
        assert!(!result);
    }

    #[test]
    fn test_set_default_output() {
        let mut manager = DesktopVolumeManager::new();

        let device = DesktopVolumeDevice::new(
            "Headphones".to_string(),
            "Headphones".to_string(),
            DesktopVolumeDeviceType::Output,
        );

        let id = manager.add_device(device);
        assert!(manager.set_default_output(&id));

        let default = manager.get_default_output().unwrap();
        assert_eq!(default.id, id);
    }

    #[test]
    fn test_set_default_input() {
        let mut manager = DesktopVolumeManager::new();

        let device = DesktopVolumeDevice::new(
            "External Mic".to_string(),
            "External Mic".to_string(),
            DesktopVolumeDeviceType::Input,
        );

        let id = manager.add_device(device);
        assert!(manager.set_default_input(&id));

        let default = manager.get_default_input().unwrap();
        assert_eq!(default.id, id);
    }

    #[test]
    fn test_device_volume() {
        let mut manager = DesktopVolumeManager::new();
        let device_id = "output_0";

        assert!(manager.set_device_volume(device_id, 75));

        let device = manager.get_device(device_id).unwrap();
        assert_eq!(device.volume, 75);

        assert!(manager.set_device_volume(device_id, 150));

        let device = manager.get_device(device_id).unwrap();
        assert_eq!(device.volume, 100);
    }

    #[test]
    fn test_device_muted() {
        let mut manager = DesktopVolumeManager::new();
        let device_id = "output_0";

        assert!(manager.set_device_muted(device_id, true));

        let device = manager.get_device(device_id).unwrap();
        assert!(device.muted);
    }

    #[test]
    fn test_system_volume() {
        let mut manager = DesktopVolumeManager::new();

        manager.set_system_volume(75);
        assert_eq!(manager.get_system_volume(), 75);

        manager.set_system_volume(150);
        assert_eq!(manager.get_system_volume(), 100);
    }

    #[test]
    fn test_system_muted() {
        let mut manager = DesktopVolumeManager::new();

        manager.set_system_muted(true);
        assert!(manager.get_system_muted());
    }

    #[test]
    fn test_output_devices() {
        let manager = DesktopVolumeManager::new();

        let output = manager.get_output_devices();
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn test_input_devices() {
        let manager = DesktopVolumeManager::new();

        let input = manager.get_input_devices();
        assert_eq!(input.len(), 1);
    }

    #[test]
    fn test_active_devices() {
        let manager = DesktopVolumeManager::new();

        let active = manager.get_active_devices();
        assert_eq!(active.len(), 1);
    }
}
