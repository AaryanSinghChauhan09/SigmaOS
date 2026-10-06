// SPDX-License-Identifier: MIT
// SigmaOS Omarchy-Inspired Plugin System
// Omarchy Quickshell-inspired plugin architecture with manifest system

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Plugin kind/type
#[derive(Debug, Clone, PartialEq)]
pub enum PluginKind {
    BarWidget,
    Bar,
    Panel,
    Overlay,
    Menu,
    Service,
    Custom(String),
}

/// Plugin activation mode
#[derive(Debug, Clone, PartialEq)]
pub enum PluginActivation {
    OnDemand,
    OnStartup,
    KeepLoaded,
}

/// Plugin manifest
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub description: String,
    pub kinds: Vec<PluginKind>,
    pub entry_points: BTreeMap<String, String>,
    pub activation: PluginActivation,
    pub trusted: bool,
    pub enabled: bool,
}

impl PluginManifest {
    pub fn new(id: String, name: String) -> Self {
        Self {
            schema_version: 1,
            id,
            name,
            version: String::from("1.0.0"),
            author: String::new(),
            license: String::from("MIT"),
            description: String::new(),
            kinds: Vec::new(),
            entry_points: BTreeMap::new(),
            activation: PluginActivation::OnDemand,
            trusted: false,
            enabled: true,
        }
    }

    /// Add plugin kind
    pub fn add_kind(&mut self, kind: PluginKind) {
        self.kinds.push(kind);
    }

    /// Add entry point
    pub fn add_entry_point(&mut self, kind: String, file: String) {
        self.entry_points.insert(kind, file);
    }

    /// Check if plugin is first-party (omarchy.* namespace)
    pub fn is_first_party(&self) -> bool {
        self.id.starts_with("omarchy.")
    }

    /// Validate manifest
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.id.is_empty() {
            return Err("Plugin ID cannot be empty");
        }

        if self.name.is_empty() {
            return Err("Plugin name cannot be empty");
        }

        if self.kinds.is_empty() {
            return Err("Plugin must have at least one kind");
        }

        if self.entry_points.is_empty() {
            return Err("Plugin must have at least one entry point");
        }

        Ok(())
    }
}

/// Plugin instance
#[derive(Debug, Clone)]
pub struct PluginInstance {
    pub manifest: PluginManifest,
    pub loaded: bool,
    pub error: Option<String>,
    pub last_loaded: Option<String>,
}

impl PluginInstance {
    pub fn new(manifest: PluginManifest) -> Self {
        Self {
            manifest,
            loaded: false,
            error: None,
            last_loaded: None,
        }
    }

    /// Mark as loaded
    pub fn mark_loaded(&mut self) {
        self.loaded = true;
        self.error = None;
    }

    /// Mark with error
    pub fn mark_error(&mut self, error: String) {
        self.loaded = false;
        self.error = Some(error);
    }
}

