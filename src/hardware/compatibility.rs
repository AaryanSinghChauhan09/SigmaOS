// OOP-based Hardware Compatibility Matrix for SigmaOS
// Implements supported legacy, ancient (1980s/1990s), and modern hardware devices compatibility matrix.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DeviceID = usize;

#[repr(C)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    Supported = 0,
    Partial = 1,
    Unsupported = 2,
    Unknown = 3,
}

pub trait Device {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn vendor_id(&self) -> u16;
    fn device_id(&self) -> u16;
    fn name(&self) -> &[u8];
    fn support_status(&self) -> SupportStatus;
}

pub struct SimpleDevice {
    pub id: DeviceID,
    pub device_type: AtomicUsize,
    pub vendor_id: AtomicUsize,
    pub device_id: AtomicUsize,
    pub name: [u8; 128],
    pub support_status: AtomicUsize,
}

impl SimpleDevice {
    pub fn new(
        id: DeviceID,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        name: &[u8],
        status: SupportStatus,
    ) -> Self {
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

        SimpleDevice {
            id,
            device_type: AtomicUsize::new(device_type as usize),
            vendor_id: AtomicUsize::new(vendor_id as usize),
            device_id: AtomicUsize::new(device_id as usize),
            name: name_array,
            support_status: AtomicUsize::new(status as usize),
        }
    }
}

impl Device for SimpleDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    fn device_type(&self) -> DeviceType {
        match self.device_type.load(Ordering::SeqCst) {
            0 => DeviceType::GPU,
            1 => DeviceType::WiFi,
            2 => DeviceType::Printer,
            3 => DeviceType::Chipset,
            4 => DeviceType::Audio,
            5 => DeviceType::Storage,
            _ => DeviceType::LegacyBus,
        }
    }
    fn vendor_id(&self) -> u16 {
        self.vendor_id.load(Ordering::SeqCst) as u16
    }
    fn device_id(&self) -> u16 {
        self.device_id.load(Ordering::SeqCst) as u16
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(128);
        &self.name[..len]
    }
    fn support_status(&self) -> SupportStatus {
        match self.support_status.load(Ordering::SeqCst) {
            0 => SupportStatus::Supported,
            1 => SupportStatus::Partial,
            2 => SupportStatus::Unsupported,
            _ => SupportStatus::Unknown,
        }
    }
}

pub trait CompatibilityMatrix {
    fn add_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, ()>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), ()>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID>;
    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID>;
    fn list_supported(&self) -> Vec<DeviceID>;
}

pub struct SimpleCompatibilityMatrix {
    pub devices: Vec<Option<Box<dyn Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCompatibilityMatrix {
    pub fn new() -> Self {
        SimpleCompatibilityMatrix {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    /// Seeds the matrix with a wide array of both ancient/legacy and modern system devices (Linux-inspired)
    pub fn seed_with_defaults(&mut self) {
        // --- 1. Ancient & Legacy Era Devices (1980s / 1990s) ---
        let sb16 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Audio,
            0x0001, // Simulated Legacy ISA Vendor ID
            0x0016, // Sound Blaster 16 ID
            b"Creative Labs Sound Blaster 16 (ISA)",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(sb16)));

        let floppy = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x0002, // Legacy Floppy Controller Vendor
            0x03F0, // Standard Floppy disk port
            b"Floppy Disk Controller (Intel 82077AA)",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(floppy)));

        let com1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::LegacyBus,
            0x0003, // Standard Serial Vendor
            0x03F8, // UART 16550 COM1 port address
            b"Serial Port COM1 (UART 16550)",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(com1)));

        // --- 2. Modern & High-Performance Devices (2010s / Present) ---
        let nvme = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x144D, // Samsung Vendor ID
            0xA808, // PCIe 980 Pro SSD ID
            b"Samsung PCIe Gen 4 NVMe Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(nvme)));

        let gpu1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::GPU,
            0x10DE,
            0x1C02,
            b"NVIDIA GeForce RTX 3060",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(gpu1)));

        let wifi1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x8086,
            0x2723,
            b"Intel Wi-Fi 6 AX200",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(wifi1)));
    }
}

