// SPDX-License-Identifier: MIT
// SigmaOS Cinnamon-Inspired Desktop Environment
// Linux Mint Cinnamon-inspired desktop components

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Cinnamon-inspired applet types
#[derive(Debug, Clone, PartialEq)]
pub enum CinnamonAppletType {
    Launcher,
    Menu,
    Panel,
    Notification,
    WorkspaceSwitcher,
    SystemMonitor,
    VolumeControl,
    NetworkStatus,
    PowerOptions,
    Clock,
    Systray,
}

/// Cinnamon-inspired applet configuration
#[derive(Debug, Clone)]
pub struct CinnamonApplet {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub applet_type: CinnamonAppletType,
    pub icon: String,
    pub enabled: bool,
    pub position: u32,
    pub panel: String,
    pub config: BTreeMap<String, String>,
}

impl CinnamonApplet {
    pub fn new(uuid: String, name: String, applet_type: CinnamonAppletType) -> Self {
        Self {
            uuid,
            name,
            description: String::new(),
            applet_type,
            icon: String::new(),
            enabled: true,
            position: 0,
            panel: String::from("top"),
            config: BTreeMap::new(),
        }
    }

    /// Set applet configuration
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    /// Get applet configuration
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}

/// Cinnamon-inspired panel configuration
#[derive(Debug, Clone)]
pub struct CinnamonPanel {
    pub id: String,
    pub position: String, // top, bottom, left, right
    pub size: u32,
    pub autohide: bool,
    pub show_applets: bool,
    pub applets: Vec<String>,
    pub monitor: u32,
}

impl CinnamonPanel {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: String::from("bottom"),
            size: 48,
            autohide: false,
            show_applets: true,
            applets: Vec::new(),
            monitor: 0,
        }
    }

    /// Add applet to panel
    pub fn add_applet(&mut self, applet_id: String) {
        self.applets.push(applet_id);
    }

    /// Remove applet from panel
    pub fn remove_applet(&mut self, applet_id: &str) {
        self.applets.retain(|id| id != applet_id);
    }
}

/// Cinnamon-inspired desktop manager
#[derive(Debug, Clone)]
pub struct CinnamonDesktopManager {
    pub applets: BTreeMap<String, CinnamonApplet>,
    pub panels: BTreeMap<String, CinnamonPanel>,
    pub enabled_applets: Vec<String>,
    pub theme: String,
    pub layout: String,
}

impl CinnamonDesktopManager {
    pub fn new() -> Self {
        Self {
            applets: BTreeMap::new(),
            panels: BTreeMap::new(),
            enabled_applets: Vec::new(),
            theme: String::from("Mint-X"),
            layout: String::from("traditional"),
        }
    }

    /// Add applet
    pub fn add_applet(&mut self, applet: CinnamonApplet) {
        let uuid = applet.uuid.clone();
        self.applets.insert(uuid, applet);
    }

    /// Enable applet
    pub fn enable_applet(&mut self, uuid: String) {
        if self.applets.contains_key(&uuid) {
            self.enabled_applets.push(uuid);
        }
    }

    /// Disable applet
    pub fn disable_applet(&mut self, uuid: &str) {
        self.enabled_applets.retain(|id| id != uuid);
    }

    /// Add panel
    pub fn add_panel(&mut self, panel: CinnamonPanel) {
        let id = panel.id.clone();
        self.panels.insert(id, panel);
    }

    /// Get applet by UUID
    pub fn get_applet(&self, uuid: &str) -> Option<&CinnamonApplet> {
        self.applets.get(uuid)
    }

    /// Get panel by ID
    pub fn get_panel(&self, id: &str) -> Option<&CinnamonPanel> {
        self.panels.get(id)
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
    }

    /// Set layout
    pub fn set_layout(&mut self, layout: String) {
        self.layout = layout;
    }

    /// List all applets
    pub fn list_applets(&self) -> Vec<String> {
        self.applets.keys().cloned().collect()
    }

    /// List all panels
    pub fn list_panels(&self) -> Vec<String> {
        self.panels.keys().cloned().collect()
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
    fn test_cinnamon_applet() {
        let mut applet = CinnamonApplet::new(
            String::from("menu@cinnamon.org"),
            String::from("Menu"),
            CinnamonAppletType::Menu,
        );
        applet.set_config(String::from("icon"), String::from("start-here"));

        assert_eq!(applet.applet_type, CinnamonAppletType::Menu);
        assert_eq!(applet.get_config("icon"), Some(&String::from("start-here")));
    }

    #[test]
    fn test_cinnamon_panel() {
        let mut panel = CinnamonPanel::new(String::from("panel1"));
        panel.add_applet(String::from("menu@cinnamon.org"));
        panel.add_applet(String::from("systray@cinnamon.org"));

        assert_eq!(panel.applets.len(), 2);
        panel.remove_applet("menu@cinnamon.org");
        assert_eq!(panel.applets.len(), 1);
    }

    #[test]
    fn test_cinnamon_desktop_manager() {
        let mut manager = CinnamonDesktopManager::new();

        let applet = CinnamonApplet::new(
            String::from("clock@cinnamon.org"),
            String::from("Clock"),
            CinnamonAppletType::Clock,
        );
        manager.add_applet(applet);

        let panel = CinnamonPanel::new(String::from("bottom-panel"));
        manager.add_panel(panel);

        assert!(manager.get_applet("clock@cinnamon.org").is_some());
        assert!(manager.get_panel("bottom-panel").is_some());
    }

    #[test]
    fn test_applet_enable_disable() {
        let mut manager = CinnamonDesktopManager::new();
        let applet = CinnamonApplet::new(
            String::from("network@cinnamon.org"),
            String::from("Network"),
            CinnamonAppletType::NetworkStatus,
        );
        manager.add_applet(applet);

        manager.enable_applet(String::from("network@cinnamon.org"));
        assert!(manager
            .enabled_applets
            .contains(&String::from("network@cinnamon.org")));

        manager.disable_applet("network@cinnamon.org");
        assert!(!manager
            .enabled_applets
            .contains(&String::from("network@cinnamon.org")));
    }

    #[test]
    fn test_theme_and_layout() {
        let mut manager = CinnamonDesktopManager::new();
        manager.set_theme(String::from("Mint-Y-Dark"));
        manager.set_layout(String::from("modern"));

        assert_eq!(manager.theme, String::from("Mint-Y-Dark"));
        assert_eq!(manager.layout, String::from("modern"));
    }
}
