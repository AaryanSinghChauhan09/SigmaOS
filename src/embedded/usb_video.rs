#![no_std]
#![no_main]

/// OOP-based USB Video for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2276
/// Implements USB Video (UVC)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type VideoUSBID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VideoUSBError { Success = 0, NotFound = 1 }

pub trait USBVideo {
    fn id(&self) -> VideoUSBID;
    fn is_streaming(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUSBVideo {
    pub id: VideoUSBID,
    pub streaming: AtomicUsize,
}

impl SimpleUSBVideo {
    pub fn new(id: VideoUSBID) -> Self {
        SimpleUSBVideo {
            id,
            streaming: AtomicUsize::new(0),
        }
    }
}

impl USBVideo for SimpleUSBVideo {
    fn id(&self) -> VideoUSBID { self.id }
    fn is_streaming(&self) -> bool { self.streaming.load(Ordering::SeqCst) == 1 }
}

pub trait VideoUSBController {
    fn init(&mut self, video_id: VideoUSBID) -> Result<(), VideoUSBError>;
    fn start_stream(&mut self, video_id: VideoUSBID) -> Result<(), VideoUSBError>;
    def stop_stream(&mut self, video_id: VideoUSBID) -> Result<(), VideoUSBError>;
}

#[repr(C)]
pub struct SimpleVideoUSBController {
    pub video_devices: Vec<Option<Box<dyn USBVideo>>>,
    pub next_id: AtomicUsize,
}

impl SimpleVideoUSBController {
    pub fn new() -> Self {
        SimpleVideoUSBController {
            video_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VideoUSBController for SimpleVideoUSBController {
    fn init(&mut self, _video_id: VideoUSBID) -> Result<(), VideoUSBError> {
        Ok(())
    }
    
    fn start_stream(&mut self, video_id: VideoUSBID) -> Result<(), VideoUSBError> {
        for video_option in &mut self.video_devices {
            if let Some(ref mut video) = *video_option {
                if video.id() == video_id {
                    video.streaming.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(VideoUSBError::NotFound)
    }
    
    fn stop_stream(&mut self, video_id: VideoUSBID) -> Result<(), VideoUSBError> {
        for video_option in &mut self.video_devices {
            if let Some(ref mut video) = *video_option {
                if video.id() == video_id {
                    video.streaming.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(VideoUSBError::NotFound)
    }
}

pub trait VideoFormat {
    def set_format(&mut self, video_id: VideoUSBID, width: u16, height: u16, format: u8) -> Result<(), VideoUSBError>;
    def get_frame(&self, video_id: VideoUSBID, buffer: &mut [u8]) -> Result<usize, VideoUSBError>;
}

#[repr(C)]
pub struct SimpleVideoFormat {
    pub controller: SimpleVideoUSBController,
    pub formats: Vec<(VideoUSBID, (AtomicUsize, AtomicUsize, AtomicUsize))>,
}

impl SimpleVideoFormat {
    pub fn new(controller: SimpleVideoUSBController) -> Self {
        SimpleVideoFormat {
            controller,
            formats: Vec::new(),
        }
    }
}

impl VideoFormat for SimpleVideoFormat {
    fn set_format(&mut self, video_id: VideoUSBID, width: u16, height: u16, format: u8) -> Result<(), VideoUSBError> {
        self.formats.push((video_id, (AtomicUsize::new(width as usize), AtomicUsize::new(height as usize), AtomicUsize::new(format as usize))));
        Ok(())
    }
    
    fn get_frame(&self, video_id: VideoUSBID, buffer: &mut [u8]) -> Result<usize, VideoUSBError> {
        if self.controller.get_video(video_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(VideoUSBError::NotFound)
        }
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
