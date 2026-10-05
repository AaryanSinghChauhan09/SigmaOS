//! Clipboard Manager
//!
//! Clipboard management inspired by Linux Mint's clipboard and Omarchy's
//! clipboard utilities, supporting text, image, and file history.

use std::collections::VecDeque;

/// Clipboard item type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardItemType {
    Text,
    Image,
    Files,
    HTML,
}

/// Clipboard item
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: String,
    pub item_type: ClipboardItemType,
    pub content: String,
    pub timestamp: u64,
    pub source: String,
}

impl ClipboardItem {
    pub fn new(id: String, item_type: ClipboardItemType, content: String, source: String) -> Self {
        Self {
            id,
            item_type,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source,
        }
    }
}

/// Clipboard manager
#[derive(Debug)]
pub struct ClipboardHistoryManager {
    history: VecDeque<ClipboardItem>,
    current: Option<ClipboardItem>,
    max_history: usize,
    next_id: u64,
}

impl ClipboardHistoryManager {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current: None,
            max_history: 50,
            next_id: 1,
        }
    }

    /// Set max history size
    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        while self.history.len() > max {
            self.history.pop_back();
        }
    }

    /// Copy text to clipboard
    pub fn copy_text(&mut self, text: String, source: String) {
        let id = format!("clip-{}", self.next_id);
        self.next_id += 1;

        let item = ClipboardItem::new(
            id.clone(),
            ClipboardItemType::Text,
            text,
            source,
        );

        self.current = Some(item.clone());
        self.history.push_front(item);

        while self.history.len() > self.max_history {
            self.history.pop_back();
        }
    }

    /// Copy image to clipboard (base64 encoded)
    pub fn copy_image(&mut self, image_data: String, source: String) {
        let id = format!("clip-{}", self.next_id);
        self.next_id += 1;

        let item = ClipboardItem::new(
            id.clone(),
            ClipboardItemType::Image,
            image_data,
            source,
        );

        self.current = Some(item.clone());
        self.history.push_front(item);

        while self.history.len() > self.max_history {
            self.history.pop_back();
        }
    }

    /// Copy files to clipboard (newline-separated paths)
    pub fn copy_files(&mut self, files: String, source: String) {
        let id = format!("clip-{}", self.next_id);
        self.next_id += 1;

        let item = ClipboardItem::new(
            id.clone(),
            ClipboardItemType::Files,
            files,
            source,
        );

        self.current = Some(item.clone());
        self.history.push_front(item);

        while self.history.len() > self.max_history {
            self.history.pop_back();
        }
    }

    /// Copy HTML to clipboard
    pub fn copy_html(&mut self, html: String, source: String) {
        let id = format!("clip-{}", self.next_id);
        self.next_id += 1;

        let item = ClipboardItem::new(
            id.clone(),
            ClipboardItemType::HTML,
            html,
            source,
        );

        self.current = Some(item.clone());
        self.history.push_front(item);

        while self.history.len() > self.max_history {
            self.history.pop_back();
        }
    }

    /// Get current clipboard content
    pub fn get_current(&self) -> Option<&ClipboardItem> {
        self.current.as_ref()
    }

    /// Get current text
    pub fn get_text(&self) -> Option<String> {
        self.current.as_ref()
            .filter(|item| item.item_type == ClipboardItemType::Text)
            .map(|item| item.content.clone())
    }

    /// Get current image
    pub fn get_image(&self) -> Option<String> {
        self.current.as_ref()
            .filter(|item| item.item_type == ClipboardItemType::Image)
            .map(|item| item.content.clone())
    }

    /// Get current files
    pub fn get_files(&self) -> Option<String> {
        self.current.as_ref()
            .filter(|item| item.item_type == ClipboardItemType::Files)
            .map(|item| item.content.clone())
    }

    /// Get history
    pub fn get_history(&self) -> Vec<&ClipboardItem> {
        self.history.iter().collect()
    }

    /// Get history by type
    pub fn get_history_by_type(&self, item_type: ClipboardItemType) -> Vec<&ClipboardItem> {
        self.history.iter()
            .filter(|item| item.item_type == item_type)
            .collect()
    }

    /// Restore from history
    pub fn restore(&mut self, id: &str) -> Result<(), String> {
        let item = self.history.iter()
            .find(|item| item.id == id)
            .ok_or_else(|| format!("Item {} not found in history", id))?;

        self.current = Some(item.clone());
        Ok(())
    }

    /// Clear current clipboard
    pub fn clear(&mut self) {
        self.current = None;
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Clear all
    pub fn clear_all(&mut self) {
        self.current = None;
        self.history.clear();
    }

    /// Get statistics
    pub fn get_statistics(&self) -> ClipboardHistoryStatistics {
        let total_items = self.history.len();
        let text_count = self.history.iter()
            .filter(|item| item.item_type == ClipboardItemType::Text)
            .count();
        let image_count = self.history.iter()
            .filter(|item| item.item_type == ClipboardItemType::Image)
            .count();
        let files_count = self.history.iter()
            .filter(|item| item.item_type == ClipboardItemType::Files)
            .count();
        let html_count = self.history.iter()
            .filter(|item| item.item_type == ClipboardItemType::HTML)
            .count();

        ClipboardHistoryStatistics {
            total_items,
            text_count,
            image_count,
            files_count,
            html_count,
            max_history: self.max_history,
            has_current: self.current.is_some(),
        }
    }
}

