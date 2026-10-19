//! SigmaOS Plugin Architecture
//!
//! Inspired by: Omarchy's plugin system (third-party plugins from git, managed from Setup > Plugins)
//! Following principles: PLUG_IN_DRIVER_MODEL, COMPOSITION_OVER_INHERITANCE, MODULARITY,
//! COMPONENT_ISOLATION, INVERSION_OF_CONTROL, INTERFACE_SEGREGATION, OPEN_CLOSED,
//! SINGLE_RESPONSIBILITY, LOOSE_COUPLING, DISCOVERABILITY, CODE_REUSABILITY,
//! OPTIMISE_FOR_DELETION, YOU_AREN_T_GONNA_NEED_IT, RULE_OF_THREE.

use std::collections::HashMap;
use std::fs;

/// Plugin kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginKind {
    BarWidget,
    Panel,
    Overlay,
    Menu,
    Service,
    Theme,
    Effect,
}

/// Plugin manifest
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub kinds: Vec<PluginKind>,
    pub entry_points: HashMap<String, String>,
    pub keep_loaded: bool,
    pub default_enabled: bool,
    pub dependencies: Vec<String>,
}

/// Plugin
#[derive(Debug, Clone)]
pub struct Plugin {
    pub manifest: PluginManifest,
    pub path: String,
    pub enabled: bool,
    pub loaded: bool,
    pub error: Option<String>,
}

