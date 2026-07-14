#![no_std]
#![no_main]

/// OOP-based JSN-SR04T Ultrasonic for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3856
/// Implements JSN-SR04T waterproof ultrasonic sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type JSNSR04TID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JSNSR04TError { Success = 0, NotFound = 1 }

pub trait JSNSR04TSensor {
    fn id(&self) -> JSNSR04TID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleJSNSR04TSensor {
    pub id: JSNSR04TID,
    pub initialized: AtomicUsize,
}

impl SimpleJSNSR04TSensor {
    pub fn new(id: JSNSR04TID) -> Self {
        SimpleJSNSR04TSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl JSNSR04TSensor for SimpleJSNSR04TSensor {
    fn id(&self) -> JSNSR04TID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait JSNSR04TController {
    fn init(&mut self, sensor_id: JSNSR04TID) -> Result<(), JSNSR04TError>;
    fn read_distance(&self, sensor_id: JSNSR04TID) -> Result<u16, JSNSR04TError>;
    def set_mode(&mut self, sensor_id: JSNSR04TID, mode: u8) -> Result<(), JSNSR04TError>;
}

#[repr(C)]
pub struct SimpleJSNSR04TController {
    pub sensors: Vec<Option<Box<dyn JSNSR04TSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleJSNSR04TController {
    pub fn new() -> Self {
        SimpleJSNSR04TController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl JSNSR04TController for SimpleJSNSR04TController {
    fn init(&mut self, sensor_id: JSNSR04TID) -> Result<(), JSNSR04TError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(JSNSR04TError::NotFound)
    }
    
    fn read_distance(&self, sensor_id: JSNSR04TID) -> Result<u16, JSNSR04TError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(JSNSR04TError::NotFound)
        }
    }
    
    fn set_mode(&mut self, sensor_id: JSNSR04TID, _mode: u8) -> Result<(), JSNSR04TError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(JSNSR04TError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: JSNSR04TID) -> Option<&dyn JSNSR04TSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait JSNSR04TFilter {
    def enable_filter(&mut self, sensor_id: JSNSR04TID, enable: bool) -> Result<(), JSNSR04TError>;
}

#[repr(C)]
pub struct SimpleJSNSR04TFilter {
    pub controller: SimpleJSNSR04TController,
    pub filters: Vec<(JSNSR04TID, AtomicUsize)>,
}

impl SimpleJSNSR04TFilter {
    pub fn new(controller: SimpleJSNSR04TController) -> Self {
        SimpleJSNSR04TFilter {
            controller,
            filters: Vec::new(),
        }
    }
}

impl JSNSR04TFilter for SimpleJSNSR04TFilter {
    fn enable_filter(&mut self, sensor_id: JSNSR04TID, enable: bool) -> Result<(), JSNSR04TError> {
        self.filters.push((sensor_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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
