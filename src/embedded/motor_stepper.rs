#![no_std]
#![no_main]

/// OOP-based Stepper Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2696
/// Implements stepper motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type StepperID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum StepperError { Success = 0, NotFound = 1 }

pub trait StepperMotor {
    fn id(&self) -> StepperID;
    fn position(&self) -> i32;
}

#[repr(C)]
pub struct SimpleStepperMotor {
    pub id: StepperID,
    pub position: AtomicUsize,
}

impl SimpleStepperMotor {
    pub fn new(id: StepperID) -> Self {
        SimpleStepperMotor {
            id,
            position: AtomicUsize::new(0),
        }
    }
}

impl StepperMotor for SimpleStepperMotor {
    fn id(&self) -> StepperID { self.id }
    fn position(&self) -> i32 { self.position.load(Ordering::SeqCst) as i32 }
}

pub trait StepperController {
    fn step(&self, stepper_id: StepperID, steps: i32) -> Result<(), StepperError>;
    def set_speed(&mut self, stepper_id: StepperID, rpm: u16) -> Result<(), StepperError>;
    def home(&self, stepper_id: StepperID) -> Result<(), StepperError>;
}

#[repr(C)]
pub struct SimpleStepperController {
    pub steppers: Vec<Option<Box<dyn StepperMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleStepperController {
    pub fn new() -> Self {
        SimpleStepperController {
            steppers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl StepperController for SimpleStepperController {
    fn step(&self, stepper_id: StepperID, steps: i32) -> Result<(), StepperError> {
        for stepper_option in &self.steppers {
            if let Some(ref stepper) = *stepper_option {
                if stepper.id() == stepper_id {
                    let current = stepper.position.load(Ordering::SeqCst) as i32;
                    stepper.position.store((current + steps) as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(StepperError::NotFound)
    }
    
    fn set_speed(&mut self, _stepper_id: StepperID, _rpm: u16) -> Result<(), StepperError> {
        Ok(())
    }
    
    fn home(&self, stepper_id: StepperID) -> Result<(), StepperError> {
        for stepper_option in &self.steppers {
            if let Some(ref stepper) = *stepper_option {
                if stepper.id() == stepper_id {
                    stepper.position.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(StepperError::NotFound)
    }
    
    fn get_stepper(&self, id: StepperID) -> Option<&dyn StepperMotor> {
        for stepper_option in &self.steppers {
            if let Some(ref stepper) = *stepper_option {
                if stepper.id() == id { return Some(stepper.as_ref()); }
            }
        }
        None
    }
}

pub trait StepperMicrostep {
    def set_microstep(&mut self, stepper_id: StepperID, mode: u8) -> Result<(), StepperError>;
    def get_microstep(&self, stepper_id: StepperID) -> Result<u8, StepperError>;
}

#[repr(C)]
pub struct SimpleStepperMicrostep {
    pub controller: SimpleStepperController,
    pub microsteps: Vec<(StepperID, AtomicUsize)>,
}

impl SimpleStepperMicrostep {
    pub fn new(controller: SimpleStepperController) -> Self {
        SimpleStepperMicrostep {
            controller,
            microsteps: Vec::new(),
        }
    }
}

impl StepperMicrostep for SimpleStepperMicrostep {
    fn set_microstep(&mut self, stepper_id: StepperID, mode: u8) -> Result<(), StepperError> {
        self.microsteps.push((stepper_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_microstep(&self, stepper_id: StepperID) -> Result<u8, StepperError> {
        for &(id, ref mode) in &self.microsteps {
            if id == stepper_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(StepperError::NotFound)
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
