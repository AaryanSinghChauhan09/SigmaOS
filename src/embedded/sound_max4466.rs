#![no_std]
#![no_main]

/// OOP-based MAX4466 Sound Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3386
/// Implements MAX4466 microphone amplifier

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MAX4466ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MAX4466Error { Success = 0, NotFound = 1 }

pub trait MAX4466Sensor {
    fn id(&self) -> MAX4466ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMAX4466Sensor {
    pub id: MAX4466ID,
    pub initialized: AtomicUsize,
}

impl SimpleMAX4466Sensor {
    pub fn new(id: MAX4466ID) -> Self {
        SimpleMAX4466Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MAX4466Sensor for SimpleMAX4466Sensor {
    fn id(&self) -> MAX4466ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MAX4466Controller {
    fn init(&mut self, sensor_id: MAX4466ID) -> Result<(), MAX4466Error>;
    fn read(&self, sensor_id: MAX4466ID) -> Result<u16, MAX4466Error>;
    def set_gain(&mut self, sensor_id: MAX4466ID, gain: u8) -> Result<(), MAX4466Error>;
}

#[repr(C)]
pub struct SimpleMAX4466Controller {
    pub sensors: Vec<Option<Box<dyn MAX4466Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMAX4466Controller {
    pub fn new() -> Self {
        SimpleMAX4466Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MAX4466Controller for SimpleMAX4466Controller {
    fn init(&mut self, sensor_id: MAX4466ID) -> Result<(), MAX4466Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MAX4466Error::NotFound)
    }
    
    fn read(&self, sensor_id: MAX4466ID) -> Result<u16, MAX4466Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(MAX4466Error::NotFound)
        }
    }
    
    fn set_gain(&mut self, _sensor_id: MAX4466ID, _gain: u8) -> Result<(), MAX4466Error> {
        Ok(())
    }
    
    fn get_sensor(&self, id: MAX4466ID) -> Option<&dyn MAX4466Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
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
