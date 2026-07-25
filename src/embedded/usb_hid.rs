#![no_std]
#![no_main]

/// OOP-based USB HID for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2206
/// Implements USB HID (Human Interface Device)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HIDID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HIDError { Success = 0, NotFound = 1 }

pub trait USBHID {
    fn id(&self) -> HIDID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBHID {
    pub id: HIDID,
    pub connected: AtomicUsize,
}

impl SimpleUSBHID {
    pub fn new(id: HIDID) -> Self {
        SimpleUSBHID {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBHID for SimpleUSBHID {
    fn id(&self) -> HIDID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait HIDController {
    fn init(&mut self, hid_id: HIDID) -> Result<(), HIDError>;
    fn send_report(&self, hid_id: HIDID, report: &[u8]) -> Result<(), HIDError>;
    def receive_report(&self, hid_id: HIDID, buffer: &mut [u8]) -> Result<usize, HIDError>;
}

#[repr(C)]
pub struct SimpleHIDController {
    pub hid_devices: Vec<Option<Box<dyn USBHID>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHIDController {
    pub fn new() -> Self {
        SimpleHIDController {
            hid_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HIDController for SimpleHIDController {
    fn init(&mut self, hid_id: HIDID) -> Result<(), HIDError> {
        for hid_option in &mut self.hid_devices {
            if let Some(ref mut hid) = *hid_option {
                if hid.id() == hid_id {
                    hid.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HIDError::NotFound)
    }
    
    fn send_report(&self, hid_id: HIDID, _report: &[u8]) -> Result<(), HIDError> {
        if self.get_hid(hid_id).is_some() {
            Ok(())
        } else {
            Err(HIDError::NotFound)
        }
    }
    
    fn receive_report(&self, hid_id: HIDID, buffer: &mut [u8]) -> Result<usize, HIDError> {
        if self.get_hid(hid_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(HIDError::NotFound)
        }
    }
    
    fn get_hid(&self, id: HIDID) -> Option<&dyn USBHID> {
        for hid_option in &self.hid_devices {
            if let Some(ref hid) = *hid_option {
                if hid.id() == id { return Some(hid.as_ref()); }
            }
        }
        None
    }
}

pub trait Keyboard {
    def send_key(&self, hid_id: HIDID, key: u8, modifier: u8) -> Result<(), HIDError>;
    def release_key(&self, hid_id: HIDID, key: u8) -> Result<(), HIDError>;
}

#[repr(C)]
pub struct SimpleKeyboard {
    pub controller: SimpleHIDController,
}

impl SimpleKeyboard {
    pub fn new(controller: SimpleHIDController) -> Self {
        SimpleKeyboard { controller }
    }
}

impl Keyboard for SimpleKeyboard {
    fn send_key(&self, hid_id: HIDID, _key: u8, _modifier: u8) -> Result<(), HIDError> {
        if self.controller.get_hid(hid_id).is_some() {
            Ok(())
        } else {
            Err(HIDError::NotFound)
        }
    }
    
    fn release_key(&self, hid_id: HIDID, _key: u8) -> Result<(), HIDError> {
        if self.controller.get_hid(hid_id).is_some() {
            Ok(())
        } else {
            Err(HIDError::NotFound)
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
