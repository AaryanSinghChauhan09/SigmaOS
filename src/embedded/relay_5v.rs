#![no_std]
#![no_main]

/// OOP-based 5V Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3576
/// Implements 5V relay module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Relay5VID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Relay5VError { Success = 0, NotFound = 1 }

pub trait Relay5VDevice {
    fn id(&self) -> Relay5VID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRelay5VDevice {
    pub id: Relay5VID,
    pub initialized: AtomicUsize,
}

impl SimpleRelay5VDevice {
    pub fn new(id: Relay5VID) -> Self {
        SimpleRelay5VDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl Relay5VDevice for SimpleRelay5VDevice {
    fn id(&self) -> Relay5VID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait Relay5VController {
    fn init(&mut self, relay_id: Relay5VID) -> Result<(), Relay5VError>;
    fn set(&self, relay_id: Relay5VID, state: bool) -> Result<(), Relay5VError>;
    def toggle(&self, relay_id: Relay5VID) -> Result<(), Relay5VError>;
}

#[repr(C)]
pub struct SimpleRelay5VController {
    pub relays: Vec<Option<Box<dyn Relay5VDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRelay5VController {
    pub fn new() -> Self {
        SimpleRelay5VController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Relay5VController for SimpleRelay5VController {
    fn init(&mut self, relay_id: Relay5VID) -> Result<(), Relay5VError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    relay.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(Relay5VError::NotFound)
    }
    
    fn set(&self, relay_id: Relay5VID, _state: bool) -> Result<(), Relay5VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay5VError::NotFound)
        }
    }
    
    fn toggle(&self, relay_id: Relay5VID) -> Result<(), Relay5VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay5VError::NotFound)
        }
    }
    
    fn get_relay(&self, id: Relay5VID) -> Option<&dyn Relay5VDevice> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait Relay5VPulse {
    def pulse(&self, relay_id: Relay5VID, duration_ms: u16) -> Result<(), Relay5VError>;
}

#[repr(C)]
pub struct SimpleRelay5VPulse {
    pub controller: SimpleRelay5VController,
}

impl SimpleRelay5VPulse {
    pub fn new(controller: SimpleRelay5VController) -> Self {
        SimpleRelay5VPulse { controller }
    }
}

impl Relay5VPulse for SimpleRelay5VPulse {
    fn pulse(&self, relay_id: Relay5VID, _duration_ms: u16) -> Result<(), Relay5VError> {
        if self.controller.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay5VError::NotFound)
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
