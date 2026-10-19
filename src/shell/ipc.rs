//! SigmaOS Shell IPC Manager
//!
//! Inspired by: Omarchy shell IPC (omancy-shell, quickshell IPC contract)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, COMMAND_QUERY_SEPARATION,
//! INTERFACE_SEGREGATION, INVERSION_OF_CONTROL, POLYMORPHISM, COMPOSITION_OVER_INHERITANCE,
/// RACE_FREE DESIGN, SAFE_CONCURRENCY, DEADLOCK_AVOIDANCE, ZERO_COPY, MEMORY_SAFETY.

/// IPC method
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcMethod {
    Ping,
    Summon,
    Hide,
    Toggle,
    Call,
    ReloadConfig,
    SetPluginEnabled,
    ListPlugins,
    RescanPlugins,
}

/// IPC target
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcTarget {
    Shell,
    Plugin(String),
}

/// IPC message
#[derive(Debug, Clone)]
pub struct IpcMessage {
    pub id: u64,
    pub target: IpcTarget,
    pub method: IpcMethod,
    pub payload: Option<String>,
    pub timestamp: u64,
}

/// IPC response
#[derive(Debug, Clone)]
pub enum IpcResponse {
    Ok,
    Unknown,
    Success(String),
    Error(String),
    PluginList(Vec<String>),
}

/// IPC manager
pub struct IpcManager {
    pub next_id: u64,
    pub pending_calls: HashMap<u64, IpcMessage>,
    pub plugin_registry: HashMap<String, PluginInfo>,
}

/// Plugin info
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub entry_point: String,
    pub is_enabled: bool,
    pub is_loaded: bool,
}

impl IpcManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            pending_calls: HashMap::new(),
            plugin_registry: HashMap::new(),
        }
    }

    /// Send an IPC message
    pub fn send(&mut self, target: IpcTarget, method: IpcMethod, payload: Option<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let message = IpcMessage {
            id,
            target,
            method,
            payload,
            timestamp: now_secs(),
        };
        self.pending_calls.insert(id, message);
        id
    }

    /// Register a plugin
    pub fn register_plugin(&mut self, info: PluginInfo) {
        self.plugin_registry.insert(info.id.clone(), info);
    }

    /// List plugins
    pub fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.plugin_registry.values().collect()
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, id: &str) -> Result<(), String> {
        let plugin = self
            .plugin_registry
            .get_mut(id)
            .ok_or_else(|| format!("Plugin {} not found", id))?;
        plugin.is_enabled = true;
        Ok(())
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, id: &str) -> Result<(), String> {
        let plugin = self
            .plugin_registry
            .get_mut(id)
            .ok_or_else(|| format!("Plugin {} not found", id))?;
        plugin.is_enabled = false;
        plugin.is_loaded = false;
        Ok(())
    }

    /// Load a plugin
    pub fn load_plugin(&mut self, id: &str) -> Result<(), String> {
        let plugin = self
            .plugin_registry
            .get_mut(id)
            .ok_or_else(|| format!("Plugin {} not found", id))?;
        if !plugin.is_enabled {
            return Err(format!("Plugin {} is not enabled", id));
        }
        plugin.is_loaded = true;
        Ok(())
    }

    /// Unload a plugin
    pub fn unload_plugin(&mut self, id: &str) -> Result<(), String> {
        let plugin = self
            .plugin_registry
            .get_mut(id)
            .ok_or_else(|| format!("Plugin {} not found", id))?;
        plugin.is_loaded = false;
        Ok(())
    }

    /// Get pending call
    pub fn get_pending_call(&self, id: u64) -> Option<&IpcMessage> {
        self.pending_calls.get(&id)
    }

    /// Complete a pending call
    pub fn complete_call(&mut self, id: u64) -> Option<IpcMessage> {
        self.pending_calls.remove(&id)
    }

    /// Pending call count
    pub fn pending_call_count(&self) -> usize {
        self.pending_calls.len()
    }

    /// Plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugin_registry.len()
    }
}

impl Default for IpcManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message() {
        let mut mgr = IpcManager::new();
        let id = mgr.send(
            IpcTarget::Shell,
            IpcMethod::Ping,
            None,
        );
        assert_eq!(id, 1);
        assert_eq!(mgr.pending_call_count(), 1);
    }

    #[test]
    fn test_register_plugin() {
        let mut mgr = IpcManager::new();
        let plugin = PluginInfo {
            id: "weather-widget".to_string(),
            name: "Weather Widget".to_string(),
            kind: "bar-widget".to_string(),
            entry_point: "Weather.qml".to_string(),
            is_enabled: false,
            is_loaded: false,
        };
        mgr.register_plugin(plugin);
        assert_eq!(mgr.plugin_count(), 1);
    }

    #[test]
    fn test_enable_load_plugin() {
        let mut mgr = IpcManager::new();
        mgr.register_plugin(PluginInfo {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            kind: "service".to_string(),
            entry_point: "Test.qml".to_string(),
            is_enabled: false,
            is_loaded: false,
        });
        assert!(mgr.enable_plugin("test-plugin").is_ok());
        assert!(mgr.load_plugin("test-plugin").is_ok());
        let plugin = mgr.plugin_registry.get("test-plugin").unwrap();
        assert!(plugin.is_enabled);
        assert!(plugin.is_loaded);
    }

    #[test]
    fn test_load_disabled_plugin_fails() {
        let mut mgr = IpcManager::new();
        mgr.register_plugin(PluginInfo {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            kind: "service".to_string(),
            entry_point: "Test.qml".to_string(),
            is_enabled: false,
            is_loaded: false,
        });
        assert!(mgr.load_plugin("test-plugin").is_err());
    }

    #[test]
    fn test_complete_call() {
        let mut mgr = IpcManager::new();
        let id = mgr.send(IpcTarget::Shell, IpcMethod::Ping, None);
        assert_eq!(mgr.pending_call_count(), 1);
        assert!(mgr.complete_call(id).is_some());
        assert_eq!(mgr.pending_call_count(), 0);
    }

    #[test]
    fn test_list_plugins() {
        let mut mgr = IpcManager::new();
        mgr.register_plugin(PluginInfo {
            id: "plugin1".to_string(),
            name: "Plugin 1".to_string(),
            kind: "bar-widget".to_string(),
            entry_point: "P1.qml".to_string(),
            is_enabled: true,
            is_loaded: true,
        });
        mgr.register_plugin(PluginInfo {
            id: "plugin2".to_string(),
            name: "Plugin 2".to_string(),
            kind: "panel".to_string(),
            entry_point: "P2.qml".to_string(),
            is_enabled: false,
            is_loaded: false,
        });
        assert_eq!(mgr.list_plugins().len(), 2);
    }
}