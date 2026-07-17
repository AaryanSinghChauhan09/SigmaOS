#![no_std]
#![no_main]

/// OOP-based ESP32 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3016
/// Implements ESP32 WiFi/BT module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ESP32ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ESP32Error { Success = 0, NotFound = 1 }

pub trait ESP32Device {
    fn id(&self) -> ESP32ID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleESP32Device {
    pub id: ESP32ID,
    pub connected: AtomicUsize,
}

impl SimpleESP32Device {
    pub fn new(id: ESP32ID) -> Self {
        SimpleESP32Device {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl ESP32Device for SimpleESP32Device {
    fn id(&self) -> ESP32ID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait ESP32Controller {
    fn init(&mut self, esp_id: ESP32ID) -> Result<(), ESP32Error>;
    fn connect_wifi(&self, esp_id: ESP32ID, ssid: &[u8], pass: &[u8]) -> Result<(), ESP32Error>;
    def send(&self, esp_id: ESP32ID, data: &[u8]) -> Result<usize, ESP32Error>;
}

#[repr(C)]
pub struct SimpleESP32Controller {
    pub devices: Vec<Option<Box<dyn ESP32Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleESP32Controller {
    pub fn new() -> Self {
        SimpleESP32Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ESP32Controller for SimpleESP32Controller {
    fn init(&mut self, esp_id: ESP32ID) -> Result<(), ESP32Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == esp_id {
                    device.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ESP32Error::NotFound)
    }
    
    fn connect_wifi(&self, esp_id: ESP32ID, _ssid: &[u8], _pass: &[u8]) -> Result<(), ESP32Error> {
        if self.get_device(esp_id).is_some() {
            Ok(())
        } else {
            Err(ESP32Error::NotFound)
        }
    }
    
    fn send(&self, esp_id: ESP32ID, data: &[u8]) -> Result<usize, ESP32Error> {
        if self.get_device(esp_id).is_some() {
            Ok(data.len())
        } else {
            Err(ESP32Error::NotFound)
        }
    }
    
    fn get_device(&self, id: ESP32ID) -> Option<&dyn ESP32Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait ESP32Bluetooth {
    def init_bt(&mut self, esp_id: ESP32ID) -> Result<(), ESP32Error>;
    def send_bt(&self, esp_id: ESP32ID, data: &[u8]) -> Result<usize, ESP32Error>;
}

#[repr(C)]
pub struct SimpleESP32Bluetooth {
    pub controller: SimpleESP32Controller,
}

impl SimpleESP32Bluetooth {
    pub fn new(controller: SimpleESP32Controller) -> Self {
        SimpleESP32Bluetooth { controller }
    }
}

impl ESP32Bluetooth for SimpleESP32Bluetooth {
    fn init_bt(&mut self, _esp_id: ESP32ID) -> Result<(), ESP32Error> {
        Ok(())
    }
    
    fn send_bt(&self, esp_id: ESP32ID, data: &[u8]) -> Result<usize, ESP32Error> {
        if self.controller.get_device(esp_id).is_some() {
            Ok(data.len())
        } else {
            Err(ESP32Error::NotFound)
        }
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
