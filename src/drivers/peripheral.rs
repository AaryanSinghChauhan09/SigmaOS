extern crate alloc;
// SigmaOS Peripheral Device Framework
// Provides OOP-based peripheral device management

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

/// Full-featured peripheral device trait used by all drivers
pub trait PeripheralDevice {
    fn device_id(&self) -> u32 { 0 }
    fn generation(&self) -> DeviceGeneration;
    fn power_state(&self) -> PowerState { PowerState::Off }
    fn name(&self) -> &'static str { "Unknown Device" }
    fn initialize(&mut self) -> Result<(), &'static str> { Ok(()) }
    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> { Ok(0) }
    fn write(&mut self, _data: &[u8]) -> Result<usize, &'static str> { Ok(0) }
    fn set_power_state(&mut self, _state: PowerState) -> Result<(), &'static str> { Ok(()) }
    fn shutdown(&mut self) -> Result<(), &'static str> { Ok(()) }
}

/// Also implement a simpler trait alias for backward compat
pub trait PeripheralDeviceTrait: PeripheralDevice {}

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

use std::vec::Vec;

impl PeripheralManager {
    pub fn new() -> Self {
        PeripheralManager {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: PeripheralDeviceInfo) {
        self.devices.push(device);
    }

    pub fn register_device(&mut self, device: alloc::boxed::Box<dyn PeripheralDevice>) -> Result<(), &'static str> {
        let info = PeripheralDeviceInfo {
            generation: device.generation(),
            power_state: device.power_state(),
            device_id: device.device_id(),
        };
        self.devices.push(info);
        Ok(())
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn broadcast_power_state(&mut self, state: PowerState) {
        for device in &mut self.devices {
            device.set_power_state(state);
        }
    }

    pub fn get_device(&self, device_id: u32) -> Option<&PeripheralDeviceInfo> {
        self.devices.iter().find(|d| d.device_id == device_id)
    }
}

impl Default for PeripheralManager {
    fn default() -> Self {
        Self::new()
    }
}