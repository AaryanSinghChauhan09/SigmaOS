//! # NVMe Driver
//!
//! Non-Volatile Memory Express storage driver for modern SSDs.
//! Inspired by Linux drivers/nvme/ and FreeBSD sys/dev/nvme/.

#![no_std]

extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};

/// NVMe controller registers (BAR0)
pub mod nvme_regs {
    pub const CAP: usize = 0x00; // Controller Capabilities
    pub const VS: usize = 0x08; // Version
    pub const INTMS: usize = 0x0C; // Interrupt Mask Set
    pub const INTMC: usize = 0x10; // Interrupt Mask Clear
    pub const CC: usize = 0x14; // Controller Configuration
    pub const CSTS: usize = 0x1C; // Controller Status
    pub const NSSR: usize = 0x20; // NVM Subsystem Reset
    pub const AQA: usize = 0x24; // Admin Queue Attributes
    pub const ASQ: usize = 0x28; // Admin Submission Queue
    pub const ACQ: usize = 0x30; // Admin Completion Queue
    pub const SQ0TDBL: usize = 0x1000; // Submission Queue 0 Tail Doorbell
}

/// Controller Configuration bits
pub mod cc_bits {
    pub const ENABLE: u32 = 1 << 0;
    pub const CSS_NVM: u32 = 0 << 4;
    pub const MPS_4K: u32 = 0 << 7;
    pub const AMS_RR: u32 = 0 << 11;
    pub const SHN_NONE: u32 = 0 << 14;
    pub const IOSQES: u32 = 6 << 16; // 2^6 = 64 bytes
    pub const IOCQES: u32 = 4 << 20; // 2^4 = 16 bytes
}

/// Controller Status bits
pub mod csts_bits {
    pub const RDY: u32 = 1 << 0;
    pub const CFS: u32 = 1 << 1;
    pub const SHST_NORMAL: u32 = 0 << 2;
    pub const NSSRO: u32 = 1 << 4;
}

/// NVMe Admin Command Opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NvmeAdminOpcode {
    DeleteIOSQ = 0x00,
    CreateIOSQ = 0x01,
    GetLogPage = 0x02,
    DeleteIOCQ = 0x04,
    CreateIOCQ = 0x05,
    Identify = 0x06,
    Abort = 0x08,
    SetFeatures = 0x09,
    GetFeatures = 0x0A,
    AsyncEventRequest = 0x0C,
    NamespaceManagement = 0x0D,
    FirmwareCommit = 0x10,
    FirmwareDownload = 0x11,
    NamespaceAttachment = 0x15,
    FormatNVM = 0x80,
    SecuritySend = 0x81,
    SecurityReceive = 0x82,
}

/// NVMe I/O Command Opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NvmeIOOpcode {
    Flush = 0x00,
    Write = 0x01,
    Read = 0x02,
    WriteUncorrectable = 0x04,
    Compare = 0x05,
    WriteZeroes = 0x08,
    DatasetManagement = 0x09,
    ReservationRegister = 0x0D,
    ReservationReport = 0x0E,
    ReservationAcquire = 0x11,
    ReservationRelease = 0x15,
}

/// NVMe Submission Queue Entry (64 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct NvmeSQEntry {
    pub opcode: u8,
    pub flags: u8,
    pub command_id: u16,
    pub nsid: u32, // Namespace ID
    pub reserved: [u32; 2],
    pub metadata: u64,
    pub prp1: u64,  // Physical Region Page 1
    pub prp2: u64,  // Physical Region Page 2
    pub cdw10: u32, // Command Dword 10
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32,
}

/// NVMe Completion Queue Entry (16 bytes)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct NvmeCQEntry {
    pub command_specific: u32,
    pub reserved: u32,
    pub sq_head: u16,
    pub sq_id: u16,
    pub command_id: u16,
    pub status: u16, // Status field (includes phase bit)
}

impl NvmeCQEntry {
    pub fn get_phase(&self) -> bool {
        (self.status & 0x0001) != 0
    }

    pub fn get_status_code(&self) -> u16 {
        (self.status >> 1) & 0x7FFF
    }

    pub fn is_success(&self) -> bool {
        ((self.status >> 1) & 0xFF) == 0
    }
}

/// NVMe Queue Pair
pub struct NvmeQueuePair {
    pub sq_entries: Vec<NvmeSQEntry>,
    pub cq_entries: Vec<NvmeCQEntry>,
    pub sq_tail: AtomicU16,
    pub cq_head: AtomicU16,
    pub cq_phase: AtomicU16,
    pub queue_id: u16,
    pub sq_doorbell: usize,
    pub cq_doorbell: usize,
    pub depth: u16,
}

