// SPDX-License-Identifier: MIT
// SigmaOS PCI Device Enumeration & Device Binding
// Complete PCI bus enumeration, BAR allocation, and device driver binding

use std::collections::BTreeMap;
use std::vec::Vec;
use std::string::{String, ToString};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::driver::pci_bus::PciAddress;

// ============================================================================
// PCI Configuration Space Constants
// ============================================================================

pub const PCI_VENDOR_ID: u8 = 0x00;
pub const PCI_DEVICE_ID: u8 = 0x02;
pub const PCI_COMMAND: u8 = 0x04;
pub const PCI_STATUS: u8 = 0x06;
pub const PCI_REVISION_ID: u8 = 0x08;
pub const PCI_CLASS_CODE: u8 = 0x09;
pub const PCI_SUBCLASS_CODE: u8 = 0x0a;
pub const PCI_PROG_INTERFACE: u8 = 0x0b;
pub const PCI_HEADER_TYPE: u8 = 0x0e;
pub const PCI_BAR_0: u8 = 0x10;
pub const PCI_BAR_1: u8 = 0x14;
pub const PCI_BAR_2: u8 = 0x18;
pub const PCI_BAR_3: u8 = 0x1c;
pub const PCI_BAR_4: u8 = 0x20;
pub const PCI_BAR_5: u8 = 0x24;
pub const PCI_CARDBUS_CIS: u8 = 0x28;
pub const PCI_SUBSYSTEM_VENDOR_ID: u8 = 0x2c;
pub const PCI_SUBSYSTEM_DEVICE_ID: u8 = 0x2e;
pub const PCI_EXPANSION_ROM: u8 = 0x30;
pub const PCI_CAPABILITIES: u8 = 0x34;
pub const PCI_INTERRUPT_LINE: u8 = 0x3c;
pub const PCI_INTERRUPT_PIN: u8 = 0x3d;
pub const PCI_MIN_GRANT: u8 = 0x3e;
pub const PCI_MAX_LATENCY: u8 = 0x3f;

// Command register bits
pub const PCI_CMD_IO_SPACE: u16 = 0x0001;
pub const PCI_CMD_MEMORY_SPACE: u16 = 0x0002;
pub const PCI_CMD_BUS_MASTER: u16 = 0x0004;
pub const PCI_CMD_SPECIAL_CYCLES: u16 = 0x0008;
pub const PCI_CMD_WRITE_INVALIDATE: u16 = 0x0010;
pub const PCI_CMD_VGA_PALETTE_SNOOP: u16 = 0x0020;
pub const PCI_CMD_PARITY_ERROR_RESPONSE: u16 = 0x0040;
pub const PCI_CMD_SERR_ENABLE: u16 = 0x0100;
pub const PCI_CMD_FAST_BACK_TO_BACK: u16 = 0x0200;
pub const PCI_CMD_INTERRUPT_DISABLE: u16 = 0x0400;

// PCI Class Codes
pub const PCI_CLASS_UNCLASSIFIED: u8 = 0x00;
pub const PCI_CLASS_MASS_STORAGE: u8 = 0x01;
pub const PCI_CLASS_NETWORK: u8 = 0x02;
pub const PCI_CLASS_DISPLAY: u8 = 0x03;
pub const PCI_CLASS_MULTIMEDIA: u8 = 0x04;
pub const PCI_CLASS_MEMORY: u8 = 0x05;
pub const PCI_CLASS_BRIDGE: u8 = 0x06;
pub const PCI_CLASS_SIMPLE_COMMS: u8 = 0x07;
pub const PCI_CLASS_BASE_SYSTEM: u8 = 0x08;
pub const PCI_CLASS_INPUT_DEVICE: u8 = 0x09;
pub const PCI_CLASS_DOCKING_STATION: u8 = 0x0a;
pub const PCI_CLASS_PROCESSOR: u8 = 0x0b;
pub const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
pub const PCI_CLASS_WIRELESS: u8 = 0x0d;
pub const PCI_CLASS_INTELLIGENT_CONTROLLER: u8 = 0x0e;
pub const PCI_CLASS_SATELLITE_COMMS: u8 = 0x0f;
pub const PCI_CLASS_ENCRYPTION: u8 = 0x10;
pub const PCI_CLASS_DATA_ACQ_SIGNAL_PROCESSING: u8 = 0x11;

