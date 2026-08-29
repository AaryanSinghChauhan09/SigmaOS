#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based ENC28J60 Ethernet for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4166
/// Implements ENC28J60 Ethernet controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ENC28J60ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ENC28J60Error { Success = 0, NotFound = 1 }

pub trait ENC28J60Device {
    fn id(&self) -> ENC28J60ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleENC28J60Device {
    pub id: ENC28J60ID,
    pub initialized: AtomicUsize,
}

impl SimpleENC28J60Device {
    pub fn new(id: ENC28J60ID) -> Self {
        SimpleENC28J60Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ENC28J60Device for SimpleENC28J60Device {
    fn id(&self) -> ENC28J60ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ENC28J60Controller {
    fn init(&mut self, dev_id: ENC28J60ID) -> Result<(), ENC28J60Error>;
    fn read(&self, dev_id: ENC28J60ID, addr: u16, buffer: &mut [u8]) -> Result<(), ENC28J60Error>;
    def write(&self, dev_id: ENC28J60ID, addr: u16, data: &[u8]) -> Result<(), ENC28J60Error>;
}

#[repr(C)]
pub struct SimpleENC28J60Controller {
    pub devices: Vec<Option<Box<dyn ENC28J60Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleENC28J60Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleENC28J60Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ENC28J60Controller for SimpleENC28J60Controller {
    fn init(&mut self, dev_id: ENC28J60ID) -> Result<(), ENC28J60Error> {
        for dev_option in &mut self.devices {
            if let Some(ref mut dev) = *dev_option {
                if dev.id() == dev_id {
                    dev.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ENC28J60Error::NotFound)
    }
    
    fn read(&self, dev_id: ENC28J60ID, _addr: u16, buffer: &mut [u8]) -> Result<(), ENC28J60Error> {
        if self.get_device(dev_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(ENC28J60Error::NotFound)
        }
    }
    
    fn write(&self, dev_id: ENC28J60ID, _addr: u16, _data: &[u8]) -> Result<(), ENC28J60Error> {
        if self.get_device(dev_id).is_some() {
            Ok(())
        } else {
            Err(ENC28J60Error::NotFound)
        }
    }
    
    fn get_device(&self, id: ENC28J60ID) -> Option<&dyn ENC28J60Device> {
        for dev_option in &self.devices {
            if let Some(ref dev) = *dev_option {
                if dev.id() == id { return Some(dev.as_ref()); }
            }
        }
        None
    }
}

pub trait ENC28J60MAC {
    def set_mac(&mut self, dev_id: ENC28J60ID, mac: [u8; 6]) -> Result<(), ENC28J60Error>;
}

#[repr(C)]
pub struct SimpleENC28J60MAC {
    pub controller: SimpleENC28J60Controller,
    pub macs: Vec<(ENC28J60ID, AtomicUsize, AtomicUsize, AtomicUsize)>,
}

impl SimpleENC28J60MAC {
    pub fn new(controller: SimpleENC28J60Controller) -> Self {
        SimpleENC28J60MAC {
            controller,
            macs: Vec::new(),
        }
    }
}

impl ENC28J60MAC for SimpleENC28J60MAC {
    fn set_mac(&mut self, dev_id: ENC28J60ID, _mac: [u8; 6]) -> Result<(), ENC28J60Error> {
        self.macs.push((dev_id, AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0)));
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
