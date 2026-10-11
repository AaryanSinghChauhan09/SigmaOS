//! Zenith Quickshell Modular Plugin Architecture
//!
//! Inspired by Omarchy's modular plugin architecture for the Zenith compositor.
//! Implements plugin manifest contracts (`manifest.json`), dynamic lifecycle management,
//! and IPC message routing for status bar widgets, clipboard history, reminders, image pickers,
//! lock screens, and AI agent integration.

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Plugin Category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginCategory {
    BarWidget,
    ClipboardManager,
    NotificationsService,
    AiAgentIntegration,
    RemindersManager,
    ImagePicker,
    LockScreen,
    Custom(String),
}

impl PluginCategory {
    pub fn as_str(&self) -> &str {
        match self {
            PluginCategory::BarWidget => "bar_widget",
            PluginCategory::ClipboardManager => "clipboard_manager",
            PluginCategory::NotificationsService => "notifications_service",
            PluginCategory::AiAgentIntegration => "ai_agent_integration",
            PluginCategory::RemindersManager => "reminders_manager",
            PluginCategory::ImagePicker => "image_picker",
            PluginCategory::LockScreen => "lock_screen",
            PluginCategory::Custom(s) => s.as_str(),
        }
    }
}

/// Plugin Manifest Contract (`manifest.json`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifestContract {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub category: PluginCategory,
    pub main_qml_entry: String,
    pub permissions: Vec<String>,
    pub default_enabled: bool,
}

impl PluginManifestContract {
    pub fn new(plugin_id: &str, name: &str, category: PluginCategory) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            name: name.to_string(),
            version: "1.0.0".to_string(),
            author: "SigmaOS Desktop Team".to_string(),
            description: format!("Zenith Quickshell plugin for {}", name),
            category,
            main_qml_entry: format!("plugins/{}/main.qml", plugin_id),
            permissions: vec!["ipc:read".to_string(), "ipc:write".to_string()],
            default_enabled: true,
        }
    }

    /// Validates the plugin manifest fields
    pub fn validate(&self) -> Result<(), String> {
        if self.plugin_id.is_empty() {
            return Err("Plugin ID cannot be empty".to_string());
        }
        if self.name.is_empty() {
            return Err("Plugin name cannot be empty".to_string());
        }
        if self.main_qml_entry.is_empty() {
            return Err("Main QML entry point cannot be empty".to_string());
        }
        Ok(())
    }

    /// Serializes manifest contract to JSON-like string
    pub fn to_json_manifest(&self) -> String {
        format!(
            "{{\n  \"plugin_id\": \"{}\",\n  \"name\": \"{}\",\n  \"version\": \"{}\",\n  \"author\": \"{}\",\n  \"category\": \"{}\",\n  \"main_entry\": \"{}\",\n  \"enabled\": {}\n}}",
            self.plugin_id,
            self.name,
            self.version,
            self.author,
            self.category.as_str(),
            self.main_qml_entry,
            self.default_enabled
        )
    }
}

/// Dynamic Plugin State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginState {
    Unloaded,
    Loaded,
    Active,
    Error(String),
}

/// Plugin Instance Metadata
#[derive(Debug, Clone)]
pub struct QuickshellPluginInstance {
    pub manifest: PluginManifestContract,
    pub state: PluginState,
    pub last_ipc_message: Option<String>,
    pub render_count: u64,
}

/// Zenith Quickshell Plugin Registry & Host Manager
pub struct ZenithQuickshellPluginRegistry {
    pub plugins: Vec<QuickshellPluginInstance>,
    pub ipc_bus_history: Vec<String>,
}