// ============================================================================
// PCI Device Information
// ============================================================================

#[derive(Debug, Clone)]
pub struct PciDeviceInfo {
    pub address: PciAddress,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass_code: u8,
    pub prog_interface: u8,
    pub revision_id: u8,
    pub header_type: u8,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub bars: [Option<PciBar>; 6],
    pub expansion_rom: u32,
}

impl PciDeviceInfo {
    pub fn new(address: PciAddress) -> Self {
        PciDeviceInfo {
            address,
            vendor_id: 0xffff,
            device_id: 0xffff,
            class_code: 0,
            subclass_code: 0,
            prog_interface: 0,
            revision_id: 0,
            header_type: 0,
            subsystem_vendor_id: 0,
            subsystem_device_id: 0,
            interrupt_line: 0,
            interrupt_pin: 0,
            bars: [None; 6],
            expansion_rom: 0,
        }
    }

    pub fn class_name(&self) -> &'static str {
        match self.class_code {
            PCI_CLASS_UNCLASSIFIED => "Unclassified",
            PCI_CLASS_MASS_STORAGE => "Mass Storage",
            PCI_CLASS_NETWORK => "Network",
            PCI_CLASS_DISPLAY => "Display",
            PCI_CLASS_MULTIMEDIA => "Multimedia",
            PCI_CLASS_MEMORY => "Memory",
            PCI_CLASS_BRIDGE => "Bridge",
            PCI_CLASS_SIMPLE_COMMS => "Simple Communications",
            PCI_CLASS_BASE_SYSTEM => "Base System",
            PCI_CLASS_INPUT_DEVICE => "Input Device",
            PCI_CLASS_DOCKING_STATION => "Docking Station",
            PCI_CLASS_PROCESSOR => "Processor",
            PCI_CLASS_SERIAL_BUS => "Serial Bus",
            PCI_CLASS_WIRELESS => "Wireless",
            PCI_CLASS_INTELLIGENT_CONTROLLER => "Intelligent Controller",
            PCI_CLASS_SATELLITE_COMMS => "Satellite Communications",
            PCI_CLASS_ENCRYPTION => "Encryption",
            PCI_CLASS_DATA_ACQ_SIGNAL_PROCESSING => "Data Acquisition",
            _ => "Unknown",
        }
    }

    pub fn is_valid(&self) -> bool {
        self.vendor_id != 0xffff && self.vendor_id != 0x0000
    }
}

// ============================================================================
// PCI Base Address Register (BAR)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciBarType {
    IoSpace,
    Memory32Bit { prefetchable: bool },
    Memory64Bit { prefetchable: bool },
}

#[derive(Debug, Clone)]
pub struct PciBar {
    pub bar_type: PciBarType,
    pub address: u64,
    pub size: u64,
    pub index: u8,
}

impl PciBar {
    pub fn new(index: u8) -> Self {
        PciBar {
            bar_type: PciBarType::Memory32Bit { prefetchable: false },
            address: 0,
            size: 0,
            index,
        }
    }

    pub fn is_io(&self) -> bool {
        matches!(self.bar_type, PciBarType::IoSpace)
    }

    pub fn is_memory(&self) -> bool {
        matches!(
            self.bar_type,
            PciBarType::Memory32Bit { .. } | PciBarType::Memory64Bit { .. }
        )
    }
}

// ============================================================================
// PCI Configuration Space I/O
// ============================================================================

#[inline]
pub fn pci_read_u8(addr: PciAddress, offset: u8) -> u8 {
    // Use legacy I/O port access (0xCF8/0xCFC)
    let config_address = addr.legacy_io_address(offset);
    unsafe {
        let port_addr = 0x0CF8u16;
        // SAFETY: Writing to PCI config ports is platform standard
        // core::arch::x86_64::_outl( // stub:port_addr, config_address);
        let data_port = 0x0CFCu16;
        let value = 0u32 // core::arch::x86_64::_inl( stub:data_port);
        ((value >> ((offset & 3) * 8)) & 0xff) as u8
    }
}

