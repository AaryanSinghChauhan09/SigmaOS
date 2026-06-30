/// SigmaOS: Sovereign NVMe 1.4 Controller Driver
/// Built in Rust — #![no_std], no alloc, no external dependencies.
/// Implements PCI BAR MMIO, Admin & I/O queue pairs, Identify Controller/Namespace,
/// Read/Write command submission, and completion polling.

#![no_std]
#![allow(dead_code)]

// ─── Sovereign Type Aliases ─────────────────────────────────────────────────
type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaUsize = usize;
type SigmaBool = bool;
type SigmaI32 = i32;

pub const SIGMA_OK: SigmaI32 = 0;
pub const SIGMA_ERR_TIMEOUT: SigmaI32 = -1;
pub const SIGMA_ERR_INIT: SigmaI32 = -2;
pub const SIGMA_ERR_IO: SigmaI32 = -3;
pub const SIGMA_ERR_FULL: SigmaI32 = -4;
pub const SIGMA_ERR_INVALID: SigmaI32 = -5;

// ─── PCI Constants ──────────────────────────────────────────────────────────
pub const NVME_CLASS_STORAGE: SigmaU8 = 0x01;
pub const NVME_SUBCLASS_NVM: SigmaU8 = 0x08;
pub const NVME_PROG_IF_NVME: SigmaU8 = 0x02;

// ─── NVMe Controller Registers (MMIO BAR0) ─────────────────────────────────
#[repr(C)]
pub struct NvmeControllerRegs {
    pub cap_lo: SigmaU32,       // 0x00: Controller Capabilities (low)
    pub cap_hi: SigmaU32,       // 0x04: Controller Capabilities (high)
    pub vs: SigmaU32,           // 0x08: Version
    pub intms: SigmaU32,        // 0x0C: Interrupt Mask Set
    pub intmc: SigmaU32,        // 0x10: Interrupt Mask Clear
    pub cc: SigmaU32,           // 0x14: Controller Configuration
    pub _rsvd: SigmaU32,        // 0x18
    pub csts: SigmaU32,         // 0x1C: Controller Status
    pub nssr: SigmaU32,         // 0x20: NVM Subsystem Reset
    pub aqa: SigmaU32,          // 0x24: Admin Queue Attributes
    pub asq_lo: SigmaU32,       // 0x28: Admin Submission Queue Base (low)
    pub asq_hi: SigmaU32,       // 0x2C: Admin Submission Queue Base (high)
    pub acq_lo: SigmaU32,       // 0x30: Admin Completion Queue Base (low)
    pub acq_hi: SigmaU32,       // 0x34: Admin Completion Queue Base (high)
}

// CC (Controller Configuration) bits
pub const CC_EN: SigmaU32 = 1 << 0;           // Enable
pub const CC_CSS_NVM: SigmaU32 = 0 << 4;      // NVM Command Set
pub const CC_MPS_4K: SigmaU32 = 0 << 7;       // Memory Page Size = 4096
pub const CC_AMS_RR: SigmaU32 = 0 << 11;      // Round Robin arbitration
pub const CC_SHN_NONE: SigmaU32 = 0 << 14;    // No shutdown
pub const CC_IOSQES_6: SigmaU32 = 6 << 16;    // I/O SQ entry size = 2^6 = 64 bytes
pub const CC_IOCQES_4: SigmaU32 = 4 << 20;    // I/O CQ entry size = 2^4 = 16 bytes

// CSTS (Controller Status) bits
pub const CSTS_RDY: SigmaU32 = 1 << 0;        // Ready
pub const CSTS_CFS: SigmaU32 = 1 << 1;        // Controller Fatal Status
pub const CSTS_SHST_MASK: SigmaU32 = 3 << 2;  // Shutdown Status

// ─── NVMe Submission Queue Entry (64 bytes) ─────────────────────────────────
#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct NvmeSubmissionEntry {
    pub cdw0: SigmaU32,         // Command Dword 0: opcode, fuse, cid
    pub nsid: SigmaU32,         // Namespace ID
    pub cdw2: SigmaU32,
    pub cdw3: SigmaU32,
    pub mptr_lo: SigmaU32,      // Metadata Pointer (low)
    pub mptr_hi: SigmaU32,      // Metadata Pointer (high)
    pub prp1_lo: SigmaU32,      // PRP Entry 1 (low)
    pub prp1_hi: SigmaU32,      // PRP Entry 1 (high)
    pub prp2_lo: SigmaU32,      // PRP Entry 2 (low)
    pub prp2_hi: SigmaU32,      // PRP Entry 2 (high)
    pub cdw10: SigmaU32,
    pub cdw11: SigmaU32,
    pub cdw12: SigmaU32,
    pub cdw13: SigmaU32,
    pub cdw14: SigmaU32,
    pub cdw15: SigmaU32,
}

