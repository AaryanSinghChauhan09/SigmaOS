#![no_std]
#![no_main]

/// OOP-based USB CDC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2196
/// Implements USB CDC (Communication Device Class)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CDCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CDCError { Success = 0, NotFound = 1 }

pub trait USBCDC {
    fn id(&self) -> CDCID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBCDC {
    pub id: CDCID,
    pub connected: AtomicUsize,
}

impl SimpleUSBCDC {
    pub fn new(id: CDCID) -> Self {
        SimpleUSBCDC {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBCDC for SimpleUSBCDC {
    fn id(&self) -> CDCID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait CDCController {
    fn init(&mut self, cdc_id: CDCID) -> Result<(), CDCError>;
    fn send(&self, cdc_id: CDCID, data: &[u8]) -> Result<usize, CDCError>;
    def receive(&self, cdc_id: CDCID, buffer: &mut [u8]) -> Result<usize, CDCError>;
}

#[repr(C)]
pub struct SimpleCDCController {
    pub cdc_devices: Vec<Option<Box<dyn USBCDC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCDCController {
    pub fn new() -> Self {
        SimpleCDCController {
            cdc_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CDCController for SimpleCDCController {
    fn init(&mut self, cdc_id: CDCID) -> Result<(), CDCError> {
        for cdc_option in &mut self.cdc_devices {
            if let Some(ref mut cdc) = *cdc_option {
                if cdc.id() == cdc_id {
                    cdc.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(CDCError::NotFound)
    }
    
    fn send(&self, cdc_id: CDCID, _data: &[u8]) -> Result<usize, CDCError> {
        if self.get_cdc(cdc_id).is_some() {
            Ok(0)
        } else {
            Err(CDCError::NotFound)
        }
    }
    
    fn receive(&self, cdc_id: CDCID, buffer: &mut [u8]) -> Result<usize, CDCError> {
        if self.get_cdc(cdc_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(CDCError::NotFound)
        }
    }
    
    fn get_cdc(&self, id: CDCID) -> Option<&dyn USBCDC> {
        for cdc_option in &self.cdc_devices {
            if let Some(ref cdc) = *cdc_option {
                if cdc.id() == id { return Some(cdc.as_ref()); }
            }
        }
        None
    }
}

pub trait CDCLineCoding {
    def set_line_coding(&mut self, cdc_id: CDCID, baud: u32, bits: u8, parity: u8, stop: u8) -> Result<(), CDCError>;
    def get_line_coding(&self, cdc_id: CDCID) -> Result<(u32, u8, u8, u8), CDCError>;
}

#[repr(C)]
pub struct SimpleCDCLineCoding {
    pub controller: SimpleCDCController,
    pub line_codings: Vec<(CDCID, (AtomicUsize, AtomicUsize, AtomicUsize, AtomicUsize))>,
}

impl SimpleCDCLineCoding {
    pub fn new(controller: SimpleCDCController) -> Self {
        SimpleCDCLineCoding {
            controller,
            line_codings: Vec::new(),
        }
    }
}

impl CDCLineCoding for SimpleCDCLineCoding {
    fn set_line_coding(&mut self, cdc_id: CDCID, baud: u32, bits: u8, parity: u8, stop: u8) -> Result<(), CDCError> {
        self.line_codings.push((cdc_id, (AtomicUsize::new(baud as usize), AtomicUsize::new(bits as usize), AtomicUsize::new(parity as usize), AtomicUsize::new(stop as usize))));
        Ok(())
    }
    
    fn get_line_coding(&self, cdc_id: CDCID) -> Result<(u32, u8, u8, u8), CDCError> {
        for &(id, ref lc) in &self.line_codings {
            if id == cdc_id {
                return Ok((lc.0.load(Ordering::SeqCst) as u32, lc.1.load(Ordering::SeqCst) as u8, lc.2.load(Ordering::SeqCst) as u8, lc.3.load(Ordering::SeqCst) as u8));
            }
        }
        Err(CDCError::NotFound)
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
