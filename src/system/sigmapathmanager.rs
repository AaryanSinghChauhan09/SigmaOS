// SPDX-License-Identifier: MIT
// SigmaOS - Path Manager
// Path management inspired by Omarchy $OMARCHY_PATH and Linux path systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Path type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathType {
    Config,
    Data,
    Cache,
    Runtime,
}

/// Path entry
#[derive(Debug, Clone)]
pub struct PathEntry {
    pub path_type: PathType,
    pub path: String,
}

/// Path manager
pub struct SigmaPathManager {
    base_path: String,
    paths: Vec<PathEntry>,
}

impl SigmaPathManager {
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            paths: Vec::new(),
        }
    }

    pub fn set_base_path(&mut self, path: String) {
        self.base_path = path;
    }

    pub fn get_base_path(&self) -> &str {
        &self.base_path
    }

    pub fn add_path(&mut self, path_type: PathType, path: String) {
        self.paths.push(PathEntry { path_type, path });
    }

    pub fn get_path(&self, path_type: PathType) -> Option<&String> {
        self.paths
            .iter()
            .find(|p| p.path_type == path_type)
            .map(|p| &p.path)
    }

    pub fn resolve_path(&self, relative_path: &str) -> String {
        alloc::format!("{}/{}", self.base_path, relative_path)
    }
}

impl Default for SigmaPathManager {
    fn default() -> Self {
        Self::new("/opt/sigmaos".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_manager_creation() {
        let manager = SigmaPathManager::new("/opt/sigmaos".to_string());
        assert_eq!(manager.get_base_path(), "/opt/sigmaos");
    }

    #[test]
    fn test_set_base_path() {
        let mut manager = SigmaPathManager::new("/opt/sigmaos".to_string());
        manager.set_base_path("/usr/local/sigmaos".to_string());
        assert_eq!(manager.get_base_path(), "/usr/local/sigmaos");
    }

    #[test]
    fn test_add_path() {
        let mut manager = SigmaPathManager::new("/opt/sigmaos".to_string());
        manager.add_path(PathType::Config, "config".to_string());
        assert!(manager.get_path(PathType::Config).is_some());
    }

    #[test]
    fn test_resolve_path() {
        let manager = SigmaPathManager::new("/opt/sigmaos".to_string());
        let resolved = manager.resolve_path("config/test");
        assert_eq!(resolved, "/opt/sigmaos/config/test");
    }
}