impl Default for SimpleCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatibilityMatrix for SimpleCompatibilityMatrix {
    fn add_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, ()> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }

    fn remove_device(&mut self, id: DeviceID) -> Result<(), ()> {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(())
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Some(device.as_ref());
                }
            }
        }
        None
    }

    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.vendor_id() == vendor_id && device.device_id() == device_id {
                    return Some(device.id());
                }
            }
        }
        None
    }

    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.device_type() == device_type {
                    ids.push(device.id());
                }
            }
        }
        ids
    }

    fn list_supported(&self) -> Vec<DeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.support_status() == SupportStatus::Supported {
                    ids.push(device.id());
                }
            }
        }
        ids
    }
}

pub trait DriverManager {
    fn load_driver(&mut self, device_id: DeviceID) -> Result<(), ()>;
    fn unload_driver(&mut self, device_id: DeviceID) -> Result<(), ()>;
    fn get_driver_status(&self, device_id: DeviceID) -> bool;
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

pub trait HardwareDiagnostics {
    fn check_device(&self, device_id: DeviceID) -> DiagnosticResult;
    fn run_full_scan(&self) -> Vec<(DeviceID, DiagnosticResult)>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticResult {
    Healthy = 0,
    Warning = 1,
    Error = 2,
    Unknown = 3,
}

pub struct SimpleHardwareDiagnostics {
    pub matrix: SimpleCompatibilityMatrix,
}

impl SimpleHardwareDiagnostics {
    pub fn new(matrix: SimpleCompatibilityMatrix) -> Self {
        SimpleHardwareDiagnostics { matrix }
    }
}

impl HardwareDiagnostics for SimpleHardwareDiagnostics {
    fn check_device(&self, device_id: DeviceID) -> DiagnosticResult {
        if let Some(device) = self.matrix.get_device(device_id) {
            match device.support_status() {
                SupportStatus::Supported => DiagnosticResult::Healthy,
                SupportStatus::Partial => DiagnosticResult::Warning,
                SupportStatus::Unsupported => DiagnosticResult::Error,
                SupportStatus::Unknown => DiagnosticResult::Unknown,
            }
        } else {
            DiagnosticResult::Unknown
        }
    }

    fn run_full_scan(&self) -> Vec<(DeviceID, DiagnosticResult)> {
        let mut results = Vec::new();
        for device_option in &self.matrix.devices {
            if let Some(ref device) = *device_option {
                let result = self.check_device(device.id());
                results.push((device.id(), result));
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_generation_hardware_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();

        // 1. Verify Ancient ISA COM1 uart serial port exists and resolves
        let com1_id = matrix.find_by_vendor_device(0x0003, 0x03F8).unwrap();
        let com1_dev = matrix.get_device(com1_id).unwrap();
        assert_eq!(com1_dev.device_type(), DeviceType::LegacyBus);
        assert_eq!(com1_dev.name(), b"Serial Port COM1 (UART 16550)");

        // 2. Verify Modern high-speed NVMe controller exists and resolves
        let nvme_id = matrix.find_by_vendor_device(0x144D, 0xA808).unwrap();
        let nvme_dev = matrix.get_device(nvme_id).unwrap();
        assert_eq!(nvme_dev.device_type(), DeviceType::Storage);
        assert_eq!(nvme_dev.name(), b"Samsung PCIe Gen 4 NVMe Controller");
    }

    #[test]
    fn test_driver_manager_lifecycle() {
        let mut driver_manager = SimpleDriverManager::new();
        assert!(!driver_manager.get_driver_status(42));

        driver_manager.load_driver(42).unwrap();
        assert!(driver_manager.get_driver_status(42));

        driver_manager.unload_driver(42).unwrap();
        assert!(!driver_manager.get_driver_status(42));
    }
}
