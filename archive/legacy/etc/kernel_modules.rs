// SPDX-License-Identifier: MIT
// SigmaOS Kernel Module Manager - Arch Linux Inspired
// Implements kernel module management similar to Arch's modprobe and lsmod

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Module information
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub refcount: u32,
    pub dependencies: Vec<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub srcversion: Option<String>,
    pub vermagic: Option<String>,
    pub alias: Vec<String>,
    pub parameters: BTreeMap<String, String>,
    pub is_loaded: bool,
    pub is_builtin: bool,
    pub is_blacklisted: bool,
}

impl ModuleInfo {
    /// Create a new module info
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            size: 0,
            refcount: 0,
            dependencies: Vec::new(),
            description: None,
            license: None,
            author: None,
            version: None,
            srcversion: None,
            vermagic: None,
            alias: Vec::new(),
            parameters: BTreeMap::new(),
            is_loaded: false,
            is_builtin: false,
            is_blacklisted: false,
        }
    }

    /// Set size
    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    /// Set refcount
    pub fn refcount(mut self, refcount: u32) -> Self {
        self.refcount = refcount;
        self
    }

    /// Add dependency
    pub fn dependency(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set license
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set srcversion
    pub fn srcversion(mut self, srcversion: impl Into<String>) -> Self {
        self.srcversion = Some(srcversion.into());
        self
    }

    /// Set vermagic
    pub fn vermagic(mut self, vermagic: impl Into<String>) -> Self {
        self.vermagic = Some(vermagic.into());
        self
    }

    /// Add alias
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias.push(alias.into());
        self
    }

    /// Set parameter
    pub fn parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Set loaded status
    pub fn loaded(mut self, loaded: bool) -> Self {
        self.is_loaded = loaded;
        self
    }

    /// Set builtin status
    pub fn builtin(mut self, builtin: bool) -> Self {
        self.is_builtin = builtin;
        self
    }

    /// Set blacklisted status
    pub fn blacklisted(mut self, blacklisted: bool) -> Self {
        self.is_blacklisted = blacklisted;
        self
    }

    /// Get size in human readable format
    pub fn get_size_human(&self) -> String {
        if self.size >= 1_000_000 {
            format!("{:.1} MB", self.size as f64 / 1_000_000.0)
        } else if self.size >= 1_000 {
            format!("{:.1} KB", self.size as f64 / 1_000.0)
        } else {
            format!("{} bytes", self.size)
        }
    }

    /// Get module description
    pub fn get_description(&self) -> String {
        self.description.clone().unwrap_or_else(|| "No description".to_string())
    }

    /// Get module author
    pub fn get_author(&self) -> String {
        self.author.clone().unwrap_or_else(|| "Unknown".to_string())
    }

    /// Get module license
    pub fn get_license(&self) -> String {
        self.license.clone().unwrap_or_else(|| "Unknown".to_string())
    }
}

/// Module dependency information
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDependency {
    pub module: String,
    pub depends_on: Vec<String>,
    pub is_loaded: bool,
    pub load_order: Vec<String>,
}

/// Module blacklist
#[derive(Debug, Clone)]
pub struct ModuleBlacklist {
    pub blacklist_file: PathBuf,
    pub blacklisted_modules: Vec<String>,
    pub blacklist_patterns: Vec<String>,
}

impl ModuleBlacklist {
    /// Create a new module blacklist
    pub fn new(blacklist_file: impl Into<PathBuf>) -> Self {
        Self {
            blacklist_file: blacklist_file.into(),
            blacklisted_modules: Vec::new(),
            blacklist_patterns: Vec::new(),
        }
    }

    /// Load blacklist from file
    pub fn load(&mut self) -> io::Result<()> {
        if !self.blacklist_file.exists() {
            return Ok(());
        }

        let file = fs::File::open(&self.blacklist_file)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("blacklist ") {
                let module = line[10..].trim().to_string();
                self.blacklisted_modules.push(module);
            } else if line.starts_with("install ") || line.starts_with("remove ") {
                // These are install/remove commands, not blacklists
                continue;
            } else {
                // Treat as module name
                self.blacklisted_modules.push(line.to_string());
            }
        }

