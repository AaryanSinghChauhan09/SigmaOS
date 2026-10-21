// SPDX-License-Identifier: MIT
// SigmaOS Application Manager

#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub state: AppState,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaApplicationManager {
    pub apps: Vec<Application>,
}

impl SigmaApplicationManager {
    pub fn new() -> Self {
        Self::default()
    }
}
