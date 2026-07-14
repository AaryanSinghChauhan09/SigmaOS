#![no_std]
#![no_main]

/// OOP-based VL53L0X Proximity for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3206
/// Implements VL53L0X time-of-flight distance sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type VL53L0XID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VL53L0XError { Success = 0, NotFound = 1 }

pub trait VL53L0XSensor {
    fn id(&self) -> VL53L0XID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleVL53L0XSensor {
    pub id: VL53L0XID,
    pub initialized: AtomicUsize,
}

impl SimpleVL53L0XSensor {
    pub fn new(id: VL53L0XID) -> Self {
        SimpleVL53L0XSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl VL53L0XSensor for SimpleVL53L0XSensor {
    fn id(&self) -> VL53L0XID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait VL53L0XController {
    fn init(&mut self, sensor_id: VL53L0XID) -> Result<(), VL53L0XError>;
    fn read_distance(&self, sensor_id: VL53L0XID) -> Result<u16, VL53L0XError>;
    def set_mode(&mut self, sensor_id: VL53L0XID, mode: u8) -> Result<(), VL53L0XError>;
}

#[repr(C)]
pub struct SimpleVL53L0XController {
    pub sensors: Vec<Option<Box<dyn VL53L0XSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleVL53L0XController {
    pub fn new() -> Self {
        SimpleVL53L0XController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VL53L0XController for SimpleVL53L0XController {
    fn init(&mut self, sensor_id: VL53L0XID) -> Result<(), VL53L0XError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(VL53L0XError::NotFound)
    }
    
    fn read_distance(&self, sensor_id: VL53L0XID) -> Result<u16, VL53L0XError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(VL53L0XError::NotFound)
        }
    }
    
    fn set_mode(&mut self, _sensor_id: VL53L0XID, _mode: u8) -> Result<(), VL53L0XError> {
        Ok(())
    }
    
    fn get_sensor(&self, id: VL53L0XID) -> Option<&dyn VL53L0XSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait VL53L0XConfig {
    def set_timing_budget(&mut self, sensor_id: VL53L0XID, budget: u32) -> Result<(), VL53L0XError>;
    def get_timing_budget(&self, sensor_id: VL53L0XID) -> Result<u32, VL53L0XError>;
}

#[repr(C)]
pub struct SimpleVL53L0XConfig {
    pub controller: SimpleVL53L0XController,
    pub budgets: Vec<(VL53L0XID, AtomicUsize)>,
}

impl SimpleVL53L0XConfig {
    pub fn new(controller: SimpleVL53L0XController) -> Self {
        SimpleVL53L0XConfig {
            controller,
            budgets: Vec::new(),
        }
    }
}

impl VL53L0XConfig for SimpleVL53L0XConfig {
    fn set_timing_budget(&mut self, sensor_id: VL53L0XID, budget: u32) -> Result<(), VL53L0XError> {
        self.budgets.push((sensor_id, AtomicUsize::new(budget as usize)));
        Ok(())
    }
    
    fn get_timing_budget(&self, sensor_id: VL53L0XID) -> Result<u32, VL53L0XError> {
        for &(id, ref budget) in &self.budgets {
            if id == sensor_id {
                return Ok(budget.load(Ordering::SeqCst) as u32);
            }
        }
        Err(VL53L0XError::NotFound)
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
