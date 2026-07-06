// SigmaOS — NVMe Async Driver (MSI-X + io_uring-style queues)
// Replaces MMIO polling with interrupt-driven async I/O
// Issue I-03: 4× IOPS improvement target
#![no_std]
#![allow(dead_code)]
use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, Ordering};

// ─── NVMe Register Offsets ────────────────────────────────────────────────────
pub const NVME_CAP:   u32 = 0x000;
pub const NVME_VS:    u32 = 0x008;
pub const NVME_CC:    u32 = 0x014;
pub const NVME_CSTS:  u32 = 0x01C;
pub const NVME_AQA:   u32 = 0x024;
pub const NVME_ASQ:   u32 = 0x028;
pub const NVME_ACQ:   u32 = 0x030;
pub const NVME_SQ0TDB: u32 = 0x1000;
pub const NVME_CQ0HDB: u32 = 0x1004;

pub const NVME_CC_EN:  u32 = 1 << 0;
pub const NVME_CSTS_RDY: u32 = 1 << 0;
pub const NVME_CC_CSS_NVM: u32 = 0;
pub const NVME_CC_MPS_4K: u32 = 0 << 7;
pub const NVME_CC_AMS_RR: u32 = 0 << 11;
pub const NVME_CC_IOSQES: u32 = 6 << 16;
pub const NVME_CC_IOCQES: u32 = 4 << 20;

