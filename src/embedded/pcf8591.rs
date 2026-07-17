#![no_std]
#![no_main]

/// OOP-based PCF8591 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2866
/// Implements PCF8591 ADC/DAC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCF8591ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCF8591Error { Success = 0, NotFound = 1 }

pub trait PCF8591Device {
    fn id(&self) -> PCF8591ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCF8591Device {
    pub id: PCF8591ID,
    pub initialized: AtomicUsize,
}

impl SimplePCF8591Device {
    pub fn new(id: PCF8591ID) -> Self {
        SimplePCF8591Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCF8591Device for SimplePCF8591Device {
    fn id(&self) -> PCF8591ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCF8591Controller {
    fn init(&mut self, pcf_id: PCF8591ID) -> Result<(), PCF8591Error>;
    fn read_adc(&self, pcf_id: PCF8591ID, channel: u8) -> Result<u8, PCF8591Error>;
    def write_dac(&self, pcf_id: PCF8591ID, value: u8) -> Result<(), PCF8591Error>;
}

#[repr(C)]
pub struct SimplePCF8591Controller {
    pub devices: Vec<Option<Box<dyn PCF8591Device>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCF8591Controller {
    pub fn new() -> Self {
        SimplePCF8591Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCF8591Controller for SimplePCF8591Controller {
    fn init(&mut self, pcf_id: PCF8591ID) -> Result<(), PCF8591Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == pcf_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCF8591Error::NotFound)
    }
    
    fn read_adc(&self, pcf_id: PCF8591ID, _channel: u8) -> Result<u8, PCF8591Error> {
        if self.get_device(pcf_id).is_some() {
            Ok(0)
        } else {
            Err(PCF8591Error::NotFound)
        }
    }
    
    fn write_dac(&self, pcf_id: PCF8591ID, _value: u8) -> Result<(), PCF8591Error> {
        if self.get_device(pcf_id).is_some() {
            Ok(())
        } else {
            Err(PCF8591Error::NotFound)
        }
    }
    
    fn get_device(&self, id: PCF8591ID) -> Option<&dyn PCF8591Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait PCF8591Config {
    def set_output_enable(&mut self, pcf_id: PCF8591ID, enable: bool) -> Result<(), PCF8591Error>;
    def get_output_enable(&self, pcf_id: PCF8591ID) -> Result<bool, PCF8591Error>;
}

#[repr(C)]
pub struct SimplePCF8591Config {
    pub controller: SimplePCF8591Controller,
    pub output_enables: Vec<(PCF8591ID, AtomicUsize)>,
}

impl SimplePCF8591Config {
    pub fn new(controller: SimplePCF8591Controller) -> Self {
        SimplePCF8591Config {
            controller,
            output_enables: Vec::new(),
        }
    }
}

impl PCF8591Config for SimplePCF8591Config {
    fn set_output_enable(&mut self, pcf_id: PCF8591ID, enable: bool) -> Result<(), PCF8591Error> {
        self.output_enables.push((pcf_id, AtomicUsize::new(if enable { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_output_enable(&self, pcf_id: PCF8591ID) -> Result<bool, PCF8591Error> {
        for &(id, ref enable) in &self.output_enables {
            if id == pcf_id {
                return Ok(enable.load(Ordering::SeqCst) == 1);
            }
        }
        Err(PCF8591Error::NotFound)
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
