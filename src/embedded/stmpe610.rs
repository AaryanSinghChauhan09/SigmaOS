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

/// OOP-based STMPE610 Touch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2606
/// Implements STMPE610 resistive touch controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type STMPE610ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum STMPE610Error { Success = 0, NotFound = 1 }

pub trait STMPE610Touch {
    fn id(&self) -> STMPE610ID;
    fn is_touched(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSTMPE610Touch {
    pub id: STMPE610ID,
    pub touched: AtomicUsize,
}

impl SimpleSTMPE610Touch {
    pub fn new(id: STMPE610ID) -> Self {
        SimpleSTMPE610Touch {
            id,
            touched: AtomicUsize::new(0),
        }
    }
}

impl STMPE610Touch for SimpleSTMPE610Touch {
    fn id(&self) -> STMPE610ID { self.id }
    fn is_touched(&self) -> bool { self.touched.load(Ordering::SeqCst) == 1 }
}

pub trait STMPE610Controller {
    fn init(&mut self, stmpe_id: STMPE610ID) -> Result<(), STMPE610Error>;
    fn read(&self, stmpe_id: STMPE610ID) -> Result<(u16, u16, u8), STMPE610Error>;
    def get_touch_count(&self, stmpe_id: STMPE610ID) -> Result<u8, STMPE610Error>;
}

#[repr(C)]
pub struct SimpleSTMPE610Controller {
    pub touches: Vec<Option<Box<dyn STMPE610Touch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSTMPE610Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSTMPE610Controller {
            touches: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl STMPE610Controller for SimpleSTMPE610Controller {
    fn init(&mut self, stmpe_id: STMPE610ID) -> Result<(), STMPE610Error> {
        for touch_option in &mut self.touches {
            if let Some(ref mut touch) = *touch_option {
                if touch.id() == stmpe_id {
                    touch.touched.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(STMPE610Error::NotFound)
    }
    
    fn read(&self, stmpe_id: STMPE610ID) -> Result<(u16, u16, u8), STMPE610Error> {
        if self.get_touch(stmpe_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(STMPE610Error::NotFound)
        }
    }
    
    fn get_touch_count(&self, stmpe_id: STMPE610ID) -> Result<u8, STMPE610Error> {
        if self.get_touch(stmpe_id).is_some() {
            Ok(0)
        } else {
            Err(STMPE610Error::NotFound)
        }
    }
    
    fn get_touch(&self, id: STMPE610ID) -> Option<&dyn STMPE610Touch> {
        for touch_option in &self.touches {
            if let Some(ref touch) = *touch_option {
                if touch.id() == id { return Some(touch.as_ref()); }
            }
        }
        None
    }
}

pub trait STMPE610GPIO {
    def set_gpio_dir(&mut self, stmpe_id: STMPE610ID, pin: u8, dir: u8) -> Result<(), STMPE610Error>;
    def read_gpio(&self, stmpe_id: STMPE610ID, pin: u8) -> Result<bool, STMPE610Error>;
}

#[repr(C)]
pub struct SimpleSTMPE610GPIO {
    pub controller: SimpleSTMPE610Controller,
}

impl SimpleSTMPE610GPIO {
    pub fn new(controller: SimpleSTMPE610Controller) -> Self {
        SimpleSTMPE610GPIO { controller }
    }
}

impl STMPE610GPIO for SimpleSTMPE610GPIO {
    fn set_gpio_dir(&mut self, _stmpe_id: STMPE610ID, _pin: u8, _dir: u8) -> Result<(), STMPE610Error> {
        Ok(())
    }
    
    fn read_gpio(&self, stmpe_id: STMPE610ID, _pin: u8) -> Result<bool, STMPE610Error> {
        if self.controller.get_touch(stmpe_id).is_some() {
            Ok(false)
        } else {
            Err(STMPE610Error::NotFound)
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