impl NvmeSubmissionEntry {
    pub const fn zeroed() -> Self {
        NvmeSubmissionEntry {
            cdw0: 0, nsid: 0, cdw2: 0, cdw3: 0,
            mptr_lo: 0, mptr_hi: 0,
            prp1_lo: 0, prp1_hi: 0, prp2_lo: 0, prp2_hi: 0,
            cdw10: 0, cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0,
        }
    }

    /// Set opcode (bits 7:0 of CDW0)
    pub fn set_opcode(&mut self, opcode: SigmaU8) {
        self.cdw0 = (self.cdw0 & 0xFFFF_FF00) | (opcode as SigmaU32);
    }

    /// Set command ID (bits 31:16 of CDW0)
    pub fn set_cid(&mut self, cid: SigmaU16) {
        self.cdw0 = (self.cdw0 & 0x0000_FFFF) | ((cid as SigmaU32) << 16);
    }

    /// Set PRP1 (data buffer physical address)
    pub fn set_prp1(&mut self, addr: SigmaU64) {
        self.prp1_lo = addr as SigmaU32;
        self.prp1_hi = (addr >> 32) as SigmaU32;
    }

    /// Set PRP2 (second buffer or PRP list)
    pub fn set_prp2(&mut self, addr: SigmaU64) {
        self.prp2_lo = addr as SigmaU32;
        self.prp2_hi = (addr >> 32) as SigmaU32;
    }
}

// NVMe Admin Opcodes
pub const NVME_ADMIN_IDENTIFY: SigmaU8 = 0x06;
pub const NVME_ADMIN_CREATE_IO_SQ: SigmaU8 = 0x01;
pub const NVME_ADMIN_CREATE_IO_CQ: SigmaU8 = 0x05;
pub const NVME_ADMIN_DELETE_IO_SQ: SigmaU8 = 0x00;
pub const NVME_ADMIN_DELETE_IO_CQ: SigmaU8 = 0x04;
pub const NVME_ADMIN_SET_FEATURES: SigmaU8 = 0x09;
pub const NVME_ADMIN_GET_FEATURES: SigmaU8 = 0x0A;

// NVMe I/O Opcodes
pub const NVME_IO_READ: SigmaU8 = 0x02;
pub const NVME_IO_WRITE: SigmaU8 = 0x01;
pub const NVME_IO_FLUSH: SigmaU8 = 0x00;

// ─── NVMe Completion Queue Entry (16 bytes) ─────────────────────────────────
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct NvmeCompletionEntry {
    pub dw0: SigmaU32,          // Command-specific result
    pub dw1: SigmaU32,          // Reserved
    pub sq_head: SigmaU16,      // SQ Head Pointer
    pub sq_id: SigmaU16,        // SQ Identifier
    pub cid: SigmaU16,          // Command Identifier
    pub status_phase: SigmaU16, // Status Field + Phase Tag (bit 0)
}

impl NvmeCompletionEntry {
    pub const fn zeroed() -> Self {
        NvmeCompletionEntry {
            dw0: 0, dw1: 0,
            sq_head: 0, sq_id: 0,
            cid: 0, status_phase: 0,
        }
    }

    /// Phase bit is bit 0 of status_phase
    pub fn phase(&self) -> SigmaBool {
        (self.status_phase & 1) != 0
    }

    /// Status code (bits 15:1, right-shifted)
    pub fn status_code(&self) -> SigmaU16 {
        (self.status_phase >> 1) & 0x7FFF
    }

    /// Check if command completed successfully
    pub fn succeeded(&self) -> SigmaBool {
        self.status_code() == 0
    }
}

// ─── Queue Pair ─────────────────────────────────────────────────────────────
pub const QUEUE_DEPTH: SigmaUsize = 64;

