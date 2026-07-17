#![no_std]
#![no_main]

/// OOP-based Buzzer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1626
/// Implements buzzer control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BuzzerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BuzzerState { Off = 0, On = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BuzzerError { Success = 0, NotFound = 1 }

pub trait Buzzer {
    fn id(&self) -> BuzzerID;
    fn is_on(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBuzzer {
    pub id: BuzzerID,
    pub on: AtomicUsize,
}

impl SimpleBuzzer {
    pub fn new(id: BuzzerID) -> Self {
        SimpleBuzzer {
            id,
            on: AtomicUsize::new(0),
        }
    }
}

impl Buzzer for SimpleBuzzer {
    fn id(&self) -> BuzzerID { self.id }
    fn is_on(&self) -> bool { self.on.load(Ordering::SeqCst) == 1 }
}

pub trait BuzzerController {
    fn turn_on(&mut self, buzzer_id: BuzzerID) -> Result<(), BuzzerError>;
    fn turn_off(&mut self, buzzer_id: BuzzerID) -> Result<(), BuzzerError>;
    def beep(&mut self, buzzer_id: BuzzerID, duration_ms: u32) -> Result<(), BuzzerError>;
}

#[repr(C)]
pub struct SimpleBuzzerController {
    pub buzzers: Vec<Option<Box<dyn Buzzer>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBuzzerController {
    pub fn new() -> Self {
        SimpleBuzzerController {
            buzzers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BuzzerController for SimpleBuzzerController {
    fn turn_on(&mut self, buzzer_id: BuzzerID) -> Result<(), BuzzerError> {
        for buzzer_option in &mut self.buzzers {
            if let Some(ref mut buzzer) = *buzzer_option {
                if buzzer.id() == buzzer_id {
                    buzzer.on.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BuzzerError::NotFound)
    }
    
    fn turn_off(&mut self, buzzer_id: BuzzerID) -> Result<(), BuzzerError> {
        for buzzer_option in &mut self.buzzers {
            if let Some(ref mut buzzer) = *buzzer_option {
                if buzzer.id() == buzzer_id {
                    buzzer.on.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BuzzerError::NotFound)
    }
    
    fn beep(&mut self, buzzer_id: BuzzerID, _duration_ms: u32) -> Result<(), BuzzerError> {
        for buzzer_option in &mut self.buzzers {
            if let Some(ref mut buzzer) = *buzzer_option {
                if buzzer.id() == buzzer_id {
                    buzzer.on.store(1, Ordering::SeqCst);
                    buzzer.on.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BuzzerError::NotFound)
    }
}

pub trait ToneGenerator {
    def set_frequency(&mut self, buzzer_id: BuzzerID, frequency: u32) -> Result<(), BuzzerError>;
    def play_tone(&mut self, buzzer_id: BuzzerID, frequency: u32, duration_ms: u32) -> Result<(), BuzzerError>;
}

#[repr(C)]
pub struct SimpleToneGenerator {
    pub controller: SimpleBuzzerController,
    pub frequencies: Vec<(BuzzerID, AtomicUsize)>,
}

impl SimpleToneGenerator {
    pub fn new(controller: SimpleBuzzerController) -> Self {
        SimpleToneGenerator {
            controller,
            frequencies: Vec::new(),
        }
    }
}

impl ToneGenerator for SimpleToneGenerator {
    fn set_frequency(&mut self, buzzer_id: BuzzerID, frequency: u32) -> Result<(), BuzzerError> {
        self.frequencies.push((buzzer_id, AtomicUsize::new(frequency as usize)));
        Ok(())
    }
    
    fn play_tone(&mut self, buzzer_id: BuzzerID, frequency: u32, _duration_ms: u32) -> Result<(), BuzzerError> {
        self.frequencies.push((buzzer_id, AtomicUsize::new(frequency as usize)));
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
