#![no_std]
#![no_main]

/// OOP-based MS5611 Pressure for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3796
/// Implements MS5611 barometric pressure sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MS5611ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MS5611Error { Success = 0, NotFound = 1 }

pub trait MS5611Sensor {
    fn id(&self) -> MS5611ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMS5611Sensor {
    pub id: MS5611ID,
    pub initialized: AtomicUsize,
}

impl SimpleMS5611Sensor {
    pub fn new(id: MS5611ID) -> Self {
        SimpleMS5611Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MS5611Sensor for SimpleMS5611Sensor {
    fn id(&self) -> MS5611ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MS5611Controller {
    fn init(&mut self, sensor_id: MS5611ID) -> Result<(), MS5611Error>;
    fn read_pressure(&self, sensor_id: MS5611ID) -> Result<u32, MS5611Error>;
    def read_temp(&self, sensor_id: MS5611ID) -> Result<i16, MS5611Error>;
}

#[repr(C)]
pub struct SimpleMS5611Controller {
    pub sensors: Vec<Option<Box<dyn MS5611Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMS5611Controller {
    pub fn new() -> Self {
        SimpleMS5611Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MS5611Controller for SimpleMS5611Controller {
    fn init(&mut self, sensor_id: MS5611ID) -> Result<(), MS5611Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MS5611Error::NotFound)
    }
    
    fn read_pressure(&self, sensor_id: MS5611ID) -> Result<u32, MS5611Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(MS5611Error::NotFound)
        }
    }
    
    fn read_temp(&self, sensor_id: MS5611ID) -> Result<i16, MS5611Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(MS5611Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: MS5611ID) -> Option<&dyn MS5611Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait MS5611Oversampling {
    def set_oversampling(&mut self, sensor_id: MS5611ID, osr: u8) -> Result<(), MS5611Error>;
}

#[repr(C)]
pub struct SimpleMS5611Oversampling {
    pub controller: SimpleMS5611Controller,
    pub osrs: Vec<(MS5611ID, AtomicUsize)>,
}

impl SimpleMS5611Oversampling {
    pub fn new(controller: SimpleMS5611Controller) -> Self {
        SimpleMS5611Oversampling {
            controller,
            osrs: Vec::new(),
        }
    }
}

impl MS5611Oversampling for SimpleMS5611Oversampling {
    fn set_oversampling(&mut self, sensor_id: MS5611ID, osr: u8) -> Result<(), MS5611Error> {
        self.osrs.push((sensor_id, AtomicUsize::new(osr as usize)));
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
