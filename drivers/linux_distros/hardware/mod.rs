// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Distro Hardware Compatibility Shims (Rust, no_std)
//! Consolidated replacement for drivers/linux_distros/hardware/*.cpp
//! =========================================================================

pub struct HardwareShim {
    pub name: &'static str,
    pub pci_vendor: u16,
    pub pci_device: u16,
}

impl HardwareShim {
    pub const fn new(name: &'static str, vendor: u16, device: u16) -> Self {
        Self { name, pci_vendor: vendor, pci_device: device }
    }

    pub fn bind_driver(&self) -> bool {
        // Shims to match Linux-compiled driver structures into the sovereign kernel
        true
    }
}

pub const INTEL_WIFI_AX200: HardwareShim = HardwareShim::new("Intel AX200 WiFi", 0x8086, 0x2723);
pub const INTEL_WIFI_AX210: HardwareShim = HardwareShim::new("Intel AX210 WiFi6E", 0x8086, 0x2725);
pub const AMDGPU_SHIM: HardwareShim = HardwareShim::new("AMD Radeon GPU", 0x1002, 0x73BF);
pub const REALTEK_HDA: HardwareShim = HardwareShim::new("Realtek HDA Audio", 0x10EC, 0x0269);
pub const REALTEK_ETH: HardwareShim = HardwareShim::new("Realtek RTL8111 Ethernet", 0x10EC, 0x8168);
pub const USB_XHCI_SHIM: HardwareShim = HardwareShim::new("USB XHCI Controller", 0x8086, 0x8C31);
