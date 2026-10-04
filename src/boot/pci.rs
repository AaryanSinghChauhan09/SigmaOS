#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// S-Boot Firmware - PCI Bus Scanning
// BIOS & UEFI Specification Implementation

// (no_std only applicable at crate root - removed)

use std::vec::Vec;

pub const PCI_MAX_BUS: u16 = 256;
pub const PCI_MAX_DEVICE: u8 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciClass {
    Network,
    Storage,
    Display,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: PciClass,
}

impl PciDevice {
    pub fn new(bus: u8, slot: u8, vendor: u16, device: u16, class_code: u8) -> Self {
        let class = match class_code {
            0x02 => PciClass::Network,
            0x01 => PciClass::Storage,
            0x03 => PciClass::Display,
            _ => PciClass::Unknown,
        };
        Self {
            bus,
            slot,
            vendor_id: vendor,
            device_id: device,
            class,
        }
    }

    pub fn is_present(&self) -> bool {
        self.vendor_id != 0xFFFF
    }
}

pub struct PciBusScanner {
    pub registered_devices: Vec<PciDevice>,
}

impl PciBusScanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registered_devices: Vec::new(),
        }
    }

    pub fn scan_and_register(
        &mut self,
        bus: u8,
        slot: u8,
        vendor: u16,
        device: u16,
        class_code: u8,
    ) -> Result<(), &'static str> {
        if vendor == 0xFFFF {
            return Ok(()); // Device not present
        }

        let pci_device = PciDevice::new(bus, slot, vendor, device, class_code);

        if !pci_device.is_present() {
            return Err("Device not present");
        }

        self.registered_devices.push(pci_device);
        Ok(())
    }

    pub fn get_devices_by_class(&self, class: PciClass) -> Vec<&PciDevice> {
        self.registered_devices
            .iter()
            .filter(|d| d.class == class)
            .collect()
    }

    pub fn device_count(&self) -> usize {
        self.registered_devices.len()
    }

    pub fn clear(&mut self) {
        self.registered_devices.clear();
    }
}

impl Default for PciBusScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_device_creation() {
        let device = PciDevice::new(0, 1, 0x8086, 0x100E, 0x02);
        assert_eq!(device.bus, 0);
        assert_eq!(device.slot, 1);
        assert_eq!(device.vendor_id, 0x8086);
        assert_eq!(device.device_id, 0x100E);
        assert_eq!(device.class, PciClass::Network);
    }

    #[test]
    fn test_pci_device_presence() {
        let present = PciDevice::new(0, 1, 0x8086, 0x100E, 0x02);
        assert!(present.is_present());

        let absent = PciDevice::new(0, 1, 0xFFFF, 0x100E, 0x02);
        assert!(!absent.is_present());
    }

    #[test]
    fn test_pci_scanner() {
        let mut scanner = PciBusScanner::new();

        scanner
            .scan_and_register(0, 1, 0x8086, 0x100E, 0x02)
            .unwrap();
        assert_eq!(scanner.device_count(), 1);
    }

    #[test]
    fn test_pci_scan_absent() {
        let mut scanner = PciBusScanner::new();

        scanner
            .scan_and_register(0, 1, 0xFFFF, 0x100E, 0x02)
            .unwrap();
        assert_eq!(scanner.device_count(), 0);
    }

    #[test]
    fn test_get_devices_by_class() {
        let mut scanner = PciBusScanner::new();

        scanner
            .scan_and_register(0, 1, 0x8086, 0x100E, 0x02)
            .unwrap(); // Network
        scanner
            .scan_and_register(0, 2, 0x8086, 0x100F, 0x01)
            .unwrap(); // Storage
        scanner
            .scan_and_register(0, 3, 0x8086, 0x1010, 0x02)
            .unwrap(); // Network

        let network_devices = scanner.get_devices_by_class(PciClass::Network);
        assert_eq!(network_devices.len(), 2);

        let storage_devices = scanner.get_devices_by_class(PciClass::Storage);
        assert_eq!(storage_devices.len(), 1);
    }

    #[test]
    fn test_pci_class_mapping() {
        assert_eq!(
            PciDevice::new(0, 0, 0x8086, 0x100E, 0x02).class,
            PciClass::Network
        );
        assert_eq!(
            PciDevice::new(0, 0, 0x8086, 0x100E, 0x01).class,
            PciClass::Storage
        );
        assert_eq!(
            PciDevice::new(0, 0, 0x8086, 0x100E, 0x03).class,
            PciClass::Display
        );
        assert_eq!(
            PciDevice::new(0, 0, 0x8086, 0x100E, 0xFF).class,
            PciClass::Unknown
        );
    }

    #[test]
    fn test_scanner_clear() {
        let mut scanner = PciBusScanner::new();

        scanner
            .scan_and_register(0, 1, 0x8086, 0x100E, 0x02)
            .unwrap();
        scanner
            .scan_and_register(0, 2, 0x8086, 0x100F, 0x01)
            .unwrap();

        assert_eq!(scanner.device_count(), 2);

        scanner.clear();
        assert_eq!(scanner.device_count(), 0);
    }
}
