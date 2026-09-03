#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

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
    #[allow(clippy::new_without_default)]
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
    #[allow(clippy::new_without_default)]
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
