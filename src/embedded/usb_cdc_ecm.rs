#![no_std]
#![no_main]

/// OOP-based USB CDC ECM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2286
/// Implements USB CDC ECM (Ethernet Control Model)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ECMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ECMError { Success = 0, NotFound = 1 }

pub trait USBCDCECM {
    fn id(&self) -> ECMID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBCDCECM {
    pub id: ECMID,
    pub connected: AtomicUsize,
}

impl SimpleUSBCDCECM {
    pub fn new(id: ECMID) -> Self {
        SimpleUSBCDCECM {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBCDCECM for SimpleUSBCDCECM {
    fn id(&self) -> ECMID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait ECMController {
    fn init(&mut self, ecm_id: ECMID) -> Result<(), ECMError>;
    fn send_packet(&self, ecm_id: ECMID, packet: &[u8]) -> Result<usize, ECMError>;
    def receive_packet(&self, ecm_id: ECMID, buffer: &mut [u8]) -> Result<usize, ECMError>;
}

#[repr(C)]
pub struct SimpleECMController {
    pub ecm_devices: Vec<Option<Box<dyn USBCDCECM>>>,
    pub next_id: AtomicUsize,
}

impl SimpleECMController {
    pub fn new() -> Self {
        SimpleECMController {
            ecm_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ECMController for SimpleECMController {
    fn init(&mut self, ecm_id: ECMID) -> Result<(), ECMError> {
        for ecm_option in &mut self.ecm_devices {
            if let Some(ref mut ecm) = *ecm_option {
                if ecm.id() == ecm_id {
                    ecm.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ECMError::NotFound)
    }
    
    fn send_packet(&self, ecm_id: ECMID, _packet: &[u8]) -> Result<usize, ECMError> {
        if self.get_ecm(ecm_id).is_some() {
            Ok(0)
        } else {
            Err(ECMError::NotFound)
        }
    }
    
    fn receive_packet(&self, ecm_id: ECMID, buffer: &mut [u8]) -> Result<usize, ECMError> {
        if self.get_ecm(ecm_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(ECMError::NotFound)
        }
    }
    
    fn get_ecm(&self, id: ECMID) -> Option<&dyn USBCDCECM> {
        for ecm_option in &self.ecm_devices {
            if let Some(ref ecm) = *ecm_option {
                if ecm.id() == id { return Some(ecm.as_ref()); }
            }
        }
        None
    }
}

pub trait ECMNetwork {
    def get_mac(&self, ecm_id: ECMID) -> Result<[u8; 6], ECMError>;
    def set_mac(&mut self, ecm_id: ECMID, mac: &[u8]) -> Result<(), ECMError>;
}

#[repr(C)]
pub struct SimpleECMNetwork {
    pub controller: SimpleECMController,
    pub macs: Vec<(ECMID, [u8; 6])>,
}

impl SimpleECMNetwork {
    pub fn new(controller: SimpleECMController) -> Self {
        SimpleECMNetwork {
            controller,
            macs: Vec::new(),
        }
    }
}

impl ECMNetwork for SimpleECMNetwork {
    fn get_mac(&self, ecm_id: ECMID) -> Result<[u8; 6], ECMError> {
        for &(id, ref mac) in &self.macs {
            if id == ecm_id {
                return Ok(*mac);
            }
        }
        Err(ECMError::NotFound)
    }
    
    fn set_mac(&mut self, ecm_id: ECMID, mac: &[u8]) -> Result<(), ECMError> {
        let mut mac_array = [0u8; 6];
        let len = mac.len().min(5);
        unsafe {
            core::ptr::copy_nonoverlapping(mac.as_ptr(), mac_array.as_mut_ptr(), len);
        }
        self.macs.push((ecm_id, mac_array));
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
