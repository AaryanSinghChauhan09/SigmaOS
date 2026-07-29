#![no_std]
#![no_main]

/// OOP-based ESP32 WiFi for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4186
/// Implements ESP32 WiFi module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ESP32ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ESP32Error { Success = 0, NotFound = 1 }

pub trait ESP32Device {
    fn id(&self) -> ESP32ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleESP32Device {
    pub id: ESP32ID,
    pub initialized: AtomicUsize,
}

impl SimpleESP32Device {
    pub fn new(id: ESP32ID) -> Self {
        SimpleESP32Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ESP32Device for SimpleESP32Device {
    fn id(&self) -> ESP32ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ESP32Controller {
    fn init(&mut self, dev_id: ESP32ID) -> Result<(), ESP32Error>;
    fn connect(&self, dev_id: ESP32ID, ssid: &[u8], pass: &[u8]) -> Result<(), ESP32Error>;
    def disconnect(&self, dev_id: ESP32ID) -> Result<(), ESP32Error>;
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
    fn init(&mut self, dev_id: ESP32ID) -> Result<(), ESP32Error> {
        for dev_option in &mut self.devices {
            if let Some(ref mut dev) = *dev_option {
                if dev.id() == dev_id {
                    dev.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ESP32Error::NotFound)
    }
    
    fn connect(&self, dev_id: ESP32ID, _ssid: &[u8], _pass: &[u8]) -> Result<(), ESP32Error> {
        if self.get_device(dev_id).is_some() {
            Ok(())
        } else {
            Err(ESP32Error::NotFound)
        }
    }
    
    fn disconnect(&self, dev_id: ESP32ID) -> Result<(), ESP32Error> {
        if self.get_device(dev_id).is_some() {
            Ok(())
        } else {
            Err(ESP32Error::NotFound)
        }
    }
    
    fn get_device(&self, id: ESP32ID) -> Option<&dyn ESP32Device> {
        for dev_option in &self.devices {
            if let Some(ref dev) = *dev_option {
                if dev.id() == id { return Some(dev.as_ref()); }
            }
        }
        None
    }
}

pub trait ESP32Bluetooth {
    def enable_bluetooth(&mut self, dev_id: ESP32ID, enable: bool) -> Result<(), ESP32Error>;
}

#[repr(C)]
pub struct SimpleESP32Bluetooth {
    pub controller: SimpleESP32Controller,
    pub bt_states: Vec<(ESP32ID, AtomicUsize)>,
}

impl SimpleESP32Bluetooth {
    pub fn new(controller: SimpleESP32Controller) -> Self {
        SimpleESP32Bluetooth {
            controller,
            bt_states: Vec::new(),
        }
    }
}

impl ESP32Bluetooth for SimpleESP32Bluetooth {
    fn enable_bluetooth(&mut self, dev_id: ESP32ID, enable: bool) -> Result<(), ESP32Error> {
        self.bt_states.push((dev_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
