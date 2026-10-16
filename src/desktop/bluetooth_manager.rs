// SPDX-License-Identifier: MIT
// SigmaOS - Bluetooth Manager
// Inspired by GNOME Bluetooth and Blueman
// Provides Bluetooth device management

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Bluetooth device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Unknown,
    Computer,
    Phone,
    Audio,
    Video,
    Peripheral,
    Keyboard,
    Mouse,
    Joystick,
    Tablet,
}

/// Bluetooth device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    Available,
    Paired,
    Connected,
    Connecting,
    Disconnecting,
}

/// Bluetooth device
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub address: String,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub battery_level: Option<u8>,
    pub rssi: i32,
}

/// Bluetooth adapter
#[derive(Debug, Clone)]
pub struct BluetoothAdapter {
    pub name: String,
    pub address: String,
    pub powered: bool,
    pub discoverable: bool,
    pub pairable: bool,
}

/// Bluetooth manager
pub struct BluetoothManager {
    pub adapters: Vec<BluetoothAdapter>,
    pub devices: Vec<BluetoothDevice>,
    pub current_adapter: Option<usize>,
    pub scanning: bool,
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
            devices: Vec::new(),
            current_adapter: None,
            scanning: false,
        }
    }

    /// Add adapter
    pub fn add_adapter(&mut self, name: String, address: String) {
        let adapter = BluetoothAdapter {
            name,
            address,
            powered: true,
            discoverable: true,
            pairable: true,
        };
        self.adapters.push(adapter);
    }

    /// Set current adapter
    pub fn set_current_adapter(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.adapters.len() {
            return Err("Adapter index out of bounds");
        }
        self.current_adapter = Some(index);
        Ok(())
    }

    /// Start scanning
    pub fn start_scan(&mut self) -> Result<(), &'static str> {
        if self.current_adapter.is_none() {
            return Err("No adapter selected");
        }
        self.scanning = true;
        Ok(())
    }

    /// Stop scanning
    pub fn stop_scan(&mut self) {
        self.scanning = false;
    }

    /// Pair device
    pub fn pair_device(&mut self, address: String) -> Result<(), &'static str> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.address == address)
            .ok_or("Device not found")?;
        device.status = DeviceStatus::Paired;
        Ok(())
    }

    /// Unpair device
    pub fn unpair_device(&mut self, address: String) -> Result<(), &'static str> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.address == address)
            .ok_or("Device not found")?;
        device.status = DeviceStatus::Available;
        Ok(())
    }

    /// Connect device
    pub fn connect_device(&mut self, address: String) -> Result<(), &'static str> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.address == address)
            .ok_or("Device not found")?;
        device.status = DeviceStatus::Connected;
        Ok(())
    }

    /// Disconnect device
    pub fn disconnect_device(&mut self, address: String) -> Result<(), &'static str> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.address == address)
            .ok_or("Not found")?;
        device.status = DeviceStatus::Paired;
        Ok(())
    }

    /// Get devices
    pub fn get_devices(&self) -> &[BluetoothDevice] {
        &self.devices
    }

    /// Get paired devices
    pub fn get_paired_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices
            .iter()
            .filter(|d| d.status == DeviceStatus::Paired || d.status == DeviceStatus::Connected)
            .collect()
    }

    /// Device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

/// Global bluetooth manager
static mut BLUETOOTH_MANAGER: Option<BluetoothManager> = None;

/// Initialize bluetooth manager
pub fn init_bluetooth_manager() {
    unsafe {
        BLUETOOTH_MANAGER = Some(BluetoothManager::new());
    }
}

/// Get bluetooth manager
pub fn get_bluetooth_manager() -> &'static mut BluetoothManager {
    unsafe {
        BLUETOOTH_MANAGER
            .as_mut()
            .expect("Bluetooth manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_manager_creation() {
        let bm = BluetoothManager::new();
        assert_eq!(bm.device_count(), 0);
    }

    #[test]
    fn test_add_adapter() {
        let mut bm = BluetoothManager::new();
        bm.add_adapter("hci0".to_string(), "00:11:22:33:44:55".to_string());
        assert_eq!(bm.adapters.len(), 1);
    }

    #[test]
    fn test_pair_device() {
        let mut bm = BluetoothManager::new();
        bm.devices.push(BluetoothDevice {
            address: "00:11:22:33:44:55".to_string(),
            name: "Test Device".to_string(),
            device_type: DeviceType::Unknown,
            status: DeviceStatus::Available,
            battery_level: None,
            rssi: -50,
        });
        bm.pair_device("00:11:22:33:44:55".to_string()).unwrap();
        assert_eq!(bm.devices[0].status, DeviceStatus::Paired);
    }
}
