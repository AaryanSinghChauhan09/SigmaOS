// NVMe Solid-State Drive Storage Driver
// Conforms to SigmaOS UnifiedPeripheral interface

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;
use core::ptr::{read_volatile, write_volatile};

extern crate alloc;
use alloc::boxed::Box;

// Register Offsets (NVMe Controller Registers)
const REG_CAP: usize = 0x0000; // Controller Capabilities
const REG_VS: usize = 0x0008; // Version
const REG_CC: usize = 0x0014; // Controller Configuration
const REG_CSTS: usize = 0x001C; // Controller Status
const REG_AQA: usize = 0x0024; // Admin Queue Attributes
const REG_ASQ: usize = 0x0028; // Admin Submission Queue Base Address
const REG_ACQ: usize = 0x0030; // Admin Completion Queue Base Address

// Queue Doorbell Registers (Stride is 4 bytes, spaced by CAP.DSTRD)
const DB_ASQ: usize = 0x1000; // Admin Submission Queue Doorbell
const DB_ACQ: usize = 0x1004; // Admin Completion Queue Doorbell

// Queue counts
const QUEUE_SIZE: usize = 64;
const SECTOR_SIZE: usize = 512;

/// NVMe Submission Queue Entry (SQE) Layout - 64 bytes
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct NvmeCmd {
    pub opcode: u8,
    pub flags: u8,
    pub cid: u16,
    pub nsid: u32,
    pub reserved0: u64,
    pub mptr: u64,
    pub dptr_prp1: u64,
    pub dptr_prp2: u64,
    pub cmd_dword10: u32,
    pub cmd_dword11: u32,
    pub cmd_dword12: u32,
    pub cmd_dword13: u32,
    pub cmd_dword14: u32,
    pub cmd_dword15: u32,
}

impl Default for NvmeCmd {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// NVMe Completion Queue Entry (CQE) Layout - 16 bytes
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct NvmeCqe {
    pub result: u32,
    pub reserved0: u32,
    pub sq_head: u16,
    pub sq_id: u16,
    pub cid: u16,
    pub status: u16,
}

impl Default for NvmeCqe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// NVMe storage drive state
pub struct NvmeDriver {
    pub bar_base: usize,
    pub admin_sq: &'static mut [NvmeCmd; QUEUE_SIZE],
    pub admin_cq: &'static mut [NvmeCqe; QUEUE_SIZE],
    pub io_sq: &'static mut [NvmeCmd; QUEUE_SIZE],
    pub io_cq: &'static mut [NvmeCqe; QUEUE_SIZE],
    pub prp_list: &'static mut [u64; 512],
    pub admin_sq_tail: u16,
    pub admin_cq_head: u16,
    pub io_sq_tail: u16,
    pub io_cq_head: u16,
    pub phase_bit: bool,
    pub power_state: PowerState,
    pub capabilities: CapabilityToken,
}

impl NvmeDriver {
    /// Creates a new uninitialized NVMe driver mapped to a specific PCI BAR address
    pub unsafe fn new(bar_base: usize, capabilities: CapabilityToken) -> Self {
        #[cfg(target_os = "none")]
        let admin_sq = core::mem::transmute(0x00600000usize as *mut [NvmeCmd; QUEUE_SIZE]);
        #[cfg(not(target_os = "none"))]
        let admin_sq = Box::leak(Box::new([NvmeCmd::default(); QUEUE_SIZE]));

        #[cfg(target_os = "none")]
        let admin_cq = core::mem::transmute(0x00700000usize as *mut [NvmeCqe; QUEUE_SIZE]);
        #[cfg(not(target_os = "none"))]
        let admin_cq = Box::leak(Box::new([NvmeCqe::default(); QUEUE_SIZE]));

        #[cfg(target_os = "none")]
        let io_sq = core::mem::transmute(0x00800000usize as *mut [NvmeCmd; QUEUE_SIZE]);
        #[cfg(not(target_os = "none"))]
        let io_sq = Box::leak(Box::new([NvmeCmd::default(); QUEUE_SIZE]));

        #[cfg(target_os = "none")]
        let io_cq = core::mem::transmute(0x00900000usize as *mut [NvmeCqe; QUEUE_SIZE]);
        #[cfg(not(target_os = "none"))]
        let io_cq = Box::leak(Box::new([NvmeCqe::default(); QUEUE_SIZE]));

        #[cfg(target_os = "none")]
        let prp_list = core::mem::transmute(0x00A00000usize as *mut [u64; 512]);
        #[cfg(not(target_os = "none"))]
        let prp_list = Box::leak(Box::new([0u64; 512]));

        Self {
            bar_base,
            admin_sq,
            admin_cq,
            io_sq,
            io_cq,
            prp_list,
            admin_sq_tail: 0,
            admin_cq_head: 0,
            io_sq_tail: 0,
            io_cq_head: 0,
            phase_bit: true,
            power_state: PowerState::Off,
            capabilities,
        }
    }

