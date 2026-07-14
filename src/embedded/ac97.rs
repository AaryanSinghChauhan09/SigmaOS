#![no_std]
#![no_main]

/// OOP-based AC97 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2356
/// Implements AC97 audio codec

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AC97ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AC97Error { Success = 0, NotFound = 1 }

pub trait AC97Device {
    fn id(&self) -> AC97ID;
    fn is_ready(&self) -> bool;
}

#[repr(C)]
pub struct SimpleAC97Device {
    pub id: AC97ID,
    pub ready: AtomicUsize,
}

impl SimpleAC97Device {
    pub fn new(id: AC97ID) -> Self {
        SimpleAC97Device {
            id,
            ready: AtomicUsize::new(0),
        }
    }
}

impl AC97Device for SimpleAC97Device {
    fn id(&self) -> AC97ID { self.id }
    fn is_ready(&self) -> bool { self.ready.load(Ordering::SeqCst) == 1 }
}

pub trait AC97Controller {
    fn init(&mut self, ac97_id: AC97ID) -> Result<(), AC97Error>;
    fn read_reg(&self, ac97_id: AC97ID, reg: u8) -> Result<u16, AC97Error>;
    def write_reg(&self, ac97_id: AC97ID, reg: u8, value: u16) -> Result<(), AC97Error>;
}

#[repr(C)]
pub struct SimpleAC97Controller {
    pub devices: Vec<Option<Box<dyn AC97Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAC97Controller {
    pub fn new() -> Self {
        SimpleAC97Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AC97Controller for SimpleAC97Controller {
    fn init(&mut self, ac97_id: AC97ID) -> Result<(), AC97Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == ac97_id {
                    device.ready.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(AC97Error::NotFound)
    }
    
    fn read_reg(&self, ac97_id: AC97ID, _reg: u8) -> Result<u16, AC97Error> {
        if self.get_device(ac97_id).is_some() {
            Ok(0)
        } else {
            Err(AC97Error::NotFound)
        }
    }
    
    fn write_reg(&self, ac97_id: AC97ID, _reg: u8, _value: u16) -> Result<(), AC97Error> {
        if self.get_device(ac97_id).is_some() {
            Ok(())
        } else {
            Err(AC97Error::NotFound)
        }
    }
    
    fn get_device(&self, id: AC97ID) -> Option<&dyn AC97Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait AC97Volume {
    def set_volume(&mut self, ac97_id: AC97ID, channel: u8, volume: u8) -> Result<(), AC97Error>;
    def get_volume(&self, ac97_id: AC97ID, channel: u8) -> Result<u8, AC97Error>;
}

#[repr(C)]
pub struct SimpleAC97Volume {
    pub controller: SimpleAC97Controller,
    pub volumes: Vec<(AC97ID, (AtomicUsize, AtomicUsize))>,
}

impl SimpleAC97Volume {
    pub fn new(controller: SimpleAC97Controller) -> Self {
        SimpleAC97Volume {
            controller,
            volumes: Vec::new(),
        }
    }
}

impl AC97Volume for SimpleAC97Volume {
    fn set_volume(&mut self, ac97_id: AC97ID, channel: u8, volume: u8) -> Result<(), AC97Error> {
        self.volumes.push((ac97_id, (AtomicUsize::new(channel as usize), AtomicUsize::new(volume as usize))));
        Ok(())
    }
    
    fn get_volume(&self, ac97_id: AC97ID, channel: u8) -> Result<u8, AC97Error> {
        for &(id, ref vol) in &self.volumes {
            if id == ac97_id && vol.0.load(Ordering::SeqCst) as u8 == channel {
                return Ok(vol.1.load(Ordering::SeqCst) as u8);
            }
        }
        Err(AC97Error::NotFound)
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
