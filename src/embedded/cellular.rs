#![no_std]
#![no_main]

/// OOP-based Cellular for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2016
/// Implements cellular module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CellID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CellError { Success = 0, NotFound = 1 }

pub trait CellularModule {
    fn id(&self) -> CellID;
    fn is_registered(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCellularModule {
    pub id: CellID,
    pub registered: AtomicUsize,
}

impl SimpleCellularModule {
    pub fn new(id: CellID) -> Self {
        SimpleCellularModule {
            id,
            registered: AtomicUsize::new(0),
        }
    }
}

impl CellularModule for SimpleCellularModule {
    fn id(&self) -> CellID { self.id }
    fn is_registered(&self) -> bool { self.registered.load(Ordering::SeqCst) == 1 }
}

pub trait CellularController {
    fn power_on(&mut self, cell_id: CellID) -> Result<(), CellError>;
    fn get_signal(&self, cell_id: CellID) -> Result<i8, CellError>;
    def get_imei(&self, cell_id: CellID) -> Result<[u8; 15], CellError>;
}

#[repr(C)]
pub struct SimpleCellularController {
    pub modules: Vec<Option<Box<dyn CellularModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCellularController {
    pub fn new() -> Self {
        SimpleCellularController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CellularController for SimpleCellularController {
    fn power_on(&mut self, cell_id: CellID) -> Result<(), CellError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == cell_id {
                    module.registered.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(CellError::NotFound)
    }
    
    fn get_signal(&self, cell_id: CellID) -> Result<i8, CellError> {
        if self.get_module(cell_id).is_some() {
            Ok(0)
        } else {
            Err(CellError::NotFound)
        }
    }
    
    fn get_imei(&self, cell_id: CellID) -> Result<[u8; 15], CellError> {
        if self.get_module(cell_id).is_some() {
            Ok([b'0'; 15])
        } else {
            Err(CellError::NotFound)
        }
    }
    
    fn get_module(&self, id: CellID) -> Option<&dyn CellularModule> {
        for module_option in &self.modules {
            if let Some(ref module) = *module_option {
                if module.id() == id { return Some(module.as_ref()); }
            }
        }
        None
    }
}

pub trait CellularData {
    def connect(&mut self, cell_id: CellID, apn: &[u8]) -> Result<(), CellError>;
    def send_sms(&self, cell_id: CellID, number: &[u8], message: &[u8]) -> Result<(), CellError>;
}

#[repr(C)]
pub struct SimpleCellularData {
    pub controller: SimpleCellularController,
}

impl SimpleCellularData {
    pub fn new(controller: SimpleCellularController) -> Self {
        SimpleCellularData { controller }
    }
}

impl CellularData for SimpleCellularData {
    fn connect(&mut self, _cell_id: CellID, _apn: &[u8]) -> Result<(), CellError> {
        Ok(())
    }
    
    fn send_sms(&self, _cell_id: CellID, _number: &[u8], _message: &[u8]) -> Result<(), CellError> {
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
