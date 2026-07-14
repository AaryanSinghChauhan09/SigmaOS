#![no_std]
#![no_main]

/// OOP-based Thread for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2006
/// Implements Thread (OpenThread) module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ThreadID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadError { Success = 0, NotFound = 1 }

pub trait ThreadModule {
    fn id(&self) -> ThreadID;
    fn is_attached(&self) -> bool;
}

#[repr(C)]
pub struct SimpleThreadModule {
    pub id: ThreadID,
    pub attached: AtomicUsize,
}

impl SimpleThreadModule {
    pub fn new(id: ThreadID) -> Self {
        SimpleThreadModule {
            id,
            attached: AtomicUsize::new(0),
        }
    }
}

impl ThreadModule for SimpleThreadModule {
    fn id(&self) -> ThreadID { self.id }
    fn is_attached(&self) -> bool { self.attached.load(Ordering::SeqCst) == 1 }
}

pub trait ThreadController {
    fn start(&mut self, thread_id: ThreadID) -> Result<(), ThreadError>;
    fn stop(&mut self, thread_id: ThreadID) -> Result<(), ThreadError>;
    def get_network_key(&self, thread_id: ThreadID) -> Result<[u8; 16], ThreadError>;
}

#[repr(C)]
pub struct SimpleThreadController {
    pub modules: Vec<Option<Box<dyn ThreadModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleThreadController {
    pub fn new() -> Self {
        SimpleThreadController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ThreadController for SimpleThreadController {
    fn start(&mut self, thread_id: ThreadID) -> Result<(), ThreadError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == thread_id {
                    module.attached.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ThreadError::NotFound)
    }
    
    fn stop(&mut self, thread_id: ThreadID) -> Result<(), ThreadError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == thread_id {
                    module.attached.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ThreadError::NotFound)
    }
    
    fn get_network_key(&self, thread_id: ThreadID) -> Result<[u8; 16], ThreadError> {
        if self.get_module(thread_id).is_some() {
            Ok([0u8; 16])
        } else {
            Err(ThreadError::NotFound)
        }
    }
    
    fn get_module(&self, id: ThreadID) -> Option<&dyn ThreadModule> {
        for module_option in &self.modules {
            if let Some(ref module) = *module_option {
                if module.id() == id { return Some(module.as_ref()); }
            }
        }
        None
    }
}

pub trait BorderRouter {
    def enable_border_routing(&mut self, thread_id: ThreadID) -> Result<(), ThreadError>;
    def get_mesh_local(&self, thread_id: ThreadID) -> Result<[u8; 16], ThreadError>;
}

#[repr(C)]
pub struct SimpleBorderRouter {
    pub controller: SimpleThreadController,
}

impl SimpleBorderRouter {
    pub fn new(controller: SimpleThreadController) -> Self {
        SimpleBorderRouter { controller }
    }
}

impl BorderRouter for SimpleBorderRouter {
    fn enable_border_routing(&mut self, _thread_id: ThreadID) -> Result<(), ThreadError> {
        Ok(())
    }
    
    fn get_mesh_local(&self, thread_id: ThreadID) -> Result<[u8; 16], ThreadError> {
        if self.controller.get_module(thread_id).is_some() {
            Ok([0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        } else {
            Err(ThreadError::NotFound)
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
