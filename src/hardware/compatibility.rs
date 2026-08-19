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
use std::sync::atomic::{AtomicUsize, Ordering};

pub type DeviceID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    GPU = 0,
    WiFi = 1,
    Printer = 2,
    Chipset = 3,
    Audio = 4,
    Storage = 5,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceType { GPU = 0, WiFi = 1, Printer = 2, Chipset = 3, Audio = 4, Storage = 5 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType { GPU = 0, WiFi = 1, Printer = 2, Chipset = 3, Audio = 4, Storage = 5 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    Supported = 0,
    Partial = 1,
    Unsupported = 2,
    Unknown = 3,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SupportStatus { Supported = 0, Partial = 1, Unsupported = 2, Unknown = 3 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus { Supported = 0, Partial = 1, Unsupported = 2, Unknown = 3 }

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
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

    pub fn new(id: DeviceID, device_type: DeviceType, vendor_id: u16, device_id: u16, name: &[u8], status: SupportStatus) -> Self {
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
    pub fn new(id: DeviceID, device_type: DeviceType, vendor_id: u16, device_id: u16, name: &str, status: SupportStatus) -> Self {
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
impl Device for SimpleDevice {
    fn id(&self) -> DeviceID { self.id }
    fn device_type(&self) -> DeviceType { unsafe { core::mem::transmute(self.device_type.load(Ordering::SeqCst)) } }
    fn vendor_id(&self) -> u16 { self.vendor_id.load(Ordering::SeqCst) as u16 }
    fn device_id(&self) -> u16 { self.device_id.load(Ordering::SeqCst) as u16 }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(128);
        &self.name[..len]
    }
    fn support_status(&self) -> SupportStatus { unsafe { core::mem::transmute(self.support_status.load(Ordering::SeqCst)) } }
impl HardwareDevice for SimpleDevice {
    fn id(&self) -> DeviceID { self.id }
    fn device_type(&self) -> DeviceType { self.device_type }
    fn vendor_id(&self) -> u16 { self.vendor_id }
    fn device_id(&self) -> u16 { self.device_id }
    fn name(&self) -> &str { &self.name }
    fn support_status(&self) -> SupportStatus { self.support_status }
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
    fn trigger_hotplug(&mut self, event: HotplugEvent, device: Box<dyn HardwareDevice>) -> Result<(), &'static str>;
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
        let gpu1 = SimpleDevice::new(
            self.next_id.fetch_add(1, Ordering::SeqCst),
            DeviceType::GPU,
            0x10DE,
            0x1C02,
            "NVIDIA GeForce RTX 3060",
            SupportStatus::Supported,
        );
        self.devices.push(Some(Box::new(gpu1)));
        let gpu1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::GPU, 0x10DE, 0x1C02, b"NVIDIA GeForce RTX 3060", SupportStatus::Supported);
        self.devices.push(Some(Box::new(gpu1)));
        let gpu1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::GPU, 0x10DE, 0x1C02, "NVIDIA GeForce RTX 3060", SupportStatus::Supported);
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
        self.devices.push(Some(Box::new(wifi1)));
    }
}
        let gpu2 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::GPU, 0x1002, 0x73DF, b"AMD Radeon RX 6800 XT", SupportStatus::Supported);
        self.devices.push(Some(Box::new(gpu2)));

        let wifi1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::WiFi, 0x8086, 0x2723, b"Intel Wi-Fi 6 AX200", SupportStatus::Supported);
        self.devices.push(Some(Box::new(wifi1)));
        let gpu2 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::GPU, 0x1002, 0x73DF, "AMD Radeon RX 6800 XT", SupportStatus::Supported);
        self.devices.push(Box::new(gpu2));

        let wifi1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::WiFi, 0x8086, 0x2723, "Intel Wi-Fi 6 AX200", SupportStatus::Supported);
        self.devices.push(Box::new(wifi1));

impl Default for SimpleCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
        let wifi2 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::WiFi, 0x168C, 0x003A, b"Realtek RTL8852AE", SupportStatus::Partial);
        self.devices.push(Some(Box::new(wifi2)));

        let printer1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::Printer, 0x03F0, 0x4A17, b"HP LaserJet Pro M404n", SupportStatus::Supported);
        self.devices.push(Some(Box::new(printer1)));

        let chipset1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::Chipset, 0x8086, 0x1C02, b"Intel Z590", SupportStatus::Supported);
        self.devices.push(Some(Box::new(chipset1)));
        let wifi2 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::WiFi, 0x168C, 0x003A, "Realtek RTL8852AE", SupportStatus::Partial);
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

        // Broader driver coverage (inspired by Linux & BSD)
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
    }
}

