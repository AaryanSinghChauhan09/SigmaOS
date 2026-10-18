// SPDX-License-Identifier: MIT
// SigmaOS - Keyboard Shortcuts
// Inspired by GNOME Settings and system-config-keyboard
// Provides keyboard shortcut management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Key modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyModifier {
    None,
    Ctrl,
    Alt,
    Shift,
    Super,
    CtrlAlt,
    CtrlShift,
    AltShift,
    CtrlSuper,
    AltSuper,
    ShiftSuper,
    CtrlAltShift,
}

/// Shortcut action
#[derive(Debug, Clone)]
pub struct ShortcutAction {
    pub name: String,
    pub description: String,
    pub command: String,
}

/// Keyboard shortcut
#[derive(Debug, Clone)]
pub struct KeyboardShortcut {
    pub id: String,
    pub modifier: KeyModifier,
    pub key: String,
    pub action: ShortcutAction,
    pub is_custom: bool,
    pub is_enabled: bool,
}

/// Shortcut category
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShortcutCategory {
    System,
    Windows,
    Applications,
    Media,
    Custom,
}

/// Keyboard shortcuts manager
pub struct KeyboardShortcutsManager {
    pub shortcuts: BTreeMap<String, KeyboardShortcut>,
    pub categories: BTreeMap<ShortcutCategory, Vec<String>>,
    pub conflicts: Vec<String>,
}

impl KeyboardShortcutsManager {
    pub fn new() -> Self {
        let mut manager = Self {
            shortcuts: BTreeMap::new(),
            categories: BTreeMap::new(),
            conflicts: Vec::new(),
        };
        manager.add_default_shortcuts();
        manager
    }

    /// Add default shortcuts
    fn add_default_shortcuts(&mut self) {
        // System shortcuts
        self.add_shortcut(
            "system-lock".to_string(),
            KeyModifier::CtrlAlt,
            "L".to_string(),
            ShortcutAction {
                name: "Lock Screen".to_string(),
                description: "Lock the screen".to_string(),
                command: "sigma-lock".to_string(),
            },
            ShortcutCategory::System,
        );

        self.add_shortcut(
            "system-logout".to_string(),
            KeyModifier::CtrlAlt,
            "Delete".to_string(),
            ShortcutAction {
                name: "Log Out".to_string(),
                description: "Log out of the session".to_string(),
                command: "sigma-logout".to_string(),
            },
            ShortcutCategory::System,
        );

        // Window shortcuts
        self.add_shortcut(
            "window-close".to_string(),
            KeyModifier::Alt,
            "F4".to_string(),
            ShortcutAction {
                name: "Close Window".to_string(),
                description: "Close the focused window".to_string(),
                command: "window-close".to_string(),
            },
            ShortcutCategory::Windows,
        );

        self.add_shortcut(
            "window-maximize".to_string(),
            KeyModifier::Super,
            "Up".to_string(),
            ShortcutAction {
                name: "Maximize Window".to_string(),
                description: "Maximize the focused window".to_string(),
                command: "window-maximize".to_string(),
            },
            ShortcutCategory::Windows,
        );

        // Media shortcuts
        self.add_shortcut(
            "media-play-pause".to_string(),
            KeyModifier::None,
            "XF86AudioPlay".to_string(),
            ShortcutAction {
                name: "Play/Pause".to_string(),
                description: "Play or pause media".to_string(),
                command: "media-play-pause".to_string(),
            },
            ShortcutCategory::Media,
        );

        self.add_shortcut(
            "media-volume-up".to_string(),
            KeyModifier::None,
            "XF86AudioRaiseVolume".to_string(),
            ShortcutAction {
                name: "Volume Up".to_string(),
                description: "Increase volume".to_string(),
                command: "volume-up".to_string(),
            },
            ShortcutCategory::Media,
        );
    }

