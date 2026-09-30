// SigmaOS Sovereign Plugin System Framework
// Zero-dependency #![no_std] capability-backed sandboxed plugin lifecycle & IPC framework

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginCapability {
    FileSystemAccess,
    NetworkAccess,
    HardwareAccess,
    SystemAdmin,
    AiInference,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub required_capabilities: Vec<PluginCapability>,
    pub entry_point: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginSandboxedContext {
    pub plugin_id: String,
    pub granted_capabilities: Vec<PluginCapability>,
    pub memory_limit_mb: usize,
    pub is_active: bool,
}

pub struct SovereignPluginFramework {
    loaded_plugins: Vec<(PluginManifest, PluginSandboxedContext)>,
    max_active_plugins: usize,
}

impl SovereignPluginFramework {
    pub fn new(max_active_plugins: usize) -> Self {
        Self {
            loaded_plugins: Vec::new(),
            max_active_plugins,
        }
    }

    pub fn load_plugin(
        &mut self,
        manifest: PluginManifest,
        granted_capabilities: Vec<PluginCapability>,
        memory_limit_mb: usize,
    ) -> Result<String, &'static str> {
        if self.loaded_plugins.len() >= self.max_active_plugins {
            return Err("Max active plugin capacity reached");
        }

        // Verify required capabilities are all granted
        for req in &manifest.required_capabilities {
            if !granted_capabilities.contains(req) {
                return Err("Missing required capability permission for plugin");
            }
        }

        let plugin_id = manifest.id.clone();
        let context = PluginSandboxedContext {
            plugin_id: plugin_id.clone(),
            granted_capabilities,
            memory_limit_mb,
            is_active: true,
        };

        self.loaded_plugins.push((manifest, context));
        Ok(plugin_id)
    }

    pub fn unload_plugin(&mut self, plugin_id: &str) -> bool {
        if let Some(pos) = self.loaded_plugins.iter().position(|(m, _)| m.id == plugin_id) {
            self.loaded_plugins.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn verify_capability(&self, plugin_id: &str, cap: &PluginCapability) -> bool {
        if let Some((_, ctx)) = self.loaded_plugins.iter().find(|(m, _)| m.id == plugin_id) {
            ctx.is_active && ctx.granted_capabilities.contains(cap)
        } else {
            false
        }
    }

    pub fn execute_plugin_ipc(&self, plugin_id: &str, action: &str, payload: &str) -> Result<String, &'static str> {
        let (_, ctx) = self
            .loaded_plugins
            .iter()
            .find(|(m, _)| m.id == plugin_id)
            .ok_or("Plugin not found")?;

        if !ctx.is_active {
            return Err("Plugin is inactive");
        }

        Ok(format!("ipc_ack_{}_{}_{}", plugin_id, action, payload.len()))
    }

    pub fn loaded_plugins(&self) -> Vec<&PluginManifest> {
        self.loaded_plugins.iter().map(|(m, _)| m).collect()
    }

    pub fn active_plugins_count(&self) -> usize {
        self.loaded_plugins.iter().filter(|(_, c)| c.is_active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_plugin_framework() {
        let mut framework = SovereignPluginFramework::new(10);
        let manifest = PluginManifest {
            id: String::from("plugin_system_monitor"),
            name: String::from("System Monitor Plugin"),
            version: String::from("1.0.0"),
            author: String::from("SigmaOS Team"),
            description: String::from("Monitors CPU and RAM usage"),
            required_capabilities: alloc::vec![PluginCapability::FileSystemAccess],
            entry_point: String::from("main.wasm"),
        };

        let loaded_id = framework
            .load_plugin(
                manifest,
                alloc::vec![PluginCapability::FileSystemAccess],
                128,
            )
            .unwrap();

        assert_eq!(framework.active_plugins_count(), 1);
        assert!(framework.verify_capability(&loaded_id, &PluginCapability::FileSystemAccess));
        assert!(!framework.verify_capability(&loaded_id, &PluginCapability::NetworkAccess));

        let ipc_res = framework.execute_plugin_ipc(&loaded_id, "query_metrics", "all");
        assert!(ipc_res.is_ok());

        assert!(framework.unload_plugin(&loaded_id));
        assert_eq!(framework.active_plugins_count(), 0);
    }
}
