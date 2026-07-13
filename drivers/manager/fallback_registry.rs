// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/manager/fallback_registry.rs — Generic Driver Fallbacks

#![no_std]
#![allow(dead_code)]

/// Handles identifying generic fallback drivers when vendor-specific drivers fail.
pub struct FallbackRegistry;

impl FallbackRegistry {
    pub fn get_fallback_driver(category: &str) -> &'static str {
        match category {
            "network" => "sigma-virtio-net",
            "gpu" => "vesafb",
            "storage" => "sigma-virtio-blk",
            _ => "generic-stub",
        }
    }
}
