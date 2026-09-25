// SigmaOS Sovereign Universal Hardware Abstraction Engine
// Inspired by Linux Kernel Device Tree / ACPI / PCIe subsystem abstractions and BSD devstat/sysctl mechanics.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusType {
    PciExpress,
    PciLegacy,
    Usb3,
    Usb2,
    SataAhci,
    NvmeExpress,
    I2c,
    Spi,
    Gpio,
    AcpiSystemBus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareDeviceKind {
    CpuCore,
    GpuAccelerator,
    NetworkController,
    StorageDrive,
    AudioCodec,
    WirelessAdapter,
    InputDevice,
    ThermalSensor,
    PowerManagementUnit,
}

#[derive(Debug, Clone)]
pub struct HardwareDeviceDescriptor {
    pub id: u32,
    pub name: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bus_type: BusType,
    pub device_kind: HardwareDeviceKind,
    pub numa_node: u8,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct CpuTopology {
    pub total_sockets: u16,
    pub total_cores: u16,
    pub total_threads: u16,
    pub numa_nodes: u8,
}

#[derive(Debug, Clone)]
pub struct AcpiPowerThermalState {
    pub cpu_temperature_celsius: f32,
    pub battery_percentage: u8,
    pub is_ac_connected: bool,
    pub active_power_profile: String,
}

pub struct HardwareTopology {
    pub cpu_info: CpuTopology,
    pub devices: BTreeMap<u32, HardwareDeviceDescriptor>,
}

pub struct SovereignUniversalHardwareAbstractionEngine {
    topology: HardwareTopology,
    power_state: AcpiPowerThermalState,
}

impl SovereignUniversalHardwareAbstractionEngine {
    pub fn new() -> Self {
        let cpu_info = CpuTopology {
            total_sockets: 1,
            total_cores: 8,
            total_threads: 16,
            numa_nodes: 1,
        };

        let mut devices = BTreeMap::new();
        devices.insert(
            1,
            HardwareDeviceDescriptor {
                id: 1,
                name: "Intel Wi-Fi 6E AX210".to_string(),
                vendor_id: 0x8086,
                device_id: 0x2725,
                bus_type: BusType::PciExpress,
                device_kind: HardwareDeviceKind::WirelessAdapter,
                numa_node: 0,
                is_enabled: true,
            },
        );
        devices.insert(
            2,
            HardwareDeviceDescriptor {
                id: 2,
                name: "Samsung NVMe SSD 980 PRO".to_string(),
                vendor_id: 0x144D,
                device_id: 0xA809,
                bus_type: BusType::NvmeExpress,
                device_kind: HardwareDeviceKind::StorageDrive,
                numa_node: 0,
                is_enabled: true,
            },
        );

        let power_state = AcpiPowerThermalState {
            cpu_temperature_celsius: 42.5,
            battery_percentage: 95,
            is_ac_connected: true,
            active_power_profile: "Balanced".to_string(),
        };

        Self {
            topology: HardwareTopology { cpu_info, devices },
            power_state,
        }
    }

    pub fn register_device(&mut self, device: HardwareDeviceDescriptor) {
        self.topology.devices.insert(device.id, device);
    }

    pub fn get_device(&self, id: u32) -> Option<&HardwareDeviceDescriptor> {
        self.topology.devices.get(&id)
    }

    pub fn list_devices_by_kind(&self, kind: HardwareDeviceKind) -> Vec<&HardwareDeviceDescriptor> {
        self.topology
            .devices
            .values()
            .filter(|dev| dev.device_kind == kind)
            .collect()
    }

    pub fn cpu_topology(&self) -> &CpuTopology {
        &self.topology.cpu_info
    }

    pub fn power_thermal_state(&self) -> &AcpiPowerThermalState {
        &self.power_state
    }

    pub fn total_registered_devices(&self) -> usize {
        self.topology.devices.len()
    }
}

#[cfg(test)]
mod hardware_tests {
    use super::*;

    #[test]
    fn test_hardware_engine_init() {
        let engine = SovereignUniversalHardwareAbstractionEngine::new();
        assert_eq!(engine.cpu_topology().total_cores, 8);
        assert_eq!(engine.cpu_topology().total_threads, 16);
        assert!(engine.total_registered_devices() >= 2);
    }

    #[test]
    fn test_register_and_lookup_device() {
        let mut engine = SovereignUniversalHardwareAbstractionEngine::new();
        let dev = HardwareDeviceDescriptor {
            id: 100,
            name: "NVIDIA GeForce RTX 4090".to_string(),
            vendor_id: 0x10DE,
            device_id: 0x2684,
            bus_type: BusType::PciExpress,
            device_kind: HardwareDeviceKind::GpuAccelerator,
            numa_node: 0,
            is_enabled: true,
        };

        engine.register_device(dev);
        let queried = engine.get_device(100);
        assert!(queried.is_some());
        assert_eq!(queried.unwrap().name, "NVIDIA GeForce RTX 4090");

        let gpus = engine.list_devices_by_kind(HardwareDeviceKind::GpuAccelerator);
        assert_eq!(gpus.len(), 1);
        assert_eq!(gpus[0].vendor_id, 0x10DE);
    }

    #[test]
    fn test_power_thermal_state() {
        let engine = SovereignUniversalHardwareAbstractionEngine::new();
        let power = engine.power_thermal_state();
        assert!(power.cpu_temperature_celsius > 0.0);
        assert!(power.battery_percentage <= 100);
    }
}
