use crate::io::Port;
use super::{BusController, BusType};
use crate::drivers::DriverStatus;

const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

pub struct PciBus {
    status: DriverStatus,
}

impl PciBus {
    pub fn new() -> Self {
        Self {
            status: DriverStatus::Uninitialized,
        }
    }

    /// Read a 32-bit value from the PCI configuration space.
    pub unsafe fn read_config_u32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address = 0x80000000 
            | ((bus as u32) << 16) 
            | ((slot as u32) << 11) 
            | ((func as u32) << 8) 
            | (offset as u32 & 0xFC);
        
        // This relies on the I/O port trait we created in Phase 2
        // Since we only made read_u8/write_u8, we need to extend Port for u32 or use raw assembly here.
        // For demonstration of sovereignty, we use direct assembly.
        let mut value: u32;
        core::arch::asm!(
            "out dx, eax",
            in("dx") PCI_CONFIG_ADDRESS,
            in("eax") address,
            options(nomem, nostack, preserves_flags)
        );

        core::arch::asm!(
            "in eax, dx",
            out("eax") value,
            in("dx") PCI_CONFIG_DATA,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    /// Discover devices on the PCI bus.
    pub fn check_device(bus: u8, slot: u8) {
        unsafe {
            let vendor_id = Self::read_config_u32(bus, slot, 0, 0) & 0xFFFF;
            if vendor_id != 0xFFFF {
                // Device exists! In a full implementation, we'd add this to the DriverRegistry.
            }
        }
    }
}

impl BusController for PciBus {
    fn init(&mut self) -> Result<(), &'static str> {
        self.status = DriverStatus::Ready;
        Ok(())
    }

    fn scan(&self) {
        // Scan all 256 buses and 32 slots
        for bus in 0..=255 {
            for slot in 0..32 {
                Self::check_device(bus, slot);
            }
        }
    }

    fn get_type(&self) -> BusType {
        BusType::PCI
    }
}
