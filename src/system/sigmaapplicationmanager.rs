// SPDX-License-Identifier: MIT
// SigmaApplicationManager - Application Life Cycle & State Management

#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Stopped,
    Starting,
    Running,
    Paused,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub exec_path: String,
    pub pid: Option<u32>,
    pub state: AppState,
}

impl Application {
    pub fn new(name: &str, exec_path: &str) -> Self {
        Self {
            name: name.to_string(),
            exec_path: exec_path.to_string(),
            pid: None,
            state: AppState::Stopped,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SigmaApplicationManager {
    pub apps: Vec<Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        Self { apps: Vec::new() }
    }

    pub fn register_app(&mut self, app: Application) {
        self.apps.push(app);
    }
}
