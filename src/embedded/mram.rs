#![no_std]
#![no_main]

/// OOP-based MRAM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2396
/// Implements MRAM (Magnetoresistive RAM)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MRAMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MRAMError { Success = 0, NotFound = 1 }

pub trait MRAMDevice {
    fn id(&self) -> MRAMID;
    fn size_kb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleMRAMDevice {
    pub id: MRAMID,
    pub size_kb: AtomicUsize,
}

impl SimpleMRAMDevice {
    pub fn new(id: MRAMID, size_kb: u16) -> Self {
        SimpleMRAMDevice {
            id,
            size_kb: AtomicUsize::new(size_kb as usize),
        }
    }
}

impl MRAMDevice for SimpleMRAMDevice {
    fn id(&self) -> MRAMID { self.id }
    fn size_kb(&self) -> u16 { self.size_kb.load(Ordering::SeqCst) as u16 }
}

pub trait MRAMController {
    fn read(&self, mram_id: MRAMID, address: u32, buffer: &mut [u8]) -> Result<(), MRAMError>;
    fn write(&self, mram_id: MRAMID, address: u32, data: &[u8]) -> Result<(), MRAMError>;
}

#[repr(C)]
pub struct SimpleMRAMController {
    pub mrams: Vec<Option<Box<dyn MRAMDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMRAMController {
    pub fn new() -> Self {
        SimpleMRAMController {
            mrams: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MRAMController for SimpleMRAMController {
    fn read(&self, mram_id: MRAMID, _address: u32, buffer: &mut [u8]) -> Result<(), MRAMError> {
        if self.get_mram(mram_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(MRAMError::NotFound)
        }
    }
    
    fn write(&self, mram_id: MRAMID, _address: u32, _data: &[u8]) -> Result<(), MRAMError> {
        if self.get_mram(mram_id).is_some() {
            Ok(())
        } else {
            Err(MRAMError::NotFound)
        }
    }
    
    fn get_mram(&self, id: MRAMID) -> Option<&dyn MRAMDevice> {
        for mram_option in &self.mrams {
            if let Some(ref mram) = *mram_option {
                if mram.id() == id { return Some(mram.as_ref()); }
            }
        }
        None
    }
}

pub trait MRAMBuffer {
    def set_buffer_mode(&mut self, mram_id: MRAMID, buffered: bool) -> Result<(), MRAMError>;
    def flush(&mut self, mram_id: MRAMID) -> Result<(), MRAMError>;
}

#[repr(C)]
pub struct SimpleMRAMBuffer {
    pub controller: SimpleMRAMController,
    pub buffer_modes: Vec<(MRAMID, AtomicUsize)>,
}

impl SimpleMRAMBuffer {
    pub fn new(controller: SimpleMRAMController) -> Self {
        SimpleMRAMBuffer {
            controller,
            buffer_modes: Vec::new(),
        }
    }
}

impl MRAMBuffer for SimpleMRAMBuffer {
    fn set_buffer_mode(&mut self, mram_id: MRAMID, buffered: bool) -> Result<(), MRAMError> {
        self.buffer_modes.push((mram_id, AtomicUsize::new(if buffered { 1 } else { 0 })));
        Ok(())
    }
    
    fn flush(&mut self, _mram_id: MRAMID) -> Result<(), MRAMError> {
        Ok(())
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
