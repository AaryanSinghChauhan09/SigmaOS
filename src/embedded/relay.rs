#![no_std]
#![no_main]

/// OOP-based Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1406
/// Implements relay control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RelayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RelayState { Open = 0, Closed = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RelayError { Success = 0, NotFound = 1 }

pub trait Relay {
    fn id(&self) -> RelayID;
    fn state(&self) -> RelayState;
}

#[repr(C)]
pub struct SimpleRelay {
    pub id: RelayID,
    pub state: AtomicUsize,
}

impl SimpleRelay {
    pub fn new(id: RelayID) -> Self {
        SimpleRelay {
            id,
            state: AtomicUsize::new(RelayState::Open as usize),
        }
    }
}

impl Relay for SimpleRelay {
    fn id(&self) -> RelayID { self.id }
    fn state(&self) -> RelayState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait RelayController {
    fn open(&mut self, relay_id: RelayID) -> Result<(), RelayError>;
    fn close(&mut self, relay_id: RelayID) -> Result<(), RelayError>;
    fn toggle(&mut self, relay_id: RelayID) -> Result<(), RelayError>;
}

#[repr(C)]
pub struct SimpleRelayController {
    pub relays: Vec<Option<Box<dyn Relay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRelayController {
    pub fn new() -> Self {
        SimpleRelayController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RelayController for SimpleRelayController {
    fn open(&mut self, relay_id: RelayID) -> Result<(), RelayError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    relay.state.store(RelayState::Open as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RelayError::NotFound)
    }
    
    fn close(&mut self, relay_id: RelayID) -> Result<(), RelayError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    relay.state.store(RelayState::Closed as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RelayError::NotFound)
    }
    
    fn toggle(&mut self, relay_id: RelayID) -> Result<(), RelayError> {
        for relay_option in &mut self.relays {
            if let Some(ref mut relay) = *relay_option {
                if relay.id() == relay_id {
                    let current = relay.state.load(Ordering::SeqCst);
                    let new_state = if current == RelayState::Open as usize {
                        RelayState::Closed as usize
                    } else {
                        RelayState::Open as usize
                    };
                    relay.state.store(new_state, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RelayError::NotFound)
    }
}

pub trait RelayBank {
    def open_all(&mut self);
    def close_all(&mut self);
    def get_states(&self) -> Vec<RelayState>;
}

#[repr(C)]
pub struct SimpleRelayBank {
    pub controller: SimpleRelayController,
}

impl SimpleRelayBank {
    pub fn new(controller: SimpleRelayController) -> Self {
        SimpleRelayBank { controller }
    }
}

impl RelayBank for SimpleRelayBank {
    fn open_all(&mut self) {
        for relay_option in &mut self.controller.relays {
            if let Some(ref mut relay) = *relay_option {
                relay.state.store(RelayState::Open as usize, Ordering::SeqCst);
            }
        }
    }
    
    fn close_all(&mut self) {
        for relay_option in &mut self.controller.relays {
            if let Some(ref mut relay) = *relay_option {
                relay.state.store(RelayState::Closed as usize, Ordering::SeqCst);
            }
        }
    }
    
    fn get_states(&self) -> Vec<RelayState> {
        let mut states = Vec::new();
        for relay_option in &self.controller.relays {
            if let Some(ref relay) = *relay_option {
                states.push(relay.state());
            }
        }
        states
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
