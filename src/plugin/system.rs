extern crate alloc;
// OOP-based Plugin System for SigmaOS
// Implements plugin management using OOP principles with traits and structs.


use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Plugin ID
pub type PluginID = usize;

/// Plugin trait (OOP interface)
pub trait Plugin {
    /// Get plugin ID
    fn id(&self) -> PluginID;
    /// Get plugin name
    fn name(&self) -> &[u8];
    /// Get plugin version
    fn version(&self) -> (u32, u32, u32);
    /// Initialize plugin
    fn initialize(&mut self) -> Result<(), PluginError>;
    /// Shutdown plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;
    /// Get plugin info
    fn info(&self) -> PluginInfo;
}

/// Plugin error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginError {
    Success = 0,
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InitializationFailed = 3,
    ShutdownFailed = 4,
    PermissionDenied = 5,
    InvalidState = 6,
}

/// Plugin info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PluginInfo {
    pub id: PluginID,
    pub name: [u8; 64],
    pub version_major: u32,
    pub version_minor: u32,
    pub version_patch: u32,
    pub state: PluginState,
    pub capability: PluginCapability,
}

impl PluginInfo {
    pub fn new(id: PluginID) -> Self {
        PluginInfo {
            id,
            name: [0; 64],
            version_major: 0,
            version_minor: 0,
            version_patch: 0,
            state: PluginState::Unloaded,
            capability: PluginCapability::new(),
        }
    }
}

/// Plugin state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    Unloaded = 0,
    Loaded = 1,
    Initialized = 2,
    Running = 3,
    Stopped = 4,
    Error = 5,
}

/// Plugin capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginCapability {
    pub can_initialize: bool,
    pub can_shutdown: bool,
    pub can_configure: bool,
}

impl PluginCapability {
    pub const fn new() -> Self {
        PluginCapability {
            can_initialize: false,
            can_shutdown: false,
            can_configure: false,
        }
    }

    pub const fn full() -> Self {
        PluginCapability {
            can_initialize: true,
            can_shutdown: true,
            can_configure: true,
        }
    }
}

impl Default for PluginCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple plugin (OOP: Concrete plugin class)
pub struct SimplePlugin {
    pub id: PluginID,
    pub name: [u8; 64],
    pub version: (u32, u32, u32),
    pub state: AtomicUsize, // PluginState as usize
    pub capability: PluginCapability,
    pub data: Vec<u8>,
}

impl SimplePlugin {
    pub fn new(
        id: PluginID,
        name: &[u8],
        version: (u32, u32, u32),
        capability: PluginCapability,
    ) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        name_array[..len].copy_from_slice(&name[..len]);

        SimplePlugin {
            id,
            name: name_array,
            version,
            state: AtomicUsize::new(PluginState::Unloaded as usize),
            capability,
            data: Vec::new(),
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        self.data = data.to_vec();
    }

    pub fn get_state(&self) -> PluginState {
        match self.state.load(Ordering::SeqCst) {
            0 => PluginState::Unloaded,
            1 => PluginState::Loaded,
            2 => PluginState::Initialized,
            3 => PluginState::Running,
            4 => PluginState::Stopped,
            _ => PluginState::Error,
        }
    }