        Ok(())
    }

    /// Save blacklist to file
    pub fn save(&self) -> io::Result<()> {
        let mut content = String::new();
        content.push_str("# SigmaOS Kernel Module Blacklist\n");
        content.push_str("# Generated by SigmaOS Module Manager\n\n");

        for module in &self.blacklisted_modules {
            content.push_str(&format!("blacklist {}\n", module));
        }

        fs::write(&self.blacklist_file, content)
    }

    /// Add module to blacklist
    pub fn add(&mut self, module: impl Into<String>) {
        let module = module.into();
        if !self.blacklisted_modules.contains(&module) {
            self.blacklisted_modules.push(module);
        }
    }

    /// Remove module from blacklist
    pub fn remove(&mut self, module: &str) -> bool {
        if let Some(pos) = self.blacklisted_modules.iter().position(|m| m == module) {
            self.blacklisted_modules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if module is blacklisted
    pub fn is_blacklisted(&self, module: &str) -> bool {
        self.blacklisted_modules.contains(&module.to_string()) ||
        self.blacklist_patterns.iter().any(|pattern| {
            // Simple pattern matching (could be improved with regex)
            module.contains(pattern.as_str())
        })
    }

    /// Add blacklist pattern
    pub fn add_pattern(&mut self, pattern: impl Into<String>) {
        self.blacklist_patterns.push(pattern.into());
    }

    /// Clear blacklist
    pub fn clear(&mut self) {
        self.blacklisted_modules.clear();
        self.blacklist_patterns.clear();
    }

    /// Get blacklisted modules
    pub fn get_blacklisted(&self) -> &[String] {
        &self.blacklisted_modules
    }
}

/// Module parameter information
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleParameter {
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub type_info: Option<String>,
    pub default_value: Option<String>,
    pub is_readonly: bool,
}

impl ModuleParameter {
    /// Create a new module parameter
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            description: None,
            type_info: None,
            default_value: None,
            is_readonly: false,
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set type info
    pub fn type_info(mut self, type_info: impl Into<String>) -> Self {
        self.type_info = Some(type_info.into());
        self
    }

    /// Set default value
    pub fn default_value(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self
    }

    /// Set readonly
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.is_readonly = readonly;
        self
    }
}

/// Kernel module manager
#[derive(Debug, Clone)]
pub struct ModuleManager {
    pub modules_dir: PathBuf,
    pub modules: BTreeMap<String, ModuleInfo>,
    pub loaded_modules: Vec<String>,
    pub blacklist: ModuleBlacklist,
    pub modprobe_conf: PathBuf,
    pub depmod_conf: PathBuf,
    pub modules_dep: PathBuf,
    pub last_scan: Option<u64>,
}

impl ModuleManager {
    /// Create a new module manager
    pub fn new() -> Self {
        Self {
            modules_dir: PathBuf::from("/lib/modules"),
            modules: BTreeMap::new(),
            loaded_modules: Vec::new(),
            blacklist: ModuleBlacklist::new("/etc/modprobe.d/blacklist.conf"),
            modprobe_conf: PathBuf::from("/etc/modprobe.d/modprobe.conf"),
            depmod_conf: PathBuf::from("/etc/depmod.d/depmod.conf"),
            modules_dep: PathBuf::from("/lib/modules/"),
            last_scan: None,
        }
    }

    /// Create module manager with custom paths
    pub fn with_paths(modules_dir: PathBuf, blacklist_file: PathBuf) -> Self {
        Self {
            modules_dir,
            modules: BTreeMap::new(),
            loaded_modules: Vec::new(),
            blacklist: ModuleBlacklist::new(blacklist_file),
            modprobe_conf: PathBuf::from("/etc/modprobe.d/modprobe.conf"),
            depmod_conf: PathBuf::from("/etc/depmod.d/depmod.conf"),
            modules_dep: PathBuf::from("/lib/modules/"),
            last_scan: None,
        }
    }

    /// Scan for available modules
    pub fn scan_modules(&mut self) -> io::Result<()> {
        if !self.modules_dir.exists() {
            return Ok(());
        }

        // Find kernel version directories
        let mut kernel_versions = Vec::new();
        for entry in fs::read_dir(&self.modules_dir)? {
            let entry = entry?;
            let version = entry.file_name().to_string_lossy().to_string();
            if version.starts_with("6.") || version.starts_with("5.") {
                kernel_versions.push(version);
            }
        }

        kernel_versions.sort();
        kernel_versions.reverse(); // Latest first

        // Scan modules in latest kernel version
        if let Some(version) = kernel_versions.first() {
            let kernel_dir = self.modules_dir.join(version);
            self.scan_kernel_modules(&kernel_dir, version)?;
        }

        self.last_scan = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        Ok(())
    }

