// Desktop Theme Manager
// Linux Mint & Omarchy inspiration for comprehensive theme management

use std::collections::HashMap;

/// Desktop Theme Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopThemeType {
    Gtk,
    Icon,
    Cursor,
    Sound,
    Window,
    Application,
}

impl DesktopThemeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopThemeType::Gtk => "gtk",
            DesktopThemeType::Icon => "icon",
            DesktopThemeType::Cursor => "cursor",
            DesktopThemeType::Sound => "sound",
            DesktopThemeType::Window => "window",
            DesktopThemeType::Application => "application",
        }
    }
}

/// Desktop Theme
#[derive(Debug, Clone)]
pub struct DesktopTheme {
    pub id: String,
    pub name: String,
    pub theme_type: DesktopThemeType,
    pub path: String,
    pub author: String,
    pub version: String,
    pub dark_variant: Option<String>,
    pub is_dark: bool,
}

impl DesktopTheme {
    pub fn new(
        id: String,
        name: String,
        theme_type: DesktopThemeType,
        path: String,
        author: String,
        version: String,
    ) -> Self {
        Self {
            id,
            name,
            theme_type,
            path,
            author,
            version,
            dark_variant: None,
            is_dark: false,
        }
    }

    pub fn with_dark_variant(mut self, dark_variant: String) -> Self {
        self.dark_variant = Some(dark_variant);
        self
    }

    pub fn set_dark(&mut self, is_dark: bool) {
        self.is_dark = is_dark;
    }
}

/// Desktop Theme Manager
pub struct DesktopThemeManager {
    themes: HashMap<String, DesktopTheme>,
    current_gtk_theme: Option<String>,
    current_icon_theme: Option<String>,
    current_cursor_theme: Option<String>,
    current_sound_theme: Option<String>,
    current_window_theme: Option<String>,
    counter: u32,
}

