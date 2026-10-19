//! SigmaBluetooth - Bluetooth Management System
//!
//! Provides comprehensive Bluetooth management inspired by Linux BlueZ and BSD Bluetooth stack.
//! Supports device discovery, pairing, connection management, and profile handling.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Bluetooth adapter state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterState {
    Off,
    On,
    Discovering,
    Unknown,
}

/// Device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Disconnected,
    Connected,
    Paired,
    Unknown,
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Unknown,
    Computer,
    Phone,
    Audio,
    Peripheral,
    Keyboard,
    Mouse,
    Joystick,
    Headset,
    Speaker,
}

/// Bluetooth adapter
#[derive(Debug, Clone)]
pub struct BluetoothAdapter {
    pub id: String,
    pub name: String,
    pub address: String,
    pub state: AdapterState,
    pub discovering: bool,
    pub powered: bool,
}

impl BluetoothAdapter {
    pub fn new(id: String, name: String, address: String) -> Self {
        Self {
            id,
            name,
            address,
            state: AdapterState::Off,
            discovering: false,
            powered: false,
        }
    }

    pub fn power_on(&mut self) {
        self.powered = true;
        self.state = AdapterState::On;
    }

    pub fn power_off(&mut self) {
        self.powered = false;
        self.state = AdapterState::Off;
        self.discovering = false;
    }

    pub fn start_discovery(&mut self) {
        if self.powered {
            self.discovering = true;
            self.state = AdapterState::Discovering;
        }
    }

    pub fn stop_discovery(&mut self) {
        self.discovering = false;
        if self.powered {
            self.state = AdapterState::On;
        }
    }
}

/// Bluetooth device
#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub id: String,
    pub name: String,
    pub address: String,
    pub device_type: DeviceType,
    pub state: DeviceState,
    pub paired: bool,
    pub trusted: bool,
    pub connected: bool,
    pub rssi: Option<i32>,
}

impl BluetoothDevice {
    pub fn new(id: String, name: String, address: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            address,
            device_type,
            state: DeviceState::Disconnected,
            paired: false,
            trusted: false,
            connected: false,
            rssi: None,
        }
    }

    pub fn pair(&mut self) {
        self.paired = true;
        self.state = DeviceState::Paired;
    }

    pub fn unpair(&mut self) {
        self.paired = false;
        self.trusted = false;
        self.state = DeviceState::Disconnected;
    }

    pub fn connect(&mut self) {
        if self.paired {
            self.connected = true;
            self.state = DeviceState::Connected;
        }
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.state = DeviceState::Paired;
    }

    pub fn trust(&mut self) {
        self.trusted = true;
    }

    pub fn untrust(&mut self) {
        self.trusted = false;
    }

    pub fn set_rssi(&mut self, rssi: i32) {
        self.rssi = Some(rssi);
    }
}

/// Bluetooth manager
pub struct SigmaBluetooth {
    adapters: BTreeMap<String, BluetoothAdapter>,
    devices: BTreeMap<String, BluetoothDevice>,
    next_adapter_id: u64,
    next_device_id: u64,
}

impl SigmaBluetooth {
    pub fn new() -> Self {
        Self {
            adapters: BTreeMap::new(),
            devices: BTreeMap::new(),
            next_adapter_id: 1,
            next_device_id: 1,
        }
    }

    pub fn add_adapter(&mut self, name: String, address: String) -> String {
        let id = format!("adapter-{}", self.next_adapter_id);
        self.next_adapter_id += 1;
        let adapter = BluetoothAdapter::new(id.clone(), name, address);
        self.adapters.insert(id.clone(), adapter);
        id
    }

    pub fn get_adapter(&self, id: &str) -> Option<&BluetoothAdapter> {
        self.adapters.get(id)
    }

    pub fn get_adapter_mut(&mut self, id: &str) -> Option<&mut BluetoothAdapter> {
        self.adapters.get_mut(id)
    }

    pub fn list_adapters(&self) -> Vec<&BluetoothAdapter> {
        self.adapters.values().collect()
    }

    pub fn remove_adapter(&mut self, id: &str) -> bool {
        self.adapters.remove(id).is_some()
    }

    pub fn add_device(&mut self, name: String, address: String, device_type: DeviceType) -> String {
        let id = format!("device-{}", self.next_device_id);
        self.next_device_id += 1;
        let device = BluetoothDevice::new(id.clone(), name, address, device_type);
        self.devices.insert(id.clone(), device);
        id
    }

