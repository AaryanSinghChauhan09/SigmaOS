#![no_std]
#![allow(dead_code)]

/// SigmaOS PCI Subsystem
/// Zero-allocation PCI bus enumeration and configuration space access.

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

const MAX_PCI_DEVICES: usize = 32;

#[derive(Copy, Clone, Default)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_id: u8,
    pub subclass_id: u8,
}

pub struct PciBus {
    pub devices: [PciDevice; MAX_PCI_DEVICES],
    pub device_count: usize,
}

impl PciBus {
    pub const fn new() -> Self {
        Self {
            devices: [PciDevice {
                bus: 0, slot: 0, func: 0, 
                vendor_id: 0, device_id: 0, class_id: 0, subclass_id: 0
            }; MAX_PCI_DEVICES],
            device_count: 0,
        }
    }

    /// Read 32-bit register from PCI Configuration Space
    unsafe fn pci_config_read_dword(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = 1 << 31
            | ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | (offset as u32 & 0xFC);
        
        // In a real x86 OS, use `outl` and `inl` here.
        // extern "C" { 
        //     fn outl(port: u16, val: u32);
        //     fn inl(port: u16) -> u32;
        // }
        // outl(CONFIG_ADDRESS, address);
        // inl(CONFIG_DATA)
        
        0xFFFFFFFF // Stub return
    }

    pub fn enumerate_bus(&mut self) {
        // Simple brute-force enumeration of bus 0 for demonstration.
        for slot in 0..32 {
            // Read Vendor ID (Offset 0)
            let vendor_dev = unsafe { Self::pci_config_read_dword(0, slot, 0, 0) };
            let vendor_id = (vendor_dev & 0xFFFF) as u16;
            
            if vendor_id != 0xFFFF {
                let device_id = (vendor_dev >> 16) as u16;
                
                // Read Class Code (Offset 8)
                let class_reg = unsafe { Self::pci_config_read_dword(0, slot, 0, 8) };
                let class_id = (class_reg >> 24) as u8;
                let subclass_id = (class_reg >> 16) as u8;

                if self.device_count < MAX_PCI_DEVICES {
                    self.devices[self.device_count] = PciDevice {
                        bus: 0,
                        slot,
                        func: 0,
                        vendor_id,
                        device_id,
                        class_id,
                        subclass_id,
                    };
                    self.device_count += 1;
                }
            }
        }
    }
}

static mut G_PCI_BUS: PciBus = PciBus::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_pci_init() {
    G_PCI_BUS.enumerate_bus();
}
