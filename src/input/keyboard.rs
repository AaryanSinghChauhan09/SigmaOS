#![no_std]
#![no_main]

/// OOP-based Keyboard Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 71
/// Implements keyboard input handling and key mapping

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyCode = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyState { Released = 0, Pressed = 1, Repeated = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Modifier { Shift = 1, Ctrl = 2, Alt = 4, Super = 8 }

pub trait KeyboardDevice {
    fn read_key(&mut self) -> Option<(KeyCode, KeyState)>;
    fn get_modifiers(&self) -> u8;
    fn set_leds(&mut self, caps: bool, num: bool, scroll: bool);
}

#[repr(C)]
pub struct SimpleKeyboardDevice {
    pub modifiers: AtomicUsize,
    pub leds: AtomicUsize,
}

impl SimpleKeyboardDevice {
    pub fn new() -> Self {
        SimpleKeyboardDevice {
            modifiers: AtomicUsize::new(0),
            leds: AtomicUsize::new(0),
        }
    }
}

impl KeyboardDevice for SimpleKeyboardDevice {
    fn read_key(&mut self) -> Option<(KeyCode, KeyState)> {
        None
    }
    
    fn get_modifiers(&self) -> u8 { self.modifiers.load(Ordering::SeqCst) as u8 }
    
    fn set_leds(&mut self, caps: bool, num: bool, scroll: bool) {
        let mut leds = 0;
        if caps { leds |= 1; }
        if num { leds |= 2; }
        if scroll { leds |= 4; }
        self.leds.store(leds, Ordering::SeqCst);
    }
}

pub trait KeyMapper {
    fn map_scancode(&self, scancode: KeyCode) -> char;
    fn set_layout(&mut self, layout: &[u8]);
}

#[repr(C)]
pub struct SimpleKeyMapper {
    pub layout: [u8; 32],
}

impl SimpleKeyMapper {
    pub fn new() -> Self {
        SimpleKeyMapper {
            layout: *b"us-qwerty",
        }
    }
}

impl KeyMapper for SimpleKeyMapper {
    fn map_scancode(&self, scancode: KeyCode) -> char {
        match scancode {
            4 => 'a',
            5 => 'b',
            6 => 'c',
            16 => 'q',
            17 => 'w',
            18 => 'e',
            30 => '1',
            31 => '2',
            32 => '3',
            _ => '\0',
        }
    }
    
    fn set_layout(&mut self, layout: &[u8]) {
        let mut layout_array = [0u8; 32];
        let layout_len = layout.len().min(31);
        for i in 0..layout_len {
            layout_array[i] = layout[i];
        }
        self.layout = layout_array;
    }
}

pub trait InputBuffer {
    fn push_key(&mut self, key: char);
    fn pop_key(&mut self) -> Option<char>;
    fn peek_key(&self) -> Option<char>;
    fn is_empty(&self) -> bool;
}

#[repr(C)]
pub struct SimpleInputBuffer {
    pub buffer: Vec<char>,
    pub size: AtomicUsize,
}

impl SimpleInputBuffer {
    pub fn new(size: usize) -> Self {
        SimpleInputBuffer {
            buffer: Vec::new(),
            size: AtomicUsize::new(size),
        }
    }
}

impl InputBuffer for SimpleInputBuffer {
    fn push_key(&mut self, key: char) {
        let max = self.size.load(Ordering::SeqCst);
        if self.buffer.len() < max {
            self.buffer.push(key);
        }
    }
    
    fn pop_key(&mut self) -> Option<char> {
        if !self.buffer.is_empty() {
            Some(self.buffer.remove(0))
        } else {
            None
        }
    }
    
    fn peek_key(&self) -> Option<char> {
        if !self.buffer.is_empty() {
            Some(self.buffer[0])
        } else {
            None
        }
    }
    
    fn is_empty(&self) -> bool { self.buffer.is_empty() }
}

pub trait KeyboardHandler {
    fn handle_key_event(&mut self, key: KeyCode, state: KeyState, modifiers: u8);
    fn register_callback(&mut self, callback: fn(KeyCode, KeyState, u8));
}

#[repr(C)]
pub struct SimpleKeyboardHandler {
    pub callbacks: Vec<fn(KeyCode, KeyState, u8)>,
}

impl SimpleKeyboardHandler {
    pub fn new() -> Self {
        SimpleKeyboardHandler {
            callbacks: Vec::new(),
        }
    }
}

impl KeyboardHandler for SimpleKeyboardHandler {
    fn handle_key_event(&mut self, key: KeyCode, state: KeyState, modifiers: u8) {
        for &callback in &self.callbacks {
            callback(key, state, modifiers);
        }
    }
    
    fn register_callback(&mut self, callback: fn(KeyCode, KeyState, u8)) {
        self.callbacks.push(callback);
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
    fn is_empty(&self) -> bool { self.len == 0 }
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