impl DesktopThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_gtk_theme: None,
            current_icon_theme: None,
            current_cursor_theme: None,
            current_sound_theme: None,
            current_window_theme: None,
            counter: 1000,
        };

        // Add default themes
        manager.add_default_themes();

        manager
    }

    fn add_default_themes(&mut self) {
        // GTK themes
        let adwaita = DesktopTheme::new(
            "gtk_1".to_string(),
            "Adwaita".to_string(),
            DesktopThemeType::Gtk,
            "/usr/share/themes/Adwaita".to_string(),
            "GNOME Project".to_string(),
            "42.0".to_string(),
        )
        .with_dark_variant("Adwaita-dark".to_string());

        let yaru = DesktopTheme::new(
            "gtk_2".to_string(),
            "Yaru".to_string(),
            DesktopThemeType::Gtk,
            "/usr/share/themes/Yaru".to_string(),
            "Ubuntu".to_string(),
            "22.04".to_string(),
        )
        .with_dark_variant("Yaru-dark".to_string());

        let mint_y = DesktopTheme::new(
            "gtk_3".to_string(),
            "Mint-Y".to_string(),
            DesktopThemeType::Gtk,
            "/usr/share/themes/Mint-Y".to_string(),
            "Linux Mint".to_string(),
            "21.0".to_string(),
        )
        .with_dark_variant("Mint-Y-Dark".to_string());

        // Icon themes
        let adwaita_icons = DesktopTheme::new(
            "icon_1".to_string(),
            "Adwaita".to_string(),
            DesktopThemeType::Icon,
            "/usr/share/icons/Adwaita".to_string(),
            "GNOME Project".to_string(),
            "42.0".to_string(),
        );

        let mint_icons = DesktopTheme::new(
            "icon_2".to_string(),
            "Mint-X".to_string(),
            DesktopThemeType::Icon,
            "/usr/share/icons/Mint-X".to_string(),
            "Linux Mint".to_string(),
            "21.0".to_string(),
        );

        let papirus = DesktopTheme::new(
            "icon_3".to_string(),
            "Papirus".to_string(),
            DesktopThemeType::Icon,
            "/usr/share/icons/Papirus".to_string(),
            "Papirus Development Team".to_string(),
            "2023.01".to_string(),
        );

        // Cursor themes
        let adwaita_cursor = DesktopTheme::new(
            "cursor_1".to_string(),
            "Adwaita".to_string(),
            DesktopThemeType::Cursor,
            "/usr/share/icons/Adwaita/cursors".to_string(),
            "GNOME Project".to_string(),
            "42.0".to_string(),
        );

        let breeze_cursor = DesktopTheme::new(
            "cursor_2".to_string(),
            "Breeze".to_string(),
            DesktopThemeType::Cursor,
            "/usr/share/icons/Breeze".to_string(),
            "KDE Project".to_string(),
            "5.27".to_string(),
        );

        // Sound themes
        let freedesktop = DesktopTheme::new(
            "sound_1".to_string(),
            "freedesktop".to_string(),
            DesktopThemeType::Sound,
            "/usr/share/sounds/freedesktop".to_string(),
            "freedesktop.org".to_string(),
            "1.0".to_string(),
        );

        let mint_sound = DesktopTheme::new(
            "sound_2".to_string(),
            "Mint".to_string(),
            DesktopThemeType::Sound,
            "/usr/share/sounds/linuxmint".to_string(),
            "Linux Mint".to_string(),
            "21.0".to_string(),
        );

        // Add themes
        self.themes.insert(adwaita.id.clone(), adwaita);
        self.themes.insert(yaru.id.clone(), yaru);
        self.themes.insert(mint_y.id.clone(), mint_y);
        self.themes.insert(adwaita_icons.id.clone(), adwaita_icons);
        self.themes.insert(mint_icons.id.clone(), mint_icons);
        self.themes.insert(papirus.id.clone(), papirus);
        self.themes
            .insert(adwaita_cursor.id.clone(), adwaita_cursor);
        self.themes.insert(breeze_cursor.id.clone(), breeze_cursor);
        self.themes.insert(freedesktop.id.clone(), freedesktop);
        self.themes.insert(mint_sound.id.clone(), mint_sound);

        // Set defaults
        self.current_gtk_theme = Some("gtk_1".to_string());
        self.current_icon_theme = Some("icon_1".to_string());
        self.current_cursor_theme = Some("cursor_1".to_string());
        self.current_sound_theme = Some("sound_1".to_string());
        self.current_window_theme = Some("gtk_1".to_string());
    }

    pub fn add_theme(&mut self, theme: DesktopTheme) -> String {
        let id = format!("theme_{}", self.counter);
        self.counter += 1;

        let theme = DesktopTheme {
            id: id.clone(),
            ..theme
        };

        self.themes.insert(id.clone(), theme);
        id
    }

    pub fn remove_theme(&mut self, id: &str) -> bool {
        // Don't remove if it's currently in use
        if Some(id.to_string()) == self.current_gtk_theme
            || Some(id.to_string()) == self.current_icon_theme
            || Some(id.to_string()) == self.current_cursor_theme
            || Some(id.to_string()) == self.current_sound_theme
            || Some(id.to_string()) == self.current_window_theme
        {
            return false;
        }

        self.themes.remove(id).is_some()
    }

    pub fn get_theme(&self, id: &str) -> Option<&DesktopTheme> {
        self.themes.get(id)
    }

    pub fn get_themes(&self) -> Vec<&DesktopTheme> {
        self.themes.values().collect()
    }

    pub fn get_themes_by_type(&self, theme_type: DesktopThemeType) -> Vec<&DesktopTheme> {
        self.themes
            .values()
            .filter(|t| t.theme_type == theme_type)
            .collect()
    }

    pub fn set_gtk_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_gtk_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn set_icon_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_icon_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn set_cursor_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_cursor_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn set_sound_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_sound_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn set_window_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_window_theme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_gtk_theme(&self) -> Option<&DesktopTheme> {
        self.current_gtk_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn get_current_icon_theme(&self) -> Option<&DesktopTheme> {
        self.current_icon_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn get_current_cursor_theme(&self) -> Option<&DesktopTheme> {
        self.current_cursor_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn get_current_sound_theme(&self) -> Option<&DesktopTheme> {
        self.current_sound_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn get_current_window_theme(&self) -> Option<&DesktopTheme> {
        self.current_window_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    pub fn search_themes(&self, query: &str) -> Vec<&DesktopTheme> {
        self.themes
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query.to_lowercase())
                    || t.author.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_statistics(&self) -> DesktopThemeStatistics {
        DesktopThemeStatistics {
            total_themes: self.themes.len(),
            gtk_themes: self.get_themes_by_type(DesktopThemeType::Gtk).len(),
            icon_themes: self.get_themes_by_type(DesktopThemeType::Icon).len(),
            cursor_themes: self.get_themes_by_type(DesktopThemeType::Cursor).len(),
            sound_themes: self.get_themes_by_type(DesktopThemeType::Sound).len(),
            window_themes: self.get_themes_by_type(DesktopThemeType::Window).len(),
            current_gtk_set: self.current_gtk_theme.is_some(),
            current_icon_set: self.current_icon_theme.is_some(),
        }
    }
}

impl Default for DesktopThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// DesktopThemeStatistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopThemeStatistics {
    pub total_themes: usize,
    pub gtk_themes: usize,
    pub icon_themes: usize,
    pub cursor_themes: usize,
    pub sound_themes: usize,
    pub window_themes: usize,
    pub current_gtk_set: bool,
    pub current_icon_set: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopThemeManager::new();
        let stats = manager.get_statistics();

        assert!(stats.total_themes >= 10);
        assert!(stats.gtk_themes >= 3);
        assert!(stats.icon_themes >= 3);
        assert!(stats.cursor_themes >= 2);
        assert!(stats.sound_themes >= 2);
        assert!(stats.current_gtk_set);
        assert!(stats.current_icon_set);
    }

    #[test]
    fn test_add_theme() {
        let mut manager = DesktopThemeManager::new();
        let initial_count = manager.get_themes().len();

        let new_theme = DesktopTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            DesktopThemeType::Gtk,
            "/usr/share/themes/Custom".to_string(),
            "Author".to_string(),
            "1.0".to_string(),
        );

        let id = manager.add_theme(new_theme);
        assert!(manager.get_theme(&id).is_some());
        assert_eq!(manager.get_themes().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_theme() {
        let mut manager = DesktopThemeManager::new();

        // Add a custom theme
        let new_theme = DesktopTheme::new(
            "custom".to_string(),
            "Custom Theme".to_string(),
            DesktopThemeType::Gtk,
            "/usr/share/themes/Custom".to_string(),
            "Author".to_string(),
            "1.0".to_string(),
        );

        let id = manager.add_theme(new_theme);
        assert!(manager.remove_theme(&id));
        assert!(manager.get_theme(&id).is_none());
    }

    #[test]
    fn test_remove_current_theme() {
        let mut manager = DesktopThemeManager::new();

        // Try to remove current GTK theme (should fail)
        let current_id = manager.get_current_gtk_theme().unwrap().id.clone();
        let result = manager.remove_theme(&current_id);
        assert!(!result);
    }

    #[test]
    fn test_set_gtk_theme() {
        let mut manager = DesktopThemeManager::new();
        let gtk_themes = manager.get_themes_by_type(DesktopThemeType::Gtk);

        if gtk_themes.len() > 1 {
            let new_theme_id = gtk_themes[1].id.clone();
            assert!(manager.set_gtk_theme(&new_theme_id));
            assert_eq!(manager.get_current_gtk_theme().unwrap().id, new_theme_id);
        }
    }

    #[test]
    fn test_set_invalid_theme() {
        let mut manager = DesktopThemeManager::new();
        assert!(!manager.set_gtk_theme("nonexistent"));
    }

    #[test]
    fn test_filter_by_type() {
        let manager = DesktopThemeManager::new();
        let gtk_themes = manager.get_themes_by_type(DesktopThemeType::Gtk);
        let icon_themes = manager.get_themes_by_type(DesktopThemeType::Icon);

        assert!(!gtk_themes.is_empty());
        assert!(!icon_themes.is_empty());

        for theme in gtk_themes {
            assert_eq!(theme.theme_type, DesktopThemeType::Gtk);
        }

        for theme in icon_themes {
            assert_eq!(theme.theme_type, DesktopThemeType::Icon);
        }
    }

    #[test]
    fn test_search_themes() {
        let manager = DesktopThemeManager::new();
        let results = manager.search_themes("Adwaita");

        assert!(!results.is_empty());
        for theme in results {
            assert!(theme.name.to_lowercase().contains("adwaita"));
        }
    }

    #[test]
    fn test_dark_variant() {
        let mut manager = DesktopThemeManager::new();
        let gtk_themes = manager.get_themes_by_type(DesktopThemeType::Gtk);

        // Some themes should have dark variants
        let has_dark_variant = gtk_themes.iter().any(|t| t.dark_variant.is_some());
        assert!(has_dark_variant);
    }
}
