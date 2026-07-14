#![no_std]
#![no_main]

/// OOP-based WiFi for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1966
/// Implements WiFi module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WiFiID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WiFiError { Success = 0, NotFound = 1, NotConnected = 2 }

pub trait WiFiModule {
    fn id(&self) -> WiFiID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleWiFiModule {
    pub id: WiFiID,
    pub connected: AtomicUsize,
}

impl SimpleWiFiModule {
    pub fn new(id: WiFiID) -> Self {
        SimpleWiFiModule {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl WiFiModule for SimpleWiFiModule {
    fn id(&self) -> WiFiID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait WiFiController {
    fn connect(&mut self, wifi_id: WiFiID, ssid: &[u8], password: &[u8]) -> Result<(), WiFiError>;
    fn disconnect(&mut self, wifi_id: WiFiID) -> Result<(), WiFiError>;
    def get_ip(&self, wifi_id: WiFiID) -> Result<[u8; 4], WiFiError>;
}

#[repr(C)]
pub struct SimpleWiFiController {
    pub modules: Vec<Option<Box<dyn WiFiModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleWiFiController {
    pub fn new() -> Self {
        SimpleWiFiController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WiFiController for SimpleWiFiController {
    fn connect(&mut self, wifi_id: WiFiID, _ssid: &[u8], _password: &[u8]) -> Result<(), WiFiError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == wifi_id {
                    module.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WiFiError::NotFound)
    }
    
    fn disconnect(&mut self, wifi_id: WiFiID) -> Result<(), WiFiError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == wifi_id {
                    module.connected.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WiFiError::NotFound)
    }
    
    fn get_ip(&self, wifi_id: WiFiID) -> Result<[u8; 4], WiFiError> {
        if self.get_module(wifi_id).is_some() {
            Ok([192, 168, 1, 100])
        } else {
            Err(WiFiError::NotFound)
        }
    }
    
    fn get_module(&self, id: WiFiID) -> Option<&dyn WiFiModule> {
        for module_option in &self.modules {
            if let Some(ref module) = *module_option {
                if module.id() == id { return Some(module.as_ref()); }
            }
        }
        None
    }
}

pub trait APMode {
    def start_ap(&mut self, wifi_id: WiFiID, ssid: &[u8], password: &[u8]) -> Result<(), WiFiError>;
    def get_stations(&self, wifi_id: WiFiID) -> Result<u8, WiFiError>;
}

#[repr(C)]
pub struct SimpleAPMode {
    pub controller: SimpleWiFiController,
}

impl SimpleAPMode {
    pub fn new(controller: SimpleWiFiController) -> Self {
        SimpleAPMode { controller }
    }
}

impl APMode for SimpleAPMode {
    fn start_ap(&mut self, _wifi_id: WiFiID, _ssid: &[u8], _password: &[u8]) -> Result<(), WiFiError> {
        Ok(())
    }
    
    fn get_stations(&self, wifi_id: WiFiID) -> Result<u8, WiFiError> {
        if self.controller.get_module(wifi_id).is_some() {
            Ok(0)
        } else {
            Err(WiFiError::NotFound)
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
