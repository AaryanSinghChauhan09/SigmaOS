#![no_std]
#![no_main]

/// OOP-based MPU9250 IMU for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3126
/// Implements MPU9250 9-axis IMU

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MPU9250ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MPU9250Error { Success = 0, NotFound = 1 }

pub trait MPU9250IMU {
    fn id(&self) -> MPU9250ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMPU9250IMU {
    pub id: MPU9250ID,
    pub initialized: AtomicUsize,
}

impl SimpleMPU9250IMU {
    pub fn new(id: MPU9250ID) -> Self {
        SimpleMPU9250IMU {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MPU9250IMU for SimpleMPU9250IMU {
    fn id(&self) -> MPU9250ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MPU9250Controller {
    fn init(&mut self, imu_id: MPU9250ID) -> Result<(), MPU9250Error>;
    fn read_accel(&self, imu_id: MPU9250ID) -> Result<(i16, i16, i16), MPU9250Error>;
    def read_gyro(&self, imu_id: MPU9250ID) -> Result<(i16, i16, i16), MPU9250Error>;
}

#[repr(C)]
pub struct SimpleMPU9250Controller {
    pub imus: Vec<Option<Box<dyn MPU9250IMU>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMPU9250Controller {
    pub fn new() -> Self {
        SimpleMPU9250Controller {
            imus: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MPU9250Controller for SimpleMPU9250Controller {
    fn init(&mut self, imu_id: MPU9250ID) -> Result<(), MPU9250Error> {
        for imu_option in &mut self.imus {
            if let Some(ref mut imu) = *imu_option {
                if imu.id() == imu_id {
                    imu.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MPU9250Error::NotFound)
    }
    
    fn read_accel(&self, imu_id: MPU9250ID) -> Result<(i16, i16, i16), MPU9250Error> {
        if self.get_imu(imu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(MPU9250Error::NotFound)
        }
    }
    
    fn read_gyro(&self, imu_id: MPU9250ID) -> Result<(i16, i16, i16), MPU9250Error> {
        if self.get_imu(imu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(MPU9250Error::NotFound)
        }
    }
    
    fn get_imu(&self, id: MPU9250ID) -> Option<&dyn MPU9250IMU> {
        for imu_option in &self.imus {
            if let Some(ref imu) = *imu_option {
                if imu.id() == id { return Some(imu.as_ref()); }
            }
        }
        None
    }
}

pub trait MPU9250Mag {
    def read_mag(&self, imu_id: MPU9250ID) -> Result<(i16, i16, i16), MPU9250Error>;
}

#[repr(C)]
pub struct SimpleMPU9250Mag {
    pub controller: SimpleMPU9250Controller,
}

impl SimpleMPU9250Mag {
    pub fn new(controller: SimpleMPU9250Controller) -> Self {
        SimpleMPU9250Mag { controller }
    }
}

impl MPU9250Mag for SimpleMPU9250Mag {
    fn read_mag(&self, imu_id: MPU9250ID) -> Result<(i16, i16, i16), MPU9250Error> {
        if self.controller.get_imu(imu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(MPU9250Error::NotFound)
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
