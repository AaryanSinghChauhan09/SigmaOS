#![no_std]
#![no_main]

/// OOP-based W5500 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2986
/// Implements W5500 Ethernet controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type W5500ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum W5500Error { Success = 0, NotFound = 1 }

pub trait W5500Device {
    fn id(&self) -> W5500ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleW5500Device {
    pub id: W5500ID,
    pub initialized: AtomicUsize,
}

impl SimpleW5500Device {
    pub fn new(id: W5500ID) -> Self {
        SimpleW5500Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl W5500Device for SimpleW5500Device {
    fn id(&self) -> W5500ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait W5500Controller {
    fn init(&mut self, w_id: W5500ID) -> Result<(), W5500Error>;
    fn read(&self, w_id: W5500ID, socket: u8, buffer: &mut [u8]) -> Result<usize, W5500Error>;
    def write(&self, w_id: W5500ID, socket: u8, data: &[u8]) -> Result<usize, W5500Error>;
}

#[repr(C)]
pub struct SimpleW5500Controller {
    pub devices: Vec<Option<Box<dyn W5500Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleW5500Controller {
    pub fn new() -> Self {
        SimpleW5500Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl W5500Controller for SimpleW5500Controller {
    fn init(&mut self, w_id: W5500ID) -> Result<(), W5500Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == w_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(W5500Error::NotFound)
    }
    
    fn read(&self, w_id: W5500ID, _socket: u8, buffer: &mut [u8]) -> Result<usize, W5500Error> {
        if self.get_device(w_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(W5500Error::NotFound)
        }
    }
    
    fn write(&self, w_id: W5500ID, _socket: u8, data: &[u8]) -> Result<usize, W5500Error> {
        if self.get_device(w_id).is_some() {
            Ok(data.len())
        } else {
            Err(W5500Error::NotFound)
        }
    }
    
    fn get_device(&self, id: W5500ID) -> Option<&dyn W5500Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait W5500Config {
    def set_mac(&mut self, w_id: W5500ID, mac: [u8; 6]) -> Result<(), W5500Error>;
    def set_ip(&mut self, w_id: W5500ID, ip: [u8; 4]) -> Result<(), W5500Error>;
}

#[repr(C)]
pub struct SimpleW5500Config {
    pub controller: SimpleW5500Controller,
}

impl SimpleW5500Config {
    pub fn new(controller: SimpleW5500Controller) -> Self {
        SimpleW5500Config { controller }
    }
}

impl W5500Config for SimpleW5500Config {
    fn set_mac(&mut self, _w_id: W5500ID, _mac: [u8; 6]) -> Result<(), W5500Error> {
        Ok(())
    }
    
    fn set_ip(&mut self, _w_id: W5500ID, _ip: [u8; 4]) -> Result<(), W5500Error> {
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
