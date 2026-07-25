#![no_std]
#![no_main]

/// OOP-based IMU for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1736
/// Implements IMU (Inertial Measurement Unit)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type IMUID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IMUError { Success = 0, NotFound = 1 }

pub trait IMU {
    fn id(&self) -> IMUID;
    fn accel_x(&self) -> f32;
    fn gyro_x(&self) -> f32;
}

#[repr(C)]
pub struct SimpleIMU {
    pub id: IMUID,
    pub accel_x: AtomicUsize,
    pub gyro_x: AtomicUsize,
}

impl SimpleIMU {
    pub fn new(id: IMUID) -> Self {
        SimpleIMU {
            id,
            accel_x: AtomicUsize::new(0),
            gyro_x: AtomicUsize::new(0),
        }
    }
}

impl IMU for SimpleIMU {
    fn id(&self) -> IMUID { self.id }
    fn accel_x(&self) -> f32 { self.accel_x.load(Ordering::SeqCst) as f32 }
    fn gyro_x(&self) -> f32 { self.gyro_x.load(Ordering::SeqCst) as f32 }
}

pub trait IMUController {
    fn read_accel(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError>;
    fn read_gyro(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError>;
    def read_mag(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError>;
}

#[repr(C)]
pub struct SimpleIMUController {
    pub imus: Vec<Option<Box<dyn IMU>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIMUController {
    pub fn new() -> Self {
        SimpleIMUController {
            imus: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IMUController for SimpleIMUController {
    fn read_accel(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError> {
        if self.get_imu(imu_id).is_some() {
            Ok((0.0, 0.0, 0.0))
        } else {
            Err(IMUError::NotFound)
        }
    }
    
    fn read_gyro(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError> {
        if self.get_imu(imu_id).is_some() {
            Ok((0.0, 0.0, 0.0))
        } else {
            Err(IMUError::NotFound)
        }
    }
    
    fn read_mag(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError> {
        if self.get_imu(imu_id).is_some() {
            Ok((0.0, 0.0, 0.0))
        } else {
            Err(IMUError::NotFound)
        }
    }
    
    fn get_imu(&self, id: IMUID) -> Option<&dyn IMU> {
        for imu_option in &self.imus {
            if let Some(ref imu) = *imu_option {
                if imu.id() == id { return Some(imu.as_ref()); }
            }
        }
        None
    }
}

pub trait SensorFusion {
    def update_fusion(&mut self, imu_id: IMUID) -> Result<(), IMUError>;
    def get_orientation(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError>;
}

#[repr(C)]
pub struct SimpleSensorFusion {
    pub controller: SimpleIMUController,
    pub orientations: Vec<(IMUID, (AtomicUsize, AtomicUsize, AtomicUsize))>,
}

impl SimpleSensorFusion {
    pub fn new(controller: SimpleIMUController) -> Self {
        SimpleSensorFusion {
            controller,
            orientations: Vec::new(),
        }
    }
}

impl SensorFusion for SimpleSensorFusion {
    fn update_fusion(&mut self, imu_id: IMUID) -> Result<(), IMUError> {
        self.orientations.push((imu_id, (AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0))));
        Ok(())
    }
    
    fn get_orientation(&self, imu_id: IMUID) -> Result<(f32, f32, f32), IMUError> {
        if self.controller.get_imu(imu_id).is_some() {
            Ok((0.0, 0.0, 0.0))
        } else {
            Err(IMUError::NotFound)
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
