// SPDX-License-Identifier: MIT
// SigmaOS - Clipboard Manager
// Inspired by CopyQ and GNOME Clipboard
// Provides clipboard history and management

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Clipboard item
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: u32,
    pub content: String,
    pub timestamp: i64,
    pub is_pinned: bool,
}

/// Clipboard manager
pub struct ClipboardManager {
    pub history: VecDeque<ClipboardItem>,
    pub current_content: String,
    pub max_history_size: usize,
    pub next_id: u32,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current_content: String::new(),
            max_history_size: 100,
            next_id: 0,
        }
    }

    /// Set clipboard content
    pub fn set_content(&mut self, content: String) {
        self.current_content = content.clone();

        // Add to history
        let item = ClipboardItem {
            id: self.next_id,
            content,
            timestamp: 0, // Simplified
            is_pinned: false,
        };
        self.next_id += 1;

        self.history.push_front(item);

        // Limit history size
        while self.history.len() > self.max_history_size {
            self.history.pop_back();
        }
    }

    /// Get current content
    pub fn get_content(&self) -> &str {
        &self.current_content
    }

    /// Get history
    pub fn get_history(&self) -> Vec<&ClipboardItem> {
        self.history.iter().collect()
    }

    /// Select from history
    pub fn select_from_history(&mut self, id: u32) -> Result<(), &'static str> {
        if let Some(item) = self.history.iter().find(|i| i.id == id) {
            self.current_content = item.content.clone();
            Ok(())
        } else {
            Err("Item not found in history")
        }
    }

    /// Pin item
    pub fn pin_item(&mut self, id: u32) -> Result<(), &'static str> {
        let item = self
            .history
            .iter_mut()
            .find(|i| i.id == id)
            .ok_or("Item not found")?;
        item.is_pinned = true;
        Ok(())
    }

    /// Unpin item
    pub fn unpin_item(&mut self, id: u32) -> Result<(), &'static str> {
        let item = self
            .history
            .iter_mut()
            .find(|i| i.id == id)
            .ok_or("Item not found")?;
        item.is_pinned = false;
        Ok(())
    }

    /// Remove item from history
    pub fn remove_item(&mut self, id: u32) -> Result<(), &'static str> {
        let index = self
            .history
            .iter()
            .position(|i| i.id == id)
            .ok_or("Item not found")?;
        if self.history[index].is_pinned {
            return Err("Cannot remove pinned item");
        }
        self.history.remove(index);
        Ok(())
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        // Only remove unpinned items
        self.history.retain(|i| i.is_pinned);
    }

    /// Clear all
    pub fn clear_all(&mut self) {
        self.history.clear();
        self.current_content = String::new();
    }

    /// Set max history size
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        while self.history.len() > self.max_history_size {
            self.history.pop_back();
        }
    }

    /// Search history
    pub fn search(&self, query: &str) -> Vec<&ClipboardItem> {
        let query_lower = query.to_lowercase();
        self.history
            .iter()
            .filter(|i| i.content.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// History size
    pub fn history_size(&self) -> usize {
        self.history.len()
    }

    /// Pinned count
    pub fn pinned_count(&self) -> usize {
        self.history.iter().filter(|i| i.is_pinned).count()
    }
}

/// Global clipboard manager
static mut CLIPBOARD_MANAGER: Option<ClipboardManager> = None;

/// Initialize clipboard manager
pub fn init_clipboard_manager() {
    unsafe {
        CLIPBOARD_MANAGER = Some(ClipboardManager::new());
    }
}

/// Get clipboard manager
pub fn get_clipboard_manager() -> &'static mut ClipboardManager {
    unsafe {
        CLIPBOARD_MANAGER
            .as_mut()
            .expect("Clipboard manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_creation() {
        let cm = ClipboardManager::new();
        assert_eq!(cm.history_size(), 0);
    }

    #[test]
    fn test_set_content() {
        let mut cm = ClipboardManager::new();
        cm.set_content("Hello, World!".to_string());
        assert_eq!(cm.get_content(), "Hello, World!");
        assert_eq!(cm.history_size(), 1);
    }

    #[test]
    fn test_select_from_history() {
        let mut cm = ClipboardManager::new();
        cm.set_content("First".to_string());
        cm.set_content("Second".to_string());
        cm.select_from_history(0).unwrap();
        assert_eq!(cm.get_content(), "First");
    }

    #[test]
    fn test_pin_item() {
        let mut cm = ClipboardManager::new();
        cm.set_content("Pinned".to_string());
        cm.pin_item(0).unwrap();
        assert!(cm.get_history()[0].is_pinned);
    }

    #[test]
    fn test_remove_item() {
        let mut cm = ClipboardManager::new();
        cm.set_content("Removable".to_string());
        cm.remove_item(0).unwrap();
        assert_eq!(cm.history_size(), 0);
    }

    #[test]
    fn test_search() {
        let mut cm = ClipboardManager::new();
        cm.set_content("Hello World".to_string());
        cm.set_content("Goodbye".to_string());
        let results = cm.search("hello");
        assert_eq!(results.len(), 1);
    }
}
