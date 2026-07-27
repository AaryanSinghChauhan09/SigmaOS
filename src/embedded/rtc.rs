#![no_std]
#![no_main]

/// OOP-based RTC Backup for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1226
/// Implements RTC backup registers

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RegisterID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RTCError { Success = 0, NotFound = 1 }

pub trait RTCRegister {
    fn id(&self) -> RegisterID;
    fn value(&self) -> u32;
    fn set_value(&mut self, value: u32);
}

#[repr(C)]
pub struct SimpleRTCRegister {
    pub id: RegisterID,
    pub value: AtomicUsize,
}

impl SimpleRTCRegister {
    pub fn new(id: RegisterID, value: u32) -> Self {
        SimpleRTCRegister {
            id,
            value: AtomicUsize::new(value as usize),
        }
    }
}

impl RTCRegister for SimpleRTCRegister {
    fn id(&self) -> RegisterID { self.id }
    fn value(&self) -> u32 { self.value.load(Ordering::SeqCst) as u32 }
    
    fn set_value(&mut self, value: u32) {
        self.value.store(value as usize, Ordering::SeqCst);
    }
}

pub trait RTCBackup {
    fn write_register(&mut self, reg_id: RegisterID, value: u32) -> Result<(), RTCError>;
    fn read_register(&self, reg_id: RegisterID) -> Result<u32, RTCError>;
    fn is_backup_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRTCBackup {
    pub registers: Vec<Option<Box<dyn RTCRegister>>>,
    pub backup_enabled: AtomicUsize,
}

impl SimpleRTCBackup {
    pub fn new() -> Self {
        SimpleRTCBackup {
            registers: Vec::new(),
            backup_enabled: AtomicUsize::new(1),
        }
    }
}

impl RTCBackup for SimpleRTCBackup {
    fn write_register(&mut self, reg_id: RegisterID, value: u32) -> Result<(), RTCError> {
        for reg_option in &mut self.registers {
            if let Some(ref reg) = *reg_option {
                if reg.id() == reg_id {
                    reg.set_value(value);
                    return Ok(());
                }
            }
        }
        let reg = SimpleRTCRegister::new(reg_id, value);
        self.registers.push(Some(Box::new(reg)));
        Ok(())
    }
    
    fn read_register(&self, reg_id: RegisterID) -> Result<u32, RTCError> {
        for reg_option in &self.registers {
            if let Some(ref reg) = *reg_option {
                if reg.id() == reg_id {
                    return Ok(reg.value());
                }
            }
        }
        Err(RTCError::NotFound)
    }
    
    fn is_backup_enabled(&self) -> bool { self.backup_enabled.load(Ordering::SeqCst) == 1 }
}

pub trait TamperDetection {
    def enable_tamper(&mut self, pin: u8);
    def is_tampered(&self) -> bool;
    def clear_tamper(&mut self);
}

#[repr(C)]
pub struct SimpleTamperDetection {
    pub tampered: AtomicUsize,
    pub tamper_pins: Vec<u8>,
}

impl SimpleTamperDetection {
    pub fn new() -> Self {
        SimpleTamperDetection {
            tampered: AtomicUsize::new(0),
            tamper_pins: Vec::new(),
        }
    }
}

impl TamperDetection for SimpleTamperDetection {
    fn enable_tamper(&mut self, pin: u8) {
        self.tamper_pins.push(pin);
    }
    
    fn is_tampered(&self) -> bool {
        self.tampered.load(Ordering::SeqCst) == 1
    }
    
    fn clear_tamper(&mut self) {
        self.tampered.store(0, Ordering::SeqCst);
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
