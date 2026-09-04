#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based LoRa for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1986
/// Implements LoRa module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LoRaID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LoRaError { Success = 0, NotFound = 1 }

pub trait LoRaModule {
    fn id(&self) -> LoRaID;
    fn is_receiving(&self) -> bool;
}

#[repr(C)]
pub struct SimpleLoRaModule {
    pub id: LoRaID,
    pub receiving: AtomicUsize,
}

impl SimpleLoRaModule {
    pub fn new(id: LoRaID) -> Self {
        SimpleLoRaModule {
            id,
            receiving: AtomicUsize::new(0),
        }
    }
}

impl LoRaModule for SimpleLoRaModule {
    fn id(&self) -> LoRaID { self.id }
    fn is_receiving(&self) -> bool { self.receiving.load(Ordering::SeqCst) == 1 }
}

pub trait LoRaController {
    fn send(&self, lora_id: LoRaID, data: &[u8]) -> Result<(), LoRaError>;
    fn receive(&self, lora_id: LoRaID, buffer: &mut [u8]) -> Result<usize, LoRaError>;
    def set_frequency(&mut self, lora_id: LoRaID, frequency: u32) -> Result<(), LoRaError>;
}

#[repr(C)]
pub struct SimpleLoRaController {
    pub modules: Vec<Option<Box<dyn LoRaModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLoRaController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleLoRaController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LoRaController for SimpleLoRaController {
    fn send(&self, _lora_id: LoRaID, _data: &[u8]) -> Result<(), LoRaError> {
        Ok(())
    }
    
    fn receive(&self, lora_id: LoRaID, buffer: &mut [u8]) -> Result<usize, LoRaError> {
        if self.get_module(lora_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(LoRaError::NotFound)
        }
    }
    
    fn set_frequency(&mut self, _lora_id: LoRaID, _frequency: u32) -> Result<(), LoRaError> {
        Ok(())
    }
    
    fn get_module(&self, id: LoRaID) -> Option<&dyn LoRaModule> {
        for module_option in &self.modules {
            if let Some(ref module) = *module_option {
                if module.id() == id { return Some(module.as_ref()); }
            }
        }
        None
    }
}

pub trait LoRaWAN {
    def join(&mut self, lora_id: LoRaID, deveui: &[u8], appeui: &[u8], appkey: &[u8]) -> Result<(), LoRaError>;
    def send_uplink(&self, lora_id: LoRaID, data: &[u8], port: u8) -> Result<(), LoRaError>;
}

#[repr(C)]
pub struct SimpleLoRaWAN {
    pub controller: SimpleLoRaController,
}

impl SimpleLoRaWAN {
    pub fn new(controller: SimpleLoRaController) -> Self {
        SimpleLoRaWAN { controller }
    }
}

impl LoRaWAN for SimpleLoRaWAN {
    fn join(&mut self, _lora_id: LoRaID, _deveui: &[u8], _appeui: &[u8], _appkey: &[u8]) -> Result<(), LoRaError> {
        Ok(())
    }
    
    fn send_uplink(&self, _lora_id: LoRaID, _data: &[u8], _port: u8) -> Result<(), LoRaError> {
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
