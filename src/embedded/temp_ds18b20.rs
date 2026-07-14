#![no_std]
#![no_main]

/// OOP-based DS18B20 Temperature for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3756
/// Implements DS18B20 digital temperature sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DS18B20ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DS18B20Error { Success = 0, NotFound = 1 }

pub trait DS18B20Sensor {
    fn id(&self) -> DS18B20ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDS18B20Sensor {
    pub id: DS18B20ID,
    pub initialized: AtomicUsize,
}

impl SimpleDS18B20Sensor {
    pub fn new(id: DS18B20ID) -> Self {
        SimpleDS18B20Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DS18B20Sensor for SimpleDS18B20Sensor {
    fn id(&self) -> DS18B20ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DS18B20Controller {
    fn init(&mut self, sensor_id: DS18B20ID) -> Result<(), DS18B20Error>;
    fn read(&self, sensor_id: DS18B20ID) -> Result<f32, DS18B20Error>;
    def start_conversion(&self, sensor_id: DS18B20ID) -> Result<(), DS18B20Error>;
}

#[repr(C)]
pub struct SimpleDS18B20Controller {
    pub sensors: Vec<Option<Box<dyn DS18B20Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDS18B20Controller {
    pub fn new() -> Self {
        SimpleDS18B20Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DS18B20Controller for SimpleDS18B20Controller {
    fn init(&mut self, sensor_id: DS18B20ID) -> Result<(), DS18B20Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DS18B20Error::NotFound)
    }
    
    fn read(&self, sensor_id: DS18B20ID) -> Result<f32, DS18B20Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(DS18B20Error::NotFound)
        }
    }
    
    fn start_conversion(&self, sensor_id: DS18B20ID) -> Result<(), DS18B20Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(DS18B20Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: DS18B20ID) -> Option<&dyn DS18B20Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait DS18B20Resolution {
    def set_resolution(&mut self, sensor_id: DS18B20ID, bits: u8) -> Result<(), DS18B20Error>;
}

#[repr(C)]
pub struct SimpleDS18B20Resolution {
    pub controller: SimpleDS18B20Controller,
    pub resolutions: Vec<(DS18B20ID, AtomicUsize)>,
}

impl SimpleDS18B20Resolution {
    pub fn new(controller: SimpleDS18B20Controller) -> Self {
        SimpleDS18B20Resolution {
            controller,
            resolutions: Vec::new(),
        }
    }
}

impl DS18B20Resolution for SimpleDS18B20Resolution {
    fn set_resolution(&mut self, sensor_id: DS18B20ID, bits: u8) -> Result<(), DS18B20Error> {
        self.resolutions.push((sensor_id, AtomicUsize::new(bits as usize)));
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
