#![no_std]
#![no_main]

/// OOP-based Barcode Scanner for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1696
/// Implements barcode scanner

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ScannerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BarcodeError { Success = 0, NotFound = 1, NoCode = 2 }

pub trait BarcodeScanner {
    fn id(&self) -> ScannerID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBarcodeScanner {
    pub id: ScannerID,
    pub enabled: AtomicUsize,
}

impl SimpleBarcodeScanner {
    pub fn new(id: ScannerID) -> Self {
        SimpleBarcodeScanner {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl BarcodeScanner for SimpleBarcodeScanner {
    fn id(&self) -> ScannerID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait BarcodeController {
    fn enable(&mut self, scanner_id: ScannerID) -> Result<(), BarcodeError>;
    fn disable(&mut self, scanner_id: ScannerID) -> Result<(), BarcodeError>;
    def read_code(&self, scanner_id: ScannerID) -> Result<[u8; 32], BarcodeError>;
}

#[repr(C)]
pub struct SimpleBarcodeController {
    pub scanners: Vec<Option<Box<dyn BarcodeScanner>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBarcodeController {
    pub fn new() -> Self {
        SimpleBarcodeController {
            scanners: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BarcodeController for SimpleBarcodeController {
    fn enable(&mut self, scanner_id: ScannerID) -> Result<(), BarcodeError> {
        for scanner_option in &mut self.scanners {
            if let Some(ref mut scanner) = *scanner_option {
                if scanner.id() == scanner_id {
                    scanner.enabled.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BarcodeError::NotFound)
    }
    
    fn disable(&mut self, scanner_id: ScannerID) -> Result<(), BarcodeError> {
        for scanner_option in &mut self.scanners {
            if let Some(ref mut scanner) = *scanner_option {
                if scanner.id() == scanner_id {
                    scanner.enabled.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BarcodeError::NotFound)
    }
    
    fn read_code(&self, scanner_id: ScannerID) -> Result<[u8; 32], BarcodeError> {
        if self.get_scanner(scanner_id).is_some() {
            Ok([0u8; 32])
        } else {
            Err(BarcodeError::NotFound)
        }
    }
    
    fn get_scanner(&self, id: ScannerID) -> Option<&dyn BarcodeScanner> {
        for scanner_option in &self.scanners {
            if let Some(ref scanner) = *scanner_option {
                if scanner.id() == id { return Some(scanner.as_ref()); }
            }
        }
        None
    }
}

pub trait QRScanner {
    def read_qr(&self, scanner_id: ScannerID) -> Result<[u8; 128], BarcodeError>;
    def set_mode(&mut self, scanner_id: ScannerID, mode: u8) -> Result<(), BarcodeError>;
}

#[repr(C)]
pub struct SimpleQRScanner {
    pub controller: SimpleBarcodeController,
    pub modes: Vec<(ScannerID, AtomicUsize)>,
}

impl SimpleQRScanner {
    pub fn new(controller: SimpleBarcodeController) -> Self {
        SimpleQRScanner {
            controller,
            modes: Vec::new(),
        }
    }
}

impl QRScanner for SimpleQRScanner {
    fn read_qr(&self, scanner_id: ScannerID) -> Result<[u8; 128], BarcodeError> {
        if self.controller.get_scanner(scanner_id).is_some() {
            Ok([0u8; 128])
        } else {
            Err(BarcodeError::NotFound)
        }
    }
    
    fn set_mode(&mut self, scanner_id: ScannerID, mode: u8) -> Result<(), BarcodeError> {
        self.modes.push((scanner_id, AtomicUsize::new(mode as usize)));
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
