// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Application execution state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Installed,
    Running,
    Stopped,
    Crashed,
}

/// Application metadata
#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub version: String,
    pub path: String,
    pub state: AppState,
    pub dependencies: Vec<String>,
}

/// Application manager for managing installed applications
/// Inspired by Omarchy's application management and launch- commands
#[derive(Debug, Clone)]
pub struct SigmaApplicationManager {
    applications: Vec<Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        Self {
            applications: Vec::new(),
        }
    }

    /// Register a new application
    pub fn register_application(&mut self, app: Application) {
        self.applications.push(app);
    }

    /// Get application by ID
    pub fn get_application(&self, id: &str) -> Option<&Application> {
        self.applications.iter().find(|a| a.id == id)
    }

    /// Get all applications
    pub fn get_applications(&self) -> &[Application] {
        &self.applications
    }

    /// Launch an application
    pub fn launch_application(&mut self, id: &str) -> Result<String, String> {
        if let Some(app) = self.applications.iter_mut().find(|a| a.id == id) {
            app.state = AppState::Running;
            Ok(format!("Launched application: {}", app.name))
        } else {
            Err(format!("Application not found: {}", id))
        }
    }

    /// Stop an application
    pub fn stop_application(&mut self, id: &str) -> Result<String, String> {
        if let Some(app) = self.applications.iter_mut().find(|a| a.id == id) {
            app.state = AppState::Stopped;
            Ok(format!("Stopped application: {}", app.name))
        } else {
            Err(format!("Application not found: {}", id))
        }
    }

    /// Uninstall an application
    pub fn uninstall_application(&mut self, id: &str) -> Result<String, String> {
        if let Some(pos) = self.applications.iter().position(|a| a.id == id) {
            let app = self.applications.remove(pos);
            Ok(format!("Uninstalled application: {}", app.name))
        } else {
            Err(format!("Application not found: {}", id))
        }
    }

    /// Get running applications
    pub fn get_running_applications(&self) -> Vec<&Application> {
        self.applications
            .iter()
            .filter(|a| a.state == AppState::Running)
            .collect()
    }

    /// Check if application dependencies are satisfied
    pub fn check_dependencies(&self, id: &str) -> Result<bool, String> {
        if let Some(app) = self.get_application(id) {
            let missing: Vec<_> = app
                .dependencies
                .iter()
                .filter(|dep| !self.applications.iter().any(|a| &a.id == *dep))
                .collect();
            if missing.is_empty() {
                Ok(true)
            } else {
                Err(format!("Missing dependencies: {:?}", missing))
            }
        } else {
            Err(format!("Application not found: {}", id))
        }
    }
}

impl Default for SigmaApplicationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_application() {
        let mut manager = SigmaApplicationManager::new();
        let app = Application {
            id: "test-app".to_string(),
            name: "Test App".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/test".to_string(),
            state: AppState::Installed,
            dependencies: Vec::new(),
        };
        manager.register_application(app);
        assert_eq!(manager.get_applications().len(), 1);
    }

    #[test]
    fn test_launch_application() {
        let mut manager = SigmaApplicationManager::new();
        let app = Application {
            id: "test-app".to_string(),
            name: "Test App".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/test".to_string(),
            state: AppState::Installed,
            dependencies: Vec::new(),
        };
        manager.register_application(app);
        manager.launch_application("test-app").unwrap();
        assert_eq!(
            manager.get_application("test-app").unwrap().state,
            AppState::Running
        );
    }

    #[test]
    fn test_stop_application() {
        let mut manager = SigmaApplicationManager::new();
        let app = Application {
            id: "test-app".to_string(),
            name: "Test App".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/test".to_string(),
            state: AppState::Running,
            dependencies: Vec::new(),
        };
        manager.register_application(app);
        manager.stop_application("test-app").unwrap();
        assert_eq!(
            manager.get_application("test-app").unwrap().state,
            AppState::Stopped
        );
    }

    #[test]
    fn test_uninstall_application() {
        let mut manager = SigmaApplicationManager::new();
        let app = Application {
            id: "test-app".to_string(),
            name: "Test App".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/test".to_string(),
            state: AppState::Installed,
            dependencies: Vec::new(),
        };
        manager.register_application(app);
        manager.uninstall_application("test-app").unwrap();
        assert_eq!(manager.get_applications().len(), 0);
    }

    #[test]
    fn test_get_running_applications() {
        let mut manager = SigmaApplicationManager::new();
        let app1 = Application {
            id: "app1".to_string(),
            name: "App 1".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/app1".to_string(),
            state: AppState::Running,
            dependencies: Vec::new(),
        };
        let app2 = Application {
            id: "app2".to_string(),
            name: "App 2".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/app2".to_string(),
            state: AppState::Stopped,
            dependencies: Vec::new(),
        };
        manager.register_application(app1);
        manager.register_application(app2);
        assert_eq!(manager.get_running_applications().len(), 1);
    }

    #[test]
    fn test_check_dependencies() {
        let mut manager = SigmaApplicationManager::new();
        let dep = Application {
            id: "dep".to_string(),
            name: "Dependency".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/dep".to_string(),
            state: AppState::Installed,
            dependencies: Vec::new(),
        };
        let app = Application {
            id: "app".to_string(),
            name: "App".to_string(),
            version: "1.0.0".to_string(),
            path: "/usr/bin/app".to_string(),
            state: AppState::Installed,
            dependencies: vec!["dep".to_string()],
        };
        manager.register_application(dep);
        manager.register_application(app);
        assert!(manager.check_dependencies("app").is_ok());
    }
}
