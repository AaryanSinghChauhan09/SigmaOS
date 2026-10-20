//! Sigma Application Manager
//!
//! Manages system application lifecycle, states, and process launch profiles.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Crashed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub name: String,
    pub binary_path: String,
    pub state: AppState,
    pub pid: Option<u32>,
}

#[derive(Debug, Default)]
pub struct SigmaApplicationManager {
    pub apps: Vec<Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_app(&mut self, app: Application) {
        self.apps.push(app);
    }
}
