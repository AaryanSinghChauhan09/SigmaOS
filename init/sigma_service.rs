// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// init/sigma_service.rs — Service Parser and Manager
// Implements: Parsing of .service definitions, dependency tracking,
// and state management for system services.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
}

pub struct ServiceDef {
    pub name: String,
    pub exec_start: String,
    pub dependencies: Vec<String>,
    pub restart_on_fail: bool,
    pub state: ServiceState,
    pub pid: Option<u32>,
}

impl ServiceDef {
    pub fn parse(_content: &str) -> Option<Self> {
        // STUB: Parse INI/YAML-like service file format
        None
    }
}
