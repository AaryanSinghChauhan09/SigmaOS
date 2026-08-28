extern crate alloc;
// OOP-based Hardware Compatibility Matrix for SigmaOS
// Implements supported legacy, ancient (1980s/1990s), and modern hardware devices compatibility matrix.


use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DeviceID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    GPU = 0,
    WiFi = 1,
    Printer = 2,
    Chipset = 3,
    Audio = 4,
    Storage = 5,
    LegacyBus = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    Supported = 0,
    Partial = 1,
    Unsupported = 2,
    Unknown = 3,
}

pub trait HardwareDevice {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn vendor_id(&self) -> u16;
    fn device_id(&self) -> u16;
    fn name(&self) -> &str;
    fn support_status(&self) -> SupportStatus;
}

pub struct SimpleDevice {
    pub id: DeviceID,
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub name: String,
    pub support_status: SupportStatus,
}

impl SimpleDevice {
    pub fn new(
        id: DeviceID,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        name: &str,
        status: SupportStatus,
    ) -> Self {
        SimpleDevice {
            id,
            device_type,
            vendor_id,
            device_id,
            name: name.to_string(),
            support_status: status,
        }
    }
}

impl HardwareDevice for SimpleDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    fn device_type(&self) -> DeviceType {
        self.device_type
    }
    fn vendor_id(&self) -> u16 {
        self.vendor_id
    }
    fn device_id(&self) -> u16 {
        self.device_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn support_status(&self) -> SupportStatus {
        self.support_status
    }
}

pub trait HardwareCompatibilityManager {
    fn add_device(
        &mut self,
        device: Box<dyn HardwareDevice>,
    ) -> Result<DeviceID, CompatibilityError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice>;
    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID>;
    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID>;
    fn list_supported(&self) -> Vec<DeviceID>;
}

pub trait DriverManager {
    fn load_driver(&mut self, device_id: DeviceID) -> Result<(), ()>;
    fn unload_driver(&mut self, device_id: DeviceID) -> Result<(), ()>;
    fn get_driver_status(&self, device_id: DeviceID) -> bool;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityError {
    Success = 0,
    DeviceNotFound = 1,
    DuplicateDevice = 2,
    InvalidParameter = 3,
}

/// Hotplug event type (inspired by Linux udev)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotplugEvent {
    Add = 0,
    Remove = 1,
}

pub trait HotplugManager {
    fn trigger_hotplug(
        &mut self,
        event: HotplugEvent,
        device: Box<dyn HardwareDevice>,
    ) -> Result<(), &'static str>;
    fn list_hotplug_history(&self) -> &[(HotplugEvent, DeviceID)];
}

pub struct SimpleCompatibilityMatrix {
    pub devices: Vec<Box<dyn HardwareDevice>>,
    pub next_id: AtomicUsize,
    pub hotplug_history: Vec<(HotplugEvent, DeviceID)>,
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
            hotplug_history: Vec::new(),
        }
    }

    pub fn seed_with_defaults(&mut self) {
        let com1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::LegacyBus,
            0x0003,
            0x03F8,
            "Serial Port COM1 (UART 16550)",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(com1));

        let nvme = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x144D,
            0xA808,
            "Samsung PCIe Gen 4 NVMe Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(nvme));

        let gpu1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::GPU,
            0x10DE,
            0x1C02,
            "NVIDIA GeForce RTX 3060",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(gpu1));

        let gpu2 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::GPU,
            0x1002,
            0x73DF,
            "AMD Radeon RX 6800 XT",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(gpu2));

        let wifi1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x8086,
            0x2723,
            "Intel Wi-Fi 6 AX200",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(wifi1));

        let wifi2 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x168C,
            0x003A,
            "Realtek RTL8852AE",
            SupportStatus::Partial,
        );
        self.devices.push(Box::new(wifi2));

        let printer1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Printer,
            0x03F0,
            0x4A17,
            "HP LaserJet Pro M404n",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(printer1));

        let chipset1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Chipset,
            0x8086,
            0x1C02,
            "Intel Z590",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(chipset1));

        let audio1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Audio,
            0x10EC,
            0x0887,
            "Realtek ALC887",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(audio1));

        let storage1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x8086,
            0x2822,
            "Intel SATA Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(storage1));

        let wifi3 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x8086,
            0x2725,
            "Intel Wi-Fi 6E AX210",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(wifi3));

        let usb1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Chipset,
            0x8086,
            0xA36D,
            "Intel xHCI USB 3.2 Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(usb1));

        let nvme1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x144D,
            0xA809,
            "Samsung NVMe SSD Controller 980 Pro",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(nvme1));

        let virtio_net = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x1AF4,
            0x1000,
            "VirtIO Network Adapter",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(virtio_net));

        let virtio_blk = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x1AF4,
            0x1001,
            "VirtIO Block Storage Device",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(virtio_blk));
    }
}

