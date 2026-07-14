#![no_std]
#![no_main]

/// OOP-based Log Rotation for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 211
/// Implements log file rotation and management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LogFileID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RotationPolicy { Size = 0, Time = 1, Daily = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RotationError { Success = 0, NotFound = 1, RotationFailed = 2 }

pub trait LogFile {
    fn id(&self) -> LogFileID;
    fn path(&self) -> &[u8];
    fn size(&self) -> usize;
    fn created(&self) -> u64;
}

#[repr(C)]
pub struct SimpleLogFile {
    pub id: LogFileID,
    pub path: [u8; 256],
    pub size: AtomicUsize,
    pub created: AtomicUsize,
}

impl SimpleLogFile {
    pub fn new(id: LogFileID, path: &[u8]) -> Self {
        let mut path_array = [0u8; 256];
        let path_len = path.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), path_array.as_mut_ptr(), path_len);
        }
        SimpleLogFile {
            id,
            path: path_array,
            size: AtomicUsize::new(0),
            created: AtomicUsize::new(1000000),
        }
    }
}

impl LogFile for SimpleLogFile {
    fn id(&self) -> LogFileID { self.id }
    fn path(&self) -> &[u8] {
        let len = self.path.iter().position(|&b| b == 0).unwrap_or(256);
        &self.path[..len]
    }
    fn size(&self) -> usize { self.size.load(Ordering::SeqCst) }
    fn created(&self) -> u64 { self.created.load(Ordering::SeqCst) as u64 }
}

pub trait LogRotator {
    fn add_log_file(&mut self, log_file: Box<dyn LogFile>) -> Result<LogFileID, RotationError>;
    fn set_rotation_policy(&mut self, policy: RotationPolicy, threshold: usize);
    fn check_rotation(&mut self) -> Vec<LogFileID>;
    fn rotate(&mut self, id: LogFileID) -> Result<(), RotationError>;
}

#[repr(C)]
pub struct SimpleLogRotator {
    pub log_files: Vec<Option<Box<dyn LogFile>>>,
    pub policy: AtomicUsize,
    pub threshold: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleLogRotator {
    pub fn new() -> Self {
        SimpleLogRotator {
            log_files: Vec::new(),
            policy: AtomicUsize::new(RotationPolicy::Size as usize),
            threshold: AtomicUsize::new(10 * 1024 * 1024),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LogRotator for SimpleLogRotator {
    fn add_log_file(&mut self, log_file: Box<dyn LogFile>) -> Result<LogFileID, RotationError> {
        let id = log_file.id();
        self.log_files.push(Some(log_file));
        Ok(id)
    }

    fn set_rotation_policy(&mut self, policy: RotationPolicy, threshold: usize) {
        self.policy.store(policy as usize, Ordering::SeqCst);
        self.threshold.store(threshold, Ordering::SeqCst);
    }

    fn check_rotation(&mut self) -> Vec<LogFileID> {
        let mut to_rotate = Vec::new();
        let threshold = self.threshold.load(Ordering::SeqCst);

        for log_file_option in &self.log_files {
            if let Some(ref log_file) = *log_file_option {
                if log_file.size() >= threshold {
                    to_rotate.push(log_file.id());
                }
            }
        }

        to_rotate
    }

    fn rotate(&mut self, id: LogFileID) -> Result<(), RotationError> {
        for log_file_option in &mut self.log_files {
            if let Some(ref mut log_file) = *log_file_option {
                if log_file.id() == id {
                    log_file.size.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RotationError::NotFound)
    }
}

pub trait LogCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError>;
}

#[repr(C)]
pub struct SimpleLogCompressor;

impl SimpleLogCompressor {
    pub fn new() -> Self { SimpleLogCompressor }
}

impl LogCompressor for SimpleLogCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        let mut compressed = Vec::new();
        for &byte in data {
            compressed.push(byte);
        }
        Ok(compressed)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        let mut decompressed = Vec::new();
        for &byte in data {
            decompressed.push(byte);
        }
        Ok(decompressed)
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
