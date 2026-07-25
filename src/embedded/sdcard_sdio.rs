#![no_std]
#![no_main]

/// OOP-based SD Card SDIO for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2976
/// Implements SD Card over SDIO

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SDCardSDIOID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SDCardSDIOError { Success = 0, NotFound = 1 }

pub trait SDCardSDIO {
    fn id(&self) -> SDCardSDIOID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSDCardSDIO {
    pub id: SDCardSDIOID,
    pub initialized: AtomicUsize,
}

impl SimpleSDCardSDIO {
    pub fn new(id: SDCardSDIOID) -> Self {
        SimpleSDCardSDIO {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SDCardSDIO for SimpleSDCardSDIO {
    fn id(&self) -> SDCardSDIOID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SDCardSDIOController {
    fn init(&mut self, sd_id: SDCardSDIOID) -> Result<(), SDCardSDIOError>;
    fn read_block(&self, sd_id: SDCardSDIOID, block: u32, buffer: &mut [u8]) -> Result<(), SDCardSDIOError>;
    def write_block(&self, sd_id: SDCardSDIOID, block: u32, buffer: &[u8]) -> Result<(), SDCardSDIOError>;
}

#[repr(C)]
pub struct SimpleSDCardSDIOController {
    pub cards: Vec<Option<Box<dyn SDCardSDIO>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSDCardSDIOController {
    pub fn new() -> Self {
        SimpleSDCardSDIOController {
            cards: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SDCardSDIOController for SimpleSDCardSDIOController {
    fn init(&mut self, sd_id: SDCardSDIOID) -> Result<(), SDCardSDIOError> {
        for card_option in &mut self.cards {
            if let Some(ref mut card) = *card_option {
                if card.id() == sd_id {
                    card.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SDCardSDIOError::NotFound)
    }
    
    fn read_block(&self, sd_id: SDCardSDIOID, _block: u32, buffer: &mut [u8]) -> Result<(), SDCardSDIOError> {
        if self.get_card(sd_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(SDCardSDIOError::NotFound)
        }
    }
    
    fn write_block(&self, sd_id: SDCardSDIOID, _block: u32, _buffer: &[u8]) -> Result<(), SDCardSDIOError> {
        if self.get_card(sd_id).is_some() {
            Ok(())
        } else {
            Err(SDCardSDIOError::NotFound)
        }
    }
    
    fn get_card(&self, id: SDCardSDIOID) -> Option<&dyn SDCardSDIO> {
        for card_option in &self.cards {
            if let Some(ref card) = *card_option {
                if card.id() == id { return Some(card.as_ref()); }
            }
        }
        None
    }
}

pub trait SDCardSDIOInfo {
    def read_cid(&self, sd_id: SDCardSDIOID) -> Result<[u8; 16], SDCardSDIOError>;
    def read_csd(&self, sd_id: SDCardSDIOID) -> Result<[u8; 16], SDCardSDIOError>;
}

#[repr(C)]
pub struct SimpleSDCardSDIOInfo {
    pub controller: SimpleSDCardSDIOController,
}

impl SimpleSDCardSDIOInfo {
    pub fn new(controller: SimpleSDCardSDIOController) -> Self {
        SimpleSDCardSDIOInfo { controller }
    }
}

impl SDCardSDIOInfo for SimpleSDCardSDIOInfo {
    fn read_cid(&self, sd_id: SDCardSDIOID) -> Result<[u8; 16], SDCardSDIOError> {
        if self.controller.get_card(sd_id).is_some() {
            Ok([0u8; 16])
        } else {
            Err(SDCardSDIOError::NotFound)
        }
    }
    
    fn read_csd(&self, sd_id: SDCardSDIOID) -> Result<[u8; 16], SDCardSDIOError> {
        if self.controller.get_card(sd_id).is_some() {
            Ok([0u8; 16])
        } else {
            Err(SDCardSDIOError::NotFound)
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
