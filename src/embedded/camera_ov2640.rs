#![no_std]
#![no_main]

/// OOP-based OV2640 Camera for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2636
/// Implements OV2640 camera sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OV2640ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OV2640Error { Success = 0, NotFound = 1 }

pub trait OV2640Camera {
    fn id(&self) -> OV2640ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOV2640Camera {
    pub id: OV2640ID,
    pub initialized: AtomicUsize,
}

impl SimpleOV2640Camera {
    pub fn new(id: OV2640ID) -> Self {
        SimpleOV2640Camera {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl OV2640Camera for SimpleOV2640Camera {
    fn id(&self) -> OV2640ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait OV2640Controller {
    fn init(&mut self, ov_id: OV2640ID) -> Result<(), OV2640Error>;
    fn capture(&self, ov_id: OV2640ID, buffer: &mut [u8]) -> Result<usize, OV2640Error>;
    def set_resolution(&mut self, ov_id: OV2640ID, width: u16, height: u16) -> Result<(), OV2640Error>;
}

#[repr(C)]
pub struct SimpleOV2640Controller {
    pub cameras: Vec<Option<Box<dyn OV2640Camera>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOV2640Controller {
    pub fn new() -> Self {
        SimpleOV2640Controller {
            cameras: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OV2640Controller for SimpleOV2640Controller {
    fn init(&mut self, ov_id: OV2640ID) -> Result<(), OV2640Error> {
        for camera_option in &mut self.cameras {
            if let Some(ref mut camera) = *camera_option {
                if camera.id() == ov_id {
                    camera.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(OV2640Error::NotFound)
    }
    
    fn capture(&self, ov_id: OV2640ID, buffer: &mut [u8]) -> Result<usize, OV2640Error> {
        if self.get_camera(ov_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(OV2640Error::NotFound)
        }
    }
    
    fn set_resolution(&mut self, _ov_id: OV2640ID, _width: u16, _height: u16) -> Result<(), OV2640Error> {
        Ok(())
    }
    
    fn get_camera(&self, id: OV2640ID) -> Option<&dyn OV2640Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
}

pub trait OV2640Format {
    def set_format(&mut self, ov_id: OV2640ID, format: u8) -> Result<(), OV2640Error>;
    def get_format(&self, ov_id: OV2640ID) -> Result<u8, OV2640Error>;
}

#[repr(C)]
pub struct SimpleOV2640Format {
    pub controller: SimpleOV2640Controller,
    pub formats: Vec<(OV2640ID, AtomicUsize)>,
}

impl SimpleOV2640Format {
    pub fn new(controller: SimpleOV2640Controller) -> Self {
        SimpleOV2640Format {
            controller,
            formats: Vec::new(),
        }
    }
}

impl OV2640Format for SimpleOV2640Format {
    fn set_format(&mut self, ov_id: OV2640ID, format: u8) -> Result<(), OV2640Error> {
        self.formats.push((ov_id, AtomicUsize::new(format as usize)));
        Ok(())
    }
    
    fn get_format(&self, ov_id: OV2640ID) -> Result<u8, OV2640Error> {
        for &(id, ref fmt) in &self.formats {
            if id == ov_id {
                return Ok(fmt.load(Ordering::SeqCst) as u8);
            }
        }
        Err(OV2640Error::NotFound)
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
