#![no_std]
#![no_main]

/// OOP-based DFU for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2126
/// Implements DFU (Device Firmware Upgrade)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DFUID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DFUState { Idle = 0, Busy = 1, Error = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DFUError { Success = 0, NotFound = 1 }

pub trait DFUModule {
    fn id(&self) -> DFUID;
    fn state(&self) -> DFUState;
}

#[repr(C)]
pub struct SimpleDFUModule {
    pub id: DFUID,
    pub state: AtomicUsize,
}

impl SimpleDFUModule {
    pub fn new(id: DFUID) -> Self {
        SimpleDFUModule {
            id,
            state: AtomicUsize::new(DFUState::Idle as usize),
        }
    }
}

impl DFUModule for SimpleDFUModule {
    fn id(&self) -> DFUID { self.id }
    fn state(&self) -> DFUState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait DFUController {
    fn start(&mut self, dfu_id: DFUID) -> Result<(), DFUError>;
    fn write(&self, dfu_id: DFUID, address: u32, data: &[u8]) -> Result<(), DFUError>;
    def finish(&mut self, dfu_id: DFUID) -> Result<(), DFUError>;
}

#[repr(C)]
pub struct SimpleDFUController {
    pub modules: Vec<Option<Box<dyn DFUModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDFUController {
    pub fn new() -> Self {
        SimpleDFUController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DFUController for SimpleDFUController {
    fn start(&mut self, dfu_id: DFUID) -> Result<(), DFUError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == dfu_id {
                    module.state.store(DFUState::Busy as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DFUError::NotFound)
    }
    
    fn write(&self, dfu_id: DFUID, _address: u32, _data: &[u8]) -> Result<(), DFUError> {
        if self.get_module(dfu_id).is_some() {
            Ok(())
        } else {
            Err(DFUError::NotFound)
        }
    }
    
    fn finish(&mut self, dfu_id: DFUID) -> Result<(), DFUError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == dfu_id {
                    module.state.store(DFUState::Idle as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DFUError::NotFound)
    }
    
    fn get_module(&self, id: DFUID) -> Option<&dyn DFUModule> {
        for module_option in &self.modules {
            if let Some(ref module) = *module_option {
                if module.id() == id { return Some(module.as_ref()); }
            }
        }
        None
    }
}

pub trait DFUStatus {
    def get_progress(&self, dfu_id: DFUID) -> Result<u8, DFUError>;
    def abort(&mut self, dfu_id: DFUID) -> Result<(), DFUError>;
}

#[repr(C)]
pub struct SimpleDFUStatus {
    pub controller: SimpleDFUController,
    pub progress: Vec<(DFUID, AtomicUsize)>,
}

impl SimpleDFUStatus {
    pub fn new(controller: SimpleDFUController) -> Self {
        SimpleDFUStatus {
            controller,
            progress: Vec::new(),
        }
    }
}

impl DFUStatus for SimpleDFUStatus {
    fn get_progress(&self, dfu_id: DFUID) -> Result<u8, DFUError> {
        for &(id, ref prog) in &self.progress {
            if id == dfu_id {
                return Ok(prog.load(Ordering::SeqCst) as u8);
            }
        }
        Err(DFUError::NotFound)
    }
    
    fn abort(&mut self, dfu_id: DFUID) -> Result<(), DFUError> {
        for module_option in &mut self.controller.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == dfu_id {
                    module.state.store(DFUState::Idle as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DFUError::NotFound)
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
