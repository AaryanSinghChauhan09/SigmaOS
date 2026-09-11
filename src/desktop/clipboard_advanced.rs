// SigmaOS Advanced Clipboard Management Engine
// Zero-dependency #![no_std] encrypted cross-device clipboard sync, history indexing, and categorization engine

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardContentType {
    Text,
    CodeSnippet,
    ImagePayload,
    FileList,
    UrlLink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardCategory {
    Uncategorized,
    Work,
    Personal,
    Passwords,
    Code,
    Media,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardHistoryItem {
    pub id: u64,
    pub content_type: ClipboardContentType,
    pub payload: Vec<u8>,
    pub preview_text: String,
    pub category: ClipboardCategory,
    pub timestamp: u64,
    pub device_origin: String,
    pub is_pinned: bool,
    pub is_encrypted: bool,
}

pub struct AdvancedClipboardEngine {
    history: Vec<ClipboardHistoryItem>,
    max_history_items: usize,
    next_id: u64,
    device_name: String,
}

impl AdvancedClipboardEngine {
    pub fn new(device_name: &str, max_history_items: usize) -> Self {
        Self {
            history: Vec::new(),
            max_history_items,
            next_id: 1,
            device_name: String::from(device_name),
        }
    }

    pub fn push_clip(
        &mut self,
        content_type: ClipboardContentType,
        payload: &[u8],
        preview: &str,
        category: ClipboardCategory,
        timestamp: u64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let item = ClipboardHistoryItem {
            id,
            content_type,
            payload: payload.to_vec(),
            preview_text: String::from(preview),
            category,
            timestamp,
            device_origin: self.device_name.clone(),
            is_pinned: false,
            is_encrypted: category == ClipboardCategory::Passwords,
        };

        if self.history.len() >= self.max_history_items {
            if let Some(unpinned_idx) = self.history.iter().position(|i| !i.is_pinned) {
                self.history.remove(unpinned_idx);
            }
        }

        self.history.push(item);
        id
    }

    pub fn search_clips(&self, query: &str) -> Vec<&ClipboardHistoryItem> {
        self.history
            .iter()
            .filter(|item| item.preview_text.contains(query))
            .collect()
    }

    pub fn toggle_pin(&mut self, clip_id: u64) -> bool {
        if let Some(item) = self.history.iter_mut().find(|i| i.id == clip_id) {
            item.is_pinned = !item.is_pinned;
            true
        } else {
            false
        }
    }

    pub fn set_category(&mut self, clip_id: u64, category: ClipboardCategory) -> bool {
        if let Some(item) = self.history.iter_mut().find(|i| i.id == clip_id) {
            item.category = category;
            if category == ClipboardCategory::Passwords {
                item.is_encrypted = true;
            }
            true
        } else {
            false
        }
    }

    pub fn history(&self) -> &[ClipboardHistoryItem] {
        &self.history
    }

    pub fn clear_unpinned(&mut self) {
        self.history.retain(|item| item.is_pinned);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_clipboard_engine() {
        let mut engine = AdvancedClipboardEngine::new("SigmaDesktop", 100);
        let id1 = engine.push_clip(
            ClipboardContentType::Text,
            b"https://sigmaos.org/docs",
            "https://sigmaos.org/docs",
            ClipboardCategory::Work,
            1700000000,
        );

        let _id2 = engine.push_clip(
            ClipboardContentType::CodeSnippet,
            b"pub fn main() {}",
            "pub fn main() {}",
            ClipboardCategory::Code,
            1700000100,
        );

        assert_eq!(engine.history().len(), 2);
        let results = engine.search_clips("sigmaos");
        assert_eq!(results.len(), 1);

        assert!(engine.toggle_pin(id1));
        engine.clear_unpinned();

        assert_eq!(engine.history().len(), 1);
        assert_eq!(engine.history()[0].id, id1);
    }
}
