//! SigmaOS Clipboard Manager
//!
//! Inspired by: Omarchy's clipboard manager (image previews, sensitive-content exclusion)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, MEMORY_SAFETY, ZERO_COPY, MEMORY_DISCIPLINE, PRIVACY.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Clipboard item
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: u64,
    pub content: ClipboardContent,
    pub timestamp: u64,
    pub source_app: Option<String>,
    pub is_sensitive: bool,
    pub size_bytes: usize,
}

/// Clipboard content types
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image { width: u32, height: u32, format: String, data_len: usize },
    File { paths: Vec<String> },
    Html(String),
    RichText(String),
}

/// Clipboard manager
pub struct ClipboardManager {
    pub history: VecDeque<ClipboardItem>,
    pub max_history: usize,
    pub current: Option<ClipboardItem>,
    pub sensitive_exclusion: bool,
    pub image_previews: bool,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_history: 50,
            current: None,
            sensitive_exclusion: true,
            image_previews: true,
        }
    }

    /// Copy content to clipboard
    pub fn copy(
        &mut self,
        content: ClipboardContent,
        source_app: Option<String>,
        is_sensitive: bool,
    ) -> Result<u64, String> {
        if self.sensitive_exclusion && is_sensitive {
            return Err("Sensitive content excluded from clipboard history".to_string());
        }

        let size_bytes = self.estimate_size(&content);
        let id = self.current.as_ref().map_or(1, |c| c.id + 1);
        let timestamp = now_secs();

        let item = ClipboardItem {
            id,
            content,
            timestamp,
            source_app,
            is_sensitive,
            size_bytes,
        };

        self.current = Some(item.clone());
        self.history.push_back(item);

        while self.history.len() > self.max_history {
            self.history.pop_front();
        }

        Ok(id)
    }

    /// Paste from clipboard
    pub fn paste(&self) -> Option<&ClipboardItem> {
        self.current.as_ref()
    }

    /// Paste from history by ID
    pub fn paste_from_history(&self, id: u64) -> Option<&ClipboardItem> {
        self.history.iter().find(|item| item.id == id)
    }

    /// Clear clipboard history
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.current = None;
    }

    /// Remove item from history
    pub fn remove_from_history(&mut self, id: u64) -> Result<(), String> {
        if let Some(pos) = self.history.iter().position(|i| i.id == id) {
            self.history.remove(pos);
            Ok(())
        } else {
            Err(format!("Clipboard item {} not found", id))
        }
    }

    /// List history
    pub fn list_history(&self) -> Vec<&ClipboardItem> {
        self.history.iter().rev().collect()
    }

    /// Toggle sensitive content exclusion
    pub fn toggle_sensitive_exclusion(&mut self) -> bool {
        self.sensitive_exclusion = !self.sensitive_exclusion;
        self.sensitive_exclusion
    }

    /// Get total history size
    pub fn total_size(&self) -> usize {
        self.history.iter().map(|i| i.size_bytes).sum()
    }

    fn estimate_size(&self, content: &ClipboardContent) -> usize {
        match content {
            ClipboardContent::Text(t) => t.len(),
            ClipboardContent::Image { data_len, .. } => *data_len,
            ClipboardContent::File { paths } => paths.iter().map(|p| p.len()).sum(),
            ClipboardContent::Html(h) => h.len(),
            ClipboardContent::RichText(r) => r.len(),
        }
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_text() {
        let mut mgr = ClipboardManager::new();
        let id = mgr.copy(
            ClipboardContent::Text("hello".to_string()),
            Some("terminal".to_string()),
            false,
        )
        .unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.list_history().len(), 1);
    }

    #[test]
    fn test_sensitive_exclusion() {
        let mut mgr = ClipboardManager::new();
        let result = mgr.copy(
            ClipboardContent::Text("secret".to_string()),
            None,
            true,
        );
        assert!(result.is_err());
        assert!(mgr.list_history().is_empty());
    }

    #[test]
    fn test_toggle_sensitive_exclusion() {
        let mut mgr = ClipboardManager::new();
        assert!(mgr.sensitive_exclusion);
        assert!(!mgr.toggle_sensitive_exclusion());
        assert!(!mgr.sensitive_exclusion);
    }

    #[test]
    fn test_clear_history() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(ClipboardContent::Text("a".to_string()), None, false).unwrap();
        mgr.copy(ClipboardContent::Text("b".to_string()), None, false).unwrap();
        assert_eq!(mgr.list_history().len(), 2);
        mgr.clear_history();
        assert_eq!(mgr.list_history().len(), 0);
    }

    #[test]
    fn test_image_preview_metadata() {
        let mut mgr = ClipboardManager::new();
        mgr.copy(
            ClipboardContent::Image {
                width: 1920,
                height: 1080,
                format: "png".to_string(),
                data_len: 500000,
            },
            None,
            false,
        ).unwrap();
        let item = mgr.list_history()[0];
        match &item.content {
            ClipboardContent::Image { width, height, .. } => {
                assert_eq!(*width, 1920);
                assert_eq!(*height, 1080);
            }
            _ => panic!("Expected image content"),
        }
    }
}