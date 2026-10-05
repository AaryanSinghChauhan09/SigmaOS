// SPDX-License-Identifier: MIT
// SigmaOS Quickshell-Inspired Desktop Shell
// Omarchy Quickshell-inspired unified desktop shell system

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Quickshell-inspired plugin types
#[derive(Debug, Clone, PartialEq)]
pub enum QuickshellPluginType {
    Bar,
    Launcher,
    Menu,
    Notification,
    OSD,
    Panel,
    LockScreen,
    PolkitAgent,
    Widget,
}

/// Quickshell-inspired plugin configuration
#[derive(Debug, Clone)]
pub struct QuickshellPlugin {
    pub id: String,
    pub name: String,
    pub plugin_type: QuickshellPluginType,
    pub enabled: bool,
    pub config: BTreeMap<String, String>,
    pub ipc_endpoint: String,
    pub theme_colors: Vec<String>,
}

impl QuickshellPlugin {
    pub fn new(id: String, name: String, plugin_type: QuickshellPluginType) -> Self {
        Self {
            id,
            name,
            plugin_type,
            enabled: true,
            config: BTreeMap::new(),
            ipc_endpoint: String::new(),
            theme_colors: Vec::new(),
        }
    }

    /// Set plugin configuration
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    /// Set IPC endpoint
    pub fn set_ipc_endpoint(&mut self, endpoint: String) {
        self.ipc_endpoint = endpoint;
    }

    /// Set theme colors (base 24 color scheme)
    pub fn set_theme_colors(&mut self, colors: Vec<String>) {
        self.theme_colors = colors;
    }
}

/// Quickshell-inspired unified shell
#[derive(Debug, Clone)]
pub struct QuickshellUnifiedShell {
    pub plugins: BTreeMap<String, QuickshellPlugin>,
    pub active_plugins: Vec<String>,
    pub theme_base: String,
    pub layout_mode: String,
    pub ipc_enabled: bool,
    pub plugin_system_enabled: bool,
}

impl QuickshellUnifiedShell {
    pub fn new() -> Self {
        Self {
            plugins: BTreeMap::new(),
            active_plugins: Vec::new(),
            theme_base: String::from("24-color"),
            layout_mode: String::from("adaptive"),
            ipc_enabled: true,
            plugin_system_enabled: true,
        }
    }

    /// Add plugin
    pub fn add_plugin(&mut self, plugin: QuickshellPlugin) {
        let id = plugin.id.clone();
        self.plugins.insert(id, plugin);
    }

    /// Activate plugin
    pub fn activate_plugin(&mut self, id: String) -> Result<(), &'static str> {
        if !self.plugins.contains_key(&id) {
            return Err("Plugin not found");
        }
        if !self.active_plugins.contains(&id) {
            self.active_plugins.push(id);
        }
        Ok(())
    }

    /// Deactivate plugin
    pub fn deactivate_plugin(&mut self, id: &str) {
        self.active_plugins.retain(|active_id| active_id != id);
    }

    /// Enable IPC communication
    pub fn enable_ipc(&mut self) {
        self.ipc_enabled = true;
    }

    /// Disable IPC communication
    pub fn disable_ipc(&mut self) {
        self.ipc_enabled = false;
    }

    /// Enable plugin system
    pub fn enable_plugin_system(&mut self) {
        self.plugin_system_enabled = true;
    }

    /// Disable plugin system
    pub fn disable_plugin_system(&mut self) {
        self.plugin_system_enabled = false;
    }

    /// Set theme base
    pub fn set_theme_base(&mut self, base: String) {
        self.theme_base = base;
    }

    /// Set layout mode
    pub fn set_layout_mode(&mut self, mode: String) {
        self.layout_mode = mode;
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, id: &str) -> Option<&QuickshellPlugin> {
        self.plugins.get(id)
    }

    /// List all plugins
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// List active plugins
    pub fn list_active_plugins(&self) -> Vec<String> {
        self.active_plugins.clone()
    }

    /// Send IPC message to plugin
    pub fn send_ipc_message(&self, plugin_id: &str, _message: String) -> Result<(), &'static str> {
        if !self.ipc_enabled {
            return Err("IPC disabled");
        }
        if !self.active_plugins.contains(&plugin_id.to_string()) {
            return Err("Plugin not active");
        }
        // In real implementation, would send message via IPC
        Ok(())
    }
}

impl Default for QuickshellUnifiedShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickshell_plugin() {
        let mut plugin = QuickshellPlugin::new(
            String::from("bar"),
            String::from("Unified Bar"),
            QuickshellPluginType::Bar
        );
        plugin.set_config(String::from("position"), String::from("top"));
        plugin.set_ipc_endpoint(String::from("unix:/tmp/bar.sock"));
        
        assert_eq!(plugin.plugin_type, QuickshellPluginType::Bar);
        assert_eq!(plugin.ipc_endpoint, String::from("unix:/tmp/bar.sock"));
    }

    #[test]
    fn test_quickshell_unified_shell() {
        let mut shell = QuickshellUnifiedShell::new();
        
        let plugin = QuickshellPlugin::new(
            String::from("launcher"),
            String::from("Unified Launcher"),
            QuickshellPluginType::Launcher
        );
        shell.add_plugin(plugin);
        
        assert!(shell.activate_plugin(String::from("launcher")).is_ok());
        assert!(shell.active_plugins.contains(&String::from("launcher")));
    }

    #[test]
    fn test_plugin_activation_deactivation() {
        let mut shell = QuickshellUnifiedShell::new();
        let plugin = QuickshellPlugin::new(
            String::from("menu"),
            String::from("Unified Menu"),
            QuickshellPluginType::Menu
        );
        shell.add_plugin(plugin);
        
        shell.activate_plugin(String::from("menu")).unwrap();
        shell.deactivate_plugin("menu");
        assert!(!shell.active_plugins.contains(&String::from("menu")));
    }

    #[test]
    fn test_ipc_communication() {
        let mut shell = QuickshellUnifiedShell::new();
        let plugin = QuickshellPlugin::new(
            String::from("notification"),
            String::from("Unified Notification"),
            QuickshellPluginType::Notification
        );
        shell.add_plugin(plugin);
        shell.activate_plugin(String::from("notification")).unwrap();
        
        assert!(shell.send_ipc_message("notification", String::from("show:hello")).is_ok());
        shell.disable_ipc();
        assert!(shell.send_ipc_message("notification", String::from("show:hello")).is_err());
    }

    #[test]
    fn test_theme_and_layout() {
        let mut shell = QuickshellUnifiedShell::new();
        shell.set_theme_base(String::from("24-color"));
        shell.set_layout_mode(String::from("adaptive"));
        
        assert_eq!(shell.theme_base, String::from("24-color"));
        assert_eq!(shell.layout_mode, String::from("adaptive"));
    }
}