// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/compat_shims/mod.rs — Driver Warehouse Compatibility Shims Entry

#![no_std]

pub mod irq_compat;
pub mod dma_compat;
pub mod pci_compat;
pub mod netdev_compat;

/// Kernel version category for matching drivers to compatibility profiles.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KernelEra {
    Legacy26, // 2.6.x
    Legacy3x, // 3.x
    Legacy4x, // 4.x
    Modern,   // 5.x and 6.x
}

impl KernelEra {
    pub fn from_version(major: u32, minor: u32) -> Self {
        match major {
            2 => KernelEra::Legacy26,
            3 => KernelEra::Legacy3x,
            4 => KernelEra::Legacy4x,
            _ => KernelEra::Modern,
        }
    }
}
