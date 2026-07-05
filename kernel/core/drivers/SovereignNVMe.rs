#![no_std]
#![allow(dead_code)]

/// SigmaOS Sovereign NVMe Driver
/// A high-performance, `no_std`, `no_alloc` PCIe NVMe driver stub using static arrays.
/// Implements NVMe Base Specification 1.4 register offsets and submission queues.

use core::sync::atomic::{AtomicU32, Ordering};
use core::ptr::{read_volatile, write_volatile};

// ── NVMe Register Offsets ────────────────────────────────────────────────
const NVME_REG_CAP:  usize = 0x00; // Controller Capabilities
const NVME_REG_VS:   usize = 0x08; // Version
const NVME_REG_INTMS:usize = 0x0C; // Interrupt Mask Set
const NVME_REG_INTMC:usize = 0x10; // Interrupt Mask Clear
const NVME_REG_CC:   usize = 0x14; // Controller Configuration
const NVME_REG_CSTS: usize = 0x1C; // Controller Status
const NVME_REG_AQA:  usize = 0x24; // Admin Queue Attributes
const NVME_REG_ASQ:  usize = 0x28; // Admin Submission Queue Base Address
const NVME_REG_ACQ:  usize = 0x30; // Admin Completion Queue Base Address

// CC (Controller Configuration) Bitmasks
const CC_EN: u32 = 1 << 0; // Enable
const CC_CSS_NVM: u32 = 0 << 4; // NVM Command Set
const CC_IOSQES_64: u32 = 6 << 16; // I/O Submission Queue Entry Size (2^6 = 64)
const CC_IOCQES_16: u32 = 4 << 20; // I/O Completion Queue Entry Size (2^4 = 16)

// CSTS (Controller Status) Bitmasks
const CSTS_RDY: u32 = 1 << 0; // Ready

// ── NVMe Command Structs ──────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct NvmeCmd {
    pub opcode: u8,
    pub flags: u8,
    pub cid: u16,
    pub nsid: u32,
    pub rsvd2: u64,
    pub mptr: u64,
    pub prp1: u64,
    pub prp2: u64,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct NvmeCompletion {
    pub cdw0: u32,
    pub rsvd1: u32,
    pub sqhead: u16,
    pub sqid: u16,
    pub cid: u16,
    pub status: u16,
}

// ── Fixed Queue Sizes ─────────────────────────────────────────────────────
const ASQ_SIZE: usize = 32;
const ACQ_SIZE: usize = 32;
const MAX_ASYNC_QUEUE_DEPTH: usize = 32;

pub struct SovereignNVMe {
    bar0: usize,
    mmio_base: u64,
    async_enabled: bool,
    asq: [NvmeCmd; ASQ_SIZE],
    acq: [NvmeCompletion; ACQ_SIZE],
    iosq: [NvmeCmd; ASQ_SIZE],
    iocq: [NvmeCompletion; ACQ_SIZE],
    asq_tail: u16,
    acq_head: u16,
    iosq_tail: u16,
    iocq_head: u16,
    initialized: bool,
}

impl SovereignNVMe {
    pub const fn new() -> Self {
        Self {
            bar0: 0,
            mmio_base: 0,
            async_enabled: false,
            asq: [NvmeCmd {
                opcode: 0, flags: 0, cid: 0, nsid: 0, rsvd2: 0, mptr: 0,
                prp1: 0, prp2: 0, cdw10: 0, cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0
            }; ASQ_SIZE],
            acq: [NvmeCompletion { cdw0: 0, rsvd1: 0, sqhead: 0, sqid: 0, cid: 0, status: 0 }; ACQ_SIZE],
            iosq: [NvmeCmd {
                opcode: 0, flags: 0, cid: 0, nsid: 0, rsvd2: 0, mptr: 0,
                prp1: 0, prp2: 0, cdw10: 0, cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0
            }; ASQ_SIZE],
            iocq: [NvmeCompletion { cdw0: 0, rsvd1: 0, sqhead: 0, sqid: 0, cid: 0, status: 0 }; ACQ_SIZE],
            asq_tail: 0,
            acq_head: 0,
            iosq_tail: 0,
            iocq_head: 0,
            initialized: false,
        }
    }

    /// Read a 32-bit MMIO register.
    unsafe fn read_reg32(&self, offset: usize) -> u32 {
        read_volatile((self.bar0 + offset) as *const u32)
    }

    /// Write a 32-bit MMIO register.
    unsafe fn write_reg32(&mut self, offset: usize, val: u32) {
        write_volatile((self.bar0 + offset) as *mut u32, val);
    }

    /// Write a 64-bit MMIO register.
    unsafe fn write_reg64(&mut self, offset: usize, val: u64) {
        write_volatile((self.bar0 + offset) as *mut u64, val);
    }

