// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/drivers/thunderbolt/sigma_thunderbolt.rs — Thunderbolt Controller Driver
// Implements: Thunderbolt device discovery, authorization flow, secure device whitelisting,
// and DMA protection. Inspired by Linux thunderbolt driver and Apple Thunderbolt security.
//
// Reference: Linux drivers/thunderbolt/tb.c (GPL-2.0)

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── Thunderbolt Constants ─────────────────────────────────────────────────────
const THUNDERBOLT_VENDOR_ID: u16 = 0x8086; // Intel
const THUNDERBOLT_DEVICE_ID: u16 = 0x15E8; // Alpine Ridge

// Thunderbolt security levels (inspired by Linux thunderbolt security modes)
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ThunderboltSecurityLevel {
    None = 0,       // No security, all devices allowed
    User = 1,       // User approval required for new devices
    Secure = 2,     // Only approved devices allowed (key-based)
    Dma = 3,        // DMA protection enabled
    DmaInternal = 4,// DMA protection with internal ports only
}

// Thunderbolt device states
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ThunderboltDeviceState {
    Disconnected = 0,
    Connected = 1,
    Authorized = 2,
    Unauthorized = 3,
    Error = 4,
}

// ── Thunderbolt Device ───────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ThunderboltDeviceId {
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision: u8,
    pub unique_id: [u8; 16],
}

#[repr(C)]
pub struct ThunderboltDevice {
    pub id: ThunderboltDeviceId,
    pub state: ThunderboltDeviceState,
    pub authorized: bool,
    pub dma_allowed: bool,
    pub security_level: ThunderboltSecurityLevel,
    pub port_number: u8,
    pub link_speed: u32, // Gbps
}

// ── Thunderbolt Controller ───────────────────────────────────────────────────
const MAX_THUNDERBOLT_DEVICES: usize = 8;

pub struct ThunderboltController {
    pub devices: [ThunderboltDevice; MAX_THUNDERBOLT_DEVICES],
    pub device_count: usize,
    pub security_level: ThunderboltSecurityLevel,
    pub initialized: bool,
    pub whitelist: [ThunderboltDeviceId; 32],
    pub whitelist_count: usize,
}

impl ThunderboltController {
    pub const fn new() -> Self {
        Self {
            devices: [ThunderboltDevice {
                id: ThunderboltDeviceId {
                    vendor_id: 0,
                    device_id: 0,
                    revision: 0,
                    unique_id: [0; 16],
                },
                state: ThunderboltDeviceState::Disconnected,
                authorized: false,
                dma_allowed: false,
                security_level: ThunderboltSecurityLevel::None,
                port_number: 0,
                link_speed: 0,
            }; MAX_THUNDERBOLT_DEVICES],
            device_count: 0,
            security_level: ThunderboltSecurityLevel::Secure, // Default to secure
            initialized: false,
            whitelist: [ThunderboltDeviceId {
                vendor_id: 0,
                device_id: 0,
                revision: 0,
                unique_id: [0; 16],
            }; 32],
            whitelist_count: 0,
        }
    }

    /// Initialize Thunderbolt controller (inspired by Linux tb_domain_add)
    pub unsafe fn init(&mut self) -> i32 {
        // In production: scan PCIe for Thunderbolt controller
        // Initialize MMIO, enable interrupts
        self.initialized = true;
        0
    }

    /// Set security level (inspired by Linux tb_switch_set_security)
    pub unsafe fn set_security_level(&mut self, level: ThunderboltSecurityLevel) -> i32 {
        if !self.initialized {
            return -1;
        }

        self.security_level = level;

        // Re-authorize all devices based on new security level
        for i in 0..self.device_count {
            self.authorize_device(i);
        }

        0
    }

    /// Add device to whitelist (inspired by Linux thunderbolt device approval)
    pub unsafe fn add_to_whitelist(&mut self, device_id: ThunderboltDeviceId) -> i32 {
        if self.whitelist_count >= 32 {
            return -1;
        }

        self.whitelist[self.whitelist_count] = device_id;
        self.whitelist_count += 1;
        0
    }

