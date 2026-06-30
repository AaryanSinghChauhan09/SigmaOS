// nvme_core.rs: NVMe Block Device Driver for SigmaOS
// Inspired by Linux drivers/nvme/host/pci.c

#![no_std]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::{read_volatile, write_volatile};

/// NVMe Memory-Mapped Registers (BAR0)
#[repr(C)]
pub struct NvmeRegisters {
    pub cap: u64,     // Controller Capabilities
    pub vs: u32,      // Version
    pub intms: u32,   // Interrupt Mask Set
    pub intmc: u32,   // Interrupt Mask Clear
    pub cc: u32,      // Controller Configuration
    pub rsvd1: u32,
    pub csts: u32,    // Controller Status
    pub nssr: u32,    // NVM Subsystem Reset
    pub aqa: u32,     // Admin Queue Attributes
    pub asq: u64,     // Admin Submission Queue Base Address
    pub acq: u64,     // Admin Completion Queue Base Address
}

/// NVMe Command (Submission Queue Entry - 64 Bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NvmeCmd {
    pub opcode: u8,
    pub flags: u8,
    pub cid: u16,
    pub nsid: u32,
    pub rsvd2: u64,
    pub metadata: u64,
    pub prp1: u64,
    pub prp2: u64,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
}

/// NVMe Completion (Completion Queue Entry - 16 Bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NvmeCompletion {
    pub result: u32,
    pub rsvd: u32,
    pub sq_head: u16,
    pub sq_id: u16,
    pub cid: u16,
    pub status: u16,
}

const CC_ENABLE: u32 = 1 << 0;
const CSTS_READY: u32 = 1 << 0;

pub struct NvmeController {
    regs: *mut NvmeRegisters,
    doorbell_stride: u32,
    admin_sq: Vec<NvmeCmd>,
    admin_cq: Vec<NvmeCompletion>,
    io_sq: Vec<NvmeCmd>,
    io_cq: Vec<NvmeCompletion>,
    admin_sq_tail: u16,
    admin_cq_head: u16,
    io_sq_tail: u16,
    io_cq_head: u16,
}

impl NvmeController {
    pub fn new(bar_address: usize) -> Self {
        // Assume QEMU generic doorbell stride (CAP.DSTRD usually determines this, simplifying for MVP)
        Self {
            regs: bar_address as *mut NvmeRegisters,
            doorbell_stride: 4, 
            admin_sq: alloc::vec![NvmeCmd { opcode: 0, flags: 0, cid: 0, nsid: 0, rsvd2: 0, metadata: 0, prp1: 0, prp2: 0, cdw10: 0, cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0 }; 64],
            admin_cq: alloc::vec![NvmeCompletion { result: 0, rsvd: 0, sq_head: 0, sq_id: 0, cid: 0, status: 0 }; 64],
            io_sq: alloc::vec![NvmeCmd { opcode: 0, flags: 0, cid: 0, nsid: 0, rsvd2: 0, metadata: 0, prp1: 0, prp2: 0, cdw10: 0, cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0 }; 256],
            io_cq: alloc::vec![NvmeCompletion { result: 0, rsvd: 0, sq_head: 0, sq_id: 0, cid: 0, status: 0 }; 256],
            admin_sq_tail: 0,
            admin_cq_head: 0,
            io_sq_tail: 0,
            io_cq_head: 0,
        }
    }

    /// Initialize Admin Queues and Controller State Machine
    pub fn init(&mut self) -> Result<(), &'static str> {
        unsafe {
            // 1. Disable the controller
            let mut cc = read_volatile(&mut (*self.regs).cc);
            cc &= !CC_ENABLE;
            write_volatile(&mut (*self.regs).cc, cc);

            // Wait for CSTS.RDY == 0
            while (read_volatile(&mut (*self.regs).csts) & CSTS_READY) != 0 {
                // busy wait (poll)
            }

            // 2. Configure Admin Queue Attributes (AQA) - 64 entries (0-based, so 63)
            let aqa = (63 << 16) | 63;
            write_volatile(&mut (*self.regs).aqa, aqa);

            // 3. Set Admin Queue Base Addresses (ASQ / ACQ)
            // Note: In a real system, these must be physical DMA addresses. 
            // For MVP, we pass the pointer address assuming identity mapping in QEMU stub.
            write_volatile(&mut (*self.regs).asq, self.admin_sq.as_ptr() as u64);
            write_volatile(&mut (*self.regs).acq, self.admin_cq.as_ptr() as u64);

            // 4. Enable the controller
            cc = read_volatile(&mut (*self.regs).cc);
            cc |= CC_ENABLE;
            // Set Command Set to NVM (0), IOSQES=6 (64B), IOCQES=4 (16B)
            cc |= (6 << 16) | (4 << 20); 
            write_volatile(&mut (*self.regs).cc, cc);

            // Wait for CSTS.RDY == 1
            while (read_volatile(&mut (*self.regs).csts) & CSTS_READY) == 0 {
                // busy wait
            }

            // At this point Admin Queues are ready. We would issue Identify Controller here.
        }

