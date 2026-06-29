// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Example Driver (Rust, no_std)
//! =========================================================================

#![no_std]
#![no_builtins]

#[path = "../kernel/mod.rs"]
pub mod sdk;

use sdk::oop::SigmaObject;
use sdk::types::{SigmaStatus, SIGMA_OK};

pub struct HelloDriver {
    active: bool,
}

impl HelloDriver {
    pub const fn new() -> Self {
        Self { active: false }
    }
}

impl SigmaObject for HelloDriver {
    fn initialize(&mut self) -> SigmaStatus {
        self.active = true;
        SIGMA_OK
    }

    fn class_name(&self) -> &'static str {
        "HelloDriver"
    }
}
