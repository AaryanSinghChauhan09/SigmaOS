#![no_std]
#![no_main]

/// OOP-based MPU6050 IMU for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3116
/// Implements MPU6050 6-axis IMU

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MPU6050ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MPU6050Error { Success = 0, NotFound = 1 }

pub trait MPU6050IMU {
    fn id(&self) -> MPU6050ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMPU6050IMU {
    pub id: MPU6050ID,
    pub initialized: AtomicUsize,
}

impl SimpleMPU6050IMU {
    pub fn new(id: MPU6050ID) -> Self {
        SimpleMPU6050IMU {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MPU6050IMU for SimpleMPU6050IMU {
    fn id(&self) -> MPU6050ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MPU6050Controller {
    fn init(&mut self, imu_id: MPU6050ID) -> Result<(), MPU6050Error>;
    fn read_accel(&self, imu_id: MPU6050ID) -> Result<(i16, i16, i16), MPU6050Error>;
    def read_gyro(&self, imu_id: MPU6050ID) -> Result<(i16, i16, i16), MPU6050Error>;
}

#[repr(C)]
pub struct SimpleMPU6050Controller {
    pub imus: Vec<Option<Box<dyn MPU6050IMU>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMPU6050Controller {
    pub fn new() -> Self {
        SimpleMPU6050Controller {
            imus: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MPU6050Controller for SimpleMPU6050Controller {
    fn init(&mut self, imu_id: MPU6050ID) -> Result<(), MPU6050Error> {
        for imu_option in &mut self.imus {
            if let Some(ref mut imu) = *imu_option {
                if imu.id() == imu_id {
                    imu.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MPU6050Error::NotFound)
    }
    
    fn read_accel(&self, imu_id: MPU6050ID) -> Result<(i16, i16, i16), MPU6050Error> {
        if self.get_imu(imu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(MPU6050Error::NotFound)
        }
    }
    
    fn read_gyro(&self, imu_id: MPU6050ID) -> Result<(i16, i16, i16), MPU6050Error> {
        if self.get_imu(imu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(MPU6050Error::NotFound)
        }
    }
    
    fn get_imu(&self, id: MPU6050ID) -> Option<&dyn MPU6050IMU> {
        for imu_option in &self.imus {
            if let Some(ref imu) = *imu_option {
                if imu.id() == id { return Some(imu.as_ref()); }
            }
        }
        None
    }
}

pub trait MPU6050Temp {
    def read_temp(&self, imu_id: MPU6050ID) -> Result<i16, MPU6050Error>;
}

#[repr(C)]
pub struct SimpleMPU6050Temp {
    pub controller: SimpleMPU6050Controller,
}

impl SimpleMPU6050Temp {
    pub fn new(controller: SimpleMPU6050Controller) -> Self {
        SimpleMPU6050Temp { controller }
    }
}

impl MPU6050Temp for SimpleMPU6050Temp {
    fn read_temp(&self, imu_id: MPU6050ID) -> Result<i16, MPU6050Error> {
        if self.controller.get_imu(imu_id).is_some() {
            Ok(0)
        } else {
            Err(MPU6050Error::NotFound)
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
