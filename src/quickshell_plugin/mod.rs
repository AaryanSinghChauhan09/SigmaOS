// Quickshell Plugin System
// Inspired by Omarchy v4.0 Quickshell plugin architecture
// Manages bar widgets, panels, overlays, menus, and services in a unified shell

use std::collections::HashMap;

/// Plugin kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginKind {
    BarWidget,
    Bar,
    Panel,
    Overlay,
    Menu,
    Service,
}

/// Plugin state
#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    Loaded,
    Unloaded,
    Summoned,
    Hidden,
    Error(String),
}

/// Plugin manifest
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub kinds: Vec<PluginKind>,
    pub entry_points: HashMap<String, String>,
    pub is_first_party: bool,
    pub keep_loaded: bool,
}

impl PluginManifest {
    pub fn new(
        id: String,
        name: String,
        version: String,
        author: String,
        description: String,
    ) -> Self {
        Self {
            id,
            name,
            version,
            author,
            description,
            kinds: Vec::new(),
            entry_points: HashMap::new(),
            is_first_party: false,
            keep_loaded: false,
        }
    }

    pub fn with_kinds(mut self, kinds: Vec<PluginKind>) -> Self {
        self.kinds = kinds;
        self
    }

    pub fn with_entry_point(mut self, kind: String, path: String) -> Self {
        self.entry_points.insert(kind, path);
        self
    }

    pub fn with_first_party(mut self, is_first_party: bool) -> Self {
        self.is_first_party = is_first_party;
        self
    }

    pub fn with_keep_loaded(mut self, keep_loaded: bool) -> Self {
        self.keep_loaded = keep_loaded;
        self
    }
}

/// Plugin instance
#[derive(Debug, Clone)]
pub struct PluginInstance {
    pub manifest: PluginManifest,
    pub state: PluginState,
    pub enabled: bool,
    pub config: HashMap<String, String>,
    pub payload: Option<String>,
}

impl PluginInstance {
    pub fn new(manifest: PluginManifest) -> Self {
        Self {
            manifest,
            state: PluginState::Unloaded,
            enabled: false,
            config: HashMap::new(),
            payload: None,
        }
    }

    pub fn with_config(mut self, config: HashMap<String, String>) -> Self {
        self.config = config;
        self
    }

    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    pub fn summon(&mut self, payload: Option<String>) {
        self.payload = payload;
        self.state = PluginState::Summoned;
    }

    pub fn hide(&mut self) {
        self.state = PluginState::Hidden;
        self.payload = None;
    }
}

/// Quickshell Plugin Manager
#[derive(Debug, Clone)]
pub struct QuickshellPluginManager {
    plugins: HashMap<String, PluginInstance>,
    enabled_plugins: Vec<String>,
    first_party_path: String,
    third_party_path: String,
    active_bar: Option<String>,
}

impl QuickshellPluginManager {
    pub fn new(first_party_path: String, third_party_path: String) -> Self {
        Self {
            plugins: HashMap::new(),
            enabled_plugins: Vec::new(),
            first_party_path,
            third_party_path,
            active_bar: None,
        }
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, manifest: PluginManifest) {
        let instance = PluginInstance::new(manifest.clone());
        self.plugins.insert(manifest.id.clone(), instance);
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, id: &str) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        if !self.enabled_plugins.contains(&id.to_string()) {
            self.enabled_plugins.push(id.to_string());
        }

