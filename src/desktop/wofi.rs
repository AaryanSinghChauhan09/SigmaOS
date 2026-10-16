// SPDX-License-Identifier: MIT
// SigmaOS - Wofi Application Launcher
// Inspired by Wofi (https://hg.sr.ht/~scoopta/wofi)
// Provides application launcher for Wayland

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Application entry
#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub description: String,
    pub desktop_file: String,
}

/// Wofi instance
pub struct WofiInstance {
    pub entries: Vec<AppEntry>,
    pub filter: String,
    pub selected_index: AtomicU32,
    pub visible: AtomicBool,
}

impl WofiInstance {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            filter: String::new(),
            selected_index: AtomicU32::new(0),
            visible: AtomicBool::new(false),
        }
    }

    /// Add application entry
    pub fn add_entry(&mut self, entry: AppEntry) {
        self.entries.push(entry);
    }

    /// Load applications from .desktop files
    pub fn load_applications(&mut self) {
        // In real implementation, would scan /usr/share/applications/*.desktop
        // and ~/.local/share/applications/*.desktop

        // Add some common applications
        self.add_entry(AppEntry {
            name: "Firefox".to_string(),
            exec: "firefox".to_string(),
            icon: "firefox".to_string(),
            description: "Web Browser".to_string(),
            desktop_file: "/usr/share/applications/firefox.desktop".to_string(),
        });

        self.add_entry(AppEntry {
            name: "Chromium".to_string(),
            exec: "chromium".to_string(),
            icon: "chromium".to_string(),
            description: "Web Browser".to_string(),
            desktop_file: "/usr/share/applications/chromium.desktop".to_string(),
        });

        self.add_entry(AppEntry {
            name: "Terminal".to_string(),
            exec: "alacritty".to_string(),
            icon: "terminal".to_string(),
            description: "Terminal Emulator".to_string(),
            desktop_file: "/usr/share/applications/Alacritty.desktop".to_string(),
        });

        self.add_entry(AppEntry {
            name: "Steam".to_string(),
            exec: "steam".to_string(),
            icon: "steam".to_string(),
            description: "Steam Client".to_string(),
            desktop_file: "/usr/share/applications/steam.desktop".to_string(),
        });
    }

    /// Show launcher
    pub fn show(&self) {
        self.visible.store(true, Ordering::Relaxed);
    }

    /// Hide launcher
    pub fn hide(&self) {
        self.visible.store(false, Ordering::Relaxed);
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }

    /// Get filtered entries
    pub fn get_filtered_entries(&self) -> Vec<&AppEntry> {
        if self.filter.is_empty() {
            self.entries.iter().collect()
        } else {
            self.entries
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&self.filter.to_lowercase()))
                .collect()
        }
    }

    /// Select next
    pub fn select_next(&self) {
        let filtered = self.get_filtered_entries();
        let current = self.selected_index.load(Ordering::Relaxed) as usize;
        let next = (current + 1) % filtered.len();
        self.selected_index.store(next as u32, Ordering::Relaxed);
    }

    /// Select previous
    pub fn select_previous(&self) {
        let filtered = self.get_filtered_entries();
        let current = self.selected_index.load(Ordering::Relaxed) as usize;
        let prev = if current == 0 {
            filtered.len() - 1
        } else {
            current - 1
        };
        self.selected_index.store(prev as u32, Ordering::Relaxed);
    }

    /// Get selected entry
    pub fn get_selected_entry(&self) -> Option<&AppEntry> {
        let filtered = self.get_filtered_entries();
        let index = self.selected_index.load(Ordering::Relaxed) as usize;
        filtered.get(index).copied()
    }

    /// Execute selected entry
    pub fn execute_selected(&self) -> Result<(), &'static str> {
        let entry = self.get_selected_entry().ok_or("No entry selected")?;

        // In real implementation, would execute:
        // exec {entry.exec}

        Ok(())
    }
}

impl Default for WofiInstance {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Wofi instance
static mut WOFI_INSTANCE: Option<WofiInstance> = None;

/// Initialize Wofi
pub fn init_wofi() {
    unsafe {
        WOFI_INSTANCE = Some(WofiInstance::new());
    }
}

/// Get global Wofi instance
pub fn get_wofi() -> Option<&'static mut WofiInstance> {
    unsafe { WOFI_INSTANCE.as_mut() }
}

/// Show Wofi launcher
pub fn show_wofi() {
    if let Some(wofi) = get_wofi() {
        wofi.load_applications();
        wofi.show();
    }
}

/// Hide Wofi launcher
pub fn hide_wofi() {
    if let Some(wofi) = get_wofi() {
        wofi.hide();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wofi_init() {
        let wofi = WofiInstance::new();
        assert!(!wofi.is_visible());
    }

    #[test]
    fn test_add_entry() {
        let mut wofi = WofiInstance::new();
        let entry = AppEntry {
            name: "Test".to_string(),
            exec: "test".to_string(),
            icon: "".to_string(),
            description: "".to_string(),
            desktop_file: "".to_string(),
        };
        wofi.add_entry(entry);
        assert_eq!(wofi.entries.len(), 1);
    }

    #[test]
    fn test_load_applications() {
        let mut wofi = WofiInstance::new();
        wofi.load_applications();
        assert!(wofi.entries.len() > 0);
    }

    #[test]
    fn test_show_hide() {
        let wofi = WofiInstance::new();
        wofi.show();
        assert!(wofi.is_visible());
        wofi.hide();
        assert!(!wofi.is_visible());
    }

    #[test]
    fn test_filter() {
        let mut wofi = WofiInstance::new();
        wofi.load_applications();
        wofi.set_filter("Fire".to_string());
        let filtered = wofi.get_filtered_entries();
        assert!(filtered.iter().any(|e| e.name.contains("Fire")));
    }

    #[test]
    fn test_select_next_previous() {
        let mut wofi = WofiInstance::new();
        wofi.load_applications();
        wofi.select_next();
        assert_eq!(wofi.selected_index.load(Ordering::Relaxed), 1);
        wofi.select_previous();
        assert_eq!(wofi.selected_index.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_get_selected_entry() {
        let mut wofi = WofiInstance::new();
        wofi.load_applications();
        let entry = wofi.get_selected_entry();
        assert!(entry.is_some());
    }
}
