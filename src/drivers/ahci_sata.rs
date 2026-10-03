//! # AHCI SATA Driver
//!
//! Advanced Host Controller Interface for SATA drives.
//! Inspired by Linux drivers/ata/ahci.c and FreeBSD sys/dev/ahci/.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// AHCI Generic Host Control registers
pub mod ahci_regs {
    pub const CAP: usize = 0x00;      // Host Capabilities
    pub const GHC: usize = 0x04;      // Global Host Control
    pub const IS: usize = 0x08;       // Interrupt Status
    pub const PI: usize = 0x0C;       // Ports Implemented
    pub const VS: usize = 0x10;       // Version
    pub const CCC_CTL: usize = 0x14;  // Command Completion Coalescing Control
    pub const CCC_PORTS: usize = 0x18; // Command Completion Coalescing Ports
    pub const EM_LOC: usize = 0x1C;   // Enclosure Management Location
    pub const EM_CTL: usize = 0x20;   // Enclosure Management Control
    pub const CAP2: usize = 0x24;     // Host Capabilities Extended
    pub const BOHC: usize = 0x28;     // BIOS/OS Handoff Control and Status
}

/// AHCI Port registers (offset from port base)
pub mod port_regs {
    pub const CLB: usize = 0x00;      // Command List Base Address
    pub const CLBU: usize = 0x04;     // Command List Base Address Upper
    pub const FB: usize = 0x08;       // FIS Base Address
    pub const FBU: usize = 0x0C;      // FIS Base Address Upper
    pub const IS: usize = 0x10;       // Interrupt Status
    pub const IE: usize = 0x14;       // Interrupt Enable
    pub const CMD: usize = 0x18;      // Command and Status
    pub const TFD: usize = 0x20;      // Task File Data
    pub const SIG: usize = 0x24;      // Signature
    pub const SSTS: usize = 0x28;     // SATA Status
    pub const SCTL: usize = 0x2C;     // SATA Control
    pub const SERR: usize = 0x30;     // SATA Error
    pub const SACT: usize = 0x34;     // SATA Active
    pub const CI: usize = 0x38;       // Command Issue
}

/// AHCI GHC bits
pub mod ghc_bits {
    pub const HR: u32 = 1 << 0;       // HBA Reset
    pub const IE: u32 = 1 << 1;       // Interrupt Enable
    pub const AE: u32 = 1 << 31;      // AHCI Enable
}

/// AHCI Port CMD bits
pub mod cmd_bits {
    pub const ST: u32 = 1 << 0;       // Start
    pub const FRE: u32 = 1 << 4;      // FIS Receive Enable
    pub const FR: u32 = 1 << 14;      // FIS Receive Running
    pub const CR: u32 = 1 << 15;      // Command List Running
}

/// ATA command opcodes
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AtaCommand {
    ReadDma = 0xC8,
    ReadDmaExt = 0x25,
    WriteDma = 0xCA,
    WriteDmaExt = 0x35,
    IdentifyDevice = 0xEC,
    SetFeatures = 0xEF,
    FlushCache = 0xE7,
    FlushCacheExt = 0xEA,
}

/// FIS (Frame Information Structure) types
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FisType {
    RegH2D = 0x27,       // Register FIS - host to device
    RegD2H = 0x34,       // Register FIS - device to host
    DmaSetup = 0x41,     // DMA Setup FIS
    Data = 0x46,         // Data FIS
    Bist = 0x58,         // BIST Activate FIS
    PioSetup = 0x5F,     // PIO Setup FIS
    SetDeviceBits = 0xA1, // Set Device Bits FIS
}

/// Register FIS - Host to Device
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FisRegH2D {
    pub fis_type: u8,        // FIS_TYPE_REG_H2D (0x27)
    pub pmport_c: u8,        // Port multiplier (bits 0-3), C bit (bit 7)
    pub command: u8,         // ATA command
    pub feature_low: u8,     // Feature register (low)
    pub lba0: u8,            // LBA bits 0-7
    pub lba1: u8,            // LBA bits 8-15
    pub lba2: u8,            // LBA bits 16-23
    pub device: u8,          // Device register
    pub lba3: u8,            // LBA bits 24-31
    pub lba4: u8,            // LBA bits 32-39
    pub lba5: u8,            // LBA bits 40-47
    pub feature_high: u8,    // Feature register (high)
    pub count_low: u8,       // Count register (low)
    pub count_high: u8,      // Count register (high)
    pub icc: u8,             // Isochronous command completion
    pub control: u8,         // Control register
    pub reserved: [u8; 4],
}