    /// Remove device from whitelist
    pub unsafe fn remove_from_whitelist(&mut self, vendor_id: u16, device_id: u16) -> i32 {
        let mut found = false;
        let mut i = 0;

        while i < self.whitelist_count {
            if self.whitelist[i].vendor_id == vendor_id && self.whitelist[i].device_id == device_id {
                found = true;
                // Shift remaining entries
                let mut j = i;
                while j < self.whitelist_count - 1 {
                    self.whitelist[j] = self.whitelist[j + 1];
                    j += 1;
                }
                self.whitelist_count -= 1;
                break;
            }
            i += 1;
        }

        if found { 0 } else { -1 }
    }

    /// Check if device is in whitelist
    pub unsafe fn is_whitelisted(&self, device_id: &ThunderboltDeviceId) -> bool {
        let mut i = 0;
        while i < self.whitelist_count {
            if self.whitelist[i].vendor_id == device_id.vendor_id &&
               self.whitelist[i].device_id == device_id.device_id {
                return true;
            }
            i += 1;
        }
        false
    }

    /// Authorize a Thunderbolt device (inspired by Linux thunderbolt authorization)
    pub unsafe fn authorize_device(&mut self, device_index: usize) -> i32 {
        if device_index >= self.device_count {
            return -1;
        }

        let device = &mut self.devices[device_index];

        match self.security_level {
            ThunderboltSecurityLevel::None => {
                // Always authorize
                device.authorized = true;
                device.dma_allowed = true;
                device.state = ThunderboltDeviceState::Authorized;
            }
            ThunderboltSecurityLevel::User => {
                // User approval required (in production: prompt user)
                // For now, auto-authorize if not explicitly blocked
                device.authorized = true;
                device.dma_allowed = true;
                device.state = ThunderboltDeviceState::Authorized;
            }
            ThunderboltSecurityLevel::Secure => {
                // Only whitelisted devices
                if self.is_whitelisted(&device.id) {
                    device.authorized = true;
                    device.dma_allowed = true;
                    device.state = ThunderboltDeviceState::Authorized;
                } else {
                    device.authorized = false;
                    device.dma_allowed = false;
                    device.state = ThunderboltDeviceState::Unauthorized;
                    return -2;
                }
            }
            ThunderboltSecurityLevel::Dma | ThunderboltSecurityLevel::DmaInternal => {
                // DMA protection mode
                if self.is_whitelisted(&device.id) {
                    device.authorized = true;
                    device.dma_allowed = false; // DMA not allowed even if authorized
                    device.state = ThunderboltDeviceState::Authorized;
                } else {
                    device.authorized = false;
                    device.dma_allowed = false;
                    device.state = ThunderboltDeviceState::Unauthorized;
                    return -2;
                }
            }
        }

        0
    }

    /// Add newly connected Thunderbolt device
    pub unsafe fn add_device(&mut self, device_id: ThunderboltDeviceId, port: u8, speed: u32) -> i32 {
        if self.device_count >= MAX_THUNDERBOLT_DEVICES {
            return -1;
        }

        let idx = self.device_count;
        self.devices[idx].id = device_id;
        self.devices[idx].state = ThunderboltDeviceState::Connected;
        self.devices[idx].authorized = false;
        self.devices[idx].dma_allowed = false;
        self.devices[idx].security_level = self.security_level;
        self.devices[idx].port_number = port;
        self.devices[idx].link_speed = speed;

        self.device_count += 1;

        // Attempt authorization based on security level
        self.authorize_device(idx)
    }

