#![no_std]
#![no_main]

/// OOP-based Device Manager for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 91
/// Implements device detection, registration, and management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceClass { Block = 0, Character = 1, Network = 2, Input = 3, Output = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceError { Success = 0, NotFound = 1, AlreadyRegistered = 2, InitFailed = 3 }

pub trait Device {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn device_class(&self) -> DeviceClass;
    fn initialize(&mut self) -> Result<(), DeviceError>;
    fn shutdown(&mut self) -> Result<(), DeviceError>;
}

#[repr(C)]
pub struct SimpleDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub device_class: AtomicUsize,
}

impl SimpleDevice {
    pub fn new(id: DeviceID, name: &[u8], device_class: DeviceClass) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleDevice {
            id,
            name: name_array,
            device_class: AtomicUsize::new(device_class as usize),
        }
    }
}

impl Device for SimpleDevice {
    fn id(&self) -> DeviceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn device_class(&self) -> DeviceClass { unsafe { core::mem::transmute(self.device_class.load(Ordering::SeqCst)) } }

    fn initialize(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
}

pub trait DeviceManager {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, DeviceError>;
    fn unregister_device(&mut self, id: DeviceID) -> Result<(), DeviceError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    fn list_devices(&self, device_class: DeviceClass) -> Vec<DeviceID>;
    fn scan_devices(&mut self) -> Vec<DeviceID>;
}

#[repr(C)]
pub struct SimpleDeviceManager {
    pub devices: Vec<Option<Box<dyn Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDeviceManager {
    pub fn new() -> Self {
        SimpleDeviceManager {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DeviceManager for SimpleDeviceManager {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, DeviceError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }

    fn unregister_device(&mut self, id: DeviceID) -> Result<(), DeviceError> {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(DeviceError::NotFound)
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }

    fn list_devices(&self, device_class: DeviceClass) -> Vec<DeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.device_class() == device_class {
                    ids.push(device.id());
                }
            }
        }
        ids
    }

    fn scan_devices(&mut self) -> Vec<DeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                ids.push(device.id());
            }
        }
        ids
    }
}

pub trait DeviceDriver {
    fn device_id(&self) -> DeviceID;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError>;
    fn ioctl(&mut self, request: u32, arg: usize) -> Result<(), DeviceError>;
}

#[repr(C)]
pub struct SimpleDeviceDriver {
    pub device_id: DeviceID,
}

impl SimpleDeviceDriver {
    pub fn new(device_id: DeviceID) -> Self {
        SimpleDeviceDriver { device_id }
    }
}

impl DeviceDriver for SimpleDeviceDriver {
    fn device_id(&self) -> DeviceID { self.device_id }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        for i in 0..buffer.len() {
            buffer[i] = 0u8;
        }
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError> {
        Ok(data.len())
    }

    fn ioctl(&mut self, _request: u32, _arg: usize) -> Result<(), DeviceError> {
        Ok(())
    }
}

pub trait DeviceHotplug {
    fn on_device_added(&mut self, device_id: DeviceID);
    fn on_device_removed(&mut self, device_id: DeviceID);
    fn enable_hotplug(&mut self, enabled: bool);
}

#[repr(C)]
pub struct SimpleDeviceHotplug {
    pub enabled: AtomicUsize,
    pub added_devices: Vec<DeviceID>,
    pub removed_devices: Vec<DeviceID>,
}

impl SimpleDeviceHotplug {
    pub fn new() -> Self {
        SimpleDeviceHotplug {
            enabled: AtomicUsize::new(1),
            added_devices: Vec::new(),
            removed_devices: Vec::new(),
        }
    }
}

impl DeviceHotplug for SimpleDeviceHotplug {
    fn on_device_added(&mut self, device_id: DeviceID) {
        if self.enabled.load(Ordering::SeqCst) == 1 {
            self.added_devices.push(device_id);
        }
    }

    fn on_device_removed(&mut self, device_id: DeviceID) {
        if self.enabled.load(Ordering::SeqCst) == 1 {
            self.removed_devices.push(device_id);
        }
    }

    fn enable_hotplug(&mut self, enabled: bool) {
        self.enabled.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
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
