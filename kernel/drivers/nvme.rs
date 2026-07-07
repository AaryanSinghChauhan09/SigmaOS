//! SigmaOS — NVMe Storage Driver
//! Bare-metal NVMe controller driver for direct disk access.
//! No std, no allocator — fixed-size submission/completion queues.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── NVMe Controller Registers (BAR0, MMIO) ─────────────────────────────────
const NVME_REG_CAP:  Usize = 0x00;  // Controller Capabilities
const NVME_REG_VS:   Usize = 0x08;  // Version
const NVME_REG_INTMS: Usize = 0x0C; // Interrupt Mask Set
const NVME_REG_INTMC: Usize = 0x10; // Interrupt Mask Clear
const NVME_REG_CC:   Usize = 0x14;  // Controller Configuration
const NVME_REG_CSTS: Usize = 0x1C;  // Controller Status
const NVME_REG_AQA:  Usize = 0x24;  // Admin Queue Attributes
const NVME_REG_ASQ:  Usize = 0x28;  // Admin Submission Queue Base
const NVME_REG_ACQ:  Usize = 0x30;  // Admin Completion Queue Base

// CC register fields
const CC_EN:       U32 = 1 << 0;
const CC_CSS_NVM:  U32 = 0 << 4;
const CC_MPS_4K:   U32 = 0 << 7;  // 2^(12+0) = 4K
const CC_AMS_RR:   U32 = 0 << 11; // Round Robin
const CC_IOSQES:   U32 = 6 << 16; // 2^6 = 64 bytes
const CC_IOCQES:   U32 = 4 << 20; // 2^4 = 16 bytes

// CSTS fields
const CSTS_RDY: U32 = 1 << 0;

// ── NVMe Command Opcodes ────────────────────────────────────────────────────
const ADMIN_DELETE_IO_SQ: U8 = 0x00;
const ADMIN_CREATE_IO_SQ: U8 = 0x01;
const ADMIN_DELETE_IO_CQ: U8 = 0x04;
const ADMIN_CREATE_IO_CQ: U8 = 0x05;
const ADMIN_IDENTIFY:     U8 = 0x06;
const ADMIN_SET_FEATURES:  U8 = 0x09;

const IO_WRITE: U8 = 0x01;
const IO_READ:  U8 = 0x02;
const IO_FLUSH: U8 = 0x00;

// ── Submission Queue Entry (64 bytes) ───────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct NvmeSubmissionEntry {
    pub opcode:    U8,
    pub flags:     U8,
    pub cid:       U16,
    pub nsid:      U32,
    pub rsvd:      U64,
    pub mptr:      U64,
    pub prp1:      U64,
    pub prp2:      U64,
    pub cdw10:     U32,
    pub cdw11:     U32,
    pub cdw12:     U32,
    pub cdw13:     U32,
    pub cdw14:     U32,
    pub cdw15:     U32,
}

impl NvmeSubmissionEntry {
    pub const fn zero() -> Self {
        NvmeSubmissionEntry {
            opcode: 0, flags: 0, cid: 0, nsid: 0, rsvd: 0,
            mptr: 0, prp1: 0, prp2: 0,
            cdw10: 0, cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0,
        }
    }
}

// ── Completion Queue Entry (16 bytes) ───────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct NvmeCompletionEntry {
    pub dword0:    U32,
    pub rsvd:      U32,
    pub sq_head:   U16,
    pub sq_id:     U16,
    pub cid:       U16,
    pub status:    U16,
}

impl NvmeCompletionEntry {
    pub const fn zero() -> Self {
        NvmeCompletionEntry {
            dword0: 0, rsvd: 0, sq_head: 0, sq_id: 0, cid: 0, status: 0,
        }
    }
}

// ── Queue pair ──────────────────────────────────────────────────────────────
const QUEUE_DEPTH: Usize = 64;

pub struct NvmeQueuePair {
    sq:       [NvmeSubmissionEntry; QUEUE_DEPTH],
    cq:       [NvmeCompletionEntry; QUEUE_DEPTH],
    sq_tail:  U16,
    cq_head:  U16,
    cq_phase: bool,
    cid_next: U16,
}

