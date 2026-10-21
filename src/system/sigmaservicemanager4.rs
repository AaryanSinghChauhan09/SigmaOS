// SPDX-License-Identifier: MIT
// SigmaOS Service Manager 4

#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Running,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub state: ServiceState,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaServiceManager4 {
    pub services: Vec<Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        Self::default()
    }
}
