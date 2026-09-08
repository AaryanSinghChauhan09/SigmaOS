//! Linux Mint Cinnamon-inspired Desktop Environment
//! 
//! This module implements desktop environment features inspired by Linux Mint's
//! Cinnamon desktop, including panels, desklets, themes, extensions, and XApp integration.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Cinnamon panel position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonPanelPosition {
    /// Top of screen
    Top,
    /// Bottom of screen
    Bottom,
    /// Left of screen
    Left,
    /// Right of screen
    Right,
}

impl CinnamonPanelPosition {
    /// Get display name for the position
    pub fn display_name(&self) -> &'static str {
        match self {
            CinnamonPanelPosition::Top => "Top",
            CinnamonPanelPosition::Bottom => "Bottom",
            CinnamonPanelPosition::Left => "Left",
            CinnamonPanelPosition::Right => "Right",
        }
    }
}

/// Panel applet type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelAppletType {
    /// Application menu launcher
    Menu,
    /// Task switcher
    TaskSwitcher,
    /// System tray
    SystemTray,
    /// Clock/calendar
    Clock,
    /// Volume control
    Volume,
    /// Network status
    Network,
    /// Battery indicator
    Battery,
    /// Custom applet
    Custom(String),
}

/// Panel applet configuration
#[derive(Debug, Clone)]
pub struct PanelApplet {
    /// Applet type
    pub applet_type: PanelAppletType,
    /// Applet ID
    pub id: String,
    /// Whether applet is enabled
    pub enabled: bool,
    /// Applet position in panel (left-to-right index)
    pub position: u32,
    /// Applet-specific configuration
    pub config: BTreeMap<String, String>,
}

/// Cinnamon panel configuration
#[derive(Debug, Clone)]
pub struct CinnamonPanel {
    /// Panel ID
    pub id: String,
    /// Panel position
    pub position: CinnamonPanelPosition,
    /// Panel height in pixels
    pub height: u32,
    /// Whether panel is auto-hidden
    pub auto_hide: bool,
    /// Panel applets
    pub applets: Vec<PanelApplet>,
    /// Panel background color (hex)
    pub background_color: String,
    /// Panel opacity (0-255)
    pub opacity: u8,
}

impl CinnamonPanel {
    /// Create a new panel
    pub fn new(id: String, position: CinnamonPanelPosition) -> Self {
        Self {
            id,
            position,
            height: 40,
            auto_hide: false,
            applets: Vec::new(),
            background_color: "#2c2c2c".to_string(),
            opacity: 255,
        }
    }

    /// Add an applet to the panel
    pub fn add_applet(&mut self, applet: PanelApplet) {
        self.applets.push(applet);
    }

    /// Remove an applet by ID
    pub fn remove_applet(&mut self, applet_id: &str) {
        self.applets.retain(|a| a.id != applet_id);
    }

    /// Get applet by ID
    pub fn get_applet(&self, applet_id: &str) -> Option<&PanelApplet> {
        self.applets.iter().find(|a| a.id == applet_id)
    }

    /// Move applet to new position
    pub fn move_applet(&mut self, applet_id: &str, new_position: u32) {
        if let Some(idx) = self.applets.iter().position(|a| a.id == applet_id) {
            if let Some(applet) = self.applets.remove(idx) {
                let mut applet = applet;
                applet.position = new_position;
                // Insert at correct position
                let insert_idx = self
                    .applets
                    .binary_search_by_key(&new_position, |a| a.position)
                    .unwrap_or_else(|e| e);
                self.applets.insert(insert_idx, applet);
            }
        }
    }
}

/// Desklet (desktop widget) configuration
#[derive(Debug, Clone)]
pub struct CinnamonDesklet {
    /// Desklet ID
    pub id: String,
    /// Desklet name
    pub name: String,
    /// Desklet UUID
    pub uuid: String,
    /// X position on desktop
    pub x: i32,
    /// Y position on desktop
    pub y: i32,
    /// Desklet width
    pub width: u32,
    /// Desklet height
    pub height: u32,
    /// Whether desklet is enabled
    pub enabled: bool,
    /// Desklet-specific configuration
    pub config: BTreeMap<String, String>,
}

impl CinnamonDesklet {
    /// Create a new desklet
    pub fn new(id: String, name: String, uuid: String) -> Self {
        Self {
            id,
            name,
            uuid,
            x: 100,
            y: 100,
            width: 200,
            height: 150,
            enabled: true,
            config: BTreeMap::new(),
        }
    }
}

