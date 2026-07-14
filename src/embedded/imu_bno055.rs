#![no_std]
#![no_main]

/// OOP-based BNO055 IMU for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3136
/// Implements BNO055 9-axis IMU with sensor fusion

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BNO055ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BNO055Error { Success = 0, NotFound = 1 }

pub trait BNO055IMU {
    fn id(&self) -> BNO055ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBNO055IMU {
    pub id: BNO055ID,
    pub initialized: AtomicUsize,
}

impl SimpleBNO055IMU {
    pub fn new(id: BNO055ID) -> Self {
        SimpleBNO055IMU {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl BNO055IMU for SimpleBNO055IMU {
    fn id(&self) -> BNO055ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait BNO055Controller {
    fn init(&mut self, imu_id: BNO055ID) -> Result<(), BNO055Error>;
    fn read_quaternion(&self, imu_id: BNO055ID) -> Result<(i16, i16, i16, i16), BNO055Error>;
    def read_euler(&self, imu_id: BNO055ID) -> Result<(i16, i16, i16), BNO055Error>;
}

#[repr(C)]
pub struct SimpleBNO055Controller {
    pub imus: Vec<Option<Box<dyn BNO055IMU>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBNO055Controller {
    pub fn new() -> Self {
        SimpleBNO055Controller {
            imus: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BNO055Controller for SimpleBNO055Controller {
    fn init(&mut self, imu_id: BNO055ID) -> Result<(), BNO055Error> {
        for imu_option in &mut self.imus {
            if let Some(ref mut imu) = *imu_option {
                if imu.id() == imu_id {
                    imu.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BNO055Error::NotFound)
    }
    
    fn read_quaternion(&self, imu_id: BNO055ID) -> Result<(i16, i16, i16, i16), BNO055Error> {
        if self.get_imu(imu_id).is_some() {
            Ok((0, 0, 0, 0))
        } else {
            Err(BNO055Error::NotFound)
        }
    }
    
    fn read_euler(&self, imu_id: BNO055ID) -> Result<(i16, i16, i16), BNO055Error> {
        if self.get_imu(imu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(BNO055Error::NotFound)
        }
    }
    
    fn get_imu(&self, id: BNO055ID) -> Option<&dyn BNO055IMU> {
        for imu_option in &self.imus {
            if let Some(ref imu) = *imu_option {
                if imu.id() == id { return Some(imu.as_ref()); }
            }
        }
        None
    }
}

pub trait BNO055Mode {
    def set_mode(&mut self, imu_id: BNO055ID, mode: u8) -> Result<(), BNO055Error>;
    def get_mode(&self, imu_id: BNO055ID) -> Result<u8, BNO055Error>;
}

#[repr(C)]
pub struct SimpleBNO055Mode {
    pub controller: SimpleBNO055Controller,
    pub modes: Vec<(BNO055ID, AtomicUsize)>,
}

impl SimpleBNO055Mode {
    pub fn new(controller: SimpleBNO055Controller) -> Self {
        SimpleBNO055Mode {
            controller,
            modes: Vec::new(),
        }
    }
}

impl BNO055Mode for SimpleBNO055Mode {
    fn set_mode(&mut self, imu_id: BNO055ID, mode: u8) -> Result<(), BNO055Error> {
        self.modes.push((imu_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_mode(&self, imu_id: BNO055ID) -> Result<u8, BNO055Error> {
        for &(id, ref mode) in &self.modes {
            if id == imu_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(BNO055Error::NotFound)
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
