// Desklet and Applet Manager
// Inspired by Linux Mint Cinnamon desklet and applet system
// Manages desktop widgets (desklets) and panel widgets (applets) with lifecycle management

use std::collections::HashMap;

/// Desklet/Applet kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtensionKind {
    Desklet,
    Applet,
    Extension,
    Theme,
    Action,
}

/// Extension state
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionState {
    Loaded,
    Unloaded,
    Error(String),
}

/// Extension metadata
#[derive(Debug, Clone)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub kind: ExtensionKind,
    pub entry_point: String,
    pub icon: Option<String>,
    pub uuid: String,
}

impl ExtensionMetadata {
    pub fn new(
        id: String,
        name: String,
        version: String,
        author: String,
        description: String,
        kind: ExtensionKind,
        entry_point: String,
    ) -> Self {
        Self {
            id,
            name,
            version,
            author,
            description,
            kind,
            entry_point,
            icon: None,
            uuid: generate_uuid(),
        }
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Extension instance
#[derive(Debug, Clone)]
pub struct ExtensionInstance {
    pub metadata: ExtensionMetadata,
    pub state: ExtensionState,
    pub enabled: bool,
    pub config: HashMap<String, String>,
}

impl ExtensionInstance {
    pub fn new(metadata: ExtensionMetadata) -> Self {
        Self {
            metadata,
            state: ExtensionState::Unloaded,
            enabled: false,
            config: HashMap::new(),
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
}

/// Desklet/Applet Manager
#[derive(Debug, Clone)]
pub struct DeskletAppletManager {
    extensions: HashMap<String, ExtensionInstance>,
    enabled_extensions: Vec<String>,
    extension_path: String,
}

impl DeskletAppletManager {
    pub fn new(extension_path: String) -> Self {
        Self {
            extensions: HashMap::new(),
            enabled_extensions: Vec::new(),
            extension_path,
        }
    }

    /// Register an extension
    pub fn register_extension(&mut self, metadata: ExtensionMetadata) {
        let instance = ExtensionInstance::new(metadata.clone());
        self.extensions.insert(metadata.id.clone(), instance);
    }

    /// Enable an extension
    pub fn enable_extension(&mut self, id: &str) -> Result<(), String> {
        if !self.extensions.contains_key(id) {
            return Err(format!("Extension {} not found", id));
        }

        if !self.enabled_extensions.contains(&id.to_string()) {
            self.enabled_extensions.push(id.to_string());
        }

        let instance = self.extensions.get_mut(id).unwrap();
        instance.enabled = true;
        instance.state = ExtensionState::Loaded;

        Ok(())
    }

    /// Disable an extension
    pub fn disable_extension(&mut self, id: &str) -> Result<(), String> {
        if !self.extensions.contains_key(id) {
            return Err(format!("Extension {} not found", id));
        }

        self.enabled_extensions.retain(|x| x != id);

        let instance = self.extensions.get_mut(id).unwrap();
        instance.enabled = false;
        instance.state = ExtensionState::Unloaded;

        Ok(())
    }

    /// Get all extensions
    pub fn get_extensions(&self) -> Vec<&ExtensionInstance> {
        self.extensions.values().collect()
    }

    /// Get extensions by kind
    pub fn get_extensions_by_kind(&self, kind: ExtensionKind) -> Vec<&ExtensionInstance> {
        self.extensions
            .values()
            .filter(|e| e.metadata.kind == kind)
            .collect()
    }

    /// Get enabled extensions
    pub fn get_enabled_extensions(&self) -> Vec<&ExtensionInstance> {
        self.enabled_extensions
            .iter()
            .filter_map(|id| self.extensions.get(id))
            .collect()
    }

    /// Get extension by ID
    pub fn get_extension(&self, id: &str) -> Option<&ExtensionInstance> {
        self.extensions.get(id)
    }

    /// Update an extension
    pub fn update_extension(&mut self, id: &str, new_metadata: ExtensionMetadata) -> Result<(), String> {
        if !self.extensions.contains_key(id) {
            return Err(format!("Extension {} not found", id));
        }

        let instance = self.extensions.get_mut(id).unwrap();
        instance.metadata = new_metadata;

        Ok(())
    }

    /// Remove an extension
    pub fn remove_extension(&mut self, id: &str) -> Result<(), String> {
        if !self.extensions.contains_key(id) {
            return Err(format!("Extension {} not found", id));
        }

        self.disable_extension(id)?;
        self.extensions.remove(id);

        Ok(())
    }

    /// Check for updates
    pub fn check_updates(&self) -> Vec<&ExtensionInstance> {
        self.extensions
            .values()
            .filter(|e| self.has_update(e))
            .collect()
    }

    /// Update all extensions of a kind
    pub fn update_kind(&mut self, kind: ExtensionKind) -> Result<usize, String> {
        let extensions = self.get_extensions_by_kind(kind.clone());
        let mut updated = 0;

        for ext in extensions {
            if self.has_update(ext) {
                // Simulate update
                updated += 1;
            }
        }

        Ok(updated)
    }

    /// Get extension statistics
    pub fn get_statistics(&self) -> ExtensionStatistics {
        let total = self.extensions.len();
        let enabled = self.enabled_extensions.len();
        let desklets = self.get_extensions_by_kind(ExtensionKind::Desklet).len();
        let applets = self.get_extensions_by_kind(ExtensionKind::Applet).len();
        let extensions = self.get_extensions_by_kind(ExtensionKind::Extension).len();
        let themes = self.get_extensions_by_kind(ExtensionKind::Theme).len();
        let actions = self.get_extensions_by_kind(ExtensionKind::Action).len();

        ExtensionStatistics {
            total,
            enabled,
            desklets,
            applets,
            extensions,
            themes,
            actions,
        }
    }

    fn has_update(&self, _ext: &ExtensionInstance) -> bool {
        // In a real implementation, this would check remote versions
        false
    }
}

impl Default for DeskletAppletManager {
    fn default() -> Self {
        Self::new("/usr/share/sigmaos/extensions".to_string())
    }
}

/// Extension statistics
#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionStatistics {
    pub total: usize,
    pub enabled: usize,
    pub desklets: usize,
    pub applets: usize,
    pub extensions: usize,
    pub themes: usize,
    pub actions: usize,
}

/// Generate a simple UUID
fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = DeskletAppletManager::new("/test/path".to_string());
        assert_eq!(manager.get_extensions().len(), 0);
    }

    #[test]
    fn test_register_extension() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        let metadata = ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock desklet".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        );
        manager.register_extension(metadata);
        assert_eq!(manager.get_extensions().len(), 1);
    }

