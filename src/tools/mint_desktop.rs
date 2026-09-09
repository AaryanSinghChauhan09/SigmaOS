extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// MintDesktop-inspired desktop settings manager
/// Provides additional desktop environment settings similar to Linux Mint's mintdesktop
/// which allows users to customize desktop behavior, window management, and appearance

#[derive(Debug, Clone, PartialEq)]
pub enum WindowManager {
    Marco,
    Compiz,
    Metacity,
    Mutter,
    KWin,
    Openbox,
    I3,
    Sway,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DesktopLayout {
    Traditional,
    Modern,
    Minimal,
    Tiling,
    Floating,
}

#[derive(Debug, Clone)]
pub struct DesktopSettings {
    pub window_manager: WindowManager,
    pub desktop_layout: DesktopLayout,
    pub show_desktop_icons: bool,
    pub show_mounted_volumes: bool,
    pub use_compositing: bool,
    pub enable_desktop_effects: bool,
    pub font_size: u8,
    pub icon_size: u8,
    pub panel_position: String,
    pub workspace_count: u8,
}

impl Default for DesktopSettings {
    fn default() -> Self {
        Self {
            window_manager: WindowManager::Marco,
            desktop_layout: DesktopLayout::Traditional,
            show_desktop_icons: true,
            show_mounted_volumes: true,
            use_compositing: true,
            enable_desktop_effects: true,
            font_size: 12,
            icon_size: 48,
            panel_position: String::from("bottom"),
            workspace_count: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeSettings {
    pub current_theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub window_theme: String,
    pub gtk_theme: String,
    pub font_name: String,
    pub document_font: String,
    pub monospace_font: String,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            current_theme: String::from("Mint-X"),
            icon_theme: String::from("Mint-X"),
            cursor_theme: String::from("default"),
            window_theme: String::from("Mint-X"),
            gtk_theme: String::from("Adwaita"),
            font_name: String::from("Ubuntu 11"),
            document_font: String::from("Ubuntu 11"),
            monospace_font: String::from("Ubuntu Mono 13"),
        }
    }
}

/// MintDesktop - Desktop settings manager
pub struct MintDesktop {
    pub desktop_settings: DesktopSettings,
    pub theme_settings: ThemeSettings,
    pub available_themes: Vec<String>,
    pub available_window_managers: Vec<WindowManager>,
}

impl MintDesktop {
    pub fn new() -> Self {
        Self {
            desktop_settings: DesktopSettings::default(),
            theme_settings: ThemeSettings::default(),
            available_themes: vec![
                String::from("Mint-X"),
                String::from("Mint-Y"),
                String::from("Mint-Z"),
                String::from("Adwaita"),
                String::from("Yaru"),
                String::from("Arc"),
                String::from("Numix"),
            ],
            available_window_managers: vec![
                WindowManager::Marco,
                WindowManager::Compiz,
                WindowManager::Metacity,
                WindowManager::Mutter,
                WindowManager::KWin,
                WindowManager::Openbox,
                WindowManager::I3,
                WindowManager::Sway,
            ],
        }
    }

    /// Set window manager
    pub fn set_window_manager(&mut self, manager: WindowManager) -> Result<(), String> {
        if self.available_window_managers.contains(&manager) {
            self.desktop_settings.window_manager = manager;
            Ok(())
        } else {
            Err(format!("Window manager {:?} not available", manager))
        }
    }

    /// Set desktop layout
    pub fn set_desktop_layout(&mut self, layout: DesktopLayout) {
        self.desktop_settings.desktop_layout = layout;
    }

    /// Toggle desktop icons
    pub fn toggle_desktop_icons(&mut self) {
        self.desktop_settings.show_desktop_icons = !self.desktop_settings.show_desktop_icons;
    }

    /// Toggle mounted volumes display
    pub fn toggle_mounted_volumes(&mut self) {
        self.desktop_settings.show_mounted_volumes = !self.desktop_settings.show_mounted_volumes;
    }

