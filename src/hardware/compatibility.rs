// OOP-based Hardware Compatibility Matrix for SigmaOS
// Implements supported legacy, ancient (1980s/1990s), and modern hardware devices compatibility matrix.
#![no_std]
#![no_main]
// OOP-based Hardware Compatibility Matrix for SigmaOS
// Implements supported GPUs, Wi-Fi, printers, and chipsets matrix

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
/// OOP-based Hardware Compatibility Matrix for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 2
/// Implements supported GPUs, Wi-Fi, printers, and chipsets matrix

use core::mem;

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
        name: &[u8],
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

pub trait HardwareCompatibilityManager {
    fn add_device(&mut self, device: Box<dyn HardwareDevice>) -> Result<DeviceID, CompatibilityError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice>;
    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID>;
    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID>;
    fn list_supported(&self) -> Vec<DeviceID>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityError {
    Success = 0,
    DeviceNotFound = 1,
    DuplicateDevice = 2,
    InvalidParameter = 3,
}

pub struct SimpleCompatibilityMatrix {
    pub devices: Vec<Box<dyn HardwareDevice>>,
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
        self.devices.push(Box::new(gpu1));

        let wifi1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::WiFi,
            0x8086,
            0x2723,
            b"Intel Wi-Fi 6 AX200",
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

<<<<<<< HEAD
        let printer1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::Printer, 0x03F0, 0x4A17, "HP LaserJet Pro M404n", SupportStatus::Supported);
        self.devices.push(Box::new(printer1));

        let chipset1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::Chipset, 0x8086, 0x1C02, "Intel Z590", SupportStatus::Supported);
        self.devices.push(Box::new(chipset1));
=======
        let isa_com = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::LegacyBus,
            0x0003,
            0x03F8,
            "Serial Port COM1 (UART 16550)",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(isa_com));

        let nvme1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Storage,
            0x144D,
            0xA808,
            "Samsung PCIe Gen 4 NVMe Controller",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(nvme1));

        let audio1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::Audio,
            0x10EC,
            0x0887,
            "Realtek ALC887",
            SupportStatus::Supported,
        );
        self.devices.push(Box::new(audio1));
>>>>>>> origin/feat/activity-manager-paging-segmentation-613287197188639572
    }
}

impl HardwareCompatibilityManager for SimpleCompatibilityMatrix {
    fn add_device(&mut self, device: Box<dyn HardwareDevice>) -> Result<DeviceID, CompatibilityError> {
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
        self.devices.iter().find(|d| d.id() == id).map(|d| d.as_ref())
    }

    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID> {
        self.devices.iter()
            .find(|d| d.vendor_id() == vendor_id && d.device_id() == device_id)
            .map(|d| d.id())
    }

    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID> {
        self.devices.iter()
            .filter(|d| d.device_type() == device_type)
            .map(|d| d.id())
            .collect()
    }

    fn list_supported(&self) -> Vec<DeviceID> {
        self.devices.iter()
            .filter(|d| d.support_status() == SupportStatus::Supported)
            .map(|d| d.id())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityResult { Healthy = 0, Warning = 1, Error = 2, Unknown = 3 }

pub struct CompatibilityReport {
    pub results: Vec<(DeviceID, CompatibilityResult)>,
}

#[repr(C)]
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

<<<<<<< HEAD
pub struct SimpleHardwareDiagnostics {
impl SimpleDriverManager {
    pub fn new() -> Self {
        SimpleDriverManager {
            loaded_drivers: Vec::new(),
        }
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
=======
pub trait CompatibilityCheck {
    fn check_device(&self, device_id: DeviceID) -> CompatibilityResult;
    fn run_full_scan(&self) -> CompatibilityReport;
>>>>>>> origin/feat/activity-manager-paging-segmentation-613287197188639572
}

pub trait HardwareDiagnostics {
    fn check_device(&self, device_id: DeviceID) -> DiagnosticResult;
    fn run_full_scan(&self) -> Vec<(DeviceID, DiagnosticResult)>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticResult { Healthy = 0, Warning = 1, Error = 2, Unknown = 3 }

#[repr(C)]
pub struct SimpleHardwareDiagnostics {
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
            let result = self.check_device(device.id());
            results.push((device.id(), result));
        }
        CompatibilityReport { results }
    }
}

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

        let nvme_id = matrix.find_by_vendor_device(0x144D, 0xA808).unwrap();
        let nvme_dev = matrix.get_device(nvme_id).unwrap();
        assert_eq!(nvme_dev.device_type(), DeviceType::Storage);
    }

    #[test]
    fn test_compatibility_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
<<<<<<< HEAD
        assert_eq!(matrix.list_supported().len(), 5);
=======
        assert_eq!(matrix.list_supported().len(), 7);
>>>>>>> origin/feat/activity-manager-paging-segmentation-613287197188639572
        assert_eq!(matrix.list_by_type(DeviceType::WiFi).len(), 2);
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

    #[test]
    fn test_diagnostics() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        let diag = SimpleDiagnostics::new(matrix);
        let report = diag.run_full_scan();
        assert_eq!(report.results.len(), 6);
    }
}
