#![no_std]
#![no_main]

/// OOP-based USB RNDIS for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2306
/// Implements USB RNDIS (Remote NDIS)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RNDISID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RNDISError { Success = 0, NotFound = 1 }

pub trait USBRNDIS {
    fn id(&self) -> RNDISID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBRNDIS {
    pub id: RNDISID,
    pub connected: AtomicUsize,
}

impl SimpleUSBRNDIS {
    pub fn new(id: RNDISID) -> Self {
        SimpleUSBRNDIS {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBRNDIS for SimpleUSBRNDIS {
    fn id(&self) -> RNDISID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait RNDISController {
    fn init(&mut self, rndis_id: RNDISID) -> Result<(), RNDISError>;
    fn send_packet(&self, rndis_id: RNDISID, packet: &[u8]) -> Result<usize, RNDISError>;
    def receive_packet(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<usize, RNDISError>;
}

#[repr(C)]
pub struct SimpleRNDISController {
    pub rndis_devices: Vec<Option<Box<dyn USBRNDIS>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRNDISController {
    pub fn new() -> Self {
        SimpleRNDISController {
            rndis_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RNDISController for SimpleRNDISController {
    fn init(&mut self, rndis_id: RNDISID) -> Result<(), RNDISError> {
        for rndis_option in &mut self.rndis_devices {
            if let Some(ref mut rndis) = *rndis_option {
                if rndis.id() == rndis_id {
                    rndis.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RNDISError::NotFound)
    }
    
    fn send_packet(&self, rndis_id: RNDISID, _packet: &[u8]) -> Result<usize, RNDISError> {
        if self.get_rndis(rndis_id).is_some() {
            Ok(0)
        } else {
            Err(RNDISError::NotFound)
        }
    }
    
    fn receive_packet(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<usize, RNDISError> {
        if self.get_rndis(rndis_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(RNDISError::NotFound)
        }
    }
    
    fn get_rndis(&self, id: RNDISID) -> Option<&dyn USBRNDIS> {
        for rndis_option in &self.rndis_devices {
            if let Some(ref rndis) = *rndis_option {
                if rndis.id() == id { return Some(rndis.as_ref()); }
            }
        }
        None
    }
}

pub trait RNDISMessage {
    def send_msg(&self, rndis_id: RNDISID, msg_id: u32, data: &[u8]) -> Result<(), RNDISError>;
    def receive_msg(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<(u32, usize), RNDISError>;
}

#[repr(C)]
pub struct SimpleRNDISMessage {
    pub controller: SimpleRNDISController,
}

impl SimpleRNDISMessage {
    pub fn new(controller: SimpleRNDISController) -> Self {
        SimpleRNDISMessage { controller }
    }
}

impl RNDISMessage for SimpleRNDISMessage {
    fn send_msg(&self, rndis_id: RNDISID, _msg_id: u32, _data: &[u8]) -> Result<(), RNDISError> {
        if self.controller.get_rndis(rndis_id).is_some() {
            Ok(())
        } else {
            Err(RNDISError::NotFound)
        }
    }
    
    fn receive_msg(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<(u32, usize), RNDISError> {
        if self.controller.get_rndis(rndis_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok((0, buffer.len()))
        } else {
            Err(RNDISError::NotFound)
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