    /// Toggle compositing
    pub fn toggle_compositing(&mut self) {
        self.desktop_settings.use_compositing = !self.desktop_settings.use_compositing;
    }

    /// Toggle desktop effects
    pub fn toggle_desktop_effects(&mut self) {
        self.desktop_settings.enable_desktop_effects = !self.desktop_settings.enable_desktop_effects;
    }

    /// Set font size
    pub fn set_font_size(&mut self, size: u8) {
        self.desktop_settings.font_size = size;
    }

    /// Set icon size
    pub fn set_icon_size(&mut self, size: u8) {
        self.desktop_settings.icon_size = size;
    }

    /// Set panel position
    pub fn set_panel_position(&mut self, position: &str) {
        self.desktop_settings.panel_position = String::from(position);
    }

    /// Set workspace count
    pub fn set_workspace_count(&mut self, count: u8) {
        self.desktop_settings.workspace_count = count;
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: &str) -> Result<(), String> {
        if self.available_themes.contains(&String::from(theme)) {
            self.theme_settings.current_theme = String::from(theme);
            Ok(())
        } else {
            Err(format!("Theme {} not available", theme))
        }
    }

    /// Set icon theme
    pub fn set_icon_theme(&mut self, theme: &str) {
        self.theme_settings.icon_theme = String::from(theme);
    }

    /// Set cursor theme
    pub fn set_cursor_theme(&mut self, theme: &str) {
        self.theme_settings.cursor_theme = String::from(theme);
    }

    /// Set window theme
    pub fn set_window_theme(&mut self, theme: &str) {
        self.theme_settings.window_theme = String::from(theme);
    }

    /// Set GTK theme
    pub fn set_gtk_theme(&mut self, theme: &str) {
        self.theme_settings.gtk_theme = String::from(theme);
    }

    /// Set font name
    pub fn set_font_name(&mut self, font: &str) {
        self.theme_settings.font_name = String::from(font);
    }

    /// Set document font
    pub fn set_document_font(&mut self, font: &str) {
        self.theme_settings.document_font = String::from(font);
    }

    /// Set monospace font
    pub fn set_monospace_font(&mut self, font: &str) {
        self.theme_settings.monospace_font = String::from(font);
    }

    /// Get available themes
    pub fn get_available_themes(&self) -> Vec<&String> {
        self.available_themes.iter().collect()
    }

    /// Get available window managers
    pub fn get_available_window_managers(&self) -> Vec<&WindowManager> {
        self.available_window_managers.iter().collect()
    }

    /// Display desktop settings
    pub fn display_desktop_settings(&self) -> String {
        let mut output = String::from("=== Desktop Settings ===\n\n");
        output.push_str(&format!("Window Manager: {:?}\n", self.desktop_settings.window_manager));
        output.push_str(&format!("Desktop Layout: {:?}\n", self.desktop_settings.desktop_layout));
        output.push_str(&format!("Show Desktop Icons: {}\n", if self.desktop_settings.show_desktop_icons { "Yes" } else { "No" }));
        output.push_str(&format!("Show Mounted Volumes: {}\n", if self.desktop_settings.show_mounted_volumes { "Yes" } else { "No" }));
        output.push_str(&format!("Use Compositing: {}\n", if self.desktop_settings.use_compositing { "Yes" } else { "No" }));
        output.push_str(&format!("Enable Desktop Effects: {}\n", if self.desktop_settings.enable_desktop_effects { "Yes" } else { "No" }));
        output.push_str(&format!("Font Size: {}\n", self.desktop_settings.font_size));
        output.push_str(&format!("Icon Size: {}\n", self.desktop_settings.icon_size));
        output.push_str(&format!("Panel Position: {}\n", self.desktop_settings.panel_position));
        output.push_str(&format!("Workspace Count: {}\n", self.desktop_settings.workspace_count));
        output
    }