impl NvmeQueuePair {
    pub const fn new() -> Self {
        NvmeQueuePair {
            sq: [NvmeSubmissionEntry::zero(); QUEUE_DEPTH],
            cq: [NvmeCompletionEntry::zero(); QUEUE_DEPTH],
            sq_tail: 0,
            cq_head: 0,
            cq_phase: true,
            cid_next: 0,
        }
    }

    fn next_cid(&mut self) -> U16 {
        let cid = self.cid_next;
        self.cid_next = self.cid_next.wrapping_add(1);
        cid
    }

    fn submit(&mut self, mut cmd: NvmeSubmissionEntry) -> U16 {
        let cid = self.next_cid();
        cmd.cid = cid;
        let idx = self.sq_tail as Usize;
        self.sq[idx] = cmd;
        self.sq_tail = ((self.sq_tail as Usize + 1) % QUEUE_DEPTH) as U16;
        cid
    }

    fn poll_completion(&mut self) -> Option<NvmeCompletionEntry> {
        let idx = self.cq_head as Usize;
        let entry = self.cq[idx];
        let phase_bit = (entry.status & 1) != 0;
        if phase_bit != self.cq_phase {
            return None;
        }
        self.cq_head = ((self.cq_head as Usize + 1) % QUEUE_DEPTH) as U16;
        if self.cq_head == 0 {
            self.cq_phase = !self.cq_phase;
        }
        Some(entry)
    }
}

// ── NVMe Controller ────────────────────────────────────────────────────────
pub struct NvmeController {
    bar0:         U64,
    admin_queue:  NvmeQueuePair,
    io_queue:     NvmeQueuePair,
    stride:       U32,
    max_transfer: U32,
    serial:       [U8; 20],
    model:        [U8; 40],
    ns_count:     U32,
    ns_size:      U64,  // total LBAs of namespace 1
    lba_shift:    U8,   // log2(sector size), typically 9 (512B) or 12 (4K)
    ready:        bool,
}

impl NvmeController {
    pub const fn new() -> Self {
        NvmeController {
            bar0: 0,
            admin_queue: NvmeQueuePair::new(),
            io_queue: NvmeQueuePair::new(),
            stride: 0,
            max_transfer: 0,
            serial: [0; 20],
            model: [0; 40],
            ns_count: 0,
            ns_size: 0,
            lba_shift: 9,
            ready: false,
        }
    }
}

// ── MMIO helpers ────────────────────────────────────────────────────────────
#[inline]
unsafe fn mmio_read32(base: U64, offset: Usize) -> U32 {
    let ptr = (base + offset as U64) as *const U32;
    core::ptr::read_volatile(ptr)
}

#[inline]
unsafe fn mmio_write32(base: U64, offset: Usize, val: U32) {
    let ptr = (base + offset as U64) as *mut U32;
    core::ptr::write_volatile(ptr, val);
}

#[inline]
unsafe fn mmio_read64(base: U64, offset: Usize) -> U64 {
    let ptr = (base + offset as U64) as *const U64;
    core::ptr::read_volatile(ptr)
}

#[inline]
unsafe fn mmio_write64(base: U64, offset: Usize, val: U64) {
    let ptr = (base + offset as U64) as *mut U64;
    core::ptr::write_volatile(ptr, val);
}

// ── Doorbell helpers ────────────────────────────────────────────────────────
#[inline]
fn sq_doorbell_offset(qid: U16, stride: U32) -> Usize {
    0x1000 + (2 * qid as Usize) * (4 << stride as Usize)
}

#[inline]
fn cq_doorbell_offset(qid: U16, stride: U32) -> Usize {
    0x1000 + (2 * qid as Usize + 1) * (4 << stride as Usize)
}

