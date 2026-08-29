#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

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
    #[allow(clippy::new_without_default)]
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
