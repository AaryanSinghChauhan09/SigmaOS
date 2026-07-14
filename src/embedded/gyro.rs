#![no_std]
#![no_main]

/// OOP-based Gyroscope for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1726
/// Implements gyroscope sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GyroID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GyroError { Success = 0, NotFound = 1 }

pub trait Gyroscope {
    fn id(&self) -> GyroID;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

#[repr(C)]
pub struct SimpleGyroscope {
    pub id: GyroID,
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub z: AtomicUsize,
}

impl SimpleGyroscope {
    pub fn new(id: GyroID) -> Self {
        SimpleGyroscope {
            id,
            x: AtomicUsize::new(0),
            y: AtomicUsize::new(0),
            z: AtomicUsize::new(0),
        }
    }
}

impl Gyroscope for SimpleGyroscope {
    fn id(&self) -> GyroID { self.id }
    fn x(&self) -> f32 { self.x.load(Ordering::SeqCst) as f32 }
    fn y(&self) -> f32 { self.y.load(Ordering::SeqCst) as f32 }
    fn z(&self) -> f32 { self.z.load(Ordering::SeqCst) as f32 }
}

pub trait GyroController {
    fn read(&self, gyro_id: GyroID) -> Result<(f32, f32, f32), GyroError>;
    def zero(&mut self, gyro_id: GyroID) -> Result<(), GyroError>;
}

#[repr(C)]
pub struct SimpleGyroController {
    pub gyros: Vec<Option<Box<dyn Gyroscope>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGyroController {
    pub fn new() -> Self {
        SimpleGyroController {
            gyros: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GyroController for SimpleGyroController {
    fn read(&self, gyro_id: GyroID) -> Result<(f32, f32, f32), GyroError> {
        for gyro_option in &self.gyros {
            if let Some(ref gyro) = *gyro_option {
                if gyro.id() == gyro_id {
                    return Ok((gyro.x(), gyro.y(), gyro.z()));
                }
            }
        }
        Err(GyroError::NotFound)
    }
    
    fn zero(&mut self, gyro_id: GyroID) -> Result<(), GyroError> {
        for gyro_option in &mut self.gyros {
            if let Some(ref mut gyro) = *gyro_option {
                if gyro.id() == gyro_id {
                    gyro.x.store(0, Ordering::SeqCst);
                    gyro.y.store(0, Ordering::SeqCst);
                    gyro.z.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(GyroError::NotFound)
    }
}

pub trait GyroIntegration {
    def integrate(&mut self, gyro_id: GyroID, dt: f32) -> Result<(f32, f32, f32), GyroError>;
    def reset_integration(&mut self, gyro_id: GyroID) -> Result<(), GyroError>;
}

#[repr(C)]
pub struct SimpleGyroIntegration {
    pub controller: SimpleGyroController,
    pub angles: Vec<(GyroID, (AtomicUsize, AtomicUsize, AtomicUsize))>,
}

impl SimpleGyroIntegration {
    pub fn new(controller: SimpleGyroController) -> Self {
        SimpleGyroIntegration {
            controller,
            angles: Vec::new(),
        }
    }
}

impl GyroIntegration for SimpleGyroIntegration {
    fn integrate(&mut self, gyro_id: GyroID, _dt: f32) -> Result<(f32, f32, f32), GyroError> {
        self.angles.push((gyro_id, (AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0))));
        Ok((0.0, 0.0, 0.0))
    }
    
    fn reset_integration(&mut self, _gyro_id: GyroID) -> Result<(), GyroError> {
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
