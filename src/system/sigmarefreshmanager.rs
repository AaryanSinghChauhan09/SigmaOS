// SPDX-License-Identifier: MIT
// SigmaOS - Refresh Manager
// Config refresh management inspired by Omarchy refresh-* commands and Linux config refresh

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Refresh status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshStatus {
    Pending,
    Refreshing,
    Completed,
    Failed,
}

/// Refresh item
#[derive(Debug, Clone)]
pub struct RefreshItem {
    pub id: String,
    pub default_path: String,
    pub user_path: String,
    pub status: RefreshStatus,
}

/// Refresh manager
pub struct SigmaRefreshManager {
    items: Vec<RefreshItem>,
}

impl SigmaRefreshManager {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: RefreshItem) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &[RefreshItem] {
        &self.items
    }

    pub fn get_item(&self, id: &str) -> Option<&RefreshItem> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn refresh_config(&mut self, id: String) -> Result<(), String> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = RefreshStatus::Refreshing;
            item.status = RefreshStatus::Completed;
            Ok(())
        } else {
            Err(format!("Item {} not found", id))
        }
    }

    pub fn refresh_all(&mut self) {
        for item in &mut self.items {
            item.status = RefreshStatus::Refreshing;
            item.status = RefreshStatus::Completed;
        }
    }

    pub fn get_pending_items(&self) -> Vec<&RefreshItem> {
        self.items
            .iter()
            .filter(|i| i.status == RefreshStatus::Pending)
            .collect()
    }
}

impl Default for SigmaRefreshManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_manager_creation() {
        let manager = SigmaRefreshManager::new();
        assert_eq!(manager.get_items().len(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut manager = SigmaRefreshManager::new();
        let item = RefreshItem {
            id: "hyprland".to_string(),
            default_path: "/config/hypr/hyprland.lua".to_string(),
            user_path: "~/.config/hypr/hyprland.lua".to_string(),
            status: RefreshStatus::Pending,
        };
        manager.add_item(item);
        assert_eq!(manager.get_items().len(), 1);
    }

    #[test]
    fn test_refresh_config() {
        let mut manager = SigmaRefreshManager::new();
        let item = RefreshItem {
            id: "hyprland".to_string(),
            default_path: "/config/hypr/hyprland.lua".to_string(),
            user_path: "~/.config/hypr/hyprland.lua".to_string(),
            status: RefreshStatus::Pending,
        };
        manager.add_item(item);
        manager.refresh_config("hyprland".to_string()).unwrap();
        assert_eq!(manager.get_items()[0].status, RefreshStatus::Completed);
    }

    #[test]
    fn test_refresh_all() {
        let mut manager = SigmaRefreshManager::new();
        let item1 = RefreshItem {
            id: "hyprland".to_string(),
            default_path: "/config/hypr/hyprland.lua".to_string(),
            user_path: "~/.config/hypr/hyprland.lua".to_string(),
            status: RefreshStatus::Pending,
        };
        let item2 = RefreshItem {
            id: "waybar".to_string(),
            default_path: "/config/waybar/config".to_string(),
            user_path: "~/.config/waybar/config".to_string(),
            status: RefreshStatus::Pending,
        };
        manager.add_item(item1);
        manager.add_item(item2);
        manager.refresh_all();
        assert!(manager
            .get_items()
            .iter()
            .all(|i| i.status == RefreshStatus::Completed));
    }

    #[test]
    fn test_pending_items() {
        let mut manager = SigmaRefreshManager::new();
        let item1 = RefreshItem {
            id: "hyprland".to_string(),
            default_path: "/config/hypr/hyprland.lua".to_string(),
            user_path: "~/.config/hypr/hyprland.lua".to_string(),
            status: RefreshStatus::Completed,
        };
        let item2 = RefreshItem {
            id: "waybar".to_string(),
            default_path: "/config/waybar/config".to_string(),
            user_path: "~/.config/waybar/config".to_string(),
            status: RefreshStatus::Pending,
        };
        manager.add_item(item1);
        manager.add_item(item2);
        assert_eq!(manager.get_pending_items().len(), 1);
    }
}
