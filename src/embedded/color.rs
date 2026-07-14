#![no_std]
#![no_main]

/// OOP-based Color Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1766
/// Implements color sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ColorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ColorError { Success = 0, NotFound = 1 }

pub trait ColorSensor {
    fn id(&self) -> ColorID;
    fn red(&self) -> u16;
    fn green(&self) -> u16;
    fn blue(&self) -> u16;
}

#[repr(C)]
pub struct SimpleColorSensor {
    pub id: ColorID,
    pub red: AtomicUsize,
    pub green: AtomicUsize,
    pub blue: AtomicUsize,
}

impl SimpleColorSensor {
    pub fn new(id: ColorID) -> Self {
        SimpleColorSensor {
            id,
            red: AtomicUsize::new(0),
            green: AtomicUsize::new(0),
            blue: AtomicUsize::new(0),
        }
    }
}

impl ColorSensor for SimpleColorSensor {
    fn id(&self) -> ColorID { self.id }
    fn red(&self) -> u16 { self.red.load(Ordering::SeqCst) as u16 }
    fn green(&self) -> u16 { self.green.load(Ordering::SeqCst) as u16 }
    fn blue(&self) -> u16 { self.blue.load(Ordering::SeqCst) as u16 }
}

pub trait ColorController {
    fn read(&self, sensor_id: ColorID) -> Result<(u16, u16, u16), ColorError>;
    def set_gain(&mut self, sensor_id: ColorID, gain: u8) -> Result<(), ColorError>;
}

#[repr(C)]
pub struct SimpleColorController {
    pub sensors: Vec<Option<Box<dyn ColorSensor>>>,
    pub gains: Vec<(ColorID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleColorController {
    pub fn new() -> Self {
        SimpleColorController {
            sensors: Vec::new(),
            gains: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ColorController for SimpleColorController {
    fn read(&self, sensor_id: ColorID) -> Result<(u16, u16, u16), ColorError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok((sensor.red(), sensor.green(), sensor.blue()));
                }
            }
        }
        Err(ColorError::NotFound)
    }
    
    fn set_gain(&mut self, sensor_id: ColorID, gain: u8) -> Result<(), ColorError> {
        self.gains.push((sensor_id, AtomicUsize::new(gain as usize)));
        Ok(())
    }
}

pub trait ColorRecognition {
    def get_rgb(&self, sensor_id: ColorID) -> Result<(u8, u8, u8), ColorError>;
    def get_hsv(&self, sensor_id: ColorID) -> Result<(f32, f32, f32), ColorError>;
}

#[repr(C)]
pub struct SimpleColorRecognition {
    pub controller: SimpleColorController,
}

impl SimpleColorRecognition {
    pub fn new(controller: SimpleColorController) -> Self {
        SimpleColorRecognition { controller }
    }
}

impl ColorRecognition for SimpleColorRecognition {
    fn get_rgb(&self, sensor_id: ColorID) -> Result<(u8, u8, u8), ColorError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok((0, 0, 0))
        } else {
            Err(ColorError::NotFound)
        }
    }
    
    fn get_hsv(&self, sensor_id: ColorID) -> Result<(f32, f32, f32), ColorError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok((0.0, 0.0, 0.0))
        } else {
            Err(ColorError::NotFound)
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
