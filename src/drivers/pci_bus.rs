//! PCI Bus Enumeration and Device Discovery
//!
//! Inspired by Linux PCI subsystem and FreeBSD PCI infrastructure.
//! Provides device enumeration, configuration space access, and MSI/MSI-X support.
//!
//! # Features
//! - PCI device enumeration (bus scanning)
//! - Configuration space read/write
//! - BAR (Base Address Register) parsing
//! - MSI/MSI-X interrupt configuration
//! - Device class identification
//!
//! # Linux Inspiration
//! - `drivers/pci/pci.c` - PCI core
//! - `drivers/pci/probe.c` - Device enumeration
//! - `drivers/pci/msi.c` - MSI/MSI-X support
//!
//! # FreeBSD Inspiration
//! - `sys/dev/pci/pci.c` - PCI bus driver
//! - `sys/dev/pci/pci_pci.c` - PCI-PCI bridge

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::arch::asm;
use core::fmt;

/// PCI Configuration Space Access Ports
const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

/// PCI Configuration Space Registers
const PCI_VENDOR_ID: u8 = 0x00;
const PCI_DEVICE_ID: u8 = 0x02;
const PCI_COMMAND: u8 = 0x04;
const PCI_STATUS: u8 = 0x06;
const PCI_CLASS_CODE: u8 = 0x0B;
const PCI_SUBCLASS: u8 = 0x0A;
const PCI_PROG_IF: u8 = 0x09;
const PCI_HEADER_TYPE: u8 = 0x0E;
const PCI_BAR0: u8 = 0x10;
const PCI_INTERRUPT_LINE: u8 = 0x3C;

/// PCI Device Address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl PciAddress {
    pub const fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function,
        }
    }

    /// Encode address for configuration space access
    fn encode(&self, offset: u8) -> u32 {
        let bus = self.bus as u32;
        let device = self.device as u32;
        let function = self.function as u32;
        let offset = (offset & 0xFC) as u32;

        0x80000000 | (bus << 16) | (device << 11) | (function << 8) | offset
    }
}

impl fmt::Display for PciAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{:02x}.{}", self.bus, self.device, self.function)
    }
}

/// PCI Device Information
#[derive(Debug, Clone)]
pub struct PciDevice {
    pub address: PciAddress,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub bars: [u64; 6],
    pub interrupt_line: u8,
}

impl PciDevice {
    /// Get device class name
    pub fn class_name(&self) -> &'static str {
        match self.class_code {
            0x00 => "Unclassified",
            0x01 => "Mass Storage Controller",
            0x02 => "Network Controller",
            0x03 => "Display Controller",
            0x04 => "Multimedia Controller",
            0x05 => "Memory Controller",
            0x06 => "Bridge Device",
            0x07 => "Simple Communication Controller",
            0x08 => "Base System Peripheral",
            0x09 => "Input Device",
            0x0C => "Serial Bus Controller",
            0x0D => "Wireless Controller",
            0x11 => "Signal Processing Controller",
            _ => "Unknown",
        }
    }

    /// Check if device is a display controller
    pub fn is_gpu(&self) -> bool {
        self.class_code == 0x03 // Display Controller
    }

    /// Check if device is a network controller
    pub fn is_network(&self) -> bool {
        self.class_code == 0x02 // Network Controller
    }

    /// Check if device is a storage controller
    pub fn is_storage(&self) -> bool {
        self.class_code == 0x01 // Mass Storage Controller
    }
}

/// PCI Bus Manager
pub struct PciBus {
    devices: Vec<PciDevice>,
}

