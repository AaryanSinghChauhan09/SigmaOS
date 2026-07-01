// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Driver Template (Rust, no_std)
//! Replaces: ecosystem/templates/SovereignDriverTemplate.c
//! =========================================================================

#![no_std]
#![no_builtins]

use crate::sdk::oop::SigmaObject;
use crate::sdk::types::{SigmaStatus, SIGMA_OK};

#[path = "../../sdk/kernel/mod.rs"]
pub mod sdk;

pub struct SovereignDriverTemplate {
    device_id: u32,
    initialized: bool,
}

impl SovereignDriverTemplate {
    pub const fn new(id: u32) -> Self {
        Self { device_id: id, initialized: false }
    }
}

impl SigmaObject for SovereignDriverTemplate {
    fn initialize(&mut self) -> SigmaStatus {
        self.initialized = true;
        SIGMA_OK
    }

    fn class_name(&self) -> &'static str {
        "SovereignDriverTemplate"
    }
}
