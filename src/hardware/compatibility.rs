// SigmaOS Hardware Compatibility Matrix & Driver Interoperability Layer
// Implements driver registration, hardware detection, power management, and hardware query abstractions.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DeviceID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Storage = 0,
    Network = 1,
    Display = 2,
    Input = 3,
    Audio = 4,
    Bus = 5,
    Other = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    D0 = 0, // Fully operational
    D1 = 1, // Light sleep
    D2 = 2, // Deep sleep
    D3 = 3, // Powered off
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareError {
    Success = 0,
    DeviceNotFound = 1,
    InitializationFailed = 2,
    UnsupportedOperation = 3,
    PowerStateTransitionFailed = 4,
}

pub trait Device {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn name(&self) -> &str;
}

pub trait HardwareDevice: Device {
    fn initialize(&mut self) -> Result<(), HardwareError>;
    fn reset(&mut self) -> Result<(), HardwareError>;
    fn set_power_state(&mut self, state: PowerState) -> Result<(), HardwareError>;
    fn power_state(&self) -> PowerState;
}

pub struct SimpleDevice {
    pub id: DeviceID,
    pub dev_type: DeviceType,
    pub name: String,
    pub power: PowerState,
}

impl SimpleDevice {
    pub fn new(id: DeviceID, dev_type: DeviceType, name: String) -> Self {
        SimpleDevice {
            id,
            dev_type,
            name,
            power: PowerState::D0,
        }
    }
}

impl Device for SimpleDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    fn device_type(&self) -> DeviceType {
        self.dev_type
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl HardwareDevice for SimpleDevice {
    fn initialize(&mut self) -> Result<(), HardwareError> {
        self.power = PowerState::D0;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), HardwareError> {
        self.power = PowerState::D0;
        Ok(())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), HardwareError> {
        self.power = state;
        Ok(())
    }

    fn power_state(&self) -> PowerState {
        self.power
    }
}

pub trait HardwareCompatibilityManager {
    fn register_device(&mut self, device: Box<dyn HardwareDevice>) -> Result<DeviceID, HardwareError>;
    fn unregister_device(&mut self, id: DeviceID) -> Result<(), HardwareError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
}

pub struct SimpleCompatibilityMatrix {
    pub devices: Vec<Option<Box<dyn HardwareDevice>>>,
    pub next_id: AtomicUsize,
}

impl Default for SimpleCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleCompatibilityMatrix {
    pub fn new() -> Self {
        SimpleCompatibilityMatrix {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HardwareCompatibilityManager for SimpleCompatibilityMatrix {
    fn register_device(&mut self, mut device: Box<dyn HardwareDevice>) -> Result<DeviceID, HardwareError> {
        let _ = device.initialize();
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }

    fn unregister_device(&mut self, id: DeviceID) -> Result<(), HardwareError> {
        if let Some(pos) = self.devices.iter().position(|d| match d {
            Some(dev) => dev.id() == id,
            None => false,
        }) {
            self.devices.remove(pos);
            Ok(())
        } else {
            Err(HardwareError::DeviceNotFound)
        }
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for dev_opt in &self.devices {
            if let Some(ref dev) = *dev_opt {
                if dev.id() == id {
                    return Some(dev.as_ref());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_lifecycle() {
        let mut dev = SimpleDevice::new(1, DeviceType::Storage, "nvme0".to_string());
        assert_eq!(dev.power_state(), PowerState::D0);
        assert!(dev.set_power_state(PowerState::D3).is_ok());
        assert_eq!(dev.power_state(), PowerState::D3);
    }

    #[test]
    fn test_compatibility_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        let dev = Box::new(SimpleDevice::new(100, DeviceType::Network, "eth0".to_string()));
        let id = matrix.register_device(dev).unwrap();
        assert_eq!(id, 100);

        let registered = matrix.get_device(100).unwrap();
        assert_eq!(registered.name(), "eth0");

        assert!(matrix.unregister_device(100).is_ok());
        assert!(matrix.get_device(100).is_none());
    }
}
