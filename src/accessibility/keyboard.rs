#![no_std]
#![no_main]

/// OOP-based Accessibility Keyboard for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 836
/// Implements on-screen keyboard and accessibility input

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyType { Character = 0, Modifier = 1, Function = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyboardError { Success = 0, NotFound = 1 }

pub trait VirtualKey {
    fn id(&self) -> KeyID;
    fn label(&self) -> &[u8];
    fn key_type(&self) -> KeyType;
    fn is_pressed(&self) -> bool;
}

#[repr(C)]
pub struct SimpleVirtualKey {
    pub id: KeyID,
    pub label: [u8; 8],
    pub key_type: AtomicUsize,
    pub pressed: AtomicUsize,
}

impl SimpleVirtualKey {
    pub fn new(id: KeyID, label: &[u8], key_type: KeyType) -> Self {
        let mut label_array = [0u8; 8];
        let label_len = label.len().min(7);
        for i in 0..label_len {
            label_array[i] = label[i];
        }
        SimpleVirtualKey {
            id,
            label: label_array,
            key_type: AtomicUsize::new(key_type as usize),
            pressed: AtomicUsize::new(0),
        }
    }
}

impl VirtualKey for SimpleVirtualKey {
    fn id(&self) -> KeyID { self.id }
    fn label(&self) -> &[u8] {
        let len = self.label.iter().position(|&b| b == 0).unwrap_or(8);
        &self.label[..len]
    }
    fn key_type(&self) -> KeyType { unsafe { core::mem::transmute(self.key_type.load(Ordering::SeqCst)) } }
    fn is_pressed(&self) -> bool { self.pressed.load(Ordering::SeqCst) == 1 }
}

pub trait OnScreenKeyboard {
    fn press_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError>;
    fn release_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError>;
    fn get_key(&self, id: KeyID) -> Option<&dyn VirtualKey>;
    def set_layout(&mut self, layout: &[u8]);
}

#[repr(C)]
pub struct SimpleOnScreenKeyboard {
    pub keys: Vec<Option<Box<dyn VirtualKey>>>,
    pub layout: [u8; 32],
    pub next_id: AtomicUsize,
}

impl SimpleOnScreenKeyboard {
    pub fn new() -> Self {
        let mut layout_array = [0u8; 32];
        let layout_len = b"QWERTY".len().min(31);
        for i in 0..layout_len {
            layout_array[i] = b"QWERTY"[i];
        }
        SimpleOnScreenKeyboard {
            keys: Vec::new(),
            layout: layout_array,
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OnScreenKeyboard for SimpleOnScreenKeyboard {
    fn press_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError> {
        for key_option in &mut self.keys {
            if let Some(ref mut key) = *key_option {
                if key.id() == key_id {
                    key.pressed.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(KeyboardError::NotFound)
    }
    
    fn release_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError> {
        for key_option in &mut self.keys {
            if let Some(ref mut key) = *key_option {
                if key.id() == key_id {
                    key.pressed.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(KeyboardError::NotFound)
    }
    
    fn get_key(&self, id: KeyID) -> Option<&dyn VirtualKey> {
        for key_option in &self.keys {
            if let Some(ref key) = *key_option {
                if key.id() == id { return Some(key.as_ref()); }
            }
        }
        None
    }
    
    fn set_layout(&mut self, layout: &[u8]) {
        let layout_len = layout.len().min(31);
        for i in 0..layout_len {
            self.layout[i] = layout[i];
        }
    }
}

pub trait StickyKeys {
    fn enable_sticky(&mut self, key_id: KeyID);
    fn disable_sticky(&mut self, key_id: KeyID);
    fn is_sticky(&self, key_id: KeyID) -> bool;
}

#[repr(C)]
pub struct SimpleStickyKeys {
    pub sticky_keys: Vec<KeyID>,
}

impl SimpleStickyKeys {
    pub fn new() -> Self {
        SimpleStickyKeys {
            sticky_keys: Vec::new(),
        }
    }
}

impl StickyKeys for SimpleStickyKeys {
    fn enable_sticky(&mut self, key_id: KeyID) {
        if !self.sticky_keys.contains(&key_id) {
            self.sticky_keys.push(key_id);
        }
    }
    
    fn disable_sticky(&mut self, key_id: KeyID) {
        for i in 0..self.sticky_keys.len() {
            if self.sticky_keys[i] == key_id {
                self.sticky_keys.remove(i);
                return;
            }
        }
    }
    
    fn is_sticky(&self, key_id: KeyID) -> bool {
        self.sticky_keys.contains(&key_id)
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    fn contains(&self, item: KeyID) -> bool {
        for i in 0..self.len {
            unsafe {
                let stored = core::ptr::read(self.data.add(i));
                if stored == item {
                    return true;
                }
            }
        }
        false
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
