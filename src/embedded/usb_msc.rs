#![no_std]
#![no_main]

/// OOP-based USB MSC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2216
/// Implements USB MSC (Mass Storage Class)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MSCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MSCError { Success = 0, NotFound = 1 }

pub trait USBMSC {
    fn id(&self) -> MSCID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBMSC {
    pub id: MSCID,
    pub connected: AtomicUsize,
}

impl SimpleUSBMSC {
    pub fn new(id: MSCID) -> Self {
        SimpleUSBMSC {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBMSC for SimpleUSBMSC {
    fn id(&self) -> MSCID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait MSCController {
    fn init(&mut self, msc_id: MSCID) -> Result<(), MSCError>;
    def read_block(&self, msc_id: MSCID, lba: u32, buffer: &mut [u8]) -> Result<(), MSCError>;
    def write_block(&self, msc_id: MSCID, lba: u32, buffer: &[u8]) -> Result<(), MSCError>;
}

#[repr(C)]
pub struct SimpleMSCController {
    pub msc_devices: Vec<Option<Box<dyn USBMSC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMSCController {
    pub fn new() -> Self {
        SimpleMSCController {
            msc_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MSCController for SimpleMSCController {
    fn init(&mut self, msc_id: MSCID) -> Result<(), MSCError> {
        for msc_option in &mut self.msc_devices {
            if let Some(ref mut msc) = *msc_option {
                if msc.id() == msc_id {
                    msc.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MSCError::NotFound)
    }
    
    fn read_block(&self, msc_id: MSCID, _lba: u32, buffer: &mut [u8]) -> Result<(), MSCError> {
        if self.get_msc(msc_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(MSCError::NotFound)
        }
    }
    
    fn write_block(&self, msc_id: MSCID, _lba: u32, _buffer: &[u8]) -> Result<(), MSCError> {
        if self.get_msc(msc_id).is_some() {
            Ok(())
        } else {
            Err(MSCError::NotFound)
        }
    }
    
    fn get_msc(&self, id: MSCID) -> Option<&dyn USBMSC> {
        for msc_option in &self.msc_devices {
            if let Some(ref msc) = *msc_option {
                if msc.id() == id { return Some(msc.as_ref()); }
            }
        }
        None
    }
}

pub trait MSCCapacity {
    def get_capacity(&self, msc_id: MSCID) -> Result<u32, MSCError>;
    def get_block_size(&self, msc_id: MSCID) -> Result<u32, MSCError>;
}

#[repr(C)]
pub struct SimpleMSCCapacity {
    pub controller: SimpleMSCController,
    pub capacities: Vec<(MSCID, AtomicUsize)>,
}

impl SimpleMSCCapacity {
    pub fn new(controller: SimpleMSCController) -> Self {
        SimpleMSCCapacity {
            controller,
            capacities: Vec::new(),
        }
    }
}

impl MSCCapacity for SimpleMSCCapacity {
    fn get_capacity(&self, msc_id: MSCID) -> Result<u32, MSCError> {
        for &(id, ref cap) in &self.capacities {
            if id == msc_id {
                return Ok(cap.load(Ordering::SeqCst) as u32);
            }
        }
        Err(MSCError::NotFound)
    }
    
    fn get_block_size(&self, msc_id: MSCID) -> Result<u32, MSCError> {
        if self.controller.get_msc(msc_id).is_some() {
            Ok(512)
        } else {
            Err(MSCError::NotFound)
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