    /// Display theme settings
    pub fn display_theme_settings(&self) -> String {
        let mut output = String::from("=== Theme Settings ===\n\n");
        output.push_str(&format!("Current Theme: {}\n", self.theme_settings.current_theme));
        output.push_str(&format!("Icon Theme: {}\n", self.theme_settings.icon_theme));
        output.push_str(&format!("Cursor Theme: {}\n", self.theme_settings.cursor_theme));
        output.push_str(&format!("Window Theme: {}\n", self.theme_settings.window_theme));
        output.push_str(&format!("GTK Theme: {}\n", self.theme_settings.gtk_theme));
        output.push_str(&format!("Font Name: {}\n", self.theme_settings.font_name));
        output.push_str(&format!("Document Font: {}\n", self.theme_settings.document_font));
        output.push_str(&format!("Monospace Font: {}\n", self.theme_settings.monospace_font));
        output
    }

    /// Display all settings
    pub fn display_all_settings(&self) -> String {
        let mut output = self.display_desktop_settings();
        output.push_str("\n");
        output.push_str(&self.display_theme_settings());
        output
    }

    /// Reset to defaults
    pub fn reset_to_defaults(&mut self) {
        self.desktop_settings = DesktopSettings::default();
        self.theme_settings = ThemeSettings::default();
    }
}

impl Default for MintDesktop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_desktop_creation() {
        let mint_desktop = MintDesktop::new();
        assert_eq!(mint_desktop.available_themes.len(), 7);
        assert_eq!(mint_desktop.available_window_managers.len(), 8);
    }

    #[test]
    fn test_set_window_manager() {
        let mut mint_desktop = MintDesktop::new();
        let result = mint_desktop.set_window_manager(WindowManager::Compiz);
        
        assert!(result.is_ok());
        assert_eq!(mint_desktop.desktop_settings.window_manager, WindowManager::Compiz);
    }

    #[test]
    fn test_set_desktop_layout() {
        let mut mint_desktop = MintDesktop::new();
        mint_desktop.set_desktop_layout(DesktopLayout::Tiling);
        
        assert_eq!(mint_desktop.desktop_settings.desktop_layout, DesktopLayout::Tiling);
    }

    #[test]
    fn test_toggle_desktop_icons() {
        let mut mint_desktop = MintDesktop::new();
        let initial = mint_desktop.desktop_settings.show_desktop_icons;
        
        mint_desktop.toggle_desktop_icons();
        assert_ne!(mint_desktop.desktop_settings.show_desktop_icons, initial);
    }

    #[test]
    fn test_set_font_size() {
        let mut mint_desktop = MintDesktop::new();
        mint_desktop.set_font_size(14);
        
        assert_eq!(mint_desktop.desktop_settings.font_size, 14);
    }

    #[test]
    fn test_set_theme() {
        let mut mint_desktop = MintDesktop::new();
        let result = mint_desktop.set_theme("Adwaita");
        
        assert!(result.is_ok());
        assert_eq!(mint_desktop.theme_settings.current_theme, "Adwaita");
    }

    #[test]
    fn test_set_invalid_theme() {
        let mut mint_desktop = MintDesktop::new();
        let result = mint_desktop.set_theme("InvalidTheme");
        
        assert!(result.is_err());
    }

    #[test]
    fn test_reset_to_defaults() {
        let mut mint_desktop = MintDesktop::new();
        mint_desktop.set_font_size(20);
        mint_desktop.set_workspace_count(8);
        
        mint_desktop.reset_to_defaults();
        assert_eq!(mint_desktop.desktop_settings.font_size, 12);
        assert_eq!(mint_desktop.desktop_settings.workspace_count, 4);
    }

    #[test]
    fn test_display_settings() {
        let mint_desktop = MintDesktop::new();
        let output = mint_desktop.display_desktop_settings();
        
        assert!(output.contains("Desktop Settings"));
        assert!(output.contains("Window Manager"));
    }
}
