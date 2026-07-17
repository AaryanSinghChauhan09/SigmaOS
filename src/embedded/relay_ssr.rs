#![no_std]
#![no_main]

/// OOP-based SSR Relay for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2816
/// Implements Solid State Relay (SSR)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SSRRelayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SSRRelayError { Success = 0, NotFound = 1 }

pub trait SSRRelay {
    fn id(&self) -> SSRRelayID;
    fn is_on(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSSRRelay {
    pub id: SSRRelayID,
    pub on: AtomicUsize,
}

impl SimpleSSRRelay {
    pub fn new(id: SSRRelayID) -> Self {
        SimpleSSRRelay {
            id,
            on: AtomicUsize::new(0),
        }
    }
}

impl SSRRelay for SimpleSSRRelay {
    fn id(&self) -> SSRRelayID { self.id }
    fn is_on(&self) -> bool { self.on.load(Ordering::SeqCst) == 1 }
}

pub trait SSRRelayController {
    fn set(&self, ssr_id: SSRRelayID, on: bool) -> Result<(), SSRRelayError>;
    fn toggle(&self, ssr_id: SSRRelayID) -> Result<(), SSRRelayError>;
    def get_state(&self, ssr_id: SSRRelayID) -> Result<bool, SSRRelayError>;
}

#[repr(C)]
pub struct SimpleSSRRelayController {
    pub relays: Vec<Option<Box<dyn SSRRelay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSSRRelayController {
    pub fn new() -> Self {
        SimpleSSRRelayController {
            relays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SSRRelayController for SimpleSSRRelayController {
    fn set(&self, ssr_id: SSRRelayID, on: bool) -> Result<(), SSRRelayError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == ssr_id {
                    relay.on.store(if on { 1 } else { 0 }, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SSRRelayError::NotFound)
    }
    
    fn toggle(&self, ssr_id: SSRRelayID) -> Result<(), SSRRelayError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == ssr_id {
                    let current = relay.on.load(Ordering::SeqCst);
                    relay.on.store(if current == 1 { 0 } else { 1 }, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SSRRelayError::NotFound)
    }
    
    fn get_state(&self, ssr_id: SSRRelayID) -> Result<bool, SSRRelayError> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == ssr_id {
                    return Ok(relay.on.load(Ordering::SeqCst) == 1);
                }
            }
        }
        Err(SSRRelayError::NotFound)
    }
    
    fn get_relay(&self, id: SSRRelayID) -> Option<&dyn SSRRelay> {
        for relay_option in &self.relays {
            if let Some(ref relay) = *relay_option {
                if relay.id() == id { return Some(relay.as_ref()); }
            }
        }
        None
    }
}

pub trait SSRRelayZeroCross {
    def set_zero_cross(&mut self, ssr_id: SSRRelayID, enable: bool) -> Result<(), SSRRelayError>;
    def get_zero_cross(&self, ssr_id: SSRRelayID) -> Result<bool, SSRRelayError>;
}

#[repr(C)]
pub struct SimpleSSRRelayZeroCross {
    pub controller: SimpleSSRRelayController,
    pub zero_cross_states: Vec<(SSRRelayID, AtomicUsize)>,
}

impl SimpleSSRRelayZeroCross {
    pub fn new(controller: SimpleSSRRelayController) -> Self {
        SimpleSSRRelayZeroCross {
            controller,
            zero_cross_states: Vec::new(),
        }
    }
}

impl SSRRelayZeroCross for SimpleSSRRelayZeroCross {
    fn set_zero_cross(&mut self, ssr_id: SSRRelayID, enable: bool) -> Result<(), SSRRelayError> {
        self.zero_cross_states.push((ssr_id, AtomicUsize::new(if enable { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_zero_cross(&self, ssr_id: SSRRelayID) -> Result<bool, SSRRelayError> {
        for &(id, ref state) in &self.zero_cross_states {
            if id == ssr_id {
                return Ok(state.load(Ordering::SeqCst) == 1);
            }
        }
        Err(SSRRelayError::NotFound)
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