// ─── Submission/Completion Queue Entry ───────────────────────────────────────
pub const NVME_SQ_ENTRY_SIZE: usize = 64;
pub const NVME_CQ_ENTRY_SIZE: usize = 16;
pub const NVME_QUEUE_DEPTH:   usize = 256;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NvmeSqEntry {
    pub opcode:   u8,
    pub fuse:     u8,
    pub cid:      u16,
    pub nsid:     u32,
    pub rsvd1:    [u32; 2],
    pub mptr:     u64,
    pub prp1:     u64,
    pub prp2:     u64,
    pub cdw10:    u32,
    pub cdw11:    u32,
    pub cdw12:    u32,
    pub cdw13:    u32,
    pub cdw14:    u32,
    pub cdw15:    u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NvmeCqEntry {
    pub dw0:     u32,
    pub rsvd:    u32,
    pub sq_head: u16,
    pub sq_id:   u16,
    pub cid:     u16,
    pub status:  u16,
}

impl NvmeCqEntry {
    pub fn success(&self) -> bool { (self.status >> 1) & 0xFF == 0 }
    pub fn phase(&self) -> bool   { self.status & 0x1 != 0 }
}

// ─── NVMe Opcodes ─────────────────────────────────────────────────────────────
pub const NVME_OP_FLUSH:     u8 = 0x00;
pub const NVME_OP_WRITE:     u8 = 0x01;
pub const NVME_OP_READ:      u8 = 0x02;
pub const NVME_OP_IDENTIFY:  u8 = 0x06;
pub const NVME_OP_CREATE_SQ: u8 = 0x01; // admin
pub const NVME_OP_CREATE_CQ: u8 = 0x05; // admin
pub const NVME_OP_SET_FEAT:  u8 = 0x09;
pub const NVME_OP_GET_FEAT:  u8 = 0x0A;

// ─── Submission Queue ─────────────────────────────────────────────────────────
pub struct NvmeSubmissionQueue {
    pub entries: [NvmeSqEntry; NVME_QUEUE_DEPTH],
    pub tail:    AtomicU16,
    pub head:    AtomicU16,
    pub db_reg:  u64,  // doorbell register address
    pub qid:     u16,
    pub depth:   u16,
}

impl NvmeSubmissionQueue {
    pub const fn new(db_reg: u64, qid: u16) -> Self {
        NvmeSubmissionQueue {
            entries: [NvmeSqEntry {
                opcode:0, fuse:0, cid:0, nsid:0, rsvd1:[0;2],
                mptr:0, prp1:0, prp2:0,
                cdw10:0, cdw11:0, cdw12:0, cdw13:0, cdw14:0, cdw15:0,
            }; NVME_QUEUE_DEPTH],
            tail: AtomicU16::new(0), head: AtomicU16::new(0),
            db_reg, qid, depth: NVME_QUEUE_DEPTH as u16,
        }
    }

    pub fn submit(&mut self, entry: NvmeSqEntry) -> Option<u16> {
        let tail = self.tail.load(Ordering::Acquire);
        let next = (tail + 1) % self.depth;
        let head = self.head.load(Ordering::Acquire);
        if next == head { return None; } // queue full
        self.entries[tail as usize] = entry;
        self.tail.store(next, Ordering::Release);
        // Ring doorbell
        unsafe { (self.db_reg as *mut u32).write_volatile(next as u32); }
        Some(entry.cid)
    }

    pub fn available(&self) -> u16 {
        let h = self.head.load(Ordering::Acquire);
        let t = self.tail.load(Ordering::Acquire);
        if t >= h { self.depth - (t - h) - 1 } else { h - t - 1 }
    }
}

// ─── Completion Queue ─────────────────────────────────────────────────────────
pub struct NvmeCompletionQueue {
    pub entries: [NvmeCqEntry; NVME_QUEUE_DEPTH],
    pub head:    AtomicU16,
    pub phase:   AtomicU16, // expected phase bit
    pub db_reg:  u64,
    pub qid:     u16,
}

impl NvmeCompletionQueue {
    pub const fn new(db_reg: u64, qid: u16) -> Self {
        NvmeCompletionQueue {
            entries: [NvmeCqEntry { dw0:0, rsvd:0, sq_head:0, sq_id:0, cid:0, status:0 }; NVME_QUEUE_DEPTH],
            head: AtomicU16::new(0), phase: AtomicU16::new(1),
            db_reg, qid,
        }
    }

    /// Poll for completed entries. Returns number processed.
    pub fn poll(&mut self, sq: &mut NvmeSubmissionQueue) -> usize {
        let mut processed = 0;
        loop {
            let head = self.head.load(Ordering::Acquire);
            let phase = self.phase.load(Ordering::Acquire) as u8;
            let entry = &self.entries[head as usize];
            if entry.phase() as u8 != (phase & 1) { break; }
            // Update SQ head
            sq.head.store(entry.sq_head, Ordering::Release);
            processed += 1;
            let next_head = (head + 1) % NVME_QUEUE_DEPTH as u16;
            if next_head == 0 {
                // Toggle phase bit on wrap
                self.phase.store((phase as u16 ^ 1) & 1, Ordering::Release);
            }
            self.head.store(next_head, Ordering::Release);
            // Ring CQ doorbell
            unsafe { (self.db_reg as *mut u32).write_volatile(next_head as u32); }
        }
        processed
    }
}

// ─── Async I/O Request ───────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum IoStatus { Pending, Complete, Error }

#[derive(Clone, Copy)]
pub struct AsyncIoRequest {
    pub cid:     u16,
    pub status:  IoStatus,
    pub lba:     u64,
    pub blocks:  u16,
    pub buf:     u64,  // physical address of buffer
    pub write:   bool,
}

pub const MAX_PENDING_IOS: usize = 512;

// ─── NVMe Controller ──────────────────────────────────────────────────────────
pub struct NvmeController {
    pub mmio_base:  u64,
    pub admin_sq:   NvmeSubmissionQueue,
    pub admin_cq:   NvmeCompletionQueue,
    pub io_sq:      NvmeSubmissionQueue,
    pub io_cq:      NvmeCompletionQueue,
    pub nsid:       u32,
    pub block_size: u32,
    pub blocks:     u64,
    pub next_cid:   AtomicU16,
    pub pending:    [AsyncIoRequest; MAX_PENDING_IOS],
    pub n_pending:  usize,
    pub completions: AtomicU64,
    pub initialized: bool,
}

fn doorbell(mmio: u64, qid: u16, is_sq: bool) -> u64 {
    // Doorbell stride = 1 << (2 + CAP.DSTRD)
    let stride = 4u64;
    mmio + 0x1000 + (2 * qid as u64 + if is_sq { 0 } else { 1 }) * stride
}

impl NvmeController {
    pub fn new(mmio_base: u64) -> Self {
        let asq_db = doorbell(mmio_base, 0, true);
        let acq_db = doorbell(mmio_base, 0, false);
        let isq_db = doorbell(mmio_base, 1, true);
        let icq_db = doorbell(mmio_base, 1, false);
        NvmeController {
            mmio_base,
            admin_sq: NvmeSubmissionQueue::new(asq_db, 0),
            admin_cq: NvmeCompletionQueue::new(acq_db, 0),
            io_sq:    NvmeSubmissionQueue::new(isq_db, 1),
            io_cq:    NvmeCompletionQueue::new(icq_db, 1),
            nsid: 1, block_size: 512, blocks: 0,
            next_cid: AtomicU16::new(1),
            pending: [AsyncIoRequest { cid:0, status: IoStatus::Pending, lba:0, blocks:0, buf:0, write:false }; MAX_PENDING_IOS],
            n_pending: 0, completions: AtomicU64::new(0),
            initialized: false,
        }
    }

    fn mmio_read64(&self, off: u32) -> u64 {
        unsafe { ((self.mmio_base + off as u64) as *const u64).read_volatile() }
    }
    fn mmio_write32(&self, off: u32, v: u32) {
        unsafe { ((self.mmio_base + off as u64) as *mut u32).write_volatile(v); }
    }
    fn mmio_read32(&self, off: u32) -> u32 {
        unsafe { ((self.mmio_base + off as u64) as *const u32).read_volatile() }
    }

    pub fn init(&mut self) -> bool {
        // Disable controller
        self.mmio_write32(NVME_CC, 0);
        // Wait not ready
        for _ in 0..100000 {
            if self.mmio_read32(NVME_CSTS) & NVME_CSTS_RDY == 0 { break; }
            core::hint::spin_loop();
        }
        // Set AQA: admin queue depth 32 each
        self.mmio_write32(NVME_AQA, 0x001F001F);
        // Set ASQ/ACQ to physical addresses (stub: use mmio+2MB offset)
        let asq_phys = self.mmio_base + 0x200000;
        let acq_phys = self.mmio_base + 0x201000;
        unsafe {
            ((self.mmio_base + NVME_ASQ as u64) as *mut u64).write_volatile(asq_phys);
            ((self.mmio_base + NVME_ACQ as u64) as *mut u64).write_volatile(acq_phys);
        }
        // Enable with NVM command set, 4K page size, round-robin arbitration
        let cc = NVME_CC_EN | NVME_CC_CSS_NVM | NVME_CC_MPS_4K | NVME_CC_AMS_RR
                 | NVME_CC_IOSQES | NVME_CC_IOCQES;
        self.mmio_write32(NVME_CC, cc);
        // Wait ready
        for _ in 0..100000 {
            if self.mmio_read32(NVME_CSTS) & NVME_CSTS_RDY != 0 {
                self.initialized = true;
                return true;
            }
            core::hint::spin_loop();
        }
        false
    }

    fn alloc_cid(&self) -> u16 {
        self.next_cid.fetch_add(1, Ordering::Relaxed)
    }

    pub fn submit_read(&mut self, lba: u64, blocks: u16, prp1: u64) -> Option<u16> {
        if !self.initialized { return None; }
        let cid = self.alloc_cid();
        let entry = NvmeSqEntry {
            opcode: NVME_OP_READ, fuse: 0, cid, nsid: self.nsid,
            rsvd1: [0; 2], mptr: 0, prp1, prp2: 0,
            cdw10: lba as u32, cdw11: (lba >> 32) as u32,
            cdw12: (blocks - 1) as u32,
            cdw13: 0, cdw14: 0, cdw15: 0,
        };
        self.io_sq.submit(entry)?;
        if self.n_pending < MAX_PENDING_IOS {
            self.pending[self.n_pending] = AsyncIoRequest {
                cid, status: IoStatus::Pending, lba, blocks, buf: prp1, write: false
            };
            self.n_pending += 1;
        }
        Some(cid)
    }

    pub fn submit_write(&mut self, lba: u64, blocks: u16, prp1: u64) -> Option<u16> {
        if !self.initialized { return None; }
        let cid = self.alloc_cid();
        let entry = NvmeSqEntry {
            opcode: NVME_OP_WRITE, fuse: 0, cid, nsid: self.nsid,
            rsvd1: [0; 2], mptr: 0, prp1, prp2: 0,
            cdw10: lba as u32, cdw11: (lba >> 32) as u32,
            cdw12: (blocks - 1) as u32,
            cdw13: 0, cdw14: 0, cdw15: 0,
        };
        self.io_sq.submit(entry)?;
        if self.n_pending < MAX_PENDING_IOS {
            self.pending[self.n_pending] = AsyncIoRequest {
                cid, status: IoStatus::Pending, lba, blocks, buf: prp1, write: true
            };
            self.n_pending += 1;
        }
        Some(cid)
    }

    /// Called from IRQ handler to process completions.
    pub fn handle_completion(&mut self) -> usize {
        let n = self.io_cq.poll(&mut self.io_sq);
        self.completions.fetch_add(n as u64, Ordering::Relaxed);
        // Mark pending requests complete
        let head = self.io_cq.head.load(Ordering::Acquire);
        // Simplified: mark first N pending as complete
        for i in 0..n.min(self.n_pending) {
            self.pending[i].status = IoStatus::Complete;
        }
        n
    }

    pub fn completions(&self) -> u64 { self.completions.load(Ordering::Relaxed) }
}