    /// Add shortcut
    pub fn add_shortcut(
        &mut self,
        id: String,
        modifier: KeyModifier,
        key: String,
        action: ShortcutAction,
        category: ShortcutCategory,
    ) {
        let shortcut = KeyboardShortcut {
            id: id.clone(),
            modifier,
            key,
            action,
            is_custom: false,
            is_enabled: true,
        };
        self.shortcuts.insert(id.clone(), shortcut);
        self.categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id);
    }

    /// Remove shortcut
    pub fn remove_shortcut(&mut self, id: String) -> Result<(), &'static str> {
        if !self.shortcuts.contains_key(&id) {
            return Err("Shortcut not found");
        }
        self.shortcuts.remove(&id);
        Ok(())
    }

    /// Update shortcut
    pub fn update_shortcut(
        &mut self,
        id: String,
        modifier: KeyModifier,
        key: String,
    ) -> Result<(), &'static str> {
        let shortcut = self.shortcuts.get_mut(&id).ok_or("Shortcut not found")?;
        shortcut.modifier = modifier;
        shortcut.key = key;
        shortcut.is_custom = true;
        Ok(())
    }

    /// Enable shortcut
    pub fn enable_shortcut(&mut self, id: String) -> Result<(), &'static str> {
        let shortcut = self.shortcuts.get_mut(&id).ok_or("Shortcut not found")?;
        shortcut.is_enabled = true;
        Ok(())
    }

    /// Disable shortcut
    pub fn disable_shortcut(&mut self, id: String) -> Result<(), &'static str> {
        let shortcut = self.shortcuts.get_mut(&id).ok_or("Shortcut not found")?;
        shortcut.is_enabled = false;
        Ok(())
    }

    /// Check for conflicts
    pub fn check_conflicts(&mut self) {
        self.conflicts.clear();
        let mut key_map: BTreeMap<(KeyModifier, String), Vec<String>> = BTreeMap::new();

        for (id, shortcut) in &self.shortcuts {
            if shortcut.is_enabled {
                key_map
                    .entry((shortcut.modifier, shortcut.key.clone()))
                    .or_insert_with(Vec::new)
                    .push(id.clone());
            }
        }

        for (_, ids) in key_map {
            if ids.len() > 1 {
                self.conflicts.extend(ids);
            }
        }
    }

    /// Get shortcuts by category
    pub fn get_shortcuts_by_category(&self, category: ShortcutCategory) -> Vec<&KeyboardShortcut> {
        self.categories
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.shortcuts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get all shortcuts
    pub fn get_all_shortcuts(&self) -> Vec<&KeyboardShortcut> {
        self.shortcuts.values().collect()
    }

    /// Get conflicts
    pub fn get_conflicts(&self) -> &[String] {
        &self.conflicts
    }

    /// Shortcut count
    pub fn shortcut_count(&self) -> usize {
        self.shortcuts.len()
    }
}

/// Global keyboard shortcuts manager
static mut KEYBOARD_SHORTCUTS_MANAGER: Option<KeyboardShortcutsManager> = None;

/// Initialize keyboard shortcuts manager
pub fn init_keyboard_shortcuts_manager() {
    unsafe {
        KEYBOARD_SHORTCUTS_MANAGER = Some(KeyboardShortcutsManager::new());
    }
}

/// Get keyboard shortcuts manager
pub fn get_keyboard_shortcuts_manager() -> &'static mut KeyboardShortcutsManager {
    unsafe {
        KEYBOARD_SHORTCUTS_MANAGER
            .as_mut()
            .expect("Keyboard shortcuts manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_shortcuts_manager_creation() {
        let ksm = KeyboardShortcutsManager::new();
        assert!(ksm.shortcut_count() > 0);
    }

    #[test]
    fn test_add_shortcut() {
        let mut ksm = KeyboardShortcutsManager::new();
        ksm.add_shortcut(
            "test".to_string(),
            KeyModifier::Ctrl,
            "T".to_string(),
            ShortcutAction {
                name: "Test".to_string(),
                description: "Test".to_string(),
                command: "test".to_string(),
            },
            ShortcutCategory::Custom,
        );
        assert_eq!(ksm.shortcut_count(), 7);
    }

    #[test]
    fn test_update_shortcut() {
        let mut ksm = KeyboardShortcutsManager::new();
        ksm.update_shortcut(
            "system-lock".to_string(),
            KeyModifier::CtrlShift,
            "L".to_string(),
        )
        .unwrap();
        let shortcut = ksm.shortcuts.get("system-lock").unwrap();
        assert_eq!(shortcut.modifier, KeyModifier::CtrlShift);
    }
}
