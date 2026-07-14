#![no_std]
#![no_main]

/// OOP-based PIR Motion for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3826
/// Implements PIR motion sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PIRID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PIRError { Success = 0, NotFound = 1 }

pub trait PIRSensor {
    fn id(&self) -> PIRID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePIRSensor {
    pub id: PIRID,
    pub initialized: AtomicUsize,
}

impl SimplePIRSensor {
    pub fn new(id: PIRID) -> Self {
        SimplePIRSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PIRSensor for SimplePIRSensor {
    fn id(&self) -> PIRID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PIRController {
    fn init(&mut self, sensor_id: PIRID) -> Result<(), PIRError>;
    fn read(&self, sensor_id: PIRID) -> Result<bool, PIRError>;
    def set_sensitivity(&mut self, sensor_id: PIRID, level: u8) -> Result<(), PIRError>;
}

#[repr(C)]
pub struct SimplePIRController {
    pub sensors: Vec<Option<Box<dyn PIRSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimplePIRController {
    pub fn new() -> Self {
        SimplePIRController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PIRController for SimplePIRController {
    fn init(&mut self, sensor_id: PIRID) -> Result<(), PIRError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PIRError::NotFound)
    }
    
    fn read(&self, sensor_id: PIRID) -> Result<bool, PIRError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(PIRError::NotFound)
        }
    }
    
    fn set_sensitivity(&mut self, sensor_id: PIRID, _level: u8) -> Result<(), PIRError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(PIRError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: PIRID) -> Option<&dyn PIRSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait PIRDelay {
    def set_delay(&mut self, sensor_id: PIRID, delay_ms: u16) -> Result<(), PIRError>;
}

#[repr(C)]
pub struct SimplePIRDelay {
    pub controller: SimplePIRController,
    pub delays: Vec<(PIRID, AtomicUsize)>,
}

impl SimplePIRDelay {
    pub fn new(controller: SimplePIRController) -> Self {
        SimplePIRDelay {
            controller,
            delays: Vec::new(),
        }
    }
}

impl PIRDelay for SimplePIRDelay {
    fn set_delay(&mut self, sensor_id: PIRID, delay_ms: u16) -> Result<(), PIRError> {
        self.delays.push((sensor_id, AtomicUsize::new(delay_ms as usize)));
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
