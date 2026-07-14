#![no_std]
#![no_main]

/// OOP-based USB Device for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1176
/// Implements USB device stack

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EndpointID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EndpointType { Control = 0, Isochronous = 1, Bulk = 2, Interrupt = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum USBError { Success = 0, NotFound = 1 }

pub trait USBEndpoint {
    fn id(&self) -> EndpointID;
    fn endpoint_type(&self) -> EndpointType;
    fn max_packet_size(&self) -> u16;
}

#[repr(C)]
pub struct SimpleUSBEndpoint {
    pub id: EndpointID,
    pub endpoint_type: AtomicUsize,
    pub max_packet_size: AtomicUsize,
}

impl SimpleUSBEndpoint {
    pub fn new(id: EndpointID, endpoint_type: EndpointType, max_packet_size: u16) -> Self {
        SimpleUSBEndpoint {
            id,
            endpoint_type: AtomicUsize::new(endpoint_type as usize),
            max_packet_size: AtomicUsize::new(max_packet_size as usize),
        }
    }
}

impl USBEndpoint for SimpleUSBEndpoint {
    fn id(&self) -> EndpointID { self.id }
    fn endpoint_type(&self) -> EndpointType { unsafe { core::mem::transmute(self.endpoint_type.load(Ordering::SeqCst)) } }
    fn max_packet_size(&self) -> u16 { self.max_packet_size.load(Ordering::SeqCst) as u16 }
}

pub trait USBDevice {
    fn vid(&self) -> u16;
    fn pid(&self) -> u16;
    def configure(&mut self) -> Result<(), USBError>;
}

#[repr(C)]
pub struct SimpleUSBDevice {
    pub vid: AtomicUsize,
    pub pid: AtomicUsize,
}

impl SimpleUSBDevice {
    pub fn new(vid: u16, pid: u16) -> Self {
        SimpleUSBDevice {
            vid: AtomicUsize::new(vid as usize),
            pid: AtomicUsize::new(pid as usize),
        }
    }
}

impl USBDevice for SimpleUSBDevice {
    fn vid(&self) -> u16 { self.vid.load(Ordering::SeqCst) as u16 }
    fn pid(&self) -> u16 { self.pid.load(Ordering::SeqCst) as u16 }
    
    fn configure(&mut self) -> Result<(), USBError> {
        Ok(())
    }
}

pub trait USBStack {
    fn init(&mut self) -> Result<(), USBError>;
    fn add_endpoint(&mut self, endpoint: Box<dyn USBEndpoint>) -> Result<EndpointID, USBError>;
    fn send(&self, endpoint_id: EndpointID, data: &[u8]) -> Result<(), USBError>;
    fn receive(&self, endpoint_id: EndpointID, buffer: &mut [u8]) -> Result<usize, USBError>;
}

#[repr(C)]
pub struct SimpleUSBStack {
    pub device: SimpleUSBDevice,
    pub endpoints: Vec<Option<Box<dyn USBEndpoint>>>,
    pub next_id: AtomicUsize,
}

impl SimpleUSBStack {
    pub fn new(device: SimpleUSBDevice) -> Self {
        SimpleUSBStack {
            device,
            endpoints: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl USBStack for SimpleUSBStack {
    fn init(&mut self) -> Result<(), USBError> {
        Ok(())
    }
    
    fn add_endpoint(&mut self, endpoint: Box<dyn USBEndpoint>) -> Result<EndpointID, USBError> {
        let id = endpoint.id();
        self.endpoints.push(Some(endpoint));
        Ok(id)
    }
    
    fn send(&self, endpoint_id: EndpointID, _data: &[u8]) -> Result<(), USBError> {
        if self.get_endpoint(endpoint_id).is_some() {
            Ok(())
        } else {
            Err(USBError::NotFound)
        }
    }
    
    fn receive(&self, endpoint_id: EndpointID, buffer: &mut [u8]) -> Result<usize, USBError> {
        if self.get_endpoint(endpoint_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(USBError::NotFound)
        }
    }
    
    fn get_endpoint(&self, id: EndpointID) -> Option<&dyn USBEndpoint> {
        for endpoint_option in &self.endpoints {
            if let Some(ref endpoint) = *endpoint_option {
                if endpoint.id() == id { return Some(endpoint.as_ref()); }
            }
        }
        None
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