impl Default for ClipboardHistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Clipboard statistics
#[derive(Debug, Clone)]
pub struct ClipboardHistoryStatistics {
    pub total_items: usize,
    pub text_count: usize,
    pub image_count: usize,
    pub files_count: usize,
    pub html_count: usize,
    pub max_history: usize,
    pub has_current: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_manager_creation() {
        let manager = ClipboardHistoryManager::new();
        assert_eq!(manager.get_history().len(), 0);
    }

    #[test]
    fn test_copy_text() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_text("Hello, World!".to_string(), "TestApp".to_string());
        assert!(manager.get_current().is_some());
        assert_eq!(manager.get_text(), Some("Hello, World!".to_string()));
    }

    #[test]
    fn test_copy_image() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_image("base64imagedata".to_string(), "TestApp".to_string());
        assert!(manager.get_current().is_some());
        assert_eq!(manager.get_image(), Some("base64imagedata".to_string()));
    }

    #[test]
    fn test_copy_files() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_files("/tmp/file1.txt\n/tmp/file2.txt".to_string(), "TestApp".to_string());
        assert!(manager.get_current().is_some());
    }

    #[test]
    fn test_history() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_text("Text 1".to_string(), "App1".to_string());
        manager.copy_text("Text 2".to_string(), "App2".to_string());
        manager.copy_text("Text 3".to_string(), "App3".to_string());

        let history = manager.get_history();
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_restore() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_text("Text 1".to_string(), "App1".to_string());
        manager.copy_text("Text 2".to_string(), "App2".to_string());

        let first_id = manager.get_history()[1].id.clone();
        manager.restore(&first_id).ok();
        assert_eq!(manager.get_text(), Some("Text 1".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_text("Text".to_string(), "App".to_string());
        manager.clear();
        assert!(manager.get_current().is_none());
    }

    #[test]
    fn test_max_history() {
        let mut manager = ClipboardHistoryManager::new();
        manager.set_max_history(3);

        for i in 0..5 {
            manager.copy_text(format!("Text {}", i), "App".to_string());
        }

        assert_eq!(manager.get_history().len(), 3);
    }

    #[test]
    fn test_statistics() {
        let mut manager = ClipboardHistoryManager::new();
        manager.copy_text("Text".to_string(), "App".to_string());
        manager.copy_image("img".to_string(), "App".to_string());
        manager.copy_files("/tmp/file".to_string(), "App".to_string());

        let stats = manager.get_statistics();
        assert_eq!(stats.total_items, 3);
        assert_eq!(stats.text_count, 1);
        assert_eq!(stats.image_count, 1);
        assert_eq!(stats.files_count, 1);
    }
}