impl FisRegH2D {
    pub fn new(command: AtaCommand, lba: u64, count: u16) -> Self {
        Self {
            fis_type: FisType::RegH2D as u8,
            pmport_c: 0x80,          // Command bit set
            command: command as u8,
            feature_low: 0,
            lba0: (lba & 0xFF) as u8,
            lba1: ((lba >> 8) & 0xFF) as u8,
            lba2: ((lba >> 16) & 0xFF) as u8,
            device: 0x40,            // LBA mode
            lba3: ((lba >> 24) & 0xFF) as u8,
            lba4: ((lba >> 32) & 0xFF) as u8,
            lba5: ((lba >> 40) & 0xFF) as u8,
            feature_high: 0,
            count_low: (count & 0xFF) as u8,
            count_high: ((count >> 8) & 0xFF) as u8,
            icc: 0,
            control: 0,
            reserved: [0; 4],
        }
    }
}

/// AHCI Command Header
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AhciCommandHeader {
    pub flags: u16,              // Various flags
    pub prdtl: u16,              // Physical Region Descriptor Table Length
    pub prdbc: u32,              // Physical Region Descriptor Byte Count
    pub ctba: u32,               // Command Table Base Address (low)
    pub ctbau: u32,              // Command Table Base Address (upper)
    pub reserved: [u32; 4],
}

/// Physical Region Descriptor Table Entry
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AhciPrdt {
    pub dba: u32,                // Data Base Address (low)
    pub dbau: u32,               // Data Base Address (upper)
    pub reserved: u32,
    pub dbc: u32,                // Byte count (bit 0 must be 1, max 4MB)
}

/// AHCI Command Table
#[repr(C)]
pub struct AhciCommandTable {
    pub cfis: [u8; 64],          // Command FIS
    pub acmd: [u8; 16],          // ATAPI command
    pub reserved: [u8; 48],
    pub prdt: [AhciPrdt; 8],     // Physical Region Descriptor Table
}

/// AHCI Port
pub struct AhciPort {
    port_num: u8,
    base_addr: usize,
    command_list: Vec<AhciCommandHeader>,
    command_tables: Vec<AhciCommandTable>,
    next_slot: AtomicU32,
}

impl AhciPort {
    const NUM_COMMAND_SLOTS: usize = 32;
    
    pub fn new(port_num: u8, base_addr: usize) -> Self {
        let mut command_list = alloc::vec::Vec::with_capacity(Self::NUM_COMMAND_SLOTS);
        let mut command_tables = alloc::vec::Vec::with_capacity(Self::NUM_COMMAND_SLOTS);
        for _ in 0..Self::NUM_COMMAND_SLOTS {
            command_list.push(unsafe { core::mem::zeroed() });
            command_tables.push(unsafe { core::mem::zeroed() });
        }
        Self {
            port_num,
            base_addr,
            command_list,
            command_tables,
            next_slot: AtomicU32::new(0),
        }
    }
    
    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.base_addr + offset) as *const u32) }
    }
    
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.base_addr + offset) as *mut u32, val); }
    }
    
    /// Start port
    pub fn start(&self) -> Result<(), AhciError> {
        // Wait for CR to clear
        let mut timeout = 1000;
        while (self.read_reg(port_regs::CMD) & cmd_bits::CR) != 0 && timeout > 0 {
            timeout -= 1;
        }
        
        if timeout == 0 {
            return Err(AhciError::Timeout);
        }
        
        // Set FRE and ST
        let cmd = self.read_reg(port_regs::CMD);
        self.write_reg(port_regs::CMD, cmd | cmd_bits::FRE | cmd_bits::ST);
        
        Ok(())
    }
    
    /// Stop port
    pub fn stop(&self) -> Result<(), AhciError> {
        // Clear ST
        let cmd = self.read_reg(port_regs::CMD);
        self.write_reg(port_regs::CMD, cmd & !cmd_bits::ST);
        
        // Wait for CR to clear
        let mut timeout = 1000;
        while (self.read_reg(port_regs::CMD) & cmd_bits::CR) != 0 && timeout > 0 {
            timeout -= 1;
        }
        
        if timeout == 0 {
            return Err(AhciError::Timeout);
        }
        
        // Clear FRE
        let cmd = self.read_reg(port_regs::CMD);
        self.write_reg(port_regs::CMD, cmd & !cmd_bits::FRE);
        
        Ok(())
    }
    
    /// Issue command
    pub fn issue_command(&mut self, command: AtaCommand, lba: u64, count: u16, buffer: &mut [u8]) -> Result<(), AhciError> {
        let slot = self.next_slot.fetch_add(1, Ordering::SeqCst) % Self::NUM_COMMAND_SLOTS as u32;
        
        // Build command FIS
        let fis = FisRegH2D::new(command, lba, count);
        
        // Setup command table
        let ct = &mut self.command_tables[slot as usize];
        unsafe {
            let fis_bytes = core::slice::from_raw_parts(
                &fis as *const FisRegH2D as *const u8,
                core::mem::size_of::<FisRegH2D>(),
            );
            ct.cfis[..fis_bytes.len()].copy_from_slice(fis_bytes);
        }
        
        // Setup PRDT
        ct.prdt[0].dba = buffer.as_ptr() as u32;
        ct.prdt[0].dbau = ((buffer.as_ptr() as u64) >> 32) as u32;
        ct.prdt[0].dbc = (buffer.len() as u32 - 1) | 1; // Bit 0 must be 1
        
        // Setup command header
        let ch = &mut self.command_list[slot as usize];
        ch.prdtl = 1; // One PRDT entry
        ch.flags = (core::mem::size_of::<FisRegH2D>() / 4) as u16; // FIS length in dwords
        ch.ctba = ct as *const AhciCommandTable as u32;
        ch.ctbau = ((ct as *const AhciCommandTable as u64) >> 32) as u32;
        
        // Issue command
        self.write_reg(port_regs::CI, 1 << slot);
        
        // Wait for completion
        let mut timeout = 10000;
        while (self.read_reg(port_regs::CI) & (1 << slot)) != 0 && timeout > 0 {
            timeout -= 1;
        }
        
        if timeout == 0 {
            return Err(AhciError::Timeout);
        }
        
        // Check for errors
        let is = self.read_reg(port_regs::IS);
        if is & 0x40000000 != 0 {
            return Err(AhciError::TaskFileError);
        }
        
        Ok(())
    }
    
    /// Read sectors
    pub fn read_sectors(&mut self, lba: u64, count: u16, buffer: &mut [u8]) -> Result<(), AhciError> {
        let command = if lba > 0xFFFFFFF {
            AtaCommand::ReadDmaExt
        } else {
            AtaCommand::ReadDma
        };
        
        self.issue_command(command, lba, count, buffer)
    }
    
    /// Write sectors
    pub fn write_sectors(&mut self, lba: u64, count: u16, buffer: &mut [u8]) -> Result<(), AhciError> {
        let command = if lba > 0xFFFFFFF {
            AtaCommand::WriteDmaExt
        } else {
            AtaCommand::WriteDma
        };
        
        self.issue_command(command, lba, count, buffer)
    }
}

