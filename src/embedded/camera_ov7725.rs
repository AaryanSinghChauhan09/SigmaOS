#![no_std]
#![no_main]

/// OOP-based OV7725 Camera for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2656
/// Implements OV7725 camera sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OV7725ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OV7725Error { Success = 0, NotFound = 1 }

pub trait OV7725Camera {
    fn id(&self) -> OV7725ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOV7725Camera {
    pub id: OV7725ID,
    pub initialized: AtomicUsize,
}

impl SimpleOV7725Camera {
    pub fn new(id: OV7725ID) -> Self {
        SimpleOV7725Camera {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl OV7725Camera for SimpleOV7725Camera {
    fn id(&self) -> OV7725ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait OV7725Controller {
    fn init(&mut self, ov_id: OV7725ID) -> Result<(), OV7725Error>;
    fn capture(&self, ov_id: OV7725ID, buffer: &mut [u8]) -> Result<usize, OV7725Error>;
    def set_resolution(&mut self, ov_id: OV7725ID, width: u16, height: u16) -> Result<(), OV7725Error>;
}

#[repr(C)]
pub struct SimpleOV7725Controller {
    pub cameras: Vec<Option<Box<dyn OV7725Camera>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOV7725Controller {
    pub fn new() -> Self {
        SimpleOV7725Controller {
            cameras: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OV7725Controller for SimpleOV7725Controller {
    fn init(&mut self, ov_id: OV7725ID) -> Result<(), OV7725Error> {
        for camera_option in &mut self.cameras {
            if let Some(ref mut camera) = *camera_option {
                if camera.id() == ov_id {
                    camera.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(OV7725Error::NotFound)
    }
    
    fn capture(&self, ov_id: OV7725ID, buffer: &mut [u8]) -> Result<usize, OV7725Error> {
        if self.get_camera(ov_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(OV7725Error::NotFound)
        }
    }
    
    fn set_resolution(&mut self, _ov_id: OV7725ID, _width: u16, _height: u16) -> Result<(), OV7725Error> {
        Ok(())
    }
    
    fn get_camera(&self, id: OV7725ID) -> Option<&dyn OV7725Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
}

pub trait OV7725AWB {
    def set_awb(&mut self, ov_id: OV7725ID, enable: bool) -> Result<(), OV7725Error>;
    def set_aec(&mut self, ov_id: OV7725ID, enable: bool) -> Result<(), OV7725Error>;
}

#[repr(C)]
pub struct SimpleOV7725AWB {
    pub controller: SimpleOV7725Controller,
}

impl SimpleOV7725AWB {
    pub fn new(controller: SimpleOV7725Controller) -> Self {
        SimpleOV7725AWB { controller }
    }
}

impl OV7725AWB for SimpleOV7725AWB {
    fn set_awb(&mut self, _ov_id: OV7725ID, _enable: bool) -> Result<(), OV7725Error> {
        Ok(())
    }
    
    fn set_aec(&mut self, _ov_id: OV7725ID, _enable: bool) -> Result<(), OV7725Error> {
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
