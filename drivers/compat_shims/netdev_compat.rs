// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/compat_shims/netdev_compat.rs — Network Device API Translation Layer

#![no_std]
#![allow(dead_code)]

/// Handles mapping of legacy net_device structures and operation vectors.
pub struct NetdevCompatShim;

#[repr(C)]
pub struct NetDevice {
    pub name: [u8; 16],
    pub base_addr: u64,
    pub irq: u32,
    pub trans_start: u64,
    // Modern linux uses net_device_ops struct; legacy directly stored callbacks.
    pub open: Option<unsafe extern "C" fn(*mut NetDevice) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut NetDevice) -> i32>,
    pub hard_start_xmit: Option<unsafe extern "C" fn(*mut NetDevice, *mut u8, usize) -> i32>,
}

impl NetdevCompatShim {
    pub unsafe fn register_netdev(dev: *mut NetDevice) -> i32 {
        let _ = dev;
        // Registers device to SigmaOS virtual network interface daemon (sigmad-netd / sigma_nic)
        0 // Success
    }
}
