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
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Matrix Keyboard for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3916
/// Implements matrix keyboard

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyboardMatrixID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyboardMatrixError { Success = 0, NotFound = 1 }

pub trait KeyboardMatrixDevice {
    fn id(&self) -> KeyboardMatrixID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleKeyboardMatrixDevice {
    pub id: KeyboardMatrixID,
    pub initialized: AtomicUsize,
}

impl SimpleKeyboardMatrixDevice {
    pub fn new(id: KeyboardMatrixID) -> Self {
        SimpleKeyboardMatrixDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl KeyboardMatrixDevice for SimpleKeyboardMatrixDevice {
    fn id(&self) -> KeyboardMatrixID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait KeyboardMatrixController {
    fn init(&mut self, kb_id: KeyboardMatrixID) -> Result<(), KeyboardMatrixError>;
    fn scan(&self, kb_id: KeyboardMatrixID, buffer: &mut [u8]) -> Result<(), KeyboardMatrixError>;
    def get_key(&self, kb_id: KeyboardMatrixID, row: u8, col: u8) -> Result<bool, KeyboardMatrixError>;
}

#[repr(C)]
pub struct SimpleKeyboardMatrixController {
    pub keyboards: Vec<Option<Box<dyn KeyboardMatrixDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleKeyboardMatrixController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleKeyboardMatrixController {
            keyboards: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl KeyboardMatrixController for SimpleKeyboardMatrixController {
    fn init(&mut self, kb_id: KeyboardMatrixID) -> Result<(), KeyboardMatrixError> {
        for kb_option in &mut self.keyboards {
            if let Some(ref mut kb) = *kb_option {
                if kb.id() == kb_id {
                    kb.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(KeyboardMatrixError::NotFound)
    }
    
    fn scan(&self, kb_id: KeyboardMatrixID, buffer: &mut [u8]) -> Result<(), KeyboardMatrixError> {
        if self.get_keyboard(kb_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(KeyboardMatrixError::NotFound)
        }
    }
    
    fn get_key(&self, kb_id: KeyboardMatrixID, _row: u8, _col: u8) -> Result<bool, KeyboardMatrixError> {
        if self.get_keyboard(kb_id).is_some() {
            Ok(false)
        } else {
            Err(KeyboardMatrixError::NotFound)
        }
    }
    
    fn get_keyboard(&self, id: KeyboardMatrixID) -> Option<&dyn KeyboardMatrixDevice> {
        for kb_option in &self.keyboards {
            if let Some(ref kb) = *kb_option {
                if kb.id() == id { return Some(kb.as_ref()); }
            }
        }
        None
    }
}

pub trait KeyboardMatrixDebounce {
    def set_debounce(&mut self, kb_id: KeyboardMatrixID, delay_ms: u16) -> Result<(), KeyboardMatrixError>;
}

#[repr(C)]
pub struct SimpleKeyboardMatrixDebounce {
    pub controller: SimpleKeyboardMatrixController,
    pub debounces: Vec<(KeyboardMatrixID, AtomicUsize)>,
}

impl SimpleKeyboardMatrixDebounce {
    pub fn new(controller: SimpleKeyboardMatrixController) -> Self {
        SimpleKeyboardMatrixDebounce {
            controller,
            debounces: Vec::new(),
        }
    }
}

impl KeyboardMatrixDebounce for SimpleKeyboardMatrixDebounce {
    fn set_debounce(&mut self, kb_id: KeyboardMatrixID, delay_ms: u16) -> Result<(), KeyboardMatrixError> {
        self.debounces.push((kb_id, AtomicUsize::new(delay_ms as usize)));
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