impl ZenithQuickshellPluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            plugins: Vec::new(),
            ipc_bus_history: Vec::new(),
        };
        registry.register_first_party_plugins();
        registry
    }

    /// Registers standard Omarchy-inspired first-party plugins
    fn register_first_party_plugins(&mut self) {
        let first_party = vec![
            PluginManifestContract::new("omarchy.bar", "Omarchy Status Bar & Widgets", PluginCategory::BarWidget),
            PluginManifestContract::new("omarchy.clipboard", "Unified Clipboard & History", PluginCategory::ClipboardManager),
            PluginManifestContract::new("omarchy.notifications", "Sovereign Notifications Service", PluginCategory::NotificationsService),
            PluginManifestContract::new("omarchy.agents", "Herdr AI Workstation Agent", PluginCategory::AiAgentIntegration),
            PluginManifestContract::new("omarchy.reminders", "Tasks & Reminders Manager", PluginCategory::RemindersManager),
            PluginManifestContract::new("omarchy.image_picker", "Quick Image & Screenshot Picker", PluginCategory::ImagePicker),
            PluginManifestContract::new("omarchy.lockscreen", "Zenith Lock Screen", PluginCategory::LockScreen),
        ];

        for manifest in first_party {
            let _ = self.register_plugin(manifest);
        }
    }

    /// Registers a new plugin with manifest contract
    pub fn register_plugin(&mut self, manifest: PluginManifestContract) -> Result<(), String> {
        manifest.validate()?;
        if self.plugins.iter().any(|p| p.manifest.plugin_id == manifest.plugin_id) {
            return Err(format!("Plugin ID '{}' is already registered", manifest.plugin_id));
        }

        self.plugins.push(QuickshellPluginInstance {
            manifest,
            state: PluginState::Loaded,
            last_ipc_message: None,
            render_count: 0,
        });
        Ok(())
    }

    /// Enables and activates a plugin by ID
    pub fn activate_plugin(&mut self, plugin_id: &str) -> Result<String, String> {
        if let Some(plugin) = self.plugins.iter_mut().find(|p| p.manifest.plugin_id == plugin_id) {
            plugin.state = PluginState::Active;
            Ok(format!("Plugin '{}' activated successfully", plugin_id))
        } else {
            Err(format!("Plugin '{}' not found", plugin_id))
        }
    }

    /// Dispatches IPC message to target plugin or broadcasts to all
    pub fn dispatch_ipc_message(&mut self, target_plugin_id: Option<&str>, message: &str) -> usize {
        let ipc_entry = format!("[IPC] Target: {:?}, Message: {}", target_plugin_id, message);
        self.ipc_bus_history.push(ipc_entry);

        let mut delivered = 0;
        for plugin in &mut self.plugins {
            if plugin.state == PluginState::Active || plugin.state == PluginState::Loaded {
                if let Some(target) = target_plugin_id {
                    if plugin.manifest.plugin_id == target {
                        plugin.last_ipc_message = Some(message.to_string());
                        delivered += 1;
                    }
                } else {
                    plugin.last_ipc_message = Some(message.to_string());
                    delivered += 1;
                }
            }
        }
        delivered
    }

    /// Triggers rendering cycle across all active plugins
    pub fn render_quickshell_frame(&mut self) -> String {
        let mut active_count = 0;
        for plugin in &mut self.plugins {
            if plugin.state == PluginState::Active {
                plugin.render_count += 1;
                active_count += 1;
            }
        }
        format!("Zenith Quickshell Frame Rendered: {} active plugins updated", active_count)
    }
}

impl Default for ZenithQuickshellPluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest_validation() {
        let manifest = PluginManifestContract::new("test.bar", "Test Bar", PluginCategory::BarWidget);
        assert!(manifest.validate().is_ok());
        assert!(manifest.to_json_manifest().contains("test.bar"));

        let invalid = PluginManifestContract::new("", "", PluginCategory::BarWidget);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_quickshell_plugin_registry_lifecycle() {
        let mut registry = ZenithQuickshellPluginRegistry::new();
        assert!(registry.plugins.len() >= 7);

        assert!(registry.activate_plugin("omarchy.bar").is_ok());
        let delivered = registry.dispatch_ipc_message(Some("omarchy.bar"), "UPDATE_STATUS");
        assert_eq!(delivered, 1);

        let render_res = registry.render_quickshell_frame();
        assert!(render_res.contains("1 active plugins updated"));
    }
}
