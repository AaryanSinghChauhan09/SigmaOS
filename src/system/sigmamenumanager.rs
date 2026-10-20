// SPDX-License-Identifier: MIT
// SigmaOS - Menu Manager
// Menu management inspired by Omarchy menu structure and Linux menu systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Menu entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEntryType {
    Command,
    Submenu,
    Separator,
}

/// Menu entry
#[derive(Debug, Clone)]
pub struct MenuEntry {
    pub id: String,
    pub name: String,
    pub entry_type: MenuEntryType,
    pub command: Option<String>,
    pub icon: Option<String>,
    pub hidden: bool,
}

/// Menu
#[derive(Debug, Clone)]
pub struct Menu {
    pub id: String,
    pub name: String,
    pub entries: Vec<MenuEntry>,
}

/// Menu manager
pub struct SigmaMenuManager {
    menus: Vec<Menu>,
}

impl SigmaMenuManager {
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
        }
    }

    pub fn add_menu(&mut self, menu: Menu) {
        self.menus.push(menu);
    }

    pub fn get_menus(&self) -> &[Menu] {
        &self.menus
    }

    pub fn get_menu(&self, id: &str) -> Option<&Menu> {
        self.menus.iter().find(|m| m.id == id)
    }

    pub fn add_entry(&mut self, menu_id: String, entry: MenuEntry) -> Result<(), String> {
        if let Some(menu) = self.menus.iter_mut().find(|m| m.id == menu_id) {
            menu.entries.push(entry);
            Ok(())
        } else {
            Err(format!("Menu {} not found", menu_id))
        }
    }

    pub fn get_visible_entries(&self, menu_id: &str) -> Vec<&MenuEntry> {
        self.get_menu(menu_id)
            .map(|m| m.entries.iter().filter(|e| !e.hidden).collect())
            .unwrap_or_default()
    }

    pub fn remove_menu(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.menus.iter().position(|m| m.id == id) {
            self.menus.remove(idx);
            Ok(())
        } else {
            Err(format!("Menu {} not found", id))
        }
    }
}

impl Default for SigmaMenuManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_manager_creation() {
        let manager = SigmaMenuManager::new();
        assert_eq!(manager.get_menus().len(), 0);
    }

    #[test]
    fn test_add_menu() {
        let mut manager = SigmaMenuManager::new();
        let menu = Menu {
            id: "main".to_string(),
            name: "Main Menu".to_string(),
            entries: Vec::new(),
        };
        manager.add_menu(menu);
        assert_eq!(manager.get_menus().len(), 1);
    }

    #[test]
    fn test_add_entry() {
        let mut manager = SigmaMenuManager::new();
        let menu = Menu {
            id: "main".to_string(),
            name: "Main Menu".to_string(),
            entries: Vec::new(),
        };
        manager.add_menu(menu);
        let entry = MenuEntry {
            id: "entry1".to_string(),
            name: "Test Entry".to_string(),
            entry_type: MenuEntryType::Command,
            command: Some("test".to_string()),
            icon: None,
            hidden: false,
        };
        manager.add_entry("main".to_string(), entry).unwrap();
        assert_eq!(manager.get_menu("main").unwrap().entries.len(), 1);
    }

    #[test]
    fn test_visible_entries() {
        let mut manager = SigmaMenuManager::new();
        let menu = Menu {
            id: "main".to_string(),
            name: "Main Menu".to_string(),
            entries: Vec::new(),
        };
        manager.add_menu(menu);
        let entry1 = MenuEntry {
            id: "visible".to_string(),
            name: "Visible".to_string(),
            entry_type: MenuEntryType::Command,
            command: Some("test".to_string()),
            icon: None,
            hidden: false,
        };
        let entry2 = MenuEntry {
            id: "hidden".to_string(),
            name: "Hidden".to_string(),
            entry_type: MenuEntryType::Command,
            command: Some("test".to_string()),
            icon: None,
            hidden: true,
        };
        manager.add_entry("main".to_string(), entry1).unwrap();
        manager.add_entry("main".to_string(), entry2).unwrap();
        assert_eq!(manager.get_visible_entries("main").len(), 1);
    }

    #[test]
    fn test_remove_menu() {
        let mut manager = SigmaMenuManager::new();
        let menu = Menu {
            id: "main".to_string(),
            name: "Main Menu".to_string(),
            entries: Vec::new(),
        };
        manager.add_menu(menu);
        manager.remove_menu("main".to_string()).unwrap();
        assert_eq!(manager.get_menus().len(), 0);
    }
}
