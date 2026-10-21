// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Bluetooth device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothState {
    Disconnected,
    Connected,
    Paired,
    Connecting,
}

/// Bluetooth device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothType {
    Audio,
    Input,
    Output,
    Phone,
    Other,
}

/// Bluetooth device
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub id: String,
    pub name: String,
    pub mac_address: String,
    pub device_type: BluetoothType,
    pub state: BluetoothState,
    pub is_paired: bool,
}

/// Bluetooth manager for managing Bluetooth devices
/// Inspired by Omarchy bluetooth- commands and Linux Bluetooth management
#[derive(Debug, Clone)]
pub struct SigmaBluetoothManager2 {
    devices: Vec<BluetoothDevice>,
    adapter_enabled: bool,
}

impl SigmaBluetoothManager2 {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            adapter_enabled: true,
        }
    }

    /// Add a Bluetooth device
    pub fn add_device(&mut self, device: BluetoothDevice) {
        self.devices.push(device);
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&BluetoothDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    /// Get all devices
    pub fn get_devices(&self) -> &[BluetoothDevice] {
        &self.devices
    }

    /// Connect to device
    pub fn connect(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.state = BluetoothState::Connected;
            Ok(format!("Connected to: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Disconnect from device
    pub fn disconnect(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.state = BluetoothState::Disconnected;
            Ok(format!("Disconnected from: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Pair device
    pub fn pair(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.is_paired = true;
            Ok(format!("Paired with: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Unpair device
    pub fn unpair(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.is_paired = false;
            device.state = BluetoothState::Disconnected;
            Ok(format!("Unpaired from: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Enable adapter
    pub fn enable_adapter(&mut self) -> Result<String, String> {
        self.adapter_enabled = true;
        Ok("Bluetooth adapter enabled".to_string())
    }

    /// Disable adapter
    pub fn disable_adapter(&mut self) -> Result<String, String> {
        self.adapter_enabled = false;
        Ok("Bluetooth adapter disabled".to_string())
    }

    /// Get paired devices
    pub fn get_paired_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.iter().filter(|d| d.is_paired).collect()
    }

    /// Get connected devices
    pub fn get_connected_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices
            .iter()
            .filter(|d| d.state == BluetoothState::Connected)
            .collect()
    }
}

impl Default for SigmaBluetoothManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_device() {
        let mut manager = SigmaBluetoothManager2::new();
        let device = BluetoothDevice {
            id: "bt-1".to_string(),
            name: "Headphones".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            device_type: BluetoothType::Audio,
            state: BluetoothState::Disconnected,
            is_paired: false,
        };
        manager.add_device(device);
        assert_eq!(manager.get_devices().len(), 1);
    }

    #[test]
    fn test_connect() {
        let mut manager = SigmaBluetoothManager2::new();
        let device = BluetoothDevice {
            id: "bt-1".to_string(),
            name: "Headphones".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            device_type: BluetoothType::Audio,
            state: BluetoothState::Disconnected,
            is_paired: true,
        };
        manager.add_device(device);
        manager.connect("bt-1").unwrap();
        assert_eq!(
            manager.get_device("bt-1").unwrap().state,
            BluetoothState::Connected
        );
    }

    #[test]
    fn test_pair() {
        let mut manager = SigmaBluetoothManager2::new();
        let device = BluetoothDevice {
            id: "bt-1".to_string(),
            name: "Headphones".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            device_type: BluetoothType::Audio,
            state: BluetoothState::Disconnected,
            is_paired: false,
        };
        manager.add_device(device);
        manager.pair("bt-1").unwrap();
        assert!(manager.get_device("bt-1").unwrap().is_paired);
    }

    #[test]
    fn test_get_paired_devices() {
        let mut manager = SigmaBluetoothManager2::new();
        let device1 = BluetoothDevice {
            id: "bt-1".to_string(),
            name: "Headphones".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            device_type: BluetoothType::Audio,
            state: BluetoothState::Disconnected,
            is_paired: true,
        };
        let device2 = BluetoothDevice {
            id: "bt-2".to_string(),
            name: "Mouse".to_string(),
            mac_address: "00:11:22:33:44:56".to_string(),
            device_type: BluetoothType::Input,
            state: BluetoothState::Disconnected,
            is_paired: false,
        };
        manager.add_device(device1);
        manager.add_device(device2);
        assert_eq!(manager.get_paired_devices().len(), 1);
    }
}
