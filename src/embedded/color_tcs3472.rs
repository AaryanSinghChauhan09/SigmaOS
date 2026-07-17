#![no_std]
#![no_main]

/// OOP-based TCS3472 Color Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4136
/// Implements TCS3472 RGB color sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TCS3472ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TCS3472Error { Success = 0, NotFound = 1 }

pub trait TCS3472Sensor {
    fn id(&self) -> TCS3472ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTCS3472Sensor {
    pub id: TCS3472ID,
    pub initialized: AtomicUsize,
}

impl SimpleTCS3472Sensor {
    pub fn new(id: TCS3472ID) -> Self {
        SimpleTCS3472Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TCS3472Sensor for SimpleTCS3472Sensor {
    fn id(&self) -> TCS3472ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TCS3472Controller {
    fn init(&mut self, sensor_id: TCS3472ID) -> Result<(), TCS3472Error>;
    fn read_rgb(&self, sensor_id: TCS3472ID) -> Result<(u16, u16, u16), TCS3472Error>;
    def set_gain(&mut self, sensor_id: TCS3472ID, gain: u8) -> Result<(), TCS3472Error>;
}

#[repr(C)]
pub struct SimpleTCS3472Controller {
    pub sensors: Vec<Option<Box<dyn TCS3472Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTCS3472Controller {
    pub fn new() -> Self {
        SimpleTCS3472Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TCS3472Controller for SimpleTCS3472Controller {
    fn init(&mut self, sensor_id: TCS3472ID) -> Result<(), TCS3472Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TCS3472Error::NotFound)
    }
    
    fn read_rgb(&self, sensor_id: TCS3472ID) -> Result<(u16, u16, u16), TCS3472Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(TCS3472Error::NotFound)
        }
    }
    
    fn set_gain(&mut self, sensor_id: TCS3472ID, _gain: u8) -> Result<(), TCS3472Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(TCS3472Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: TCS3472ID) -> Option<&dyn TCS3472Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait TCS3472Clear {
    def read_clear(&self, sensor_id: TCS3472ID) -> Result<u16, TCS3472Error>;
}

#[repr(C)]
pub struct SimpleTCS3472Clear {
    pub controller: SimpleTCS3472Controller,
}

impl SimpleTCS3472Clear {
    pub fn new(controller: SimpleTCS3472Controller) -> Self {
        SimpleTCS3472Clear { controller }
    }
}

impl TCS3472Clear for SimpleTCS3472Clear {
    fn read_clear(&self, sensor_id: TCS3472ID) -> Result<u16, TCS3472Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(TCS3472Error::NotFound)
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
