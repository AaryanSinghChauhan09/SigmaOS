#![no_std]
#![no_main]

/// OOP-based IMX477 Camera for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2676
/// Implements IMX477 camera sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type IMX477ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IMX477Error { Success = 0, NotFound = 1 }

pub trait IMX477Camera {
    fn id(&self) -> IMX477ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleIMX477Camera {
    pub id: IMX477ID,
    pub initialized: AtomicUsize,
}

impl SimpleIMX477Camera {
    pub fn new(id: IMX477ID) -> Self {
        SimpleIMX477Camera {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl IMX477Camera for SimpleIMX477Camera {
    fn id(&self) -> IMX477ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait IMX477Controller {
    fn init(&mut self, imx_id: IMX477ID) -> Result<(), IMX477Error>;
    fn capture(&self, imx_id: IMX477ID, buffer: &mut [u8]) -> Result<usize, IMX477Error>;
    def set_resolution(&mut self, imx_id: IMX477ID, width: u16, height: u16) -> Result<(), IMX477Error>;
}

#[repr(C)]
pub struct SimpleIMX477Controller {
    pub cameras: Vec<Option<Box<dyn IMX477Camera>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIMX477Controller {
    pub fn new() -> Self {
        SimpleIMX477Controller {
            cameras: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IMX477Controller for SimpleIMX477Controller {
    fn init(&mut self, imx_id: IMX477ID) -> Result<(), IMX477Error> {
        for camera_option in &mut self.cameras {
            if let Some(ref mut camera) = *camera_option {
                if camera.id() == imx_id {
                    camera.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(IMX477Error::NotFound)
    }
    
    fn capture(&self, imx_id: IMX477ID, buffer: &mut [u8]) -> Result<usize, IMX477Error> {
        if self.get_camera(imx_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(IMX477Error::NotFound)
        }
    }
    
    fn set_resolution(&mut self, _imx_id: IMX477ID, _width: u16, _height: u16) -> Result<(), IMX477Error> {
        Ok(())
    }
    
    fn get_camera(&self, id: IMX477ID) -> Option<&dyn IMX477Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
}

pub trait IMX477HDR {
    def set_hdr_mode(&mut self, imx_id: IMX477ID, enable: bool) -> Result<(), IMX477Error>;
    def set_exposure(&mut self, imx_id: IMX477ID, exposure: u32) -> Result<(), IMX477Error>;
}

#[repr(C)]
pub struct SimpleIMX477HDR {
    pub controller: SimpleIMX477Controller,
}

impl SimpleIMX477HDR {
    pub fn new(controller: SimpleIMX477Controller) -> Self {
        SimpleIMX477HDR { controller }
    }
}

impl IMX477HDR for SimpleIMX477HDR {
    fn set_hdr_mode(&mut self, _imx_id: IMX477ID, _enable: bool) -> Result<(), IMX477Error> {
        Ok(())
    }
    
    fn set_exposure(&mut self, _imx_id: IMX477ID, _exposure: u32) -> Result<(), IMX477Error> {
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
