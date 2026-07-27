#![no_std]
#![no_main]

/// OOP-based Desktop Menu for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 736
/// Implements application menu and context menu

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MenuItemID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MenuItemType { Separator = 0, Action = 1, Submenu = 2, Checkbox = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MenuError { Success = 0, NotFound = 1 }

pub trait MenuItem {
    fn id(&self) -> MenuItemID;
    fn label(&self) -> &[u8];
    fn item_type(&self) -> MenuItemType;
    fn is_enabled(&self) -> bool;
    fn is_checked(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMenuItem {
    pub id: MenuItemID,
    pub label: [u8; 128],
    pub item_type: AtomicUsize,
    pub enabled: AtomicUsize,
    pub checked: AtomicUsize,
}

impl SimpleMenuItem {
    pub fn new(id: MenuItemID, label: &[u8], item_type: MenuItemType) -> Self {
        let mut label_array = [0u8; 128];
        let label_len = label.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(label.as_ptr(), label_array.as_mut_ptr(), label_len);
        }
        SimpleMenuItem {
            id,
            label: label_array,
            item_type: AtomicUsize::new(item_type as usize),
            enabled: AtomicUsize::new(1),
            checked: AtomicUsize::new(0),
        }
    }
}

impl MenuItem for SimpleMenuItem {
    fn id(&self) -> MenuItemID { self.id }
    fn label(&self) -> &[u8] {
        let len = self.label.iter().position(|&b| b == 0).unwrap_or(128);
        &self.label[..len]
    }
    fn item_type(&self) -> MenuItemType { unsafe { core::mem::transmute(self.item_type.load(Ordering::SeqCst)) } }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
    fn is_checked(&self) -> bool { self.checked.load(Ordering::SeqCst) == 1 }
}

pub trait Menu {
    fn add_item(&mut self, item: Box<dyn MenuItem>) -> Result<MenuItemID, MenuError>;
    fn remove_item(&mut self, id: MenuItemID) -> Result<(), MenuError>;
    fn get_item(&self, id: MenuItemID) -> Option<&dyn MenuItem>;
    fn show_at(&self, x: i32, y: i32);
}

#[repr(C)]
pub struct SimpleMenu {
    pub items: Vec<Option<Box<dyn MenuItem>>>,
    pub visible: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleMenu {
    pub fn new() -> Self {
        SimpleMenu {
            items: Vec::new(),
            visible: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Menu for SimpleMenu {
    fn add_item(&mut self, item: Box<dyn MenuItem>) -> Result<MenuItemID, MenuError> {
        let id = item.id();
        self.items.push(Some(item));
        Ok(id)
    }
    
    fn remove_item(&mut self, id: MenuItemID) -> Result<(), MenuError> {
        for item_option in &mut self.items {
            if let Some(ref item) = *item_option {
                if item.id() == id {
                    return Ok(());
                }
            }
        }
        Err(MenuError::NotFound)
    }
    
    fn get_item(&self, id: MenuItemID) -> Option<&dyn MenuItem> {
        for item_option in &self.items {
            if let Some(ref item) = *item_option {
                if item.id() == id { return Some(item.as_ref()); }
            }
        }
        None
    }
    
    fn show_at(&self, _x: i32, _y: i32) {
        self.visible.store(1, Ordering::SeqCst);
    }
}

pub trait ContextMenu {
    fn show_context(&mut self, x: i32, y: i32, target: &[u8]);
    fn hide(&mut self);
}

#[repr(C)]
pub struct SimpleContextMenu {
    pub menu: SimpleMenu,
}

impl SimpleContextMenu {
    pub fn new(menu: SimpleMenu) -> Self {
        SimpleContextMenu { menu }
    }
}

impl ContextMenu for SimpleContextMenu {
    fn show_context(&mut self, x: i32, y: i32, _target: &[u8]) {
        self.menu.show_at(x, y);
    }
    
    fn hide(&mut self) {
        self.menu.visible.store(0, Ordering::SeqCst);
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
