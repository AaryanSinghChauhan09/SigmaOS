#![no_std]
#![no_main]

/// OOP-based SSR 12V Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3606
/// Implements 12V solid state relay

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SSR12VID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SSR12VError { Success = 0, NotFound = 1 }

pub trait SSR12VDevice {
    fn id(&self) -> SSR12VID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSSR12VDevice {
    pub id: SSR12VID,
    pub initialized: AtomicUsize,
}

impl SimpleSSR12VDevice {
    pub fn new(id: SSR12VID) -> Self {
        SimpleSSR12VDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SSR12VDevice for SimpleSSR12VDevice {
    fn id(&self) -> SSR12VID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SSR12VController {
    fn init(&mut self, relay_id: SSR12VID) -> Result<(), SSR12VError>;
    fn set(&self, relay_id: SSR12VID, state: bool) -> Result<(), SSR12VError>;
    def toggle(&self, relay_id: SSR12VID) -> Result<(), SSR12VError>;
}

#[repr(C)]
pub struct SimpleSSR12VController {
    pub relays: Vec<Option<Box<dyn SSR12VDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSSR12VController {
    pub fn new() -> Self {
        SimpleSSR12VController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SSR12VController for SimpleSSR12VController {
    fn init(&mut self, relay_id: SSR12VID) -> Result<(), SSR12VError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    relay.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SSR12VError::NotFound)
    }
    
    fn set(&self, relay_id: SSR12VID, _state: bool) -> Result<(), SSR12VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(SSR12VError::NotFound)
        }
    }
    
    fn toggle(&self, relay_id: SSR12VID) -> Result<(), SSR12VError> {
        if self.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(SSR12VError::NotFound)
        }
    }
    
    fn get_relay(&self, id: SSR12VID) -> Option<&dyn SSR12VDevice> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait SSR12VLoad {
    def get_load(&self, relay_id: SSR12VID) -> Result<f32, SSR12VError>;
}

#[repr(C)]
pub struct SimpleSSR12VLoad {
    pub controller: SimpleSSR12VController,
}

impl SimpleSSR12VLoad {
    pub fn new(controller: SimpleSSR12VController) -> Self {
        SimpleSSR12VLoad { controller }
    }
}

impl SSR12VLoad for SimpleSSR12VLoad {
    fn get_load(&self, relay_id: SSR12VID) -> Result<f32, SSR12VError> {
        if self.controller.get_relay(relay_id).is_some() {
            Ok(0.0)
        } else {
            Err(SSR12VError::NotFound)
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
