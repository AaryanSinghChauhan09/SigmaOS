#![no_std]
#![no_main]

/// OOP-based LED Matrix for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1466
/// Implements LED matrix display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MatrixID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MatrixError { Success = 0, NotFound = 1 }

pub trait LEDMatrix {
    fn id(&self) -> MatrixID;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
}

#[repr(C)]
pub struct SimpleLEDMatrix {
    pub id: MatrixID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleLEDMatrix {
    pub fn new(id: MatrixID, width: u8, height: u8) -> Self {
        SimpleLEDMatrix {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl LEDMatrix for SimpleLEDMatrix {
    fn id(&self) -> MatrixID { self.id }
    fn width(&self) -> u8 { self.width.load(Ordering::SeqCst) as u8 }
    fn height(&self) -> u8 { self.height.load(Ordering::SeqCst) as u8 }
}

pub trait MatrixController {
    fn set_pixel(&self, matrix_id: MatrixID, x: u8, y: u8, color: u32) -> Result<(), MatrixError>;
    fn clear(&self, matrix_id: MatrixID) -> Result<(), MatrixError>;
    def draw_line(&self, matrix_id: MatrixID, x1: u8, y1: u8, x2: u8, y2: u8, color: u32) -> Result<(), MatrixError>;
}

#[repr(C)]
pub struct SimpleMatrixController {
    pub matrices: Vec<Option<Box<dyn LEDMatrix>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMatrixController {
    pub fn new() -> Self {
        SimpleMatrixController {
            matrices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MatrixController for SimpleMatrixController {
    fn set_pixel(&self, matrix_id: MatrixID, _x: u8, _y: u8, _color: u32) -> Result<(), MatrixError> {
        if self.get_matrix(matrix_id).is_some() {
            Ok(())
        } else {
            Err(MatrixError::NotFound)
        }
    }
    
    fn clear(&self, matrix_id: MatrixID) -> Result<(), MatrixError> {
        if self.get_matrix(matrix_id).is_some() {
            Ok(())
        } else {
            Err(MatrixError::NotFound)
        }
    }
    
    fn draw_line(&self, matrix_id: MatrixID, _x1: u8, _y1: u8, _x2: u8, _y2: u8, _color: u32) -> Result<(), MatrixError> {
        if self.get_matrix(matrix_id).is_some() {
            Ok(())
        } else {
            Err(MatrixError::NotFound)
        }
    }
    
    fn get_matrix(&self, id: MatrixID) -> Option<&dyn LEDMatrix> {
        for matrix_option in &self.matrices {
            if let Some(ref matrix) = *matrix_option {
                if matrix.id() == id { return Some(matrix.as_ref()); }
            }
        }
        None
    }
}

pub trait ScrollingText {
    def set_text(&mut self, text: &[u8]);
    def scroll(&mut self);
    def get_position(&self) -> u16;
}

#[repr(C)]
pub struct SimpleScrollingText {
    pub text: [u8; 128],
    pub position: AtomicUsize,
    pub text_len: AtomicUsize,
}

impl SimpleScrollingText {
    pub fn new() -> Self {
        SimpleScrollingText {
            text: [0u8; 128],
            position: AtomicUsize::new(0),
            text_len: AtomicUsize::new(0),
        }
    }
}

impl ScrollingText for SimpleScrollingText {
    fn set_text(&mut self, text: &[u8]) {
        let text_len = text.len().min(127);
        for i in 0..text_len {
            self.text[i] = text[i];
        }
        self.text_len.store(text_len, Ordering::SeqCst);
        self.position.store(0, Ordering::SeqCst);
    }
    
    fn scroll(&mut self) {
        let len = self.text_len.load(Ordering::SeqCst);
        let pos = self.position.load(Ordering::SeqCst);
        self.position.store((pos + 1) % (len + 10), Ordering::SeqCst);
    }
    
    fn get_position(&self) -> u16 {
        self.position.load(Ordering::SeqCst) as u16
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
