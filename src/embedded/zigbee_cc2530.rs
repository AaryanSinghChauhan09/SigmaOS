#![no_std]
#![no_main]

/// OOP-based CC2530 ZigBee for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3056
/// Implements CC2530 ZigBee transceiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CC2530ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CC2530Error { Success = 0, NotFound = 1 }

pub trait CC2530Device {
    fn id(&self) -> CC2530ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCC2530Device {
    pub id: CC2530ID,
    pub initialized: AtomicUsize,
}

impl SimpleCC2530Device {
    pub fn new(id: CC2530ID) -> Self {
        SimpleCC2530Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl CC2530Device for SimpleCC2530Device {
    fn id(&self) -> CC2530ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait CC2530Controller {
    fn init(&mut self, cc_id: CC2530ID) -> Result<(), CC2530Error>;
    fn send(&self, cc_id: CC2530ID, data: &[u8]) -> Result<(), CC2530Error>;
    def receive(&self, cc_id: CC2530ID, buffer: &mut [u8]) -> Result<usize, CC2530Error>;
}

#[repr(C)]
pub struct SimpleCC2530Controller {
    pub devices: Vec<Option<Box<dyn CC2530Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCC2530Controller {
    pub fn new() -> Self {
        SimpleCC2530Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CC2530Controller for SimpleCC2530Controller {
    fn init(&mut self, cc_id: CC2530ID) -> Result<(), CC2530Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == cc_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(CC2530Error::NotFound)
    }
    
    fn send(&self, cc_id: CC2530ID, _data: &[u8]) -> Result<(), CC2530Error> {
        if self.get_device(cc_id).is_some() {
            Ok(())
        } else {
            Err(CC2530Error::NotFound)
        }
    }
    
    fn receive(&self, cc_id: CC2530ID, buffer: &mut [u8]) -> Result<usize, CC2530Error> {
        if self.get_device(cc_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(CC2530Error::NotFound)
        }
    }
    
    fn get_device(&self, id: CC2530ID) -> Option<&dyn CC2530Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait CC2530Network {
    def set_pan_id(&mut self, cc_id: CC2530ID, pan_id: u16) -> Result<(), CC2530Error>;
    def set_channel(&mut self, cc_id: CC2530ID, channel: u8) -> Result<(), CC2530Error>;
}

#[repr(C)]
pub struct SimpleCC2530Network {
    pub controller: SimpleCC2530Controller,
    pub pan_ids: Vec<(CC2530ID, AtomicUsize)>,
}

impl SimpleCC2530Network {
    pub fn new(controller: SimpleCC2530Controller) -> Self {
        SimpleCC2530Network {
            controller,
            pan_ids: Vec::new(),
        }
    }
}

impl CC2530Network for SimpleCC2530Network {
    fn set_pan_id(&mut self, cc_id: CC2530ID, pan_id: u16) -> Result<(), CC2530Error> {
        self.pan_ids.push((cc_id, AtomicUsize::new(pan_id as usize)));
        Ok(())
    }
    
    fn set_channel(&mut self, _cc_id: CC2530ID, _channel: u8) -> Result<(), CC2530Error> {
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