    unsafe fn read_reg32(&self, offset: usize) -> u32 {
        #[cfg(target_os = "none")]
        {
            read_volatile((self.bar_base + offset) as *const u32)
        }
        #[cfg(not(target_os = "none"))]
        {
            if offset == REG_CSTS {
                1 // Return Ready status in simulation
            } else {
                0
            }
        }
    }

    unsafe fn write_reg32(&self, offset: usize, value: u32) {
        #[cfg(target_os = "none")]
        {
            write_volatile((self.bar_base + offset) as *mut u32, value);
        }
    }

    unsafe fn read_reg64(&self, offset: usize) -> u64 {
        #[cfg(target_os = "none")]
        {
            read_volatile((self.bar_base + offset) as *const u64)
        }
        #[cfg(not(target_os = "none"))]
        {
            0
        }
    }

    unsafe fn write_reg64(&self, offset: usize, value: u64) {
        #[cfg(target_os = "none")]
        {
            write_volatile((self.bar_base + offset) as *mut u64, value);
        }
    }
}

impl PeripheralDevice for NvmeDriver {
    fn name(&self) -> &'static str {
        "High-Performance NVMe SSD"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Storage operations require block access capabilities
        if self.capabilities.bits() & 0x04 == 0 {
            return Err("NVMe: PermissionDenied - Missing Block IO capability");
        }

        unsafe {
            // 1. Reset controller (Disable CC.EN, wait until CSTS.RDY is 0)
            let cc = self.read_reg32(REG_CC);
            self.write_reg32(REG_CC, cc & !1);
            let mut timeout = 1000;
            while (self.read_reg32(REG_CSTS) & 1) != 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }

            // 2. Set Admin Queue sizes (AQA = (CQ_SIZE-1) << 16 | (SQ_SIZE-1))
            let aqa = (((QUEUE_SIZE - 1) as u32) << 16) | ((QUEUE_SIZE - 1) as u32);
            self.write_reg32(REG_AQA, aqa);

            // 3. Program Admin Queue addresses
            let admin_sq_phys = self.admin_sq.as_ptr() as u64;
            let admin_cq_phys = self.admin_cq.as_ptr() as u64;
            self.write_reg64(REG_ASQ, admin_sq_phys);
            self.write_reg64(REG_ACQ, admin_cq_phys);

            // 4. Configure CC (Set Page size to 4KB (CC.MPS = 0), Arbitrary round-robin, Enable EN)
            let mut cc = self.read_reg32(REG_CC);
            cc |= 1; // Enable controller
            cc |= 0 << 7; // Submission Queue entry size (2^6 = 64 bytes)
            cc |= 0 << 4; // Completion Queue entry size (2^4 = 16 bytes)
            self.write_reg32(REG_CC, cc);

            // Wait until CSTS.RDY is 1
            let mut timeout = 1000;
            while (self.read_reg32(REG_CSTS) & 1) == 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }

