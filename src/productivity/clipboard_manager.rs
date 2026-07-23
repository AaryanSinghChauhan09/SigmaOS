// SigmaOS Clipboard Manager
// OOP-based clipboard management with history and type support

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Clipboard item type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardItemType {
    Text,
    Image,
    Html,
    Rtf,
    File,
}

/// Clipboard item
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: String,
    pub item_type: ClipboardItemType,
    pub content: Vec<u8>,
    pub metadata: HashMap<String, String>,
    pub timestamp: Instant,
    pub source_app: Option<String>,
}

/// Clipboard history config
#[derive(Debug, Clone)]
pub struct ClipboardHistoryConfig {
    pub max_items: usize,
    pub max_age: Duration,
    pub persist_across_reboot: bool,
}

/// Clipboard filter
#[derive(Debug, Clone)]
pub struct ClipboardFilter {
    pub item_type: Option<ClipboardItemType>,
    pub source_app: Option<String>,
    pub min_age: Option<Duration>,
    pub max_age: Option<Duration>,
    pub search_text: Option<String>,
}

/// OOP trait for clipboard backends
pub trait ClipboardBackend {
    /// Copy to clipboard
    fn copy(&mut self, item: ClipboardItem) -> Result<(), ClipboardError>;
    /// Paste from clipboard
    fn paste(&self) -> Result<ClipboardItem, ClipboardError>;
    /// Clear clipboard
    fn clear(&mut self) -> Result<(), ClipboardError>;
    /// Get backend name
    fn name(&self) -> &str;
}

/// System clipboard backend
pub struct SystemClipboardBackend {
    current_item: Option<ClipboardItem>,
}

impl SystemClipboardBackend {
    pub fn new() -> Self {
        Self { current_item: None }
    }
}

impl ClipboardBackend for SystemClipboardBackend {
    fn copy(&mut self, item: ClipboardItem) -> Result<(), ClipboardError> {
        self.current_item = Some(item);
        Ok(())
    }

    fn paste(&self) -> Result<ClipboardItem, ClipboardError> {
        self.current_item
            .clone()
            .ok_or_else(|| ClipboardError::EmptyClipboard)
    }

    fn clear(&mut self) -> Result<(), ClipboardError> {
        self.current_item = None;
        Ok(())
    }

    fn name(&self) -> &str {
        "SystemClipboard"
    }
}

/// OOP-based Clipboard Manager
pub struct ClipboardManager {
    backend: Box<dyn ClipboardBackend>,
    history: Vec<ClipboardItem>,
    history_config: ClipboardHistoryConfig,
    auto_clear_enabled: bool,
    auto_clear_delay: Duration,
    last_copy_time: Option<Instant>,
}

impl ClipboardManager {
    pub fn new(backend: Box<dyn ClipboardBackend>, history_config: ClipboardHistoryConfig) -> Self {
        Self {
            backend,
            history: Vec::new(),
            history_config,
            auto_clear_enabled: false,
            auto_clear_delay: Duration::from_secs(60),
            last_copy_time: None,
        }
    }

    /// Enable auto-clear
    pub fn with_auto_clear(mut self, enabled: bool, delay: Duration) -> Self {
        self.auto_clear_enabled = enabled;
        self.auto_clear_delay = delay;
        self
    }

    /// Copy to clipboard
    pub fn copy(&mut self, item: ClipboardItem) -> Result<(), ClipboardError> {
        self.backend.copy(item.clone())?;

        // Add to history
        self.history.insert(0, item);
        self.last_copy_time = Some(Instant::now());

        // Trim history
        self.trim_history();

        Ok(())
    }

