// S-BOOT FIRMWARE (PCI & PCIe Express Bus Scanner)
// BIOS, UEFI & PCIe ECAM (Enhanced Configuration Access Mechanism) Specification compliance

#[cfg(not(feature = "standalone_test"))]
use alloc::vec::Vec;

#[cfg(feature = "standalone_test")]
extern crate alloc;
#[cfg(feature = "standalone_test")]
use alloc::vec::Vec;

pub const PCI_MAX_BUS: usize = 256;
pub const PCI_MAX_DEVICE: u8 = 32;
pub const PCI_MAX_FUNCTION: u8 = 8;
pub const PCIE_ECAM_OFFSET_PER_BUS: usize = 1 << 20; // 1MB per bus

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciClass {
    Storage = 0x01,
    Network = 0x02,
    Display = 0x03,
    Multimedia = 0x04,
    Memory = 0x05,
    Bridge = 0x06,
    Communication = 0x07,
    SystemPeripheral = 0x08,
    InputDevice = 0x09,
    SerialBus = 0x0C,
    Wireless = 0x0D,
    Crypto = 0x10,
    Unknown = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarType {
    Memory32 { prefetchable: bool },
    Memory64 { prefetchable: bool },
    IoSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciBar {
    pub bar_index: u8,
    pub bar_type: BarType,
    pub address: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciCapabilityId {
    PowerManagement,
    Msi,
    Pcie,
    MsiX,
    VendorSpecific,
    Unknown(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciCapability {
    pub id: PciCapabilityId,
    pub offset: u8,
}

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: PciClass,
    pub bars: Vec<PciBar>,
    pub capabilities: Vec<PciCapability>,
}

impl PciDevice {
    pub fn new(bus: u8, slot: u8, vendor: u16, device: u16, class_code: u8) -> Self {
        Self::new_function(bus, slot, 0, vendor, device, class_code)
    }

    pub fn new_function(
        bus: u8,
        slot: u8,
        function: u8,
        vendor: u16,
        device: u16,
        class_code: u8,
    ) -> Self {
        let class = match class_code {
            0x01 => PciClass::Storage,
            0x02 => PciClass::Network,
            0x03 => PciClass::Display,
            0x04 => PciClass::Multimedia,
            0x05 => PciClass::Memory,
            0x06 => PciClass::Bridge,
            0x07 => PciClass::Communication,
            0x08 => PciClass::SystemPeripheral,
            0x09 => PciClass::InputDevice,
            0x0C => PciClass::SerialBus,
            0x0D => PciClass::Wireless,
            0x10 => PciClass::Crypto,
            _ => PciClass::Unknown,
        };
        Self {
            bus,
            slot,
            function,
            vendor_id: vendor,
            device_id: device,
            class,
            bars: Vec::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn decode_bar(&mut self, index: u8, raw_bar0: u32, raw_bar1: Option<u32>, size: u64) {
        if raw_bar0 == 0 || raw_bar0 == 0xFFFF_FFFF {
            return;
        }

        let is_io = (raw_bar0 & 0x01) != 0;
        if is_io {
            let addr = (raw_bar0 & !0x03) as u64;
            self.bars.push(PciBar {
                bar_index: index,
                bar_type: BarType::IoSpace,
                address: addr,
                size,
            });
        } else {
            let is_64bit = ((raw_bar0 >> 1) & 0x03) == 0x02;
            let prefetchable = (raw_bar0 & 0x08) != 0;

            let addr = if is_64bit {
                let high = raw_bar1.unwrap_or(0) as u64;
                let low = (raw_bar0 & !0x0F) as u64;
                (high << 32) | low
            } else {
                (raw_bar0 & !0x0F) as u64
            };

            let bar_type = if is_64bit {
                BarType::Memory64 { prefetchable }
            } else {
                BarType::Memory32 { prefetchable }
            };

            self.bars.push(PciBar {
                bar_index: index,
                bar_type,
                address: addr,
                size,
            });
        }
    }

    pub fn add_capability(&mut self, id_raw: u8, offset: u8) {
        let id = match id_raw {
            0x01 => PciCapabilityId::PowerManagement,
            0x05 => PciCapabilityId::Msi,
            0x09 => PciCapabilityId::VendorSpecific,
            0x10 => PciCapabilityId::Pcie,
            0x11 => PciCapabilityId::MsiX,
            other => PciCapabilityId::Unknown(other),
        };
        self.capabilities.push(PciCapability { id, offset });
    }
}

pub struct PcieEcamManager {
    pub base_mmio_address: u64,
}

impl PcieEcamManager {
    pub fn new(ecam_base: u64) -> Self {
        Self { base_mmio_address: ecam_base }
    }

    pub fn calculate_function_offset(&self, bus: u8, slot: u8, func: u8) -> u64 {
        self.base_mmio_address
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
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

impl Default for PciBusScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcie_ecam_addressing() {
        let ecam = PcieEcamManager::new(0xE000_0000);
        let addr = ecam.calculate_function_offset(1, 2, 3);
        // Base + 1MB*1 + 32KB*2 + 4KB*3 = 0xE000_0000 + 0x100000 + 0x10000 + 0x3000 = 0xE011_3000
        assert_eq!(addr, 0xE000_0000 + 0x100000 + 0x10000 + 0x3000);
    }

    #[test]
    fn test_pci_bar_decoding_mem64() {
        let mut dev = PciDevice::new(0, 1, 0x8086, 0x10D3, 0x02);
        // BAR0 raw memory 64-bit prefetchable at low 0xFE000008, high 0x00000001
        dev.decode_bar(0, 0xFE00000C, Some(0x00000001), 65536);
        assert_eq!(dev.bars.len(), 1);
        assert_eq!(dev.bars[0].address, 0x0000_0001_FE00_0000);
        assert_eq!(
            dev.bars[0].bar_type,
            BarType::Memory64 { prefetchable: true }
        );
    }

    #[test]
    fn test_pci_capabilities_msix() {
        let mut dev = PciDevice::new(0, 2, 0x10DE, 0x2204, 0x03);
        dev.add_capability(0x11, 0x60); // MSI-X at offset 0x60
        dev.add_capability(0x10, 0x70); // PCIe Capability at offset 0x70

        assert_eq!(dev.capabilities.len(), 2);
        assert_eq!(dev.capabilities[0].id, PciCapabilityId::MsiX);
        assert_eq!(dev.capabilities[1].id, PciCapabilityId::Pcie);
    }

    #[test]
    fn test_pci_scanner_capacity() {
        let mut scanner = PciBusScanner::new();
        for i in 0..16 {
            assert!(scanner
                .scan_and_register(0, i as u8, 0x1000 + i as u16, 0x2000, 0x02)
                .is_ok());
        }
        assert!(scanner.scan_and_register(1, 0, 0x9999, 0x8888, 0x02).is_err());
    }
}
