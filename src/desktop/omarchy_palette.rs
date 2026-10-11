//! Unified Clipboard History & Keyboard-First Command Palette Engine
//!
//! Inspired by Omarchy's persistent clipboard history (search, image preview, timestamping)
//! and zero-mouse keyboard-first workflow with command palette (`Cmd+K` / `Super+K`).

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Clipboard Item Content Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardContentType {
    PlainText(String),
    RichText(String),
    ImageMetadata {
        width: u32,
        height: u32,
        format: String,
        preview_data_hash: String,
    },
    CodeSnippet {
        language: String,
        code: String,
    },
}

/// Unified Clipboard History Entry
#[derive(Debug, Clone)]
pub struct ClipboardHistoryEntry {
    pub entry_id: u64,
    pub timestamp_sec: u64,
    pub content: ClipboardContentType,
    pub is_pinned: bool,
    pub tags: Vec<String>,
}

impl ClipboardHistoryEntry {
    pub fn new_text(entry_id: u64, timestamp_sec: u64, text: &str) -> Self {
        Self {
            entry_id,
            timestamp_sec,
            content: ClipboardContentType::PlainText(text.to_string()),
            is_pinned: false,
            tags: Vec::new(),
        }
    }

    pub fn new_image_metadata(
        entry_id: u64,
        timestamp_sec: u64,
        width: u32,
        height: u32,
        format: &str,
        hash: &str,
    ) -> Self {
        Self {
            entry_id,
            timestamp_sec,
            content: ClipboardContentType::ImageMetadata {
                width,
                height,
                format: format.to_string(),
                preview_data_hash: hash.to_string(),
            },
            is_pinned: false,
            tags: vec!["image".to_string()],
        }
    }

    pub fn searchable_text(&self) -> String {
        match &self.content {
            ClipboardContentType::PlainText(s) => s.clone(),
            ClipboardContentType::RichText(s) => s.clone(),
            ClipboardContentType::ImageMetadata { format, preview_data_hash, .. } => {
                format!("image {} {}", format, preview_data_hash)
            }
            ClipboardContentType::CodeSnippet { language, code } => {
                format!("{} {}", language, code)
            }
        }
    }
}

/// Persistent Unified Clipboard History Manager
pub struct UnifiedClipboardHistoryManager {
    pub entries: Vec<ClipboardHistoryEntry>,
    pub max_capacity: usize,
    pub next_entry_id: u64,
}

impl UnifiedClipboardHistoryManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_capacity: capacity,
            next_entry_id: 1,
        }
    }

    pub fn push_text(&mut self, text: &str) -> u64 {
        let id = self.next_entry_id;
        self.next_entry_id += 1;

        // Deduplicate if identical to latest
        if let Some(last) = self.entries.last() {
            if last.searchable_text() == text {
                return last.entry_id;
            }
        }

        let entry = ClipboardHistoryEntry::new_text(id, 1600000000 + id, text);
        self.entries.push(entry);

        if self.entries.len() > self.max_capacity {
            // Remove oldest unpinned
            if let Some(pos) = self.entries.iter().position(|e| !e.is_pinned) {
                self.entries.remove(pos);
            }
        }
        id
    }

    pub fn search_history(&self, query: &str) -> Vec<&ClipboardHistoryEntry> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.searchable_text().to_lowercase().contains(&q))
            .collect()
    }
}

impl Default for UnifiedClipboardHistoryManager {
    fn default() -> Self {
        Self::new(500)
    }
}

/// Command Palette Action Item
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteActionType {
    LaunchApp(String),
    ExecuteSystemCommand(String),
    SwitchWorkspace(u32),
    FocusWindow(u32),
    TriggerAiAgentPrompt(String),
    ToggleThemePreset(String),
}

/// Command Palette Match Item
#[derive(Debug, Clone)]
pub struct CommandPaletteItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub hotkey_shortcut: Option<String>,
    pub action: PaletteActionType,
}

/// Zero-Mouse Keyboard-First Command Palette Engine (`Cmd+K` / `Super+K`)
pub struct CmdKCommandPaletteEngine {
    pub items: Vec<CommandPaletteItem>,
    pub is_visible: bool,
    pub active_selection_index: usize,
}

impl CmdKCommandPaletteEngine {
    pub fn new() -> Self {
        let mut palette = Self {
            items: Vec::new(),
            is_visible: false,
            active_selection_index: 0,
        };
        palette.register_default_commands();
        palette
    }

    fn register_default_commands(&mut self) {
        self.items.push(CommandPaletteItem {
            id: "cmd.terminal".to_string(),
            title: "Launch Ghostty GPU Terminal".to_string(),
            category: "Applications".to_string(),
            hotkey_shortcut: Some("Super+Return".to_string()),
            action: PaletteActionType::LaunchApp("ghostty".to_string()),
        });
        self.items.push(CommandPaletteItem {
            id: "cmd.tdl_ai".to_string(),
            title: "Spawn TDL AI Tri-Pane Workstation Layout".to_string(),
            category: "Workstation".to_string(),
            hotkey_shortcut: Some("Super+Alt+K".to_string()),
            action: PaletteActionType::TriggerAiAgentPrompt("tdl ai".to_string()),
        });
        self.items.push(CommandPaletteItem {
            id: "cmd.toggle_theme_dark".to_string(),
            title: "Theme: Switch to Omarchy Dark / TokyoNight".to_string(),
            category: "Appearance".to_string(),
            hotkey_shortcut: Some("Super+T".to_string()),
            action: PaletteActionType::ToggleThemePreset("TokyoNight".to_string()),
        });
    }

    pub fn toggle_palette(&mut self) {
        self.is_visible = !self.is_visible;
        self.active_selection_index = 0;
    }

    pub fn fuzzy_search(&self, query: &str) -> Vec<&CommandPaletteItem> {
        if query.trim().is_empty() {
            return self.items.iter().collect();
        }
        let q = query.to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                item.title.to_lowercase().contains(&q)
                    || item.category.to_lowercase().contains(&q)
                    || item.id.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn execute_selected(&self, matches: &[&CommandPaletteItem]) -> Option<String> {
        if matches.is_empty() || self.active_selection_index >= matches.len() {
            return None;
        }
        let item = matches[self.active_selection_index];
        Some(format!("Executed command '{}' ({:?})", item.title, item.action))
    }
}

impl Default for CmdKCommandPaletteEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_clipboard_history() {
        let mut manager = UnifiedClipboardHistoryManager::new(10);
        manager.push_text("cargo build --release");
        manager.push_text("https://sigmaos.org");

        let results = manager.search_history("cargo");
        assert_eq!(results.len(), 1);
        assert!(results[0].searchable_text().contains("cargo build"));
    }

    #[test]
    fn test_cmdk_command_palette() {
        let mut palette = CmdKCommandPaletteEngine::new();
        palette.toggle_palette();
        assert!(palette.is_visible);

        let matches = palette.fuzzy_search("Ghostty");
        assert_eq!(matches.len(), 1);

        let exec_res = palette.execute_selected(&matches);
        assert!(exec_res.unwrap().contains("Launch Ghostty"));
    }
}