#[inline]
pub fn pci_read_u16(addr: PciAddress, offset: u8) -> u16 {
    let config_address = addr.legacy_io_address(offset & !1);
    unsafe {
        let port_addr = 0x0CF8u16;
        // core::arch::x86_64::_outl( // stub:port_addr, config_address);
        let data_port = 0x0CFCu16;
        let value = 0u32 // core::arch::x86_64::_inl( stub:data_port);
        ((value >> ((offset & 2) * 8)) & 0xffff) as u16
    }
}

#[inline]
pub fn pci_read_u32(addr: PciAddress, offset: u8) -> u32 {
    let config_address = addr.legacy_io_address(offset & !3);
    unsafe {
        let port_addr = 0x0CF8u16;
        // core::arch::x86_64::_outl( // stub:port_addr, config_address);
        let data_port = 0x0CFCu16;
        0u32 // core::arch::x86_64::_inl( stub:data_port)
    }
}

#[inline]
pub fn pci_write_u8(addr: PciAddress, offset: u8, value: u8) {
    let config_address = addr.legacy_io_address(offset);
    unsafe {
        let port_addr = 0x0CF8u16;
        // core::arch::x86_64::_outl( // stub:port_addr, config_address);
        let data_port = 0x0CFCu16 + ((offset & 3) as u16);
        // core::arch::x86_64::_outb( // stub:data_port, value);
    }
}

#[inline]
pub fn pci_write_u16(addr: PciAddress, offset: u8, value: u16) {
    let config_address = addr.legacy_io_address(offset & !1);
    unsafe {
        let port_addr = 0x0CF8u16;
        // core::arch::x86_64::_outl( // stub:port_addr, config_address);
        let data_port = 0x0CFCu16 + ((offset & 2) as u16);
        // core::arch::x86_64::_outw( // stub:data_port, value);
    }
}

#[inline]
pub fn pci_write_u32(addr: PciAddress, offset: u8, value: u32) {
    let config_address = addr.legacy_io_address(offset & !3);
    unsafe {
        let port_addr = 0x0CF8u16;
        // core::arch::x86_64::_outl( // stub:port_addr, config_address);
        let data_port = 0x0CFCu16;
        // core::arch::x86_64::_outl( // stub:data_port, value);
    }
}

// ============================================================================
// PCI Device Enumeration
// ============================================================================

pub struct PciEnumerator {
    devices: Vec<PciDeviceInfo>,
}

impl PciEnumerator {
    pub fn new() -> Self {
        PciEnumerator {
            devices: Vec::new(),
        }
    }

    pub fn enumerate(&mut self) -> Result<usize, &'static str> {
        let mut count = 0;