/// AHCI Controller
pub struct AhciController {
    abar: usize,                     // AHCI Base Address Register
    ports: Vec<Option<AhciPort>>,
}

impl AhciController {
    pub fn new(abar: usize) -> Self {
        let mut ports = alloc::vec::Vec::with_capacity(32);
        for _ in 0..32 {
            ports.push(None);
        }
        Self {
            abar,
            ports,
        }
    }
    
    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.abar + offset) as *const u32) }
    }
    
    fn write_reg(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.abar + offset) as *mut u32, val); }
    }
    
    /// Initialize controller
    pub fn init(&mut self) -> Result<(), AhciError> {
        // Enable AHCI
        let ghc = self.read_reg(ahci_regs::GHC);
        self.write_reg(ahci_regs::GHC, ghc | ghc_bits::AE);
        
        // Reset HBA
        self.write_reg(ahci_regs::GHC, ghc | ghc_bits::HR);
        
        // Wait for reset to complete
        let mut timeout = 1000;
        while (self.read_reg(ahci_regs::GHC) & ghc_bits::HR) != 0 && timeout > 0 {
            timeout -= 1;
        }
        
        if timeout == 0 {
            return Err(AhciError::Timeout);
        }
        
        // Re-enable AHCI
        self.write_reg(ahci_regs::GHC, ghc | ghc_bits::AE);
        
        // Probe ports
        let pi = self.read_reg(ahci_regs::PI);
        for i in 0..32 {
            if pi & (1 << i) != 0 {
                let port_base = self.abar + 0x100 + (i * 0x80);
                let port = AhciPort::new(i as u8, port_base);
                self.ports[i] = Some(port);
            }
        }
        
        Ok(())
    }
    
    /// Get port by number
    pub fn get_port(&mut self, port_num: u8) -> Option<&mut AhciPort> {
        self.ports[port_num as usize].as_mut()
    }
}

/// AHCI errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AhciError {
    Timeout,
    TaskFileError,
    NoDevice,
    InvalidPort,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fis_construction() {
        let fis = FisRegH2D::new(AtaCommand::ReadDma, 0x1234, 1);
        assert_eq!(fis.fis_type, FisType::RegH2D as u8);
        assert_eq!(fis.command, AtaCommand::ReadDma as u8);
        assert_eq!(fis.lba0, 0x34);
        assert_eq!(fis.lba1, 0x12);
    }
}
