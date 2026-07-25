#![no_std]
#![no_main]

/// OOP-based SEGGER RTT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2186
/// Implements SEGGER RTT

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SEGGERID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SEGGERError { Success = 0, NotFound = 1 }

pub trait SEGGERChannel {
    fn id(&self) -> SEGGERID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSEGGERChannel {
    pub id: SEGGERID,
    pub initialized: AtomicUsize,
}

impl SimpleSEGGERChannel {
    pub fn new(id: SEGGERID) -> Self {
        SimpleSEGGERChannel {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SEGGERChannel for SimpleSEGGERChannel {
    fn id(&self) -> SEGGERID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SEGGERController {
    fn init(&mut self, segger_id: SEGGERID) -> Result<(), SEGGERError>;
    fn write(&self, segger_id: SEGGERID, buffer: &[u8]) -> Result<usize, SEGGERError>;
    def read(&self, segger_id: SEGGERID, buffer: &mut [u8]) -> Result<usize, SEGGERError>;
}

#[repr(C)]
pub struct SimpleSEGGERController {
    pub channels: Vec<Option<Box<dyn SEGGERChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSEGGERController {
    pub fn new() -> Self {
        SimpleSEGGERController {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SEGGERController for SimpleSEGGERController {
    fn init(&mut self, segger_id: SEGGERID) -> Result<(), SEGGERError> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == segger_id {
                    channel.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SEGGERError::NotFound)
    }
    
    fn write(&self, segger_id: SEGGERID, _buffer: &[u8]) -> Result<usize, SEGGERError> {
        if self.get_channel(segger_id).is_some() {
            Ok(0)
        } else {
            Err(SEGGERError::NotFound)
        }
    }
    
    fn read(&self, segger_id: SEGGERID, buffer: &mut [u8]) -> Result<usize, SEGGERError> {
        if self.get_channel(segger_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(SEGGERError::NotFound)
        }
    }
    
    fn get_channel(&self, id: SEGGERID) -> Option<&dyn SEGGERChannel> {
        for channel_option in &self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == id { return Some(channel.as_ref()); }
            }
        }
        None
    }
}

pub trait SEGGERTerminal {
    def write_string(&self, segger_id: SEGGERID, s: &[u8]) -> Result<usize, SEGGERError>;
    def write_hex(&self, segger_id: SEGGERID, value: u32) -> Result<usize, SEGGERError>;
}

#[repr(C)]
pub struct SimpleSEGGERTerminal {
    pub controller: SimpleSEGGERController,
}

impl SimpleSEGGERTerminal {
    pub fn new(controller: SimpleSEGGERController) -> Self {
        SimpleSEGGERTerminal { controller }
    }
}

impl SEGGERTerminal for SimpleSEGGERTerminal {
    fn write_string(&self, segger_id: SEGGERID, _s: &[u8]) -> Result<usize, SEGGERError> {
        if self.controller.get_channel(segger_id).is_some() {
            Ok(0)
        } else {
            Err(SEGGERError::NotFound)
        }
    }
    
    fn write_hex(&self, segger_id: SEGGERID, _value: u32) -> Result<usize, SEGGERError> {
        if self.controller.get_channel(segger_id).is_some() {
            Ok(0)
        } else {
            Err(SEGGERError::NotFound)
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