pub struct NvmeQueue {
    sq: [NvmeSubmissionEntry; QUEUE_DEPTH],
    cq: [NvmeCompletionEntry; QUEUE_DEPTH],
    sq_tail: SigmaU16,
    cq_head: SigmaU16,
    phase: SigmaBool,
    depth: SigmaU16,
    qid: SigmaU16,
    cid_counter: SigmaU16,
}

impl NvmeQueue {
    pub const fn new(qid: SigmaU16) -> Self {
        NvmeQueue {
            sq: [NvmeSubmissionEntry::zeroed(); QUEUE_DEPTH],
            cq: [NvmeCompletionEntry::zeroed(); QUEUE_DEPTH],
            sq_tail: 0,
            cq_head: 0,
            phase: true,
            depth: QUEUE_DEPTH as SigmaU16,
            qid,
            cid_counter: 0,
        }
    }

    /// Submit a command to the submission queue
    pub fn submit(&mut self, mut cmd: NvmeSubmissionEntry) -> Option<SigmaU16> {
        let next = (self.sq_tail + 1) % self.depth;
        // Check if queue is full (simplified: no full detection for admin queue)
        cmd.set_cid(self.cid_counter);
        let cid = self.cid_counter;
        self.cid_counter = self.cid_counter.wrapping_add(1);

        self.sq[self.sq_tail as SigmaUsize] = cmd;
        self.sq_tail = next;
        Some(cid)
    }

    /// Check for and consume one completion entry
    pub fn poll_completion(&mut self) -> Option<NvmeCompletionEntry> {
        let entry = &self.cq[self.cq_head as SigmaUsize];
        if entry.phase() != self.phase {
            return None;
        }
        let result = *entry;
        self.cq_head += 1;
        if self.cq_head >= self.depth {
            self.cq_head = 0;
            self.phase = !self.phase;
        }
        Some(result)
    }

    /// Get physical address of submission queue base
    pub fn sq_phys(&self) -> SigmaU64 {
        &self.sq[0] as *const NvmeSubmissionEntry as SigmaU64
    }

    /// Get physical address of completion queue base
    pub fn cq_phys(&self) -> SigmaU64 {
        &self.cq[0] as *const NvmeCompletionEntry as SigmaU64
    }
}

// ─── NVMe Namespace ─────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct NvmeNamespace {
    pub nsid: SigmaU32,
    pub block_count: SigmaU64,
    pub block_size: SigmaU32,
    pub active: SigmaBool,
}

impl NvmeNamespace {
    pub const fn empty() -> Self {
        NvmeNamespace {
            nsid: 0,
            block_count: 0,
            block_size: 512,
            active: false,
        }
    }

    /// Total capacity in bytes
    pub fn capacity_bytes(&self) -> SigmaU64 {
        self.block_count * (self.block_size as SigmaU64)
    }
}

// ─── NVMe Controller ───────────────────────────────────────────────────────
pub const MAX_NAMESPACES: SigmaUsize = 8;
pub const MAX_IO_QUEUES: SigmaUsize = 4;

pub struct NvmeController {
    mmio_base: SigmaU64,
    admin_queue: NvmeQueue,
    io_queues: [NvmeQueue; MAX_IO_QUEUES],
    namespaces: [NvmeNamespace; MAX_NAMESPACES],
    num_io_queues: SigmaU8,
    num_namespaces: SigmaU8,
    doorbell_stride: SigmaU32,
    max_transfer_size: SigmaU32,
    serial_number: [SigmaU8; 20],
    model_number: [SigmaU8; 40],
    initialized: SigmaBool,
}

impl NvmeController {
    pub const fn new() -> Self {
        NvmeController {
            mmio_base: 0,
            admin_queue: NvmeQueue::new(0),
            io_queues: [
                NvmeQueue::new(1),
                NvmeQueue::new(2),
                NvmeQueue::new(3),
                NvmeQueue::new(4),
            ],
            namespaces: [NvmeNamespace::empty(); MAX_NAMESPACES],
            num_io_queues: 0,
            num_namespaces: 0,
            doorbell_stride: 4,
            max_transfer_size: 0,
            serial_number: [0; 20],
            model_number: [0; 40],
            initialized: false,
        }
    }

    // ── MMIO Helpers ────────────────────────────────────────────────────────
    unsafe fn read32(&self, offset: SigmaUsize) -> SigmaU32 {
        let ptr = (self.mmio_base as *const SigmaU8).add(offset) as *const SigmaU32;
        core::ptr::read_volatile(ptr)
    }

