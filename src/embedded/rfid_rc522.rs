#![no_std]
#![no_main]

/// OOP-based RC522 RFID for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3226
/// Implements RC522 RFID reader

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RC522ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RC522Error { Success = 0, NotFound = 1 }

pub trait RC522Reader {
    fn id(&self) -> RC522ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRC522Reader {
    pub id: RC522ID,
    pub initialized: AtomicUsize,
}

impl SimpleRC522Reader {
    pub fn new(id: RC522ID) -> Self {
        SimpleRC522Reader {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl RC522Reader for SimpleRC522Reader {
    fn id(&self) -> RC522ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait RC522Controller {
    fn init(&mut self, reader_id: RC522ID) -> Result<(), RC522Error>;
    fn read_uid(&self, reader_id: RC522ID) -> Result<[u8; 4], RC522Error>;
    def write_block(&self, reader_id: RC522ID, block: u8, data: &[u8]) -> Result<(), RC522Error>;
}

#[repr(C)]
pub struct SimpleRC522Controller {
    pub readers: Vec<Option<Box<dyn RC522Reader>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRC522Controller {
    pub fn new() -> Self {
        SimpleRC522Controller {
            readers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RC522Controller for SimpleRC522Controller {
    fn init(&mut self, reader_id: RC522ID) -> Result<(), RC522Error> {
        for reader_option in &mut self.readers {
            if let Some(ref mut reader) = *reader_option {
                if reader.id() == reader_id {
                    reader.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RC522Error::NotFound)
    }
    
    fn read_uid(&self, reader_id: RC522ID) -> Result<[u8; 4], RC522Error> {
        if self.get_reader(reader_id).is_some() {
            Ok([0, 0, 0, 0])
        } else {
            Err(RC522Error::NotFound)
        }
    }
    
    fn write_block(&self, reader_id: RC522ID, _block: u8, _data: &[u8]) -> Result<(), RC522Error> {
        if self.get_reader(reader_id).is_some() {
            Ok(())
        } else {
            Err(RC522Error::NotFound)
        }
    }
    
    fn get_reader(&self, id: RC522ID) -> Option<&dyn RC522Reader> {
        for reader_option in &self.readers {
            if let Some(ref reader) = *reader_option {
                if reader.id() == id { return Some(reader.as_ref()); }
            }
        }
        None
    }
}

pub trait RC522Auth {
    def authenticate(&mut self, reader_id: RC522ID, key: &[u8]) -> Result<(), RC522Error>;
}

#[repr(C)]
pub struct SimpleRC522Auth {
    pub controller: SimpleRC522Controller,
}

impl SimpleRC522Auth {
    pub fn new(controller: SimpleRC522Controller) -> Self {
        SimpleRC522Auth { controller }
    }
}

impl RC522Auth for SimpleRC522Auth {
    fn authenticate(&mut self, reader_id: RC522ID, _key: &[u8]) -> Result<(), RC522Error> {
        if self.controller.get_reader(reader_id).is_some() {
            Ok(())
        } else {
            Err(RC522Error::NotFound)
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
