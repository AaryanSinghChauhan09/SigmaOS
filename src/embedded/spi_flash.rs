#![no_std]
#![no_main]

/// OOP-based SPI Flash for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2376
/// Implements SPI Flash memory

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FlashID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FlashError { Success = 0, NotFound = 1 }

pub trait SPIFlash {
    fn id(&self) -> FlashID;
    fn size_kb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleSPIFlash {
    pub id: FlashID,
    pub size_kb: AtomicUsize,
}

impl SimpleSPIFlash {
    pub fn new(id: FlashID, size_kb: u16) -> Self {
        SimpleSPIFlash {
            id,
            size_kb: AtomicUsize::new(size_kb as usize),
        }
    }
}

impl SPIFlash for SimpleSPIFlash {
    fn id(&self) -> FlashID { self.id }
    fn size_kb(&self) -> u16 { self.size_kb.load(Ordering::SeqCst) as u16 }
}

pub trait FlashController {
    fn read(&self, flash_id: FlashID, address: u32, buffer: &mut [u8]) -> Result<(), FlashError>;
    fn write(&self, flash_id: FlashID, address: u32, data: &[u8]) -> Result<(), FlashError>;
    def erase(&mut self, flash_id: FlashID, address: u32, len: u32) -> Result<(), FlashError>;
}

#[repr(C)]
pub struct SimpleFlashController {
    pub flashes: Vec<Option<Box<dyn SPIFlash>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFlashController {
    pub fn new() -> Self {
        SimpleFlashController {
            flashes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FlashController for SimpleFlashController {
    fn read(&self, flash_id: FlashID, _address: u32, buffer: &mut [u8]) -> Result<(), FlashError> {
        if self.get_flash(flash_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(FlashError::NotFound)
        }
    }
    
    fn write(&self, flash_id: FlashID, _address: u32, _data: &[u8]) -> Result<(), FlashError> {
        if self.get_flash(flash_id).is_some() {
            Ok(())
        } else {
            Err(FlashError::NotFound)
        }
    }
    
    fn erase(&mut self, _flash_id: FlashID, _address: u32, _len: u32) -> Result<(), FlashError> {
        Ok(())
    }
    
    fn get_flash(&self, id: FlashID) -> Option<&dyn SPIFlash> {
        for flash_option in &self.flashes {
            if let Some(ref flash) = *flash_option {
                if flash.id() == id { return Some(flash.as_ref()); }
            }
        }
        None
    }
}

pub trait FlashSector {
    def erase_sector(&mut self, flash_id: FlashID, sector: u16) -> Result<(), FlashError>;
    def get_sector_size(&self, flash_id: FlashID) -> Result<u16, FlashError>;
}

#[repr(C)]
pub struct SimpleFlashSector {
    pub controller: SimpleFlashController,
    pub sector_sizes: Vec<(FlashID, AtomicUsize)>,
}

impl SimpleFlashSector {
    pub fn new(controller: SimpleFlashController) -> Self {
        SimpleFlashSector {
            controller,
            sector_sizes: Vec::new(),
        }
    }
}

impl FlashSector for SimpleFlashSector {
    fn erase_sector(&mut self, _flash_id: FlashID, _sector: u16) -> Result<(), FlashError> {
        Ok(())
    }
    
    fn get_sector_size(&self, flash_id: FlashID) -> Result<u16, FlashError> {
        for &(id, ref size) in &self.sector_sizes {
            if id == flash_id {
                return Ok(size.load(Ordering::SeqCst) as u16);
            }
        }
        Err(FlashError::NotFound)
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
