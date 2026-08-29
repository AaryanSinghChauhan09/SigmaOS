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

/// OOP-based Hotkey Manager for SigmaOS
/// Based on Ideas-999-Structured: Automation & Scripting Item 876
/// Implements hotkey registration and handling

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HotkeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HotkeyError { Success = 0, NotFound = 1, Conflict = 2 }

pub trait Hotkey {
    fn id(&self) -> HotkeyID;
    fn modifiers(&self) -> u8;
    fn key(&self) -> u8;
    fn action(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleHotkey {
    pub id: HotkeyID,
    pub modifiers: AtomicUsize,
    pub key: AtomicUsize,
    pub action: [u8; 64],
}

impl SimpleHotkey {
    pub fn new(id: HotkeyID, modifiers: u8, key: u8, action: &[u8]) -> Self {
        let mut action_array = [0u8; 64];
        let action_len = action.len().min(63);
        for i in 0..action_len {
            action_array[i] = action[i];
        }
        SimpleHotkey {
            id,
            modifiers: AtomicUsize::new(modifiers as usize),
            key: AtomicUsize::new(key as usize),
            action: action_array,
        }
    }
}

impl Hotkey for SimpleHotkey {
    fn id(&self) -> HotkeyID { self.id }
    fn modifiers(&self) -> u8 { self.modifiers.load(Ordering::SeqCst) as u8 }
    fn key(&self) -> u8 { self.key.load(Ordering::SeqCst) as u8 }
    fn action(&self) -> &[u8] {
        let len = self.action.iter().position(|&b| b == 0).unwrap_or(64);
        &self.action[..len]
    }
}

pub trait HotkeyManager {
    fn register_hotkey(&mut self, modifiers: u8, key: u8, action: &[u8]) -> Result<HotkeyID, HotkeyError>;
    fn unregister_hotkey(&mut self, id: HotkeyID) -> Result<(), HotkeyError>;
    fn trigger_hotkey(&self, modifiers: u8, key: u8) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleHotkeyManager {
    pub hotkeys: Vec<Option<Box<dyn Hotkey>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHotkeyManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleHotkeyManager {
            hotkeys: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HotkeyManager for SimpleHotkeyManager {
    fn register_hotkey(&mut self, modifiers: u8, key: u8, action: &[u8]) -> Result<HotkeyID, HotkeyError> {
        for hotkey_option in &self.hotkeys {
            if let Some(ref hotkey) = *hotkey_option {
                if hotkey.modifiers() == modifiers && hotkey.key() == key {
                    return Err(HotkeyError::Conflict);
                }
            }
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let hotkey = SimpleHotkey::new(id, modifiers, key, action);
        self.hotkeys.push(Some(Box::new(hotkey)));
        Ok(id)
    }
    
    fn unregister_hotkey(&mut self, id: HotkeyID) -> Result<(), HotkeyError> {
        for hotkey_option in &mut self.hotkeys {
            if let Some(ref hotkey) = *hotkey_option {
                if hotkey.id() == id {
                    return Ok(());
                }
            }
        }
        Err(HotkeyError::NotFound)
    }
    
    fn trigger_hotkey(&self, modifiers: u8, key: u8) -> Option<&[u8]> {
        for hotkey_option in &self.hotkeys {
            if let Some(ref hotkey) = *hotkey_option {
                if hotkey.modifiers() == modifiers && hotkey.key() == key {
                    return Some(hotkey.action());
                }
            }
        }
        None
    }
}

pub trait HotkeyProfile {
    fn save_profile(&self, name: &[u8]) -> Result<(), HotkeyError>;
    fn load_profile(&mut self, name: &[u8]) -> Result<(), HotkeyError>;
}

#[repr(C)]
pub struct SimpleHotkeyProfile {
    pub profiles: Vec<([u8; 64], Vec<(u8, u8, [u8; 64])>)>,
}

impl SimpleHotkeyProfile {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleHotkeyProfile {
            profiles: Vec::new(),
        }
    }
}

impl HotkeyProfile for SimpleHotkeyProfile {
    fn save_profile(&self, _name: &[u8]) -> Result<(), HotkeyError> {
        Ok(())
    }
    
    fn load_profile(&mut self, name: &[u8]) -> Result<(), HotkeyError> {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.profiles.push((name_array, Vec::new()));
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
