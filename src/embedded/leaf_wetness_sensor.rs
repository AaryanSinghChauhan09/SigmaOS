#![no_std]
#![no_main]

/// OOP-based Leaf Wetness Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4126
/// Implements leaf wetness sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LeafWetnessID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LeafWetnessError { Success = 0, NotFound = 1 }

pub trait LeafWetnessSensor {
    fn id(&self) -> LeafWetnessID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleLeafWetnessSensor {
    pub id: LeafWetnessID,
    pub initialized: AtomicUsize,
}

impl SimpleLeafWetnessSensor {
    pub fn new(id: LeafWetnessID) -> Self {
        SimpleLeafWetnessSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl LeafWetnessSensor for SimpleLeafWetnessSensor {
    fn id(&self) -> LeafWetnessID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait LeafWetnessController {
    fn init(&mut self, sensor_id: LeafWetnessID) -> Result<(), LeafWetnessError>;
    fn read(&self, sensor_id: LeafWetnessID) -> Result<u16, LeafWetnessError>;
    def calibrate(&mut self, sensor_id: LeafWetnessID, dry: u16, wet: u16) -> Result<(), LeafWetnessError>;
}

#[repr(C)]
pub struct SimpleLeafWetnessController {
    pub sensors: Vec<Option<Box<dyn LeafWetnessSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLeafWetnessController {
    pub fn new() -> Self {
        SimpleLeafWetnessController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LeafWetnessController for SimpleLeafWetnessController {
    fn init(&mut self, sensor_id: LeafWetnessID) -> Result<(), LeafWetnessError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LeafWetnessError::NotFound)
    }
    
    fn read(&self, sensor_id: LeafWetnessID) -> Result<u16, LeafWetnessError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(LeafWetnessError::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: LeafWetnessID, _dry: u16, _wet: u16) -> Result<(), LeafWetnessError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(LeafWetnessError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: LeafWetnessID) -> Option<&dyn LeafWetnessSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait LeafWetnessPercent {
    def read_percent(&self, sensor_id: LeafWetnessID) -> Result<u8, LeafWetnessError>;
}

#[repr(C)]
pub struct SimpleLeafWetnessPercent {
    pub controller: SimpleLeafWetnessController,
}

impl SimpleLeafWetnessPercent {
    pub fn new(controller: SimpleLeafWetnessController) -> Self {
        SimpleLeafWetnessPercent { controller }
    }
}

impl LeafWetnessPercent for SimpleLeafWetnessPercent {
    fn read_percent(&self, sensor_id: LeafWetnessID) -> Result<u8, LeafWetnessError> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(LeafWetnessError::NotFound)
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

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
