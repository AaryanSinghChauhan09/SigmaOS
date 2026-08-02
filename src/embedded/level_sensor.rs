// (no_std only applicable at crate root - removed)
#![no_main]

/// OOP-based Level Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4056
/// Implements liquid level sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LevelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LevelError { Success = 0, NotFound = 1 }

pub trait LevelSensor {
    fn id(&self) -> LevelID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleLevelSensor {
    pub id: LevelID,
    pub initialized: AtomicUsize,
}

impl SimpleLevelSensor {
    pub fn new(id: LevelID) -> Self {
        SimpleLevelSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl LevelSensor for SimpleLevelSensor {
    fn id(&self) -> LevelID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait LevelController {
    fn init(&mut self, sensor_id: LevelID) -> Result<(), LevelError>;
    fn read(&self, sensor_id: LevelID) -> Result<u16, LevelError>;
    def set_threshold(&mut self, sensor_id: LevelID, threshold: u16) -> Result<(), LevelError>;
}

#[repr(C)]
pub struct SimpleLevelController {
    pub sensors: Vec<Option<Box<dyn LevelSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLevelController {
    pub fn new() -> Self {
        SimpleLevelController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LevelController for SimpleLevelController {
    fn init(&mut self, sensor_id: LevelID) -> Result<(), LevelError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LevelError::NotFound)
    }
    
    fn read(&self, sensor_id: LevelID) -> Result<u16, LevelError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(LevelError::NotFound)
        }
    }
    
    fn set_threshold(&mut self, sensor_id: LevelID, _threshold: u16) -> Result<(), LevelError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(LevelError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: LevelID) -> Option<&dyn LevelSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait LevelPercent {
    def read_percent(&self, sensor_id: LevelID) -> Result<u8, LevelError>;
}

#[repr(C)]
pub struct SimpleLevelPercent {
    pub controller: SimpleLevelController,
}

impl SimpleLevelPercent {
    pub fn new(controller: SimpleLevelController) -> Self {
        SimpleLevelPercent { controller }
    }
}

impl LevelPercent for SimpleLevelPercent {
    fn read_percent(&self, sensor_id: LevelID) -> Result<u8, LevelError> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
