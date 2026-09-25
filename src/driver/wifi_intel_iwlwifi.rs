// SPDX-License-Identifier: MIT
// SigmaOS Intel iwlwifi Wireless Driver
// Inspired by Linux (drivers/net/wireless/intel/iwlwifi/) and FreeBSD (sys/dev/iwm/)

use std::boxed::Box;
use std::string::String;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

#[cfg(not(feature = "standalone_test"))]
use crate::driver::pci_enumeration::{PciDeviceInfo, PciDriver};

#[cfg(feature = "standalone_test")]
#[path = "pci_enumeration.rs"]
mod pci_enumeration;
#[cfg(feature = "standalone_test")]
use pci_enumeration::{PciDeviceInfo, PciDriver};

// ============================================================================
// Intel WiFi Constants
// ============================================================================

pub const INTEL_VENDOR_ID: u16 = 0x8086;

// Intel Wireless Device IDs
pub const IWL_AX200_DEVICE_ID: u16 = 0x2723; // Wi-Fi 6 AX200
pub const IWL_AX210_DEVICE_ID: u16 = 0x2725; // Wi-Fi 6E AX210
pub const IWL_7265_DEVICE_ID: u16 = 0x095A;  // Wireless-AC 7265
pub const IWL_3165_DEVICE_ID: u16 = 0x095B;  // Wireless-AC 3165

// Register Offsets
pub const REG_CSR_HW_IF_CONFIG_REG: u32 = 0x000;
pub const REG_CSR_INT: u32 = 0x008;
pub const REG_CSR_INT_MASK: u32 = 0x00C;
pub const REG_CSR_FH_INT_STATUS: u32 = 0x01B;
pub const REG_CSR_GIO_REG: u32 = 0x020;
pub const REG_CSR_GP_CNTRL: u32 = 0x024;

// ============================================================================
// Intel iwlwifi Driver Implementation
// ============================================================================

pub struct IntelIwlwifiDriver {
    pub device_id: u16,
    pub pci_address: String,
    pub mmio_base: u64,
    pub mmio_size: u64,
    pub mac_address: [u8; 6],
    pub interrupt_line: u8,
    pub is_enabled: bool,
    pub current_channel: u8,
    pub tx_packets: AtomicU32,
    pub rx_packets: AtomicU32,
    pub power_saving_enabled: AtomicBool,
}

impl IntelIwlwifiDriver {
    pub fn new(device_id: u16, pci_addr: &str) -> Self {
        Self {
            device_id,
            pci_address: pci_addr.to_string(),
            mmio_base: 0,
            mmio_size: 0,
            mac_address: [0x00, 0x1E, 0x67, 0x8A, 0x90, 0x12],
            interrupt_line: 0,
            is_enabled: false,
            current_channel: 36, // Default 5GHz channel
            tx_packets: AtomicU32::new(0),
            rx_packets: AtomicU32::new(0),
            power_saving_enabled: AtomicBool::new(false),
        }
    }

    pub fn init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str> {
        self.mmio_base = bar;
        self.mmio_size = size;
        self.is_enabled = true;
        Ok(())
    }

    pub fn set_channel(&mut self, channel: u8) -> Result<(), &'static str> {
        if channel == 0 || channel > 165 {
            return Err("Invalid channel number");
        }
        self.current_channel = channel;
        Ok(())
    }

    pub fn enable_power_saving(&self, enabled: bool) {
        self.power_saving_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn is_power_saving(&self) -> bool {
        self.power_saving_enabled.load(Ordering::SeqCst)
    }

    pub fn get_stats(&self) -> (u32, u32) {
        (
            self.tx_packets.load(Ordering::SeqCst),
            self.rx_packets.load(Ordering::SeqCst),
        )
    }
}

// ============================================================================
// PciDriver Wrapper
// ============================================================================

pub struct IntelIwlwifiPciDriver {
    wifi: Option<Box<IntelIwlwifiDriver>>,
}

impl IntelIwlwifiPciDriver {
    pub fn new() -> Self {
        Self { wifi: None }
    }

    pub fn get_wifi(&self) -> Option<&IntelIwlwifiDriver> {
        self.wifi.as_ref().map(|b| b.as_ref())
    }

    pub fn get_wifi_mut(&mut self) -> Option<&mut IntelIwlwifiDriver> {
        self.wifi.as_mut().map(|b| b.as_mut())
    }
}

impl Default for IntelIwlwifiPciDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl PciDriver for IntelIwlwifiPciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str> {
        if device.vendor_id != INTEL_VENDOR_ID {
            return Ok(false);
        }

        let supported = match device.device_id {
            IWL_AX200_DEVICE_ID | IWL_AX210_DEVICE_ID | IWL_7265_DEVICE_ID | IWL_3165_DEVICE_ID => true,
            _ => false,
        };

        if !supported {
            return Ok(false);
        }

        let mut wifi = Box::new(IntelIwlwifiDriver::new(
            device.device_id,
            &device.address.sysfs_format(),
        ));

        if let Some(ref bar) = device.bars[0] {
            wifi.init_mmio(bar.address, bar.size)?;
        } else {
            return Err("No MMIO BAR found");
        }

        wifi.interrupt_line = device.interrupt_line;
        self.wifi = Some(wifi);
        Ok(true)
    }

    fn remove(&mut self, _device: &PciDeviceInfo) -> Result<(), &'static str> {
        self.wifi = None;
        Ok(())
    }

    fn name(&self) -> &str {
        "intel_iwlwifi"
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_iwlwifi_driver_creation_and_channel_setting() {
        let mut driver = IntelIwlwifiDriver::new(IWL_AX200_DEVICE_ID, "0000:01:00.0");
        assert_eq!(driver.device_id, IWL_AX200_DEVICE_ID);
        assert!(!driver.is_enabled);

        assert!(driver.init_mmio(0xE0000000, 0x2000).is_ok());
        assert!(driver.is_enabled);

        assert!(driver.set_channel(36).is_ok());
        assert_eq!(driver.current_channel, 36);

        driver.enable_power_saving(true);
        assert!(driver.is_power_saving());
    }

    #[test]
    fn test_intel_iwlwifi_pci_driver_name() {
        let pci_driver = IntelIwlwifiPciDriver::new();
        assert_eq!(pci_driver.name(), "intel_iwlwifi");
    }
}
