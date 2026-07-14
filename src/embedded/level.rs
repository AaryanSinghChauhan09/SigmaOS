#![no_std]
#![no_main]

/// OOP-based Level Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1806
/// Implements level sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LevelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LevelError { Success = 0, NotFound = 1 }

pub trait LevelSensor {
    fn id(&self) -> LevelID;
    fn level(&self) -> f32;
}

#[repr(C)]
pub struct SimpleLevelSensor {
    pub id: LevelID,
    pub level: AtomicUsize,
}

impl SimpleLevelSensor {
    pub fn new(id: LevelID) -> Self {
        SimpleLevelSensor {
            id,
            level: AtomicUsize::new(0),
        }
    }
}

impl LevelSensor for SimpleLevelSensor {
    fn id(&self) -> LevelID { self.id }
    fn level(&self) -> f32 { self.level.load(Ordering::SeqCst) as f32 }
}

pub trait LevelController {
    fn read(&self, sensor_id: LevelID) -> Result<f32, LevelError>;
    def set_capacity(&mut self, sensor_id: LevelID, capacity: f32) -> Result<(), LevelError>;
}

#[repr(C)]
pub struct SimpleLevelController {
    pub sensors: Vec<Option<Box<dyn LevelSensor>>>,
    pub capacities: Vec<(LevelID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleLevelController {
    pub fn new() -> Self {
        SimpleLevelController {
            sensors: Vec::new(),
            capacities: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LevelController for SimpleLevelController {
    fn read(&self, sensor_id: LevelID) -> Result<f32, LevelError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.level());
                }
            }
        }
        Err(LevelError::NotFound)
    }
    
    fn set_capacity(&mut self, sensor_id: LevelID, capacity: f32) -> Result<(), LevelError> {
        self.capacities.push((sensor_id, AtomicUsize::new(capacity as usize)));
        Ok(())
    }
}

pub trait TankMonitor {
    def get_percentage(&self, sensor_id: LevelID) -> Result<f32, LevelError>;
    def is_empty(&self, sensor_id: LevelID) -> Result<bool, LevelError>;
}

#[repr(C)]
pub struct SimpleTankMonitor {
    pub controller: SimpleLevelController,
}

impl SimpleTankMonitor {
    pub fn new(controller: SimpleLevelController) -> Self {
        SimpleTankMonitor { controller }
    }
}

impl TankMonitor for SimpleTankMonitor {
    fn get_percentage(&self, sensor_id: LevelID) -> Result<f32, LevelError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(LevelError::NotFound)
        }
    }
    
    fn is_empty(&self, sensor_id: LevelID) -> Result<bool, LevelError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(LevelError::NotFound)
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
