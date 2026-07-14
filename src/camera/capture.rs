#![no_std]
#![no_main]

/// OOP-based Camera Capture for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 281
/// Implements camera device management and capture

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CameraID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CameraFormat { RGB24 = 0, YUYV = 1, MJPEG = 2, H264 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CameraError { Success = 0, NotFound = 1, CaptureFailed = 2 }

pub trait Camera {
    fn id(&self) -> CameraID;
    fn name(&self) -> &[u8];
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn format(&self) -> CameraFormat;
}

#[repr(C)]
pub struct SimpleCamera {
    pub id: CameraID,
    pub name: [u8; 64],
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub format: AtomicUsize,
}

impl SimpleCamera {
    pub fn new(id: CameraID, name: &[u8], width: u32, height: u32, format: CameraFormat) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleCamera {
            id,
            name: name_array,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            format: AtomicUsize::new(format as usize),
        }
    }
}

impl Camera for SimpleCamera {
    fn id(&self) -> CameraID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn width(&self) -> u32 { self.width.load(Ordering::SeqCst) as u32 }
    fn height(&self) -> u32 { self.height.load(Ordering::SeqCst) as u32 }
    fn format(&self) -> CameraFormat { unsafe { core::mem::transmute(self.format.load(Ordering::SeqCst)) } }
}

pub trait CameraManager {
    fn add_camera(&mut self, camera: Box<dyn Camera>) -> Result<CameraID, CameraError>;
    fn remove_camera(&mut self, id: CameraID) -> Result<(), CameraError>;
    fn get_camera(&self, id: CameraID) -> Option<&dyn Camera>;
    fn capture_frame(&self, id: CameraID, buffer: &mut [u8]) -> Result<usize, CameraError>;
    fn set_format(&mut self, id: CameraID, format: CameraFormat) -> Result<(), CameraError>;
}

#[repr(C)]
pub struct SimpleCameraManager {
    pub cameras: Vec<Option<Box<dyn Camera>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCameraManager {
    pub fn new() -> Self {
        SimpleCameraManager {
            cameras: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CameraManager for SimpleCameraManager {
    fn add_camera(&mut self, camera: Box<dyn Camera>) -> Result<CameraID, CameraError> {
        let id = camera.id();
        self.cameras.push(Some(camera));
        Ok(id)
    }
    
    fn remove_camera(&mut self, id: CameraID) -> Result<(), CameraError> {
        for camera_option in &mut self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id {
                    return Ok(());
                }
            }
        }
        Err(CameraError::NotFound)
    }
    
    fn get_camera(&self, id: CameraID) -> Option<&dyn Camera> {
        for camera_option in &self.cameras {
            if let Some(ref camera) = *camera_option {
                if camera.id() == id { return Some(camera.as_ref()); }
            }
        }
        None
    }
    
    fn capture_frame(&self, id: CameraID, buffer: &mut [u8]) -> Result<usize, CameraError> {
        if let Some(camera) = self.get_camera(id) {
            let width = camera.width();
            let height = camera.height();
            let frame_size = (width * height * 3) as usize;
            
            for byte in buffer.iter_mut().take(frame_size) {
                *byte = 128u8;
            }
            
            Ok(frame_size.min(buffer.len()))
        } else {
            Err(CameraError::NotFound)
        }
    }
    
    fn set_format(&mut self, id: CameraID, format: CameraFormat) -> Result<(), CameraError> {
        for camera_option in &mut self.cameras {
            if let Some(ref mut camera) = *camera_option {
                if camera.id() == id {
                    camera.format.store(format as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(CameraError::NotFound)
    }
}

pub trait VideoRecorder {
    fn start_recording(&mut self, camera_id: CameraID, output: &[u8]) -> Result<(), CameraError>;
    fn stop_recording(&mut self, camera_id: CameraID) -> Result<(), CameraError>;
    fn is_recording(&self, camera_id: CameraID) -> bool;
}

#[repr(C)]
pub struct SimpleVideoRecorder {
    pub recording: Vec<(CameraID, [u8; 256])>,
}

impl SimpleVideoRecorder {
    pub fn new() -> Self {
        SimpleVideoRecorder {
            recording: Vec::new(),
        }
    }
}

impl VideoRecorder for SimpleVideoRecorder {
    fn start_recording(&mut self, camera_id: CameraID, output: &[u8]) -> Result<(), CameraError> {
        let mut output_array = [0u8; 256];
        let output_len = output.len().min(255);
        for i in 0..output_len {
            output_array[i] = output[i];
        }
        self.recording.push((camera_id, output_array));
        Ok(())
    }
    
    fn stop_recording(&mut self, camera_id: CameraID) -> Result<(), CameraError> {
        for i in 0..self.recording.len() {
            if self.recording[i].0 == camera_id {
                self.recording.remove(i);
                return Ok(());
            }
        }
        Err(CameraError::NotFound)
    }
    
    fn is_recording(&self, camera_id: CameraID) -> bool {
        for &(id, _) in &self.recording {
            if id == camera_id {
                return true;
            }
        }
        false
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
