#![no_std]
#![no_main]

/// OOP-based Embedded Filesystem for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2066
/// Implements embedded filesystem (LittleFS/FAT)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FSID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FSError { Success = 0, NotFound = 1, NoSpace = 2 }

pub trait Filesystem {
    fn id(&self) -> FSID;
    fn is_mounted(&self) -> bool;
}

#[repr(C)]
pub struct SimpleFilesystem {
    pub id: FSID,
    pub mounted: AtomicUsize,
}

impl SimpleFilesystem {
    pub fn new(id: FSID) -> Self {
        SimpleFilesystem {
            id,
            mounted: AtomicUsize::new(0),
        }
    }
}

impl Filesystem for SimpleFilesystem {
    fn id(&self) -> FSID { self.id }
    fn is_mounted(&self) -> bool { self.mounted.load(Ordering::SeqCst) == 1 }
}

pub trait FSController {
    fn mount(&mut self, fs_id: FSID) -> Result<(), FSError>;
    fn unmount(&mut self, fs_id: FSID) -> Result<(), FSError>;
    def format(&mut self, fs_id: FSID) -> Result<(), FSError>;
}

#[repr(C)]
pub struct SimpleFSController {
    pub filesystems: Vec<Option<Box<dyn Filesystem>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFSController {
    pub fn new() -> Self {
        SimpleFSController {
            filesystems: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FSController for SimpleFSController {
    fn mount(&mut self, fs_id: FSID) -> Result<(), FSError> {
        for fs_option in &mut self.filesystems {
            if let Some(ref mut fs) = *fs_option {
                if fs.id() == fs_id {
                    fs.mounted.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(FSError::NotFound)
    }
    
    fn unmount(&mut self, fs_id: FSID) -> Result<(), FSError> {
        for fs_option in &mut self.filesystems {
            if let Some(ref mut fs) = *fs_option {
                if fs.id() == fs_id {
                    fs.mounted.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(FSError::NotFound)
    }
    
    fn format(&mut self, _fs_id: FSID) -> Result<(), FSError> {
        Ok(())
    }
}

pub trait FileOps {
    def open(&self, fs_id: FSID, path: &[u8]) -> Result<(), FSError>;
    def read(&self, fs_id: FSID, buffer: &mut [u8]) -> Result<usize, FSError>;
}

#[repr(C)]
pub struct SimpleFileOps {
    pub controller: SimpleFSController,
}

impl SimpleFileOps {
    pub fn new(controller: SimpleFSController) -> Self {
        SimpleFileOps { controller }
    }
}

impl FileOps for SimpleFileOps {
    fn open(&self, fs_id: FSID, _path: &[u8]) -> Result<(), FSError> {
        if self.get_fs(fs_id).is_some() {
            Ok(())
        } else {
            Err(FSError::NotFound)
        }
    }
    
    fn read(&self, fs_id: FSID, buffer: &mut [u8]) -> Result<usize, FSError> {
        if self.get_fs(fs_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(FSError::NotFound)
        }
    }
    
    fn get_fs(&self, id: FSID) -> Option<&dyn Filesystem> {
        for fs_option in &self.controller.filesystems {
            if let Some(ref fs) = *fs_option {
                if fs.id() == id { return Some(fs.as_ref()); }
            }
        }
        None
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
