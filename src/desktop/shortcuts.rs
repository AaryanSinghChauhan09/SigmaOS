// Keyboard Shortcuts Manager for SigmaOS
// Keyboard shortcuts per Wiki 08-Desktop.md
// Provides global shortcuts and window management shortcuts

use std::string::{String, ToString};
use std::vec::Vec;

/// Key modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyModifier {
    Super,
    Alt,
    Control,
    Shift,
}

impl KeyModifier {
    pub fn as_str(&self) -> &str {
        match self {
            KeyModifier::Super => "Super",
            KeyModifier::Alt => "Alt",
            KeyModifier::Control => "Ctrl",
            KeyModifier::Shift => "Shift",
        }
    }
}

/// Key action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    OpenLauncher,
    OpenTerminal,
    OpenFileManager,
    OpenWebBrowser,
    ShowDesktop,
    LockScreen,
    Screenshot,
    ScreenRecording,
    ToggleTheme,
    MaximizeWindow,
    TileWindow,
    CloseWindow,
}

impl KeyAction {
    pub fn as_str(&self) -> &str {
        match self {
            KeyAction::OpenLauncher => "open_launcher",
            KeyAction::OpenTerminal => "open_terminal",
            KeyAction::OpenFileManager => "open_file_manager",
            KeyAction::OpenWebBrowser => "open_web_browser",
            KeyAction::ShowDesktop => "show_desktop",
            KeyAction::LockScreen => "lock_screen",
            KeyAction::Screenshot => "screenshot",
            KeyAction::ScreenRecording => "screen_recording",
            KeyAction::ToggleTheme => "toggle_theme",
            KeyAction::MaximizeWindow => "maximize_window",
            KeyAction::TileWindow => "tile_window",
            KeyAction::CloseWindow => "close_window",
        }
    }
}

/// Keyboard shortcut
#[derive(Debug, Clone)]
pub struct KeyboardShortcut {
    pub modifiers: Vec<KeyModifier>,
    pub key: String,
    pub action: KeyAction,
    pub description: String,
}

impl KeyboardShortcut {
    pub fn new(
        modifiers: Vec<KeyModifier>,
        key: String,
        action: KeyAction,
        description: String,
    ) -> Self {
        KeyboardShortcut {
            modifiers,
            key,
            action,
            description,
        }
    }

    pub fn get_key_combination(&self) -> String {
        let mut combo = String::new();
        for modifier in &self.modifiers {
            combo.push_str(modifier.as_str());
            combo.push('+');
        }
        combo.push_str(&self.key);
        combo
    }

    pub fn matches(&self, modifiers: &[KeyModifier], key: &str) -> bool {
        if self.key != key {
            return false;
        }

        if modifiers.len() != self.modifiers.len() {
            return false;
        }

        for modifier in modifiers {
            if !self.modifiers.contains(modifier) {
                return false;
            }
        }

        true
    }
}

/// Shortcut category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutCategory {
    Global,
    WindowManagement,
    Application,
}

impl ShortcutCategory {
    pub fn as_str(&self) -> &str {
        match self {
            ShortcutCategory::Global => "global",
            ShortcutCategory::WindowManagement => "window_management",
            ShortcutCategory::Application => "application",
        }
    }
}

/// Shortcut configuration
#[derive(Debug, Clone)]
pub struct ShortcutConfig {
    pub enabled: bool,
    pub allow_override: bool,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        ShortcutConfig {
            enabled: true,
            allow_override: false,
        }
    }
}

/// Keyboard shortcuts manager
#[derive(Debug, Clone)]
pub struct KeyboardShortcutsManager {
    pub shortcuts: Vec<KeyboardShortcut>,
    pub config: ShortcutConfig,
}

impl Default for KeyboardShortcutsManager {
    fn default() -> Self {
        KeyboardShortcutsManager {
            shortcuts: Vec::new(),
            config: ShortcutConfig::default(),
        }
    }
}

