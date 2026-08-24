#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use core::ptr;

const HBA_PxCMD_ST: u32 = 0x0001;
const HBA_PxCMD_FRE: u32 = 0x0010;
const HBA_PxCMD_FR: u32 = 0x4000;
const HBA_PxCMD_CR: u32 = 0x8000;

#[repr(C)]
pub struct HbaPortRegs {
    pub clb: u32,
    pub clbu: u32,
    pub fb: u32,
    pub fbu: u32,
    pub is: u32,
    pub ie: u32,
    pub cmd: u32,
    pub rsv0: u32,
    pub tfd: u32,
    pub sig: u32,
    pub ssts: u32,
    pub sctl: u32,
    pub serr: u32,
    pub sact: u32,
    pub ci: u32,
    pub sntf: u32,
    pub fbs: u32,
    pub rsv1: [u32; 11],
    pub vendor: [u32; 4],
}

#[repr(C)]
pub struct HbaMem {
    pub cap: u32,
    pub ghc: u32,
    pub is: u32,
    pub pi: u32,
    pub vs: u32,
    pub ccc_ctl: u32,
    pub ccc_pts: u32,
    pub em_loc: u32,
    pub em_ctl: u32,
    pub cap2: u32,
    pub bohc: u32,
    pub rsv: [u8; 116],
    pub vendor: [u8; 96],
    pub ports: [HbaPortRegs; 32],
}

pub struct AhciHba {
    pub abar: *mut HbaMem,
    pub ports_implemented: u32,
}

impl AhciHba {
    pub fn new(abar_addr: u32) -> Self {
        let abar = abar_addr as *mut HbaMem;
        let ports_implemented = unsafe { (*abar).pi };
        Self { abar, ports_implemented }
    }
    
    pub fn init(&mut self) {
        unsafe {
            // Enable AHCI mode and global interrupts
            (*self.abar).ghc |= (1 << 31) | (1 << 1);
        }
    }
    
    pub fn start_cmd(&self, port_idx: usize) {
        unsafe {
            let port = &mut (*self.abar).ports[port_idx];
            while (port.cmd & HBA_PxCMD_CR) != 0 {}
            port.cmd |= HBA_PxCMD_FRE;
            port.cmd |= HBA_PxCMD_ST;
        }
    }
    
    pub fn stop_cmd(&self, port_idx: usize) {
        unsafe {
            let port = &mut (*self.abar).ports[port_idx];
            port.cmd &= !HBA_PxCMD_ST;
            port.cmd &= !HBA_PxCMD_FRE;
            while (port.cmd & (HBA_PxCMD_CR | HBA_PxCMD_FR)) != 0 {}
        }
    }
    
    pub fn identify_device(&self, port_idx: usize) {
        // Construct Command FIS for IDENTIFY
    }
    
    pub fn read_dma(&self, port_idx: usize, lba: u64, count: u16, buf: &mut [u8]) -> Result<(), &'static str> {
        // Command table construction and PRDTs
        Ok(())
    }
}