    /// Initialize the NVMe controller.
    pub fn init(&mut self, bar0_addr: usize) -> bool {
        self.bar0 = bar0_addr;
        unsafe {
            // 1. Disable the controller if enabled
            let mut cc = self.read_reg32(NVME_REG_CC);
            if (cc & CC_EN) != 0 {
                cc &= !CC_EN;
                self.write_reg32(NVME_REG_CC, cc);
                // Wait for CSTS.RDY to become 0
                while (self.read_reg32(NVME_REG_CSTS) & CSTS_RDY) != 0 {}
            }

            // 2. Set Admin Queue Attributes (AQA)
            let aqa = ((ACQ_SIZE as u32 - 1) << 16) | (ASQ_SIZE as u32 - 1);
            self.write_reg32(NVME_REG_AQA, aqa);

            // 3. Set Admin Queue Base Addresses (Physical Address)
            // Note: In a real kernel, this needs virt-to-phys conversion.
            let asq_phys = self.asq.as_ptr() as u64;
            let acq_phys = self.acq.as_ptr() as u64;
            self.write_reg64(NVME_REG_ASQ, asq_phys);
            self.write_reg64(NVME_REG_ACQ, acq_phys);

            // 4. Enable Controller with NVM Command Set and standard Queue Entry Sizes
            cc |= CC_EN | CC_CSS_NVM | CC_IOSQES_64 | CC_IOCQES_16;
            self.write_reg32(NVME_REG_CC, cc);

            // 5. Wait for CSTS.RDY to become 1
            while (self.read_reg32(NVME_REG_CSTS) & CSTS_RDY) == 0 {}
        }

        self.initialized = true;
        true
    }

    pub fn init_async(&mut self, enable: bool, mmio_base: u64) {
        self.async_enabled = enable;
        self.mmio_base = mmio_base;
    }

    /// Submit an Admin command.
    pub fn submit_admin_cmd(&mut self, cmd: NvmeCmd) {
        if !self.initialized { return; }
        let tail = self.asq_tail as usize;
        self.asq[tail] = cmd;
        self.asq_tail = (self.asq_tail + 1) % (ASQ_SIZE as u16);
        
        // Ring doorbell for ASQ (Offset 0x1000 + SQID * 8)
        let doorbell_offset = 0x1000; 
        unsafe {
            self.write_reg32(doorbell_offset, self.asq_tail as u32);
        }
    }

    /// Submit an I/O command (Read/Write)
    pub fn submit_io_cmd(&mut self, cmd: NvmeCmd) {
        if !self.initialized { return; }
        let tail = self.iosq_tail as usize;
        self.iosq[tail] = cmd;
        self.iosq_tail = (self.iosq_tail + 1) % (ASQ_SIZE as u16);
        
        // Ring doorbell for I/O SQ (SQID 1)
        // Offset: 0x1000 + (1 * 2 * DoorbellStride). Assuming stride=0, so 0x1000 + 8 = 0x1008
        let doorbell_offset = 0x1008; 
        unsafe {
            self.write_reg32(doorbell_offset, self.iosq_tail as u32);
        }
    }

    /// Poll for I/O completion. In a real implementation this would check the phase bit.
    pub fn poll_io_completion(&mut self) -> bool {
        if !self.initialized { return false; }
        let head = self.iocq_head as usize;
        let comp = &self.iocq[head];
        
        // Simulated phase bit check (status != 0 means completed for this stub)
        if comp.status != 0 {
            self.iocq_head = (self.iocq_head + 1) % (ACQ_SIZE as u16);
            // Ring doorbell for I/O CQ (CQID 1)
            // Offset: 0x1000 + (1 * 2 * DoorbellStride + DoorbellStride) = 0x1000 + 8 + 4 = 0x100C
            let doorbell_offset = 0x100C;
            unsafe {
                self.write_reg32(doorbell_offset, self.iocq_head as u32);
            }
            return true; // Completed
        }
        false // Not completed yet
    }
}

static mut G_NVME: SovereignNVMe = SovereignNVMe::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_nvme_init(bar0: usize) -> i32 {
    if G_NVME.init(bar0) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_nvme_identify() -> i32 {
    let mut cmd = NvmeCmd::default();
    cmd.opcode = 0x06; // Identify
    cmd.nsid = 0;      // Controller ID
    cmd.prp1 = 0x200000; // Static dummy physical address for identify buffer
    G_NVME.submit_admin_cmd(cmd);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_nvme_read(lba: u64, block_count: u16, prp1: u64) -> i32 {
    let mut cmd = NvmeCmd::default();
    cmd.opcode = 0x02; // Read
    cmd.nsid = 1;      // Namespace 1
    cmd.prp1 = prp1;
    cmd.cdw10 = (lba & 0xFFFFFFFF) as u32; // SLBA lower 32
    cmd.cdw11 = (lba >> 32) as u32;        // SLBA upper 32
    cmd.cdw12 = (block_count as u32) | (1 << 30); // block count + Force Unit Access
    
    G_NVME.submit_io_cmd(cmd);
    
    // Simulate polling for completion
    let mut spins = 0;
    while !G_NVME.poll_io_completion() && spins < 1000 {
        spins += 1;
    }
    
    if spins >= 1000 { -1 } else { 0 }
}
