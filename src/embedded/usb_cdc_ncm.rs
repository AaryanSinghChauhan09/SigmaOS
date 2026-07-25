#![no_std]
#![no_main]

/// OOP-based USB CDC NCM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2296
/// Implements USB CDC NCM (Network Control Model)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NCMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NCMError { Success = 0, NotFound = 1 }

pub trait USBCDCNCM {
    fn id(&self) -> NCMID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBCDCNCM {
    pub id: NCMID,
    pub connected: AtomicUsize,
}

impl SimpleUSBCDCNCM {
    pub fn new(id: NCMID) -> Self {
        SimpleUSBCDCNCM {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBCDCNCM for SimpleUSBCDCNCM {
    fn id(&self) -> NCMID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait NCMController {
    fn init(&mut self, ncm_id: NCMID) -> Result<(), NCMError>;
    fn send_ntb(&self, ncm_id: NCMID, ntb: &[u8]) -> Result<usize, NCMError>;
    def receive_ntb(&self, ncm_id: NCMID, buffer: &mut [u8]) -> Result<usize, NCMError>;
}

#[repr(C)]
pub struct SimpleNCMController {
    pub ncm_devices: Vec<Option<Box<dyn USBCDCNCM>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNCMController {
    pub fn new() -> Self {
        SimpleNCMController {
            ncm_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NCMController for SimpleNCMController {
    fn init(&mut self, ncm_id: NCMID) -> Result<(), NCMError> {
        for ncm_option in &mut self.ncm_devices {
            if let Some(ref mut ncm) = *ncm_option {
                if ncm.id() == ncm_id {
                    ncm.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(NCMError::NotFound)
    }
    
    fn send_ntb(&self, ncm_id: NCMID, _ntb: &[u8]) -> Result<usize, NCMError> {
        if self.get_ncm(ncm_id).is_some() {
            Ok(0)
        } else {
            Err(NCMError::NotFound)
        }
    }
    
    fn receive_ntb(&self, ncm_id: NCMID, buffer: &mut [u8]) -> Result<usize, NCMError> {
        if self.get_ncm(ncm_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(NCMError::NotFound)
        }
    }
    
    fn get_ncm(&self, id: NCMID) -> Option<&dyn USBCDCNCM> {
        for ncm_option in &self.ncm_devices {
            if let Some(ref ncm) = *ncm_option {
                if ncm.id() == id { return Some(ncm.as_ref()); }
            }
        }
        None
    }
}

pub trait NCMDatagram {
    def set_datagram_size(&mut self, ncm_id: NCMID, size: u16) -> Result<(), NCMError>;
    def get_datagram_size(&self, ncm_id: NCMID) -> Result<u16, NCMError>;
}

#[repr(C)]
pub struct SimpleNCMDatagram {
    pub controller: SimpleNCMController,
    pub sizes: Vec<(NCMID, AtomicUsize)>,
}

impl SimpleNCMDatagram {
    pub fn new(controller: SimpleNCMController) -> Self {
        SimpleNCMDatagram {
            controller,
            sizes: Vec::new(),
        }
    }
}

impl NCMDatagram for SimpleNCMDatagram {
    fn set_datagram_size(&mut self, ncm_id: NCMID, size: u16) -> Result<(), NCMError> {
        self.sizes.push((ncm_id, AtomicUsize::new(size as usize)));
        Ok(())
    }
    
    fn get_datagram_size(&self, ncm_id: NCMID) -> Result<u16, NCMError> {
        for &(id, ref size) in &self.sizes {
            if id == ncm_id {
                return Ok(size.load(Ordering::SeqCst) as u16);
            }
        }
        Err(NCMError::NotFound)
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
