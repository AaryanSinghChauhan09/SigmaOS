#![no_std]
#![no_main]

/// OOP-based BH1750 Light for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3196
/// Implements BH1750 light sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BH1750ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BH1750Error { Success = 0, NotFound = 1 }

pub trait BH1750Sensor {
    fn id(&self) -> BH1750ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBH1750Sensor {
    pub id: BH1750ID,
    pub initialized: AtomicUsize,
}

impl SimpleBH1750Sensor {
    pub fn new(id: BH1750ID) -> Self {
        SimpleBH1750Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl BH1750Sensor for SimpleBH1750Sensor {
    fn id(&self) -> BH1750ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait BH1750Controller {
    fn init(&mut self, sensor_id: BH1750ID) -> Result<(), BH1750Error>;
    fn read(&self, sensor_id: BH1750ID) -> Result<u16, BH1750Error>;
    def set_mode(&mut self, sensor_id: BH1750ID, mode: u8) -> Result<(), BH1750Error>;
}

#[repr(C)]
pub struct SimpleBH1750Controller {
    pub sensors: Vec<Option<Box<dyn BH1750Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBH1750Controller {
    pub fn new() -> Self {
        SimpleBH1750Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BH1750Controller for SimpleBH1750Controller {
    fn init(&mut self, sensor_id: BH1750ID) -> Result<(), BH1750Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BH1750Error::NotFound)
    }
    
    fn read(&self, sensor_id: BH1750ID) -> Result<u16, BH1750Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BH1750Error::NotFound)
        }
    }
    
    fn set_mode(&mut self, _sensor_id: BH1750ID, _mode: u8) -> Result<(), BH1750Error> {
        Ok(())
    }
    
    fn get_sensor(&self, id: BH1750ID) -> Option<&dyn BH1750Sensor> {
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
