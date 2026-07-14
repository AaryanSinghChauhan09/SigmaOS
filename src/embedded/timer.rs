#![no_std]
#![no_main]

/// OOP-based Hardware Timer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1216
/// Implements hardware timers

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TimerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TimerMode { OneShot = 0, Periodic = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TimerError { Success = 0, NotFound = 1 }

pub trait HardwareTimer {
    fn id(&self) -> TimerID;
    fn counter(&self) -> u32;
    fn is_running(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHardwareTimer {
    pub id: TimerID,
    pub counter: AtomicUsize,
    pub running: AtomicUsize,
}

impl SimpleHardwareTimer {
    pub fn new(id: TimerID) -> Self {
        SimpleHardwareTimer {
            id,
            counter: AtomicUsize::new(0),
            running: AtomicUsize::new(0),
        }
    }
}

impl HardwareTimer for SimpleHardwareTimer {
    fn id(&self) -> TimerID { self.id }
    fn counter(&self) -> u32 { self.counter.load(Ordering::SeqCst) as u32 }
    fn is_running(&self) -> bool { self.running.load(Ordering::SeqCst) == 1 }
}

pub trait TimerController {
    fn start(&mut self, timer_id: TimerID, mode: TimerMode, period: u32) -> Result<(), TimerError>;
    fn stop(&mut self, timer_id: TimerID) -> Result<(), TimerError>;
    fn set_callback(&mut self, timer_id: TimerID, callback: fn());
}

#[repr(C)]
pub struct SimpleTimerController {
    pub timers: Vec<Option<Box<dyn HardwareTimer>>>,
    pub callbacks: Vec<(TimerID, fn())>,
    pub next_id: AtomicUsize,
}

impl SimpleTimerController {
    pub fn new() -> Self {
        SimpleTimerController {
            timers: Vec::new(),
            callbacks: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TimerController for SimpleTimerController {
    fn start(&mut self, timer_id: TimerID, _mode: TimerMode, _period: u32) -> Result<(), TimerError> {
        for timer_option in &mut self.timers {
            if let Some(ref mut timer) = *timer_option {
                if timer.id() == timer_id {
                    timer.running.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TimerError::NotFound)
    }
    
    fn stop(&mut self, timer_id: TimerID) -> Result<(), TimerError> {
        for timer_option in &mut self.timers {
            if let Some(ref mut timer) = *timer_option {
                if timer.id() == timer_id {
                    timer.running.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TimerError::NotFound)
    }
    
    fn set_callback(&mut self, timer_id: TimerID, callback: fn()) {
        self.callbacks.push((timer_id, callback));
    }
}

pub trait PWMOutput {
    def set_duty_cycle(&mut self, timer_id: TimerID, duty: f32) -> Result<(), TimerError>;
    def get_duty_cycle(&self, timer_id: TimerID) -> Result<f32, TimerError>;
}

#[repr(C)]
pub struct SimplePWMOutput {
    pub controller: SimpleTimerController,
    pub duty_cycles: Vec<(TimerID, AtomicUsize)>,
}

impl SimplePWMOutput {
    pub fn new(controller: SimpleTimerController) -> Self {
        SimplePWMOutput {
            controller,
            duty_cycles: Vec::new(),
        }
    }
}

impl PWMOutput for SimplePWMOutput {
    fn set_duty_cycle(&mut self, timer_id: TimerID, duty: f32) -> Result<(), TimerError> {
        self.duty_cycles.push((timer_id, AtomicUsize::new((duty * 1000.0) as usize)));
        Ok(())
    }
    
    fn get_duty_cycle(&self, timer_id: TimerID) -> Result<f32, TimerError> {
        for &(id, ref duty) in &self.duty_cycles {
            if id == timer_id {
                return Ok((duty.load(Ordering::SeqCst) as f32) / 1000.0);
            }
        }
        Err(TimerError::NotFound)
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
