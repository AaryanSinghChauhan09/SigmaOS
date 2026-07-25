#![no_std]
#![no_main]

/// OOP-based I2C for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1116
/// Implements I2C communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum I2CError { Success = 0, NotFound = 1, NACK = 2 }

pub trait I2CDevice {
    fn id(&self) -> DeviceID;
    fn address(&self) -> u8;
}

#[repr(C)]
pub struct SimpleI2CDevice {
    pub id: DeviceID,
    pub address: AtomicUsize,
}

impl SimpleI2CDevice {
    pub fn new(id: DeviceID, address: u8) -> Self {
        SimpleI2CDevice {
            id,
            address: AtomicUsize::new(address as usize),
        }
    }
}

impl I2CDevice for SimpleI2CDevice {
    fn id(&self) -> DeviceID { self.id }
    fn address(&self) -> u8 { self.address.load(Ordering::SeqCst) as u8 }
}

pub trait I2CBus {
    fn write(&self, device_id: DeviceID, data: &[u8]) -> Result<(), I2CError>;
    fn read(&self, device_id: DeviceID, buffer: &mut [u8]) -> Result<(), I2CError>;
    fn write_read(&self, device_id: DeviceID, write_data: &[u8], read_buffer: &mut [u8]) -> Result<(), I2CError>;
}

#[repr(C)]
pub struct SimpleI2CBus {
    pub devices: Vec<Option<Box<dyn I2CDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleI2CBus {
    pub fn new() -> Self {
        SimpleI2CBus {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl I2CBus for SimpleI2CBus {
    fn write(&self, device_id: DeviceID, _data: &[u8]) -> Result<(), I2CError> {
        if self.get_device(device_id).is_some() {
            Ok(())
        } else {
            Err(I2CError::NotFound)
        }
    }
    
    fn read(&self, device_id: DeviceID, buffer: &mut [u8]) -> Result<(), I2CError> {
        if self.get_device(device_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(I2CError::NotFound)
        }
    }
    
    fn write_read(&self, device_id: DeviceID, _write_data: &[u8], read_buffer: &mut [u8]) -> Result<(), I2CError> {
        if self.get_device(device_id).is_some() {
            for byte in read_buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(I2CError::NotFound)
        }
    }
    
    fn get_device(&self, id: DeviceID) -> Option<&dyn I2CDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait I2CScanner {
    fn scan(&mut self) -> Vec<u8>;
    def register_device(&mut self, address: u8) -> Result<DeviceID, I2CError>;
}

#[repr(C)]
pub struct SimpleI2CScanner {
    pub bus: SimpleI2CBus,
}

impl SimpleI2CScanner {
    pub fn new(bus: SimpleI2CBus) -> Self {
        SimpleI2CScanner { bus }
    }
}

impl I2CScanner for SimpleI2CScanner {
    fn scan(&mut self) -> Vec<u8> {
        let mut found = Vec::new();
        for addr in 0..128 {
            found.push(addr as u8);
        }
        found
    }
    
    fn register_device(&mut self, address: u8) -> Result<DeviceID, I2CError> {
        let id = self.bus.next_id.fetch_add(1, Ordering::SeqCst);
        let device = SimpleI2CDevice::new(id, address);
        self.bus.devices.push(Some(Box::new(device)));
        Ok(id)
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
