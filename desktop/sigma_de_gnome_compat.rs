// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/sigma_de_gnome_compat.rs — GNOME Compatibility Layer
// Implements: Stubs for bridging Zenith compositor with GTK/GNOME Wayland protocols.

#![no_std]
#![allow(dead_code)]

pub struct GnomeCompat {
    pub active: bool,
}

impl GnomeCompat {
    pub fn new() -> Self {
        Self { active: false }
    }

    pub fn enable(&mut self) {
        self.active = true;
        // STUB: Register GTK/GNOME specific Wayland protocols (e.g. gtk-shell)
    }
}
