#![no_std]
#![no_main]

/// OOP-based CAN for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1156
/// Implements CAN bus communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CANID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CANError { Success = 0, NotFound = 1 }

pub trait CANMessage {
    fn id(&self) -> u32;
    fn data(&self) -> &[u8];
    fn dlc(&self) -> u8;
}

#[repr(C)]
pub struct SimpleCANMessage {
    pub id: u32,
    pub data: [u8; 8],
    pub dlc: AtomicUsize,
}

impl SimpleCANMessage {
    pub fn new(id: u32, data: &[u8]) -> Self {
        let mut data_array = [0u8; 8];
        let data_len = data.len().min(8);
        for i in 0..data_len {
            data_array[i] = data[i];
        }
        SimpleCANMessage {
            id,
            data: data_array,
            dlc: AtomicUsize::new(data_len),
        }
    }
}

impl CANMessage for SimpleCANMessage {
    fn id(&self) -> u32 { self.id }
    fn data(&self) -> &[u8] { &self.data[..self.dlc.load(Ordering::SeqCst)] }
    fn dlc(&self) -> u8 { self.dlc.load(Ordering::SeqCst) as u8 }
}

pub trait CANBus {
    fn send(&self, message: &dyn CANMessage) -> Result<(), CANError>;
    fn receive(&self) -> Option<Box<dyn CANMessage>>;
    def set_filter(&mut self, id: u32, mask: u32);
}

#[repr(C)]
pub struct SimpleCANBus {
    pub messages: Vec<Option<Box<dyn CANMessage>>>,
    pub filter_id: AtomicUsize,
    pub filter_mask: AtomicUsize,
}

impl SimpleCANBus {
    pub fn new() -> Self {
        SimpleCANBus {
            messages: Vec::new(),
            filter_id: AtomicUsize::new(0),
            filter_mask: AtomicUsize::new(0x7FF),
        }
    }
}

impl CANBus for SimpleCANBus {
    fn send(&self, message: &dyn CANMessage) -> Result<(), CANError> {
        Ok(())
    }
    
    fn receive(&self) -> Option<Box<dyn CANMessage>> {
        if !self.messages.is_empty() {
            self.messages.remove(0)
        } else {
            None
        }
    }
    
    fn set_filter(&mut self, id: u32, mask: u32) {
        self.filter_id.store(id as usize, Ordering::SeqCst);
        self.filter_mask.store(mask as usize, Ordering::SeqCst);
    }
}

pub trait CANController {
    fn init(&mut self, baud_rate: u32) -> Result<(), CANError>;
    fn start(&mut self) -> Result<(), CANError>;
    fn stop(&mut self) -> Result<(), CANError>;
}

#[repr(C)]
pub struct SimpleCANController {
    pub bus: SimpleCANBus,
    pub running: AtomicUsize,
}

impl SimpleCANController {
    pub fn new(bus: SimpleCANBus) -> Self {
        SimpleCANController {
            bus,
            running: AtomicUsize::new(0),
        }
    }
}

impl CANController for SimpleCANController {
    fn init(&mut self, _baud_rate: u32) -> Result<(), CANError> {
        Ok(())
    }
    
    fn start(&mut self) -> Result<(), CANError> {
        self.running.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn stop(&mut self) -> Result<(), CANError> {
        self.running.store(0, Ordering::SeqCst);
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
    fn remove(&mut self, index: usize) -> Result<Box<T>, ()> {
        unsafe {
            if index < self.len {
                let item = Box::new(core::ptr::read(self.data.add(index)));
                for i in index..self.len - 1 {
                    core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
                }
                self.len -= 1;
                Ok(item)
            } else {
                Err(())
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
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
