#![no_std]
#![no_main]

/// OOP-based AT25SF041 Flash for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3736
/// Implements AT25SF041 SPI flash memory

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AT25SF041ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AT25SF041Error { Success = 0, NotFound = 1 }

pub trait AT25SF041Device {
    fn id(&self) -> AT25SF041ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleAT25SF041Device {
    pub id: AT25SF041ID,
    pub initialized: AtomicUsize,
}

impl SimpleAT25SF041Device {
    pub fn new(id: AT25SF041ID) -> Self {
        SimpleAT25SF041Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl AT25SF041Device for SimpleAT25SF041Device {
    fn id(&self) -> AT25SF041ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait AT25SF041Controller {
    fn init(&mut self, flash_id: AT25SF041ID) -> Result<(), AT25SF041Error>;
    fn read(&self, flash_id: AT25SF041ID, addr: u32, buffer: &mut [u8]) -> Result<(), AT25SF041Error>;
    def write(&self, flash_id: AT25SF041ID, addr: u32, data: &[u8]) -> Result<(), AT25SF041Error>;
}

#[repr(C)]
pub struct SimpleAT25SF041Controller {
    pub flashes: Vec<Option<Box<dyn AT25SF041Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAT25SF041Controller {
    pub fn new() -> Self {
        SimpleAT25SF041Controller {
            flashes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AT25SF041Controller for SimpleAT25SF041Controller {
    fn init(&mut self, flash_id: AT25SF041ID) -> Result<(), AT25SF041Error> {
        for flash_option in &mut self.flashes {
            if let Some(ref mut flash) = *flash_option {
                if flash.id() == flash_id {
                    flash.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(AT25SF041Error::NotFound)
    }
    
    fn read(&self, flash_id: AT25SF041ID, _addr: u32, buffer: &mut [u8]) -> Result<(), AT25SF041Error> {
        if self.get_flash(flash_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(AT25SF041Error::NotFound)
        }
    }
    
    fn write(&self, flash_id: AT25SF041ID, _addr: u32, _data: &[u8]) -> Result<(), AT25SF041Error> {
        if self.get_flash(flash_id).is_some() {
            Ok(())
        } else {
            Err(AT25SF041Error::NotFound)
        }
    }
    
    fn get_flash(&self, id: AT25SF041ID) -> Option<&dyn AT25SF041Device> {
        for flash_option in &self.flashes {
            if let Some(ref flash) = *flash_option {
                if flash.id() == id { return Some(flash.as_ref()); }
            }
        }
        None
    }
}

pub trait AT25SF041Status {
    def read_status(&self, flash_id: AT25SF041ID) -> Result<u8, AT25SF041Error>;
}

#[repr(C)]
pub struct SimpleAT25SF041Status {
    pub controller: SimpleAT25SF041Controller,
}

impl SimpleAT25SF041Status {
    pub fn new(controller: SimpleAT25SF041Controller) -> Self {
        SimpleAT25SF041Status { controller }
    }
}

impl AT25SF041Status for SimpleAT25SF041Status {
    fn read_status(&self, flash_id: AT25SF041ID) -> Result<u8, AT25SF041Error> {
        if self.controller.get_flash(flash_id).is_some() {
            Ok(0)
        } else {
            Err(AT25SF041Error::NotFound)
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