impl NvmeQueuePair {
    pub fn new(queue_id: u16, depth: u16, doorbell_base: usize, doorbell_stride: usize) -> Self {
        let sq_doorbell = doorbell_base + (2 * queue_id as usize * doorbell_stride);
        let cq_doorbell = doorbell_base + ((2 * queue_id as usize + 1) * doorbell_stride);

        Self {
            sq_entries: alloc::vec![unsafe { core::mem::zeroed() }; depth as usize],
            cq_entries: alloc::vec![unsafe { core::mem::zeroed() }; depth as usize],
            sq_tail: AtomicU16::new(0),
            cq_head: AtomicU16::new(0),
            cq_phase: AtomicU16::new(1),
            queue_id,
            sq_doorbell,
            cq_doorbell,
            depth,
        }
    }

    /// Submit command to submission queue
    pub fn submit_command(&mut self, cmd: NvmeSQEntry) -> Result<u16, NvmeError> {
        let tail = self.sq_tail.load(Ordering::Acquire);
        let next_tail = (tail + 1) % self.depth;

        // Check if queue is full
        let cq_head = self.cq_head.load(Ordering::Acquire);
        if next_tail == cq_head {
            return Err(NvmeError::QueueFull);
        }

        // Write command to SQ
        self.sq_entries[tail as usize] = cmd;

        // Update tail and ring doorbell
        self.sq_tail.store(next_tail, Ordering::Release);
        unsafe {
            core::ptr::write_volatile(self.sq_doorbell as *mut u32, next_tail as u32);
        }

        Ok(cmd.command_id)
    }

    /// Process completions from completion queue
    pub fn process_completions(&mut self) -> Vec<NvmeCQEntry> {
        let mut completions = Vec::new();
        let phase = self.cq_phase.load(Ordering::Acquire);
        let mut head = self.cq_head.load(Ordering::Acquire);

        loop {
            let entry = self.cq_entries[head as usize];

            // Check phase bit
            if entry.get_phase() != (phase != 0) {
                break;
            }

            completions.push(entry);

            // Move to next entry
            head = (head + 1) % self.depth;

            // Toggle phase on wrap
            if head == 0 {
                self.cq_phase.fetch_xor(1, Ordering::Release);
            }
        }

        if !completions.is_empty() {
            self.cq_head.store(head, Ordering::Release);

            // Ring completion queue doorbell
            unsafe {
                core::ptr::write_volatile(self.cq_doorbell as *mut u32, head as u32);
            }
        }

        completions
    }
}

/// NVMe Identify Controller Data
#[repr(C, packed)]
pub struct NvmeIdentifyController {
    pub vid: u16,     // PCI Vendor ID
    pub ssvid: u16,   // PCI Subsystem Vendor ID
    pub sn: [u8; 20], // Serial Number
    pub mn: [u8; 40], // Model Number
    pub fr: [u8; 8],  // Firmware Revision
    pub rab: u8,
    pub ieee: [u8; 3],
    pub cmic: u8,
    pub mdts: u8, // Maximum Data Transfer Size
    pub cntlid: u16,
    pub ver: u32, // Version
    // ... (many more fields in real structure)
    pub reserved: [u8; 3824],
}

/// NVMe Namespace
pub struct NvmeNamespace {
    pub nsid: u32,
    pub block_size: u32,
    pub num_blocks: u64,
    pub capacity: u64, // In bytes
}

/// NVMe Controller
pub struct NvmeController {
    mmio_base: usize,
    admin_queue: NvmeQueuePair,
    io_queues: Vec<Arc<NvmeQueuePair>>,
    namespaces: Vec<NvmeNamespace>,
    next_command_id: AtomicU16,
    doorbell_stride: usize,
}

impl NvmeController {
    const ADMIN_QUEUE_SIZE: u16 = 64;
    const IO_QUEUE_SIZE: u16 = 256;

