#![no_std]
#![no_main]

/// OOP-based NRF52 BLE for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3066
/// Implements NRF52 BLE transceiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NRF52ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NRF52Error { Success = 0, NotFound = 1 }

pub trait NRF52Device {
    fn id(&self) -> NRF52ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNRF52Device {
    pub id: NRF52ID,
    pub initialized: AtomicUsize,
}

impl SimpleNRF52Device {
    pub fn new(id: NRF52ID) -> Self {
        SimpleNRF52Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl NRF52Device for SimpleNRF52Device {
    fn id(&self) -> NRF52ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait NRF52Controller {
    fn init(&mut self, nrf_id: NRF52ID) -> Result<(), NRF52Error>;
    fn advertise(&self, nrf_id: NRF52ID, data: &[u8]) -> Result<(), NRF52Error>;
    def connect(&self, nrf_id: NRF52ID, addr: &[u8]) -> Result<(), NRF52Error>;
}

#[repr(C)]
pub struct SimpleNRF52Controller {
    pub devices: Vec<Option<Box<dyn NRF52Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNRF52Controller {
    pub fn new() -> Self {
        SimpleNRF52Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NRF52Controller for SimpleNRF52Controller {
    fn init(&mut self, nrf_id: NRF52ID) -> Result<(), NRF52Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == nrf_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(NRF52Error::NotFound)
    }
    
    fn advertise(&self, nrf_id: NRF52ID, _data: &[u8]) -> Result<(), NRF52Error> {
        if self.get_device(nrf_id).is_some() {
            Ok(())
        } else {
            Err(NRF52Error::NotFound)
        }
    }
    
    fn connect(&self, nrf_id: NRF52ID, _addr: &[u8]) -> Result<(), NRF52Error> {
        if self.get_device(nrf_id).is_some() {
            Ok(())
        } else {
            Err(NRF52Error::NotFound)
        }
    }
    
    fn get_device(&self, id: NRF52ID) -> Option<&dyn NRF52Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait NRF52GATT {
    def add_service(&mut self, nrf_id: NRF52ID, uuid: &[u8]) -> Result<(), NRF52Error>;
    def add_characteristic(&mut self, nrf_id: NRF52ID, uuid: &[u8]) -> Result<(), NRF52Error>;
}

#[repr(C)]
pub struct SimpleNRF52GATT {
    pub controller: SimpleNRF52Controller,
}

impl SimpleNRF52GATT {
    pub fn new(controller: SimpleNRF52Controller) -> Self {
        SimpleNRF52GATT { controller }
    }
}

impl NRF52GATT for SimpleNRF52GATT {
    fn add_service(&mut self, _nrf_id: NRF52ID, _uuid: &[u8]) -> Result<(), NRF52Error> {
        Ok(())
    }
    
    fn add_characteristic(&mut self, _nrf_id: NRF52ID, _uuid: &[u8]) -> Result<(), NRF52Error> {
        Ok(())
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
