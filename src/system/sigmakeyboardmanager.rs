//! SigmaKeyboardManager - Keyboard and Input Management System
//!
//! Provides comprehensive keyboard and input management inspired by Linux Mint and Omarchy.
//! Supports keyboard layouts, keybindings, input methods, and keyboard shortcuts.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Keyboard layout
#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    pub id: String,
    pub name: String,
    pub language: String,
    pub variant: String,
    pub short_name: String,
}

impl KeyboardLayout {
    pub fn new(
        id: String,
        name: String,
        language: String,
        variant: String,
        short_name: String,
    ) -> Self {
        Self {
            id,
            name,
            language,
            variant,
            short_name,
        }
    }
}

/// Keybinding
#[derive(Debug, Clone)]
pub struct Keybinding {
    pub id: String,
    pub name: String,
    pub keys: String, // e.g., "Ctrl+Alt+T"
    pub action: String,
    pub command: String,
    pub enabled: bool,
}

impl Keybinding {
    pub fn new(id: String, name: String, keys: String, action: String, command: String) -> Self {
        Self {
            id,
            name,
            keys,
            action,
            command,
            enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Input method
#[derive(Debug, Clone)]
pub struct InputMethod {
    pub id: String,
    pub name: String,
    pub language: String,
    pub enabled: bool,
}

impl InputMethod {
    pub fn new(id: String, name: String, language: String) -> Self {
        Self {
            id,
            name,
            language,
            enabled: false,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Keyboard settings
#[derive(Debug, Clone)]
pub struct KeyboardSettings {
    pub id: String,
    pub repeat_delay: u32, // milliseconds
    pub repeat_rate: u32,  // characters per second
    pub numlock_on_startup: bool,
    pub capslock_indicator: bool,
}

impl KeyboardSettings {
    pub fn new(id: String) -> Self {
        Self {
            id,
            repeat_delay: 500,
            repeat_rate: 30,
            numlock_on_startup: false,
            capslock_indicator: true,
        }
    }

    pub fn set_repeat_delay(&mut self, delay: u32) {
        self.repeat_delay = delay;
    }

    pub fn set_repeat_rate(&mut self, rate: u32) {
        self.repeat_rate = rate;
    }

    pub fn set_numlock_on_startup(&mut self, enabled: bool) {
        self.numlock_on_startup = enabled;
    }

    pub fn set_capslock_indicator(&mut self, enabled: bool) {
        self.capslock_indicator = enabled;
    }
}

/// SigmaKeyboardManager - Keyboard and input management
#[derive(Debug)]
pub struct SigmaKeyboardManager {
    layouts: BTreeMap<String, KeyboardLayout>,
    keybindings: BTreeMap<String, Keybinding>,
    input_methods: BTreeMap<String, InputMethod>,
    settings: KeyboardSettings,
    active_layout: Option<String>,
}

impl SigmaKeyboardManager {
    pub fn new() -> Self {
        Self {
            layouts: BTreeMap::new(),
            keybindings: BTreeMap::new(),
            input_methods: BTreeMap::new(),
            settings: KeyboardSettings::new("default".to_string()),
            active_layout: None,
        }
    }

    pub fn add_layout(&mut self, layout: KeyboardLayout) {
        self.layouts.insert(layout.id.clone(), layout);
    }

    pub fn remove_layout(&mut self, id: &str) -> bool {
        self.layouts.remove(id).is_some()
    }

    pub fn get_layout(&self, id: &str) -> Option<&KeyboardLayout> {
        self.layouts.get(id)
    }

    pub fn add_keybinding(&mut self, keybinding: Keybinding) {
        self.keybindings.insert(keybinding.id.clone(), keybinding);
    }

    pub fn remove_keybinding(&mut self, id: &str) -> bool {
        self.keybindings.remove(id).is_some()
    }

    pub fn get_keybinding(&self, id: &str) -> Option<&Keybinding> {
        self.keybindings.get(id)
    }

    pub fn add_input_method(&mut self, method: InputMethod) {
        self.input_methods.insert(method.id.clone(), method);
    }

    pub fn remove_input_method(&mut self, id: &str) -> bool {
        self.input_methods.remove(id).is_some()
    }

    pub fn get_input_method(&self, id: &str) -> Option<&InputMethod> {
        self.input_methods.get(id)
    }

    pub fn set_active_layout(&mut self, id: &str) -> bool {
        if self.layouts.contains_key(id) {
            self.active_layout = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_active_layout(&self) -> Option<&KeyboardLayout> {
        self.active_layout
            .as_ref()
            .and_then(|id| self.layouts.get(id))
    }

    pub fn get_settings(&self) -> &KeyboardSettings {
        &self.settings
    }

    pub fn get_settings_mut(&mut self) -> &mut KeyboardSettings {
        &mut self.settings
    }

    pub fn list_enabled_keybindings(&self) -> Vec<&Keybinding> {
        self.keybindings.values().filter(|kb| kb.enabled).collect()
    }

    pub fn list_enabled_input_methods(&self) -> Vec<&InputMethod> {
        self.input_methods
            .values()
            .filter(|im| im.enabled)
            .collect()
    }

    pub fn layout_count(&self) -> usize {
        self.layouts.len()
    }

    pub fn keybinding_count(&self) -> usize {
        self.keybindings.len()
    }
}

impl Default for SigmaKeyboardManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaKeyboardManager instance
static mut SIGMAKEYBOARDMANAGER_INSTANCE: Option<SigmaKeyboardManager> = None;

/// Initialize the global SigmaKeyboardManager instance
pub fn init_sigmakeyboardmanager() {
    unsafe {
        SIGMAKEYBOARDMANAGER_INSTANCE = Some(SigmaKeyboardManager::new());
    }
}

/// Get the global SigmaKeyboardManager instance
pub fn get_sigmakeyboardmanager() -> Option<&'static SigmaKeyboardManager> {
    unsafe { SIGMAKEYBOARDMANAGER_INSTANCE.as_ref() }
}

/// Get the global SigmaKeyboardManager instance (mutable)
pub fn get_sigmakeyboardmanager_mut() -> Option<&'static mut SigmaKeyboardManager> {
    unsafe { SIGMAKEYBOARDMANAGER_INSTANCE.as_mut() }
}

/// Convenience function to add a layout
pub fn add_layout(layout: KeyboardLayout) -> bool {
    if let Some(manager) = get_sigmakeyboardmanager_mut() {
        manager.add_layout(layout);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_layout_creation() {
        let layout = KeyboardLayout::new(
            "us".to_string(),
            "English (US)".to_string(),
            "en".to_string(),
            "".to_string(),
            "en".to_string(),
        );
        assert_eq!(layout.id, "us");
        assert_eq!(layout.language, "en");
    }

    #[test]
    fn test_keybinding_creation() {
        let kb = Keybinding::new(
            "kb1".to_string(),
            "Terminal".to_string(),
            "Ctrl+Alt+T".to_string(),
            "Open Terminal".to_string(),
            "gnome-terminal".to_string(),
        );
        assert_eq!(kb.id, "kb1");
        assert!(kb.enabled);
    }

    #[test]
    fn test_keybinding_modification() {
        let mut kb = Keybinding::new(
            "kb1".to_string(),
            "Test".to_string(),
            "Ctrl+S".to_string(),
            "Save".to_string(),
            "save".to_string(),
        );
        kb.set_enabled(false);
        assert!(!kb.enabled);
    }

    #[test]
    fn test_input_method_creation() {
        let im = InputMethod::new("ibus".to_string(), "IBus".to_string(), "en".to_string());
        assert_eq!(im.id, "ibus");
        assert!(!im.enabled);
    }

    #[test]
    fn test_keyboard_settings_creation() {
        let settings = KeyboardSettings::new("default".to_string());
        assert_eq!(settings.repeat_delay, 500);
        assert_eq!(settings.repeat_rate, 30);
    }

    #[test]
    fn test_keyboard_settings_modification() {
        let mut settings = KeyboardSettings::new("default".to_string());
        settings.set_repeat_delay(300);
        settings.set_repeat_rate(40);
        settings.set_numlock_on_startup(true);
        assert_eq!(settings.repeat_delay, 300);
        assert!(settings.numlock_on_startup);
    }

    #[test]
    fn test_sigmakeyboardmanager_creation() {
        let manager = SigmaKeyboardManager::new();
        assert_eq!(manager.layout_count(), 0);
        assert_eq!(manager.keybinding_count(), 0);
    }

    #[test]
    fn test_sigmakeyboardmanager_management() {
        let mut manager = SigmaKeyboardManager::new();
        let layout = KeyboardLayout::new(
            "us".to_string(),
            "US".to_string(),
            "en".to_string(),
            "".to_string(),
            "en".to_string(),
        );
        manager.add_layout(layout);
        assert_eq!(manager.layout_count(), 1);
    }

    #[test]
    fn test_active_layout() {
        let mut manager = SigmaKeyboardManager::new();
        let layout = KeyboardLayout::new(
            "us".to_string(),
            "US".to_string(),
            "en".to_string(),
            "".to_string(),
            "en".to_string(),
        );
        manager.add_layout(layout);
        manager.set_active_layout("us");
        let active = manager.get_active_layout();
        assert_eq!(active.unwrap().id, "us");
    }
}
