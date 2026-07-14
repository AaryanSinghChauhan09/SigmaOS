#![no_std]
#![no_main]

/// OOP-based eMMC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2036
/// Implements eMMC interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MMCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MMCError { Success = 0, NotFound = 1 }

pub trait MMCDevice {
    fn id(&self) -> MMCID;
    fn size_mb(&self) -> u32;
}

#[repr(C)]
pub struct SimpleMMCDevice {
    pub id: MMCID,
    pub size_mb: AtomicUsize,
}

impl SimpleMMCDevice {
    pub fn new(id: MMCID, size_mb: u32) -> Self {
        SimpleMMCDevice {
            id,
            size_mb: AtomicUsize::new(size_mb as usize),
        }
    }
}

impl MMCDevice for SimpleMMCDevice {
    fn id(&self) -> MMCID { self.id }
    fn size_mb(&self) -> u32 { self.size_mb.load(Ordering::SeqCst) as u32 }
}

pub trait MMCController {
    fn init(&mut self, mmc_id: MMCID) -> Result<(), MMCError>;
    fn read(&self, mmc_id: MMCID, address: u32, buffer: &mut [u8]) -> Result<(), MMCError>;
    def write(&self, mmc_id: MMCID, address: u32, buffer: &[u8]) -> Result<(), MMCError>;
}

#[repr(C)]
pub struct SimpleMMCController {
    pub devices: Vec<Option<Box<dyn MMCDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMMCController {
    pub fn new() -> Self {
        SimpleMMCController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MMCController for SimpleMMCController {
    fn init(&mut self, _mmc_id: MMCID) -> Result<(), MMCError> {
        Ok(())
    }
    
    fn read(&self, mmc_id: MMCID, _address: u32, buffer: &mut [u8]) -> Result<(), MMCError> {
        if self.get_device(mmc_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(MMCError::NotFound)
        }
    }
    
    fn write(&self, mmc_id: MMCID, _address: u32, _buffer: &[u8]) -> Result<(), MMCError> {
        if self.get_device(mmc_id).is_some() {
            Ok(())
        } else {
            Err(MMCError::NotFound)
        }
    }
    
    fn get_device(&self, id: MMCID) -> Option<&dyn MMCDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait MMCBoot {
    def set_boot_partition(&mut self, mmc_id: MMCID, partition: u8) -> Result<(), MMCError>;
    def get_boot_config(&self, mmc_id: MMCID) -> Result<u8, MMCError>;
}

#[repr(C)]
pub struct SimpleMMCBoot {
    pub controller: SimpleMMCController,
    pub boot_partitions: Vec<(MMCID, AtomicUsize)>,
}

impl SimpleMMCBoot {
    pub fn new(controller: SimpleMMCController) -> Self {
        SimpleMMCBoot {
            controller,
            boot_partitions: Vec::new(),
        }
    }
}

impl MMCBoot for SimpleMMCBoot {
    fn set_boot_partition(&mut self, mmc_id: MMCID, partition: u8) -> Result<(), MMCError> {
        self.boot_partitions.push((mmc_id, AtomicUsize::new(partition as usize)));
        Ok(())
    }
    
    fn get_boot_config(&self, mmc_id: MMCID) -> Result<u8, MMCError> {
        for &(id, ref part) in &self.boot_partitions {
            if id == mmc_id {
                return Ok(part.load(Ordering::SeqCst) as u8);
            }
        }
        Err(MMCError::NotFound)
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
