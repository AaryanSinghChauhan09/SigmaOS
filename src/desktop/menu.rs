#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};
use std::boxed::Box;
use std::vec::Vec;

pub type MenuItemID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemType {
    Separator = 0,
    Action = 1,
    Submenu = 2,
    Checkbox = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuError {
    Success = 0,
    NotFound = 1,
}

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
    pub label_len: u8,
    pub item_type: AtomicUsize,
    pub enabled: AtomicUsize,
    pub checked: AtomicUsize,
}

impl SimpleMenuItem {
    pub fn new(id: MenuItemID, label: &[u8], item_type: MenuItemType) -> Self {
        let mut label_array = [0u8; 128];
        let label_len = label.len().min(127);
        label_array[..label_len].copy_from_slice(&label[..label_len]);

        SimpleMenuItem {
            id,
            label: label_array,
            label_len: label_len as u8,
            item_type: AtomicUsize::new(item_type as usize),
            enabled: AtomicUsize::new(1),
            checked: AtomicUsize::new(0),
        }
    }
}

impl MenuItem for SimpleMenuItem {
    fn id(&self) -> MenuItemID {
        self.id
    }

    fn label(&self) -> &[u8] {
        // O(1) constant-time slice range indexing using precomputed label_len,
        // avoiding O(N) zero-byte linear scan (.position(|&b| b == 0)) on every menu label query.
        &self.label[..self.label_len as usize]
    }

    fn item_type(&self) -> MenuItemType {
        match self.item_type.load(Ordering::SeqCst) {
            0 => MenuItemType::Separator,
            1 => MenuItemType::Action,
            2 => MenuItemType::Submenu,
            _ => MenuItemType::Checkbox,
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    fn is_checked(&self) -> bool {
        self.checked.load(Ordering::SeqCst) == 1
    }
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
    #[allow(clippy::new_without_default)]
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
                    *item_option = None;
                    return Ok(());
                }
            }
        }
        Err(MenuError::NotFound)
    }

    fn get_item(&self, id: MenuItemID) -> Option<&dyn MenuItem> {
        for item_option in &self.items {
            if let Some(ref item) = *item_option {
                if item.id() == id {
                    return Some(item.as_ref());
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_menu_item_cached_label_length() {
        let item = SimpleMenuItem::new(1, b"Open File", MenuItemType::Action);
        assert_eq!(item.id(), 1);
        assert_eq!(item.label(), b"Open File");
        assert_eq!(item.item_type(), MenuItemType::Action);
        assert!(item.is_enabled());
        assert!(!item.is_checked());
        assert_eq!(item.label_len, 9);
    }

    #[test]
    fn test_simple_menu_operations() {
        let mut menu = SimpleMenu::new();
        let item1 = SimpleMenuItem::new(101, b"Copy", MenuItemType::Action);
        let item2 = SimpleMenuItem::new(102, b"Paste", MenuItemType::Action);

        assert_eq!(menu.add_item(Box::new(item1)).unwrap(), 101);
        assert_eq!(menu.add_item(Box::new(item2)).unwrap(), 102);

        let retrieved = menu.get_item(101).unwrap();
        assert_eq!(retrieved.label(), b"Copy");

        assert!(menu.remove_item(101).is_ok());
        assert!(menu.get_item(101).is_none());
    }
}
