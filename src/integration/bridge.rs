#![no_std]
#![no_main]

/// OOP-based Bridge for SigmaOS
/// Based on Ideas-999-Structured: Integration & Interoperability Item 916
/// Implements bridging between different systems

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BridgeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BridgeType { Native = 0, Foreign = 1, Hybrid = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BridgeError { Success = 0, NotFound = 1, ConnectionFailed = 2 }

pub trait Bridge {
    fn id(&self) -> BridgeID;
    fn bridge_type(&self) -> BridgeType;
    fn target_system(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleBridge {
    pub id: BridgeID,
    pub bridge_type: AtomicUsize,
    pub target_system: [u8; 64],
}

impl SimpleBridge {
    pub fn new(id: BridgeID, bridge_type: BridgeType, target_system: &[u8]) -> Self {
        let mut target_array = [0u8; 64];
        let target_len = target_system.len().min(63);
        for i in 0..target_len {
            target_array[i] = target_system[i];
        }
        SimpleBridge {
            id,
            bridge_type: AtomicUsize::new(bridge_type as usize),
            target_system: target_array,
        }
    }
}

impl Bridge for SimpleBridge {
    fn id(&self) -> BridgeID { self.id }
    fn bridge_type(&self) -> BridgeType { unsafe { core::mem::transmute(self.bridge_type.load(Ordering::SeqCst)) } }
    fn target_system(&self) -> &[u8] {
        let len = self.target_system.iter().position(|&b| b == 0).unwrap_or(64);
        &self.target_system[..len]
    }
}

pub trait BridgeManager {
    fn create_bridge(&mut self, bridge_type: BridgeType, target: &[u8]) -> Result<BridgeID, BridgeError>;
    fn destroy_bridge(&mut self, id: BridgeID) -> Result<(), BridgeError>;
    fn get_bridge(&self, id: BridgeID) -> Option<&dyn Bridge>;
    def send_data(&self, bridge_id: BridgeID, data: &[u8]) -> Result<(), BridgeError>;
}

#[repr(C)]
pub struct SimpleBridgeManager {
    pub bridges: Vec<Option<Box<dyn Bridge>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBridgeManager {
    pub fn new() -> Self {
        SimpleBridgeManager {
            bridges: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BridgeManager for SimpleBridgeManager {
    fn create_bridge(&mut self, bridge_type: BridgeType, target: &[u8]) -> Result<BridgeID, BridgeError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let bridge = SimpleBridge::new(id, bridge_type, target);
        self.bridges.push(Some(Box::new(bridge)));
        Ok(id)
    }
    
    fn destroy_bridge(&mut self, id: BridgeID) -> Result<(), BridgeError> {
        for bridge_option in &mut self.bridges {
            if let Some(ref bridge) = *bridge_option {
                if bridge.id() == id {
                    return Ok(());
                }
            }
        }
        Err(BridgeError::NotFound)
    }
    
    fn get_bridge(&self, id: BridgeID) -> Option<&dyn Bridge> {
        for bridge_option in &self.bridges {
            if let Some(ref bridge) = *bridge_option {
                if bridge.id() == id { return Some(bridge.as_ref()); }
            }
        }
        None
    }
    
    fn send_data(&self, bridge_id: BridgeID, _data: &[u8]) -> Result<(), BridgeError> {
        if self.get_bridge(bridge_id).is_some() {
            Ok(())
        } else {
            Err(BridgeError::NotFound)
        }
    }
}

pub trait DataConverter {
    fn convert(&self, data: &[u8], from_format: &[u8], to_format: &[u8]) -> Result<Vec<u8>, BridgeError>;
}

#[repr(C)]
pub struct SimpleDataConverter;

impl SimpleDataConverter {
    pub fn new() -> Self { SimpleDataConverter }
}

impl DataConverter for SimpleDataConverter {
    fn convert(&self, data: &[u8], _from_format: &[u8], _to_format: &[u8]) -> Result<Vec<u8>, BridgeError> {
        let mut result = Vec::new();
        for &byte in data {
            result.push(byte);
        }
        Ok(result)
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
