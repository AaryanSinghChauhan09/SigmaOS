#![no_std]
#![no_main]

/// OOP-based Zigbee for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1996
/// Implements Zigbee module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ZigbeeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ZigbeeError { Success = 0, NotFound = 1 }

pub trait ZigbeeModule {
    fn id(&self) -> ZigbeeID;
    fn is_coordinator(&self) -> bool;
}

#[repr(C)]
pub struct SimpleZigbeeModule {
    pub id: ZigbeeID,
    pub coordinator: AtomicUsize,
}

impl SimpleZigbeeModule {
    pub fn new(id: ZigbeeID) -> Self {
        SimpleZigbeeModule {
            id,
            coordinator: AtomicUsize::new(0),
        }
    }
}

impl ZigbeeModule for SimpleZigbeeModule {
    fn id(&self) -> ZigbeeID { self.id }
    fn is_coordinator(&self) -> bool { self.coordinator.load(Ordering::SeqCst) == 1 }
}

pub trait ZigbeeController {
    fn start_coordinator(&mut self, zigbee_id: ZigbeeID) -> Result<(), ZigbeeError>;
    fn send(&self, zigbee_id: ZigbeeID, dest: u16, data: &[u8]) -> Result<(), ZigbeeError>;
    def receive(&self, zigbee_id: ZigbeeID, buffer: &mut [u8]) -> Result<usize, ZigbeeError>;
}

#[repr(C)]
pub struct SimpleZigbeeController {
    pub modules: Vec<Option<Box<dyn ZigbeeModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleZigbeeController {
    pub fn new() -> Self {
        SimpleZigbeeController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ZigbeeController for SimpleZigbeeController {
    fn start_coordinator(&mut self, zigbee_id: ZigbeeID) -> Result<(), ZigbeeError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == zigbee_id {
                    module.coordinator.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ZigbeeError::NotFound)
    }
    
    fn send(&self, _zigbee_id: ZigbeeID, _dest: u16, _data: &[u8]) -> Result<(), ZigbeeError> {
        Ok(())
    }
    
    fn receive(&self, zigbee_id: ZigbeeID, buffer: &mut [u8]) -> Result<usize, ZigbeeError> {
        if self.get_module(zigbee_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(ZigbeeError::NotFound)
        }
    }
    
    fn get_module(&self, id: ZigbeeID) -> Option<&dyn ZigbeeModule> {
        for module_option in &self.modules {
            if let Some(ref module) = *module_option {
                if module.id() == id { return Some(module.as_ref()); }
            }
        }
        None
    }
}

pub trait ZCL {
    def send_zcl(&self, zigbee_id: ZigbeeID, endpoint: u8, cluster: u16, command: u8, data: &[u8]) -> Result<(), ZigbeeError>;
    def bind(&mut self, zigbee_id: ZigbeeID, src_ep: u8, cluster: u16, dst_addr: u16, dst_ep: u8) -> Result<(), ZigbeeError>;
}

#[repr(C)]
pub struct SimpleZCL {
    pub controller: SimpleZigbeeController,
}

impl SimpleZCL {
    pub fn new(controller: SimpleZigbeeController) -> Self {
        SimpleZCL { controller }
    }
}

impl ZCL for SimpleZCL {
    fn send_zcl(&self, _zigbee_id: ZigbeeID, _endpoint: u8, _cluster: u16, _command: u8, _data: &[u8]) -> Result<(), ZigbeeError> {
        Ok(())
    }
    
    fn bind(&mut self, _zigbee_id: ZigbeeID, _src_ep: u8, _cluster: u16, _dst_addr: u16, _dst_ep: u8) -> Result<(), ZigbeeError> {
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