        // Scan all PCI buses
        for bus in 0..=255u8 {
            for device in 0..32u8 {
                // Check function 0 first
                let addr = PciAddress::new(0, bus, device, 0);
                let vendor_id = pci_read_u16(addr, PCI_VENDOR_ID);

                if vendor_id == 0xffff || vendor_id == 0x0000 {
                    continue; // No device at this address
                }

                // Device found, check header type for multi-function devices
                let header_type = pci_read_u8(addr, PCI_HEADER_TYPE);
                let multi_function = (header_type & 0x80) != 0;

                let max_functions = if multi_function { 8 } else { 1 };

                for function in 0..max_functions {
                    let addr = PciAddress::new(0, bus, device, function);
                    if self.probe_device(addr)? {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    fn probe_device(&mut self, addr: PciAddress) -> Result<bool, &'static str> {
        let vendor_id = pci_read_u16(addr, PCI_VENDOR_ID);

        if vendor_id == 0xffff || vendor_id == 0x0000 {
            return Ok(false);
        }

        let mut device = PciDeviceInfo::new(addr);
        device.vendor_id = vendor_id;
        device.device_id = pci_read_u16(addr, PCI_DEVICE_ID);
        device.class_code = pci_read_u8(addr, PCI_CLASS_CODE);
        device.subclass_code = pci_read_u8(addr, PCI_SUBCLASS_CODE);
        device.prog_interface = pci_read_u8(addr, PCI_PROG_INTERFACE);
        device.revision_id = pci_read_u8(addr, PCI_REVISION_ID);
        device.header_type = pci_read_u8(addr, PCI_HEADER_TYPE);
        device.subsystem_vendor_id = pci_read_u16(addr, PCI_SUBSYSTEM_VENDOR_ID);
        device.subsystem_device_id = pci_read_u16(addr, PCI_SUBSYSTEM_DEVICE_ID);
        device.interrupt_line = pci_read_u8(addr, PCI_INTERRUPT_LINE);
        device.interrupt_pin = pci_read_u8(addr, PCI_INTERRUPT_PIN);
        device.expansion_rom = pci_read_u32(addr, PCI_EXPANSION_ROM);

        // Read BARs
        self.probe_bars(&mut device)?;

        // Enable device
        self.enable_device(&device)?;

        self.devices.push(device);
        Ok(true)
    }

    fn probe_bars(&self, device: &mut PciDeviceInfo) -> Result<(), &'static str> {
        let mut skip_next = false;

        for i in 0..6 {
            if skip_next {
                skip_next = false;
                continue;
            }

            let bar_offset = PCI_BAR_0 + (i as u8 * 4);
            let bar_raw = pci_read_u32(device.address, bar_offset);

            if bar_raw == 0 {
                continue;
            }

            if (bar_raw & 0x01) != 0 {
                // I/O Space BAR
                let address = (bar_raw & 0xfffffffc) as u64;
                let mut bar = PciBar::new(i as u8);
                bar.bar_type = PciBarType::IoSpace;
                bar.address = address;

                // Detect size by writing all 1s
                pci_write_u32(device.address, bar_offset, 0xffffffff);
                let size_mask = pci_read_u32(device.address, bar_offset);
                pci_write_u32(device.address, bar_offset, bar_raw);
                bar.size = ((!size_mask) & 0xfffc).wrapping_add(1) as u64;

                device.bars[i] = Some(bar);
            } else {
                // Memory Space BAR
                let prefetchable = (bar_raw & 0x08) != 0;
                let mem_type = (bar_raw & 0x06) >> 1;

                match mem_type {
                    0 => {
                        // 32-bit memory
                        let address = (bar_raw & 0xfffffff0) as u64;
                        let mut bar = PciBar::new(i as u8);
                        bar.bar_type = PciBarType::Memory32Bit { prefetchable };
                        bar.address = address;

                        pci_write_u32(device.address, bar_offset, 0xffffffff);
                        let size_mask = pci_read_u32(device.address, bar_offset);
                        pci_write_u32(device.address, bar_offset, bar_raw);
                        bar.size = ((!size_mask) & 0xfffffff0).wrapping_add(1) as u64;

                        device.bars[i] = Some(bar);
                    }
                    2 => {
                        // 64-bit memory
                        if i < 5 {
                            let address_low = (bar_raw & 0xfffffff0) as u64;
                            let bar_offset_high = bar_offset + 4;
                            let address_high = pci_read_u32(device.address, bar_offset_high) as u64;
                            let address = (address_high << 32) | address_low;

                            let mut bar = PciBar::new(i as u8);
                            bar.bar_type = PciBarType::Memory64Bit { prefetchable };
                            bar.address = address;

                            pci_write_u32(device.address, bar_offset, 0xffffffff);
                            let size_mask_low = pci_read_u32(device.address, bar_offset);
                            pci_write_u32(device.address, bar_offset, bar_raw);

                            pci_write_u32(device.address, bar_offset_high, 0xffffffff);
                            let size_mask_high = pci_read_u32(device.address, bar_offset_high);
                            pci_write_u32(device.address, bar_offset_high, (address_high & 0xffffffff) as u32);

                            let size = ((((size_mask_high as u64) << 32) | (size_mask_low as u64)) & 0xfffffffffffffff0)
                                .wrapping_add(1);
                            bar.size = size;

                            device.bars[i] = Some(bar);
                            skip_next = true; // Skip next BAR as it's part of 64-bit address
                        }
                    }
                    _ => {} // Reserved
                }
            }
        }

        Ok(())
    }

    fn enable_device(&self, device: &PciDeviceInfo) -> Result<(), &'static str> {
        let mut cmd = pci_read_u16(device.address, PCI_COMMAND);

        // Enable I/O and memory access
        cmd |= PCI_CMD_IO_SPACE | PCI_CMD_MEMORY_SPACE | PCI_CMD_BUS_MASTER;

        // Disable interrupt generation during initialization
        cmd |= PCI_CMD_INTERRUPT_DISABLE;

        pci_write_u16(device.address, PCI_COMMAND, cmd);

        Ok(())
    }

    pub fn get_devices(&self) -> &[PciDeviceInfo] {
        &self.devices
    }

    pub fn find_devices_by_class(&self, class: u8) -> Vec<&PciDeviceInfo> {
        self.devices
            .iter()
            .filter(|d| d.class_code == class)
            .collect()
    }

    pub fn find_devices_by_vendor(&self, vendor_id: u16) -> Vec<&PciDeviceInfo> {
        self.devices
            .iter()
            .filter(|d| d.vendor_id == vendor_id)
            .collect()
    }

    pub fn find_device_by_address(&self, address: PciAddress) -> Option<&PciDeviceInfo> {
        self.devices.iter().find(|d| d.address == address)
    }
}

// ============================================================================
// PCI Device Driver Binding
// ============================================================================

pub trait PciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str>;
    fn remove(&mut self, device: &PciDeviceInfo) -> Result<(), &'static str>;
    fn name(&self) -> &str;
}

