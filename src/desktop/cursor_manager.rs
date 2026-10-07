// Desktop Cursor Manager
// Linux Mint & Omarchy inspiration for comprehensive cursor management

use std::collections::HashMap;

/// Desktop Cursor Size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopCursorSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl DesktopCursorSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopCursorSize::Small => "small",
            DesktopCursorSize::Medium => "medium",
            DesktopCursorSize::Large => "large",
            DesktopCursorSize::ExtraLarge => "extra-large",
        }
    }

    pub fn as_pixels(&self) -> u32 {
        match self {
            DesktopCursorSize::Small => 16,
            DesktopCursorSize::Medium => 24,
            DesktopCursorSize::Large => 32,
            DesktopCursorSize::ExtraLarge => 48,
        }
    }
}

/// Desktop Cursor Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopCursorType {
    Default,
    Pointer,
    Text,
    Move,
    ResizeHorizontal,
    ResizeVertical,
    ResizeDiagonal,
    Busy,
    Progress,
    Crosshair,
    Hand,
    Help,
    NotAllowed,
}

impl DesktopCursorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopCursorType::Default => "default",
            DesktopCursorType::Pointer => "pointer",
            DesktopCursorType::Text => "text",
            DesktopCursorType::Move => "move",
            DesktopCursorType::ResizeHorizontal => "ew-resize",
            DesktopCursorType::ResizeVertical => "ns-resize",
            DesktopCursorType::ResizeDiagonal => "nwse-resize",
            DesktopCursorType::Busy => "busy",
            DesktopCursorType::Progress => "progress",
            DesktopCursorType::Crosshair => "crosshair",
            DesktopCursorType::Hand => "hand",
            DesktopCursorType::Help => "help",
            DesktopCursorType::NotAllowed => "not-allowed",
        }
    }
}

/// Desktop Cursor Theme
#[derive(Debug, Clone)]
pub struct DesktopCursorTheme {
    pub id: String,
    pub name: String,
    pub author: String,
    pub cursors: HashMap<DesktopCursorType, String>,
}

impl DesktopCursorTheme {
    pub fn new(id: String, name: String, author: String) -> Self {
        Self {
            id,
            name,
            author,
            cursors: HashMap::new(),
        }
    }

    pub fn add_cursor(&mut self, cursor_type: DesktopCursorType, file_path: String) {
        self.cursors.insert(cursor_type, file_path);
    }

    pub fn get_cursor(&self, cursor_type: DesktopCursorType) -> Option<&String> {
        self.cursors.get(&cursor_type)
    }
}

/// Desktop Cursor Manager
pub struct DesktopCursorManager {
    themes: HashMap<String, DesktopCursorTheme>,
    current_theme: Option<String>,
    cursor_size: DesktopCursorSize,
    cursor_speed: u32,  // 0-100
    default_cursor: DesktopCursorType,
    counter: u32,
}

impl DesktopCursorManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: None,
            cursor_size: DesktopCursorSize::Medium,
            cursor_speed: 50,
            default_cursor: DesktopCursorType::Default,
            counter: 1000,
        };

        // Add default theme
        manager.add_default_theme();

        manager
    }

    fn add_default_theme(&mut self) {
        let mut theme = DesktopCursorTheme::new(
            "theme_0".to_string(),
            "Default Cursor Theme".to_string(),
            "SigmaOS".to_string(),
        );

        // Add default cursors
        theme.add_cursor(DesktopCursorType::Default, "/usr/share/cursors/default/left_ptr".to_string());
        theme.add_cursor(DesktopCursorType::Pointer, "/usr/share/cursors/default/hand2".to_string());
        theme.add_cursor(DesktopCursorType::Text, "/usr/share/cursors/default/text".to_string());
        theme.add_cursor(DesktopCursorType::Move, "/usr/share/cursors/default/fleur".to_string());
        theme.add_cursor(DesktopCursorType::Busy, "/usr/share/cursors/default/watch".to_string());
        theme.add_cursor(DesktopCursorType::Progress, "/usr/share/cursors/default/left_ptr_watch".to_string());
        theme.add_cursor(DesktopCursorType::Crosshair, "/usr/share/cursors/default/crosshair".to_string());
        theme.add_cursor(DesktopCursorType::Hand, "/usr/share/cursors/default/hand1".to_string());
        theme.add_cursor(DesktopCursorType::NotAllowed, "/usr/share/cursors/default/circle".to_string());

        let theme_id = theme.id.clone();
        self.themes.insert(theme_id.clone(), theme);
        self.current_theme = Some(theme_id);
    }

    pub fn add_theme(&mut self, theme: DesktopCursorTheme) -> String {
        let id = format!("theme_{}", self.counter);
        self.counter += 1;

        let theme = DesktopCursorTheme {
            id: id.clone(),
            ..theme
        };

        self.themes.insert(id.clone(), theme);
        id
    }

    pub fn remove_theme(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.current_theme {
            return false;
        }
        self.themes.remove(id).is_some()
    }

    pub fn get_theme(&self, id: &str) -> Option<&DesktopCursorTheme> {
        self.themes.get(id)
    }

    pub fn get_themes(&self) -> Vec<&DesktopCursorTheme> {
        self.themes.values().collect()
    }

    pub fn set_current_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_theme(&self) -> Option<&DesktopCursorTheme> {
        self.current_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn set_cursor_size(&mut self, size: DesktopCursorSize) {
        self.cursor_size = size;
    }

    pub fn get_cursor_size(&self) -> DesktopCursorSize {
        self.cursor_size
    }

    pub fn set_cursor_speed(&mut self, speed: u32) {
        self.cursor_speed = speed.min(100);
    }

    pub fn get_cursor_speed(&self) -> u32 {
        self.cursor_speed
    }

    pub fn set_default_cursor(&mut self, cursor_type: DesktopCursorType) {
        self.default_cursor = cursor_type;
    }

    pub fn get_default_cursor(&self) -> DesktopCursorType {
        self.default_cursor
    }

    pub fn get_cursor(&self, cursor_type: DesktopCursorType) -> Option<&String> {
        if let Some(theme) = self.get_current_theme() {
            theme.get_cursor(cursor_type)
        } else {
            None
        }
    }

    pub fn add_cursor_to_theme(&mut self, theme_id: &str, cursor_type: DesktopCursorType, file_path: String) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.add_cursor(cursor_type, file_path);
            true
        } else {
            false
        }
    }

    pub fn remove_cursor_from_theme(&mut self, theme_id: &str, cursor_type: DesktopCursorType) -> bool {
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.cursors.remove(&cursor_type).is_some()
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> DesktopCursorManagerStatistics {
        DesktopCursorManagerStatistics {
            total_themes: self.themes.len(),
            current_theme_set: self.current_theme.is_some(),
            cursor_size: self.cursor_size.as_str().to_string(),
            cursor_speed: self.cursor_speed,
        }
    }
}

