// SPDX-License-Identifier: MIT
// SigmaOS - Alias Manager
// Alias management inspired by Omarchy menu aliases and Linux alias systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Alias
#[derive(Debug, Clone)]
pub struct Alias {
    pub id: String,
    pub name: String,
    pub target: String,
}

/// Alias manager
pub struct SigmaAliasManager {
    aliases: Vec<Alias>,
}

impl SigmaAliasManager {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
        }
    }

    pub fn add_alias(&mut self, alias: Alias) {
        self.aliases.push(alias);
    }

    pub fn get_aliases(&self) -> &[Alias] {
        &self.aliases
    }

    pub fn get_alias(&self, name: &str) -> Option<&Alias> {
        self.aliases.iter().find(|a| a.name == name)
    }

    pub fn resolve_alias(&self, name: &str) -> Option<&String> {
        self.get_alias(name).map(|a| &a.target)
    }
}

impl Default for SigmaAliasManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_manager_creation() {
        let manager = SigmaAliasManager::new();
        assert_eq!(manager.get_aliases().len(), 0);
    }

    #[test]
    fn test_add_alias() {
        let mut manager = SigmaAliasManager::new();
        let alias = Alias {
            id: "term".to_string(),
            name: "terminal".to_string(),
            target: "kitty".to_string(),
        };
        manager.add_alias(alias);
        assert_eq!(manager.get_aliases().len(), 1);
    }

    #[test]
    fn test_resolve_alias() {
        let mut manager = SigmaAliasManager::new();
        let alias = Alias {
            id: "term".to_string(),
            name: "terminal".to_string(),
            target: "kitty".to_string(),
        };
        manager.add_alias(alias);
        assert_eq!(
            manager.resolve_alias("terminal"),
            Some(&"kitty".to_string())
        );
    }
}
