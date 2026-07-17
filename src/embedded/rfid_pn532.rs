#![no_std]
#![no_main]

/// OOP-based PN532 NFC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3236
/// Implements PN532 NFC reader

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PN532ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PN532Error { Success = 0, NotFound = 1 }

pub trait PN532Reader {
    fn id(&self) -> PN532ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePN532Reader {
    pub id: PN532ID,
    pub initialized: AtomicUsize,
}

impl SimplePN532Reader {
    pub fn new(id: PN532ID) -> Self {
        SimplePN532Reader {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PN532Reader for SimplePN532Reader {
    fn id(&self) -> PN532ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PN532Controller {
    fn init(&mut self, reader_id: PN532ID) -> Result<(), PN532Error>;
    fn read_uid(&self, reader_id: PN532ID) -> Result<[u8; 7], PN532Error>;
    def write_data(&self, reader_id: PN532ID, data: &[u8]) -> Result<(), PN532Error>;
}

#[repr(C)]
pub struct SimplePN532Controller {
    pub readers: Vec<Option<Box<dyn PN532Reader>>>,
    pub next_id: AtomicUsize,
}

impl SimplePN532Controller {
    pub fn new() -> Self {
        SimplePN532Controller {
            readers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PN532Controller for SimplePN532Controller {
    fn init(&mut self, reader_id: PN532ID) -> Result<(), PN532Error> {
        for reader_option in &mut self.readers {
            if let Some(ref mut reader) = *reader_option {
                if reader.id() == reader_id {
                    reader.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PN532Error::NotFound)
    }
    
    fn read_uid(&self, reader_id: PN532ID) -> Result<[u8; 7], PN532Error> {
        if self.get_reader(reader_id).is_some() {
            Ok([0, 0, 0, 0, 0, 0, 0])
        } else {
            Err(PN532Error::NotFound)
        }
    }
    
    fn write_data(&self, reader_id: PN532ID, _data: &[u8]) -> Result<(), PN532Error> {
        if self.get_reader(reader_id).is_some() {
            Ok(())
        } else {
            Err(PN532Error::NotFound)
        }
    }
    
    fn get_reader(&self, id: PN532ID) -> Option<&dyn PN532Reader> {
        for reader_option in &self.readers {
            if let Some(ref reader) = *reader_option {
                if reader.id() == id { return Some(reader.as_ref()); }
            }
        }
        None
    }
}

pub trait PN532Mode {
    def set_mode(&mut self, reader_id: PN532ID, mode: u8) -> Result<(), PN532Error>;
    def get_mode(&self, reader_id: PN532ID) -> Result<u8, PN532Error>;
}

#[repr(C)]
pub struct SimplePN532Mode {
    pub controller: SimplePN532Controller,
    pub modes: Vec<(PN532ID, AtomicUsize)>,
}

impl SimplePN532Mode {
    pub fn new(controller: SimplePN532Controller) -> Self {
        SimplePN532Mode {
            controller,
            modes: Vec::new(),
        }
    }
}

impl PN532Mode for SimplePN532Mode {
    fn set_mode(&mut self, reader_id: PN532ID, mode: u8) -> Result<(), PN532Error> {
        self.modes.push((reader_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_mode(&self, reader_id: PN532ID) -> Result<u8, PN532Error> {
        for &(id, ref mode) in &self.modes {
            if id == reader_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(PN532Error::NotFound)
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