    pub fn set_state(&self, state: PluginState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Plugin for SimplePlugin {
    fn id(&self) -> PluginID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn version(&self) -> (u32, u32, u32) {
        self.version
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        if !self.capability.can_initialize {
            return Err(PluginError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == PluginState::Initialized || current_state == PluginState::Running {
            return Err(PluginError::AlreadyInitialized);
        }

        self.set_state(PluginState::Initialized);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        if !self.capability.can_shutdown {
            return Err(PluginError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == PluginState::Unloaded {
            return Err(PluginError::NotInitialized);
        }

        self.set_state(PluginState::Stopped);
        Ok(())
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            id: self.id,
            name: self.name,
            version_major: self.version.0,
            version_minor: self.version.1,
            version_patch: self.version.2,
            state: self.get_state(),
            capability: self.capability,
        }
    }
}

/// Plugin manager trait (OOP interface)
pub trait PluginManager {
    /// Load plugin
    fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<PluginID, PluginError>;
    /// Unload plugin
    fn unload_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    /// Initialize plugin
    fn initialize_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    /// Shutdown plugin
    fn shutdown_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    /// Get plugin reference
    fn get_plugin(&self, id: PluginID) -> Option<&dyn Plugin>;
    /// Get manager statistics
    fn stats(&self) -> PluginStats;
}

/// Plugin statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginStats {
    pub total_plugins: usize,
    pub loaded_plugins: usize,
    pub initialized_plugins: usize,
    pub running_plugins: usize,
}

impl PluginStats {
    pub const fn new() -> Self {
        PluginStats {
            total_plugins: 0,
            loaded_plugins: 0,
            initialized_plugins: 0,
            running_plugins: 0,
        }
    }
}

impl Default for PluginStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple plugin manager (OOP: Concrete manager class)
pub struct SimplePluginManager {
    plugins: Vec<Option<Box<dyn Plugin>>>,
    stats: PluginStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagerCapability {
    pub can_load: bool,
    pub can_unload: bool,
    pub can_initialize: bool,
    pub can_shutdown: bool,
}

impl ManagerCapability {
    pub const fn new() -> Self {
        ManagerCapability {
            can_load: false,
            can_unload: false,
            can_initialize: false,
            can_shutdown: false,
        }
    }

    pub const fn full() -> Self {
        ManagerCapability {
            can_load: true,
            can_unload: true,
            can_initialize: true,
            can_shutdown: true,
        }
    }
}

impl Default for ManagerCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl SimplePluginManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimplePluginManager {
            plugins: Vec::new(),
            stats: PluginStats::new(),
            capability,
        }
    }
}

impl PluginManager for SimplePluginManager {
    fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<PluginID, PluginError> {
        if !self.capability.can_load {
            return Err(PluginError::PermissionDenied);
        }

        let id = plugin.id();
        self.plugins.push(Some(plugin));
        self.stats.total_plugins += 1;
        self.stats.loaded_plugins += 1;
        Ok(id)
    }

    fn unload_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        if !self.capability.can_unload {
            return Err(PluginError::PermissionDenied);
        }

        let mut index = None;
        for (i, plugin_option) in self.plugins.iter().enumerate() {
            if let Some(ref plugin) = *plugin_option {
                if plugin.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.plugins[i] = None;
            self.stats.loaded_plugins -= 1;
            Ok(())
        } else {
            Err(PluginError::InvalidState)
        }
    }

    fn initialize_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        if !self.capability.can_initialize {
            return Err(PluginError::PermissionDenied);
        }

        for plugin_option in &mut self.plugins {
            if let Some(ref mut plugin) = *plugin_option {
                if plugin.id() == id {
                    let res = plugin.initialize();
                    if res.is_ok() {
                        self.stats.initialized_plugins += 1;
                    }
                    return res;
                }
            }
        }
        Err(PluginError::InvalidState)
    }

    fn shutdown_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        if !self.capability.can_shutdown {
            return Err(PluginError::PermissionDenied);
        }

        for plugin_option in &mut self.plugins {
            if let Some(ref mut plugin) = *plugin_option {
                if plugin.id() == id {
                    let res = plugin.shutdown();
                    if res.is_ok() {
                        self.stats.initialized_plugins -= 1;
                    }
                    return res;
                }
            }
        }
        Err(PluginError::InvalidState)
    }

    fn get_plugin(&self, id: PluginID) -> Option<&dyn Plugin> {
        for plugin_option in &self.plugins {
            if let Some(ref plugin) = *plugin_option {
                if plugin.id() == id {
                    return Some(plugin.as_ref());
                }
            }
        }
        None
    }

    fn stats(&self) -> PluginStats {
        self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_oop_system_flows() {
        let mut manager = SimplePluginManager::new(ManagerCapability::full());
        let plugin = SimplePlugin::new(
            42,
            b"SovereignSecurityAgent",
            (1, 0, 0),
            PluginCapability::full(),
        );

        let id = manager.load_plugin(Box::new(plugin)).unwrap();
        assert_eq!(id, 42);

        let stats_before = manager.stats();
        assert_eq!(stats_before.loaded_plugins, 1);
        assert_eq!(stats_before.initialized_plugins, 0);

        // Initialize plugin
        manager.initialize_plugin(42).unwrap();
        let stats_after = manager.stats();
        assert_eq!(stats_after.initialized_plugins, 1);

        // Fetch plugin details
        let plugin_ref = manager.get_plugin(42).unwrap();
        assert_eq!(plugin_ref.name(), b"SovereignSecurityAgent");
        assert_eq!(plugin_ref.version(), (1, 0, 0));
    }
}

// ==========================================
// PLUGIN MARKETPLACE (VS Code & GNOME Extensions Parity)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionType {
    VsCodeExtension,
    GnomeExtension,
    SigmaNativeExtension,
}

#[derive(Clone)]
pub struct MarketplaceItem {
    pub id: PluginID,
    pub name: String,
    pub publisher: String,
    pub category: String,
    pub rating: f32,
    pub downloads: u64,
    pub is_verified: bool,
    pub extension_type: ExtensionType,
}

pub struct PluginMarketplace {
    pub catalog: Vec<MarketplaceItem>,
}

impl PluginMarketplace {
    pub fn new() -> Self {
        Self { catalog: Vec::new() }
    }

