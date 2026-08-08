// S-BOOT FIRMWARE (PCI Bus Scanner)
// BIOS & UEFI Specification compliance for PCI device scanning and registration

#![no_std]

extern crate alloc;

pub const PCI_MAX_BUS: usize = 256;
pub const PCI_MAX_DEVICE: u8 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciClass {
    Network,
    Storage,
    Display,
    Unknown,
}

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
}

pub struct PciBusScanner {
    pub registered_devices: [Option<PciDevice>; 16],
}

impl PciBusScanner {
    pub fn new() -> Self {
        const NONE_DEV: Option<PciDevice> = None;
        Self {
            registered_devices: [NONE_DEV; 16],
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
        let dev = PciDevice::new(bus, slot, vendor, device, class_code);
        for slot in self.registered_devices.iter_mut() {
            if slot.is_none() {
                *slot = Some(dev);
                return Ok(());
            }
        }
        Err("Active boot firmware PCI registry full")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_device_creation() {
        let device = PciDevice::new(0, 1, 0x1234, 0x5678, 0x02);
        assert_eq!(device.bus, 0);
        assert_eq!(device.slot, 1);
        assert_eq!(device.vendor_id, 0x1234);
        assert_eq!(device.device_id, 0x5678);
        assert_eq!(device.class, PciClass::Network);
    }

    #[test]
    fn test_pci_scanner_initialization() {
        let scanner = PciBusScanner::new();
        assert_eq!(scanner.registered_devices.len(), 16);
    }

    #[test]
    fn test_pci_scan_and_register() {
        let mut scanner = PciBusScanner::new();
        
        // Register a valid device
        let result = scanner.scan_and_register(0, 1, 0x1234, 0x5678, 0x02);
        assert!(result.is_ok());
        
        // Check device was registered
        assert!(scanner.registered_devices[0].is_some());
        let device = scanner.registered_devices[0].unwrap();
        assert_eq!(device.vendor_id, 0x1234);
    }

    #[test]
    fn test_pci_scan_absent_device() {
        let mut scanner = PciBusScanner::new();
        
        // Try to register an absent device (vendor 0xFFFF)
        let result = scanner.scan_and_register(0, 1, 0xFFFF, 0x5678, 0x02);
        assert!(result.is_ok());
        
        // Device should not be registered
        assert!(scanner.registered_devices[0].is_none());
    }

    #[test]
    fn test_pci_scanner_capacity() {
        let mut scanner = PciBusScanner::new();
        
        // Fill all 16 slots
        for i in 0..16 {
            let result = scanner.scan_and_register(0, i as u8, 0x1000 + i as u16, 0x2000, 0x02);
            assert!(result.is_ok());
        }
        
        // Try to register one more device (should fail)
        let result = scanner.scan_and_register(1, 0, 0x9999, 0x8888, 0x02);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Active boot firmware PCI registry full");
    }

    #[test]
    fn test_pci_class_detection() {
        let network_device = PciDevice::new(0, 1, 0x1234, 0x5678, 0x02);
        assert_eq!(network_device.class, PciClass::Network);
        
        let storage_device = PciDevice::new(0, 2, 0x1234, 0x5678, 0x01);
        assert_eq!(storage_device.class, PciClass::Storage);
        
        let display_device = PciDevice::new(0, 3, 0x1234, 0x5678, 0x03);
        assert_eq!(display_device.class, PciClass::Display);
        
        let unknown_device = PciDevice::new(0, 4, 0x1234, 0x5678, 0xFF);
        assert_eq!(unknown_device.class, PciClass::Unknown);
    }
}