    pub fn get_device(&self, id: &str) -> Option<&BluetoothDevice> {
        self.devices.get(id)
    }

    pub fn get_device_mut(&mut self, id: &str) -> Option<&mut BluetoothDevice> {
        self.devices.get_mut(id)
    }

    pub fn list_devices(&self) -> Vec<&BluetoothDevice> {
        self.devices.values().collect()
    }

    pub fn list_devices_by_type(&self, device_type: DeviceType) -> Vec<&BluetoothDevice> {
        self.devices.values().filter(|d| d.device_type == device_type).collect()
    }

    pub fn list_devices_by_state(&self, state: DeviceState) -> Vec<&BluetoothDevice> {
        self.devices.values().filter(|d| d.state == state).collect()
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.devices.remove(id).is_some()
    }

    pub fn pair_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.pair();
            true
        } else {
            false
        }
    }

    pub fn unpair_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.unpair();
            true
        } else {
            false
        }
    }

    pub fn connect_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.connect();
            true
        } else {
            false
        }
    }

    pub fn disconnect_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.disconnect();
            true
        } else {
            false
        }
    }

    pub fn trust_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.trust();
            true
        } else {
            false
        }
    }

    pub fn adapter_count(&self) -> usize {
        self.adapters.len()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn connected_device_count(&self) -> usize {
        self.devices.values().filter(|d| d.connected).count()
    }
}

impl Default for SigmaBluetooth {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMABLUETOOTH_INSTANCE: Option<SigmaBluetooth> = None;

pub fn init_sigmabluetooth() {
    unsafe {
        SIGMABLUETOOTH_INSTANCE = Some(SigmaBluetooth::new());
    }
}

pub fn get_sigmabluetooth() -> &'static SigmaBluetooth {
    unsafe {
        SIGMABLUETOOTH_INSTANCE
            .as_ref()
            .expect("SigmaBluetooth not initialized")
    }
}

pub fn get_sigmabluetooth_mut() -> &'static mut SigmaBluetooth {
    unsafe {
        SIGMABLUETOOTH_INSTANCE
            .as_mut()
            .expect("SigmaBluetooth not initialized")
    }
}