/// Cinnamon theme configuration
#[derive(Debug, Clone)]
pub struct CinnamonTheme {
    /// Theme name
    pub name: String,
    /// Theme ID
    pub id: String,
    /// GTK theme name
    pub gtk_theme: String,
    /// Icon theme name
    pub icon_theme: String,
    /// Window theme name
    pub window_theme: String,
    /// Cursor theme name
    pub cursor_theme: String,
    /// Font name
    pub font_name: String,
    /// Font size
    pub font_size: u32,
    /// Whether theme is dark
    pub dark_mode: bool,
}

impl CinnamonTheme {
    /// Create a new theme
    pub fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
            gtk_theme: "Adwaita".to_string(),
            icon_theme: "Adwaita".to_string(),
            window_theme: "Adwaita".to_string(),
            cursor_theme: "default".to_string(),
            font_name: "Sans".to_string(),
            font_size: 11,
            dark_mode: false,
        }
    }

    /// Apply dark mode
    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
        if dark {
            self.gtk_theme = "Adwaita-dark".to_string();
        } else {
            self.gtk_theme = "Adwaita".to_string();
        }
    }
}

/// Cinnamon extension configuration
#[derive(Debug, Clone)]
pub struct CinnamonExtension {
    /// Extension ID
    pub id: String,
    /// Extension name
    pub name: String,
    /// Extension UUID
    pub uuid: String,
    /// Extension description
    pub description: String,
    /// Extension version
    pub version: String,
    /// Whether extension is enabled
    pub enabled: bool,
    /// Extension-specific configuration
    pub config: BTreeMap<String, String>,
}

impl CinnamonExtension {
    /// Create a new extension
    pub fn new(id: String, name: String, uuid: String) -> Self {
        Self {
            id,
            name,
            uuid,
            description: String::new(),
            version: "1.0.0".to_string(),
            enabled: true,
            config: BTreeMap::new(),
        }
    }
}

/// XApp preferences (cross-desktop integration)
#[derive(Debug, Clone)]
pub struct XAppPreferences {
    /// Application name
    pub app_name: String,
    /// Dark mode preference
    pub dark_mode: bool,
    /// Accent color (hex)
    pub accent_color: String,
    /// Enable animations
    pub enable_animations: bool,
    /// Animation speed (1-10)
    pub animation_speed: u8,
    /// Enable sound effects
    pub enable_sounds: bool,
    /// Default font
    pub default_font: String,
    /// Monospace font
    pub monospace_font: String,
    /// Locale
    pub locale: String,
    /// Time format (12h or 24h)
    pub time_format: String,
}

impl XAppPreferences {
    /// Create new XApp preferences
    pub fn new(app_name: String) -> Self {
        Self {
            app_name,
            dark_mode: false,
            accent_color: "#3daee9".to_string(),
            enable_animations: true,
            animation_speed: 5,
            enable_sounds: true,
            default_font: "Sans 11".to_string(),
            monospace_font: "Monospace 10".to_string(),
            locale: "en_US.UTF-8".to_string(),
            time_format: "24h".to_string(),
        }
    }

    /// Set dark mode
    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
    }

    /// Set accent color
    pub fn set_accent_color(&mut self, color: String) {
        self.accent_color = color;
    }

    /// Set locale
    pub fn set_locale(&mut self, locale: String) {
        self.locale = locale;
    }
}

/// Cinnamon Desktop Manager - manages Cinnamon desktop environment
#[derive(Debug)]
pub struct CinnamonDesktopManager {
    /// Configured panels
    pub panels: Vec<CinnamonPanel>,
    /// Configured desklets
    pub desklets: Vec<CinnamonDesklet>,
    /// Active theme
    pub theme: CinnamonTheme,
    /// Installed extensions
    pub extensions: Vec<CinnamonExtension>,
    /// XApp preferences
    pub xapp_prefs: XAppPreferences,
}

