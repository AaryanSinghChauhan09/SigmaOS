// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaBluetooth - Bluetooth Daemon
//!
//! SigmaBluetooth provides Bluetooth management for SigmaOS, inspired by BlueZ from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Bluetooth device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothDeviceType {
    /// Unknown device
    Unknown,
    /// Phone
    Phone,
    /// Computer
    Computer,
    /// Headset
    Headset,
    /// Headphones
    Headphones,
    /// Audio device
    Audio,
    /// Keyboard
    Keyboard,
    /// Mouse
    Mouse,
    /// Joypad
    Joypad,
    /// Tablet
    Tablet,
    /// Peripheral
    Peripheral,
}

impl fmt::Display for BluetoothDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BluetoothDeviceType::Unknown => write!(f, "unknown"),
            BluetoothDeviceType::Phone => write!(f, "phone"),
            BluetoothDeviceType::Computer => write!(f, "computer"),
            BluetoothDeviceType::Headset => write!(f, "headset"),
            BluetoothDeviceType::Headphones => write!(f, "headphones"),
            BluetoothDeviceType::Audio => write!(f, "audio"),
            BluetoothDeviceType::Keyboard => write!(f, "keyboard"),
            BluetoothDeviceType::Mouse => write!(f, "mouse"),
            BluetoothDeviceType::Joypad => write!(f, "joypad"),
            BluetoothDeviceType::Tablet => write!(f, "tablet"),
            BluetoothDeviceType::Peripheral => write!(f, "peripheral"),
        }
    }
}

/// Bluetooth device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothDeviceStatus {
    /// Unknown status
    Unknown,
    /// Device is available
    Available,
    /// Device is paired
    Paired,
    /// Device is connected
    Connected,
    /// Device is disconnected
    Disconnected,
    /// Device is pairing
    Pairing,
}

impl fmt::Display for BluetoothDeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BluetoothDeviceStatus::Unknown => write!(f, "unknown"),
            BluetoothDeviceStatus::Available => write!(f, "available"),
            BluetoothDeviceStatus::Paired => write!(f, "paired"),
            BluetoothDeviceStatus::Connected => write!(f, "connected"),
            BluetoothDeviceStatus::Disconnected => write!(f, "disconnected"),
            BluetoothDeviceStatus::Pairing => write!(f, "pairing"),
        }
    }
}

/// Bluetooth adapter state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterState {
    /// Powered off
    PoweredOff,
    /// Powered on
    PoweredOn,
    /// Discovering
    Discovering,
}

impl fmt::Display for AdapterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdapterState::PoweredOff => write!(f, "powered_off"),
            AdapterState::PoweredOn => write!(f, "powered_on"),
            AdapterState::Discovering => write!(f, "discovering"),
        }
    }
}

/// Bluetooth device
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    /// Device address (MAC)
    pub address: String,
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: BluetoothDeviceType,
    /// Device status
    pub status: BluetoothDeviceStatus,
    /// Signal strength (RSSI)
    pub rssi: Option<i16>,
    /// Battery level (percentage)
    pub battery: Option<u8>,
}

impl BluetoothDevice {
    /// Create a new Bluetooth device
    pub fn new(address: String, name: String, device_type: BluetoothDeviceType) -> Self {
        Self {
            address,
            name,
            device_type,
            status: BluetoothDeviceStatus::Available,
            rssi: None,
            battery: None,
        }
    }

    /// Set status
    pub fn with_status(mut self, status: BluetoothDeviceStatus) -> Self {
        self.status = status;
        self
    }

    /// Set RSSI
    pub fn with_rssi(mut self, rssi: i16) -> Self {
        self.rssi = Some(rssi);
        self
    }

    /// Set battery level
    pub fn with_battery(mut self, battery: u8) -> Self {
        self.battery = Some(battery);
        self
    }
}

/// SigmaBluetooth - Bluetooth Daemon
pub struct SigmaBluetooth {
    /// Bluetooth devices (address -> device)
    devices: BTreeMap<String, BluetoothDevice>,
    /// Adapter state
    adapter_state: AdapterState,
    /// Running state
    running: bool,
    /// Discovering flag
    discovering: bool,
    /// Device counter
    device_counter: u32,
}