pub fn add_adapter(name: String, address: String) -> String {
    let manager = get_sigmabluetooth_mut();
    manager.add_adapter(name, address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let adapter = BluetoothAdapter::new(
            "adapter-1".to_string(),
            "hci0".to_string(),
            "00:11:22:33:44:55".to_string(),
        );
        assert_eq!(adapter.id, "adapter-1");
        assert_eq!(adapter.name, "hci0");
        assert_eq!(adapter.state, AdapterState::Off);
        assert!(!adapter.powered);
    }

    #[test]
    fn test_adapter_power_control() {
        let mut adapter = BluetoothAdapter::new(
            "adapter-1".to_string(),
            "hci0".to_string(),
            "00:11:22:33:44:55".to_string(),
        );
        assert!(!adapter.powered);

        adapter.power_on();
        assert!(adapter.powered);
        assert_eq!(adapter.state, AdapterState::On);

        adapter.power_off();
        assert!(!adapter.powered);
        assert_eq!(adapter.state, AdapterState::Off);
    }

    #[test]
    fn test_adapter_discovery() {
        let mut adapter = BluetoothAdapter::new(
            "adapter-1".to_string(),
            "hci0".to_string(),
            "00:11:22:33:44:55".to_string(),
        );
        adapter.power_on();

        adapter.start_discovery();
        assert!(adapter.discovering);
        assert_eq!(adapter.state, AdapterState::Discovering);

        adapter.stop_discovery();
        assert!(!adapter.discovering);
        assert_eq!(adapter.state, AdapterState::On);
    }

    #[test]
    fn test_device_creation() {
        let device = BluetoothDevice::new(
            "device-1".to_string(),
            "Headset".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            DeviceType::Headset,
        );
        assert_eq!(device.id, "device-1");
        assert_eq!(device.name, "Headset");
        assert_eq!(device.device_type, DeviceType::Headset);
        assert_eq!(device.state, DeviceState::Disconnected);
    }

    #[test]
    fn test_device_pairing() {
        let mut device = BluetoothDevice::new(
            "device-1".to_string(),
            "Keyboard".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            DeviceType::Keyboard,
        );
        assert!(!device.paired);

        device.pair();
        assert!(device.paired);
        assert_eq!(device.state, DeviceState::Paired);

        device.unpair();
        assert!(!device.paired);
        assert_eq!(device.state, DeviceState::Disconnected);
    }

    #[test]
    fn test_device_connection() {
        let mut device = BluetoothDevice::new(
            "device-1".to_string(),
            "Mouse".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            DeviceType::Mouse,
        );
        device.pair();

        device.connect();
        assert!(device.connected);
        assert_eq!(device.state, DeviceState::Connected);

        device.disconnect();
        assert!(!device.connected);
        assert_eq!(device.state, DeviceState::Paired);
    }

    #[test]
    fn test_device_trust() {
        let mut device = BluetoothDevice::new(
            "device-1".to_string(),
            "Headset".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            DeviceType::Headset,
        );
        assert!(!device.trusted);

        device.trust();
        assert!(device.trusted);

        device.untrust();
        assert!(!device.trusted);
    }

    #[test]
    fn test_bluetooth_manager() {
        let manager = SigmaBluetooth::new();
        assert_eq!(manager.adapter_count(), 0);
        assert_eq!(manager.device_count(), 0);
    }

    #[test]
    fn test_add_adapter() {
        let mut manager = SigmaBluetooth::new();
        let id = manager.add_adapter("hci0".to_string(), "00:11:22:33:44:55".to_string());
        assert_eq!(id, "adapter-1");
        assert_eq!(manager.adapter_count(), 1);
    }

    #[test]
    fn test_add_device() {
        let mut manager = SigmaBluetooth::new();
        let id = manager.add_device("Headset".to_string(), "AA:BB:CC:DD:EE:FF".to_string(), DeviceType::Headset);
        assert_eq!(id, "device-1");
        assert_eq!(manager.device_count(), 1);
    }

    #[test]
    fn test_pair_connect_device() {
        let mut manager = SigmaBluetooth::new();
        let id = manager.add_device("Keyboard".to_string(), "AA:BB:CC:DD:EE:FF".to_string(), DeviceType::Keyboard);

        assert!(manager.pair_device(&id));
        assert!(manager.get_device(&id).unwrap().paired);

        assert!(manager.connect_device(&id));
        assert!(manager.get_device(&id).unwrap().connected);
    }

    #[test]
    fn test_list_devices_by_type() {
        let mut manager = SigmaBluetooth::new();
        manager.add_device("Headset".to_string(), "AA:BB:CC:DD:EE:FF".to_string(), DeviceType::Headset);
        manager.add_device("Keyboard".to_string(), "BB:CC:DD:EE:FF:00".to_string(), DeviceType::Keyboard);
        manager.add_device("Speaker".to_string(), "CC:DD:EE:FF:00:11".to_string(), DeviceType::Speaker);

        let audio_devices = manager.list_devices_by_type(DeviceType::Headset);
        assert_eq!(audio_devices.len(), 1);
    }

    #[test]
    fn test_list_devices_by_state() {
        let mut manager = SigmaBluetooth::new();
        let id1 = manager.add_device("Device1".to_string(), "AA:BB:CC:DD:EE:FF".to_string(), DeviceType::Mouse);
        let id2 = manager.add_device("Device2".to_string(), "BB:CC:DD:EE:FF:00".to_string(), DeviceType::Keyboard);
        manager.pair_device(&id1);
        manager.connect_device(&id1);

        let connected = manager.list_devices_by_state(DeviceState::Connected);
        assert_eq!(connected.len(), 1);
    }

    #[test]
    fn test_connected_device_count() {
        let mut manager = SigmaBluetooth::new();
        let id1 = manager.add_device("Device1".to_string(), "AA:BB:CC:DD:EE:FF".to_string(), DeviceType::Mouse);
        let id2 = manager.add_device("Device2".to_string(), "BB:CC:DD:EE:FF:00".to_string(), DeviceType::Keyboard);
        manager.pair_device(&id1);
        manager.pair_device(&id2);
        manager.connect_device(&id1);
        manager.connect_device(&id2);

        assert_eq!(manager.connected_device_count(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmabluetooth();
        let manager = get_sigmabluetooth();
        assert_eq!(manager.adapter_count(), 0);

        let manager_mut = get_sigmabluetooth_mut();
        manager_mut.add_adapter("hci0".to_string(), "00:11:22:33:44:55".to_string());
        assert_eq!(manager.adapter_count(), 1);
    }

    #[test]
    fn test_add_adapter_convenience() {
        init_sigmabluetooth();
        let id = add_adapter("hci0".to_string(), "00:11:22:33:44:55".to_string());
        assert_eq!(id, "adapter-1");

        let manager = get_sigmabluetooth();
        assert!(manager.get_adapter(&id).is_some());
    }
}