static mut NVME: NvmeController = NvmeController::new();

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the NVMe controller at the given BAR0 MMIO address.
/// 1. Disable controller (CC.EN = 0)
/// 2. Configure admin queues
/// 3. Enable controller
/// 4. Issue Identify Controller + Identify Namespace
#[no_mangle]
pub unsafe extern "C" fn nvme_init(bar0: U64) -> i32 {
    let ctrl = &mut NVME;
    ctrl.bar0 = bar0;

    // Read capabilities
    let cap = mmio_read64(bar0, NVME_REG_CAP);
    ctrl.stride = ((cap >> 32) & 0xF) as U32;
    ctrl.max_transfer = ((cap >> 37) & 0xFF) as U32;

    // 1. Disable controller
    let mut cc = mmio_read32(bar0, NVME_REG_CC);
    cc &= !CC_EN;
    mmio_write32(bar0, NVME_REG_CC, cc);

    // Wait for CSTS.RDY == 0
    let mut timeout = 500_000u32;
    while mmio_read32(bar0, NVME_REG_CSTS) & CSTS_RDY != 0 {
        timeout -= 1;
        if timeout == 0 { return -1; }
    }

    // 2. Set admin queue attributes
    let aqa = ((QUEUE_DEPTH as U32 - 1) << 16) | (QUEUE_DEPTH as U32 - 1);
    mmio_write32(bar0, NVME_REG_AQA, aqa);

    // Set admin queue base addresses
    let sq_phys = ctrl.admin_queue.sq.as_ptr() as U64;
    let cq_phys = ctrl.admin_queue.cq.as_ptr() as U64;
    mmio_write64(bar0, NVME_REG_ASQ, sq_phys);
    mmio_write64(bar0, NVME_REG_ACQ, cq_phys);

    // 3. Enable controller
    cc = CC_EN | CC_CSS_NVM | CC_MPS_4K | CC_AMS_RR | CC_IOSQES | CC_IOCQES;
    mmio_write32(bar0, NVME_REG_CC, cc);

    // Wait for ready
    timeout = 500_000;
    while mmio_read32(bar0, NVME_REG_CSTS) & CSTS_RDY == 0 {
        timeout -= 1;
        if timeout == 0 { return -2; }
    }

    ctrl.ready = true;
    0
}

/// Submit a block read command to namespace 1.
/// `lba` — starting logical block address
/// `block_count` — number of blocks (0-indexed for NVMe, so n-1 internally)
/// `buf_phys` — physical address of destination buffer
#[no_mangle]
pub unsafe extern "C" fn nvme_read(lba: U64, block_count: U16, buf_phys: U64) -> i32 {
    let ctrl = &mut NVME;
    if !ctrl.ready { return -1; }

    let mut cmd = NvmeSubmissionEntry::zero();
    cmd.opcode = IO_READ;
    cmd.nsid = 1;
    cmd.prp1 = buf_phys;
    cmd.cdw10 = lba as U32;
    cmd.cdw11 = (lba >> 32) as U32;
    cmd.cdw12 = (block_count.wrapping_sub(1)) as U32;

    let _cid = ctrl.io_queue.submit(cmd);

    // Ring submission doorbell
    mmio_write32(ctrl.bar0, sq_doorbell_offset(1, ctrl.stride), ctrl.io_queue.sq_tail as U32);
    0
}

/// Submit a block write command to namespace 1.
#[no_mangle]
pub unsafe extern "C" fn nvme_write(lba: U64, block_count: U16, buf_phys: U64) -> i32 {
    let ctrl = &mut NVME;
    if !ctrl.ready { return -1; }

    let mut cmd = NvmeSubmissionEntry::zero();
    cmd.opcode = IO_WRITE;
    cmd.nsid = 1;
    cmd.prp1 = buf_phys;
    cmd.cdw10 = lba as U32;
    cmd.cdw11 = (lba >> 32) as U32;
    cmd.cdw12 = (block_count.wrapping_sub(1)) as U32;

    let _cid = ctrl.io_queue.submit(cmd);
    mmio_write32(ctrl.bar0, sq_doorbell_offset(1, ctrl.stride), ctrl.io_queue.sq_tail as U32);
    0
}

/// Poll for a completion on the I/O queue. Returns 0 on success, -1 if none pending.
#[no_mangle]
pub unsafe extern "C" fn nvme_poll_io() -> i32 {
    let ctrl = &mut NVME;
    match ctrl.io_queue.poll_completion() {
        Some(cqe) => {
            // Ring completion doorbell
            mmio_write32(ctrl.bar0, cq_doorbell_offset(1, ctrl.stride), ctrl.io_queue.cq_head as U32);
            let status = (cqe.status >> 1) & 0xFF;
            if status == 0 { 0 } else { status as i32 }
        }
        None => -1,
    }
}
