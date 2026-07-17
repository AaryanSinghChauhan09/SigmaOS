#![no_std]
#![no_main]

/// OOP-based W25Q64 Flash for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3726
/// Implements W25Q64 SPI flash memory

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type W25Q64ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum W25Q64Error { Success = 0, NotFound = 1 }

pub trait W25Q64Device {
    fn id(&self) -> W25Q64ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleW25Q64Device {
    pub id: W25Q64ID,
    pub initialized: AtomicUsize,
}

impl SimpleW25Q64Device {
    pub fn new(id: W25Q64ID) -> Self {
        SimpleW25Q64Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl W25Q64Device for SimpleW25Q64Device {
    fn id(&self) -> W25Q64ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait W25Q64Controller {
    fn init(&mut self, flash_id: W25Q64ID) -> Result<(), W25Q64Error>;
    fn read(&self, flash_id: W25Q64ID, addr: u32, buffer: &mut [u8]) -> Result<(), W25Q64Error>;
    def write(&self, flash_id: W25Q64ID, addr: u32, data: &[u8]) -> Result<(), W25Q64Error>;
}

#[repr(C)]
pub struct SimpleW25Q64Controller {
    pub flashes: Vec<Option<Box<dyn W25Q64Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleW25Q64Controller {
    pub fn new() -> Self {
        SimpleW25Q64Controller {
            flashes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl W25Q64Controller for SimpleW25Q64Controller {
    fn init(&mut self, flash_id: W25Q64ID) -> Result<(), W25Q64Error> {
        for flash_option in &mut self.flashes {
            if let Some(ref mut flash) = *flash_option {
                if flash.id() == flash_id {
                    flash.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(W25Q64Error::NotFound)
    }
    
    fn read(&self, flash_id: W25Q64ID, _addr: u32, buffer: &mut [u8]) -> Result<(), W25Q64Error> {
        if self.get_flash(flash_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(W25Q64Error::NotFound)
        }
    }
    
    fn write(&self, flash_id: W25Q64ID, _addr: u32, _data: &[u8]) -> Result<(), W25Q64Error> {
        if self.get_flash(flash_id).is_some() {
            Ok(())
        } else {
            Err(W25Q64Error::NotFound)
        }
    }
    
    fn get_flash(&self, id: W25Q64ID) -> Option<&dyn W25Q64Device> {
        for flash_option in &self.flashes {
            if let Some(ref flash) = *flash_option {
                if flash.id() == id { return Some(flash.as_ref()); }
            }
        }
        None
    }
}

pub trait W25Q64Erase {
    def erase_sector(&self, flash_id: W25Q64ID, addr: u32) -> Result<(), W25Q64Error>;
    def erase_chip(&self, flash_id: W25Q64ID) -> Result<(), W25Q64Error>;
}

#[repr(C)]
pub struct SimpleW25Q64Erase {
    pub controller: SimpleW25Q64Controller,
}

impl SimpleW25Q64Erase {
    pub fn new(controller: SimpleW25Q64Controller) -> Self {
        SimpleW25Q64Erase { controller }
    }
}

impl W25Q64Erase for SimpleW25Q64Erase {
    fn erase_sector(&self, flash_id: W25Q64ID, _addr: u32) -> Result<(), W25Q64Error> {
        if self.controller.get_flash(flash_id).is_some() {
            Ok(())
        } else {
            Err(W25Q64Error::NotFound)
        }
    }
    
    fn erase_chip(&self, flash_id: W25Q64ID) -> Result<(), W25Q64Error> {
        if self.controller.get_flash(flash_id).is_some() {
            Ok(())
        } else {
            Err(W25Q64Error::NotFound)
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