    unsafe fn write32(&self, offset: SigmaUsize, val: SigmaU32) {
        let ptr = (self.mmio_base as *mut SigmaU8).add(offset) as *mut SigmaU32;
        core::ptr::write_volatile(ptr, val);
    }

    /// Write submission queue doorbell
    unsafe fn ring_sq_doorbell(&self, qid: SigmaU16, tail: SigmaU16) {
        let offset = 0x1000 + ((2 * qid as SigmaUsize) * self.doorbell_stride as SigmaUsize);
        self.write32(offset, tail as SigmaU32);
    }

    /// Write completion queue doorbell
    unsafe fn ring_cq_doorbell(&self, qid: SigmaU16, head: SigmaU16) {
        let offset = 0x1000 + ((2 * qid as SigmaUsize + 1) * self.doorbell_stride as SigmaUsize);
        self.write32(offset, head as SigmaU32);
    }

    // ── Initialization ──────────────────────────────────────────────────────

    /// Initialize the NVMe controller from PCI BAR0 MMIO base
    pub unsafe fn init(&mut self, mmio_base: SigmaU64) -> SigmaI32 {
        self.mmio_base = mmio_base;

        // 1. Read CAP register for doorbell stride
        let cap_hi = self.read32(0x04);
        self.doorbell_stride = 4 << ((cap_hi >> 0) & 0xF); // DSTRD

        // 2. Disable controller (clear CC.EN)
        let mut cc = self.read32(0x14);
        cc &= !CC_EN;
        self.write32(0x14, cc);

        // Wait for CSTS.RDY == 0
        let mut timeout = 500_000u32;
        while (self.read32(0x1C) & CSTS_RDY) != 0 {
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }

        // 3. Configure Admin Queue Attributes (AQA)
        let aqa = ((QUEUE_DEPTH as SigmaU32 - 1) << 16) | (QUEUE_DEPTH as SigmaU32 - 1);
        self.write32(0x24, aqa);

        // 4. Set Admin Submission Queue base address (ASQ)
        let asq = self.admin_queue.sq_phys();
        self.write32(0x28, asq as SigmaU32);
        self.write32(0x2C, (asq >> 32) as SigmaU32);

        // 5. Set Admin Completion Queue base address (ACQ)
        let acq = self.admin_queue.cq_phys();
        self.write32(0x30, acq as SigmaU32);
        self.write32(0x34, (acq >> 32) as SigmaU32);

        // 6. Configure CC: NVM command set, 4K pages, RR arb, SQ entry=64, CQ entry=16
        cc = CC_EN | CC_CSS_NVM | CC_MPS_4K | CC_AMS_RR | CC_IOSQES_6 | CC_IOCQES_4;
        self.write32(0x14, cc);

        // 7. Wait for CSTS.RDY == 1
        timeout = 500_000;
        while (self.read32(0x1C) & CSTS_RDY) == 0 {
            if (self.read32(0x1C) & CSTS_CFS) != 0 {
                return SIGMA_ERR_INIT; // Fatal controller error
            }
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }

        self.initialized = true;
        SIGMA_OK
    }

    // ── Admin Commands ──────────────────────────────────────────────────────

    /// Send Identify Controller command (CNS=1)
    pub unsafe fn identify_controller(&mut self, data_buf_phys: SigmaU64) -> SigmaI32 {
        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_ADMIN_IDENTIFY);
        cmd.set_prp1(data_buf_phys);
        cmd.cdw10 = 1; // CNS = 1 (Identify Controller)

        if self.admin_queue.submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(0, self.admin_queue.sq_tail);

