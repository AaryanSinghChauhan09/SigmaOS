#![no_std]
#![no_main]

/// OOP-based RFID for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1676
/// Implements RFID reader

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RFIDID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RFIDError { Success = 0, NotFound = 1, NoTag = 2 }

pub trait RFIDReader {
    fn id(&self) -> RFIDID;
    fn is_present(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRFIDReader {
    pub id: RFIDID,
    pub tag_present: AtomicUsize,
}

impl SimpleRFIDReader {
    pub fn new(id: RFIDID) -> Self {
        SimpleRFIDReader {
            id,
            tag_present: AtomicUsize::new(0),
        }
    }
}

impl RFIDReader for SimpleRFIDReader {
    fn id(&self) -> RFIDID { self.id }
    fn is_present(&self) -> bool { self.tag_present.load(Ordering::SeqCst) == 1 }
}

pub trait RFIDController {
    fn read_uid(&self, rfid_id: RFIDID) -> Result<[u8; 10], RFIDError>;
    def write_block(&self, rfid_id: RFIDID, block: u8, data: &[u8]) -> Result<(), RFIDError>;
}

#[repr(C)]
pub struct SimpleRFIDController {
    pub readers: Vec<Option<Box<dyn RFIDReader>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRFIDController {
    pub fn new() -> Self {
        SimpleRFIDController {
            readers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RFIDController for SimpleRFIDController {
    fn read_uid(&self, rfid_id: RFIDID) -> Result<[u8; 10], RFIDError> {
        if self.get_reader(rfid_id).is_some() {
            Ok([0u8; 10])
        } else {
            Err(RFIDError::NotFound)
        }
    }
    
    fn write_block(&self, rfid_id: RFIDID, _block: u8, _data: &[u8]) -> Result<(), RFIDError> {
        if self.get_reader(rfid_id).is_some() {
            Ok(())
        } else {
            Err(RFIDError::NotFound)
        }
    }
    
    fn get_reader(&self, id: RFIDID) -> Option<&dyn RFIDReader> {
        for reader_option in &self.readers {
            if let Some(ref reader) = *reader_option {
                if reader.id() == id { return Some(reader.as_ref()); }
            }
        }
        None
    }
}

pub trait RFIDAuth {
    def authenticate(&self, rfid_id: RFIDID, key: &[u8]) -> Result<(), RFIDError>;
    def read_block(&self, rfid_id: RFIDID, block: u8) -> Result<[u8; 16], RFIDError>;
}

#[repr(C)]
pub struct SimpleRFIDAuth {
    pub controller: SimpleRFIDController,
}

impl SimpleRFIDAuth {
    pub fn new(controller: SimpleRFIDController) -> Self {
        SimpleRFIDAuth { controller }
    }
}

impl RFIDAuth for SimpleRFIDAuth {
    fn authenticate(&self, rfid_id: RFIDID, _key: &[u8]) -> Result<(), RFIDError> {
        if self.controller.get_reader(rfid_id).is_some() {
            Ok(())
        } else {
            Err(RFIDError::NotFound)
        }
    }
    
    fn read_block(&self, rfid_id: RFIDID, _block: u8) -> Result<[u8; 16], RFIDError> {
        if self.controller.get_reader(rfid_id).is_some() {
            Ok([0u8; 16])
        } else {
            Err(RFIDError::NotFound)
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
