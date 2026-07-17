#![no_std]
#![no_main]

/// OOP-based Input Event System for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 696
/// Implements input event handling and dispatching

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EventID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InputEventType { KeyPress = 0, KeyRelease = 1, MouseMove = 2, MouseClick = 3, MouseScroll = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InputError { Success = 0, InvalidEvent = 1 }

pub trait InputEvent {
    fn id(&self) -> EventID;
    fn event_type(&self) -> InputEventType;
    fn timestamp(&self) -> u64;
}

#[repr(C)]
pub struct SimpleInputEvent {
    pub id: EventID,
    pub event_type: AtomicUsize,
    pub timestamp: AtomicUsize,
}

impl SimpleInputEvent {
    pub fn new(id: EventID, event_type: InputEventType) -> Self {
        SimpleInputEvent {
            id,
            event_type: AtomicUsize::new(event_type as usize),
            timestamp: AtomicUsize::new(1000000),
        }
    }
}

impl InputEvent for SimpleInputEvent {
    fn id(&self) -> EventID { self.id }
    fn event_type(&self) -> InputEventType { unsafe { core::mem::transmute(self.event_type.load(Ordering::SeqCst)) } }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
}

pub trait InputDispatcher {
    fn dispatch(&mut self, event: Box<dyn InputEvent>) -> Result<(), InputError>;
    fn register_handler(&mut self, event_type: InputEventType, handler: fn(&dyn InputEvent));
    fn get_handlers(&self, event_type: InputEventType) -> Vec<fn(&dyn InputEvent)>;
}

#[repr(C)]
pub struct SimpleInputDispatcher {
    pub handlers: Vec<(InputEventType, Vec<fn(&dyn InputEvent)>)>,
}

impl SimpleInputDispatcher {
    pub fn new() -> Self {
        SimpleInputDispatcher {
            handlers: Vec::new(),
        }
    }
}

impl InputDispatcher for SimpleInputDispatcher {
    fn dispatch(&mut self, event: Box<dyn InputEvent>) -> Result<(), InputError> {
        for &(event_type, ref handlers) in &self.handlers {
            if event_type == event.event_type() {
                for &handler in handlers {
                    handler(event.as_ref());
                }
            }
        }
        Ok(())
    }

    fn register_handler(&mut self, event_type: InputEventType, handler: fn(&dyn InputEvent)) {
        for &mut (et, ref mut handlers) in &mut self.handlers {
            if et == event_type {
                handlers.push(handler);
                return;
            }
        }
        self.handlers.push((event_type, vec![handler]));
    }

    fn get_handlers(&self, event_type: InputEventType) -> Vec<fn(&dyn InputEvent)> {
        for &(et, ref handlers) in &self.handlers {
            if et == event_type {
                return handlers.clone();
            }
        }
        Vec::new()
    }
}

pub trait GestureRecognizer {
    fn recognize(&mut self, events: &Vec<Box<dyn InputEvent>>) -> Vec<&[u8]>;
    fn add_gesture(&mut self, name: &[u8], pattern: Vec<InputEventType>);
}

#[repr(C)]
pub struct SimpleGestureRecognizer {
    pub gestures: Vec<([u8; 64], Vec<InputEventType>)>,
}

impl SimpleGestureRecognizer {
    pub fn new() -> Self {
        SimpleGestureRecognizer {
            gestures: Vec::new(),
        }
    }
}

impl GestureRecognizer for SimpleGestureRecognizer {
    fn recognize(&mut self, _events: &Vec<Box<dyn InputEvent>>) -> Vec<&[u8]> {
        let mut recognized = Vec::new();
        for &(ref name, _) in &self.gestures {
            let len = name.iter().position(|&b| b == 0).unwrap_or(64);
            recognized.push(&name[..len]);
        }
        recognized
    }

    fn add_gesture(&mut self, name: &[u8], pattern: Vec<InputEventType>) {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.gestures.push((name_array, pattern));
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
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
