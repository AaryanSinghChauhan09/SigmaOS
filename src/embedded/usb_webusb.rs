#![no_std]
#![no_main]

/// OOP-based USB WebUSB for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2236
/// Implements USB WebUSB

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WebUSBID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WebUSBError { Success = 0, NotFound = 1 }

pub trait USBWebUSB {
    fn id(&self) -> WebUSBID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBWebUSB {
    pub id: WebUSBID,
    pub connected: AtomicUsize,
}

impl SimpleUSBWebUSB {
    pub fn new(id: WebUSBID) -> Self {
        SimpleUSBWebUSB {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBWebUSB for SimpleUSBWebUSB {
    fn id(&self) -> WebUSBID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait WebUSBController {
    fn init(&mut self, webusb_id: WebUSBID) -> Result<(), WebUSBError>;
    fn send(&self, webusb_id: WebUSBID, data: &[u8]) -> Result<usize, WebUSBError>;
    def receive(&self, webusb_id: WebUSBID, buffer: &mut [u8]) -> Result<usize, WebUSBError>;
}

#[repr(C)]
pub struct SimpleWebUSBController {
    pub webusb_devices: Vec<Option<Box<dyn USBWebUSB>>>,
    pub next_id: AtomicUsize,
}

impl SimpleWebUSBController {
    pub fn new() -> Self {
        SimpleWebUSBController {
            webusb_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WebUSBController for SimpleWebUSBController {
    fn init(&mut self, webusb_id: WebUSBID) -> Result<(), WebUSBError> {
        for webusb_option in &mut self.webusb_devices {
            if let Some(ref mut webusb) = *webusb_option {
                if webusb.id() == webusb_id {
                    webusb.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WebUSBError::NotFound)
    }
    
    fn send(&self, webusb_id: WebUSBID, _data: &[u8]) -> Result<usize, WebUSBError> {
        if self.get_webusb(webusb_id).is_some() {
            Ok(0)
        } else {
            Err(WebUSBError::NotFound)
        }
    }
    
    fn receive(&self, webusb_id: WebUSBID, buffer: &mut [u8]) -> Result<usize, WebUSBError> {
        if self.get_webusb(webusb_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(WebUSBError::NotFound)
        }
    }
    
    fn get_webusb(&self, id: WebUSBID) -> Option<&dyn USBWebUSB> {
        for webusb_option in &self.webusb_devices {
            if let Some(ref webusb) = *webusb_option {
                if webusb.id() == id { return Some(webusb.as_ref()); }
            }
        }
        None
    }
}

pub trait WebURL {
    def set_landing_url(&mut self, webusb_id: WebUSBID, url: &[u8]) -> Result<(), WebUSBError>;
    def get_landing_url(&self, webusb_id: WebUSBID) -> Result<[u8; 128], WebUSBError>;
}

#[repr(C)]
pub struct SimpleWebURL {
    pub controller: SimpleWebUSBController,
    pub urls: Vec<(WebUSBID, [u8; 128])>,
}

impl SimpleWebURL {
    pub fn new(controller: SimpleWebUSBController) -> Self {
        SimpleWebURL {
            controller,
            urls: Vec::new(),
        }
    }
}

impl WebURL for SimpleWebURL {
    fn set_landing_url(&mut self, webusb_id: WebUSBID, url: &[u8]) -> Result<(), WebUSBError> {
        let mut url_array = [0u8; 128];
        let len = url.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(url.as_ptr(), url_array.as_mut_ptr(), len);
        }
        self.urls.push((webusb_id, url_array));
        Ok(())
    }
    
    fn get_landing_url(&self, webusb_id: WebUSBID) -> Result<[u8; 128], WebUSBError> {
        for &(id, ref url) in &self.urls {
            if id == webusb_id {
                return Ok(*url);
            }
        }
        Err(WebUSBError::NotFound)
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
