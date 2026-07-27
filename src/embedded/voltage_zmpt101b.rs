#![no_std]
#![no_main]

/// OOP-based ZMPT101B Voltage Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3366
/// Implements ZMPT101B AC voltage sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ZMPT101BID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ZMPT101BError { Success = 0, NotFound = 1 }

pub trait ZMPT101BSensor {
    fn id(&self) -> ZMPT101BID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleZMPT101BSensor {
    pub id: ZMPT101BID,
    pub initialized: AtomicUsize,
}

impl SimpleZMPT101BSensor {
    pub fn new(id: ZMPT101BID) -> Self {
        SimpleZMPT101BSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ZMPT101BSensor for SimpleZMPT101BSensor {
    fn id(&self) -> ZMPT101BID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ZMPT101BController {
    fn init(&mut self, sensor_id: ZMPT101BID) -> Result<(), ZMPT101BError>;
    fn read_voltage(&self, sensor_id: ZMPT101BID) -> Result<f32, ZMPT101BError>;
    def calibrate(&mut self, sensor_id: ZMPT101BID) -> Result<(), ZMPT101BError>;
}

#[repr(C)]
pub struct SimpleZMPT101BController {
    pub sensors: Vec<Option<Box<dyn ZMPT101BSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleZMPT101BController {
    pub fn new() -> Self {
        SimpleZMPT101BController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ZMPT101BController for SimpleZMPT101BController {
    fn init(&mut self, sensor_id: ZMPT101BID) -> Result<(), ZMPT101BError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ZMPT101BError::NotFound)
    }
    
    fn read_voltage(&self, sensor_id: ZMPT101BID) -> Result<f32, ZMPT101BError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(ZMPT101BError::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: ZMPT101BID) -> Result<(), ZMPT101BError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(ZMPT101BError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: ZMPT101BID) -> Option<&dyn ZMPT101BSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait ZMPT101BRMS {
    def read_rms(&self, sensor_id: ZMPT101BID) -> Result<f32, ZMPT101BError>;
}

#[repr(C)]
pub struct SimpleZMPT101BRMS {
    pub controller: SimpleZMPT101BController,
}

impl SimpleZMPT101BRMS {
    pub fn new(controller: SimpleZMPT101BController) -> Self {
        SimpleZMPT101BRMS { controller }
    }
}

impl ZMPT101BRMS for SimpleZMPT101BRMS {
    fn read_rms(&self, sensor_id: ZMPT101BID) -> Result<f32, ZMPT101BError> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(ZMPT101BError::NotFound)
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
