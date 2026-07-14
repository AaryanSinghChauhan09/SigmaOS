#![no_std]
#![no_main]

/// OOP-based Backup SRAM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1236
/// Implements backup SRAM for data retention

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SRAMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SRAMError { Success = 0, NotFound = 1 }

pub trait BackupSRAM {
    fn id(&self) -> SRAMID;
    fn size(&self) -> u32;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBackupSRAM {
    pub id: SRAMID,
    pub size: AtomicUsize,
    pub enabled: AtomicUsize,
}

impl SimpleBackupSRAM {
    pub fn new(id: SRAMID, size: u32) -> Self {
        SimpleBackupSRAM {
            id,
            size: AtomicUsize::new(size as usize),
            enabled: AtomicUsize::new(0),
        }
    }
}

impl BackupSRAM for SimpleBackupSRAM {
    fn id(&self) -> SRAMID { self.id }
    fn size(&self) -> u32 { self.size.load(Ordering::SeqCst) as u32 }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait BackupMemory {
    fn enable(&mut self, sram_id: SRAMID) -> Result<(), SRAMError>;
    fn write(&self, sram_id: SRAMID, offset: u32, data: &[u8]) -> Result<(), SRAMError>;
    fn read(&self, sram_id: SRAMID, offset: u32, buffer: &mut [u8]) -> Result<(), SRAMError>;
}

#[repr(C)]
pub struct SimpleBackupMemory {
    pub srams: Vec<Option<Box<dyn BackupSRAM>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBackupMemory {
    pub fn new() -> Self {
        SimpleBackupMemory {
            srams: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BackupMemory for SimpleBackupMemory {
    fn enable(&mut self, sram_id: SRAMID) -> Result<(), SRAMError> {
        for sram_option in &mut self.srams {
            if let Some(ref mut sram) = *sram_option {
                if sram.id() == sram_id {
                    sram.enabled.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SRAMError::NotFound)
    }
    
    fn write(&self, sram_id: SRAMID, _offset: u32, _data: &[u8]) -> Result<(), SRAMError> {
        if self.get_sram(sram_id).is_some() {
            Ok(())
        } else {
            Err(SRAMError::NotFound)
        }
    }
    
    fn read(&self, sram_id: SRAMID, _offset: u32, buffer: &mut [u8]) -> Result<(), SRAMError> {
        if self.get_sram(sram_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(SRAMError::NotFound)
        }
    }
    
    fn get_sram(&self, id: SRAMID) -> Option<&dyn BackupSRAM> {
        for sram_option in &self.srams {
            if let Some(ref sram) = *sram_option {
                if sram.id() == id { return Some(sram.as_ref()); }
            }
        }
        None
    }
}

pub trait DataRetention {
    def set_retention_mode(&mut self, mode: u8);
    def get_retention_mode(&self) -> u8;
}

#[repr(C)]
pub struct SimpleDataRetention {
    pub retention_mode: AtomicUsize,
}

impl SimpleDataRetention {
    pub fn new() -> Self {
        SimpleDataRetention {
            retention_mode: AtomicUsize::new(0),
        }
    }
}

impl DataRetention for SimpleDataRetention {
    fn set_retention_mode(&mut self, mode: u8) {
        self.retention_mode.store(mode as usize, Ordering::SeqCst);
    }
    
    fn get_retention_mode(&self) -> u8 {
        self.retention_mode.load(Ordering::SeqCst) as u8
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
