// SPDX-License-Identifier: GPL-2.0-or-later
// FuryBSD ZFS Boot Environment manager for SigmaOS.
// Manages creation, activation, deletion, and rollback of
// ZFS boot environments (BEs) to support safe system updates.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents a single ZFS boot environment.
#[derive(Debug, Clone)]
pub struct BootEnvironment {
    pub name: String,
    pub dataset: String,
    pub active: bool,
    pub created_at: u64,
}

impl BootEnvironment {
    pub fn new(name: &str, dataset: &str, created_at: u64) -> Self {
        Self {
            name: name.to_string(),
            dataset: dataset.to_string(),
            active: false,
            created_at,
        }
    }
}

/// Operations that can be performed on a boot environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeOperation {
    Create,
    Activate,
    Delete,
    Rollback,
}

/// Manages the collection of ZFS boot environments.
pub struct ZfsBeManager {
    pub environments: Vec<BootEnvironment>,
    pub active_name: Option<String>,
}

impl ZfsBeManager {
    pub fn new() -> Self {
        Self {
            environments: Vec::new(),
            active_name: None,
        }
    }

    pub fn create_be(
        &mut self,
        name: &str,
        dataset: &str,
        created_at: u64,
    ) -> Result<(), &'static str> {
        if self.environments.iter().any(|e| e.name == name) {
            return Err("Boot environment already exists");
        }
        self.environments
            .push(BootEnvironment::new(name, dataset, created_at));
        Ok(())
    }

    pub fn activate_be(&mut self, name: &str) -> Result<(), &'static str> {
        let exists = self.environments.iter().any(|e| e.name == name);
        if !exists {
            return Err("Boot environment not found");
        }
        for e in &mut self.environments {
            e.active = e.name == name;
        }
        self.active_name = Some(name.to_string());
        Ok(())
    }

    pub fn delete_be(&mut self, name: &str) -> Result<(), &'static str> {
        if self.active_name.as_deref() == Some(name) {
            return Err("Cannot delete the active boot environment");
        }
        let before = self.environments.len();
        self.environments.retain(|e| e.name != name);
        if self.environments.len() == before {
            return Err("Boot environment not found");
        }
        Ok(())
    }

    pub fn list_bes(&self) -> Vec<&BootEnvironment> {
        self.environments.iter().collect()
    }

    pub fn rollback_to_be(&mut self, name: &str) -> Result<(), &'static str> {
        self.activate_be(name)
    }
}

impl Default for ZfsBeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_activate_be() {
        let mut mgr = ZfsBeManager::new();
        mgr.create_be("default", "zroot/ROOT/default", 1000000)
            .unwrap();
        mgr.create_be("upgrade-2024", "zroot/ROOT/upgrade-2024", 1000100)
            .unwrap();

        mgr.activate_be("default").unwrap();
        assert_eq!(mgr.active_name.as_deref(), Some("default"));

        mgr.activate_be("upgrade-2024").unwrap();
        assert_eq!(mgr.active_name.as_deref(), Some("upgrade-2024"));
        assert!(
            mgr.environments
                .iter()
                .find(|e| e.name == "upgrade-2024")
                .unwrap()
                .active
        );
        assert!(
            !mgr.environments
                .iter()
                .find(|e| e.name == "default")
                .unwrap()
                .active
        );
    }

    #[test]
    fn test_cannot_delete_active_be() {
        let mut mgr = ZfsBeManager::new();
        mgr.create_be("base", "zroot/ROOT/base", 0).unwrap();
        mgr.activate_be("base").unwrap();
        assert!(mgr.delete_be("base").is_err());
    }

    #[test]
    fn test_rollback() {
        let mut mgr = ZfsBeManager::new();
        mgr.create_be("old", "zroot/ROOT/old", 0).unwrap();
        mgr.create_be("new", "zroot/ROOT/new", 1).unwrap();
        mgr.activate_be("new").unwrap();
        mgr.rollback_to_be("old").unwrap();
        assert_eq!(mgr.active_name.as_deref(), Some("old"));
    }
}