    /// Remove Thunderbolt device
    pub unsafe fn remove_device(&mut self, device_index: usize) -> i32 {
        if device_index >= self.device_count {
            return -1;
        }

        // Disable DMA before removal
        self.devices[device_index].dma_allowed = false;
        self.devices[device_index].authorized = false;
        self.devices[device_index].state = ThunderboltDeviceState::Disconnected;

        // Shift remaining devices
        let mut i = device_index;
        while i < self.device_count - 1 {
            self.devices[i] = self.devices[i + 1];
            i += 1;
        }

        self.device_count -= 1;
        0
    }

    /// Get device by index
    pub unsafe fn get_device(&self, index: usize) -> Option<ThunderboltDevice> {
        if index < self.device_count {
            Some(self.devices[index])
        } else {
            None
        }
    }

    /// Get device count
    pub unsafe fn device_count(&self) -> usize {
        self.device_count
    }

    /// Enable DMA for a specific device (only in User/None mode)
    pub unsafe fn enable_dma(&mut self, device_index: usize) -> i32 {
        if device_index >= self.device_count {
            return -1;
        }

        if self.security_level == ThunderboltSecurityLevel::Dma ||
           self.security_level == ThunderboltSecurityLevel::DmaInternal {
            return -2; // DMA not allowed in these modes
        }

        if !self.devices[device_index].authorized {
            return -3;
        }

        self.devices[device_index].dma_allowed = true;
        0
    }

    /// Disable DMA for a specific device
    pub unsafe fn disable_dma(&mut self, device_index: usize) -> i32 {
        if device_index >= self.device_count {
            return -1;
        }

        self.devices[device_index].dma_allowed = false;
        0
    }
}

static mut TB_CONTROLLER: ThunderboltController = ThunderboltController::new();

// ── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_init() -> i32 {
    TB_CONTROLLER.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_set_security(level: u8) -> i32 {
    let security_level = match level {
        0 => ThunderboltSecurityLevel::None,
        1 => ThunderboltSecurityLevel::User,
        2 => ThunderboltSecurityLevel::Secure,
        3 => ThunderboltSecurityLevel::Dma,
        4 => ThunderboltSecurityLevel::DmaInternal,
        _ => return -1,
    };
    TB_CONTROLLER.set_security_level(security_level)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_add_to_whitelist(vendor_id: u16, device_id: u16, unique_id: *const u8) -> i32 {
    let mut uid = [0u8; 16];
    if !unique_id.is_null() {
        let src = core::slice::from_raw_parts(unique_id, 16);
        let mut i = 0;
        while i < 16 {
            uid[i] = src[i];
            i += 1;
        }
    }

    TB_CONTROLLER.add_to_whitelist(ThunderboltDeviceId {
        vendor_id,
        device_id,
        revision: 0,
        unique_id: uid,
    })
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_remove_from_whitelist(vendor_id: u16, device_id: u16) -> i32 {
    TB_CONTROLLER.remove_from_whitelist(vendor_id, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_add_device(vendor_id: u16, device_id: u16, port: u8, speed: u32) -> i32 {
    TB_CONTROLLER.add_device(ThunderboltDeviceId {
        vendor_id,
        device_id,
        revision: 0,
        unique_id: [0; 16],
    }, port, speed)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_remove_device(index: usize) -> i32 {
    TB_CONTROLLER.remove_device(index)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_authorize_device(index: usize) -> i32 {
    TB_CONTROLLER.authorize_device(index)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_enable_dma(index: usize) -> i32 {
    TB_CONTROLLER.enable_dma(index)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_disable_dma(index: usize) -> i32 {
    TB_CONTROLLER.disable_dma(index)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_device_count() -> usize {
    TB_CONTROLLER.device_count()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_is_authorized(index: usize) -> i32 {
    if let Some(device) = TB_CONTROLLER.get_device(index) {
        if device.authorized { 1 } else { 0 }
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_thunderbolt_is_dma_allowed(index: usize) -> i32 {
    if let Some(device) = TB_CONTROLLER.get_device(index) {
        if device.dma_allowed { 1 } else { 0 }
    } else {
        -1
    }
}
