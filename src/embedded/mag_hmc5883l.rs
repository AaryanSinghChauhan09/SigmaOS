#![no_std]
#![no_main]

/// OOP-based HMC5883L Magnetometer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3146
/// Implements HMC5883L 3-axis magnetometer

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HMC5883LID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HMC5883LError { Success = 0, NotFound = 1 }

pub trait HMC5883LMag {
    fn id(&self) -> HMC5883LID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHMC5883LMag {
    pub id: HMC5883LID,
    pub initialized: AtomicUsize,
}

impl SimpleHMC5883LMag {
    pub fn new(id: HMC5883LID) -> Self {
        SimpleHMC5883LMag {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HMC5883LMag for SimpleHMC5883LMag {
    fn id(&self) -> HMC5883LID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HMC5883LController {
    fn init(&mut self, mag_id: HMC5883LID) -> Result<(), HMC5883LError>;
    fn read(&self, mag_id: HMC5883LID) -> Result<(i16, i16, i16), HMC5883LError>;
    def set_gain(&mut self, mag_id: HMC5883LID, gain: u8) -> Result<(), HMC5883LError>;
}

#[repr(C)]
pub struct SimpleHMC5883LController {
    pub mags: Vec<Option<Box<dyn HMC5883LMag>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHMC5883LController {
    pub fn new() -> Self {
        SimpleHMC5883LController {
            mags: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HMC5883LController for SimpleHMC5883LController {
    fn init(&mut self, mag_id: HMC5883LID) -> Result<(), HMC5883LError> {
        for mag_option in &mut self.mags {
            if let Some(ref mut mag) = *mag_option {
                if mag.id() == mag_id {
                    mag.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HMC5883LError::NotFound)
    }
    
    fn read(&self, mag_id: HMC5883LID) -> Result<(i16, i16, i16), HMC5883LError> {
        if self.get_mag(mag_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(HMC5883LError::NotFound)
        }
    }
    
    fn set_gain(&mut self, _mag_id: HMC5883LID, _gain: u8) -> Result<(), HMC5883LError> {
        Ok(())
    }
    
    fn get_mag(&self, id: HMC5883LID) -> Option<&dyn HMC5883LMag> {
        for mag_option in &self.mags {
            if let Some(ref mag) = *mag_option {
                if mag.id() == id { return Some(mag.as_ref()); }
            }
        }
        None
    }
}

pub trait HMC5883LConfig {
    def set_samples(&mut self, mag_id: HMC5883LID, samples: u8) -> Result<(), HMC5883LError>;
    def get_samples(&self, mag_id: HMC5883LID) -> Result<u8, HMC5883LError>;
}

#[repr(C)]
pub struct SimpleHMC5883LConfig {
    pub controller: SimpleHMC5883LController,
    pub samples: Vec<(HMC5883LID, AtomicUsize)>,
}

impl SimpleHMC5883LConfig {
    pub fn new(controller: SimpleHMC5883LController) -> Self {
        SimpleHMC5883LConfig {
            controller,
            samples: Vec::new(),
        }
    }
}

impl HMC5883LConfig for SimpleHMC5883LConfig {
    fn set_samples(&mut self, mag_id: HMC5883LID, samples: u8) -> Result<(), HMC5883LError> {
        self.samples.push((mag_id, AtomicUsize::new(samples as usize)));
        Ok(())
    }
    
    fn get_samples(&self, mag_id: HMC5883LID) -> Result<u8, HMC5883LError> {
        for &(id, ref samples) in &self.samples {
            if id == mag_id {
                return Ok(samples.load(Ordering::SeqCst) as u8);
            }
        }
        Err(HMC5883LError::NotFound)
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
