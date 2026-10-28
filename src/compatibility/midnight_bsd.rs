// SPDX-License-Identifier: GPL-2.0-or-later
// MidnightBSD compatibility layer for SigmaOS.
// Provides mports package engine and GNUstep bridge for
// MidnightBSD's native packaging and application runtime.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A mport package descriptor.
#[derive(Debug, Clone)]
pub struct MPort {
    pub name: String,
    pub version: String,
    pub deps: Vec<String>,
}

impl MPort {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            deps: Vec::new(),
        }
    }

    pub fn with_dep(mut self, dep: &str) -> Self {
        self.deps.push(dep.to_string());
        self
    }
}

/// The mports package management engine.
pub struct MPortsEngine {
    installed: BTreeMap<String, MPort>,
}

impl MPortsEngine {
    pub fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
        }
    }

    pub fn install(&mut self, port: MPort) -> Result<(), &'static str> {
        if self.installed.contains_key(&port.name) {
            return Err("Package already installed");
        }
        self.installed.insert(port.name.clone(), port);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<MPort, &'static str> {
        self.installed.remove(name).ok_or("Package not installed")
    }

    pub fn upgrade(&mut self, name: &str, new_version: &str) -> Result<(), &'static str> {
        let pkg = self.installed.get_mut(name).ok_or("Package not installed")?;
        pkg.version = new_version.to_string();
        Ok(())
    }

    pub fn list_installed(&self) -> Vec<&MPort> {
        self.installed.values().collect()
    }

    pub fn search(&self, query: &str) -> Vec<&MPort> {
        self.installed
            .values()
            .filter(|p| p.name.contains(query))
            .collect()
    }
}

impl Default for MPortsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// GNUstep application runtime bridge.
pub struct GnuStepBridge {
    pub bundle_path: String,
    pub defaults_initialized: bool,
}

impl GnuStepBridge {
    pub fn new(bundle_path: &str) -> Self {
        Self {
            bundle_path: bundle_path.to_string(),
            defaults_initialized: false,
        }
    }

    pub fn initialize_defaults_system(&mut self) -> Result<(), &'static str> {
        if self.bundle_path.is_empty() {
            return Err("Bundle path is empty");
        }
        self.defaults_initialized = true;
        Ok(())
    }

    pub fn run_app_kit(&self) -> Result<String, &'static str> {
        if !self.defaults_initialized {
            return Err("Defaults system not initialized");
        }
        Ok(alloc::format!("AppKit running from: {}", self.bundle_path))
    }

    pub fn bundle_path(&self) -> &str {
        &self.bundle_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_can_be_added_to_engine() {
        let mut engine = MPortsEngine::new();
        let pkg = MPort::new("vim", "9.0").with_dep("ncurses");
        assert!(engine.install(pkg).is_ok());
        assert_eq!(engine.list_installed().len(), 1);
    }

    #[test]
    fn test_duplicate_install_fails() {
        let mut engine = MPortsEngine::new();
        engine.install(MPort::new("bash", "5.2")).unwrap();
        assert!(engine.install(MPort::new("bash", "5.2")).is_err());
    }

    #[test]
    fn test_gnustep_bridge() {
        let mut bridge = GnuStepBridge::new("/usr/local/GNUstep");
        assert!(bridge.run_app_kit().is_err());
        bridge.initialize_defaults_system().unwrap();
        assert!(bridge.run_app_kit().is_ok());
    }

    #[test]
    fn test_search_and_remove() {
        let mut engine = MPortsEngine::new();
        engine.install(MPort::new("vim", "9.0")).unwrap();
        engine.install(MPort::new("vim-nox", "9.0")).unwrap();
        assert_eq!(engine.search("vim").len(), 2);
        engine.remove("vim").unwrap();
        assert_eq!(engine.list_installed().len(), 1);
    }
}