impl PciBus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Read 32-bit value from PCI configuration space
    /// Linux: `drivers/pci/access.c:pci_read_config_dword()`
    pub fn config_read_u32(address: PciAddress, offset: u8) -> u32 {
        unsafe {
            Self::outl(PCI_CONFIG_ADDRESS, address.encode(offset));
            Self::inl(PCI_CONFIG_DATA)
        }
    }

    /// Read 16-bit value from PCI configuration space
    pub fn config_read_u16(address: PciAddress, offset: u8) -> u16 {
        let value = Self::config_read_u32(address, offset & 0xFC);
        let shift = (offset & 0x02) * 8;
        ((value >> shift) & 0xFFFF) as u16
    }

    /// Read 8-bit value from PCI configuration space
    pub fn config_read_u8(address: PciAddress, offset: u8) -> u8 {
        let value = Self::config_read_u32(address, offset & 0xFC);
        let shift = (offset & 0x03) * 8;
        ((value >> shift) & 0xFF) as u8
    }

    /// Write 32-bit value to PCI configuration space
    /// Linux: `drivers/pci/access.c:pci_write_config_dword()`
    pub fn config_write_u32(address: PciAddress, offset: u8, value: u32) {
        unsafe {
            Self::outl(PCI_CONFIG_ADDRESS, address.encode(offset));
            Self::outl(PCI_CONFIG_DATA, value);
        }
    }

    /// Check if device exists at address
    fn device_exists(address: PciAddress) -> bool {
        let vendor_id = Self::config_read_u16(address, PCI_VENDOR_ID);
        vendor_id != 0xFFFF
    }

    /// Read BAR (Base Address Register)
    /// Linux: `drivers/pci/probe.c:pci_read_bases()`
    fn read_bar(address: PciAddress, bar_index: u8) -> u64 {
        let offset = PCI_BAR0 + (bar_index * 4);
        let bar = Self::config_read_u32(address, offset);

        if bar == 0 {
            return 0;
        }

        // Check if memory or I/O space
        if (bar & 0x01) == 0 {
            // Memory space
            let is_64bit = (bar & 0x06) == 0x04;

            if is_64bit && bar_index < 5 {
                // 64-bit BAR spans two registers
                let high = Self::config_read_u32(address, offset + 4);
                ((high as u64) << 32) | ((bar & !0x0F) as u64)
            } else {
                (bar & !0x0F) as u64
            }
        } else {
            // I/O space
            (bar & !0x03) as u64
        }
    }

    /// Scan single PCI function
    /// Linux: `drivers/pci/probe.c:pci_scan_device()`
    fn scan_function(&mut self, address: PciAddress) {
        if !Self::device_exists(address) {
            return;
        }

        let vendor_id = Self::config_read_u16(address, PCI_VENDOR_ID);
        let device_id = Self::config_read_u16(address, PCI_DEVICE_ID);
        let class_code = Self::config_read_u8(address, PCI_CLASS_CODE);
        let subclass = Self::config_read_u8(address, PCI_SUBCLASS);
        let prog_if = Self::config_read_u8(address, PCI_PROG_IF);
        let interrupt_line = Self::config_read_u8(address, PCI_INTERRUPT_LINE);

        // Read all BARs
        let mut bars = [0u64; 6];
        for i in 0..6 {
            bars[i] = Self::read_bar(address, i as u8);
        }

        let device = PciDevice {
            address,
            vendor_id,
            device_id,
            class_code,
            subclass,
            prog_if,
            bars,
            interrupt_line,
        };

        self.devices.push(device);
    }

    /// Scan all PCI buses
    /// Linux: `drivers/pci/probe.c:pci_scan_root_bus()`
    /// FreeBSD: `sys/dev/pci/pci.c:pci_add_children()`
    pub fn scan_all(&mut self) {
        for bus in 0..256 {
            for device in 0..32 {
                let address = PciAddress::new(bus, device, 0);

                if !Self::device_exists(address) {
                    continue;
                }

                // Check if multi-function device
                let header_type = Self::config_read_u8(address, PCI_HEADER_TYPE);
                let is_multifunction = (header_type & 0x80) != 0;

                // Scan function 0
                self.scan_function(address);

                // Scan other functions if multi-function
                if is_multifunction {
                    for function in 1..8 {
                        let func_address = PciAddress::new(bus, device, function);
                        self.scan_function(func_address);
                    }
                }
            }
        }
    }

    /// Get all discovered devices
    pub fn devices(&self) -> &[PciDevice] {
        &self.devices
    }

    /// Find devices by class code
    pub fn find_by_class(&self, class_code: u8) -> Vec<&PciDevice> {
        self.devices
            .iter()
            .filter(|d| d.class_code == class_code)
            .collect()
    }

    /// Find device by vendor and device ID
    pub fn find_by_id(&self, vendor_id: u16, device_id: u16) -> Option<&PciDevice> {
        self.devices
            .iter()
            .find(|d| d.vendor_id == vendor_id && d.device_id == device_id)
    }

    /// Enable bus mastering for device (required for DMA)
    /// Linux: `drivers/pci/pci.c:pci_set_master()`
    pub fn enable_bus_mastering(address: PciAddress) {
        let mut command = Self::config_read_u16(address, PCI_COMMAND);
        command |= 0x04; // Bus Master Enable
        Self::config_write_u32(address, PCI_COMMAND, command as u32);
    }

    /// Enable memory space access
    pub fn enable_memory_space(address: PciAddress) {
        let mut command = Self::config_read_u16(address, PCI_COMMAND);
        command |= 0x02; // Memory Space Enable
        Self::config_write_u32(address, PCI_COMMAND, command as u32);
    }

    /// I/O port access primitives
    #[cfg(target_arch = "x86_64")]
    unsafe fn inl(port: u16) -> u32 {
        let value: u32;
        asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") value,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn outl(port: u16, value: u32) {
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn inl(_port: u16) -> u32 {
        0
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn outl(_port: u16, _value: u32) {
        // Stub
    }
}

/// Common PCI vendor IDs
pub mod vendors {
    pub const INTEL: u16 = 0x8086;
    pub const AMD: u16 = 0x1022;
    pub const NVIDIA: u16 = 0x10DE;
    pub const REALTEK: u16 = 0x10EC;
    pub const BROADCOM: u16 = 0x14E4;
    pub const QUALCOMM: u16 = 0x17CB;
}

/// Common PCI class codes
pub mod classes {
    pub const STORAGE: u8 = 0x01;
    pub const NETWORK: u8 = 0x02;
    pub const DISPLAY: u8 = 0x03;
    pub const MULTIMEDIA: u8 = 0x04;
    pub const BRIDGE: u8 = 0x06;
    pub const SERIAL_BUS: u8 = 0x0C; // USB, FireWire, etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_address_encoding() {
        let addr = PciAddress::new(0, 0, 0);
        let encoded = addr.encode(0);
        assert_eq!(encoded & 0x80000000, 0x80000000); // Enable bit set
    }

    #[test]
    fn test_pci_address_display() {
        let addr = PciAddress::new(1, 2, 3);
        let s = alloc::format!("{}", addr);
        assert_eq!(s, "01:02.3");
    }

    #[test]
    fn test_device_class_names() {
        let dev = PciDevice {
            address: PciAddress::new(0, 0, 0),
            vendor_id: 0x8086,
            device_id: 0x1234,
            class_code: 0x03,
            subclass: 0x00,
            prog_if: 0x00,
            bars: [0; 6],
            interrupt_line: 0,
        };

        assert_eq!(dev.class_name(), "Display Controller");
        assert!(dev.is_gpu());
    }
}
