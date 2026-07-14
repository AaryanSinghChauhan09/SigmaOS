#![no_std]
#![no_main]

/// OOP-based SD Card SPI for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2966
/// Implements SD Card over SPI

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SDCardSPIID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SDCardSPIError { Success = 0, NotFound = 1 }

pub trait SDCardSPI {
    fn id(&self) -> SDCardSPIID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSDCardSPI {
    pub id: SDCardSPIID,
    pub initialized: AtomicUsize,
}

impl SimpleSDCardSPI {
    pub fn new(id: SDCardSPIID) -> Self {
        SimpleSDCardSPI {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SDCardSPI for SimpleSDCardSPI {
    fn id(&self) -> SDCardSPIID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SDCardSPIController {
    fn init(&mut self, sd_id: SDCardSPIID) -> Result<(), SDCardSPIError>;
    fn read_block(&self, sd_id: SDCardSPIID, block: u32, buffer: &mut [u8]) -> Result<(), SDCardSPIError>;
    def write_block(&self, sd_id: SDCardSPIID, block: u32, buffer: &[u8]) -> Result<(), SDCardSPIError>;
}

#[repr(C)]
pub struct SimpleSDCardSPIController {
    pub cards: Vec<Option<Box<dyn SDCardSPI>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSDCardSPIController {
    pub fn new() -> Self {
        SimpleSDCardSPIController {
            cards: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SDCardSPIController for SimpleSDCardSPIController {
    fn init(&mut self, sd_id: SDCardSPIID) -> Result<(), SDCardSPIError> {
        for card_option in &mut self.cards {
            if let Some(ref mut card) = *card_option {
                if card.id() == sd_id {
                    card.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SDCardSPIError::NotFound)
    }
    
    fn read_block(&self, sd_id: SDCardSPIID, _block: u32, buffer: &mut [u8]) -> Result<(), SDCardSPIError> {
        if self.get_card(sd_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(SDCardSPIError::NotFound)
        }
    }
    
    fn write_block(&self, sd_id: SDCardSPIID, _block: u32, _buffer: &[u8]) -> Result<(), SDCardSPIError> {
        if self.get_card(sd_id).is_some() {
            Ok(())
        } else {
            Err(SDCardSPIError::NotFound)
        }
    }
    
    fn get_card(&self, id: SDCardSPIID) -> Option<&dyn SDCardSPI> {
        for card_option in &self.cards {
            if let Some(ref card) = *card_option {
                if card.id() == id { return Some(card.as_ref()); }
            }
        }
        None
    }
}

pub trait SDCardSPIInfo {
    def read_cid(&self, sd_id: SDCardSPIID) -> Result<[u8; 16], SDCardSPIError>;
    def read_csd(&self, sd_id: SDCardSPIID) -> Result<[u8; 16], SDCardSPIError>;
}

#[repr(C)]
pub struct SimpleSDCardSPIInfo {
    pub controller: SimpleSDCardSPIController,
}

impl SimpleSDCardSPIInfo {
    pub fn new(controller: SimpleSDCardSPIController) -> Self {
        SimpleSDCardSPIInfo { controller }
    }
}

impl SDCardSPIInfo for SimpleSDCardSPIInfo {
    fn read_cid(&self, sd_id: SDCardSPIID) -> Result<[u8; 16], SDCardSPIError> {
        if self.controller.get_card(sd_id).is_some() {
            Ok([0u8; 16])
        } else {
            Err(SDCardSPIError::NotFound)
        }
    }
    
    fn read_csd(&self, sd_id: SDCardSPIID) -> Result<[u8; 16], SDCardSPIError> {
        if self.controller.get_card(sd_id).is_some() {
            Ok([0u8; 16])
        } else {
            Err(SDCardSPIError::NotFound)
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