impl CinnamonDesktopManager {
    /// Create a new Cinnamon Desktop Manager
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            desklets: Vec::new(),
            theme: CinnamonTheme::new("Mint-X".to_string(), "mint-x".to_string()),
            extensions: Vec::new(),
            xapp_prefs: XAppPreferences::new("cinnamon".to_string()),
        }
    }

    /// Add a panel
    pub fn add_panel(&mut self, panel: CinnamonPanel) {
        self.panels.push(panel);
    }

    /// Remove a panel by ID
    pub fn remove_panel(&mut self, panel_id: &str) {
        self.panels.retain(|p| p.id != panel_id);
    }

    /// Get panel by ID
    pub fn get_panel(&self, panel_id: &str) -> Option<&CinnamonPanel> {
        self.panels.iter().find(|p| p.id == panel_id)
    }

    /// Add a desklet
    pub fn add_desklet(&mut self, desklet: CinnamonDesklet) {
        self.desklets.push(desklet);
    }

    /// Remove a desklet by ID
    pub fn remove_desklet(&mut self, desklet_id: &str) {
        self.desklets.retain(|d| d.id != desklet_id);
    }

    /// Set the active theme
    pub fn set_theme(&mut self, theme: CinnamonTheme) {
        self.theme = theme;
    }

    /// Add an extension
    pub fn add_extension(&mut self, extension: CinnamonExtension) {
        self.extensions.push(extension);
    }

    /// Enable/disable an extension
    pub fn set_extension_enabled(&mut self, extension_id: &str, enabled: bool) {
        if let Some(ext) = self.extensions.iter_mut().find(|e| e.id == extension_id) {
            ext.enabled = enabled;
        }
    }

    /// Get enabled extensions
    pub fn get_enabled_extensions(&self) -> Vec<&CinnamonExtension> {
        self.extensions.iter().filter(|e| e.enabled).collect()
    }

    /// Apply XApp preferences
    pub fn apply_xapp_preferences(&mut self, prefs: XAppPreferences) {
        self.xapp_prefs = prefs;
        // Sync dark mode with theme
        self.theme.set_dark_mode(self.xapp_prefs.dark_mode);
    }

    /// Initialize default desktop layout
    pub fn initialize_default_layout(&mut self) {
        // Create bottom panel with default applets
        let mut bottom_panel = CinnamonPanel::new("panel-1".to_string(), CinnamonPanelPosition::Bottom);
        
        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::Menu,
            id: "menu-applet".to_string(),
            enabled: true,
            position: 0,
            config: BTreeMap::new(),
        });

        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::TaskSwitcher,
            id: "task-applet".to_string(),
            enabled: true,
            position: 1,
            config: BTreeMap::new(),
        });

        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::SystemTray,
            id: "tray-applet".to_string(),
            enabled: true,
            position: 2,
            config: BTreeMap::new(),
        });

        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::Clock,
            id: "clock-applet".to_string(),
            enabled: true,
            position: 3,
            config: BTreeMap::new(),
        });

        self.add_panel(bottom_panel);
    }
}

impl Default for CinnamonDesktopManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = CinnamonPanel::new("test-panel".to_string(), CinnamonPanelPosition::Bottom);
        assert_eq!(panel.id, "test-panel");
        assert_eq!(panel.height, 40);
        assert!(!panel.auto_hide);
    }

    #[test]
    fn test_panel_applet_management() {
        let mut panel = CinnamonPanel::new("test-panel".to_string(), CinnamonPanelPosition::Bottom);
        
        let applet = PanelApplet {
            applet_type: PanelAppletType::Menu,
            id: "menu-applet".to_string(),
            enabled: true,
            position: 0,
            config: BTreeMap::new(),
        };
        
        panel.add_applet(applet);
        assert_eq!(panel.applets.len(), 1);
        
        panel.remove_applet("menu-applet");
        assert_eq!(panel.applets.len(), 0);
    }

    #[test]
    fn test_desktop_manager_default_layout() {
        let mut manager = CinnamonDesktopManager::new();
        manager.initialize_default_layout();
        
        assert_eq!(manager.panels.len(), 1);
        assert_eq!(manager.panels[0].applets.len(), 4);
    }

    #[test]
    fn test_theme_dark_mode() {
        let mut theme = CinnamonTheme::new("test".to_string(), "test".to_string());
        assert!(!theme.dark_mode);
        
        theme.set_dark_mode(true);
        assert!(theme.dark_mode);
        assert_eq!(theme.gtk_theme, "Adwaita-dark");
    }

    #[test]
    fn test_xapp_preferences() {
        let mut prefs = XAppPreferences::new("test-app".to_string());
        assert!(!prefs.dark_mode);
        
        prefs.set_dark_mode(true);
        assert!(prefs.dark_mode);
        
        prefs.set_accent_color("#ff0000".to_string());
        assert_eq!(prefs.accent_color, "#ff0000");
    }
}
