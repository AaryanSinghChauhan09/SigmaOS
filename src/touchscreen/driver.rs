#![no_std]
#![no_main]

/// OOP-based Touchscreen Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 321
/// Implements touchscreen input and gesture recognition

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TouchID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TouchState { Up = 0, Down = 1, Move = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TouchError { Success = 0, NotFound = 1 }

pub trait TouchPoint {
    fn id(&self) -> TouchID;
    fn x(&self) -> u32;
    fn y(&self) -> u32;
    fn state(&self) -> TouchState;
    fn pressure(&self) -> u32;
}

#[repr(C)]
pub struct SimpleTouchPoint {
    pub id: TouchID,
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub state: AtomicUsize,
    pub pressure: AtomicUsize,
}

impl SimpleTouchPoint {
    pub fn new(id: TouchID, x: u32, y: u32) -> Self {
        SimpleTouchPoint {
            id,
            x: AtomicUsize::new(x as usize),
            y: AtomicUsize::new(y as usize),
            state: AtomicUsize::new(TouchState::Up as usize),
            pressure: AtomicUsize::new(0),
        }
    }
}

impl TouchPoint for SimpleTouchPoint {
    fn id(&self) -> TouchID { self.id }
    fn x(&self) -> u32 { self.x.load(Ordering::SeqCst) as u32 }
    fn y(&self) -> u32 { self.y.load(Ordering::SeqCst) as u32 }
    fn state(&self) -> TouchState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn pressure(&self) -> u32 { self.pressure.load(Ordering::SeqCst) as u32 }
}

pub trait Touchscreen {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_touches(&self) -> Vec<&dyn TouchPoint>;
    fn set_touch(&mut self, touch: Box<dyn TouchPoint>);
}

#[repr(C)]
pub struct SimpleTouchscreen {
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub touches: Vec<Option<Box<dyn TouchPoint>>>,
}

impl SimpleTouchscreen {
    pub fn new(width: u32, height: u32) -> Self {
        SimpleTouchscreen {
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            touches: Vec::new(),
        }
    }
}

impl Touchscreen for SimpleTouchscreen {
    fn width(&self) -> u32 { self.width.load(Ordering::SeqCst) as u32 }
    fn height(&self) -> u32 { self.height.load(Ordering::SeqCst) as u32 }

    fn get_touches(&self) -> Vec<&dyn TouchPoint> {
        let mut touches = Vec::new();
        for touch_option in &self.touches {
            if let Some(ref touch) = *touch_option {
                touches.push(touch.as_ref());
            }
        }
        touches
    }

    fn set_touch(&mut self, touch: Box<dyn TouchPoint>) {
        self.touches.push(Some(touch));
    }
}

pub trait GestureRecognizer {
    fn recognize_tap(&self, touches: &Vec<&dyn TouchPoint>) -> bool;
    fn recognize_swipe(&self, touches: &Vec<&dyn TouchPoint>) -> bool;
    fn recognize_pinch(&self, touches: &Vec<&dyn TouchPoint>) -> bool;
}

#[repr(C)]
pub struct SimpleGestureRecognizer;

impl SimpleGestureRecognizer {
    pub fn new() -> Self { SimpleGestureRecognizer }
}

impl GestureRecognizer for SimpleGestureRecognizer {
    fn recognize_tap(&self, touches: &Vec<&dyn TouchPoint>) -> bool {
        touches.len() == 1
    }

    fn recognize_swipe(&self, _touches: &Vec<&dyn TouchPoint>) -> bool {
        false
    }

    fn recognize_pinch(&self, touches: &Vec<&dyn TouchPoint>) -> bool {
        touches.len() >= 2
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
