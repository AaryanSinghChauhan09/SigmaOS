#![no_std]
#![no_main]

/// OOP-based SSR 5V Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3596
/// Implements 5V solid state relay

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SSR5VID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SSR5VError { Success = 0, NotFound = 1 }

pub trait SSR5VDevice {
    fn id(&self) -> SSR5VID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSSR5VDevice {
    pub id: SSR5VID,
    pub initialized: AtomicUsize,
}

impl SimpleSSR5VDevice {
    pub fn new(id: SSR5VID) -> Self {
        SimpleSSR5VDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SSR5VDevice for SimpleSSR5VDevice {
    fn id(&self) -> SSR5VID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SSR5VController {
    fn init(&mut self, relay_id: SSR5VID) -> Result<(), SSR5VError>;
    fn set(&self, relay_id: SSR5VID, state: bool) -> Result<(), SSR5VError>;
    def toggle(&self, relay_id: SSR5VID) -> Result<(), SSR5VError>;
}

#[repr(C)]
pub struct SimpleSSR5VController {
    pub relays: Vec<Option<Box<dyn SSR5VDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSSR5VController {
    pub fn new() -> Self {
        SimpleSSR5VController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SSR5VController for SimpleSSR5VController {
    fn init(&mut self, relay_id: SSR5VID) -> Result<(), SSR5VError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    relay.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SSR5VError::NotFound)
    }
    
    fn set(&self, relay_id: SSR5VID, _state: bool) -> Result<(), SSR5VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(SSR5VError::NotFound)
        }
    }
    
    fn toggle(&self, relay_id: SSR5VID) -> Result<(), SSR5VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(SSR5VError::NotFound)
        }
    }
    
    fn get_relay(&self, id: SSR5VID) -> Option<&dyn SSR5VDevice> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait SSR5VZeroCross {
    def set_zero_cross(&mut self, relay_id: SSR5VID, enable: bool) -> Result<(), SSR5VError>;
}

#[repr(C)]
pub struct SimpleSSR5VZeroCross {
    pub controller: SimpleSSR5VController,
    pub zero_cross: Vec<(SSR5VID, AtomicUsize)>,
}

impl SimpleSSR5VZeroCross {
    pub fn new(controller: SimpleSSR5VController) -> Self {
        SimpleSSR5VZeroCross {
            controller,
            zero_cross: Vec::new(),
        }
    }
}

impl SSR5VZeroCross for SimpleSSR5VZeroCross {
    fn set_zero_cross(&mut self, relay_id: SSR5VID, enable: bool) -> Result<(), SSR5VError> {
        self.zero_cross.push((relay_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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
