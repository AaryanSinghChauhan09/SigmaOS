// SPDX-License-Identifier: MIT
// SigmaOS Application Manager Engine (`src/system/sigmaapplicationmanager.rs`)

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::String;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::String;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Stopped,
    Starting,
    Running,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub exec_path: String,
    pub state: AppState,
}

#[derive(Debug, Default)]
pub struct SigmaApplicationManager {
    pub apps: BTreeMap<String, Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_app(&mut self, name: &str, exec_path: &str) {
        self.apps.insert(
            name.to_string(),
            Application {
                name: name.to_string(),
                exec_path: exec_path.to_string(),
                state: AppState::Stopped,
            },
        );
    }
}