    pub fn new(mmio_base: usize) -> Self {
        Self {
            mmio_base,
            admin_queue: NvmeQueuePair::new(0, Self::ADMIN_QUEUE_SIZE, 0, 4),
            io_queues: Vec::new(),
            namespaces: Vec::new(),
            next_command_id: AtomicU16::new(1),
            doorbell_stride: 4,
        }
    }

    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.mmio_base + offset) as *const u32) }
    }

    fn write_reg(&self, offset: usize, val: u32) {
        unsafe {
            core::ptr::write_volatile((self.mmio_base + offset) as *mut u32, val);
        }
    }

    fn read_reg_64(&self, offset: usize) -> u64 {
        let low = self.read_reg(offset) as u64;
        let high = self.read_reg(offset + 4) as u64;
        (high << 32) | low
    }

    fn write_reg_64(&self, offset: usize, val: u64) {
        self.write_reg(offset, (val & 0xFFFFFFFF) as u32);
        self.write_reg(offset + 4, (val >> 32) as u32);
    }

    /// Initialize controller
    pub fn init(&mut self) -> Result<(), NvmeError> {
        // Read capabilities
        let cap = self.read_reg_64(nvme_regs::CAP);
        let doorbell_stride = 4 << ((cap >> 32) & 0xF);
        self.doorbell_stride = doorbell_stride;

        // Disable controller
        let mut cc = self.read_reg(nvme_regs::CC);
        cc &= !cc_bits::ENABLE;
        self.write_reg(nvme_regs::CC, cc);

        // Wait for ready bit to clear
        let mut timeout = 1000;
        while (self.read_reg(nvme_regs::CSTS) & csts_bits::RDY) != 0 && timeout > 0 {
            timeout -= 1;
        }

        if timeout == 0 {
            return Err(NvmeError::Timeout);
        }

        // Setup admin queue
        let admin_sq_base = self.admin_queue.sq_entries.as_ptr() as u64;
        let admin_cq_base = self.admin_queue.cq_entries.as_ptr() as u64;

        self.write_reg(
            nvme_regs::AQA,
            ((Self::ADMIN_QUEUE_SIZE - 1) as u32) << 16 | (Self::ADMIN_QUEUE_SIZE - 1) as u32,
        );
        self.write_reg_64(nvme_regs::ASQ, admin_sq_base);
        self.write_reg_64(nvme_regs::ACQ, admin_cq_base);

        // Configure and enable controller
        cc = cc_bits::ENABLE
            | cc_bits::CSS_NVM
            | cc_bits::MPS_4K
            | cc_bits::AMS_RR
            | cc_bits::SHN_NONE
            | cc_bits::IOSQES
            | cc_bits::IOCQES;
        self.write_reg(nvme_regs::CC, cc);

        // Wait for ready
        timeout = 1000;
        while (self.read_reg(nvme_regs::CSTS) & csts_bits::RDY) == 0 && timeout > 0 {
            timeout -= 1;
        }

        if timeout == 0 {
            return Err(NvmeError::Timeout);
        }

        // Update admin queue doorbells
        self.admin_queue.sq_doorbell = self.mmio_base + nvme_regs::SQ0TDBL;
        self.admin_queue.cq_doorbell = self.mmio_base + nvme_regs::SQ0TDBL + doorbell_stride;

        Ok(())
    }

    /// Get next command ID
    fn next_cmd_id(&self) -> u16 {
        self.next_command_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Create I/O completion queue
    pub fn create_io_cq(&mut self, queue_id: u16) -> Result<(), NvmeError> {
        let qp = NvmeQueuePair::new(
            queue_id,
            Self::IO_QUEUE_SIZE,
            self.mmio_base + nvme_regs::SQ0TDBL,
            self.doorbell_stride,
        );

        let cq_base = qp.cq_entries.as_ptr() as u64;

        let mut cmd: NvmeSQEntry = unsafe { core::mem::zeroed() };
        cmd.opcode = NvmeAdminOpcode::CreateIOCQ as u8;
        cmd.command_id = self.next_cmd_id();
        cmd.prp1 = cq_base;
        cmd.cdw10 = ((Self::IO_QUEUE_SIZE - 1) as u32) << 16 | queue_id as u32;
        cmd.cdw11 = 1; // Physically contiguous

        self.admin_queue.submit_command(cmd)?;

        // Wait for completion
        // In production: use interrupt or polling

        self.io_queues.push(Arc::new(qp));
        Ok(())
    }

    /// Read block from namespace
    pub fn read_block(&mut self, nsid: u32, lba: u64, buffer: &mut [u8]) -> Result<(), NvmeError> {
        if self.io_queues.is_empty() {
            return Err(NvmeError::NoIOQueue);
        }

        let mut cmd: NvmeSQEntry = unsafe { core::mem::zeroed() };
        cmd.opcode = NvmeIOOpcode::Read as u8;
        cmd.command_id = self.next_cmd_id();
        cmd.nsid = nsid;
        cmd.prp1 = buffer.as_ptr() as u64;
        cmd.cdw10 = (lba & 0xFFFFFFFF) as u32;
        cmd.cdw11 = (lba >> 32) as u32;
        cmd.cdw12 = 0; // Read 1 block

        let queue = Arc::get_mut(&mut self.io_queues[0]).unwrap();
        queue.submit_command(cmd)?;

        Ok(())
    }
}

/// NVMe errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvmeError {
    Timeout,
    QueueFull,
    NoIOQueue,
    CommandFailed,
    InvalidNamespace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvme_cqe_phase() {
        let mut entry: NvmeCQEntry = unsafe { core::mem::zeroed() };
        entry.status = 0x0001;
        assert!(entry.get_phase());

        entry.status = 0x0000;
        assert!(!entry.get_phase());
    }

    #[test]
    fn test_nvme_queue_pair() {
        let qp = NvmeQueuePair::new(1, 16, 0x1000, 4);
        assert_eq!(qp.queue_id, 1);
        assert_eq!(qp.depth, 16);
        assert_eq!(qp.sq_doorbell, 0x1000 + 2 * 1 * 4);
        assert_eq!(qp.cq_doorbell, 0x1000 + 3 * 1 * 4);
    }
}
