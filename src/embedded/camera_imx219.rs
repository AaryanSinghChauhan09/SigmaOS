#![no_std]
#![no_main]

/// OOP-based IMX219 Camera for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2666
/// Implements IMX219 camera sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type IMX219ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IMX219Error { Success = 0, NotFound = 1 }

pub trait IMX219Camera {
    fn id(&self) -> IMX219ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleIMX219Camera {
    pub id: IMX219ID,
    pub initialized: AtomicUsize,
}

impl SimpleIMX219Camera {
    pub fn new(id: IMX219ID) -> Self {
        SimpleIMX219Camera {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl IMX219Camera for SimpleIMX219Camera {
    fn id(&self) -> IMX219ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait IMX219Controller {
    fn init(&mut self, imx_id: IMX219ID) -> Result<(), IMX219Error>;
    fn capture(&self, imx_id: IMX219ID, buffer: &mut [u8]) -> Result<usize, IMX219Error>;
    def set_resolution(&mut self, imx_id: IMX219ID, width: u16, height: u16) -> Result<(), IMX219Error>;
}

#[repr(C)]
pub struct SimpleIMX219Controller {
    pub cameras: Vec<Option<Box<dyn IMX219Camera>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIMX219Controller {
    pub fn new() -> Self {
        SimpleIMX219Controller {
            cameras: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IMX219Controller for SimpleIMX219Controller {
    fn init(&mut self, imx_id: IMX219ID) -> Result<(), IMX219Error> {
        for camera_option in &mut self.cameras {
            if let Some(ref mut camera) = *camera_option {
                if camera.id() == imx_id {
                    camera.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(IMX219Error::NotFound)
    }
    
    fn capture(&self, imx_id: IMX219ID, buffer: &mut [u8]) -> Result<usize, IMX219Error> {
        if self.get_camera(imx_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(IMX219Error::NotFound)
        }
    }
    
    fn set_resolution(&mut self, _imx_id: IMX219ID, _width: u16, _height: u16) -> Result<(), IMX219Error> {
        Ok(())
    }
    
    fn get_camera(&self, id: IMX219ID) -> Option<&dyn IMX219Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
}

pub trait IMX219Gain {
    def set_gain(&mut self, imx_id: IMX219ID, gain: u16) -> Result<(), IMX219Error>;
    def get_gain(&self, imx_id: IMX219ID) -> Result<u16, IMX219Error>;
}

#[repr(C)]
pub struct SimpleIMX219Gain {
    pub controller: SimpleIMX219Controller,
    pub gains: Vec<(IMX219ID, AtomicUsize)>,
}

impl SimpleIMX219Gain {
    pub fn new(controller: SimpleIMX219Controller) -> Self {
        SimpleIMX219Gain {
            controller,
            gains: Vec::new(),
        }
    }
}

impl IMX219Gain for SimpleIMX219Gain {
    fn set_gain(&mut self, imx_id: IMX219ID, gain: u16) -> Result<(), IMX219Error> {
        self.gains.push((imx_id, AtomicUsize::new(gain as usize)));
        Ok(())
    }
    
    fn get_gain(&self, imx_id: IMX219ID) -> Result<u16, IMX219Error> {
        for &(id, ref gain) in &self.gains {
            if id == imx_id {
                return Ok(gain.load(Ordering::SeqCst) as u16);
            }
        }
        Err(IMX219Error::NotFound)
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