    /// Scan kernel modules in a directory
    fn scan_kernel_modules(&mut self, kernel_dir: &Path, version: &str) -> io::Result<()> {
        let kernel_path = kernel_dir.join("kernel");
        if !kernel_path.exists() {
            return Ok(());
        }

        self.scan_module_directory(&kernel_path, version)?;
        Ok(())
    }

    /// Scan module directory recursively
    fn scan_module_directory(&mut self, dir: &Path, version: &str) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_module_directory(&path, version)?;
            } else if path.extension().map_or(false, |ext| ext == "ko") {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let module = ModuleInfo::new(name.clone(), path.clone());
                self.modules.insert(name, module);
            }
        }

        Ok(())
    }

    /// Load module
    pub fn load_module(&mut self, module: &str) -> Result<(), String> {
        if self.is_loaded(module) {
            return Ok(()); // Already loaded
        }

        if self.blacklist.is_blacklisted(module) {
            return Err(format!("Module {} is blacklisted", module));
        }

        // Check if module exists
        if !self.modules.contains_key(module) {
            return Err(format!("Module {} not found", module));
        }

        // Check dependencies
        if let Some(module_info) = self.modules.get(module) {
            for dep in &module_info.dependencies {
                if !self.is_loaded(dep) {
                    self.load_module(dep)?;
                }
            }
        }

        // Execute modprobe
        let output = Command::new("modprobe")
            .arg(module)
            .output()
            .map_err(|e| format!("Failed to load module {}: {}", module, e))?;

        if !output.status.success() {
            return Err(format!("modprobe failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        self.loaded_modules.push(module.to_string());
        if let Some(module_info) = self.modules.get_mut(module) {
            module_info.is_loaded = true;
        }

        Ok(())
    }

    /// Unload module
    pub fn unload_module(&mut self, module: &str) -> Result<(), String> {
        if !self.is_loaded(module) {
            return Err(format!("Module {} is not loaded", module));
        }

        // Check if module is in use
        if let Some(module_info) = self.modules.get(module) {
            if module_info.refcount > 0 {
                return Err(format!("Module {} is in use (refcount: {})", module, module_info.refcount));
            }
        }

        // Execute rmmod
        let output = Command::new("rmmod")
            .arg(module)
            .output()
            .map_err(|e| format!("Failed to unload module {}: {}", module, e))?;

        if !output.status.success() {
            return Err(format!("rmmod failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        if let Some(pos) = self.loaded_modules.iter().position(|m| m == module) {
            self.loaded_modules.remove(pos);
        }

        if let Some(module_info) = self.modules.get_mut(module) {
            module_info.is_loaded = false;
        }

        Ok(())
    }

    /// Check if module is loaded
    pub fn is_loaded(&self, module: &str) -> bool {
        self.loaded_modules.contains(&module.to_string())
    }

    /// Get module info
    pub fn get_module(&self, name: &str) -> Option<&ModuleInfo> {
        self.modules.get(name)
    }

    /// Get module mutably
    pub fn get_module_mut(&mut self, name: &str) -> Option<&mut ModuleInfo> {
        self.modules.get_mut(name)
    }

    /// Get all modules
    pub fn get_all_modules(&self) -> Vec<&ModuleInfo> {
        self.modules.values().collect()
    }

    /// Get loaded modules
    pub fn get_loaded_modules(&self) -> Vec<&ModuleInfo> {
        self.modules.values()
            .filter(|m| m.is_loaded)
            .collect()
    }

    /// Get available modules
    pub fn get_available_modules(&self) -> Vec<&ModuleInfo> {
        self.modules.values()
            .filter(|m| !m.is_loaded)
            .collect()
    }

    /// Get blacklisted modules
    pub fn get_blacklisted_modules(&self) -> Vec<&ModuleInfo> {
        self.modules.values()
            .filter(|m| m.is_blacklisted)
            .collect()
    }

    /// Load modules for hardware
    pub fn load_modules_for_hardware(&mut self, hardware: &crate::hardware_detection::HardwareDetector) -> Result<Vec<String>, String> {
        let mut loaded = Vec::new();

        // Load storage modules
        for storage in &hardware.storage_devices {
            if storage.nvme {
                self.load_module("nvme")?;
                loaded.push("nvme".to_string());
            }
            if storage.sata {
                self.load_module("ahci")?;
                loaded.push("ahci".to_string());
            }
            if storage.usb {
                self.load_module("usb_storage")?;
                loaded.push("usb_storage".to_string());
            }
        }

        // Load network modules
        for net in &hardware.network_interfaces {
            if net.wireless {
                self.load_module("cfg80211")?;
                loaded.push("cfg80211".to_string());
            }
        }

        // Load graphics modules
        for gfx in &hardware.graphics_cards {
            if gfx.driver.contains("nvidia") {
                self.load_module("nvidia")?;
                loaded.push("nvidia".to_string());
            } else if gfx.driver.contains("amdgpu") {
                self.load_module("amdgpu")?;
                loaded.push("amdgpu".to_string());
            } else if gfx.driver.contains("i915") {
                self.load_module("i915")?;
                loaded.push("i915".to_string());
            }
        }

        // Load audio modules
        for audio in &hardware.audio_devices {
            self.load_module("snd_hda_intel")?;
            loaded.push("snd_hda_intel".to_string());
        }

        // Load USB modules
        for usb in &hardware.usb_devices {
            self.load_module("usbhid")?;
            loaded.push("usbhid".to_string());
        }

        Ok(loaded)
    }

    /// Resolve module dependencies
    pub fn resolve_dependencies(&self, module: &str) -> Result<Vec<String>, String> {
        let mut deps = Vec::new();
        let mut visited = std::collections::HashSet::new();
        self.resolve_deps_recursive(module, &mut deps, &mut visited)?;
        Ok(deps)
    }

    /// Recursive dependency resolution
    fn resolve_deps_recursive(
        &self,
        module: &str,
        deps: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<(), String> {
        if visited.contains(module) {
            return Ok(());
        }
        visited.insert(module.to_string());

        if let Some(module_info) = self.modules.get(module) {
            for dep in &module_info.dependencies {
                if !visited.contains(dep) {
                    deps.push(dep.clone());
                    self.resolve_deps_recursive(dep, deps, visited)?;
                }
            }
        }

        Ok(())
    }

    /// Get module parameters
    pub fn get_module_parameters(&self, module: &str) -> Option<&BTreeMap<String, String>> {
        self.modules.get(module).map(|m| &m.parameters)
    }

    /// Set module parameter
    pub fn set_module_parameter(&mut self, module: &str, param: &str, value: &str) -> Result<(), String> {
        if let Some(module_info) = self.modules.get_mut(module) {
            module_info.parameters.insert(param.to_string(), value.to_string());
            Ok(())
        } else {
            Err(format!("Module {} not found", module))
        }
    }

    /// Get module information from modinfo
    pub fn get_modinfo(&self, module: &str) -> Result<ModuleInfo, String> {
        let output = Command::new("modinfo")
            .arg(module)
            .output()
            .map_err(|e| format!("Failed to get modinfo for {}: {}", module, e))?;

        if !output.status.success() {
            return Err(format!("modinfo failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut info = ModuleInfo::new(module.to_string(), PathBuf::new());

        for line in stdout.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "filename" => info.path = PathBuf::from(value),
                    "description" => info.description = Some(value.to_string()),
                    "license" => info.license = Some(value.to_string()),
                    "author" => info.author = Some(value.to_string()),
                    "version" => info.version = Some(value.to_string()),
                    "srcversion" => info.srcversion = Some(value.to_string()),
                    "vermagic" => info.vermagic = Some(value.to_string()),
                    "depends" => info.dependencies = value.split(',').map(String::from).collect(),
                    "alias" => info.alias = value.split(',').map(String::from).collect(),
                    _ => {}
                }
            }
        }

        Ok(info)
    }

    /// List modules matching pattern
    pub fn find_modules(&self, pattern: &str) -> Vec<&ModuleInfo> {
        self.modules.values()
            .filter(|m| m.name.contains(pattern))
            .collect()
    }

    /// Get module dependencies
    pub fn get_dependencies(&self, module: &str) -> Vec<String> {
        self.modules.get(module)
            .map(|m| m.dependencies.clone())
            .unwrap_or_default()
    }

    /// Get modules that depend on this module
    pub fn get_dependents(&self, module: &str) -> Vec<&ModuleInfo> {
        self.modules.values()
            .filter(|m| m.dependencies.contains(&module.to_string()))
            .collect()
    }

    /// Check if module can be unloaded
    pub fn can_unload(&self, module: &str) -> bool {
        if !self.is_loaded(module) {
            return false;
        }

        // Check if any loaded modules depend on this one
        let dependents = self.get_dependents(module);
        dependents.iter().all(|m| !m.is_loaded)
    }

    /// Get module load order
    pub fn get_load_order(&self) -> Vec<String> {
        // Simple topological sort based on dependencies
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for module in self.loaded_modules.iter() {
            self.topological_sort(module, &mut visited, &mut order);
        }

        order
    }

    /// Topological sort helper
    fn topological_sort(&self, module: &str, visited: &mut std::collections::HashSet<String>, order: &mut Vec<String>) {
        if visited.contains(module) {
            return;
        }
        visited.insert(module.to_string());

        if let Some(module_info) = self.modules.get(module) {
            for dep in &module_info.dependencies {
                self.topological_sort(dep, visited, order);
            }
        }

        order.push(module.to_string());
    }

    /// Export module configuration
    pub fn export_config(&self) -> String {
        let mut config = String::new();

        config.push_str("# SigmaOS Kernel Module Configuration\n");
        config.push_str("# Generated by SigmaOS Module Manager\n\n");

        config.push_str("## Loaded Modules\n");
        for module in &self.loaded_modules {
            config.push_str(&format!("load={}\n", module));
        }

        config.push_str("\n## Blacklisted Modules\n");
        for module in self.blacklist.get_blacklisted() {
            config.push_str(&format!("blacklist={}\n", module));
        }

        config.push_str("\n## Module Parameters\n");
        for (name, module) in &self.modules {
            if !module.parameters.is_empty() {
                for (param, value) in &module.parameters {
                    config.push_str(&format!("options {} {}={}\n", name, param, value));
                }
            }
        }

        config
    }

    /// Import module configuration
    pub fn import_config(&mut self, content: &str) -> Result<(), String> {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("load=") {
                let module = line[5..].trim().to_string();
                self.load_module(&module)?;
            } else if line.starts_with("blacklist=") {
                let module = line[10..].trim().to_string();
                self.blacklist.add(module);
            } else if line.starts_with("options ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let module = parts[1].to_string();
                    let param_value = parts[2];
                    if let Some((param, value)) = param_value.split_once('=') {
                        self.set_module_parameter(&module, param, value)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Get module statistics
    pub fn get_statistics(&self) -> ModuleStatistics {
        let total = self.modules.len();
        let loaded = self.loaded_modules.len();
        let blacklisted = self.blacklist.get_blacklisted().len();
        let total_size: u64 = self.modules.values().map(|m| m.size).sum();

        ModuleStatistics {
            total,
            loaded,
            blacklisted,
            total_size,
            last_scan: self.last_scan,
        }
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Module statistics
#[derive(Debug, Clone)]
pub struct ModuleStatistics {
    pub total: usize,
    pub loaded: usize,
    pub blacklisted: usize,
    pub total_size: u64,
    pub last_scan: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_info() {
        let module = ModuleInfo::new("test_module".to_string(), PathBuf::from("/lib/modules/test.ko"))
            .size(1024)
            .refcount(1)
            .dependency("dep1")
            .dependency("dep2")
            .description("Test module")
            .license("GPL")
            .author("Test Author");

        assert_eq!(module.name, "test_module");
        assert_eq!(module.size, 1024);
        assert_eq!(module.refcount, 1);
        assert_eq!(module.dependencies.len(), 2);
        assert_eq!(module.get_description(), "Test module");
        assert_eq!(module.get_author(), "Test Author");
        assert_eq!(module.get_license(), "GPL");
    }

    #[test]
    fn test_module_blacklist() {
        let mut blacklist = ModuleBlacklist::new("/tmp/test_blacklist.conf");

        blacklist.add("bad_module");
        blacklist.add("another_bad");
        blacklist.add_pattern("*_test");

        assert!(blacklist.is_blacklisted("bad_module"));
        assert!(blacklist.is_blacklisted("another_bad"));
        assert!(blacklist.is_blacklisted("my_test_module"));
        assert!(!blacklist.is_blacklisted("good_module"));

        assert!(blacklist.remove("bad_module"));
        assert!(!blacklist.is_blacklisted("bad_module"));
    }

    #[test]
    fn test_module_parameter() {
        let param = ModuleParameter::new("debug".to_string(), "1".to_string())
            .description("Enable debug mode")
            .type_info("int")
            .default_value("0")
            .readonly(false);

        assert_eq!(param.name, "debug");
        assert_eq!(param.value, "1");
        assert_eq!(param.get_description().unwrap(), "Enable debug mode");
    }

    #[test]
    fn test_module_manager() {
        let mut manager = ModuleManager::new();

        // Add test module
        let module = ModuleInfo::new("test_module".to_string(), PathBuf::from("/lib/modules/test.ko"))
            .size(2048)
            .dependency("dep1");

        manager.modules.insert("test_module".to_string(), module);

        let module_info = manager.get_module("test_module");
        assert!(module_info.is_some());
        assert_eq!(module_info.unwrap().name, "test_module");
    }

    #[test]
    fn test_module_dependencies() {
        let mut manager = ModuleManager::new();

        let mut module1 = ModuleInfo::new("module1".to_string(), PathBuf::from("/lib/modules/m1.ko"));
        module1.dependencies = vec!["module2".to_string(), "module3".to_string()];

        let mut module2 = ModuleInfo::new("module2".to_string(), PathBuf::from("/lib/modules/m2.ko"));
        module2.dependencies = vec!["module3".to_string()];

        let module3 = ModuleInfo::new("module3".to_string(), PathBuf::from("/lib/modules/m3.ko"));

        manager.modules.insert("module1".to_string(), module1);
        manager.modules.insert("module2".to_string(), module2);
        manager.modules.insert("module3".to_string(), module3);

        let deps = manager.resolve_dependencies("module1").unwrap();
        assert!(deps.contains(&"module2".to_string()));
        assert!(deps.contains(&"module3".to_string()));
    }

    #[test]
    fn test_module_load_order() {
        let mut manager = ModuleManager::new();

        let mut module1 = ModuleInfo::new("module1".to_string(), PathBuf::from("/lib/modules/m1.ko"));
        module1.dependencies = vec!["module2".to_string()];
        module1.is_loaded = true;

        let mut module2 = ModuleInfo::new("module2".to_string(), PathBuf::from("/lib/modules/m2.ko"));
        module2.dependencies = vec!["module3".to_string()];
        module2.is_loaded = true;

        let mut module3 = ModuleInfo::new("module3".to_string(), PathBuf::from("/lib/modules/m3.ko"));
        module3.is_loaded = true;

        manager.modules.insert("module1".to_string(), module1);
        manager.modules.insert("module2".to_string(), module2);
        manager.modules.insert("module3".to_string(), module3);
        manager.loaded_modules = vec!["module1".to_string(), "module2".to_string(), "module3".to_string()];

        let order = manager.get_load_order();
        assert_eq!(order.len(), 3);
        // module3 should come before module2, which should come before module1
        let pos3 = order.iter().position(|m| m == "module3").unwrap();
        let pos2 = order.iter().position(|m| m == "module2").unwrap();
        let pos1 = order.iter().position(|m| m == "module1").unwrap();
        assert!(pos3 < pos2);
        assert!(pos2 < pos1);
    }

    #[test]
    fn test_module_statistics() {
        let mut manager = ModuleManager::new();

        let module1 = ModuleInfo::new("module1".to_string(), PathBuf::from("/lib/modules/m1.ko"))
            .size(1024)
            .loaded(true);

        let module2 = ModuleInfo::new("module2".to_string(), PathBuf::from("/lib/modules/m2.ko"))
            .size(2048)
            .loaded(false);

        manager.modules.insert("module1".to_string(), module1);
        manager.modules.insert("module2".to_string(), module2);
        manager.loaded_modules = vec!["module1".to_string()];

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.loaded, 1);
        assert_eq!(stats.total_size, 3072);
    }

    #[test]
    fn test_module_config_export_import() {
        let mut manager = ModuleManager::new();

        let module = ModuleInfo::new("test_module".to_string(), PathBuf::from("/lib/modules/test.ko"))
            .parameter("debug".to_string(), "1".to_string());

        manager.modules.insert("test_module".to_string(), module);

        let config = manager.export_config();
        assert!(config.contains("test_module"));
        assert!(config.contains("debug=1"));
    }
}