impl Default for DesktopCursorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopCursorManagerStatistics
#[derive(Debug, Clone)]
pub struct DesktopCursorManagerStatistics {
    pub total_themes: usize,
    pub current_theme_set: bool,
    pub cursor_size: String,
    pub cursor_speed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopCursorManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_themes, 1);
        assert!(stats.current_theme_set);
        assert_eq!(stats.cursor_size, "medium");
        assert_eq!(stats.cursor_speed, 50);
    }

    #[test]
    fn test_add_theme() {
        let mut manager = DesktopCursorManager::new();
        let initial_count = manager.get_themes().len();

        let theme = DesktopCursorTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_theme(theme);
        assert!(manager.get_theme(&id).is_some());
        assert_eq!(manager.get_themes().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_theme() {
        let mut manager = DesktopCursorManager::new();

        let theme = DesktopCursorTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_theme(theme);
        assert!(manager.remove_theme(&id));
        assert!(manager.get_theme(&id).is_none());
    }

    #[test]
    fn test_remove_current_theme() {
        let mut manager = DesktopCursorManager::new();

        // Try to remove current theme (should fail)
        let current_id = manager.get_current_theme().unwrap().id.clone();
        let result = manager.remove_theme(&current_id);
        assert!(!result);
    }

    #[test]
    fn test_set_current_theme() {
        let mut manager = DesktopCursorManager::new();

        let theme = DesktopCursorTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_theme(theme);
        assert!(manager.set_current_theme(&id));

        let current = manager.get_current_theme().unwrap();
        assert_eq!(current.id, id);
    }

    #[test]
    fn test_get_cursor() {
        let manager = DesktopCursorManager::new();

        let cursor = manager.get_cursor(DesktopCursorType::Pointer);
        assert!(cursor.is_some());
        assert!(cursor.unwrap().contains("hand2"));

        let no_cursor = manager.get_cursor(DesktopCursorType::ResizeDiagonal);
        assert!(no_cursor.is_none());
    }

    #[test]
    fn test_cursor_size() {
        let mut manager = DesktopCursorManager::new();

        manager.set_cursor_size(DesktopCursorSize::Large);
        assert_eq!(manager.get_cursor_size(), DesktopCursorSize::Large);
        assert_eq!(manager.get_cursor_size().as_pixels(), 32);
    }

    #[test]
    fn test_cursor_speed() {
        let mut manager = DesktopCursorManager::new();

        manager.set_cursor_speed(75);
        assert_eq!(manager.get_cursor_speed(), 75);

        manager.set_cursor_speed(150);
        assert_eq!(manager.get_cursor_speed(), 100);
    }

    #[test]
    fn test_default_cursor() {
        let mut manager = DesktopCursorManager::new();

        manager.set_default_cursor(DesktopCursorType::Crosshair);
        assert_eq!(manager.get_default_cursor(), DesktopCursorType::Crosshair);
    }

    #[test]
    fn test_add_cursor_to_theme() {
        let mut manager = DesktopCursorManager::new();
        let theme_id = "theme_0";

        assert!(manager.add_cursor_to_theme(
            theme_id,
            DesktopCursorType::NotAllowed,
            "/custom/circle.png".to_string(),
        ));

        let theme = manager.get_theme(theme_id).unwrap();
        assert_eq!(theme.get_cursor(DesktopCursorType::NotAllowed), Some(&"/custom/circle.png".to_string()));
    }

    #[test]
    fn test_remove_cursor_from_theme() {
        let mut manager = DesktopCursorManager::new();
        let theme_id = "theme_0";

        assert!(manager.remove_cursor_from_theme(theme_id, DesktopCursorType::Busy));

        let theme = manager.get_theme(theme_id).unwrap();
        assert!(theme.get_cursor(DesktopCursorType::Busy).is_none());
    }
}
