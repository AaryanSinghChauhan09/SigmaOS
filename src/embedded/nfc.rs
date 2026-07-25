#![no_std]
#![no_main]

/// OOP-based NFC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1686
/// Implements NFC reader

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NFCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NFCError { Success = 0, NotFound = 1, NoTag = 2 }

pub trait NFCReader {
    fn id(&self) -> NFCID;
    fn is_present(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNFCReader {
    pub id: NFCID,
    pub tag_present: AtomicUsize,
}

impl SimpleNFCReader {
    pub fn new(id: NFCID) -> Self {
        SimpleNFCReader {
            id,
            tag_present: AtomicUsize::new(0),
        }
    }
}

impl NFCReader for SimpleNFCReader {
    fn id(&self) -> NFCID { self.id }
    fn is_present(&self) -> bool { self.tag_present.load(Ordering::SeqCst) == 1 }
}

pub trait NFCController {
    fn read_ndef(&self, nfc_id: NFCID, buffer: &mut [u8]) -> Result<usize, NFCError>;
    def write_ndef(&self, nfc_id: NFCID, data: &[u8]) -> Result<(), NFCError>;
}

#[repr(C)]
pub struct SimpleNFCController {
    pub readers: Vec<Option<Box<dyn NFCReader>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNFCController {
    pub fn new() -> Self {
        SimpleNFCController {
            readers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NFCController for SimpleNFCController {
    fn read_ndef(&self, nfc_id: NFCID, buffer: &mut [u8]) -> Result<usize, NFCError> {
        if self.get_reader(nfc_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(NFCError::NotFound)
        }
    }
    
    fn write_ndef(&self, nfc_id: NFCID, _data: &[u8]) -> Result<(), NFCError> {
        if self.get_reader(nfc_id).is_some() {
            Ok(())
        } else {
            Err(NFCError::NotFound)
        }
    }
    
    fn get_reader(&self, id: NFCID) -> Option<&dyn NFCReader> {
        for reader_option in &self.readers {
            if let Some(ref reader) = *reader_option {
                if reader.id() == id { return Some(reader.as_ref()); }
            }
        }
        None
    }
}

pub trait NFCTag {
    def format_tag(&self, nfc_id: NFCID) -> Result<(), NFCError>;
    def lock_tag(&self, nfc_id: NFCID) -> Result<(), NFCError>;
}

#[repr(C)]
pub struct SimpleNFCTag {
    pub controller: SimpleNFCController,
}

impl SimpleNFCTag {
    pub fn new(controller: SimpleNFCController) -> Self {
        SimpleNFCTag { controller }
    }
}

impl NFCTag for SimpleNFCTag {
    fn format_tag(&self, nfc_id: NFCID) -> Result<(), NFCError> {
        if self.controller.get_reader(nfc_id).is_some() {
            Ok(())
        } else {
            Err(NFCError::NotFound)
        }
    }
    
    fn lock_tag(&self, nfc_id: NFCID) -> Result<(), NFCError> {
        if self.controller.get_reader(nfc_id).is_some() {
            Ok(())
        } else {
            Err(NFCError::NotFound)
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
