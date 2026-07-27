#![no_std]
#![no_main]

/// OOP-based Hardware Compatibility Matrix for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 2
/// Implements supported GPUs, Wi-Fi, printers, and chipsets matrix

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceType { GPU = 0, WiFi = 1, Printer = 2, Chipset = 3, Audio = 4, Storage = 5 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SupportStatus { Supported = 0, Partial = 1, Unsupported = 2, Unknown = 3 }

pub trait Device {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn vendor_id(&self) -> u16;
    fn device_id(&self) -> u16;
    fn name(&self) -> &[u8];
    fn support_status(&self) -> SupportStatus;
}

#[repr(C)]
pub struct SimpleDevice {
    pub id: DeviceID,
    pub device_type: AtomicUsize,
    pub vendor_id: AtomicUsize,
    pub device_id: AtomicUsize,
    pub name: [u8; 128],
    pub support_status: AtomicUsize,
}

impl SimpleDevice {
    pub fn new(id: DeviceID, device_type: DeviceType, vendor_id: u16, device_id: u16, name: &[u8], status: SupportStatus) -> Self {
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
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
    fn id(&self) -> DeviceID { self.id }
    fn device_type(&self) -> DeviceType { unsafe { core::mem::transmute(self.device_type.load(Ordering::SeqCst)) } }
    fn vendor_id(&self) -> u16 { self.vendor_id.load(Ordering::SeqCst) as u16 }
    fn device_id(&self) -> u16 { self.device_id.load(Ordering::SeqCst) as u16 }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(128);
        &self.name[..len]
    }
    fn support_status(&self) -> SupportStatus { unsafe { core::mem::transmute(self.support_status.load(Ordering::SeqCst)) } }
}

pub trait CompatibilityMatrix {
    fn add_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, ()>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), ()>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    fn find_by_vendor_device(&self, vendor_id: u16, device_id: u16) -> Option<DeviceID>;
    fn list_by_type(&self, device_type: DeviceType) -> Vec<DeviceID>;
    fn list_supported(&self) -> Vec<DeviceID>;
}

#[repr(C)]
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

    pub fn seed_with_defaults(&mut self) {
        let gpu1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::GPU, 0x10DE, 0x1C02, b"NVIDIA GeForce RTX 3060", SupportStatus::Supported);
        self.devices.push(Some(Box::new(gpu1)));

        let gpu2 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::GPU, 0x1002, 0x73DF, b"AMD Radeon RX 6800 XT", SupportStatus::Supported);
        self.devices.push(Some(Box::new(gpu2)));

        let wifi1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::WiFi, 0x8086, 0x2723, b"Intel Wi-Fi 6 AX200", SupportStatus::Supported);
        self.devices.push(Some(Box::new(wifi1)));

        let wifi2 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::WiFi, 0x168C, 0x003A, b"Realtek RTL8852AE", SupportStatus::Partial);
        self.devices.push(Some(Box::new(wifi2)));

        let printer1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::Printer, 0x03F0, 0x4A17, b"HP LaserJet Pro M404n", SupportStatus::Supported);
        self.devices.push(Some(Box::new(printer1)));

        let chipset1 = SimpleDevice::new(self.next_id.fetch_add(1, Ordering::SeqCst), DeviceType::Chipset, 0x8086, 0x1C02, b"Intel Z590", SupportStatus::Supported);
        self.devices.push(Some(Box::new(chipset1)));
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
                if device.id() == id { return Some(device.as_ref()); }
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticResult { Healthy = 0, Warning = 1, Error = 2, Unknown = 3 }

#[repr(C)]
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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
    }
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
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