    #[test]
    fn test_enable_extension() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        let metadata = ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        );
        manager.register_extension(metadata);
        manager.enable_extension("test.clock").unwrap();
        assert_eq!(manager.get_enabled_extensions().len(), 1);
    }

    #[test]
    fn test_disable_extension() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        let metadata = ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        );
        manager.register_extension(metadata);
        manager.enable_extension("test.clock").unwrap();
        manager.disable_extension("test.clock").unwrap();
        assert_eq!(manager.get_enabled_extensions().len(), 0);
    }

    #[test]
    fn test_get_extensions_by_kind() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        manager.register_extension(ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        ));
        manager.register_extension(ExtensionMetadata::new(
            "test.menu".to_string(),
            "Menu".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A menu applet".to_string(),
            ExtensionKind::Applet,
            "menu.js".to_string(),
        ));

        let desklets = manager.get_extensions_by_kind(ExtensionKind::Desklet);
        assert_eq!(desklets.len(), 1);
    }

    #[test]
    fn test_extension_config() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        let metadata = ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        );
        manager.register_extension(metadata);
        let instance = manager.extensions.get_mut("test.clock").unwrap();
        instance.set_config("format".to_string(), "24h".to_string());
        assert_eq!(instance.get_config("format"), Some(&"24h".to_string()));
    }

    #[test]
    fn test_remove_extension() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        let metadata = ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        );
        manager.register_extension(metadata);
        manager.remove_extension("test.clock").unwrap();
        assert_eq!(manager.get_extensions().len(), 0);
    }

    #[test]
    fn test_statistics() {
        let mut manager = DeskletAppletManager::new("/test".to_string());
        manager.register_extension(ExtensionMetadata::new(
            "test.clock".to_string(),
            "Clock".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A clock".to_string(),
            ExtensionKind::Desklet,
            "clock.js".to_string(),
        ));
        manager.register_extension(ExtensionMetadata::new(
            "test.menu".to_string(),
            "Menu".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
            "A menu".to_string(),
            ExtensionKind::Applet,
            "menu.js".to_string(),
        ));
        manager.enable_extension("test.clock").unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.enabled, 1);
        assert_eq!(stats.desklets, 1);
        assert_eq!(stats.applets, 1);
    }
}