        let instance = self.plugins.get_mut(id).unwrap();
        instance.enabled = true;
        instance.state = PluginState::Loaded;

        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, id: &str) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        self.enabled_plugins.retain(|x| x != id);

        let instance = self.plugins.get_mut(id).unwrap();
        instance.enabled = false;
        instance.state = PluginState::Unloaded;

        Ok(())
    }

    /// Get all plugins
    pub fn get_plugins(&self) -> Vec<&PluginInstance> {
        self.plugins.values().collect()
    }

    /// Get plugins by kind
    pub fn get_plugins_by_kind(&self, kind: PluginKind) -> Vec<&PluginInstance> {
        self.plugins
            .values()
            .filter(|p| p.manifest.kinds.contains(&kind))
            .collect()
    }

    /// Get enabled plugins
    pub fn get_enabled_plugins(&self) -> Vec<&PluginInstance> {
        self.enabled_plugins
            .iter()
            .filter_map(|id| self.plugins.get(id))
            .collect()
    }

    /// Get first-party plugins
    pub fn get_first_party_plugins(&self) -> Vec<&PluginInstance> {
        self.plugins
            .values()
            .filter(|p| p.manifest.is_first_party)
            .collect()
    }

    /// Get third-party plugins
    pub fn get_third_party_plugins(&self) -> Vec<&PluginInstance> {
        self.plugins
            .values()
            .filter(|p| !p.manifest.is_first_party)
            .collect()
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, id: &str) -> Option<&PluginInstance> {
        self.plugins.get(id)
    }

    /// Summon a plugin (for panels, overlays, menus)
    pub fn summon_plugin(&mut self, id: &str, payload: Option<String>) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        let instance = self.plugins.get_mut(id).unwrap();
        if !instance.enabled {
            return Err(format!("Plugin {} is not enabled", id));
        }

        instance.summon(payload);
        Ok(())
    }

    /// Hide a plugin
    pub fn hide_plugin(&mut self, id: &str) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        let instance = self.plugins.get_mut(id).unwrap();
        instance.hide();
        Ok(())
    }

    /// Set active bar
    pub fn set_active_bar(&mut self, id: &str) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        let instance = self.plugins.get(id).unwrap();
        if !instance.manifest.kinds.contains(&PluginKind::Bar) {
            return Err(format!("Plugin {} is not a bar", id));
        }

        self.active_bar = Some(id.to_string());
        Ok(())
    }

    /// Get active bar
    pub fn get_active_bar(&self) -> Option<&PluginInstance> {
        self.active_bar
            .as_ref()
            .and_then(|id| self.plugins.get(id))
    }

    /// Clone a first-party plugin for customization
    pub fn clone_plugin(&mut self, id: &str, new_id: String) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        let instance = self.plugins.get(id).unwrap();
        if !instance.manifest.is_first_party {
            return Err(format!("Can only clone first-party plugins"));
        }

        let mut new_manifest = instance.manifest.clone();
        new_manifest.id = new_id.clone();
        new_manifest.is_first_party = false;

        let new_instance = PluginInstance::new(new_manifest);
        self.plugins.insert(new_id, new_instance);

        Ok(())
    }

    /// Remove a third-party plugin
    pub fn remove_plugin(&mut self, id: &str) -> Result<(), String> {
        if !self.plugins.contains_key(id) {
            return Err(format!("Plugin {} not found", id));
        }

        let instance = self.plugins.get(id).unwrap();
        if instance.manifest.is_first_party {
            return Err(format!("Cannot remove first-party plugin"));
        }

        self.disable_plugin(id)?;
        self.plugins.remove(id);

        Ok(())
    }

    /// Get plugin statistics
    pub fn get_statistics(&self) -> PluginStatistics {
        let total = self.plugins.len();
        let enabled = self.enabled_plugins.len();
        let first_party = self.get_first_party_plugins().len();
        let third_party = self.get_third_party_plugins().len();
        let bar_widgets = self.get_plugins_by_kind(PluginKind::BarWidget).len();
        let panels = self.get_plugins_by_kind(PluginKind::Panel).len();
        let overlays = self.get_plugins_by_kind(PluginKind::Overlay).len();
        let menus = self.get_plugins_by_kind(PluginKind::Menu).len();
        let services = self.get_plugins_by_kind(PluginKind::Service).len();

        PluginStatistics {
            total,
            enabled,
            first_party,
            third_party,
            bar_widgets,
            panels,
            overlays,
            menus,
            services,
        }
    }

    /// List plugins in JSON format
    pub fn list_plugins_json(&self) -> String {
        let mut list = Vec::new();
        for plugin in self.get_plugins() {
            let entry = format!(
                r#"{{"id":"{}","name":"{}","enabled":{},"is_first_party":{},"kinds":[{:?}]}}"#,
                plugin.manifest.id,
                plugin.manifest.name,
                plugin.enabled,
                plugin.manifest.is_first_party,
                plugin.manifest.kinds
            );
            list.push(entry);
        }
        format!("[{}]", list.join(","))
    }
}

