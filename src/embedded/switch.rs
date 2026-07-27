#![no_std]
#![no_main]

/// OOP-based Switch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1436
/// Implements switch input

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SwitchID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SwitchState { Off = 0, On = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SwitchError { Success = 0, NotFound = 1 }

pub trait Switch {
    fn id(&self) -> SwitchID;
    fn state(&self) -> SwitchState;
}

#[repr(C)]
pub struct SimpleSwitch {
    pub id: SwitchID,
    pub state: AtomicUsize,
}

impl SimpleSwitch {
    pub fn new(id: SwitchID) -> Self {
        SimpleSwitch {
            id,
            state: AtomicUsize::new(SwitchState::Off as usize),
        }
    }
}

impl Switch for SimpleSwitch {
    fn id(&self) -> SwitchID { self.id }
    fn state(&self) -> SwitchState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait SwitchController {
    fn read(&self, switch_id: SwitchID) -> Result<SwitchState, SwitchError>;
    def toggle(&mut self, switch_id: SwitchID) -> Result<(), SwitchError>;
}

#[repr(C)]
pub struct SimpleSwitchController {
    pub switches: Vec<Option<Box<dyn Switch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSwitchController {
    pub fn new() -> Self {
        SimpleSwitchController {
            switches: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SwitchController for SimpleSwitchController {
    fn read(&self, switch_id: SwitchID) -> Result<SwitchState, SwitchError> {
        for switch_option in &self.switches {
            if let Some(ref sw) = *switch_option {
                if sw.id() == switch_id {
                    return Ok(sw.state());
                }
            }
        }
        Err(SwitchError::NotFound)
    }
    
    fn toggle(&mut self, switch_id: SwitchID) -> Result<(), SwitchError> {
        for switch_option in &mut self.switches {
            if let Some(ref mut sw) = *switch_option {
                if sw.id() == switch_id {
                    let current = sw.state.load(Ordering::SeqCst);
                    let new_state = if current == SwitchState::Off as usize {
                        SwitchState::On as usize
                    } else {
                        SwitchState::Off as usize
                    };
                    sw.state.store(new_state, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SwitchError::NotFound)
    }
}

pub trait RotarySwitch {
    def get_position(&self, switch_id: SwitchID) -> Result<u8, SwitchError>;
    def set_position(&mut self, switch_id: SwitchID, position: u8) -> Result<(), SwitchError>;
}

#[repr(C)]
pub struct SimpleRotarySwitch {
    pub controller: SimpleSwitchController,
    pub positions: Vec<(SwitchID, AtomicUsize)>,
}

impl SimpleRotarySwitch {
    pub fn new(controller: SimpleSwitchController) -> Self {
        SimpleRotarySwitch {
            controller,
            positions: Vec::new(),
        }
    }
}

impl RotarySwitch for SimpleRotarySwitch {
    fn get_position(&self, switch_id: SwitchID) -> Result<u8, SwitchError> {
        for &(id, ref pos) in &self.positions {
            if id == switch_id {
                return Ok(pos.load(Ordering::SeqCst) as u8);
            }
        }
        Err(SwitchError::NotFound)
    }
    
    fn set_position(&mut self, switch_id: SwitchID, position: u8) -> Result<(), SwitchError> {
        self.positions.push((switch_id, AtomicUsize::new(position as usize)));
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