    pub fn register_item(&mut self, item: MarketplaceItem) {
        self.catalog.push(item);
    }

    pub fn search(&self, query: &str) -> Vec<MarketplaceItem> {
        let mut results = Vec::new();
        for item in &self.catalog {
            if item.name.contains(query) || item.publisher.contains(query) || item.category.contains(query) {
                results.push(item.clone());
            }
        }
        results
    }

    pub fn filter_by_type(&self, ext_type: ExtensionType) -> Vec<MarketplaceItem> {
        let mut results = Vec::new();
        for item in &self.catalog {
            if item.extension_type == ext_type {
                results.push(item.clone());
            }
        }
        results
    }

    pub fn install_plugin_to_manager(
        &self,
        item_id: PluginID,
        manager: &mut SimplePluginManager,
    ) -> Result<PluginID, PluginError> {
        for item in &self.catalog {
            if item.id == item_id {
                let name_bytes = item.name.as_bytes();
                let plugin = SimplePlugin::new(
                    item.id,
                    name_bytes,
                    (1, 0, 0),
                    PluginCapability::full(),
                );
                return manager.load_plugin(Box::new(plugin));
            }
        }
        Err(PluginError::InvalidState)
    }
}

impl Default for PluginMarketplace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod marketplace_tests {
    use super::*;

    #[test]
    fn test_plugin_marketplace_search_and_install() {
        let mut marketplace = PluginMarketplace::new();
        marketplace.register_item(MarketplaceItem {
            id: 101,
            name: String::from("rust-analyzer-sigma"),
            publisher: String::from("RustLang"),
            category: String::from("Programming Languages"),
            rating: 4.9,
            downloads: 125000,
            is_verified: true,
            extension_type: ExtensionType::VsCodeExtension,
        });

        marketplace.register_item(MarketplaceItem {
            id: 102,
            name: String::from("dash-to-dock-sigma"),
            publisher: String::from("GnomeDevs"),
            category: String::from("Desktop Themes"),
            rating: 4.7,
            downloads: 89000,
            is_verified: true,
            extension_type: ExtensionType::GnomeExtension,
        });

        // Search test
        let search_results = marketplace.search("rust");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "rust-analyzer-sigma");

        // Filter by extension type test
        let gnome_exts = marketplace.filter_by_type(ExtensionType::GnomeExtension);
        assert_eq!(gnome_exts.len(), 1);
        assert_eq!(gnome_exts[0].id, 102);

        // Installation test into SimplePluginManager
        let mut manager = SimplePluginManager::new(ManagerCapability::full());
        let installed_id = marketplace.install_plugin_to_manager(101, &mut manager).unwrap();
        assert_eq!(installed_id, 101);

        let stats = manager.stats();
        assert_eq!(stats.loaded_plugins, 1);

        let loaded_plugin = manager.get_plugin(101).unwrap();
        assert_eq!(loaded_plugin.name(), b"rust-analyzer-sigma");
    }
}
