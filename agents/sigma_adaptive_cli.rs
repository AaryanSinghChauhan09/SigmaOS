// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// agents/sigma_adaptive_cli.rs — Adaptive CLI
// Implements: A command-line interface wrapper that intercepts errors,
// suggests corrections using the AI agent, and adapts to user behavior.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::string::{String, ToString};

pub struct AdaptiveCli {
    pub enabled: bool,
}

static mut ADAPTIVE_CLI: AdaptiveCli = AdaptiveCli {
    enabled: false,
};

impl AdaptiveCli {
    pub fn init(&mut self) {
        self.enabled = true;
    }

    pub fn handle_error(&self, _command: &str, _error_msg: &str) -> Option<String> {
        if !self.enabled { return None; }
        // STUB: Intercept shell error, pass to AI agent for correction
        // e.g. "Did you mean `sigpkg install` instead of `sigpkg instll`?"
        Some("Suggestion: Check spelling or refer to man pages.".to_string())
    }
}

pub fn adaptive_cli_init() {
    unsafe { ADAPTIVE_CLI.init(); }
}

pub fn adaptive_cli_handle_error(cmd: &str, err: &str) -> Option<String> {
    unsafe { ADAPTIVE_CLI.handle_error(cmd, err) }
}
