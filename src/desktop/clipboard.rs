#![no_std]
#![no_main]

/// OOP-based Clipboard for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 806
/// Implements clipboard management and history

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ClipboardID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClipboardFormat { Text = 0, Image = 1, HTML = 2, Files = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClipboardError { Success = 0, Empty = 1, InvalidFormat = 2 }

pub trait ClipboardItem {
    fn id(&self) -> ClipboardID;
    fn format(&self) -> ClipboardFormat;
    fn data(&self) -> &[u8];
    fn timestamp(&self) -> u64;
}

#[repr(C)]
pub struct SimpleClipboardItem {
    pub id: ClipboardID,
    pub format: AtomicUsize,
    pub data: Vec<u8>,
    pub timestamp: AtomicUsize,
}

impl SimpleClipboardItem {
    pub fn new(id: ClipboardID, format: ClipboardFormat, data: &[u8]) -> Self {
        let mut data_vec = Vec::new();
        for &byte in data {
            data_vec.push(byte);
        }
        SimpleClipboardItem {
            id,
            format: AtomicUsize::new(format as usize),
            data: data_vec,
            timestamp: AtomicUsize::new(1000000),
        }
    }
}

impl ClipboardItem for SimpleClipboardItem {
    fn id(&self) -> ClipboardID { self.id }
    fn format(&self) -> ClipboardFormat { unsafe { core::mem::transmute(self.format.load(Ordering::SeqCst)) } }
    fn data(&self) -> &[u8] { &self.data }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
}

pub trait Clipboard {
    fn set_content(&mut self, format: ClipboardFormat, data: &[u8]) -> Result<ClipboardID, ClipboardError>;
    fn get_content(&self) -> Option<&dyn ClipboardItem>;
    def clear(&mut self);
}

#[repr(C)]
pub struct SimpleClipboard {
    pub current: Option<Box<dyn ClipboardItem>>,
    pub history: Vec<Option<Box<dyn ClipboardItem>>>,
    pub next_id: AtomicUsize,
}

impl SimpleClipboard {
    pub fn new() -> Self {
        SimpleClipboard {
            current: None,
            history: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Clipboard for SimpleClipboard {
    fn set_content(&mut self, format: ClipboardFormat, data: &[u8]) -> Result<ClipboardID, ClipboardError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let item = SimpleClipboardItem::new(id, format, data);
        
        if let Some(old) = self.current.take() {
            self.history.push(Some(old));
            if self.history.len() > 50 {
                self.history.remove(0);
            }
        }
        
        self.current = Some(Box::new(item));
        Ok(id)
    }
    
    fn get_content(&self) -> Option<&dyn ClipboardItem> {
        self.current.as_ref().map(|item| item.as_ref())
    }
    
    fn clear(&mut self) {
        self.current = None;
    }
}

pub trait ClipboardHistory {
    fn get_history(&self) -> Vec<&dyn ClipboardItem>;
    def restore_from_history(&mut self, index: usize) -> Result<(), ClipboardError>;
    def clear_history(&mut self);
}

#[repr(C)]
pub struct SimpleClipboardHistory {
    pub clipboard: SimpleClipboard,
}

impl SimpleClipboardHistory {
    pub fn new(clipboard: SimpleClipboard) -> Self {
        SimpleClipboardHistory { clipboard }
    }
}

impl ClipboardHistory for SimpleClipboardHistory {
    fn get_history(&self) -> Vec<&dyn ClipboardItem> {
        let mut items = Vec::new();
        for item_option in &self.clipboard.history {
            if let Some(ref item) = *item_option {
                items.push(item.as_ref());
            }
        }
        items
    }
    
    fn restore_from_history(&mut self, index: usize) -> Result<(), ClipboardError> {
        if index < self.clipboard.history.len() {
            if let Some(item) = self.clipboard.history.remove(index) {
                if let Some(old) = self.clipboard.current.take() {
                    self.clipboard.history.insert(0, Some(old));
                }
                self.clipboard.current = Some(item);
                return Ok(());
            }
        }
        Err(ClipboardError::Empty)
    }
    
    fn clear_history(&mut self) {
        self.clipboard.history.clear();
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
    fn insert(&mut self, index: usize, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            for i in (index..self.len).rev() {
                core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(i + 1), 1);
            }
            core::ptr::write(self.data.add(index), item);
            self.len += 1;
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