impl HotplugManager for SimpleCompatibilityMatrix {
    fn trigger_hotplug(
        &mut self,
        event: HotplugEvent,
        device: Box<dyn HardwareDevice>,
    ) -> Result<(), &'static str> {
        let dev_id = device.id();
        self.hotplug_history.push((event, dev_id));
        match event {
            HotplugEvent::Add => {
                let _ = self.add_device(device);
            }
            HotplugEvent::Remove => {
                let _ = self.remove_device(dev_id);
            }
        }
        Ok(())
    }

    fn list_hotplug_history(&self) -> &[(HotplugEvent, DeviceID)] {
        &self.hotplug_history
    }
}

impl HardwareCompatibilityManager for SimpleCompatibilityMatrix {
    fn add_device(
        &mut self,
        device: Box<dyn HardwareDevice>,
    ) -> Result<DeviceID, CompatibilityError> {
        let id = device.id();
        self.devices.push(device);
        Ok(id)
    }

    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError> {
        if let Some(pos) = self.devices.iter().position(|d| d.id() == id) {
            self.devices.remove(pos);
            Ok(())
        } else {
            Err(CompatibilityError::DeviceNotFound)
        }
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice> {
        self.devices
            .iter()
            .find(|d| d.id() == id)
            .map(|d| d.as_ref())
    }

    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID> {
        self.devices
            .iter()
            .find(|d| d.vendor_id() == vendor_id && d.device_id() == device_id)
            .map(|d| d.id())
    }

    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID> {
        self.devices
            .iter()
            .filter(|d| d.device_type() == device_type)
            .map(|d| d.id())
            .collect()
    }

    fn list_supported(&self) -> Vec<DeviceID> {
        self.devices
            .iter()
            .filter(|d| d.support_status() == SupportStatus::Supported)
            .map(|d| d.id())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityResult {
    Healthy = 0,
    Warning = 1,
    Error = 2,
    Unknown = 3,
}

pub struct CompatibilityReport {
    pub results: Vec<(DeviceID, CompatibilityResult)>,
}

pub struct SimpleDriverManager {
    pub loaded_drivers: Vec<DeviceID>,
}

impl SimpleDriverManager {
    pub fn new() -> Self {
        SimpleDriverManager {
            loaded_drivers: Vec::new(),
        }
    }
}

impl Default for SimpleDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverManager for SimpleDriverManager {
    fn load_driver(&mut self, device_id: DeviceID) -> Result<(), ()> {
        if self.loaded_drivers.contains(&device_id) {
            return Err(());
        }
        self.loaded_drivers.push(device_id);
        Ok(())
    }

    fn unload_driver(&mut self, device_id: DeviceID) -> Result<(), ()> {
        for i in 0..self.loaded_drivers.len() {
            if self.loaded_drivers[i] == device_id {
                self.loaded_drivers.remove(i);
                return Ok(());
            }
        }
        Err(())
    }

    fn get_driver_status(&self, device_id: DeviceID) -> bool {
        self.loaded_drivers.contains(&device_id)
    }
}

pub trait CompatibilityCheck {
    fn check_device(&self, device_id: DeviceID) -> CompatibilityResult;
    fn run_full_scan(&self) -> CompatibilityReport;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiPowerState {
    D0,
    D1,
    D2,
    D3,
}

pub struct SimpleAcpiManager {
    pub irq_routing: alloc::collections::BTreeMap<u32, u32>,
    pub power_states: alloc::collections::BTreeMap<u32, AcpiPowerState>,
}

impl SimpleAcpiManager {
    pub fn new() -> Self {
        Self {
            irq_routing: alloc::collections::BTreeMap::new(),
            power_states: alloc::collections::BTreeMap::new(),
        }
    }

    pub fn balance_irq_routing(&mut self, irq: u32, cpu_core: u32) -> Result<(), &'static str> {
        self.irq_routing.insert(irq, cpu_core);
        Ok(())
    }

    pub fn set_device_power_state(&mut self, device_id: u32, state: AcpiPowerState) -> Result<(), &'static str> {
        self.power_states.insert(device_id, state);
        Ok(())
    }

    pub fn get_device_power_state(&self, device_id: u32) -> Option<AcpiPowerState> {
        self.power_states.get(&device_id).copied()
    }
}

impl Default for SimpleAcpiManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SimpleDiagnostics {
    pub matrix: SimpleCompatibilityMatrix,
}

impl SimpleDiagnostics {
    pub fn new(matrix: SimpleCompatibilityMatrix) -> Self {
        SimpleDiagnostics { matrix }
    }
}

impl CompatibilityCheck for SimpleDiagnostics {
    fn check_device(&self, device_id: DeviceID) -> CompatibilityResult {
        if let Some(device) = self.matrix.get_device(device_id) {
            match device.support_status() {
                SupportStatus::Supported => CompatibilityResult::Healthy,
                SupportStatus::Partial => CompatibilityResult::Warning,
                SupportStatus::Unsupported => CompatibilityResult::Error,
                SupportStatus::Unknown => CompatibilityResult::Unknown,
            }
        } else {
            CompatibilityResult::Unknown
        }
    }

    fn run_full_scan(&self) -> CompatibilityReport {
        let mut results = Vec::new();
        for device in &self.matrix.devices {
            let result = self.check_device((**device).id());
            results.push(((**device).id(), result));
        }
        CompatibilityReport { results }
    }
}

pub type AcpiLoadBalancer = SimpleAcpiManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_generation_hardware_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();

        let com1_id = matrix.find_by_vendor_device(0x0003, 0x03F8).unwrap();
        let com1_dev = matrix.get_device(com1_id).unwrap();
        assert_eq!(com1_dev.device_type(), DeviceType::LegacyBus);
        assert_eq!(com1_dev.name(), "Serial Port COM1 (UART 16550)");

        let nvme_id = matrix.find_by_vendor_device(0x144D, 0xA808).unwrap();
        let nvme_dev = matrix.get_device(nvme_id).unwrap();
        assert_eq!(nvme_dev.device_type(), DeviceType::Storage);
        assert_eq!(nvme_dev.name(), "Samsung PCIe Gen 4 NVMe Controller");
    }

