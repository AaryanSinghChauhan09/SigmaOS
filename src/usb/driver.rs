#![no_std]
#![no_main]

/// OOP-based USB Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 101
/// Implements USB device detection and management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type USBDeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum USBDeviceType { HID = 0, MassStorage = 1, Network = 2, Audio = 3, Unknown = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum USBError { Success = 0, NotFound = 1, InitFailed = 2, TransferFailed = 3 }

pub trait USBDevice {
    fn id(&self) -> USBDeviceID;
    fn vendor_id(&self) -> u16;
    fn product_id(&self) -> u16;
    fn device_type(&self) -> USBDeviceType;
    fn initialize(&mut self) -> Result<(), USBError>;
}

#[repr(C)]
pub struct SimpleUSBDevice {
    pub id: USBDeviceID,
    pub vendor_id: AtomicUsize,
    pub product_id: AtomicUsize,
    pub device_type: AtomicUsize,
}

impl SimpleUSBDevice {
    pub fn new(id: USBDeviceID, vendor_id: u16, product_id: u16, device_type: USBDeviceType) -> Self {
        SimpleUSBDevice {
            id,
            vendor_id: AtomicUsize::new(vendor_id as usize),
            product_id: AtomicUsize::new(product_id as usize),
            device_type: AtomicUsize::new(device_type as usize),
        }
    }
}

impl USBDevice for SimpleUSBDevice {
    fn id(&self) -> USBDeviceID { self.id }
    fn vendor_id(&self) -> u16 { self.vendor_id.load(Ordering::SeqCst) as u16 }
    fn product_id(&self) -> u16 { self.product_id.load(Ordering::SeqCst) as u16 }
    fn device_type(&self) -> USBDeviceType { unsafe { core::mem::transmute(self.device_type.load(Ordering::SeqCst)) } }

    fn initialize(&mut self) -> Result<(), USBError> {
        Ok(())
    }
}

pub trait USBController {
    fn scan_devices(&mut self) -> Vec<USBDeviceID>;
    fn register_device(&mut self, device: Box<dyn USBDevice>) -> Result<USBDeviceID, USBError>;
    fn unregister_device(&mut self, id: USBDeviceID) -> Result<(), USBError>;
    fn get_device(&self, id: USBDeviceID) -> Option<&dyn USBDevice>;
}

#[repr(C)]
pub struct SimpleUSBController {
    pub devices: Vec<Option<Box<dyn USBDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleUSBController {
    pub fn new() -> Self {
        SimpleUSBController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl USBController for SimpleUSBController {
    fn scan_devices(&mut self) -> Vec<USBDeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                ids.push(device.id());
            }
        }
        ids
    }

    fn register_device(&mut self, device: Box<dyn USBDevice>) -> Result<USBDeviceID, USBError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }

    fn unregister_device(&mut self, id: USBDeviceID) -> Result<(), USBError> {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(USBError::NotFound)
    }

    fn get_device(&self, id: USBDeviceID) -> Option<&dyn USBDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait USBTransfer {
    fn bulk_transfer(&mut self, device_id: USBDeviceID, endpoint: u8, data: &mut [u8], direction: bool) -> Result<usize, USBError>;
    fn control_transfer(&mut self, device_id: USBDeviceID, request_type: u8, request: u8, value: u16, index: u16, data: &mut [u8]) -> Result<(), USBError>;
}

#[repr(C)]
pub struct SimpleUSBTransfer {
    pub controller: SimpleUSBController,
}

impl SimpleUSBTransfer {
    pub fn new(controller: SimpleUSBController) -> Self {
        SimpleUSBTransfer { controller }
    }
}

impl USBTransfer for SimpleUSBTransfer {
    fn bulk_transfer(&mut self, device_id: USBDeviceID, _endpoint: u8, _data: &mut [u8], _direction: bool) -> Result<usize, USBError> {
        if self.controller.get_device(device_id).is_some() {
            Ok(512)
        } else {
            Err(USBError::NotFound)
        }
    }

    fn control_transfer(&mut self, device_id: USBDeviceID, _request_type: u8, _request: u8, _value: u16, _index: u16, _data: &mut [u8]) -> Result<(), USBError> {
        if self.controller.get_device(device_id).is_some() {
            Ok(())
        } else {
            Err(USBError::NotFound)
        }
    }
}

pub trait USBHub {
    fn add_port(&mut self, port_num: u8);
    fn remove_port(&mut self, port_num: u8);
    fn get_connected_devices(&self, port_num: u8) -> Vec<USBDeviceID>;
}

#[repr(C)]
pub struct SimpleUSBHub {
    pub ports: Vec<(u8, Vec<USBDeviceID>)>,
}

impl SimpleUSBHub {
    pub fn new() -> Self {
        SimpleUSBHub {
            ports: Vec::new(),
        }
    }
}

impl USBHub for SimpleUSBHub {
    fn add_port(&mut self, port_num: u8) {
        self.ports.push((port_num, Vec::new()));
    }

    fn remove_port(&mut self, port_num: u8) {
        for i in 0..self.ports.len() {
            if self.ports[i].0 == port_num {
                self.ports.remove(i);
                return;
            }
        }
    }

    fn get_connected_devices(&self, port_num: u8) -> Vec<USBDeviceID> {
        for &(port, ref devices) in &self.ports {
            if port == port_num {
                return devices.clone();
            }
        }
        Vec::new()
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
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
