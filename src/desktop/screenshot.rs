#![no_std]
#![no_main]

/// OOP-based Screenshot Tool for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 796
/// Implements screenshot capture and annotation

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ScreenshotID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CaptureMode { Fullscreen = 0, Window = 1, Region = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ScreenshotError { Success = 0, CaptureFailed = 1 }

pub trait Screenshot {
    fn id(&self) -> ScreenshotID;
    fn timestamp(&self) -> u64;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn data(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleScreenshot {
    pub id: ScreenshotID,
    pub timestamp: AtomicUsize,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub data: Vec<u8>,
}

impl SimpleScreenshot {
    pub fn new(id: ScreenshotID, width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(0u8);
        }
        SimpleScreenshot {
            id,
            timestamp: AtomicUsize::new(1000000),
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            data,
        }
    }
}

impl Screenshot for SimpleScreenshot {
    fn id(&self) -> ScreenshotID { self.id }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
    fn width(&self) -> u32 { self.width.load(Ordering::SeqCst) as u32 }
    fn height(&self) -> u32 { self.height.load(Ordering::SeqCst) as u32 }
    fn data(&self) -> &[u8] { &self.data }
}

pub trait ScreenshotTool {
    fn capture(&mut self, mode: CaptureMode) -> Result<ScreenshotID, ScreenshotError>;
    fn save(&self, id: ScreenshotID, path: &[u8]) -> Result<(), ScreenshotError>;
    fn get_screenshot(&self, id: ScreenshotID) -> Option<&dyn Screenshot>;
}

#[repr(C)]
pub struct SimpleScreenshotTool {
    pub screenshots: Vec<Option<Box<dyn Screenshot>>>,
    pub next_id: AtomicUsize,
}

impl SimpleScreenshotTool {
    pub fn new() -> Self {
        SimpleScreenshotTool {
            screenshots: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ScreenshotTool for SimpleScreenshotTool {
    fn capture(&mut self, mode: CaptureMode) -> Result<ScreenshotID, ScreenshotError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (width, height) = match mode {
            CaptureMode::Fullscreen => (1920, 1080),
            CaptureMode::Window => (800, 600),
            CaptureMode::Region => (400, 300),
        };
        let screenshot = SimpleScreenshot::new(id, width, height);
        self.screenshots.push(Some(Box::new(screenshot)));
        Ok(id)
    }
    
    fn save(&self, id: ScreenshotID, _path: &[u8]) -> Result<(), ScreenshotError> {
        if self.get_screenshot(id).is_some() {
            Ok(())
        } else {
            Err(ScreenshotError::CaptureFailed)
        }
    }
    
    fn get_screenshot(&self, id: ScreenshotID) -> Option<&dyn Screenshot> {
        for screenshot_option in &self.screenshots {
            if let Some(ref screenshot) = *screenshot_option {
                if screenshot.id() == id { return Some(screenshot.as_ref()); }
            }
        }
        None
    }
}

pub trait Annotation {
    fn add_text(&mut self, screenshot_id: ScreenshotID, x: u32, y: u32, text: &[u8]);
    fn add_rectangle(&mut self, screenshot_id: ScreenshotID, x: u32, y: u32, width: u32, height: u32);
    fn add_arrow(&mut self, screenshot_id: ScreenshotID, x1: u32, y1: u32, x2: u32, y2: u32);
}

#[repr(C)]
pub struct SimpleAnnotation {
    pub annotations: Vec<(ScreenshotID, [u8; 8], u32, u32, u32, u32)>,
}

impl SimpleAnnotation {
    pub fn new() -> Self {
        SimpleAnnotation {
            annotations: Vec::new(),
        }
    }
}

impl Annotation for SimpleAnnotation {
    fn add_text(&mut self, screenshot_id: ScreenshotID, x: u32, y: u32, _text: &[u8]) {
        self.annotations.push((screenshot_id, [b't', b'e', b'x', b't', 0, 0, 0, 0], x, y, 0, 0));
    }
    
    fn add_rectangle(&mut self, screenshot_id: ScreenshotID, x: u32, y: u32, width: u32, height: u32) {
        self.annotations.push((screenshot_id, [b'r', b'e', b'c', b't', 0, 0, 0, 0], x, y, width, height));
    }
    
    fn add_arrow(&mut self, screenshot_id: ScreenshotID, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.annotations.push((screenshot_id, [b'a', b'r', b'r', b'w', 0, 0, 0, 0], x1, y1, x2, y2));
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