        // Poll for completion
        let mut timeout = 500_000u32;
        loop {
            if let Some(cqe) = self.admin_queue.poll_completion() {
                self.ring_cq_doorbell(0, self.admin_queue.cq_head);
                return if cqe.succeeded() { SIGMA_OK } else { SIGMA_ERR_IO };
            }
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }
    }

    /// Send Identify Namespace command (CNS=0)
    pub unsafe fn identify_namespace(
        &mut self,
        nsid: SigmaU32,
        data_buf_phys: SigmaU64,
    ) -> SigmaI32 {
        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_ADMIN_IDENTIFY);
        cmd.nsid = nsid;
        cmd.set_prp1(data_buf_phys);
        cmd.cdw10 = 0; // CNS = 0 (Identify Namespace)

        if self.admin_queue.submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(0, self.admin_queue.sq_tail);

        let mut timeout = 500_000u32;
        loop {
            if let Some(cqe) = self.admin_queue.poll_completion() {
                self.ring_cq_doorbell(0, self.admin_queue.cq_head);
                return if cqe.succeeded() { SIGMA_OK } else { SIGMA_ERR_IO };
            }
            timeout -= 1;
            if timeout == 0 {
                return SIGMA_ERR_TIMEOUT;
            }
        }
    }

    /// Create an I/O Completion Queue
    pub unsafe fn create_io_cq(&mut self, qid: SigmaU16) -> SigmaI32 {
        if qid == 0 || qid as SigmaUsize > MAX_IO_QUEUES {
            return SIGMA_ERR_INVALID;
        }
        let idx = (qid - 1) as SigmaUsize;
        let cq_phys = self.io_queues[idx].cq_phys();

        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_ADMIN_CREATE_IO_CQ);
        cmd.set_prp1(cq_phys);
        cmd.cdw10 = (((QUEUE_DEPTH as SigmaU32 - 1) << 16) | qid as SigmaU32);
        cmd.cdw11 = 1; // Physically Contiguous, no interrupt

        if self.admin_queue.submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(0, self.admin_queue.sq_tail);

        let mut timeout = 500_000u32;
        loop {
            if let Some(cqe) = self.admin_queue.poll_completion() {
                self.ring_cq_doorbell(0, self.admin_queue.cq_head);
                return if cqe.succeeded() { SIGMA_OK } else { SIGMA_ERR_IO };
            }
            timeout -= 1;
            if timeout == 0 { return SIGMA_ERR_TIMEOUT; }
        }
    }

    /// Create an I/O Submission Queue
    pub unsafe fn create_io_sq(&mut self, qid: SigmaU16) -> SigmaI32 {
        if qid == 0 || qid as SigmaUsize > MAX_IO_QUEUES {
            return SIGMA_ERR_INVALID;
        }
        let idx = (qid - 1) as SigmaUsize;
        let sq_phys = self.io_queues[idx].sq_phys();

        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_ADMIN_CREATE_IO_SQ);
        cmd.set_prp1(sq_phys);
        cmd.cdw10 = (((QUEUE_DEPTH as SigmaU32 - 1) << 16) | qid as SigmaU32);
        cmd.cdw11 = ((qid as SigmaU32) << 16) | 1; // CQID + Physically Contiguous

        if self.admin_queue.submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(0, self.admin_queue.sq_tail);

        let mut timeout = 500_000u32;
        loop {
            if let Some(cqe) = self.admin_queue.poll_completion() {
                self.ring_cq_doorbell(0, self.admin_queue.cq_head);
                if cqe.succeeded() {
                    self.num_io_queues += 1;
                    return SIGMA_OK;
                } else {
                    return SIGMA_ERR_IO;
                }
            }
            timeout -= 1;
            if timeout == 0 { return SIGMA_ERR_TIMEOUT; }
        }
    }

    // ── I/O Operations ──────────────────────────────────────────────────────

    /// Read LBA blocks from a namespace
    pub unsafe fn read_blocks(
        &mut self,
        nsid: SigmaU32,
        start_lba: SigmaU64,
        num_blocks: SigmaU16,
        data_buf_phys: SigmaU64,
        io_qid: SigmaU16,
    ) -> SigmaI32 {
        if io_qid == 0 || io_qid as SigmaUsize > self.num_io_queues as SigmaUsize {
            return SIGMA_ERR_INVALID;
        }
        let idx = (io_qid - 1) as SigmaUsize;

        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_IO_READ);
        cmd.nsid = nsid;
        cmd.set_prp1(data_buf_phys);
        cmd.cdw10 = start_lba as SigmaU32;           // SLBA low
        cmd.cdw11 = (start_lba >> 32) as SigmaU32;   // SLBA high
        cmd.cdw12 = (num_blocks - 1) as SigmaU32;    // NLB (0-based)

        if self.io_queues[idx].submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(io_qid, self.io_queues[idx].sq_tail);

        // Poll for completion
        let mut timeout = 1_000_000u32;
        loop {
            if let Some(cqe) = self.io_queues[idx].poll_completion() {
                self.ring_cq_doorbell(io_qid, self.io_queues[idx].cq_head);
                return if cqe.succeeded() { SIGMA_OK } else { SIGMA_ERR_IO };
            }
            timeout -= 1;
            if timeout == 0 { return SIGMA_ERR_TIMEOUT; }
        }
    }

    /// Write LBA blocks to a namespace
    pub unsafe fn write_blocks(
        &mut self,
        nsid: SigmaU32,
        start_lba: SigmaU64,
        num_blocks: SigmaU16,
        data_buf_phys: SigmaU64,
        io_qid: SigmaU16,
    ) -> SigmaI32 {
        if io_qid == 0 || io_qid as SigmaUsize > self.num_io_queues as SigmaUsize {
            return SIGMA_ERR_INVALID;
        }
        let idx = (io_qid - 1) as SigmaUsize;

        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_IO_WRITE);
        cmd.nsid = nsid;
        cmd.set_prp1(data_buf_phys);
        cmd.cdw10 = start_lba as SigmaU32;
        cmd.cdw11 = (start_lba >> 32) as SigmaU32;
        cmd.cdw12 = (num_blocks - 1) as SigmaU32;

        if self.io_queues[idx].submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(io_qid, self.io_queues[idx].sq_tail);

        let mut timeout = 1_000_000u32;
        loop {
            if let Some(cqe) = self.io_queues[idx].poll_completion() {
                self.ring_cq_doorbell(io_qid, self.io_queues[idx].cq_head);
                return if cqe.succeeded() { SIGMA_OK } else { SIGMA_ERR_IO };
            }
            timeout -= 1;
            if timeout == 0 { return SIGMA_ERR_TIMEOUT; }
        }
    }

    /// Flush (sync) a namespace
    pub unsafe fn flush(&mut self, nsid: SigmaU32, io_qid: SigmaU16) -> SigmaI32 {
        if io_qid == 0 || io_qid as SigmaUsize > self.num_io_queues as SigmaUsize {
            return SIGMA_ERR_INVALID;
        }
        let idx = (io_qid - 1) as SigmaUsize;

        let mut cmd = NvmeSubmissionEntry::zeroed();
        cmd.set_opcode(NVME_IO_FLUSH);
        cmd.nsid = nsid;

        if self.io_queues[idx].submit(cmd).is_none() {
            return SIGMA_ERR_FULL;
        }
        self.ring_sq_doorbell(io_qid, self.io_queues[idx].sq_tail);

        let mut timeout = 1_000_000u32;
        loop {
            if let Some(cqe) = self.io_queues[idx].poll_completion() {
                self.ring_cq_doorbell(io_qid, self.io_queues[idx].cq_head);
                return if cqe.succeeded() { SIGMA_OK } else { SIGMA_ERR_IO };
            }
            timeout -= 1;
            if timeout == 0 { return SIGMA_ERR_TIMEOUT; }
        }
    }

    // ── Status ──────────────────────────────────────────────────────────────
    pub fn is_initialized(&self) -> SigmaBool { self.initialized }
    pub fn namespace_count(&self) -> SigmaU8 { self.num_namespaces }
    pub fn io_queue_count(&self) -> SigmaU8 { self.num_io_queues }
}

// ─── Global State ───────────────────────────────────────────────────────────
static mut NVME: NvmeController = NvmeController::new();

// ─── C ABI Entry Points ────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sovereignnvme_init(mmio_base: SigmaU64) -> SigmaI32 {
    NVME.init(mmio_base)
}

#[no_mangle]
pub unsafe extern "C" fn sovereignnvme_read(
    nsid: SigmaU32, lba: SigmaU64, count: SigmaU16, buf: SigmaU64, qid: SigmaU16,
) -> SigmaI32 {
    NVME.read_blocks(nsid, lba, count, buf, qid)
}

#[no_mangle]
pub unsafe extern "C" fn sovereignnvme_write(
    nsid: SigmaU32, lba: SigmaU64, count: SigmaU16, buf: SigmaU64, qid: SigmaU16,
) -> SigmaI32 {
    NVME.write_blocks(nsid, lba, count, buf, qid)
}

#[no_mangle]
pub unsafe extern "C" fn sovereignnvme_flush(nsid: SigmaU32, qid: SigmaU16) -> SigmaI32 {
    NVME.flush(nsid, qid)
}