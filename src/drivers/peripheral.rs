// Minimal peripheral device management for SigmaOS
// This provides basic device generation and power state structures

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration {
    Legacy,
    Modern,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Off,
    On,
    Sleep,
    Standby,
}

// Trait for peripheral devices
pub trait PeripheralDevice {
    fn device_id(&self) -> u32;
    fn generation(&self) -> DeviceGeneration;
    fn power_state(&self) -> PowerState;
    fn set_power_state(&mut self, state: PowerState);
}

#[derive(Debug, Clone)]
pub struct PeripheralDeviceInfo {
    pub generation: DeviceGeneration,
    pub power_state: PowerState,
    pub device_id: u32,
}

impl PeripheralDeviceInfo {
    pub fn new(device_id: u32, generation: DeviceGeneration) -> Self {
        PeripheralDeviceInfo {
            generation,
            power_state: PowerState::Off,
            device_id,
        }
    }
    
    pub fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;
    }
}

#[derive(Debug, Clone)]
pub struct PeripheralManager {
    pub devices: Vec<PeripheralDeviceInfo>,
}

impl PeripheralManager {
    pub fn new() -> Self {
        PeripheralManager {
            devices: Vec::new(),
        }
    }
    
    pub fn add_device(&mut self, device: PeripheralDeviceInfo) {
        self.devices.push(device);
    }
    
    pub fn get_device(&self, device_id: u32) -> Option<&PeripheralDeviceInfo> {
        self.devices.iter().find(|d| d.device_id == device_id)
    }
}