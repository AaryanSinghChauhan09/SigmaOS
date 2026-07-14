#![no_std]
#![no_main]

/// OOP-based NEO-6M GPS for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3096
/// Implements NEO-6M GPS receiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NEO6MID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NEO6MError { Success = 0, NotFound = 1 }

pub trait NEO6MGPS {
    fn id(&self) -> NEO6MID;
    fn has_fix(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNEO6MGPS {
    pub id: NEO6MID,
    pub fix: AtomicUsize,
}

impl SimpleNEO6MGPS {
    pub fn new(id: NEO6MID) -> Self {
        SimpleNEO6MGPS {
            id,
            fix: AtomicUsize::new(0),
        }
    }
}

impl NEO6MGPS for SimpleNEO6MGPS {
    fn id(&self) -> NEO6MID { self.id }
    fn has_fix(&self) -> bool { self.fix.load(Ordering::SeqCst) == 1 }
}

pub trait NEO6MController {
    fn init(&mut self, gps_id: NEO6MID) -> Result<(), NEO6MError>;
    fn read_nmea(&self, gps_id: NEO6MID, buffer: &mut [u8]) -> Result<usize, NEO6MError>;
    def get_position(&self, gps_id: NEO6MID) -> Result<(f32, f32), NEO6MError>;
}

#[repr(C)]
pub struct SimpleNEO6MController {
    pub gps_units: Vec<Option<Box<dyn NEO6MGPS>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNEO6MController {
    pub fn new() -> Self {
        SimpleNEO6MController {
            gps_units: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NEO6MController for SimpleNEO6MController {
    fn init(&mut self, gps_id: NEO6MID) -> Result<(), NEO6MError> {
        for gps_option in &mut self.gps_units {
            if let Some(ref mut gps) = *gps_option {
                if gps.id() == gps_id {
                    gps.fix.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(NEO6MError::NotFound)
    }
    
    fn read_nmea(&self, gps_id: NEO6MID, buffer: &mut [u8]) -> Result<usize, NEO6MError> {
        if self.get_gps(gps_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(NEO6MError::NotFound)
        }
    }
    
    fn get_position(&self, gps_id: NEO6MID) -> Result<(f32, f32), NEO6MError> {
        if self.get_gps(gps_id).is_some() {
            Ok((0.0, 0.0))
        } else {
            Err(NEO6MError::NotFound)
        }
    }
    
    fn get_gps(&self, id: NEO6MID) -> Option<&dyn NEO6MGPS> {
        for gps_option in &self.gps_units {
            if let Some(ref gps) = *gps_option {
                if gps.id() == id { return Some(gps.as_ref()); }
            }
        }
        None
    }
}

pub trait NEO6MConfig {
    def set_baudrate(&mut self, gps_id: NEO6MID, baud: u32) -> Result<(), NEO6MError>;
    def set_update_rate(&mut self, gps_id: NEO6MID, rate: u16) -> Result<(), NEO6MError>;
}

#[repr(C)]
pub struct SimpleNEO6MConfig {
    pub controller: SimpleNEO6MController,
    pub baudrates: Vec<(NEO6MID, AtomicUsize)>,
}

impl SimpleNEO6MConfig {
    pub fn new(controller: SimpleNEO6MController) -> Self {
        SimpleNEO6MConfig {
            controller,
            baudrates: Vec::new(),
        }
    }
}

impl NEO6MConfig for SimpleNEO6MConfig {
    fn set_baudrate(&mut self, gps_id: NEO6MID, baud: u32) -> Result<(), NEO6MError> {
        self.baudrates.push((gps_id, AtomicUsize::new(baud as usize)));
        Ok(())
    }
    
    fn set_update_rate(&mut self, _gps_id: NEO6MID, _rate: u16) -> Result<(), NEO6MError> {
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