/// Plugin manager
pub struct PluginManager {
    pub plugins: HashMap<String, Plugin>,
    pub plugin_dirs: Vec<String>,
    pub auto_discover: bool,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_dirs: vec![
                "/usr/share/sigmaos/plugins".to_string(),
                "~/.config/sigmaos/plugins".to_string(),
            ],
            auto_discover: true,
        }
    }

    /// Register a plugin
    pub fn register(&mut self, manifest: PluginManifest, path: String) {
        let plugin = Plugin {
            manifest: manifest.clone(),
            path,
            enabled: manifest.default_enabled,
            loaded: false,
            error: None,
        };
        self.plugins.insert(manifest.id.clone(), plugin);
    }

    /// Enable a plugin
    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(id).ok_or_else(|| format!("Plugin {} not found", id))?;
        plugin.enabled = true;
        Ok(())
    }

    /// Disable a plugin
    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(id).ok_or_else(|| format!("Plugin {} not found", id))?;
        plugin.enabled = false;
        plugin.loaded = false;
        Ok(())
    }

    /// Load a plugin
    pub fn load(&mut self, id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(id).ok_or_else(|| format!("Plugin {} not found", id))?;
        if !plugin.enabled {
            return Err(format!("Plugin {} is disabled", id));
        }
        plugin.loaded = true;
        plugin.error = None;
        Ok(())
    }

    /// Unload a plugin
    pub fn unload(&mut self, id: &str) -> Result<(), String> {
        let plugin = self.plugins.get_mut(id).ok_or_else(|| format!("Plugin {} not found", id))?;
        plugin.loaded = false;
        Ok(())
    }

    /// List enabled plugins
    pub fn list_enabled(&self) -> Vec<&Plugin> {
        self.plugins.values().filter(|p| p.enabled).collect()
    }

    /// List loaded plugins
    pub fn list_loaded(&self) -> Vec<&Plugin> {
        self.plugins.values().filter(|p| p.loaded).collect()
    }

    /// Get plugin by ID
    pub fn get(&self, id: &str) -> Option<&Plugin> {
        self.plugins.get(id)
    }

    /// Auto-discover plugins from directories
    pub fn auto_discover(&mut self) -> Result<usize, String> {
        if !self.auto_discover {
            return Ok(0);
        }
        let mut count = 0;
        for dir in &self.plugin_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let manifest_path = path.join("manifest.json");
                        if manifest_path.exists() {
                            // In production, parse the manifest.json
                            // For now, register a placeholder
                            let id = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                            let manifest = PluginManifest {
                                schema_version: 1,
                                id: id.clone(),
                                name: id.clone(),
                                version: "1.0.0".to_string(),
                                author: "community".to_string(),
                                description: "Community plugin".to_string(),
                                kinds: vec![PluginKind::BarWidget],
                                entry_points: HashMap::new(),
                                keep_loaded: false,
                                default_enabled: true,
                                dependencies: vec![],
                            };
                            self.register(manifest, path.to_string_lossy().to_string());
                            count += 1;
                        }
                    }
                }
            }
        }
        Ok(count)
    }

    /// Remove a plugin
    pub fn remove(&mut self, id: &str) -> Result<Plugin, String> {
        self.plugins.remove(id).ok_or_else(|| format!("Plugin {} not found", id))
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_enable() {
        let mut mgr = PluginManager::new();
        let manifest = PluginManifest {
            schema_version: 1,
            id: "weather-widget".to_string(),
            name: "Weather Widget".to_string(),
            version: "1.0.0".to_string(),
            author: "user".to_string(),
            description: "Weather bar widget".to_string(),
            kinds: vec![PluginKind::BarWidget],
            entry_points: HashMap::new(),
            keep_loaded: false,
            default_enabled: false,
            dependencies: vec![],
        };
        mgr.register(manifest, "/plugins/weather".to_string());
        mgr.enable("weather-widget").unwrap();
        assert!(mgr.get("weather-widget").unwrap().enabled);
    }

    #[test]
    fn test_load_disabled_plugin_fails() {
        let mut mgr = PluginManager::new();
        let manifest = PluginManifest {
            schema_version: 1,
            id: "test".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            author: "user".to_string(),
            description: "Test".to_string(),
            kinds: vec![PluginKind::BarWidget],
            entry_points: HashMap::new(),
            keep_loaded: false,
            default_enabled: false,
            dependencies: vec![],
        };
        mgr.register(manifest, "/plugins/test".to_string());
        assert!(mgr.load("test").is_err());
    }

    #[test]
    fn test_list_enabled() {
        let mut mgr = PluginManager::new();
        let manifest1 = PluginManifest {
            schema_version: 1,
            id: "plugin-a".to_string(),
            name: "Plugin A".to_string(),
            version: "1.0.0".to_string(),
            author: "user".to_string(),
            description: "A".to_string(),
            kinds: vec![PluginKind::BarWidget],
            entry_points: HashMap::new(),
            keep_loaded: false,
            default_enabled: true,
            dependencies: vec![],
        };
        let manifest2 = PluginManifest {
            schema_version: 1,
            id: "plugin-b".to_string(),
            name: "Plugin B".to_string(),
            version: "1.0.0".to_string(),
            author: "user".to_string(),
            description: "B".to_string(),
            kinds: vec![PluginKind::Panel],
            entry_points: HashMap::new(),
            keep_loaded: false,
            default_enabled: false,
            dependencies: vec![],
        };
        mgr.register(manifest1, "/plugins/a".to_string());
        mgr.register(manifest2, "/plugins/b".to_string());
        assert_eq!(mgr.list_enabled().len(), 1);
        assert_eq!(mgr.list_enabled()[0].manifest.id, "plugin-a");
    }

    #[test]
    fn test_remove_plugin() {
        let mut mgr = PluginManager::new();
        let manifest = PluginManifest {
            schema_version: 1,
            id: "temp".to_string(),
            name: "Temp".to_string(),
            version: "1.0.0".to_string(),
            author: "user".to_string(),
            description: "Temp".to_string(),
            kinds: vec![PluginKind::BarWidget],
            entry_points: HashMap::new(),
            keep_loaded: false,
            default_enabled: true,
            dependencies: vec![],
        };
        mgr.register(manifest, "/plugins/temp".to_string());
        let removed = mgr.remove("temp").unwrap();
        assert_eq!(removed.manifest.id, "temp");
        assert!(mgr.get("temp").is_none());
    }
}