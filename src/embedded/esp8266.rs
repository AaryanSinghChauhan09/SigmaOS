#![no_std]
#![no_main]

/// OOP-based ESP8266 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3006
/// Implements ESP8266 WiFi module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ESP8266ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ESP8266Error { Success = 0, NotFound = 1 }

pub trait ESP8266Device {
    fn id(&self) -> ESP8266ID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleESP8266Device {
    pub id: ESP8266ID,
    pub connected: AtomicUsize,
}

impl SimpleESP8266Device {
    pub fn new(id: ESP8266ID) -> Self {
        SimpleESP8266Device {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl ESP8266Device for SimpleESP8266Device {
    fn id(&self) -> ESP8266ID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait ESP8266Controller {
    fn init(&mut self, esp_id: ESP8266ID) -> Result<(), ESP8266Error>;
    fn connect(&self, esp_id: ESP8266ID, ssid: &[u8], pass: &[u8]) -> Result<(), ESP8266Error>;
    def send(&self, esp_id: ESP8266ID, data: &[u8]) -> Result<usize, ESP8266Error>;
}

#[repr(C)]
pub struct SimpleESP8266Controller {
    pub devices: Vec<Option<Box<dyn ESP8266Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleESP8266Controller {
    pub fn new() -> Self {
        SimpleESP8266Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ESP8266Controller for SimpleESP8266Controller {
    fn init(&mut self, esp_id: ESP8266ID) -> Result<(), ESP8266Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == esp_id {
                    device.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ESP8266Error::NotFound)
    }
    
    fn connect(&self, esp_id: ESP8266ID, _ssid: &[u8], _pass: &[u8]) -> Result<(), ESP8266Error> {
        if self.get_device(esp_id).is_some() {
            Ok(())
        } else {
            Err(ESP8266Error::NotFound)
        }
    }
    
    fn send(&self, esp_id: ESP8266ID, data: &[u8]) -> Result<usize, ESP8266Error> {
        if self.get_device(esp_id).is_some() {
            Ok(data.len())
        } else {
            Err(ESP8266Error::NotFound)
        }
    }
    
    fn get_device(&self, id: ESP8266ID) -> Option<&dyn ESP8266Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait ESP8266AP {
    def start_ap(&mut self, esp_id: ESP8266ID, ssid: &[u8]) -> Result<(), ESP8266Error>;
    def stop_ap(&mut self, esp_id: ESP8266ID) -> Result<(), ESP8266Error>;
}

#[repr(C)]
pub struct SimpleESP8266AP {
    pub controller: SimpleESP8266Controller,
}

impl SimpleESP8266AP {
    pub fn new(controller: SimpleESP8266Controller) -> Self {
        SimpleESP8266AP { controller }
    }
}

impl ESP8266AP for SimpleESP8266AP {
    fn start_ap(&mut self, _esp_id: ESP8266ID, _ssid: &[u8]) -> Result<(), ESP8266Error> {
        Ok(())
    }
    
    fn stop_ap(&mut self, _esp_id: ESP8266ID) -> Result<(), ESP8266Error> {
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
