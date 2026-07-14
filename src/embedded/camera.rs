#![no_std]
#![no_main]

/// OOP-based Camera for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1666
/// Implements camera sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CameraID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Resolution { QVGA = 0, VGA = 1, SVGA = 2, XGA = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CameraError { Success = 0, NotFound = 1 }

pub trait Camera {
    fn id(&self) -> CameraID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

#[repr(C)]
pub struct SimpleCamera {
    pub id: CameraID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleCamera {
    pub fn new(id: CameraID, width: u16, height: u16) -> Self {
        SimpleCamera {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl Camera for SimpleCamera {
    fn id(&self) -> CameraID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
}

pub trait CameraController {
    fn init(&mut self, camera_id: CameraID) -> Result<(), CameraError>;
    fn set_resolution(&mut self, camera_id: CameraID, resolution: Resolution) -> Result<(), CameraError>;
    def capture_frame(&self, camera_id: CameraID, buffer: &mut [u8]) -> Result<(), CameraError>;
}

#[repr(C)]
pub struct SimpleCameraController {
    pub cameras: Vec<Option<Box<dyn Camera>>>,
    pub resolutions: Vec<(CameraID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleCameraController {
    pub fn new() -> Self {
        SimpleCameraController {
            cameras: Vec::new(),
            resolutions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CameraController for SimpleCameraController {
    fn init(&mut self, camera_id: CameraID) -> Result<(), CameraError> {
        if self.get_camera(camera_id).is_some() {
            Ok(())
        } else {
            Err(CameraError::NotFound)
        }
    }
    
    fn set_resolution(&mut self, camera_id: CameraID, resolution: Resolution) -> Result<(), CameraError> {
        self.resolutions.push((camera_id, AtomicUsize::new(resolution as usize)));
        Ok(())
    }
    
    fn capture_frame(&self, camera_id: CameraID, buffer: &mut [u8]) -> Result<(), CameraError> {
        if self.get_camera(camera_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(CameraError::NotFound)
        }
    }
    
    fn get_camera(&self, id: CameraID) -> Option<&dyn Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
}

pub trait CameraSettings {
    def set_brightness(&mut self, camera_id: CameraID, brightness: u8) -> Result<(), CameraError>;
    def set_contrast(&mut self, camera_id: CameraID, contrast: u8) -> Result<(), CameraError>;
}

#[repr(C)]
pub struct SimpleCameraSettings {
    pub controller: SimpleCameraController,
    pub brightness: Vec<(CameraID, AtomicUsize)>,
    pub contrast: Vec<(CameraID, AtomicUsize)>,
}

impl SimpleCameraSettings {
    pub fn new(controller: SimpleCameraController) -> Self {
        SimpleCameraSettings {
            controller,
            brightness: Vec::new(),
            contrast: Vec::new(),
        }
    }
}

impl CameraSettings for SimpleCameraSettings {
    fn set_brightness(&mut self, camera_id: CameraID, brightness: u8) -> Result<(), CameraError> {
        self.brightness.push((camera_id, AtomicUsize::new(brightness as usize)));
        Ok(())
    }
    
    fn set_contrast(&mut self, camera_id: CameraID, contrast: u8) -> Result<(), CameraError> {
        self.contrast.push((camera_id, AtomicUsize::new(contrast as usize)));
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
