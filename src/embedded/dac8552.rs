#![no_std]
#![no_main]

/// OOP-based DAC8552 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2906
/// Implements DAC8552 dual DAC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DAC8552ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DAC8552Error { Success = 0, NotFound = 1 }

pub trait DAC8552DAC {
    fn id(&self) -> DAC8552ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDAC8552DAC {
    pub id: DAC8552ID,
    pub initialized: AtomicUsize,
}

impl SimpleDAC8552DAC {
    pub fn new(id: DAC8552ID) -> Self {
        SimpleDAC8552DAC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DAC8552DAC for SimpleDAC8552DAC {
    fn id(&self) -> DAC8552ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DAC8552Controller {
    fn init(&mut self, dac_id: DAC8552ID) -> Result<(), DAC8552Error>;
    fn write(&self, dac_id: DAC8552ID, channel: u8, value: u16) -> Result<(), DAC8552Error>;
    def read(&self, dac_id: DAC8552ID, channel: u8) -> Result<u16, DAC8552Error>;
}

#[repr(C)]
pub struct SimpleDAC8552Controller {
    pub dacs: Vec<Option<Box<dyn DAC8552DAC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDAC8552Controller {
    pub fn new() -> Self {
        SimpleDAC8552Controller {
            dacs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DAC8552Controller for SimpleDAC8552Controller {
    fn init(&mut self, dac_id: DAC8552ID) -> Result<(), DAC8552Error> {
        for dac_option in &mut self.dacs {
            if let Some(ref mut dac) = *dac_option {
                if dac.id() == dac_id {
                    dac.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DAC8552Error::NotFound)
    }
    
    fn write(&self, dac_id: DAC8552ID, _channel: u8, _value: u16) -> Result<(), DAC8552Error> {
        if self.get_dac(dac_id).is_some() {
            Ok(())
        } else {
            Err(DAC8552Error::NotFound)
        }
    }
    
    fn read(&self, dac_id: DAC8552ID, _channel: u8) -> Result<u16, DAC8552Error> {
        if self.get_dac(dac_id).is_some() {
            Ok(0)
        } else {
            Err(DAC8552Error::NotFound)
        }
    }
    
    fn get_dac(&self, id: DAC8552ID) -> Option<&dyn DAC8552DAC> {
        for dac_option in &self.dacs {
            if let Some(ref dac) = *dac_option {
                if dac.id() == id { return Some(dac.as_ref()); }
            }
        }
        None
    }
}

pub trait DAC8552Power {
    def set_power_down(&mut self, dac_id: DAC8552ID, channel: u8, mode: u8) -> Result<(), DAC8552Error>;
    def get_power_down(&self, dac_id: DAC8552ID, channel: u8) -> Result<u8, DAC8552Error>;
}

#[repr(C)]
pub struct SimpleDAC8552Power {
    pub controller: SimpleDAC8552Controller,
    pub power_modes: Vec<(DAC8552ID, AtomicUsize)>,
}

impl SimpleDAC8552Power {
    pub fn new(controller: SimpleDAC8552Controller) -> Self {
        SimpleDAC8552Power {
            controller,
            power_modes: Vec::new(),
        }
    }
}

impl DAC8552Power for SimpleDAC8552Power {
    fn set_power_down(&mut self, dac_id: DAC8552ID, mode: u8) -> Result<(), DAC8552Error> {
        self.power_modes.push((dac_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_power_down(&self, dac_id: DAC8552ID, _channel: u8) -> Result<u8, DAC8552Error> {
        for &(id, ref mode) in &self.power_modes {
            if id == dac_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(DAC8552Error::NotFound)
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
