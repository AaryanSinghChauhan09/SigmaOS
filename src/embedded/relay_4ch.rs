#![no_std]
#![no_main]

/// OOP-based 4-Channel Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2796
/// Implements 4-channel relay module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Relay4ChID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Relay4ChError { Success = 0, NotFound = 1 }

pub trait Relay4Ch {
    fn id(&self) -> Relay4ChID;
    fn is_on(&self, channel: u8) -> bool;
}

#[repr(C)]
pub struct SimpleRelay4Ch {
    pub id: Relay4ChID,
    pub states: [AtomicUsize; 4],
}

impl SimpleRelay4Ch {
    pub fn new(id: Relay4ChID) -> Self {
        SimpleRelay4Ch {
            id,
            states: [AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0)],
        }
    }
}

impl Relay4Ch for SimpleRelay4Ch {
    fn id(&self) -> Relay4ChID { self.id }
    fn is_on(&self, channel: u8) -> bool {
        if channel < 4 {
            self.states[channel as usize].load(Ordering::SeqCst) == 1
        } else {
            false
        }
    }
}

pub trait Relay4ChController {
    fn set(&self, relay_id: Relay4ChID, channel: u8, on: bool) -> Result<(), Relay4ChError>;
    fn toggle(&self, relay_id: Relay4ChID, channel: u8) -> Result<(), Relay4ChError>;
    def set_all(&self, relay_id: Relay4ChID, on: bool) -> Result<(), Relay4ChError>;
}

#[repr(C)]
pub struct SimpleRelay4ChController {
    pub relays: Vec<Option<Box<dyn Relay4Ch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRelay4ChController {
    pub fn new() -> Self {
        SimpleRelay4ChController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Relay4ChController for SimpleRelay4ChController {
    fn set(&self, relay_id: Relay4ChID, channel: u8, on: bool) -> Result<(), Relay4ChError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == relay_id {
                    if channel < 4 {
                        relay.states[channel as usize].store(if on { 1 } else { 0 }, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(Relay4ChError::NotFound)
    }
    
    fn toggle(&self, relay_id: Relay4ChID, channel: u8) -> Result<(), Relay4ChError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == relay_id {
                    if channel < 4 {
                        let current = relay.states[channel as usize].load(Ordering::SeqCst);
                        relay.states[channel as usize].store(if current == 1 { 0 } else { 1 }, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(Relay4ChError::NotFound)
    }
    
    fn set_all(&self, relay_id: Relay4ChID, on: bool) -> Result<(), Relay4ChError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == relay_id {
                    for state in &relay.states {
                        state.store(if on { 1 } else { 0 }, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(Relay4ChError::NotFound)
    }
    
    fn get_relay(&self, id: Relay4ChID) -> Option<&dyn Relay4Ch> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait Relay4ChPulse {
    def pulse(&self, relay_id: Relay4ChID, channel: u8, duration_ms: u16) -> Result<(), Relay4ChError>;
}

#[repr(C)]
pub struct SimpleRelay4ChPulse {
    pub controller: SimpleRelay4ChController,
}

impl SimpleRelay4ChPulse {
    pub fn new(controller: SimpleRelay4ChController) -> Self {
        SimpleRelay4ChPulse { controller }
    }
}

impl Relay4ChPulse for SimpleRelay4ChPulse {
    fn pulse(&self, relay_id: Relay4ChID, _channel: u8, _duration_ms: u16) -> Result<(), Relay4ChError> {
        if self.controller.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay4ChError::NotFound)
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
