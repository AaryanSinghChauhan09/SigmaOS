// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Overlay Components
// Omarchy Quickshell-inspired overlay panels for fullscreen interactions

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Overlay type
#[derive(Debug, Clone, PartialEq)]
pub enum OverlayType {
    ImagePicker,
    EmojiPicker,
    ClipboardManager,
    Reminders,
    BackgroundSwitcher,
    Custom(String),
}

/// Overlay state
#[derive(Debug, Clone, PartialEq)]
pub enum OverlayState {
    Hidden,
    Summoned,
    Visible,
    Loading,
}

/// Overlay panel
#[derive(Debug, Clone)]
pub struct OverlayPanel {
    pub id: String,
    pub name: String,
    pub overlay_type: OverlayType,
    pub state: OverlayState,
    pub keep_loaded: bool,
    pub config: BTreeMap<String, String>,
}

impl OverlayPanel {
    pub fn new(id: String, name: String, overlay_type: OverlayType) -> Self {
        Self {
            id,
            name,
            overlay_type,
            state: OverlayState::Hidden,
            keep_loaded: false,
            config: BTreeMap::new(),
        }
    }

    /// Summon overlay
    pub fn summon(&mut self, _payload: String) -> Result<(), &'static str> {
        self.state = OverlayState::Summoned;
        Ok(())
    }

    /// Hide overlay
    pub fn hide(&mut self) -> Result<(), &'static str> {
        self.state = OverlayState::Hidden;
        Ok(())
    }

    /// Set keep loaded
    pub fn set_keep_loaded(&mut self, keep_loaded: bool) {
        self.keep_loaded = keep_loaded;
    }

    /// Set config
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }
}

/// Overlay manager
#[derive(Debug, Clone)]
pub struct OverlayManager {
    pub overlays: BTreeMap<String, OverlayPanel>,
    pub active_overlay: Option<String>,
}

impl OverlayManager {
    pub fn new() -> Self {
        Self {
            overlays: BTreeMap::new(),
            active_overlay: None,
        }
    }

    /// Add overlay
    pub fn add_overlay(&mut self, overlay: OverlayPanel) {
        let id = overlay.id.clone();
        self.overlays.insert(id, overlay);
    }

    /// Summon overlay
    pub fn summon(&mut self, id: String, payload: String) -> Result<(), &'static str> {
        if let Some(overlay) = self.overlays.get_mut(&id) {
            overlay.summon(payload)?;
            self.active_overlay = Some(id);
            Ok(())
        } else {
            Err("Overlay not found")
        }
    }

    /// Hide overlay
    pub fn hide(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(overlay) = self.overlays.get_mut(&id) {
            overlay.hide()?;
            if self.active_overlay.as_ref() == Some(&id) {
                self.active_overlay = None;
            }
            Ok(())
        } else {
            Err("Overlay not found")
        }
    }

    /// Hide all overlays
    pub fn hide_all(&mut self) {
        if let Some(ref id) = self.active_overlay {
            let _ = self.hide(id.clone());
        }
    }

    /// Get overlay by ID
    pub fn get_overlay(&self, id: &str) -> Option<&OverlayPanel> {
        self.overlays.get(id)
    }

    /// Get active overlay
    pub fn get_active_overlay(&self) -> Option<&OverlayPanel> {
        if let Some(ref id) = self.active_overlay {
            self.overlays.get(id)
        } else {
            None
        }
    }

    /// Get overlays by type
    pub fn get_by_type(&self, overlay_type: &OverlayType) -> Vec<&OverlayPanel> {
        self.overlays
            .values()
            .filter(|o| &o.overlay_type == overlay_type)
            .collect()
    }

    /// Remove overlay
    pub fn remove_overlay(&mut self, id: String) -> Result<(), &'static str> {
        if self.overlays.remove(&id).is_some() {
            if self.active_overlay.as_ref() == Some(&id) {
                self.active_overlay = None;
            }
            Ok(())
        } else {
            Err("Overlay not found")
        }
    }

    /// List all overlays
    pub fn list_overlays(&self) -> Vec<String> {
        self.overlays.keys().cloned().collect()
    }
}

impl Default for OverlayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_panel() {
        let panel = OverlayPanel::new(
            String::from("emoji-picker"),
            String::from("Emoji Picker"),
            OverlayType::EmojiPicker
        );
        
        assert_eq!(panel.overlay_type, OverlayType::EmojiPicker);
        assert_eq!(panel.state, OverlayState::Hidden);
    }

    #[test]
    fn test_overlay_manager() {
        let mut manager = OverlayManager::new();
        let panel = OverlayPanel::new(
            String::from("image-picker"),
            String::from("Image Picker"),
            OverlayType::ImagePicker
        );
        
        manager.add_overlay(panel);
        assert_eq!(manager.overlays.len(), 1);
    }

    #[test]
    fn test_summon_hide() {
        let mut manager = OverlayManager::new();
        let panel = OverlayPanel::new(
            String::from("clipboard"),
            String::from("Clipboard Manager"),
            OverlayType::ClipboardManager
        );
        
        manager.add_overlay(panel);
        assert!(manager.summon(String::from("clipboard"), String::new()).is_ok());
        assert!(manager.hide(String::from("clipboard")).is_ok());
    }

    #[test]
    fn test_active_overlay() {
        let mut manager = OverlayManager::new();
        let panel = OverlayPanel::new(
            String::from("reminders"),
            String::from("Reminders"),
            OverlayType::Reminders
        );
        
        manager.add_overlay(panel);
        manager.summon(String::from("reminders"), String::new()).unwrap();
        
        assert!(manager.get_active_overlay().is_some());
    }

    #[test]
    fn test_keep_loaded() {
        let mut panel = OverlayPanel::new(
            String::from("bg-switcher"),
            String::from("Background Switcher"),
            OverlayType::BackgroundSwitcher
        );
        
        panel.set_keep_loaded(true);
        assert!(panel.keep_loaded);
    }
}