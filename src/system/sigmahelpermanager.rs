// SPDX-License-Identifier: MIT
// SigmaOS - Helper Manager
// Helper command management inspired by Omarchy helper commands and Linux helper utilities

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Helper command
#[derive(Debug, Clone)]
pub struct HelperCommand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub usage: String,
}

/// Helper manager
pub struct SigmaHelperManager {
    helpers: Vec<HelperCommand>,
}

impl SigmaHelperManager {
    pub fn new() -> Self {
        Self {
            helpers: Vec::new(),
        }
    }

    pub fn add_helper(&mut self, helper: HelperCommand) {
        self.helpers.push(helper);
    }

    pub fn get_helpers(&self) -> &[HelperCommand] {
        &self.helpers
    }

    pub fn get_helper(&self, id: &str) -> Option<&HelperCommand> {
        self.helpers.iter().find(|h| h.id == id)
    }

    pub fn search_helpers(&self, query: &str) -> Vec<&HelperCommand> {
        self.helpers
            .iter()
            .filter(|h| h.name.contains(query) || h.description.contains(query))
            .collect()
    }
}

impl Default for SigmaHelperManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_manager_creation() {
        let manager = SigmaHelperManager::new();
        assert_eq!(manager.get_helpers().len(), 0);
    }

    #[test]
    fn test_add_helper() {
        let mut manager = SigmaHelperManager::new();
        let helper = HelperCommand {
            id: "pkg-add".to_string(),
            name: "Package Add".to_string(),
            description: "Install packages".to_string(),
            usage: "pkg-add <package>".to_string(),
        };
        manager.add_helper(helper);
        assert_eq!(manager.get_helpers().len(), 1);
    }

    #[test]
    fn test_search_helpers() {
        let mut manager = SigmaHelperManager::new();
        let helper1 = HelperCommand {
            id: "pkg-add".to_string(),
            name: "Package Add".to_string(),
            description: "Install packages".to_string(),
            usage: "pkg-add <package>".to_string(),
        };
        let helper2 = HelperCommand {
            id: "pkg-drop".to_string(),
            name: "Package Drop".to_string(),
            description: "Remove packages".to_string(),
            usage: "pkg-drop <package>".to_string(),
        };
        manager.add_helper(helper1);
        manager.add_helper(helper2);
        assert_eq!(manager.search_helpers("add").len(), 1);
    }
}
