#![no_std]
#![no_main]

/// OOP-based OV5640 Camera for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2646
/// Implements OV5640 camera sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OV5640ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OV5640Error { Success = 0, NotFound = 1 }

pub trait OV5640Camera {
    fn id(&self) -> OV5640ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOV5640Camera {
    pub id: OV5640ID,
    pub initialized: AtomicUsize,
}

impl SimpleOV5640Camera {
    pub fn new(id: OV5640ID) -> Self {
        SimpleOV5640Camera {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl OV5640Camera for SimpleOV5640Camera {
    fn id(&self) -> OV5640ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait OV5640Controller {
    fn init(&mut self, ov_id: OV5640ID) -> Result<(), OV5640Error>;
    fn capture(&self, ov_id: OV5640ID, buffer: &mut [u8]) -> Result<usize, OV5640Error>;
    def set_resolution(&mut self, ov_id: OV5640ID, width: u16, height: u16) -> Result<(), OV5640Error>;
}

#[repr(C)]
pub struct SimpleOV5640Controller {
    pub cameras: Vec<Option<Box<dyn OV5640Camera>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOV5640Controller {
    pub fn new() -> Self {
        SimpleOV5640Controller {
            cameras: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OV5640Controller for SimpleOV5640Controller {
    fn init(&mut self, ov_id: OV5640ID) -> Result<(), OV5640Error> {
        for camera_option in &mut self.cameras {
            if let Some(ref mut camera) = *camera_option {
                if camera.id() == ov_id {
                    camera.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(OV5640Error::NotFound)
    }
    
    fn capture(&self, ov_id: OV5640ID, buffer: &mut [u8]) -> Result<usize, OV5640Error> {
        if self.get_camera(ov_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(OV5640Error::NotFound)
        }
    }
    
    fn set_resolution(&mut self, _ov_id: OV5640ID, _width: u16, _height: u16) -> Result<(), OV5640Error> {
        Ok(())
    }
    
    fn get_camera(&self, id: OV5640ID) -> Option<&dyn OV5640Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
}

pub trait OV5640AF {
    def set_af_mode(&mut self, ov_id: OV5640ID, mode: u8) -> Result<(), OV5640Error>;
    def focus(&mut self, ov_id: OV5640ID) -> Result<(), OV5640Error>;
}

#[repr(C)]
pub struct SimpleOV5640AF {
    pub controller: SimpleOV5640Controller,
}

impl SimpleOV5640AF {
    pub fn new(controller: SimpleOV5640Controller) -> Self {
        SimpleOV5640AF { controller }
    }
}

impl OV5640AF for SimpleOV5640AF {
    fn set_af_mode(&mut self, _ov_id: OV5640ID, _mode: u8) -> Result<(), OV5640Error> {
        Ok(())
    }
    
    fn focus(&mut self, _ov_id: OV5640ID) -> Result<(), OV5640Error> {
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
