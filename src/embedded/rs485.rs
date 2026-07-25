#![no_std]
#![no_main]

/// OOP-based RS485 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1506
/// Implements RS485 communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NodeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RS485Error { Success = 0, NotFound = 1 }

pub trait RS485Node {
    fn id(&self) -> NodeID;
    fn address(&self) -> u8;
}

#[repr(C)]
pub struct SimpleRS485Node {
    pub id: NodeID,
    pub address: AtomicUsize,
}

impl SimpleRS485Node {
    pub fn new(id: NodeID, address: u8) -> Self {
        SimpleRS485Node {
            id,
            address: AtomicUsize::new(address as usize),
        }
    }
}

impl RS485Node for SimpleRS485Node {
    fn id(&self) -> NodeID { self.id }
    fn address(&self) -> u8 { self.address.load(Ordering::SeqCst) as u8 }
}

pub trait RS485Bus {
    fn send(&self, address: u8, data: &[u8]) -> Result<(), RS485Error>;
    fn receive(&self, buffer: &mut [u8]) -> Result<usize, RS485Error>;
    def broadcast(&self, data: &[u8]) -> Result<(), RS485Error>;
}

#[repr(C)]
pub struct SimpleRS485Bus {
    pub nodes: Vec<Option<Box<dyn RS485Node>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRS485Bus {
    pub fn new() -> Self {
        SimpleRS485Bus {
            nodes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RS485Bus for SimpleRS485Bus {
    fn send(&self, _address: u8, _data: &[u8]) -> Result<(), RS485Error> {
        Ok(())
    }
    
    fn receive(&self, buffer: &mut [u8]) -> Result<usize, RS485Error> {
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
        Ok(buffer.len())
    }
    
    fn broadcast(&self, _data: &[u8]) -> Result<(), RS485Error> {
        Ok(())
    }
}

pub trait Modbus {
    def read_register(&self, slave_id: u8, register: u16) -> Result<u16, RS485Error>;
    def write_register(&self, slave_id: u8, register: u16, value: u16) -> Result<(), RS485Error>;
}

#[repr(C)]
pub struct SimpleModbus {
    pub bus: SimpleRS485Bus,
}

impl SimpleModbus {
    pub fn new(bus: SimpleRS485Bus) -> Self {
        SimpleModbus { bus }
    }
}

impl Modbus for SimpleModbus {
    fn read_register(&self, _slave_id: u8, _register: u16) -> Result<u16, RS485Error> {
        Ok(0)
    }
    
    fn write_register(&self, _slave_id: u8, _register: u16, _value: u16) -> Result<(), RS485Error> {
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
