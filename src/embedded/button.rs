#![no_std]
#![no_main]

/// OOP-based Button for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1426
/// Implements button input

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ButtonID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ButtonState { Released = 0, Pressed = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ButtonError { Success = 0, NotFound = 1 }

pub trait Button {
    fn id(&self) -> ButtonID;
    fn is_pressed(&self) -> bool;
}

#[repr(C)]
pub struct SimpleButton {
    pub id: ButtonID,
    pub state: AtomicUsize,
}

impl SimpleButton {
    pub fn new(id: ButtonID) -> Self {
        SimpleButton {
            id,
            state: AtomicUsize::new(ButtonState::Released as usize),
        }
    }
}

impl Button for SimpleButton {
    fn id(&self) -> ButtonID { self.id }
    fn is_pressed(&self) -> bool { self.state.load(Ordering::SeqCst) == ButtonState::Pressed as usize }
}

pub trait ButtonController {
    fn read(&self, button_id: ButtonID) -> Result<bool, ButtonError>;
    def wait_for_press(&self, button_id: ButtonID);
}

#[repr(C)]
pub struct SimpleButtonController {
    pub buttons: Vec<Option<Box<dyn Button>>>,
    pub next_id: AtomicUsize,
}

impl SimpleButtonController {
    pub fn new() -> Self {
        SimpleButtonController {
            buttons: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ButtonController for SimpleButtonController {
    fn read(&self, button_id: ButtonID) -> Result<bool, ButtonError> {
        for button_option in &self.buttons {
            if let Some(ref button) = *button_option {
                if button.id() == button_id {
                    return Ok(button.is_pressed());
                }
            }
        }
        Err(ButtonError::NotFound)
    }
    
    fn wait_for_press(&self, button_id: ButtonID) {
        while !self.read(button_id).unwrap_or(false) {}
    }
}

pub trait Debounce {
    def set_debounce_time(&mut self, time_ms: u32);
    def is_debounced(&self, button_id: ButtonID) -> bool;
}

#[repr(C)]
pub struct SimpleDebounce {
    pub controller: SimpleButtonController,
    pub debounce_time: AtomicUsize,
    pub last_press: Vec<(ButtonID, AtomicUsize)>,
}

impl SimpleDebounce {
    pub fn new(controller: SimpleButtonController) -> Self {
        SimpleDebounce {
            controller,
            debounce_time: AtomicUsize::new(50),
            last_press: Vec::new(),
        }
    }
}

impl Debounce for SimpleDebounce {
    fn set_debounce_time(&mut self, time_ms: u32) {
        self.debounce_time.store(time_ms as usize, Ordering::SeqCst);
    }
    
    fn is_debounced(&self, button_id: ButtonID) -> bool {
        self.controller.read(button_id).unwrap_or(false)
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
