#![no_std]
#![no_main]

/// OOP-based Embedded Debug for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1036
/// Implements debug interface and logging

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LogID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel { Debug = 0, Info = 1, Warning = 2, Error = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DebugError { Success = 0, NotFound = 1 }

pub trait DebugLog {
    fn id(&self) -> LogID;
    fn level(&self) -> LogLevel;
    fn message(&self) -> &[u8];
    fn timestamp(&self) -> u64;
}

#[repr(C)]
pub struct SimpleDebugLog {
    pub id: LogID,
    pub level: AtomicUsize,
    pub message: [u8; 256],
    pub timestamp: AtomicUsize,
}

impl SimpleDebugLog {
    pub fn new(id: LogID, level: LogLevel, message: &[u8]) -> Self {
        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        for i in 0..msg_len {
            msg_array[i] = message[i];
        }
        SimpleDebugLog {
            id,
            level: AtomicUsize::new(level as usize),
            message: msg_array,
            timestamp: AtomicUsize::new(1000000),
        }
    }
}

impl DebugLog for SimpleDebugLog {
    fn id(&self) -> LogID { self.id }
    fn level(&self) -> LogLevel { unsafe { core::mem::transmute(self.level.load(Ordering::SeqCst)) } }
    fn message(&self) -> &[u8] {
        let len = self.message.iter().position(|&b| b == 0).unwrap_or(256);
        &self.message[..len]
    }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
}

pub trait DebugInterface {
    fn log(&mut self, level: LogLevel, message: &[u8]) -> Result<LogID, DebugError>;
    fn get_logs(&self, level: LogLevel) -> Vec<&dyn DebugLog>;
    def clear_logs(&mut self);
}

#[repr(C)]
pub struct SimpleDebugInterface {
    pub logs: Vec<Option<Box<dyn DebugLog>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDebugInterface {
    pub fn new() -> Self {
        SimpleDebugInterface {
            logs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DebugInterface for SimpleDebugInterface {
    fn log(&mut self, level: LogLevel, message: &[u8]) -> Result<LogID, DebugError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let log = SimpleDebugLog::new(id, level, message);
        self.logs.push(Some(Box::new(log)));
        Ok(id)
    }
    
    fn get_logs(&self, level: LogLevel) -> Vec<&dyn DebugLog> {
        let mut filtered = Vec::new();
        for log_option in &self.logs {
            if let Some(ref log) = *log_option {
                if log.level() == level {
                    filtered.push(log.as_ref());
                }
            }
        }
        filtered
    }
    
    fn clear_logs(&mut self) {
        self.logs.clear();
    }
}

pub trait SWDInterface {
    fn connect(&mut self) -> Result<(), DebugError>;
    fn disconnect(&mut self) -> Result<(), DebugError>;
    def read_memory(&self, address: u32) -> Result<u32, DebugError>;
    def write_memory(&self, address: u32, value: u32) -> Result<(), DebugError>;
}

#[repr(C)]
pub struct SimpleSWDInterface {
    pub connected: AtomicUsize,
}

impl SimpleSWDInterface {
    pub fn new() -> Self {
        SimpleSWDInterface {
            connected: AtomicUsize::new(0),
        }
    }
}

impl SWDInterface for SimpleSWDInterface {
    fn connect(&mut self) -> Result<(), DebugError> {
        self.connected.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn disconnect(&mut self) -> Result<(), DebugError> {
        self.connected.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn read_memory(&self, _address: u32) -> Result<u32, DebugError> {
        if self.connected.load(Ordering::SeqCst) == 1 {
            Ok(0)
        } else {
            Err(DebugError::NotFound)
        }
    }
    
    fn write_memory(&self, _address: u32, _value: u32) -> Result<(), DebugError> {
        if self.connected.load(Ordering::SeqCst) == 1 {
            Ok(())
        } else {
            Err(DebugError::NotFound)
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
    fn clear(&mut self) {
        self.len = 0;
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
