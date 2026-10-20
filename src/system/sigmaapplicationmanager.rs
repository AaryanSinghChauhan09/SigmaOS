// SPDX-License-Identifier: MIT
// SigmaOS Application Manager
// (`src/system/sigmaapplicationmanager.rs`)

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Stopped,
    Starting,
    Running,
    Paused,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub exec_path: String,
    pub state: AppState,
    pub auto_start: bool,
}

pub struct SigmaApplicationManager {
    pub applications: BTreeMap<String, Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        let mut apps = BTreeMap::new();
        apps.insert(
            "org.sigmaos.terminal".to_string(),
            Application {
                id: "org.sigmaos.terminal".to_string(),
                name: "Sigma Terminal".to_string(),
                exec_path: "/usr/bin/sigma-terminal".to_string(),
                state: AppState::Stopped,
                auto_start: true,
            },
        );
        Self { applications: apps }
    }

    pub fn register_app(&mut self, app: Application) {
        self.applications.insert(app.id.clone(), app);
    }

    pub fn start_app(&mut self, id: &str) -> bool {
        if let Some(app) = self.applications.get_mut(id) {
            app.state = AppState::Running;
            true
        } else {
            false
        }
    }

    pub fn stop_app(&mut self, id: &str) -> bool {
        if let Some(app) = self.applications.get_mut(id) {
            app.state = AppState::Stopped;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_manager() {
        let mut manager = SigmaApplicationManager::new();
        assert!(manager.start_app("org.sigmaos.terminal"));
        assert_eq!(
            manager.applications.get("org.sigmaos.terminal").unwrap().state,
            AppState::Running
        );
        assert!(manager.stop_app("org.sigmaos.terminal"));
        assert_eq!(
            manager.applications.get("org.sigmaos.terminal").unwrap().state,
            AppState::Stopped
        );
    }
}