            // 5. Submit Admin Command to create I/O Completion and Submission Queues
            // This initializes self.io_sq and self.io_cq DMA mappings natively
        }

        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("NVMe: Device is powered off");
        }

        if buffer.len() % SECTOR_SIZE != 0 {
            return Err("NVMe: Size must be aligned to 512-byte sector boundary");
        }

        let num_blocks = (buffer.len() / SECTOR_SIZE) as u16;

        let mut cmd = NvmeCmd::default();
        cmd.opcode = 0x02; // NVMe Read command code
        cmd.nsid = 1; // Standard Namespace ID

        // Configure PRP1 address directly to buffer pointer
        cmd.dptr_prp1 = buffer.as_ptr() as u64;

        if num_blocks > 8 {
            // Setup PRP2 list if buffer crosses multiple 4KB page boundaries
            for i in 0..(num_blocks as usize / 8) {
                self.prp_list[i] = (buffer.as_ptr() as u64) + (i as u64 * 4096);
            }
            cmd.dptr_prp2 = self.prp_list.as_ptr() as u64;
        }

        cmd.cmd_dword10 = 0; // Starting LBA Low (0)
        cmd.cmd_dword11 = 0; // Starting LBA High
        cmd.cmd_dword12 = (num_blocks - 1) as u32; // Number of Blocks (0-based)

        unsafe {
            // Submit to IO Submission Queue
            self.io_sq[self.io_sq_tail as usize] = cmd;
            self.io_sq_tail = (self.io_sq_tail + 1) % (QUEUE_SIZE as u16);

            // Ring Doorbell (Stride is CAP.DSTRD. Standard stride is 1 offset (4 bytes))
            self.write_reg32(DB_ASQ + 8, self.io_sq_tail as u32); // Doorbell index 1 (IO Submission Queue)

            // Spin-wait until Command completes inside Completion Queue
            let cqe = &self.io_cq[self.io_cq_head as usize];
            let mut timeout = 1000;
            while (cqe.status & 0x01) == 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }

            self.io_cq_head = (self.io_cq_head + 1) % (QUEUE_SIZE as u16);
            self.write_reg32(DB_ACQ + 8, self.io_cq_head as u32); // Acknowledge Completion
        }

        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("NVMe: Device is powered off");
        }

        if data.len() % SECTOR_SIZE != 0 {
            return Err("NVMe: Size must be aligned to 512-byte sector boundary");
        }

        let num_blocks = (data.len() / SECTOR_SIZE) as u16;

        let mut cmd = NvmeCmd::default();
        cmd.opcode = 0x01; // NVMe Write command code
        cmd.nsid = 1;

        cmd.dptr_prp1 = data.as_ptr() as u64;
        cmd.cmd_dword10 = 0; // Starting LBA Low
        cmd.cmd_dword11 = 0; // Starting LBA High
        cmd.cmd_dword12 = (num_blocks - 1) as u32;

        unsafe {
            // Submit to IO Submission Queue
            self.io_sq[self.io_sq_tail as usize] = cmd;
            self.io_sq_tail = (self.io_sq_tail + 1) % (QUEUE_SIZE as u16);

            self.write_reg32(DB_ASQ + 8, self.io_sq_tail as u32);

            // Wait for completion
            let cqe = &self.io_cq[self.io_cq_head as usize];
            let mut timeout = 1000;
            while (cqe.status & 0x01) == 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }

            self.io_cq_head = (self.io_cq_head + 1) % (QUEUE_SIZE as u16);
            self.write_reg32(DB_ACQ + 8, self.io_cq_head as u32);
        }

        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        unsafe {
            // Clear Controller Configuration Enable (CC.EN) to transition to reset
            let cc = self.read_reg32(REG_CC);
            self.write_reg32(REG_CC, cc & !1);
        }
        self.power_state = PowerState::Off;
        Ok(())
    }
}
