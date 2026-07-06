// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// sigma-pkg/sigma_pkg_gui.rs — Package Manager GUI Stub
// Implements: Basic bindings for a graphical package manager frontend.

use std::collections::HashMap;

pub struct PkgGui {
    pub is_running: bool,
}

impl PkgGui {
    pub fn new() -> Self {
        Self {
            is_running: false,
        }
    }

    pub fn run(&mut self) {
        // STUB: Initialize GUI loop, connecting to Zenith compositor
        self.is_running = true;
        println!("Sigma Package Manager GUI started.");
    }
}