impl HotplugManager for SimpleCompatibilityMatrix {
    fn trigger_hotplug(&mut self, event: HotplugEvent, device: Box<dyn HardwareDevice>) -> Result<(), &'static str> {
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

    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Some(device.as_ref());
                }
            }
        }
        None
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
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
pub enum CompatibilityResult { Healthy = 0, Warning = 1, Error = 2, Unknown = 3 }

pub struct CompatibilityReport {
    pub results: Vec<(DeviceID, CompatibilityResult)>,
}

pub struct SimpleDriverManager {
    pub loaded_drivers: Vec<DeviceID>,
#[repr(C)]
pub struct SimpleDriverManager {
    pub loaded_drivers: Vec<DeviceID>,
pub trait CompatibilityCheck {
    fn check_device(&self, device_id: DeviceID) -> CompatibilityResult;
    fn run_full_scan(&self) -> CompatibilityReport;
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
impl SimpleDriverManager {
    pub fn new() -> Self {
        SimpleDriverManager {
            loaded_drivers: Vec::new(),
        }
    }
}

pub trait CompatibilityCheck {
    fn check_device(&self, device_id: DeviceID) -> CompatibilityResult;
    fn run_full_scan(&self) -> CompatibilityReport;
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

/// ACPI power and interrupt load balancing strategy (inspired by Linux and BSD)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiPowerState {
    D0 = 0, // Fully functional
    D1 = 1,
    D2 = 2,
    D3 = 3, // Power off
}

pub trait AcpiLoadBalancer {
    fn balance_irq_routing(&mut self, interrupt_line: u8, cpu_id: usize) -> Result<(), &'static str>;
    fn set_device_power_state(&mut self, device_id: DeviceID, state: AcpiPowerState) -> Result<(), &'static str>;
    fn get_device_power_state(&self, device_id: DeviceID) -> Option<AcpiPowerState>;
}

pub struct SimpleAcpiManager {
    pub irq_routing: std::collections::HashMap<u8, usize>, // maps IRQ to CPU ID
    pub device_states: std::collections::HashMap<DeviceID, AcpiPowerState>,
}

impl SimpleAcpiManager {
    pub fn new() -> Self {
        SimpleAcpiManager {
            irq_routing: std::collections::HashMap::new(),
            device_states: std::collections::HashMap::new(),
        }
    }
}

impl Default for SimpleAcpiManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AcpiLoadBalancer for SimpleAcpiManager {
    fn balance_irq_routing(&mut self, interrupt_line: u8, cpu_id: usize) -> Result<(), &'static str> {
        self.irq_routing.insert(interrupt_line, cpu_id);
        Ok(())
    }

    fn set_device_power_state(&mut self, device_id: DeviceID, state: AcpiPowerState) -> Result<(), &'static str> {
        self.device_states.insert(device_id, state);
        Ok(())
    }

    fn get_device_power_state(&self, device_id: DeviceID) -> Option<AcpiPowerState> {
        self.device_states.get(&device_id).copied()
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
impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    #[test]
    fn test_compatibility_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        // Includes 6 standard devices + 2 new ones (Realtek ALC887, Intel SATA Controller)
        assert_eq!(matrix.list_supported().len(), 7);
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
    fn contains(&self, item: &T) -> bool where T: PartialEq {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item { return true; }
            }
        }
        false
    }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }

    #[test]
    fn test_diagnostics() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        matrix.seed_with_defaults();
        let diag = SimpleDiagnostics::new(matrix);
        let report = diag.run_full_scan();
        assert_eq!(report.results.len(), 8);
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
        let cap = SimpleDevice::new(99, DeviceType::Storage, 0x1234, 0x5678, "HotplugDisk", SupportStatus::Supported);
        assert!(matrix.trigger_hotplug(HotplugEvent::Add, Box::new(cap)).is_ok());
        assert_eq!(matrix.list_hotplug_history().len(), 1);
        assert_eq!(matrix.get_device(99).unwrap().name(), "HotplugDisk");
    }
}
