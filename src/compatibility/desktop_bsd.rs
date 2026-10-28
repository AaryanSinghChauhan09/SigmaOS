// SPDX-License-Identifier: GPL-2.0-or-later
// DesktopBSD user permission delegation layer for SigmaOS.
// Bridges Polkit action registration, temporary grants, and
// pkgng GUI package operations for desktop users.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A Polkit-style action descriptor.
#[derive(Debug, Clone)]
pub struct PolkitAction {
    pub id: String,
    pub description: String,
    pub auth_required: bool,
}

impl PolkitAction {
    pub fn new(id: &str, description: &str, auth_required: bool) -> Self {
        Self {
            id: id.to_string(),
            description: description.to_string(),
            auth_required,
        }
    }
}

/// Manages Polkit action registry and temporary permission grants.
pub struct PermissionDelegate {
    pub actions: BTreeMap<String, PolkitAction>,
    pub temp_grants: Vec<String>,
}

impl PermissionDelegate {
    pub fn new() -> Self {
        Self {
            actions: BTreeMap::new(),
            temp_grants: Vec::new(),
        }
    }

    pub fn register_action(&mut self, action: PolkitAction) {
        self.actions.insert(action.id.clone(), action);
    }

    /// Returns true if the action is allowed (either no auth required, or temporarily granted).
    pub fn check_permission(&self, action_id: &str) -> bool {
        match self.actions.get(action_id) {
            None => false,
            Some(a) => !a.auth_required || self.temp_grants.contains(&a.id),
        }
    }

    /// Grant temporary permission for an action (e.g., authenticated via GUI dialog).
    pub fn grant_temporary(&mut self, action_id: &str) -> Result<(), &'static str> {
        if !self.actions.contains_key(action_id) {
            return Err("Action not registered");
        }
        if !self.temp_grants.iter().any(|g| g == action_id) {
            self.temp_grants.push(action_id.to_string());
        }
        Ok(())
    }
}

impl Default for PermissionDelegate {
    fn default() -> Self {
        Self::new()
    }
}

/// GUI bridge for pkgng (pkg) operations.
pub struct PkgngGuiBridge {
    pub installed: Vec<String>,
    pub catalog: Vec<String>,
}

impl PkgngGuiBridge {
    pub fn new() -> Self {
        Self {
            installed: Vec::new(),
            catalog: Vec::new(),
        }
    }

    pub fn update_catalog(&mut self, packages: Vec<String>) {
        self.catalog = packages;
    }

    pub fn install_package(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.catalog.iter().any(|p| p == name) {
            return Err("Package not in catalog");
        }
        if self.installed.iter().any(|p| p == name) {
            return Err("Already installed");
        }
        self.installed.push(name.to_string());
        Ok(())
    }

    pub fn remove_package(&mut self, name: &str) -> Result<(), &'static str> {
        let before = self.installed.len();
        self.installed.retain(|p| p != name);
        if self.installed.len() == before {
            return Err("Package not installed");
        }
        Ok(())
    }
}

impl Default for PkgngGuiBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_delegation() {
        let mut delegate = PermissionDelegate::new();
        let action = PolkitAction::new("org.desktopbsd.install-pkg", "Install package", true);
        delegate.register_action(action);

        // Auth required → not allowed initially
        assert!(!delegate.check_permission("org.desktopbsd.install-pkg"));
        // Grant and check
        delegate
            .grant_temporary("org.desktopbsd.install-pkg")
            .unwrap();
        assert!(delegate.check_permission("org.desktopbsd.install-pkg"));
    }

    #[test]
    fn test_pkgng_gui_install_remove() {
        let mut bridge = PkgngGuiBridge::new();
        bridge.update_catalog(alloc::vec!["firefox".to_string(), "vim".to_string()]);
        assert!(bridge.install_package("firefox").is_ok());
        assert!(bridge.install_package("firefox").is_err()); // already installed
        assert!(bridge.remove_package("firefox").is_ok());
        assert!(bridge.remove_package("firefox").is_err()); // not installed
    }

    #[test]
    fn test_no_auth_action_always_allowed() {
        let mut delegate = PermissionDelegate::new();
        delegate.register_action(PolkitAction::new(
            "org.desktopbsd.read-log",
            "View log",
            false,
        ));
        assert!(delegate.check_permission("org.desktopbsd.read-log"));
    }
}
