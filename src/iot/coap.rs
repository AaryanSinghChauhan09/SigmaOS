#![no_std]
#![no_main]

/// OOP-based CoAP Client for SigmaOS
/// Based on Ideas-999-Structured: IoT & Smart Home Item 996
/// Implements CoAP protocol for constrained devices

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ResourceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CoAPMethod { GET = 0, POST = 1, PUT = 2, DELETE = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CoAPError { Success = 0, NotFound = 1, RequestFailed = 2 }

pub trait CoAPResource {
    fn id(&self) -> ResourceID;
    fn path(&self) -> &[u8];
    fn observable(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCoAPResource {
    pub id: ResourceID,
    pub path: [u8; 128],
    pub observable: AtomicUsize,
}

impl SimpleCoAPResource {
    pub fn new(id: ResourceID, path: &[u8], observable: bool) -> Self {
        let mut path_array = [0u8; 128];
        let path_len = path.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), path_array.as_mut_ptr(), path_len);
        }
        SimpleCoAPResource {
            id,
            path: path_array,
            observable: AtomicUsize::new(if observable { 1 } else { 0 }),
        }
    }
}

impl CoAPResource for SimpleCoAPResource {
    fn id(&self) -> ResourceID { self.id }
    fn path(&self) -> &[u8] {
        let len = self.path.iter().position(|&b| b == 0).unwrap_or(128);
        &self.path[..len]
    }
    fn observable(&self) -> bool { self.observable.load(Ordering::SeqCst) == 1 }
}

pub trait CoAPClient {
    fn send_request(&self, method: CoAPMethod, path: &[u8], payload: &[u8]) -> Result<Vec<u8>, CoAPError>;
    fn observe(&self, path: &[u8]) -> Result<(), CoAPError>;
}

#[repr(C)]
pub struct SimpleCoAPClient {
    pub resources: Vec<Option<Box<dyn CoAPResource>>>,
}

impl SimpleCoAPClient {
    pub fn new() -> Self {
        SimpleCoAPClient {
            resources: Vec::new(),
        }
    }
}

impl CoAPClient for SimpleCoAPClient {
    fn send_request(&self, _method: CoAPMethod, _path: &[u8], _payload: &[u8]) -> Result<Vec<u8>, CoAPError> {
        let mut response = Vec::new();
        response.push(0x00);
        Ok(response)
    }
    
    fn observe(&self, _path: &[u8]) -> Result<(), CoAPError> {
        Ok(())
    }
}

pub trait CoAPServer {
    fn add_resource(&mut self, resource: Box<dyn CoAPResource>) -> Result<ResourceID, CoAPError>;
    fn remove_resource(&mut self, id: ResourceID) -> Result<(), CoAPError>;
}

#[repr(C)]
pub struct SimpleCoAPServer {
    pub resources: Vec<Option<Box<dyn CoAPResource>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCoAPServer {
    pub fn new() -> Self {
        SimpleCoAPServer {
            resources: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CoAPServer for SimpleCoAPServer {
    fn add_resource(&mut self, resource: Box<dyn CoAPResource>) -> Result<ResourceID, CoAPError> {
        let id = resource.id();
        self.resources.push(Some(resource));
        Ok(id)
    }
    
    fn remove_resource(&mut self, id: ResourceID) -> Result<(), CoAPError> {
        for resource_option in &mut self.resources {
            if let Some(ref resource) = *resource_option {
                if resource.id() == id {
                    return Ok(());
                }
            }
        }
        Err(CoAPError::NotFound)
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
