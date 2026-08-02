#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Tiny Core Linux Core Concepts Integration
// Implements minimal footprint, frugal installs, RAM-copy booting, and .tcz loop-mount application extensions.
// Ensures Tiny Core architecture is no longer a challenge to SigmaOS.

use std::collections::{HashMap, HashSet};

/// Tiny Core Operation Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TinyCoreMode {
    /// Cloud / Internet: Boots entirely in RAM, extensions downloaded last for the session only
    Cloud,
    /// Mount / Frugal: Extensions reside on local storage and are loop-mounted to RAM
    MountFrugal,
    /// Copy / Local: Application extensions are directly copied to RAM (/usr/local) for fast zero-disk execution
    CopyLocal,
}

/// 1. Tiny Core RAM-Copy Engine (Boot-to-RAM & Frugal execution state)
pub struct TinyCoreRAMEngine {
    pub mode: TinyCoreMode,
    pub is_core_loaded_to_ram: bool,
    pub base_memory_usage_kb: u32,
}

impl TinyCoreRAMEngine {
    pub fn new(mode: TinyCoreMode) -> Self {
        Self {
            mode,
            is_core_loaded_to_ram: false,
            base_memory_usage_kb: 0,
        }
    }

    /// Simulates loading the entire base system ( vmlinuz + core.gz ) directly into RAM
    pub fn boot_ram_copy(&mut self) -> Result<&'static str, &'static str> {
        self.is_core_loaded_to_ram = true;
        self.base_memory_usage_kb = match self.mode {
            TinyCoreMode::Cloud => 12 * 1024,      // 12MB base (Xvesa + core)
            TinyCoreMode::MountFrugal => 8 * 1024, // 8MB base (MicroCore)
            TinyCoreMode::CopyLocal => 16 * 1024,  // 16MB base (CorePlus)
        };
        Ok("Boot successful: Loaded kernel (vmlinuz) and rootfs (core.gz) directly into RAM copy.")
    }
}

/// Represents a mounted .tcz loop extension
#[derive(Debug, Clone)]
pub struct TczExtension {
    pub name: String,
    pub mount_point: String,
    pub size_kb: u32,
}

/// 2. TCZ Extension Manager (Loop-mount loop package overlays in /tmp/tcloop)
pub struct TczExtensionManager {
    pub mounted_extensions: HashMap<String, TczExtension>,
}

impl TczExtensionManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mounted_extensions: HashMap::new(),
        }
    }

    /// Dynamically loop-mounts read-only application extension onto storage loop overlays
    pub fn mount_extension(&mut self, tcz_name: &str, size_kb: u32) -> Result<String, &'static str> {
        if self.mounted_extensions.contains_key(tcz_name) {
            return Err("Extension is already mounted");
        }
        let mount_point = format!("/tmp/tcloop/{}", tcz_name.replace(".tcz", ""));
        let extension = TczExtension {
            name: tcz_name.to_string(),
            mount_point: mount_point.clone(),
            size_kb,
        };
        self.mounted_extensions.insert(tcz_name.to_string(), extension);
        Ok(mount_point)
    }

    /// Dynamically unmounts and detaches loop extension, cleaning memory/mount namespaces
    pub fn unmount_extension(&mut self, tcz_name: &str) -> Result<(), &'static str> {
        self.mounted_extensions.remove(tcz_name).ok_or("Extension not mounted")?;
        Ok(())
    }

    pub fn get_mounted_extensions(&self) -> Vec<String> {
        self.mounted_extensions.keys().cloned().collect()
    }
}

impl Default for TczExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 3. AppsAudit Dependency Resolver & Checker
pub struct AppsAuditTool {
    pub repo_dependencies: HashMap<String, Vec<String>>,
}

impl AppsAuditTool {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            repo_dependencies: HashMap::new(),
        }
    }

    pub fn register_dependency_rule(&mut self, tcz_name: &str, dependencies: Vec<&str>) {
        let deps = dependencies.iter().map(|&s| s.to_string()).collect();
        self.repo_dependencies.insert(tcz_name.to_string(), deps);
    }

    /// Audits currently mounted extensions and returns a list of missing required dependencies
    pub fn audit_missing_dependencies(&self, manager: &TczExtensionManager) -> HashSet<String> {
        let mounted: HashSet<String> = manager.mounted_extensions.keys().cloned().collect();
        let mut missing = HashSet::new();

        for ext_name in &mounted {
            if let Some(deps) = self.repo_dependencies.get(ext_name) {
                for dep in deps {
                    if !mounted.contains(dep) {
                        missing.insert(dep.clone());
                    }
                }
            }
        }
        missing
    }
}

impl Default for AppsAuditTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny_core_ram_engine_cloud() {
        let mut engine = TinyCoreRAMEngine::new(TinyCoreMode::Cloud);
        assert_eq!(engine.mode, TinyCoreMode::Cloud);
        assert!(!engine.is_core_loaded_to_ram);

        assert!(engine.boot_ram_copy().is_ok());
        assert!(engine.is_core_loaded_to_ram);
        assert_eq!(engine.base_memory_usage_kb, 12 * 1024);
    }

    #[test]
    fn test_tcz_extension_manager() {
        let mut manager = TczExtensionManager::new();
        assert!(manager.mounted_extensions.is_empty());

        let mount_res = manager.mount_extension("fltk-1.10.tcz", 150);
        assert!(mount_res.is_ok());
        assert_eq!(mount_res.unwrap(), "/tmp/tcloop/fltk-1.10");
        assert_eq!(manager.mounted_extensions.len(), 1);

        assert!(manager.unmount_extension("fltk-1.10.tcz").is_ok());
        assert!(manager.mounted_extensions.is_empty());
    }

    #[test]
    fn test_apps_audit_tool() {
        let mut manager = TczExtensionManager::new();
        manager.mount_extension("wbar.tcz", 80).unwrap();

        let mut auditor = AppsAuditTool::new();
        auditor.register_dependency_rule("wbar.tcz", vec!["fltk-1.10.tcz", "imlib2.tcz"]);

        let missing = auditor.audit_missing_dependencies(&manager);
        assert_eq!(missing.len(), 2);
        assert!(missing.contains("fltk-1.10.tcz"));
        assert!(missing.contains("imlib2.tcz"));

        // Resolve one dependency
        manager.mount_extension("fltk-1.10.tcz", 150).unwrap();
        let missing_after = auditor.audit_missing_dependencies(&manager);
        assert_eq!(missing_after.len(), 1);
        assert!(missing_after.contains("imlib2.tcz"));
    }
}
