//! SigmaClipboardManager - Clipboard Management System
//!
//! Provides comprehensive clipboard management inspired by Linux clipboard tools.
//! Supports clipboard history, format support, and item management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Clipboard item format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardFormat {
    Text,
    Image,
    HTML,
    RTF,
    Unknown,
}

/// Clipboard item
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: String,
    pub format: ClipboardFormat,
    pub content: String,
    pub size: u64,
    pub timestamp: u64,
}

impl ClipboardItem {
    pub fn new(id: String, format: ClipboardFormat, content: String) -> Self {
        let size = content.len() as u64;
        Self {
            id,
            format,
            content,
            size,
            timestamp: 0,
        }
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// Clipboard manager
pub struct SigmaClipboardManager {
    items: Vec<ClipboardItem>,
    current_item: Option<String>,
    max_history: usize,
    next_item_id: u64,
}

impl SigmaClipboardManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            items: Vec::new(),
            current_item: None,
            max_history,
            next_item_id: 1,
        }
    }

    pub fn add_item(&mut self, format: ClipboardFormat, content: String) -> String {
        let id = format!("clipboard-{}", self.next_item_id);
        self.next_item_id += 1;
        let item = ClipboardItem::new(id.clone(), format, content);

        if self.items.len() >= self.max_history {
            self.items.remove(0);
        }

        self.items.push(item);
        self.current_item = Some(id.clone());
        id
    }

    pub fn get_item(&self, id: &str) -> Option<&ClipboardItem> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn get_current_item(&self) -> Option<&ClipboardItem> {
        self.current_item.as_ref().and_then(|id| self.get_item(id))
    }

    pub fn list_items(&self) -> Vec<&ClipboardItem> {
        self.items.iter().collect()
    }

    pub fn list_items_by_format(&self, format: ClipboardFormat) -> Vec<&ClipboardItem> {
        self.items.iter().filter(|i| i.format == format).collect()
    }

    pub fn set_current_item(&mut self, id: &str) -> bool {
        if self.get_item(id).is_some() {
            self.current_item = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn clear_history(&mut self) {
        self.items.clear();
        self.current_item = None;
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn total_size(&self) -> u64 {
        self.items.iter().map(|i| i.size).sum()
    }
}

impl Default for SigmaClipboardManager {
    fn default() -> Self {
        Self::new(50)
    }
}

static mut SIGMACLIPBOARDMANAGER_INSTANCE: Option<SigmaClipboardManager> = None;

pub fn init_sigmaclipboardmanager(max_history: usize) {
    unsafe {
        SIGMACLIPBOARDMANAGER_INSTANCE = Some(SigmaClipboardManager::new(max_history));
    }
}

pub fn get_sigmaclipboardmanager() -> &'static SigmaClipboardManager {
    unsafe {
        SIGMACLIPBOARDMANAGER_INSTANCE
            .as_ref()
            .expect("SigmaClipboardManager not initialized")
    }
}

pub fn get_sigmaclipboardmanager_mut() -> &'static mut SigmaClipboardManager {
    unsafe {
        SIGMACLIPBOARDMANAGER_INSTANCE
            .as_mut()
            .expect("SigmaClipboardManager not initialized")
    }
}

pub fn add_item(format: ClipboardFormat, content: String) -> String {
    let manager = get_sigmaclipboardmanager_mut();
    manager.add_item(format, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_item_creation() {
        let item = ClipboardItem::new(
            "clipboard-1".to_string(),
            ClipboardFormat::Text,
            "Hello, World!".to_string(),
        );
        assert_eq!(item.id, "clipboard-1");
        assert_eq!(item.format, ClipboardFormat::Text);
        assert_eq!(item.content, "Hello, World!");
    }

    #[test]
    fn test_clipboard_item_with_timestamp() {
        let item = ClipboardItem::new(
            "clipboard-1".to_string(),
            ClipboardFormat::Text,
            "Hello".to_string(),
        )
        .with_timestamp(1234567890);
        assert_eq!(item.timestamp, 1234567890);
    }

    #[test]
    fn test_clipboard_manager() {
        let manager = SigmaClipboardManager::new(10);
        assert_eq!(manager.item_count(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut manager = SigmaClipboardManager::new(10);
        let id = manager.add_item(ClipboardFormat::Text, "Hello, World!".to_string());
        assert_eq!(id, "clipboard-1");
        assert_eq!(manager.item_count(), 1);
    }

    #[test]
    fn test_get_current_item() {
        let mut manager = SigmaClipboardManager::new(10);
        manager.add_item(ClipboardFormat::Text, "Hello".to_string());

        let current = manager.get_current_item();
        assert!(current.is_some());
        assert_eq!(current.unwrap().content, "Hello");
    }

    #[test]
    fn test_list_items_by_format() {
        let mut manager = SigmaClipboardManager::new(10);
        manager.add_item(ClipboardFormat::Text, "Hello".to_string());
        manager.add_item(ClipboardFormat::Image, "image_data".to_string());
        manager.add_item(ClipboardFormat::Text, "World".to_string());

        let text_items = manager.list_items_by_format(ClipboardFormat::Text);
        assert_eq!(text_items.len(), 2);
    }

    #[test]
    fn test_clear_history() {
        let mut manager = SigmaClipboardManager::new(10);
        manager.add_item(ClipboardFormat::Text, "Hello".to_string());
        manager.add_item(ClipboardFormat::Text, "World".to_string());

        manager.clear_history();
        assert_eq!(manager.item_count(), 0);
    }

    #[test]
    fn test_history_limit() {
        let mut manager = SigmaClipboardManager::new(2);
        manager.add_item(ClipboardFormat::Text, "Item 1".to_string());
        manager.add_item(ClipboardFormat::Text, "Item 2".to_string());
        manager.add_item(ClipboardFormat::Text, "Item 3".to_string());

        assert_eq!(manager.item_count(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaclipboardmanager(10);
        let manager = get_sigmaclipboardmanager();
        assert_eq!(manager.item_count(), 0);

        let manager_mut = get_sigmaclipboardmanager_mut();
        manager_mut.add_item(ClipboardFormat::Text, "Hello".to_string());
        assert_eq!(manager.item_count(), 1);
    }

    #[test]
    fn test_add_item_convenience() {
        init_sigmaclipboardmanager(10);
        let id = add_item(ClipboardFormat::Text, "Hello".to_string());
        assert_eq!(id, "clipboard-1");

        let manager = get_sigmaclipboardmanager();
        assert!(manager.get_item(&id).is_some());
    }
}
