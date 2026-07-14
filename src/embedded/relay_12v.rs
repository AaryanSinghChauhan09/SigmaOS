#![no_std]
#![no_main]

/// OOP-based 12V Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3586
/// Implements 12V relay module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Relay12VID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Relay12VError { Success = 0, NotFound = 1 }

pub trait Relay12VDevice {
    fn id(&self) -> Relay12VID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRelay12VDevice {
    pub id: Relay12VID,
    pub initialized: AtomicUsize,
}

impl SimpleRelay12VDevice {
    pub fn new(id: Relay12VID) -> Self {
        SimpleRelay12VDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl Relay12VDevice for SimpleRelay12VDevice {
    fn id(&self) -> Relay12VID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait Relay12VController {
    fn init(&mut self, relay_id: Relay12VID) -> Result<(), Relay12VError>;
    fn set(&self, relay_id: Relay12VID, state: bool) -> Result<(), Relay12VError>;
    def toggle(&self, relay_id: Relay12VID) -> Result<(), Relay12VError>;
}

#[repr(C)]
pub struct SimpleRelay12VController {
    pub relays: Vec<Option<Box<dyn Relay12VDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRelay12VController {
    pub fn new() -> Self {
        SimpleRelay12VController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Relay12VController for SimpleRelay12VController {
    fn init(&mut self, relay_id: Relay12VID) -> Result<(), Relay12VError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    relay.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(Relay12VError::NotFound)
    }
    
    fn set(&self, relay_id: Relay12VID, _state: bool) -> Result<(), Relay12VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay12VError::NotFound)
        }
    }
    
    fn toggle(&self, relay_id: Relay12VID) -> Result<(), Relay12VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay12VError::NotFound)
        }
    }
    
    fn get_relay(&self, id: Relay12VID) -> Option<&dyn Relay12VDevice> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait Relay12VTimer {
    def set_timer(&mut self, relay_id: Relay12VID, duration_ms: u16) -> Result<(), Relay12VError>;
}

#[repr(C)]
pub struct SimpleRelay12VTimer {
    pub controller: SimpleRelay12VController,
    pub timers: Vec<(Relay12VID, AtomicUsize)>,
}

impl SimpleRelay12VTimer {
    pub fn new(controller: SimpleRelay12VController) -> Self {
        SimpleRelay12VTimer {
            controller,
            timers: Vec::new(),
        }
    }
}

impl Relay12VTimer for SimpleRelay12VTimer {
    fn set_timer(&mut self, relay_id: Relay12VID, duration_ms: u16) -> Result<(), Relay12VError> {
        self.timers.push((relay_id, AtomicUsize::new(duration_ms as usize)));
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
