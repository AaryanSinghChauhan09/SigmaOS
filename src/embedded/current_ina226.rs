#![no_std]
#![no_main]

/// OOP-based INA226 Current Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3356
/// Implements INA226 voltage/current/power monitor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type INA226ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum INA226Error { Success = 0, NotFound = 1 }

pub trait INA226Sensor {
    fn id(&self) -> INA226ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleINA226Sensor {
    pub id: INA226ID,
    pub initialized: AtomicUsize,
}

impl SimpleINA226Sensor {
    pub fn new(id: INA226ID) -> Self {
        SimpleINA226Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl INA226Sensor for SimpleINA226Sensor {
    fn id(&self) -> INA226ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait INA226Controller {
    fn init(&mut self, sensor_id: INA226ID) -> Result<(), INA226Error>;
    fn read_current(&self, sensor_id: INA226ID) -> Result<f32, INA226Error>;
    def read_voltage(&self, sensor_id: INA226ID) -> Result<f32, INA226Error>;
}

#[repr(C)]
pub struct SimpleINA226Controller {
    pub sensors: Vec<Option<Box<dyn INA226Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleINA226Controller {
    pub fn new() -> Self {
        SimpleINA226Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl INA226Controller for SimpleINA226Controller {
    fn init(&mut self, sensor_id: INA226ID) -> Result<(), INA226Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(INA226Error::NotFound)
    }
    
    fn read_current(&self, sensor_id: INA226ID) -> Result<f32, INA226Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(INA226Error::NotFound)
        }
    }
    
    fn read_voltage(&self, sensor_id: INA226ID) -> Result<f32, INA226Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(INA226Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: INA226ID) -> Option<&dyn INA226Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait INA226Power {
    def read_power(&self, sensor_id: INA226ID) -> Result<f32, INA226Error>;
}

#[repr(C)]
pub struct SimpleINA226Power {
    pub controller: SimpleINA226Controller,
}

impl SimpleINA226Power {
    pub fn new(controller: SimpleINA226Controller) -> Self {
        SimpleINA226Power { controller }
    }
}

impl INA226Power for SimpleINA226Power {
    fn read_power(&self, sensor_id: INA226ID) -> Result<f32, INA226Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(INA226Error::NotFound)
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
