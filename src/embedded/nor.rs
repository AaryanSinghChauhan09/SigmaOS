#![no_std]
#![no_main]

/// OOP-based NOR Flash for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2406
/// Implements NOR Flash

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NORID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NORError { Success = 0, NotFound = 1 }

pub trait NORFlash {
    fn id(&self) -> NORID;
    fn size_mb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleNORFlash {
    pub id: NORID,
    pub size_mb: AtomicUsize,
}

impl SimpleNORFlash {
    pub fn new(id: NORID, size_mb: u16) -> Self {
        SimpleNORFlash {
            id,
            size_mb: AtomicUsize::new(size_mb as usize),
        }
    }
}

impl NORFlash for SimpleNORFlash {
    fn id(&self) -> NORID { self.id }
    fn size_mb(&self) -> u16 { self.size_mb.load(Ordering::SeqCst) as u16 }
}

pub trait NORController {
    fn read(&self, nor_id: NORID, address: u32, buffer: &mut [u8]) -> Result<(), NORError>;
    fn write(&self, nor_id: NORID, address: u32, data: &[u8]) -> Result<(), NORError>;
    def erase(&mut self, nor_id: NORID, address: u32, len: u32) -> Result<(), NORError>;
}

#[repr(C)]
pub struct SimpleNORController {
    pub nors: Vec<Option<Box<dyn NORFlash>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNORController {
    pub fn new() -> Self {
        SimpleNORController {
            nors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NORController for SimpleNORController {
    fn read(&self, nor_id: NORID, _address: u32, buffer: &mut [u8]) -> Result<(), NORError> {
        if self.get_nor(nor_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(NORError::NotFound)
        }
    }
    
    fn write(&self, nor_id: NORID, _address: u32, _data: &[u8]) -> Result<(), NORError> {
        if self.get_nor(nor_id).is_some() {
            Ok(())
        } else {
            Err(NORError::NotFound)
        }
    }
    
    fn erase(&mut self, _nor_id: NORID, _address: u32, _len: u32) -> Result<(), NORError> {
        Ok(())
    }
    
    fn get_nor(&self, id: NORID) -> Option<&dyn NORFlash> {
        for nor_option in &self.nors {
            if let Some(ref nor) = *nor_option {
                if nor.id() == id { return Some(nor.as_ref()); }
            }
        }
        None
    }
}

pub trait NOREraseBlock {
    def erase_block(&mut self, nor_id: NORID, block: u16) -> Result<(), NORError>;
    def get_block_size(&self, nor_id: NORID) -> Result<u32, NORError>;
}

#[repr(C)]
pub struct SimpleNOREraseBlock {
    pub controller: SimpleNORController,
    pub block_sizes: Vec<(NORID, AtomicUsize)>,
}

impl SimpleNOREraseBlock {
    pub fn new(controller: SimpleNORController) -> Self {
        SimpleNOREraseBlock {
            controller,
            block_sizes: Vec::new(),
        }
    }
}

impl NOREraseBlock for SimpleNOREraseBlock {
    fn erase_block(&mut self, _nor_id: NORID, _block: u16) -> Result<(), NORError> {
        Ok(())
    }
    
    fn get_block_size(&self, nor_id: NORID) -> Result<u32, NORError> {
        for &(id, ref size) in &self.block_sizes {
            if id == nor_id {
                return Ok(size.load(Ordering::SeqCst) as u32);
            }
        }
        Err(NORError::NotFound)
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
