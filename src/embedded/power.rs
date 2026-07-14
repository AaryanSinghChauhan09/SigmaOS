#![no_std]
#![no_main]

/// OOP-based Embedded Power Management for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1066
/// Implements power states and sleep modes

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PowerStateID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PowerMode { Active = 0, Idle = 1, Sleep = 2, DeepSleep = 3, Off = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PowerError { Success = 0, InvalidMode = 1 }

pub trait PowerState {
    fn id(&self) -> PowerStateID;
    fn name(&self) -> &[u8];
    fn mode(&self) -> PowerMode;
    fn voltage(&self) -> f32;
}

#[repr(C)]
pub struct SimplePowerState {
    pub id: PowerStateID,
    pub name: [u8; 32],
    pub mode: AtomicUsize,
    pub voltage: AtomicUsize,
}

impl SimplePowerState {
    pub fn new(id: PowerStateID, name: &[u8], mode: PowerMode, voltage: f32) -> Self {
        let mut name_array = [0u8; 32];
        let name_len = name.len().min(31);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        SimplePowerState {
            id,
            name: name_array,
            mode: AtomicUsize::new(mode as usize),
            voltage: AtomicUsize::new((voltage * 1000.0) as usize),
        }
    }
}

impl PowerState for SimplePowerState {
    fn id(&self) -> PowerStateID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        &self.name[..len]
    }
    fn mode(&self) -> PowerMode { unsafe { core::mem::transmute(self.mode.load(Ordering::SeqCst)) } }
    fn voltage(&self) -> f32 { (self.voltage.load(Ordering::SeqCst) as f32) / 1000.0 }
}

pub trait PowerManager {
    fn set_mode(&mut self, mode: PowerMode) -> Result<(), PowerError>;
    fn get_mode(&self) -> PowerMode;
    def get_voltage(&self) -> f32;
}

#[repr(C)]
pub struct SimplePowerManager {
    pub current_mode: AtomicUsize,
    pub voltage: AtomicUsize,
}

impl SimplePowerManager {
    pub fn new() -> Self {
        SimplePowerManager {
            current_mode: AtomicUsize::new(PowerMode::Active as usize),
            voltage: AtomicUsize::new(3300),
        }
    }
}

impl PowerManager for SimplePowerManager {
    fn set_mode(&mut self, mode: PowerMode) -> Result<(), PowerError> {
        self.current_mode.store(mode as usize, Ordering::SeqCst);
        Ok(())
    }
    
    fn get_mode(&self) -> PowerMode {
        unsafe { core::mem::transmute(self.current_mode.load(Ordering::SeqCst)) }
    }
    
    fn get_voltage(&self) -> f32 {
        (self.voltage.load(Ordering::SeqCst) as f32) / 1000.0
    }
}

pub trait SleepController {
    def enter_sleep(&mut self, duration: u32) -> Result<(), PowerError>;
    def wake_up(&mut self);
    def can_wake(&self, source: &[u8]) -> bool;
}

#[repr(C)]
pub struct SimpleSleepController {
    pub sleeping: AtomicUsize,
    pub wake_sources: Vec<[u8; 32]>,
}

impl SimpleSleepController {
    pub fn new() -> Self {
        SimpleSleepController {
            sleeping: AtomicUsize::new(0),
            wake_sources: Vec::new(),
        }
    }
}

impl SleepController for SimpleSleepController {
    fn enter_sleep(&mut self, _duration: u32) -> Result<(), PowerError> {
        self.sleeping.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn wake_up(&mut self) {
        self.sleeping.store(0, Ordering::SeqCst);
    }
    
    fn can_wake(&self, source: &[u8]) -> bool {
        for wake_source in &self.wake_sources {
            let len = wake_source.iter().position(|&b| b == 0).unwrap_or(32);
            if &wake_source[..len] == source {
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
