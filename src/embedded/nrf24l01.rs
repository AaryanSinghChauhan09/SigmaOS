#![no_std]
#![no_main]

/// OOP-based NRF24L01 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3026
/// Implements NRF24L01 2.4GHz radio

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NRF24L01ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NRF24L01Error { Success = 0, NotFound = 1 }

pub trait NRF24L01Device {
    fn id(&self) -> NRF24L01ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNRF24L01Device {
    pub id: NRF24L01ID,
    pub initialized: AtomicUsize,
}

impl SimpleNRF24L01Device {
    pub fn new(id: NRF24L01ID) -> Self {
        SimpleNRF24L01Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl NRF24L01Device for SimpleNRF24L01Device {
    fn id(&self) -> NRF24L01ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait NRF24L01Controller {
    fn init(&mut self, nrf_id: NRF24L01ID) -> Result<(), NRF24L01Error>;
    fn send(&self, nrf_id: NRF24L01ID, data: &[u8]) -> Result<(), NRF24L01Error>;
    def receive(&self, nrf_id: NRF24L01ID, buffer: &mut [u8]) -> Result<usize, NRF24L01Error>;
}

#[repr(C)]
pub struct SimpleNRF24L01Controller {
    pub devices: Vec<Option<Box<dyn NRF24L01Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNRF24L01Controller {
    pub fn new() -> Self {
        SimpleNRF24L01Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NRF24L01Controller for SimpleNRF24L01Controller {
    fn init(&mut self, nrf_id: NRF24L01ID) -> Result<(), NRF24L01Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == nrf_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(NRF24L01Error::NotFound)
    }
    
    fn send(&self, nrf_id: NRF24L01ID, _data: &[u8]) -> Result<(), NRF24L01Error> {
        if self.get_device(nrf_id).is_some() {
            Ok(())
        } else {
            Err(NRF24L01Error::NotFound)
        }
    }
    
    fn receive(&self, nrf_id: NRF24L01ID, buffer: &mut [u8]) -> Result<usize, NRF24L01Error> {
        if self.get_device(nrf_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(NRF24L01Error::NotFound)
        }
    }
    
    fn get_device(&self, id: NRF24L01ID) -> Option<&dyn NRF24L01Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait NRF24L01Config {
    def set_channel(&mut self, nrf_id: NRF24L01ID, channel: u8) -> Result<(), NRF24L01Error>;
    def set_power(&mut self, nrf_id: NRF24L01ID, power: u8) -> Result<(), NRF24L01Error>;
}

#[repr(C)]
pub struct SimpleNRF24L01Config {
    pub controller: SimpleNRF24L01Controller,
    pub channels: Vec<(NRF24L01ID, AtomicUsize)>,
}

impl SimpleNRF24L01Config {
    pub fn new(controller: SimpleNRF24L01Controller) -> Self {
        SimpleNRF24L01Config {
            controller,
            channels: Vec::new(),
        }
    }
}

impl NRF24L01Config for SimpleNRF24L01Config {
    fn set_channel(&mut self, nrf_id: NRF24L01ID, channel: u8) -> Result<(), NRF24L01Error> {
        self.channels.push((nrf_id, AtomicUsize::new(channel as usize)));
        Ok(())
    }
    
    fn set_power(&mut self, _nrf_id: NRF24L01ID, _power: u8) -> Result<(), NRF24L01Error> {
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