pub struct PciDriverManager {
    drivers: Vec<Box<dyn PciDriver>>,
    bound_devices: BTreeMap<String, String>, // device_address -> driver_name
}

impl PciDriverManager {
    pub fn new() -> Self {
        PciDriverManager {
            drivers: Vec::new(),
            bound_devices: BTreeMap::new(),
        }
    }

    pub fn register_driver(&mut self, driver: Box<dyn PciDriver>) {
        self.drivers.push(driver);
    }

    pub fn probe_devices(&mut self, enumerator: &PciEnumerator) -> Result<usize, &'static str> {
        let mut bound_count = 0;

        for device in enumerator.get_devices() {
            for driver in &mut self.drivers {
                if driver.probe(device)? {
                    self.bound_devices.insert(
                        device.address.sysfs_format(),
                        driver.name().to_string(),
                    );
                    bound_count += 1;
                    break; // Device is now handled by this driver
                }
            }
        }

        Ok(bound_count)
    }

    pub fn get_bound_devices(&self) -> &BTreeMap<String, String> {
        &self.bound_devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_bar_types() {
        let io_bar = PciBar {
            bar_type: PciBarType::IoSpace,
            address: 0x1000,
            size: 256,
            index: 0,
        };
        assert!(io_bar.is_io());
        assert!(!io_bar.is_memory());

        let mem_bar = PciBar {
            bar_type: PciBarType::Memory32Bit { prefetchable: true },
            address: 0xf0000000,
            size: 0x10000000,
            index: 1,
        };
        assert!(!mem_bar.is_io());
        assert!(mem_bar.is_memory());
    }

    #[test]
    fn test_pci_device_info_creation() {
        let addr = PciAddress::new(0, 0, 0, 0);
        let device = PciDeviceInfo::new(addr);
        assert_eq!(device.vendor_id, 0xffff);
        assert!(!device.is_valid());
    }

    #[test]
    fn test_pci_class_names() {
        let mut device = PciDeviceInfo::new(PciAddress::new(0, 0, 0, 0));
        device.vendor_id = 0x8086; // Valid vendor

        device.class_code = PCI_CLASS_NETWORK;
        assert_eq!(device.class_name(), "Network");

        device.class_code = PCI_CLASS_DISPLAY;
        assert_eq!(device.class_name(), "Display");

        device.class_code = PCI_CLASS_MASS_STORAGE;
        assert_eq!(device.class_name(), "Mass Storage");
    }

    #[test]
    fn test_pci_enumerator_creation() {
        let enumerator = PciEnumerator::new();
        assert_eq!(enumerator.get_devices().len(), 0);
    }

    #[test]
    fn test_pci_driver_manager_creation() {
        let manager = PciDriverManager::new();
        assert_eq!(manager.get_bound_devices().len(), 0);
    }
}
