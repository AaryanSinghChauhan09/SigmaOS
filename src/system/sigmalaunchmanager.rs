// SPDX-License-Identifier: MIT
// SigmaOS - Launch Manager
// Application launching inspired by Omarchy launch-* commands and Linux launch systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Launch priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaunchPriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// Application
#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub command: String,
    pub icon: Option<String>,
    pub priority: LaunchPriority,
}

/// Launch manager
pub struct SigmaLaunchManager {
    applications: Vec<Application>,
}

impl SigmaLaunchManager {
    pub fn new() -> Self {
        Self {
            applications: Vec::new(),
        }
    }

    pub fn add_application(&mut self, app: Application) {
        self.applications.push(app);
    }

    pub fn get_applications(&self) -> &[Application] {
        &self.applications
    }

    pub fn get_application(&self, id: &str) -> Option<&Application> {
        self.applications.iter().find(|a| a.id == id)
    }

    pub fn launch_application(&self, id: String) -> Result<(), String> {
        if self.get_application(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Application {} not found", id))
        }
    }

    pub fn search_applications(&self, query: &str) -> Vec<&Application> {
        self.applications
            .iter()
            .filter(|a| a.name.contains(query) || a.command.contains(query))
            .collect()
    }

    pub fn get_high_priority_apps(&self) -> Vec<&Application> {
        self.applications
            .iter()
            .filter(|a| a.priority == LaunchPriority::High)
            .collect()
    }
}

impl Default for SigmaLaunchManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_manager_creation() {
        let manager = SigmaLaunchManager::new();
        assert_eq!(manager.get_applications().len(), 0);
    }

    #[test]
    fn test_add_application() {
        let mut manager = SigmaLaunchManager::new();
        let app = Application {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            command: "firefox".to_string(),
            icon: Some("firefox".to_string()),
            priority: LaunchPriority::Normal,
        };
        manager.add_application(app);
        assert_eq!(manager.get_applications().len(), 1);
    }

    #[test]
    fn test_launch_application() {
        let mut manager = SigmaLaunchManager::new();
        let app = Application {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            command: "firefox".to_string(),
            icon: Some("firefox".to_string()),
            priority: LaunchPriority::Normal,
        };
        manager.add_application(app);
        assert!(manager.launch_application("firefox".to_string()).is_ok());
    }

    #[test]
    fn test_search_applications() {
        let mut manager = SigmaLaunchManager::new();
        let app1 = Application {
            id: "firefox".to_string(),
            name: "Firefox".to_string(),
            command: "firefox".to_string(),
            icon: Some("firefox".to_string()),
            priority: LaunchPriority::Normal,
        };
        let app2 = Application {
            id: "chrome".to_string(),
            name: "Chrome".to_string(),
            command: "chrome".to_string(),
            icon: Some("chrome".to_string()),
            priority: LaunchPriority::Normal,
        };
        manager.add_application(app1);
        manager.add_application(app2);
        assert_eq!(manager.search_applications("fire").len(), 1);
    }

    #[test]
    fn test_high_priority_apps() {
        let mut manager = SigmaLaunchManager::new();
        let app1 = Application {
            id: "terminal".to_string(),
            name: "Terminal".to_string(),
            command: "terminal".to_string(),
            icon: Some("terminal".to_string()),
            priority: LaunchPriority::High,
        };
        let app2 = Application {
            id: "text-editor".to_string(),
            name: "Text Editor".to_string(),
            command: "editor".to_string(),
            icon: Some("editor".to_string()),
            priority: LaunchPriority::Normal,
        };
        manager.add_application(app1);
        manager.add_application(app2);
        assert_eq!(manager.get_high_priority_apps().len(), 1);
    }
}