    #[test]
    fn test_compatibility_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        assert_eq!(matrix.list_supported().len(), 14);
        assert_eq!(matrix.list_by_type(DeviceType::WiFi).len(), 4);
    }

    #[test]
    fn test_diagnostics() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        let diag = SimpleDiagnostics::new(matrix);
        let report = diag.run_full_scan();
        assert_eq!(report.results.len(), 15);
    }

    #[test]
    fn test_expanded_device_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        assert_eq!(matrix.devices.len(), 15);
        assert_eq!(matrix.list_supported().len(), 14);

        let diag = SimpleDiagnostics::new(matrix);
        let report = diag.run_full_scan();
        assert_eq!(report.results.len(), 15);
    }

    #[test]
    fn test_device_type_filtering() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();

        assert_eq!(matrix.list_by_type(DeviceType::WiFi).len(), 4);
        assert_eq!(matrix.list_by_type(DeviceType::Storage).len(), 4);
        assert_eq!(matrix.list_by_type(DeviceType::Chipset).len(), 2);
        assert_eq!(matrix.list_by_type(DeviceType::GPU).len(), 2);
        assert_eq!(matrix.list_by_type(DeviceType::Printer).len(), 1);
        assert_eq!(matrix.list_by_type(DeviceType::Audio).len(), 1);

        let nvme_id = matrix.find_by_vendor_device(0x144D, 0xA809).unwrap();
        assert_eq!(matrix.get_device(nvme_id).unwrap().name(), "Samsung NVMe SSD Controller 980 Pro");
    }

    #[test]
    fn test_acpi_load_balancing() {
        let mut acpi = SimpleAcpiManager::new();
        assert!(acpi.balance_irq_routing(11, 4).is_ok());
        assert_eq!(acpi.irq_routing.get(&11), Some(&4));

        assert!(acpi.set_device_power_state(42, AcpiPowerState::D3).is_ok());
        assert_eq!(acpi.get_device_power_state(42), Some(AcpiPowerState::D3));
    }

    #[test]
    fn test_hotplug_manager() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        let cap = SimpleDevice::new(
            99,
            DeviceType::Storage,
            0x1234,
            0x5678,
            "HotplugDisk",
            SupportStatus::Supported,
        );
        assert!(matrix
            .trigger_hotplug(HotplugEvent::Add, Box::new(cap))
            .is_ok());
        assert_eq!(matrix.list_hotplug_history().len(), 1);
        assert_eq!(matrix.get_device(99).unwrap().name(), "HotplugDisk");
    }
}
