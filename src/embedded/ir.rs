#![no_std]
#![no_main]

/// OOP-based IR Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1356
/// Implements IR sensor and remote control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait IRSensor {
    fn id(&self) -> SensorID;
    fn read_raw(&self) -> u16;
    fn is_detected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleIRSensor {
    pub id: SensorID,
    pub raw_value: AtomicUsize,
    pub detected: AtomicUsize,
}

impl SimpleIRSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleIRSensor {
            id,
            raw_value: AtomicUsize::new(0),
            detected: AtomicUsize::new(0),
        }
    }
}

impl IRSensor for SimpleIRSensor {
    fn id(&self) -> SensorID { self.id }
    fn read_raw(&self) -> u16 { self.raw_value.load(Ordering::SeqCst) as u16 }
    fn is_detected(&self) -> bool { self.detected.load(Ordering::SeqCst) == 1 }
}

pub trait IRRemote {
    def send_code(&self, code: u32) -> Result<(), SensorError>;
    def receive_code(&self) -> Option<u32>;
}

#[repr(C)]
pub struct SimpleIRRemote {
    pub sensor: SimpleIRSensor,
}

impl SimpleIRRemote {
    pub fn new(sensor: SimpleIRSensor) -> Self {
        SimpleIRRemote { sensor }
    }
}

impl IRRemote for SimpleIRRemote {
    fn send_code(&self, _code: u32) -> Result<(), SensorError> {
        Ok(())
    }
    
    fn receive_code(&self) -> Option<u32> {
        if self.sensor.is_detected() {
            Some(self.sensor.read_raw() as u32)
        } else {
            None
        }
    }
}

pub trait IRDecoder {
    def decode(&self, raw: u16) -> Option<&[u8]>;
    def learn_code(&mut self, name: &[u8], code: u32);
}

#[repr(C)]
pub struct SimpleIRDecoder {
    pub codes: Vec<([u8; 32], u32)>,
}

impl SimpleIRDecoder {
    pub fn new() -> Self {
        SimpleIRDecoder {
            codes: Vec::new(),
        }
    }
}

impl IRDecoder for SimpleIRDecoder {
    fn decode(&self, raw: u16) -> Option<&[u8]> {
        for &(ref name, code) in &self.codes {
            if (code as u16) == raw {
                let len = name.iter().position(|&b| b == 0).unwrap_or(32);
                return Some(&name[..len]);
            }
        }
        None
    }
    
    fn learn_code(&mut self, name: &[u8], code: u32) {
        let mut name_array = [0u8; 32];
        let name_len = name.len().min(31);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.codes.push((name_array, code));
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
