#![no_std]
#![no_main]

/// OOP-based 8-Channel Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2806
/// Implements 8-channel relay module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Relay8ChID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Relay8ChError { Success = 0, NotFound = 1 }

pub trait Relay8Ch {
    fn id(&self) -> Relay8ChID;
    fn is_on(&self, channel: u8) -> bool;
}

#[repr(C)]
pub struct SimpleRelay8Ch {
    pub id: Relay8ChID,
    pub states: [AtomicUsize; 8],
}

impl SimpleRelay8Ch {
    pub fn new(id: Relay8ChID) -> Self {
        SimpleRelay8Ch {
            id,
            states: [
                AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
                AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
            ],
        }
    }
}

impl Relay8Ch for SimpleRelay8Ch {
    fn id(&self) -> Relay8ChID { self.id }
    fn is_on(&self, channel: u8) -> bool {
        if channel < 8 {
            self.states[channel as usize].load(Ordering::SeqCst) == 1
        } else {
            false
        }
    }
}

pub trait Relay8ChController {
    fn set(&self, relay_id: Relay8ChID, channel: u8, on: bool) -> Result<(), Relay8ChError>;
    fn toggle(&self, relay_id: Relay8ChID, channel: u8) -> Result<(), Relay8ChError>;
    def set_all(&self, relay_id: Relay8ChID, on: bool) -> Result<(), Relay8ChError>;
}

#[repr(C)]
pub struct SimpleRelay8ChController {
    pub relays: Vec<Option<Box<dyn Relay8Ch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRelay8ChController {
    pub fn new() -> Self {
        SimpleRelay8ChController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Relay8ChController for SimpleRelay8ChController {
    fn set(&self, relay_id: Relay8ChID, channel: u8, on: bool) -> Result<(), Relay8ChError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == relay_id {
                    if channel < 8 {
                        relay.states[channel as usize].store(if on { 1 } else { 0 }, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(Relay8ChError::NotFound)
    }
    
    fn toggle(&self, relay_id: Relay8ChID, channel: u8) -> Result<(), Relay8ChError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == relay_id {
                    if channel < 8 {
                        let current = relay.states[channel as usize].load(Ordering::SeqCst);
                        relay.states[channel as usize].store(if current == 1 { 0 } else { 1 }, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(Relay8ChError::NotFound)
    }
    
    fn set_all(&self, relay_id: Relay8ChID, on: bool) -> Result<(), Relay8ChError> {
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
        Err(Relay8ChError::NotFound)
    }
    
    fn get_relay(&self, id: Relay8ChID) -> Option<&dyn Relay8Ch> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait Relay8ChPulse {
    def pulse(&self, relay_id: Relay8ChID, channel: u8, duration_ms: u16) -> Result<(), Relay8ChError>;
}

#[repr(C)]
pub struct SimpleRelay8ChPulse {
    pub controller: SimpleRelay8ChController,
}

impl SimpleRelay8ChPulse {
    pub fn new(controller: SimpleRelay8ChController) -> Self {
        SimpleRelay8ChPulse { controller }
    }
}

impl Relay8ChPulse for SimpleRelay8ChPulse {
    fn pulse(&self, relay_id: Relay8ChID, _channel: u8, _duration_ms: u16) -> Result<(), Relay8ChError> {
        if self.controller.get_relay(relay_id).is_some() {
            Ok(())
        } else {
            Err(Relay8ChError::NotFound)
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