impl Default for QuickshellPluginManager {
    fn default() -> Self {
        Self::new(
            "/usr/share/sigmaos/shell/plugins".to_string(),
            "/home/user/.config/sigmaos/plugins".to_string(),
        )
    }
}

/// Plugin statistics
#[derive(Debug, Clone, PartialEq)]
pub struct PluginStatistics {
    pub total: usize,
    pub enabled: usize,
    pub first_party: usize,
    pub third_party: usize,
    pub bar_widgets: usize,
    pub panels: usize,
    pub overlays: usize,
    pub menus: usize,
    pub services: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        assert_eq!(manager.get_plugins().len(), 0);
    }

    #[test]
    fn test_register_plugin() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock widget".to_string(),
        )
        .with_kinds(vec![PluginKind::BarWidget])
        .with_first_party(true);
        manager.register_plugin(manifest);
        assert_eq!(manager.get_plugins().len(), 1);
    }

    #[test]
    fn test_enable_plugin() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock widget".to_string(),
        )
        .with_kinds(vec![PluginKind::BarWidget])
        .with_first_party(true);
        manager.register_plugin(manifest);
        manager.enable_plugin("test.clock").unwrap();
        assert_eq!(manager.get_enabled_plugins().len(), 1);
    }

    #[test]
    fn test_summon_plugin() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.menu".to_string(),
            "Menu".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A menu".to_string(),
        )
        .with_kinds(vec![PluginKind::Menu])
        .with_first_party(true);
        manager.register_plugin(manifest);
        manager.enable_plugin("test.menu").unwrap();
        manager.summon_plugin("test.menu", Some("test".to_string())).unwrap();
        let plugin = manager.get_plugin("test.menu").unwrap();
        assert_eq!(plugin.state, PluginState::Summoned);
    }

    #[test]
    fn test_hide_plugin() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.menu".to_string(),
            "Menu".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A menu".to_string(),
        )
        .with_kinds(vec![PluginKind::Menu])
        .with_first_party(true);
        manager.register_plugin(manifest);
        manager.enable_plugin("test.menu").unwrap();
        manager.summon_plugin("test.menu", None).unwrap();
        manager.hide_plugin("test.menu").unwrap();
        let plugin = manager.get_plugin("test.menu").unwrap();
        assert_eq!(plugin.state, PluginState::Hidden);
    }

    #[test]
    fn test_set_active_bar() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.bar".to_string(),
            "Bar".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A bar".to_string(),
        )
        .with_kinds(vec![PluginKind::Bar])
        .with_first_party(true);
        manager.register_plugin(manifest);
        manager.set_active_bar("test.bar").unwrap();
        assert!(manager.get_active_bar().is_some());
    }

    #[test]
    fn test_clone_plugin() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
        )
        .with_kinds(vec![PluginKind::BarWidget])
        .with_first_party(true);
        manager.register_plugin(manifest);
        manager.clone_plugin("test.clock", "my.clock".to_string()).unwrap();
        assert_eq!(manager.get_plugins().len(), 2);
    }

    #[test]
    fn test_remove_plugin() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        let manifest = PluginManifest::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
        )
        .with_kinds(vec![PluginKind::BarWidget])
        .with_first_party(false);
        manager.register_plugin(manifest);
        manager.remove_plugin("test.clock").unwrap();
        assert_eq!(manager.get_plugins().len(), 0);
    }

    #[test]
    fn test_statistics() {
        let mut manager = QuickshellPluginManager::new("/fp".to_string(), "/tp".to_string());
        manager.register_plugin(
            PluginManifest::new(
                "test.clock".to_string(),
                "Clock".to_string(),
                "1.0".to_string(),
                "Test".to_string(),
                "A clock".to_string(),
            )
            .with_kinds(vec![PluginKind::BarWidget])
            .with_first_party(true),
        );
        manager.register_plugin(
            PluginManifest::new(
                "test.menu".to_string(),
                "Menu".to_string(),
                "1.0".to_string(),
                "Test".to_string(),
                "A menu".to_string(),
            )
            .with_kinds(vec![PluginKind::Menu])
            .with_first_party(true),
        );
        manager.enable_plugin("test.clock").unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.enabled, 1);
        assert_eq!(stats.first_party, 2);
        assert_eq!(stats.bar_widgets, 1);
        assert_eq!(stats.menus, 1);
    }
}
