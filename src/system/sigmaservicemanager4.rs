// SPDX-License-Identifier: MIT
// SigmaOS Service Manager 4 Engine (`src/system/sigmaservicemanager4.rs`)

#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::String;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Active,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub state: ServiceState,
}

#[derive(Debug, Default)]
pub struct SigmaServiceManager4 {
    pub services: BTreeMap<String, Service>,
}

impl SigmaServiceManager4 {
    pub fn new() -> Self {
        Self::default()
    }
}
