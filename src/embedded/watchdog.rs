#![no_std]
#![no_main]

/// OOP-based Watchdog Timer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1056
/// Implements watchdog timer and system reset

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WatchdogID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WatchdogState { Disabled = 0, Enabled = 1, Triggered = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WatchdogError { Success = 0, NotFound = 1 }

pub trait Watchdog {
    fn id(&self) -> WatchdogID;
    fn timeout(&self) -> u32;
    fn state(&self) -> WatchdogState;
}

#[repr(C)]
pub struct SimpleWatchdog {
    pub id: WatchdogID,
    pub timeout: AtomicUsize,
    pub state: AtomicUsize,
}

impl SimpleWatchdog {
    pub fn new(id: WatchdogID, timeout: u32) -> Self {
        SimpleWatchdog {
            id,
            timeout: AtomicUsize::new(timeout as usize),
            state: AtomicUsize::new(WatchdogState::Disabled as usize),
        }
    }
}

impl Watchdog for SimpleWatchdog {
    fn id(&self) -> WatchdogID { self.id }
    fn timeout(&self) -> u32 { self.timeout.load(Ordering::SeqCst) as u32 }
    fn state(&self) -> WatchdogState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait WatchdogManager {
    fn create(&mut self, timeout: u32) -> Result<WatchdogID, WatchdogError>;
    def enable(&mut self, id: WatchdogID) -> Result<(), WatchdogError>;
    def disable(&mut self, id: WatchdogID) -> Result<(), WatchdogError>;
    def kick(&mut self, id: WatchdogID) -> Result<(), WatchdogError>;
}

#[repr(C)]
pub struct SimpleWatchdogManager {
    pub watchdogs: Vec<Option<Box<dyn Watchdog>>>,
    pub next_id: AtomicUsize,
}

impl SimpleWatchdogManager {
    pub fn new() -> Self {
        SimpleWatchdogManager {
            watchdogs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WatchdogManager for SimpleWatchdogManager {
    fn create(&mut self, timeout: u32) -> Result<WatchdogID, WatchdogError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let watchdog = SimpleWatchdog::new(id, timeout);
        self.watchdogs.push(Some(Box::new(watchdog)));
        Ok(id)
    }
    
    fn enable(&mut self, id: WatchdogID) -> Result<(), WatchdogError> {
        for watchdog_option in &mut self.watchdogs {
            if let Some(ref mut watchdog) = *watchdog_option {
                if watchdog.id() == id {
                    watchdog.state.store(WatchdogState::Enabled as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WatchdogError::NotFound)
    }
    
    fn disable(&mut self, id: WatchdogID) -> Result<(), WatchdogError> {
        for watchdog_option in &mut self.watchdogs {
            if let Some(ref mut watchdog) = *watchdog_option {
                if watchdog.id() == id {
                    watchdog.state.store(WatchdogState::Disabled as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WatchdogError::NotFound)
    }
    
    fn kick(&mut self, id: WatchdogID) -> Result<(), WatchdogError> {
        for watchdog_option in &mut self.watchdogs {
            if let Some(ref watchdog) = *watchdog_option {
                if watchdog.id() == id && watchdog.state() == WatchdogState::Enabled {
                    return Ok(());
                }
            }
        }
        Err(WatchdogError::NotFound)
    }
}

pub trait SystemReset {
    def trigger_reset(&mut self) -> !;
    def get_reset_reason(&self) -> u32;
}

#[repr(C)]
pub struct SimpleSystemReset {
    pub reset_reason: AtomicUsize,
}

impl SimpleSystemReset {
    pub fn new() -> Self {
        SimpleSystemReset {
            reset_reason: AtomicUsize::new(0),
        }
    }
}

impl SystemReset for SimpleSystemReset {
    fn trigger_reset(&mut self) -> ! {
        self.reset_reason.store(1, Ordering::SeqCst);
        loop {}
    }
    
    fn get_reset_reason(&self) -> u32 {
        self.reset_reason.load(Ordering::SeqCst) as u32
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