impl SigmaBluetooth {
    /// Create a new SigmaBluetooth instance
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            adapter_state: AdapterState::PoweredOff,
            running: false,
            discovering: false,
            device_counter: 0,
        }
    }

    /// Start the Bluetooth daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the Bluetooth daemon
    pub fn stop(&mut self) {
        self.running = false;
        self.discovering = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Power on the adapter
    pub fn power_on(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaBluetooth not running".to_string());
        }

        self.adapter_state = AdapterState::PoweredOn;
        Ok(())
    }

    /// Power off the adapter
    pub fn power_off(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaBluetooth not running".to_string());
        }

        self.adapter_state = AdapterState::PoweredOff;
        self.discovering = false;
        Ok(())
    }

    /// Start discovery
    pub fn start_discovery(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaBluetooth not running".to_string());
        }

        if self.adapter_state != AdapterState::PoweredOn {
            return Err("Adapter not powered on".to_string());
        }

        self.adapter_state = AdapterState::Discovering;
        self.discovering = true;
        Ok(())
    }

    /// Stop discovery
    pub fn stop_discovery(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaBluetooth not running".to_string());
        }

        self.discovering = false;
        if self.adapter_state == AdapterState::Discovering {
            self.adapter_state = AdapterState::PoweredOn;
        }
        Ok(())
    }

    /// Add a device
    pub fn add_device(&mut self, device: BluetoothDevice) -> Result<(), String> {
        if self.devices.contains_key(&device.address) {
            return Err(format!("Device '{}' already exists", device.address));
        }

        self.devices.insert(device.address.clone(), device);
        self.device_counter += 1;
        Ok(())
    }

    /// Remove a device
    pub fn remove_device(&mut self, address: &str) -> Result<(), String> {
        if self.devices.remove(address).is_none() {
            return Err(format!("Device '{}' not found", address));
        }
        Ok(())
    }

    /// Get a device
    pub fn get_device(&self, address: &str) -> Option<&BluetoothDevice> {
        self.devices.get(address)
    }

    /// Pair a device
    pub fn pair_device(&mut self, address: &str) -> Result<(), String> {
        match self.devices.get_mut(address) {
            Some(device) => {
                device.status = BluetoothDeviceStatus::Paired;
                Ok(())
            }
            None => Err(format!("Device '{}' not found", address)),
        }
    }

    /// Unpair a device
    pub fn unpair_device(&mut self, address: &str) -> Result<(), String> {
        match self.devices.get_mut(address) {
            Some(device) => {
                device.status = BluetoothDeviceStatus::Available;
                Ok(())
            }
            None => Err(format!("Device '{}' not found", address)),
        }
    }

    /// Connect a device
    pub fn connect_device(&mut self, address: &str) -> Result<(), String> {
        match self.devices.get_mut(address) {
            Some(device) => {
                if device.status != BluetoothDeviceStatus::Paired {
                    return Err("Device must be paired before connecting".to_string());
                }
                device.status = BluetoothDeviceStatus::Connected;
                Ok(())
            }
            None => Err(format!("Device '{}' not found", address)),
        }
    }

    /// Disconnect a device
    pub fn disconnect_device(&mut self, address: &str) -> Result<(), String> {
        match self.devices.get_mut(address) {
            Some(device) => {
                device.status = BluetoothDeviceStatus::Paired;
                Ok(())
            }
            None => Err(format!("Device '{}' not found", address)),
        }
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().collect()
    }

    /// List paired devices
    pub fn list_paired(&self) -> Vec<&BluetoothDevice> {
        self.devices
            .values()
            .filter(|d| {
                d.status == BluetoothDeviceStatus::Paired
                    || d.status == BluetoothDeviceStatus::Connected
            })
            .collect()
    }

    /// List connected devices
    pub fn list_connected(&self) -> Vec<&BluetoothDevice> {
        self.devices
            .values()
            .filter(|d| d.status == BluetoothDeviceStatus::Connected)
            .collect()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Get adapter state
    pub fn get_adapter_state(&self) -> AdapterState {
        self.adapter_state
    }

    /// Is discovering
    pub fn is_discovering(&self) -> bool {
        self.discovering
    }

    /// Get Bluetooth statistics
    pub fn bluetooth_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_devices".to_string(), self.device_count().to_string());
        stats.insert(
            "paired_devices".to_string(),
            self.list_paired().len().to_string(),
        );
        stats.insert(
            "connected_devices".to_string(),
            self.list_connected().len().to_string(),
        );
        stats.insert("adapter_state".to_string(), self.adapter_state.to_string());
        stats.insert("discovering".to_string(), self.discovering.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaBluetooth {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaBluetooth instance (placeholder for actual global state)
static mut SIGMABLUETOOTH_INSTANCE: Option<SigmaBluetooth> = None;

/// Initialize the global SigmaBluetooth instance
pub fn init_sigmabluetooth() {
    unsafe {
        SIGMABLUETOOTH_INSTANCE = Some(SigmaBluetooth::new());
    }
}

/// Get the global SigmaBluetooth instance
pub fn get_sigmabluetooth() -> Option<&'static mut SigmaBluetooth> {
    unsafe { SIGMABLUETOOTH_INSTANCE.as_mut() }
}

/// Convenience function to pair a device
pub fn pair_bluetooth_device(address: &str) -> Result<(), String> {
    if let Some(bluetooth) = get_sigmabluetooth() {
        bluetooth.pair_device(address)
    } else {
        Err("SigmaBluetooth not initialized".to_string())
    }
}

/// Convenience function to connect a device
pub fn connect_bluetooth_device(address: &str) -> Result<(), String> {
    if let Some(bluetooth) = get_sigmabluetooth() {
        bluetooth.connect_device(address)
    } else {
        Err("SigmaBluetooth not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_device_type_display() {
        assert_eq!(BluetoothDeviceType::Phone.to_string(), "phone");
        assert_eq!(BluetoothDeviceType::Headset.to_string(), "headset");
    }

    #[test]
    fn test_bluetooth_device_status_display() {
        assert_eq!(BluetoothDeviceStatus::Connected.to_string(), "connected");
        assert_eq!(BluetoothDeviceStatus::Paired.to_string(), "paired");
    }

    #[test]
    fn test_adapter_state_display() {
        assert_eq!(AdapterState::PoweredOn.to_string(), "powered_on");
        assert_eq!(AdapterState::Discovering.to_string(), "discovering");
    }

    #[test]
    fn test_bluetooth_device() {
        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        )
        .with_status(BluetoothDeviceStatus::Paired)
        .with_rssi(-60)
        .with_battery(85);

        assert_eq!(device.address, "00:11:22:33:44:55");
        assert_eq!(device.status, BluetoothDeviceStatus::Paired);
        assert_eq!(device.rssi, Some(-60));
    }

    #[test]
    fn test_sigmabluetooth() {
        let mut bluetooth = SigmaBluetooth::new();
        assert!(!bluetooth.is_running());
        bluetooth.start();
        assert!(bluetooth.is_running());
        bluetooth.stop();
        assert!(!bluetooth.is_running());
    }

    #[test]
    fn test_sigmabluetooth_power_on() {
        let mut bluetooth = SigmaBluetooth::new();
        bluetooth.start();

        assert!(bluetooth.power_on().is_ok());
        assert_eq!(bluetooth.get_adapter_state(), AdapterState::PoweredOn);
    }

    #[test]
    fn test_sigmabluetooth_power_off() {
        let mut bluetooth = SigmaBluetooth::new();
        bluetooth.start();
        bluetooth.power_on().unwrap();

        assert!(bluetooth.power_off().is_ok());
        assert_eq!(bluetooth.get_adapter_state(), AdapterState::PoweredOff);
    }

    #[test]
    fn test_sigmabluetooth_start_discovery() {
        let mut bluetooth = SigmaBluetooth::new();
        bluetooth.start();
        bluetooth.power_on().unwrap();

        assert!(bluetooth.start_discovery().is_ok());
        assert!(bluetooth.is_discovering());
    }

    #[test]
    fn test_sigmabluetooth_add_device() {
        let mut bluetooth = SigmaBluetooth::new();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        );

        assert!(bluetooth.add_device(device).is_ok());
        assert_eq!(bluetooth.device_count(), 1);
    }

    #[test]
    fn test_sigmabluetooth_pair_device() {
        let mut bluetooth = SigmaBluetooth::new();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        );

        bluetooth.add_device(device).unwrap();
        assert!(bluetooth.pair_device("00:11:22:33:44:55").is_ok());

        let device = bluetooth.get_device("00:11:22:33:44:55").unwrap();
        assert_eq!(device.status, BluetoothDeviceStatus::Paired);
    }

    #[test]
    fn test_sigmabluetooth_connect_device() {
        let mut bluetooth = SigmaBluetooth::new();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        )
        .with_status(BluetoothDeviceStatus::Paired);

        bluetooth.add_device(device).unwrap();
        assert!(bluetooth.connect_device("00:11:22:33:44:55").is_ok());

        let device = bluetooth.get_device("00:11:22:33:44:55").unwrap();
        assert_eq!(device.status, BluetoothDeviceStatus::Connected);
    }

    #[test]
    fn test_sigmabluetooth_list_paired() {
        let mut bluetooth = SigmaBluetooth::new();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        )
        .with_status(BluetoothDeviceStatus::Paired);

        bluetooth.add_device(device).unwrap();

        let paired = bluetooth.list_paired();
        assert_eq!(paired.len(), 1);
    }

    #[test]
    fn test_sigmabluetooth_stats() {
        let mut bluetooth = SigmaBluetooth::new();
        bluetooth.start();
        bluetooth.power_on().unwrap();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        )
        .with_status(BluetoothDeviceStatus::Paired);

        bluetooth.add_device(device).unwrap();

        let stats = bluetooth.bluetooth_stats();
        assert_eq!(stats.get("total_devices"), Some(&"1".to_string()));
        assert_eq!(stats.get("paired_devices"), Some(&"1".to_string()));
    }

    #[test]
    fn test_pair_bluetooth_device_convenience() {
        init_sigmabluetooth();

        let mut bluetooth = get_sigmabluetooth().unwrap();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        );

        bluetooth.add_device(device).unwrap();

        let result = pair_bluetooth_device("00:11:22:33:44:55");
        assert!(result.is_ok());
    }

    #[test]
    fn test_connect_bluetooth_device_convenience() {
        init_sigmabluetooth();

        let mut bluetooth = get_sigmabluetooth().unwrap();

        let device = BluetoothDevice::new(
            "00:11:22:33:44:55".to_string(),
            "Test Device".to_string(),
            BluetoothDeviceType::Headset,
        )
        .with_status(BluetoothDeviceStatus::Paired);

        bluetooth.add_device(device).unwrap();

        let result = connect_bluetooth_device("00:11:22:33:44:55");
        assert!(result.is_ok());
    }
}