    /// Copy text
    pub fn copy_text(
        &mut self,
        text: String,
        source_app: Option<String>,
    ) -> Result<(), ClipboardError> {
        let item = ClipboardItem {
            id: format!(
                "item_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ),
            item_type: ClipboardItemType::Text,
            content: text.into_bytes(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("text_length".to_string(), text.len().to_string());
                meta
            },
            timestamp: Instant::now(),
            source_app,
        };
        self.copy(item)
    }

    /// Paste from clipboard
    pub fn paste(&mut self) -> Result<ClipboardItem, ClipboardError> {
        self.backend.paste()
    }

    /// Paste text
    pub fn paste_text(&mut self) -> Result<String, ClipboardError> {
        let item = self.paste()?;
        if item.item_type != ClipboardItemType::Text {
            return Err(ClipboardError::TypeMismatch);
        }
        String::from_utf8(item.content).map_err(|_| ClipboardError::InvalidContent)
    }

    /// Clear clipboard
    pub fn clear(&mut self) -> Result<(), ClipboardError> {
        self.backend.clear()
    }

    /// Get history
    pub fn history(&self) -> &[ClipboardItem] {
        &self.history
    }

    /// Get history item by index
    pub fn get_history_item(&self, index: usize) -> Option<&ClipboardItem> {
        self.history.get(index)
    }

    /// Restore from history
    pub fn restore_from_history(&mut self, index: usize) -> Result<(), ClipboardError> {
        let item = self
            .history
            .get(index)
            .ok_or_else(|| ClipboardError::HistoryIndexOutOfRange(index))?
            .clone();
        self.copy(item)
    }

    /// Filter history
    pub fn filter_history(&self, filter: &ClipboardFilter) -> Vec<&ClipboardItem> {
        self.history
            .iter()
            .filter(|item| {
                if let Some(item_type) = filter.item_type {
                    if item.item_type != item_type {
                        return false;
                    }
                }
                if let Some(ref source) = filter.source_app {
                    if item.source_app.as_ref() != Some(source) {
                        return false;
                    }
                }
                if let Some(min_age) = filter.min_age {
                    if item.timestamp.elapsed() < min_age {
                        return false;
                    }
                }
                if let Some(max_age) = filter.max_age {
                    if item.timestamp.elapsed() > max_age {
                        return false;
                    }
                }
                if let Some(ref search) = filter.search_text {
                    let content_str = String::from_utf8_lossy(&item.content);
                    if !content_str.to_lowercase().contains(&search.to_lowercase()) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Search history
    pub fn search_history(&self, query: &str) -> Vec<&ClipboardItem> {
        let filter = ClipboardFilter {
            item_type: None,
            source_app: None,
            min_age: None,
            max_age: None,
            search_text: Some(query.to_string()),
        };
        self.filter_history(&filter)
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Trim history based on config
    fn trim_history(&mut self) {
        // Trim by max items
        if self.history.len() > self.history_config.max_items {
            self.history.truncate(self.history_config.max_items);
        }

        // Trim by max age
        self.history
            .retain(|item| item.timestamp.elapsed() < self.history_config.max_age);
    }

    /// Auto-clear if needed
    pub fn auto_clear_if_needed(&mut self) -> Result<(), ClipboardError> {
        if !self.auto_clear_enabled {
            return Ok(());
        }

        if let Some(last_copy) = self.last_copy_time {
            if last_copy.elapsed() >= self.auto_clear_delay {
                self.clear()?;
            }
        }

        Ok(())
    }

    /// Get history config
    pub fn history_config(&self) -> &ClipboardHistoryConfig {
        &self.history_config
    }

    /// Update history config
    pub fn update_history_config(&mut self, config: ClipboardHistoryConfig) {
        self.history_config = config;
        self.trim_history();
    }

    /// Is auto-clear enabled
    pub fn is_auto_clear_enabled(&self) -> bool {
        self.auto_clear_enabled
    }

    /// Enable auto-clear
    pub fn enable_auto_clear(&mut self, enabled: bool) {
        self.auto_clear_enabled = enabled;
    }

    /// Get backend name
    pub fn backend_name(&self) -> &str {
        self.backend.name()
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        let config = ClipboardHistoryConfig {
            max_items: 100,
            max_age: Duration::from_secs(86400), // 24 hours
            persist_across_reboot: false,
        };

        Self::new(Box::new(SystemClipboardBackend::new()), config)
            .with_auto_clear(false, Duration::from_secs(60))
    }
}

/// Clipboard errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    EmptyClipboard,
    TypeMismatch,
    InvalidContent,
    HistoryIndexOutOfRange(usize),
    CopyFailed(String),
    PasteFailed(String),
    ClearFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_item() {
        let item = ClipboardItem {
            id: "test".to_string(),
            item_type: ClipboardItemType::Text,
            content: b"Hello".to_vec(),
            metadata: HashMap::new(),
            timestamp: Instant::now(),
            source_app: None,
        };
        assert_eq!(item.item_type, ClipboardItemType::Text);
    }

    #[test]
    fn test_system_clipboard_backend() {
        let backend = SystemClipboardBackend::new();
        assert_eq!(backend.name(), "SystemClipboard");
    }

    #[test]
    fn test_clipboard_manager() {
        let manager = ClipboardManager::default();
        assert_eq!(manager.backend_name(), "SystemClipboard");
    }

    #[test]
    fn test_copy_text() {
        let mut manager = ClipboardManager::default();
        manager.copy_text("Hello World".to_string(), None).unwrap();
        assert_eq!(manager.history().len(), 1);
    }

    #[test]
    fn test_paste_text() {
        let mut manager = ClipboardManager::default();
        manager.copy_text("Hello World".to_string(), None).unwrap();
        let text = manager.paste_text().unwrap();
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_search_history() {
        let mut manager = ClipboardManager::default();
        manager.copy_text("Hello World".to_string(), None).unwrap();
        manager.copy_text("Test Message".to_string(), None).unwrap();
        let results = manager.search_history("Hello");
        assert_eq!(results.len(), 1);
    }
}
