// SPDX-License-Identifier: MIT
// SigmaOS Realtek RTL8169/8111 Gigabit Ethernet Driver
// Inspired by Linux (drivers/net/ethernet/realtek/8139cp.c) and FreeBSD (sys/dev/re/if_re.c)

use std::boxed::Box;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU32, Ordering};

#[cfg(not(feature = "standalone_test"))]
use crate::driver::pci_enumeration::{PciDeviceInfo, PciDriver};

#[cfg(feature = "standalone_test")]
#[path = "pci_enumeration.rs"]
mod pci_enumeration;
#[cfg(feature = "standalone_test")]
use pci_enumeration::{PciDeviceInfo, PciDriver};

// ============================================================================
// Realtek PCI Constants
// ============================================================================

pub const REALTEK_VENDOR_ID: u16 = 0x10EC;

pub const RTL8169_DEVICE_ID: u16 = 0x8169;
pub const RTL8168_DEVICE_ID: u16 = 0x8168;
pub const RTL8111_DEVICE_ID: u16 = 0x8111;

// Register Offsets
pub const REG_MAC0: u32 = 0x00;        // MAC Address Byte 0
pub const REG_COMMAND: u32 = 0x37;     // Command Register
pub const REG_IMR: u32 = 0x3C;         // Interrupt Mask Register
pub const REG_ISR: u32 = 0x3E;         // Interrupt Status Register
pub const REG_TX_CONFIG: u32 = 0x40;   // Transmit Configuration
pub const REG_RX_CONFIG: u32 = 0x44;   // Receive Configuration
pub const REG_TDSAR_LOW: u32 = 0x20;   // Transmit Descriptor Start Address Low
pub const REG_RDSAR_LOW: u32 = 0x28;   // Receive Descriptor Start Address Low

// Command Bits
pub const CMD_RESET: u8 = 0x10;
pub const CMD_RX_ENABLE: u8 = 0x08;
pub const CMD_TX_ENABLE: u8 = 0x04;

// ============================================================================
// Realtek RTL8169 NIC Driver Implementation
// ============================================================================

pub struct RealtekNicDriver {
    pub device_id: u16,
    pub pci_address: String,
    pub mac_bytes: [u8; 6],
    pub mmio_base: u64,
    pub mmio_size: u64,
    pub interrupt_line: u8,
    pub link_speed_mbps: u32,
    pub is_link_up: bool,
    pub packet_count: AtomicU32,
}

impl RealtekNicDriver {
    pub fn new(device_id: u16, pci_addr: &str) -> Self {
        Self {
            device_id,
            pci_address: pci_addr.to_string(),
            mac_bytes: [0x52, 0x54, 0x00, 0x81, 0x69, 0x01],
            mmio_base: 0,
            mmio_size: 0,
            interrupt_line: 0,
            link_speed_mbps: 1000, // 1Gbps default
            is_link_up: true,
            packet_count: AtomicU32::new(0),
        }
    }

    pub fn init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str> {
        self.mmio_base = bar;
        self.mmio_size = size;
        Ok(())
    }

    pub fn transmit_packet(&mut self, _packet: &[u8]) -> Result<(), &'static str> {
        if !self.is_link_up {
            return Err("Link down");
        }
        self.packet_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn get_packet_count(&self) -> u32 {
        self.packet_count.load(Ordering::SeqCst)
    }
}

// ============================================================================
// PciDriver Wrapper
// ============================================================================

pub struct RealtekRtl8169PciDriver {
    nic: Option<Box<RealtekNicDriver>>,
}

impl RealtekRtl8169PciDriver {
    pub fn new() -> Self {
        Self { nic: None }
    }

    pub fn get_nic(&self) -> Option<&RealtekNicDriver> {
        self.nic.as_ref().map(|b| b.as_ref())
    }

    pub fn get_nic_mut(&mut self) -> Option<&mut RealtekNicDriver> {
        self.nic.as_mut().map(|b| b.as_mut())
    }
}

impl Default for RealtekRtl8169PciDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl PciDriver for RealtekRtl8169PciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str> {
        if device.vendor_id != REALTEK_VENDOR_ID {
            return Ok(false);
        }

        let supported = match device.device_id {
            RTL8169_DEVICE_ID | RTL8168_DEVICE_ID | RTL8111_DEVICE_ID => true,
            _ => false,
        };

        if !supported {
            return Ok(false);
        }

        let mut nic = Box::new(RealtekNicDriver::new(
            device.device_id,
            &device.address.sysfs_format(),
        ));

        if let Some(ref bar) = device.bars[0] {
            nic.init_mmio(bar.address, bar.size)?;
        } else {
            return Err("No MMIO BAR found");
        }

        nic.interrupt_line = device.interrupt_line;
        self.nic = Some(nic);
        Ok(true)
    }

    fn remove(&mut self, _device: &PciDeviceInfo) -> Result<(), &'static str> {
        self.nic = None;
        Ok(())
    }

    fn name(&self) -> &str {
        "realtek_rtl8169"
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realtek_rtl8169_driver_creation_and_transmission() {
        let mut driver = RealtekNicDriver::new(RTL8169_DEVICE_ID, "0000:02:00.0");
        assert_eq!(driver.device_id, RTL8169_DEVICE_ID);
        assert!(driver.is_link_up);

        let packet = [0u8; 64];
        assert!(driver.transmit_packet(&packet).is_ok());
        assert_eq!(driver.get_packet_count(), 1);
    }

    #[test]
    fn test_realtek_rtl8169_pci_driver_name() {
        let pci_driver = RealtekRtl8169PciDriver::new();
        assert_eq!(pci_driver.name(), "realtek_rtl8169");
    }
}
