#![no_std]
#![no_main]

/// OOP-based MCP2515 CAN for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3076
/// Implements MCP2515 CAN controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MCP2515ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MCP2515Error { Success = 0, NotFound = 1 }

pub trait MCP2515Device {
    fn id(&self) -> MCP2515ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMCP2515Device {
    pub id: MCP2515ID,
    pub initialized: AtomicUsize,
}

impl SimpleMCP2515Device {
    pub fn new(id: MCP2515ID) -> Self {
        SimpleMCP2515Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MCP2515Device for SimpleMCP2515Device {
    fn id(&self) -> MCP2515ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MCP2515Controller {
    fn init(&mut self, mcp_id: MCP2515ID) -> Result<(), MCP2515Error>;
    fn send(&self, mcp_id: MCP2515ID, id: u32, data: &[u8]) -> Result<(), MCP2515Error>;
    def receive(&self, mcp_id: MCP2515ID, buffer: &mut [u8]) -> Result<(u32, usize), MCP2515Error>;
}

#[repr(C)]
pub struct SimpleMCP2515Controller {
    pub devices: Vec<Option<Box<dyn MCP2515Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMCP2515Controller {
    pub fn new() -> Self {
        SimpleMCP2515Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MCP2515Controller for SimpleMCP2515Controller {
    fn init(&mut self, mcp_id: MCP2515ID) -> Result<(), MCP2515Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == mcp_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MCP2515Error::NotFound)
    }
    
    fn send(&self, mcp_id: MCP2515ID, _id: u32, _data: &[u8]) -> Result<(), MCP2515Error> {
        if self.get_device(mcp_id).is_some() {
            Ok(())
        } else {
            Err(MCP2515Error::NotFound)
        }
    }
    
    fn receive(&self, mcp_id: MCP2515ID, buffer: &mut [u8]) -> Result<(u32, usize), MCP2515Error> {
        if self.get_device(mcp_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok((0, buffer.len()))
        } else {
            Err(MCP2515Error::NotFound)
        }
    }
    
    fn get_device(&self, id: MCP2515ID) -> Option<&dyn MCP2515Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait MCP2515Filter {
    def set_filter(&mut self, mcp_id: MCP2515ID, filter: u32, mask: u32) -> Result<(), MCP2515Error>;
    def set_mode(&mut self, mcp_id: MCP2515ID, mode: u8) -> Result<(), MCP2515Error>;
}

#[repr(C)]
pub struct SimpleMCP2515Filter {
    pub controller: SimpleMCP2515Controller,
    pub modes: Vec<(MCP2515ID, AtomicUsize)>,
}

impl SimpleMCP2515Filter {
    pub fn new(controller: SimpleMCP2515Controller) -> Self {
        SimpleMCP2515Filter {
            controller,
            modes: Vec::new(),
        }
    }
}

impl MCP2515Filter for SimpleMCP2515Filter {
    fn set_filter(&mut self, _mcp_id: MCP2515ID, _filter: u32, _mask: u32) -> Result<(), MCP2515Error> {
        Ok(())
    }
    
    fn set_mode(&mut self, mcp_id: MCP2515ID, mode: u8) -> Result<(), MCP2515Error> {
        self.modes.push((mcp_id, AtomicUsize::new(mode as usize)));
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
