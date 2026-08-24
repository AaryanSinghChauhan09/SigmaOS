// #![no_std]
extern crate alloc;
use alloc::vec::Vec;

pub const CONFIG_ADDRESS: u16 = 0xCF8;
pub const CONFIG_DATA: u16 = 0xCFC;

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
}

pub struct PciBus {
    pub devices: Vec<PciDevice>,
}

impl PciBus {
    pub fn new() -> Self {
        Self { devices: Vec::new() }
    }

    pub fn in32(port: u16) -> u32 {
        // Stub for x86 in instruction
        0
    }
    
    pub fn out32(port: u16, val: u32) {
        // Stub for x86 out instruction
    }

    pub fn read_config_32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = 0x80000000 | ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | (offset as u32 & 0xFC);
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
        if vendor == 0xFFFF { return; }
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
        
        self.devices.push(PciDevice {
            bus, slot, func, vendor_id, device_id, class_id, subclass, prog_if, revision,
            header_type, bars, interrupt_line, interrupt_pin
        });
    }

    pub fn parse_msi_capability(&self, dev: &PciDevice) {
        // Find MSI capabilities
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
}