        Ok(())
    }

    /// Helper to ring a doorbell register
    unsafe fn ring_doorbell(&self, qid: u32, is_sq: bool, value: u16) {
        let base = self.regs as usize + 0x1000;
        let offset = qid * (2 * self.doorbell_stride);
        let final_offset = if is_sq { offset } else { offset + self.doorbell_stride };
        
        let db_ptr = (base + final_offset as usize) as *mut u32;
        write_volatile(db_ptr, value as u32);
    }

    /// Build and submit an I/O Read Command
    pub fn read_blocks(&mut self, start_lba: u64, count: u32, buffer_addr: u64) -> Result<(), &'static str> {
        let cmd = NvmeCmd {
            opcode: 0x02, // Read
            flags: 0,
            cid: self.io_sq_tail,
            nsid: 1,      // Namespace 1
            rsvd2: 0,
            metadata: 0,
            prp1: buffer_addr, // DMA destination buffer
            prp2: 0,
            cdw10: (start_lba & 0xFFFFFFFF) as u32,
            cdw11: (start_lba >> 32) as u32,
            cdw12: count - 1, // 0-based
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        };

        // Enqueue to Submission Queue
        self.io_sq[self.io_sq_tail as usize] = cmd;
        self.io_sq_tail = (self.io_sq_tail + 1) % 256;

        unsafe {
            // Ring SQ1 Tail Doorbell (QID 1)
            self.ring_doorbell(1, true, self.io_sq_tail);
        }

        // Poll Completion Queue (MVP strategy)
        loop {
            let comp = self.io_cq[self.io_cq_head as usize];
            // Phase tag polling determines if new entry is here
            // (Assuming simplistic non-phase tag polling for skeleton)
            if comp.cid == cmd.cid {
                // Found our completion
                self.io_cq_head = (self.io_cq_head + 1) % 256;
                unsafe {
                    // Ring CQ1 Head Doorbell (QID 1)
                    self.ring_doorbell(1, false, self.io_cq_head);
                }
                
                // Check status
                let status_field = comp.status >> 1;
                if status_field == 0 {
                    return Ok(());
                } else {
                    return Err("NVMe Read Failed with non-zero status");
                }
            }
        }
    }

    /// Build and submit an I/O Write Command
    pub fn write_blocks(&mut self, start_lba: u64, count: u32, buffer_addr: u64) -> Result<(), &'static str> {
        let cmd = NvmeCmd {
            opcode: 0x01, // Write
            flags: 0,
            cid: self.io_sq_tail,
            nsid: 1,      
            rsvd2: 0,
            metadata: 0,
            prp1: buffer_addr, // DMA source buffer
            prp2: 0,
            cdw10: (start_lba & 0xFFFFFFFF) as u32,
            cdw11: (start_lba >> 32) as u32,
            cdw12: count - 1, 
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        };

        self.io_sq[self.io_sq_tail as usize] = cmd;
        self.io_sq_tail = (self.io_sq_tail + 1) % 256;

        unsafe {
            self.ring_doorbell(1, true, self.io_sq_tail);
        }

        loop {
            let comp = self.io_cq[self.io_cq_head as usize];
            if comp.cid == cmd.cid {
                self.io_cq_head = (self.io_cq_head + 1) % 256;
                unsafe {
                    self.ring_doorbell(1, false, self.io_cq_head);
                }
                
                let status_field = comp.status >> 1;
                if status_field == 0 {
                    return Ok(());
                } else {
                    return Err("NVMe Write Failed with non-zero status");
                }
            }
        }
    }
}
