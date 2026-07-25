#![no_std]
#![no_main]

/// OOP-based GPIO for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1086
/// Implements GPIO pin control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PinID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PinMode { Input = 0, Output = 1, Alternate = 2, Analog = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PinState { Low = 0, High = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GPIOError { Success = 0, NotFound = 1, InvalidMode = 2 }

pub trait GPIOPin {
    fn id(&self) -> PinID;
    fn mode(&self) -> PinMode;
    fn set_mode(&mut self, mode: PinMode);
}

#[repr(C)]
pub struct SimpleGPIOPin {
    pub id: PinID,
    pub mode: AtomicUsize,
    pub state: AtomicUsize,
}

impl SimpleGPIOPin {
    pub fn new(id: PinID, mode: PinMode) -> Self {
        SimpleGPIOPin {
            id,
            mode: AtomicUsize::new(mode as usize),
            state: AtomicUsize::new(PinState::Low as usize),
        }
    }
}

impl GPIOPin for SimpleGPIOPin {
    fn id(&self) -> PinID { self.id }
    fn mode(&self) -> PinMode { unsafe { core::mem::transmute(self.mode.load(Ordering::SeqCst)) } }
    
    fn set_mode(&mut self, mode: PinMode) {
        self.mode.store(mode as usize, Ordering::SeqCst);
    }
}

pub trait GPIOController {
    fn configure_pin(&mut self, pin_id: PinID, mode: PinMode) -> Result<(), GPIOError>;
    fn write_pin(&mut self, pin_id: PinID, state: PinState) -> Result<(), GPIOError>;
    fn read_pin(&self, pin_id: PinID) -> Result<PinState, GPIOError>;
    def toggle_pin(&mut self, pin_id: PinID) -> Result<(), GPIOError>;
}

#[repr(C)]
pub struct SimpleGPIOController {
    pub pins: Vec<Option<Box<dyn GPIOPin>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGPIOController {
    pub fn new() -> Self {
        SimpleGPIOController {
            pins: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GPIOController for SimpleGPIOController {
    fn configure_pin(&mut self, pin_id: PinID, mode: PinMode) -> Result<(), GPIOError> {
        for pin_option in &mut self.pins {
            if let Some(ref mut pin) = *pin_option {
                if pin.id() == pin_id {
                    pin.set_mode(mode);
                    return Ok(());
                }
            }
        }
        Err(GPIOError::NotFound)
    }
    
    fn write_pin(&mut self, pin_id: PinID, state: PinState) -> Result<(), GPIOError> {
        for pin_option in &mut self.pins {
            if let Some(ref mut pin) = *pin_option {
                if pin.id() == pin_id {
                    if let SimpleGPIOPin { ref mut state_pin, .. } = **pin {
                        state_pin.store(state as usize, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(GPIOError::NotFound)
    }
    
    fn read_pin(&self, pin_id: PinID) -> Result<PinState, GPIOError> {
        for pin_option in &self.pins {
            if let Some(ref pin) = *pin_option {
                if pin.id() == pin_id {
                    if let SimpleGPIOPin { ref state_pin, .. } = **pin {
                        return Ok(unsafe { core::mem::transmute(state_pin.load(Ordering::SeqCst)) });
                    }
                }
            }
        }
        Err(GPIOError::NotFound)
    }
    
    fn toggle_pin(&mut self, pin_id: PinID) -> Result<(), GPIOError> {
        let current = self.read_pin(pin_id)?;
        let new_state = if current == PinState::Low { PinState::High } else { PinState::Low };
        self.write_pin(pin_id, new_state)
    }
}

pub trait InterruptHandler {
    def enable_interrupt(&mut self, pin_id: PinID, trigger: u8);
    def disable_interrupt(&mut self, pin_id: PinID);
    def get_interrupt_count(&self, pin_id: PinID) -> u32;
}

#[repr(C)]
pub struct SimpleInterruptHandler {
    pub interrupts: Vec<(PinID, AtomicUsize)>,
}

impl SimpleInterruptHandler {
    pub fn new() -> Self {
        SimpleInterruptHandler {
            interrupts: Vec::new(),
        }
    }
}

impl InterruptHandler for SimpleInterruptHandler {
    fn enable_interrupt(&mut self, pin_id: PinID, _trigger: u8) {
        self.interrupts.push((pin_id, AtomicUsize::new(0)));
    }
    
    fn disable_interrupt(&mut self, pin_id: PinID) {
        for i in 0..self.interrupts.len() {
            if self.interrupts[i].0 == pin_id {
                self.interrupts.remove(i);
                return;
            }
        }
    }
    
    fn get_interrupt_count(&self, pin_id: PinID) -> u32 {
        for &(id, ref count) in &self.interrupts {
            if id == pin_id {
                return count.load(Ordering::SeqCst) as u32;
            }
        }
        0
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
