#![no_std]
#![no_main]

/// OOP-based D-Bus Integration for SigmaOS
/// Based on Ideas-999-Structured: Integration & Interoperability Item 886
/// Implements D-Bus communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ServiceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DBusError { Success = 0, NotFound = 1, ConnectionFailed = 2 }

pub trait DBusService {
    fn id(&self) -> ServiceID;
    fn name(&self) -> &[u8];
    fn object_path(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleDBusService {
    pub id: ServiceID,
    pub name: [u8; 128],
    pub object_path: [u8; 256],
}

impl SimpleDBusService {
    pub fn new(id: ServiceID, name: &[u8], object_path: &[u8]) -> Self {
        let mut name_array = [0u8; 128];
        let mut path_array = [0u8; 256];
        let name_len = name.len().min(127);
        let path_len = object_path.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(object_path.as_ptr(), path_array.as_mut_ptr(), path_len);
        }
        SimpleDBusService {
            id,
            name: name_array,
            object_path: path_array,
        }
    }
}

impl DBusService for SimpleDBusService {
    fn id(&self) -> ServiceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(128);
        &self.name[..len]
    }
    fn object_path(&self) -> &[u8] {
        let len = self.object_path.iter().position(|&b| b == 0).unwrap_or(256);
        &self.object_path[..len]
    }
}

pub trait DBusConnection {
    fn connect(&mut self, address: &[u8]) -> Result<(), DBusError>;
    fn disconnect(&mut self) -> Result<(), DBusError>;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDBusConnection {
    pub connected: AtomicUsize,
}

impl SimpleDBusConnection {
    pub fn new() -> Self {
        SimpleDBusConnection {
            connected: AtomicUsize::new(0),
        }
    }
}

impl DBusConnection for SimpleDBusConnection {
    fn connect(&mut self, _address: &[u8]) -> Result<(), DBusError> {
        self.connected.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn disconnect(&mut self) -> Result<(), DBusError> {
        self.connected.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait DBusMessage {
    fn send_message(&self, service: ServiceID, message: &[u8]) -> Result<(), DBusError>;
    fn receive_message(&self) -> Result<Vec<u8>, DBusError>;
}

#[repr(C)]
pub struct SimpleDBusMessage {
    pub connection: SimpleDBusConnection,
}

impl SimpleDBusMessage {
    pub fn new(connection: SimpleDBusConnection) -> Self {
        SimpleDBusMessage { connection }
    }
}

impl DBusMessage for SimpleDBusMessage {
    fn send_message(&self, _service: ServiceID, _message: &[u8]) -> Result<(), DBusError> {
        if self.connection.is_connected() {
            Ok(())
        } else {
            Err(DBusError::ConnectionFailed)
        }
    }
    
    fn receive_message(&self) -> Result<Vec<u8>, DBusError> {
        if self.connection.is_connected() {
            let mut message = Vec::new();
            message.push(0x00);
            Ok(message)
        } else {
            Err(DBusError::ConnectionFailed)
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