impl KeyboardShortcutsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config(&mut self, config: ShortcutConfig) {
        self.config = config;
    }

    pub fn add_shortcut(&mut self, shortcut: KeyboardShortcut) {
        self.shortcuts.push(shortcut);
    }

    pub fn add_default_shortcuts(&mut self) {
        // Global shortcuts
        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from(""),
            KeyAction::OpenLauncher,
            String::from("Open application launcher"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("T"),
            KeyAction::OpenTerminal,
            String::from("Open terminal"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("E"),
            KeyAction::OpenFileManager,
            String::from("Open file manager"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("B"),
            KeyAction::OpenWebBrowser,
            String::from("Open web browser"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("D"),
            KeyAction::ShowDesktop,
            String::from("Show desktop"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("L"),
            KeyAction::LockScreen,
            String::from("Lock screen"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super, KeyModifier::Shift],
            String::from("S"),
            KeyAction::Screenshot,
            String::from("Screenshot"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super, KeyModifier::Shift],
            String::from("P"),
            KeyAction::ScreenRecording,
            String::from("Screen recording"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("Esc"),
            KeyAction::ToggleTheme,
            String::from("Toggle theme"),
        ));

        // Window management shortcuts
        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("Enter"),
            KeyAction::MaximizeWindow,
            String::from("Maximize window"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super, KeyModifier::Shift],
            String::from("Enter"),
            KeyAction::TileWindow,
            String::from("Tile window"),
        ));

        self.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super, KeyModifier::Shift],
            String::from("Q"),
            KeyAction::CloseWindow,
            String::from("Close window"),
        ));
    }

    pub fn get_shortcut(&self, action: KeyAction) -> Option<&KeyboardShortcut> {
        self.shortcuts.iter().find(|s| s.action == action)
    }

    pub fn get_shortcuts_by_category(&self, category: ShortcutCategory) -> Vec<KeyboardShortcut> {
        match category {
            ShortcutCategory::Global => self
                .shortcuts
                .iter()
                .filter(|s| {
                    matches!(
                        s.action,
                        KeyAction::OpenLauncher
                            | KeyAction::OpenTerminal
                            | KeyAction::OpenFileManager
                            | KeyAction::OpenWebBrowser
                            | KeyAction::ShowDesktop
                            | KeyAction::LockScreen
                            | KeyAction::Screenshot
                            | KeyAction::ScreenRecording
                            | KeyAction::ToggleTheme
                    )
                })
                .cloned()
                .collect(),
            ShortcutCategory::WindowManagement => self
                .shortcuts
                .iter()
                .filter(|s| {
                    matches!(
                        s.action,
                        KeyAction::MaximizeWindow | KeyAction::TileWindow | KeyAction::CloseWindow
                    )
                })
                .cloned()
                .collect(),
            ShortcutCategory::Application => self
                .shortcuts
                .iter()
                .filter(|s| {
                    !matches!(
                        s.action,
                        KeyAction::OpenLauncher
                            | KeyAction::OpenTerminal
                            | KeyAction::OpenFileManager
                            | KeyAction::OpenWebBrowser
                            | KeyAction::ShowDesktop
                            | KeyAction::LockScreen
                            | KeyAction::Screenshot
                            | KeyAction::ScreenRecording
                            | KeyAction::ToggleTheme
                            | KeyAction::MaximizeWindow
                            | KeyAction::TileWindow
                            | KeyAction::CloseWindow
                    )
                })
                .cloned()
                .collect(),
        }
    }

    pub fn handle_key_press(&self, modifiers: Vec<KeyModifier>, key: String) -> Option<KeyAction> {
        if !self.config.enabled {
            return None;
        }

        for shortcut in &self.shortcuts {
            if shortcut.matches(&modifiers, &key) {
                return Some(shortcut.action);
            }
        }

        None
    }

    pub fn list_all_shortcuts(&self) -> Vec<String> {
        self.shortcuts
            .iter()
            .map(|s| {
                format!(
                    "{} - {} ({})",
                    s.get_key_combination(),
                    s.action.as_str(),
                    s.description
                )
            })
            .collect()
    }

    pub fn list_global_shortcuts(&self) -> Vec<String> {
        self.get_shortcuts_by_category(ShortcutCategory::Global)
            .iter()
            .map(|s| {
                format!(
                    "{} - {} ({})",
                    s.get_key_combination(),
                    s.action.as_str(),
                    s.description
                )
            })
            .collect()
    }

    pub fn list_window_shortcuts(&self) -> Vec<String> {
        self.get_shortcuts_by_category(ShortcutCategory::WindowManagement)
            .iter()
            .map(|s| {
                format!(
                    "{} - {} ({})",
                    s.get_key_combination(),
                    s.action.as_str(),
                    s.description
                )
            })
            .collect()
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Keyboard Shortcuts Statistics:\n");
        stats.push_str(&format!("Total shortcuts: {}\n", self.shortcuts.len()));
        stats.push_str(&format!("Enabled: {}\n", self.config.enabled));
        stats.push_str(&format!("Allow override: {}\n", self.config.allow_override));

        let global_count = self
            .get_shortcuts_by_category(ShortcutCategory::Global)
            .len();
        let window_count = self
            .get_shortcuts_by_category(ShortcutCategory::WindowManagement)
            .len();
        let app_count = self
            .get_shortcuts_by_category(ShortcutCategory::Application)
            .len();

        stats.push_str(&format!("Global shortcuts: {}\n", global_count));
        stats.push_str(&format!("Window shortcuts: {}\n", window_count));
        stats.push_str(&format!("Application shortcuts: {}\n", app_count));

        stats
    }

    pub fn reset_to_defaults(&mut self) {
        self.shortcuts.clear();
        self.add_default_shortcuts();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_modifier_as_str() {
        assert_eq!(KeyModifier::Super.as_str(), "Super");
        assert_eq!(KeyModifier::Alt.as_str(), "Alt");
    }

    #[test]
    fn test_key_action_as_str() {
        assert_eq!(KeyAction::OpenLauncher.as_str(), "open_launcher");
        assert_eq!(KeyAction::OpenTerminal.as_str(), "open_terminal");
    }

    #[test]
    fn test_keyboard_shortcut_creation() {
        let shortcut = KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("T"),
            KeyAction::OpenTerminal,
            String::from("Open terminal"),
        );
        assert_eq!(shortcut.key, "T");
        assert_eq!(shortcut.action, KeyAction::OpenTerminal);
    }

    #[test]
    fn test_keyboard_shortcut_get_key_combination() {
        let shortcut = KeyboardShortcut::new(
            vec![KeyModifier::Super, KeyModifier::Shift],
            String::from("S"),
            KeyAction::Screenshot,
            String::from("Screenshot"),
        );
        let combo = shortcut.get_key_combination();
        assert!(combo.contains("Super+Shift+S"));
    }

    #[test]
    fn test_keyboard_shortcut_matches() {
        let shortcut = KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("T"),
            KeyAction::OpenTerminal,
            String::from("Open terminal"),
        );

        assert!(shortcut.matches(&[KeyModifier::Super], "T"));
        assert!(!shortcut.matches(&[KeyModifier::Alt], "T"));
        assert!(!shortcut.matches(&[KeyModifier::Super], "E"));
    }

    #[test]
    fn test_keyboard_shortcuts_manager_creation() {
        let manager = KeyboardShortcutsManager::new();
        assert_eq!(manager.shortcuts.len(), 0);
        assert!(manager.config.enabled);
    }

    #[test]
    fn test_keyboard_shortcuts_manager_add_shortcut() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("T"),
            KeyAction::OpenTerminal,
            String::from("Open terminal"),
        ));
        assert_eq!(manager.shortcuts.len(), 1);
    }

    #[test]
    fn test_keyboard_shortcuts_manager_add_default_shortcuts() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_default_shortcuts();
        assert!(manager.shortcuts.len() > 0);
    }

    #[test]
    fn test_keyboard_shortcuts_manager_get_shortcut() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("T"),
            KeyAction::OpenTerminal,
            String::from("Open terminal"),
        ));

        let shortcut = manager.get_shortcut(KeyAction::OpenTerminal);
        assert!(shortcut.is_some());
    }

    #[test]
    fn test_keyboard_shortcuts_manager_handle_key_press() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_shortcut(KeyboardShortcut::new(
            vec![KeyModifier::Super],
            String::from("T"),
            KeyAction::OpenTerminal,
            String::from("Open terminal"),
        ));

        let action = manager.handle_key_press(vec![KeyModifier::Super], String::from("T"));
        assert_eq!(action, Some(KeyAction::OpenTerminal));
    }

    #[test]
    fn test_keyboard_shortcuts_manager_get_shortcuts_by_category() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_default_shortcuts();

        let global = manager.get_shortcuts_by_category(ShortcutCategory::Global);
        assert!(global.len() > 0);
    }

    #[test]
    fn test_keyboard_shortcuts_manager_list_all_shortcuts() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_default_shortcuts();

        let all = manager.list_all_shortcuts();
        assert!(all.len() > 0);
    }

    #[test]
    fn test_keyboard_shortcuts_manager_get_statistics() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_default_shortcuts();

        let stats = manager.get_statistics();
        assert!(stats.contains("Total shortcuts:"));
    }

    #[test]
    fn test_keyboard_shortcuts_manager_reset_to_defaults() {
        let mut manager = KeyboardShortcutsManager::new();
        manager.add_default_shortcuts();
        let count_before = manager.shortcuts.len();

        manager.reset_to_defaults();
        assert_eq!(manager.shortcuts.len(), count_before);
    }
}