/// Plugin registry
#[derive(Debug, Clone)]
pub struct PluginRegistry {
    pub plugins: BTreeMap<String, PluginInstance>,
    pub first_party_plugins: Vec<String>,
    pub third_party_plugins: Vec<String>,
    pub disabled_plugins: Vec<String>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: BTreeMap::new(),
            first_party_plugins: Vec::new(),
            third_party_plugins: Vec::new(),
            disabled_plugins: Vec::new(),
        }
    }

    /// Register plugin
    pub fn register(&mut self, manifest: PluginManifest) -> Result<(), &'static str> {
        // Validate manifest
        manifest.validate()?;

        let id = manifest.id.clone();
        let instance = PluginInstance::new(manifest);

        // Check if first-party
        if instance.manifest.is_first_party() {
            self.first_party_plugins.push(id.clone());
        } else {
            self.third_party_plugins.push(id.clone());
        }

        self.plugins.insert(id, instance);
        Ok(())
    }

    /// Load plugin
    pub fn load(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(instance) = self.plugins.get_mut(&id) {
            if !instance.manifest.enabled {
                return Err("Plugin is disabled");
            }

            // In real implementation, would load the plugin
            instance.mark_loaded();
            Ok(())
        } else {
            Err("Plugin not found")
        }
    }

    /// Unload plugin
    pub fn unload(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(instance) = self.plugins.get_mut(&id) {
            instance.loaded = false;
            Ok(())
        } else {
            Err("Plugin not found")
        }
    }

    /// Enable plugin
    pub fn enable(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(instance) = self.plugins.get_mut(&id) {
            instance.manifest.enabled = true;
            self.disabled_plugins.retain(|d| d != &id);
            Ok(())
        } else {
            Err("Plugin not found")
        }
    }

    /// Disable plugin
    pub fn disable(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(instance) = self.plugins.get_mut(&id) {
            instance.manifest.enabled = false;
            if !self.disabled_plugins.contains(&id) {
                self.disabled_plugins.push(id);
            }
            Ok(())
        } else {
            Err("Plugin not found")
        }
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, id: &str) -> Option<&PluginInstance> {
        self.plugins.get(id)
    }

    /// Get all plugins
    pub fn get_all_plugins(&self) -> Vec<&PluginInstance> {
        self.plugins.values().collect()
    }

    /// Get plugins by kind
    pub fn get_by_kind(&self, kind: &PluginKind) -> Vec<&PluginInstance> {
        self.plugins
            .values()
            .filter(|p| p.manifest.kinds.contains(kind))
            .collect()
    }

    /// Get loaded plugins
    pub fn get_loaded(&self) -> Vec<&PluginInstance> {
        self.plugins
            .values()
            .filter(|p| p.loaded)
            .collect()
    }

    /// Get first-party plugins
    pub fn get_first_party(&self) -> Vec<&PluginInstance> {
        self.first_party_plugins
            .iter()
            .filter_map(|id| self.plugins.get(id))
            .collect()
    }

    /// Get third-party plugins
    pub fn get_third_party(&self) -> Vec<&PluginInstance> {
        self.third_party_plugins
            .iter()
            .filter_map(|id| self.plugins.get(id))
            .collect()
    }

    /// List all plugin IDs
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Rescan for plugins
    pub fn rescan(&mut self) {
        // In real implementation, would rescan plugin directories
        // For now, just ensure disabled plugins are tracked
        for (id, instance) in &self.plugins {
            if !instance.manifest.enabled && !self.disabled_plugins.contains(id) {
                // Would add to disabled list
            }
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin IPC commands
#[derive(Debug, Clone, PartialEq)]
pub enum PluginIPCCommand {
    Ping,
    Summon(String),
    Hide,
    Toggle,
    Call(String, String),
    RescanPlugins,
    ReloadConfig,
    SetPluginEnabled(String, bool),
    ListPlugins,
}

/// Plugin IPC handler
#[derive(Debug, Clone)]
pub struct PluginIPCHandler {
    pub registry: PluginRegistry,
}

impl PluginIPCHandler {
    pub fn new(registry: PluginRegistry) -> Self {
        Self { registry }
    }

    /// Handle IPC command
    pub fn handle(&mut self, command: PluginIPCCommand) -> Result<String, &'static str> {
        match command {
            PluginIPCCommand::Ping => Ok("pong".to_string()),
            PluginIPCCommand::Summon(plugin_id) => {
                self.registry.load(plugin_id)?;
                Ok("summoned".to_string())
            }
            PluginIPCCommand::Hide => {
                // In real implementation, would hide all summoned plugins
                Ok("hidden".to_string())
            }
            PluginIPCCommand::Toggle => {
                // In real implementation, would toggle visibility
                Ok("toggled".to_string())
            }
            PluginIPCCommand::Call(plugin_id, method) => {
                let _ = format!("Called {} on {}", method, plugin_id);
                Ok("called".to_string())
            }
            PluginIPCCommand::RescanPlugins => {
                self.registry.rescan();
                Ok("rescanned".to_string())
            }
            PluginIPCCommand::ReloadConfig => {
                // In real implementation, would reload configuration
                Ok("reloaded".to_string())
            }
            PluginIPCCommand::SetPluginEnabled(id, enabled) => {
                if enabled {
                    self.registry.enable(id)?;
                } else {
                    self.registry.disable(id)?;
                }
                Ok("updated".to_string())
            }
            PluginIPCCommand::ListPlugins => {
                let plugins = self.registry.list_plugins();
                Ok(format!("{:?}", plugins))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest() {
        let mut manifest = PluginManifest::new(
            String::from("omarchy.clock"),
            String::from("Clock")
        );
        
        manifest.add_kind(PluginKind::BarWidget);
        manifest.add_entry_point(String::from("barWidget"), String::from("BarWidget.qml"));
        
        assert!(manifest.is_first_party());
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        let mut manifest = PluginManifest::new(
            String::from("omarchy.network"),
            String::from("Network")
        );
        manifest.add_kind(PluginKind::Service);
        manifest.add_entry_point(String::from("service"), String::from("Service.qml"));
        
        assert!(registry.register(manifest).is_ok());
        assert_eq!(registry.plugins.len(), 1);
    }

    #[test]
    fn test_plugin_loading() {
        let mut registry = PluginRegistry::new();
        let mut manifest = PluginManifest::new(
            String::from("test.plugin"),
            String::from("Test Plugin")
        );
        manifest.add_kind(PluginKind::Service);
        manifest.add_entry_point(String::from("service"), String::from("Service.qml"));
        registry.register(manifest).unwrap();
        
        assert!(registry.load(String::from("test.plugin")).is_ok());
        assert!(registry.get_plugin("test.plugin").unwrap().loaded);
    }

    #[test]
    fn test_plugin_enable_disable() {
        let mut registry = PluginRegistry::new();
        let mut manifest = PluginManifest::new(
            String::from("test.plugin"),
            String::from("Test Plugin")
        );
        manifest.add_kind(PluginKind::Service);
        manifest.add_entry_point(String::from("service"), String::from("Service.qml"));
        registry.register(manifest).unwrap();
        
        assert!(registry.disable(String::from("test.plugin")).is_ok());
        assert!(!registry.get_plugin("test.plugin").unwrap().manifest.enabled);
        
        assert!(registry.enable(String::from("test.plugin")).is_ok());
        assert!(registry.get_plugin("test.plugin").unwrap().manifest.enabled);
    }

    #[test]
    fn test_get_by_kind() {
        let mut registry = PluginRegistry::new();
        let mut manifest = PluginManifest::new(
            String::from("omarchy.bar"),
            String::from("Bar")
        );
        manifest.add_kind(PluginKind::Bar);
        manifest.add_entry_point(String::from("bar"), String::from("Bar.qml"));
        registry.register(manifest).unwrap();
        
        let bars = registry.get_by_kind(&PluginKind::Bar);
        assert_eq!(bars.len(), 1);
    }

    #[test]
    fn test_ipc_handler() {
        let registry = PluginRegistry::new();
        let mut handler = PluginIPCHandler::new(registry);
        
        let result = handler.handle(PluginIPCCommand::Ping);
        assert_eq!(result.unwrap(), "pong");
    }

    #[test]
    fn test_first_party_detection() {
        let mut registry = PluginRegistry::new();
        
        let mut first_party = PluginManifest::new(
            String::from("omarchy.clock"),
            String::from("Clock")
        );
        first_party.add_kind(PluginKind::BarWidget);
        first_party.add_entry_point(String::from("barWidget"), String::from("BarWidget.qml"));
        registry.register(first_party).unwrap();
        
        let mut third_party = PluginManifest::new(
            String::from("custom.weather"),
            String::from("Weather")
        );
        third_party.add_kind(PluginKind::Panel);
        third_party.add_entry_point(String::from("panel"), String::from("Panel.qml"));
        registry.register(third_party).unwrap();
        
        assert_eq!(registry.get_first_party().len(), 1);
        assert_eq!(registry.get_third_party().len(), 1);
    }
}