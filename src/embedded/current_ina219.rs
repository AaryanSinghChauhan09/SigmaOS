#![no_std]
#![no_main]

/// OOP-based INA219 Current Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3346
/// Implements INA219 voltage/current/power monitor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type INA219ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum INA219Error { Success = 0, NotFound = 1 }

pub trait INA219Sensor {
    fn id(&self) -> INA219ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleINA219Sensor {
    pub id: INA219ID,
    pub initialized: AtomicUsize,
}

impl SimpleINA219Sensor {
    pub fn new(id: INA219ID) -> Self {
        SimpleINA219Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl INA219Sensor for SimpleINA219Sensor {
    fn id(&self) -> INA219ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait INA219Controller {
    fn init(&mut self, sensor_id: INA219ID) -> Result<(), INA219Error>;
    fn read_current(&self, sensor_id: INA219ID) -> Result<f32, INA219Error>;
    def read_voltage(&self, sensor_id: INA219ID) -> Result<f32, INA219Error>;
}

#[repr(C)]
pub struct SimpleINA219Controller {
    pub sensors: Vec<Option<Box<dyn INA219Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleINA219Controller {
    pub fn new() -> Self {
        SimpleINA219Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl INA219Controller for SimpleINA219Controller {
    fn init(&mut self, sensor_id: INA219ID) -> Result<(), INA219Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(INA219Error::NotFound)
    }
    
    fn read_current(&self, sensor_id: INA219ID) -> Result<f32, INA219Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(INA219Error::NotFound)
        }
    }
    
    fn read_voltage(&self, sensor_id: INA219ID) -> Result<f32, INA219Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(INA219Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: INA219ID) -> Option<&dyn INA219Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait INA219Power {
    def read_power(&self, sensor_id: INA219ID) -> Result<f32, INA219Error>;
}

#[repr(C)]
pub struct SimpleINA219Power {
    pub controller: SimpleINA219Controller,
}

impl SimpleINA219Power {
    pub fn new(controller: SimpleINA219Controller) -> Self {
        SimpleINA219Power { controller }
    }
}

impl INA219Power for SimpleINA219Power {
    fn read_power(&self, sensor_id: INA219ID) -> Result<f32, INA219Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(INA219Error::NotFound)
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
