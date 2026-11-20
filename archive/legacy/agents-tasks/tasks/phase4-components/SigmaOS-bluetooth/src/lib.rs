//! SigmaOS Bluetooth — adapter detection, device pairing, connection management,
//! and audio profile routing. BlueZ D-Bus inspired API.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Device state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Discovered,
    Pairing,
    Paired,
    Connected,
    Disconnected,
    Failed,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

// ---------------------------------------------------------------------------
// Device class / profile
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    AudioSink,        // headphones, speakers
    AudioSource,      // microphone
    Keyboard,
    Mouse,
    Controller,       // game controller
    Phone,
    Unknown,
}

// ---------------------------------------------------------------------------
// Bluetooth device
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub address: String,    // "XX:XX:XX:XX:XX:XX"
    pub name: String,
    pub class: DeviceClass,
    pub state: DeviceState,
    pub rssi: i16,          // signal strength dBm
    pub trusted: bool,
}

impl BluetoothDevice {
    pub fn new(address: impl Into<String>, name: impl Into<String>, class: DeviceClass) -> Self {
        Self {
            address: address.into(),
            name: name.into(),
            class,
            state: DeviceState::Discovered,
            rssi: -70,
            trusted: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Adapter
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterState {
    PoweredOff,
    PoweredOn,
    Discovering,
}

// ---------------------------------------------------------------------------
// Bluetooth manager
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum BluetoothError {
    AdapterOff,
    DeviceNotFound(String),
    PairingFailed(String),
    AlreadyConnected(String),
    NotConnected(String),
}

pub struct BluetoothManager {
    pub adapter_state: AdapterState,
    pub adapter_name: String,
    devices: BTreeMap<String, BluetoothDevice>,
    scan_results: Vec<String>,
}

impl BluetoothManager {
    pub fn new(adapter: impl Into<String>) -> Self {
        Self {
            adapter_state: AdapterState::PoweredOff,
            adapter_name: adapter.into(),
            devices: BTreeMap::new(),
            scan_results: Vec::new(),
        }
    }

    pub fn power_on(&mut self) {
        self.adapter_state = AdapterState::PoweredOn;
    }

    pub fn power_off(&mut self) {
        self.adapter_state = AdapterState::PoweredOff;
        // Disconnect all devices
        for dev in self.devices.values_mut() {
            if dev.state == DeviceState::Connected {
                dev.state = DeviceState::Disconnected;
            }
        }
    }

    pub fn start_scan(&mut self) -> Result<(), BluetoothError> {
        if self.adapter_state == AdapterState::PoweredOff {
            return Err(BluetoothError::AdapterOff);
        }
        self.adapter_state = AdapterState::Discovering;
        Ok(())
    }

    pub fn stop_scan(&mut self) {
        if self.adapter_state == AdapterState::Discovering {
            self.adapter_state = AdapterState::PoweredOn;
        }
    }

    /// Add a discovered device (called when a scan result arrives).
    pub fn add_discovered(&mut self, device: BluetoothDevice) {
        self.scan_results.push(device.address.clone());
        self.devices.insert(device.address.clone(), device);
    }

    pub fn pair(&mut self, address: &str) -> Result<(), BluetoothError> {
        if self.adapter_state == AdapterState::PoweredOff {
            return Err(BluetoothError::AdapterOff);
        }
        let dev = self.devices.get_mut(address)
            .ok_or_else(|| BluetoothError::DeviceNotFound(address.to_string()))?;
        dev.state = DeviceState::Pairing;
        // Scaffold: pairing always succeeds
        dev.state = DeviceState::Paired;
        dev.trusted = true;
        Ok(())
    }

    pub fn connect(&mut self, address: &str) -> Result<(), BluetoothError> {
        let dev = self.devices.get_mut(address)
            .ok_or_else(|| BluetoothError::DeviceNotFound(address.to_string()))?;
        if dev.state == DeviceState::Connected {
            return Err(BluetoothError::AlreadyConnected(address.to_string()));
        }
        dev.state = DeviceState::Connected;
        Ok(())
    }

    pub fn disconnect(&mut self, address: &str) -> Result<(), BluetoothError> {
        let dev = self.devices.get_mut(address)
            .ok_or_else(|| BluetoothError::DeviceNotFound(address.to_string()))?;
        if dev.state != DeviceState::Connected {
            return Err(BluetoothError::NotConnected(address.to_string()));
        }
        dev.state = DeviceState::Disconnected;
        Ok(())
    }

    pub fn connected_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().filter(|d| d.state == DeviceState::Connected).collect()
    }

    pub fn audio_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().filter(|d| matches!(d.class, DeviceClass::AudioSink | DeviceClass::AudioSource)).collect()
    }

    pub fn device(&self, address: &str) -> Option<&BluetoothDevice> {
        self.devices.get(address)
    }

    pub fn scan_results(&self) -> Vec<&BluetoothDevice> {
        self.scan_results.iter()
            .filter_map(|a| self.devices.get(a.as_str()))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn headphones() -> BluetoothDevice {
        BluetoothDevice::new("AA:BB:CC:DD:EE:FF", "SigmaOS Headphones", DeviceClass::AudioSink)
    }

    fn make_manager() -> BluetoothManager {
        let mut bm = BluetoothManager::new("hci0");
        bm.power_on();
        bm.start_scan().unwrap();
        bm.add_discovered(headphones());
        bm.stop_scan();
        bm
    }

    #[test]
    fn pair_and_connect() {
        let mut bm = make_manager();
        bm.pair("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(bm.device("AA:BB:CC:DD:EE:FF").unwrap().state, DeviceState::Paired);
        bm.connect("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(bm.device("AA:BB:CC:DD:EE:FF").unwrap().state, DeviceState::Connected);
    }

    #[test]
    fn disconnect_then_reconnect() {
        let mut bm = make_manager();
        bm.pair("AA:BB:CC:DD:EE:FF").unwrap();
        bm.connect("AA:BB:CC:DD:EE:FF").unwrap();
        bm.disconnect("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(bm.device("AA:BB:CC:DD:EE:FF").unwrap().state, DeviceState::Disconnected);
        bm.connect("AA:BB:CC:DD:EE:FF").unwrap();
        assert_eq!(bm.connected_devices().len(), 1);
    }

    #[test]
    fn scan_requires_adapter_on() {
        let mut bm = BluetoothManager::new("hci0");
        assert!(bm.start_scan().is_err());
        bm.power_on();
        assert!(bm.start_scan().is_ok());
    }

    #[test]
    fn audio_device_filter() {
        let mut bm = make_manager();
        let mut mouse = BluetoothDevice::new("11:22:33:44:55:66", "BT Mouse", DeviceClass::Mouse);
        bm.add_discovered(mouse);
        assert_eq!(bm.audio_devices().len(), 1);
        assert_eq!(bm.audio_devices()[0].class, DeviceClass::AudioSink);
    }

    #[test]
    fn power_off_disconnects_all() {
        let mut bm = make_manager();
        bm.pair("AA:BB:CC:DD:EE:FF").unwrap();
        bm.connect("AA:BB:CC:DD:EE:FF").unwrap();
        bm.power_off();
        assert_eq!(bm.connected_devices().len(), 0);
    }
}
