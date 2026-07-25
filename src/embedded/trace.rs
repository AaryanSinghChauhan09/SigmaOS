#![no_std]
#![no_main]

/// OOP-based Trace for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2156
/// Implements trace interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TraceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TraceError { Success = 0, NotFound = 1 }

pub trait TraceInterface {
    fn id(&self) -> TraceID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTraceInterface {
    pub id: TraceID,
    pub enabled: AtomicUsize,
}

impl SimpleTraceInterface {
    pub fn new(id: TraceID) -> Self {
        SimpleTraceInterface {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl TraceInterface for SimpleTraceInterface {
    fn id(&self) -> TraceID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait TraceController {
    fn enable(&mut self, trace_id: TraceID) -> Result<(), TraceError>;
    fn disable(&mut self, trace_id: TraceID) -> Result<(), TraceError>;
    def write(&self, trace_id: TraceID, data: &[u8]) -> Result<(), TraceError>;
}

#[repr(C)]
pub struct SimpleTraceController {
    pub interfaces: Vec<Option<Box<dyn TraceInterface>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTraceController {
    pub fn new() -> Self {
        SimpleTraceController {
            interfaces: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TraceController for SimpleTraceController {
    fn enable(&mut self, trace_id: TraceID) -> Result<(), TraceError> {
        for iface_option in &mut self.interfaces {
            if let Some(ref mut iface) = *iface_option {
                if iface.id() == trace_id {
                    iface.enabled.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TraceError::NotFound)
    }
    
    fn disable(&mut self, trace_id: TraceID) -> Result<(), TraceError> {
        for iface_option in &mut self.interfaces {
            if let Some(ref mut iface) = *iface_option {
                if iface.id() == trace_id {
                    iface.enabled.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TraceError::NotFound)
    }
    
    fn write(&self, trace_id: TraceID, _data: &[u8]) -> Result<(), TraceError> {
        if self.get_interface(trace_id).is_some() {
            Ok(())
        } else {
            Err(TraceError::NotFound)
        }
    }
    
    fn get_interface(&self, id: TraceID) -> Option<&dyn TraceInterface> {
        for iface_option in &self.interfaces {
            if let Some(ref iface) = *iface_option {
                if iface.id() == id { return Some(iface.as_ref()); }
            }
        }
        None
    }
}

pub trait TraceITM {
    def set_prescaler(&mut self, trace_id: TraceID, prescaler: u8) -> Result<(), TraceError>;
    def get_timestamp(&self, trace_id: TraceID) -> Result<u32, TraceError>;
}

#[repr(C)]
pub struct SimpleTraceITM {
    pub controller: SimpleTraceController,
    pub prescalers: Vec<(TraceID, AtomicUsize)>,
}

impl SimpleTraceITM {
    pub fn new(controller: SimpleTraceController) -> Self {
        SimpleTraceITM {
            controller,
            prescalers: Vec::new(),
        }
    }
}

impl TraceITM for SimpleTraceITM {
    fn set_prescaler(&mut self, trace_id: TraceID, prescaler: u8) -> Result<(), TraceError> {
        self.prescalers.push((trace_id, AtomicUsize::new(prescaler as usize)));
        Ok(())
    }
    
    fn get_timestamp(&self, trace_id: TraceID) -> Result<u32, TraceError> {
        if self.controller.get_interface(trace_id).is_some() {
            Ok(0)
        } else {
            Err(TraceError::NotFound)
        }
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
