#![no_std]
extern crate alloc;
use alloc::vec::Vec;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;
pub const PCIE_ECAM_BASE_ADDRESS: u64 = 0xE000_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcieGenSpeed {
    Gen1_2_5GTs = 1,
    Gen2_5_0GTs = 2,
    Gen3_8_0GTs = 3,
    Gen4_16_0GTs = 4,
    Gen5_32_0GTs = 5,
    Gen6_64_0GTs = 6,
    Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcieLinkInfo {
    pub max_link_speed: PcieGenSpeed,
    pub max_link_width: u8, // e.g. 1, 2, 4, 8, 16, 32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsixCapability {
    pub table_size: u16,
    pub table_bar: u8,
    pub table_offset: u32,
    pub pba_bar: u8,
    pub pba_offset: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcieAerStatus {
    pub uncorrectable_error_status: u32,
    pub correctable_error_status: u32,
    pub aer_control: u32,
}

pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_id: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub header_type: u8,
    pub bars: [u32; 6],
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub msix: Option<MsixCapability>,
    pub link_info: Option<PcieLinkInfo>,
    pub aer: Option<PcieAerStatus>,
}

pub struct PciBus {
    pub devices: Vec<PciDevice>,
    pub ecam_base: u64,
}

impl Default for PciBus {
    fn default() -> Self {
        Self::new()
    }
}

impl PciBus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            ecam_base: PCIE_ECAM_BASE_ADDRESS,
        }
    }

    pub fn set_ecam_base(&mut self, base: u64) {
        self.ecam_base = base;
    }

    pub fn get_ecam_address(&self, bus: u8, slot: u8, func: u8, offset: u16) -> u64 {
        self.ecam_base
            + ((bus as u64) << 20)
            + ((slot as u64) << 15)
            + ((func as u64) << 12)
            + (offset as u64)
    }

    pub fn in32(_port: u16) -> u32 {
        // Stub for x86 in instruction
        0
    }

    pub fn out32(_port: u16, _val: u32) {
        // Stub for x86 out instruction
    }

    pub fn read_config_32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = 0x80000000
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | (offset as u32 & 0xFC);
        Self::out32(CONFIG_ADDRESS, address);
        Self::in32(CONFIG_DATA)
    }

    pub fn enumerate(&mut self) {
        for bus in 0..=255 {
            for slot in 0..32 {
                self.check_device(bus, slot);
            }
        }
    }

    fn check_device(&mut self, bus: u8, slot: u8) {
        let vendor = (Self::read_config_32(bus, slot, 0, 0) & 0xFFFF) as u16;
        if vendor == 0xFFFF {
            return;
        }
        self.check_function(bus, slot, 0);
        let header_type = ((Self::read_config_32(bus, slot, 0, 0x0C) >> 16) & 0xFF) as u8;
        if (header_type & 0x80) != 0 {
            for func in 1..8 {
                let v = (Self::read_config_32(bus, slot, func, 0) & 0xFFFF) as u16;
                if v != 0xFFFF {
                    self.check_function(bus, slot, func);
                }
            }
        }
    }

    fn check_function(&mut self, bus: u8, slot: u8, func: u8) {
        let id_reg = Self::read_config_32(bus, slot, func, 0);
        let vendor_id = (id_reg & 0xFFFF) as u16;
        let device_id = (id_reg >> 16) as u16;

        let class_reg = Self::read_config_32(bus, slot, func, 0x08);
        let revision = (class_reg & 0xFF) as u8;
        let prog_if = ((class_reg >> 8) & 0xFF) as u8;
        let subclass = ((class_reg >> 16) & 0xFF) as u8;
        let class_id = (class_reg >> 24) as u8;

        let header_reg = Self::read_config_32(bus, slot, func, 0x0C);
        let header_type = ((header_reg >> 16) & 0xFF) as u8;

        let int_reg = Self::read_config_32(bus, slot, func, 0x3C);
        let interrupt_line = (int_reg & 0xFF) as u8;
        let interrupt_pin = ((int_reg >> 8) & 0xFF) as u8;

        let mut bars = [0; 6];
        if (header_type & 0x7F) == 0x00 {
            for i in 0..6 {
                bars[i] = Self::read_config_32(bus, slot, func, 0x10 + (i as u8 * 4));
            }
        }

        let msix = Self::parse_msix_capability(bus, slot, func);
        let link_info = Self::parse_pcie_link_capabilities(bus, slot, func);

        self.devices.push(PciDevice {
            bus,
            slot,
            func,
            vendor_id,
            device_id,
            class_id,
            subclass,
            prog_if,
            revision,
            header_type,
            bars,
            interrupt_line,
            interrupt_pin,
            msix,
            link_info,
            aer: None,
        });
    }

    pub fn parse_msix_capability(bus: u8, slot: u8, func: u8) -> Option<MsixCapability> {
        let status = Self::read_config_32(bus, slot, func, 0x04) >> 16;
        if (status & 0x10) != 0 {
            let mut cap_ptr = (Self::read_config_32(bus, slot, func, 0x34) & 0xFF) as u8;
            while cap_ptr != 0 {
                let cap = Self::read_config_32(bus, slot, func, cap_ptr);
                let cap_id = (cap & 0xFF) as u8;
                if cap_id == 0x11 {
                    // MSI-X capability structure
                    let msg_ctrl = (cap >> 16) as u16;
                    let table_size = (msg_ctrl & 0x07FF) + 1;

                    let table_reg = Self::read_config_32(bus, slot, func, cap_ptr + 4);
                    let table_bar = (table_reg & 0x07) as u8;
                    let table_offset = table_reg & 0xFFFF_FFF8;

                    let pba_reg = Self::read_config_32(bus, slot, func, cap_ptr + 8);
                    let pba_bar = (pba_reg & 0x07) as u8;
                    let pba_offset = pba_reg & 0xFFFF_FFF8;

                    return Some(MsixCapability {
                        table_size,
                        table_bar,
                        table_offset,
                        pba_bar,
                        pba_offset,
                    });
                }
                cap_ptr = ((cap >> 8) & 0xFF) as u8;
            }
        }
        None
    }

    pub fn parse_pcie_link_capabilities(bus: u8, slot: u8, func: u8) -> Option<PcieLinkInfo> {
        let status = Self::read_config_32(bus, slot, func, 0x04) >> 16;
        if (status & 0x10) != 0 {
            let mut cap_ptr = (Self::read_config_32(bus, slot, func, 0x34) & 0xFF) as u8;
            while cap_ptr != 0 {
                let cap = Self::read_config_32(bus, slot, func, cap_ptr);
                let cap_id = (cap & 0xFF) as u8;
                if cap_id == 0x10 {
                    // PCI Express Capability Structure
                    let link_cap = Self::read_config_32(bus, slot, func, cap_ptr + 0x0C);
                    let speed_raw = (link_cap & 0x0F) as u8;
                    let width_raw = ((link_cap >> 4) & 0x3F) as u8;

                    let max_link_speed = match speed_raw {
                        1 => PcieGenSpeed::Gen1_2_5GTs,
                        2 => PcieGenSpeed::Gen2_5_0GTs,
                        3 => PcieGenSpeed::Gen3_8_0GTs,
                        4 => PcieGenSpeed::Gen4_16_0GTs,
                        5 => PcieGenSpeed::Gen5_32_0GTs,
                        6 => PcieGenSpeed::Gen6_64_0GTs,
                        _ => PcieGenSpeed::Unknown,
                    };

                    return Some(PcieLinkInfo {
                        max_link_speed,
                        max_link_width: width_raw,
                    });
                }
                cap_ptr = ((cap >> 8) & 0xFF) as u8;
            }
        }
        None
    }

    pub fn parse_msi_capability(&self, dev: &PciDevice) {
        let status = Self::read_config_32(dev.bus, dev.slot, dev.func, 0x04) >> 16;
        if (status & 0x10) != 0 {
            let mut cap_ptr = (Self::read_config_32(dev.bus, dev.slot, dev.func, 0x34) & 0xFF) as u8;
            while cap_ptr != 0 {
                let cap = Self::read_config_32(dev.bus, dev.slot, dev.func, cap_ptr);
                let cap_id = (cap & 0xFF) as u8;
                if cap_id == 0x05 {
                    // MSI capability found
                    break;
                }
                cap_ptr = ((cap >> 8) & 0xFF) as u8;
            }
        }
    }

    pub fn get_vendor_name(vendor_id: u16) -> &'static str {
        match vendor_id {
            0x8086 => "Intel Corporation",
            0x1022 => "Advanced Micro Devices, Inc. [AMD]",
            0x10DE => "NVIDIA Corporation",
            0x1AF4 => "Red Hat, Inc. (Virtio)",
            0x1014 => "IBM",
            0x14E4 => "Broadcom Inc.",
            0x10EC => "Realtek Semiconductor Corp.",
            _ => "Unknown Vendor",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcie_ecam_address_computation() {
        let bus = PciBus::new();
        // Bus 1, Slot 2, Func 3, Offset 0x10
        let addr = bus.get_ecam_address(1, 2, 3, 0x10);
        let expected = PCIE_ECAM_BASE_ADDRESS + (1 << 20) + (2 << 15) + (3 << 12) + 0x10;
        assert_eq!(addr, expected);
    }

    #[test]
    fn test_pcie_gen_speed_decoding() {
        let link_info = PcieLinkInfo {
            max_link_speed: PcieGenSpeed::Gen4_16_0GTs,
            max_link_width: 16,
        };
        assert_eq!(link_info.max_link_speed, PcieGenSpeed::Gen4_16_0GTs);
        assert_eq!(link_info.max_link_width, 16);
    }

    #[test]
    fn test_pci_vendor_name_lookup() {
        assert_eq!(PciBus::get_vendor_name(0x8086), "Intel Corporation");
        assert_eq!(PciBus::get_vendor_name(0x10DE), "NVIDIA Corporation");
        assert_eq!(PciBus::get_vendor_name(0x1AF4), "Red Hat, Inc. (Virtio)");
    }
}
