#![no_std]
#![no_main]

/// OOP-based SPI for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1126
/// Implements SPI communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SPIMode { Mode0 = 0, Mode1 = 1, Mode2 = 2, Mode3 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SPIError { Success = 0, NotFound = 1 }

pub trait SPIDevice {
    fn id(&self) -> DeviceID;
    fn chip_select(&self) -> u8;
}

#[repr(C)]
pub struct SimpleSPIDevice {
    pub id: DeviceID,
    pub chip_select: AtomicUsize,
}

impl SimpleSPIDevice {
    pub fn new(id: DeviceID, chip_select: u8) -> Self {
        SimpleSPIDevice {
            id,
            chip_select: AtomicUsize::new(chip_select as usize),
        }
    }
}

impl SPIDevice for SimpleSPIDevice {
    fn id(&self) -> DeviceID { self.id }
    fn chip_select(&self) -> u8 { self.chip_select.load(Ordering::SeqCst) as u8 }
}

pub trait SPIBus {
    fn transfer(&self, device_id: DeviceID, tx_data: &[u8], rx_buffer: &mut [u8]) -> Result<(), SPIError>;
    def set_mode(&mut self, mode: SPIMode);
    def set_frequency(&mut self, frequency: u32);
}

#[repr(C)]
pub struct SimpleSPIBus {
    pub devices: Vec<Option<Box<dyn SPIDevice>>>,
    pub mode: AtomicUsize,
    pub frequency: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleSPIBus {
    pub fn new() -> Self {
        SimpleSPIBus {
            devices: Vec::new(),
            mode: AtomicUsize::new(SPIMode::Mode0 as usize),
            frequency: AtomicUsize::new(1000000),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SPIBus for SimpleSPIBus {
    fn transfer(&self, device_id: DeviceID, _tx_data: &[u8], rx_buffer: &mut [u8]) -> Result<(), SPIError> {
        if self.get_device(device_id).is_some() {
            for byte in rx_buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(SPIError::NotFound)
        }
    }
    
    fn set_mode(&mut self, mode: SPIMode) {
        self.mode.store(mode as usize, Ordering::SeqCst);
    }
    
    fn set_frequency(&mut self, frequency: u32) {
        self.frequency.store(frequency as usize, Ordering::SeqCst);
    }
    
    fn get_device(&self, id: DeviceID) -> Option<&dyn SPIDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait SPIDeviceManager {
    fn add_device(&mut self, chip_select: u8) -> Result<DeviceID, SPIError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), SPIError>;
}

#[repr(C)]
pub struct SimpleSPIDeviceManager {
    pub bus: SimpleSPIBus,
}

impl SimpleSPIDeviceManager {
    pub fn new(bus: SimpleSPIBus) -> Self {
        SimpleSPIDeviceManager { bus }
    }
}

impl SPIDeviceManager for SimpleSPIDeviceManager {
    fn add_device(&mut self, chip_select: u8) -> Result<DeviceID, SPIError> {
        let id = self.bus.next_id.fetch_add(1, Ordering::SeqCst);
        let device = SimpleSPIDevice::new(id, chip_select);
        self.bus.devices.push(Some(Box::new(device)));
        Ok(id)
    }
    
    fn remove_device(&mut self, id: DeviceID) -> Result<(), SPIError> {
        for device_option in &mut self.bus.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(SPIError::NotFound)
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
