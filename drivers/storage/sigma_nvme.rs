// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/storage/sigma_nvme.rs — NVMe PCIe Storage Driver
// Language: Rust #![no_std] — no libc, no alloc, no third-party crates
// Pattern: OOP via NvmeController struct implementing BlockDevice trait

#![no_std]

// ── NVMe Register Offsets (Controller Properties — BAR0) ──────────────────

const REG_CAP:   usize = 0x00; // Controller Capabilities (8 bytes)
const REG_VS:    usize = 0x08; // Version
const REG_CC:    usize = 0x14; // Controller Configuration
const REG_CSTS:  usize = 0x1C; // Controller Status
const REG_AQA:   usize = 0x24; // Admin Queue Attributes
const REG_ASQ:   usize = 0x28; // Admin Submission Queue Base (8 bytes)
const REG_ACQ:   usize = 0x30; // Admin Completion Queue Base (8 bytes)
const SQ0TDBL:   usize = 0x1000; // Submission Queue 0 Tail Doorbell

// Controller Configuration bits
const CC_ENABLE: u32 = 1 << 0;
const CC_CSS:    u32 = 0 << 4; // NVM command set
const CC_MPS:    u32 = 0 << 7; // 4KB memory page size
const CC_AMS:    u32 = 0 << 11; // Round Robin arbitration
const CC_SHN:    u32 = 0 << 14; // No shutdown
const CC_IOSQES: u32 = 6 << 16; // IO SQ entry size = 64 bytes (2^6)
const CC_IOCQES: u32 = 4 << 20; // IO CQ entry size = 16 bytes (2^4)

// Controller Status bits
const CSTS_RDY:  u32 = 1 << 0;
const CSTS_CFS:  u32 = 1 << 1; // Controller Fatal Status

// ── Admin Command Opcodes ─────────────────────────────────────────────────

const ADM_IDENTIFY:   u8 = 0x06;
const ADM_CREATE_SQ:  u8 = 0x01;
const ADM_CREATE_CQ:  u8 = 0x05;
const IO_READ:        u8 = 0x02;
const IO_WRITE:       u8 = 0x01;

// ── Submission Queue Entry (64 bytes) ────────────────────────────────────

#[repr(C, align(64))]
struct SqEntry {
    opc:   u8,
    fuse:  u8,
    rsvd:  u16,
    nsid:  u32,
    rsvd2: u64,
    mptr:  u64,
    prp1:  u64,
    prp2:  u64,
    cdw10: u32,
    cdw11: u32,
    cdw12: u32,
    cdw13: u32,
    cdw14: u32,
    cdw15: u32,
}

// ── Completion Queue Entry (16 bytes) ───────────────────────────────────

#[repr(C, align(16))]
struct CqEntry {
    dw0:   u32,
    dw1:   u32,
    sq_hd: u16,
    sq_id: u16,
    cid:   u16,
    phase: u16, // bit 0 = phase tag
}

// ── Block Device Trait ────────────────────────────────────────────────────

pub trait BlockDevice: Send + Sync {
    fn block_size(&self) -> usize;
    fn block_count(&self) -> u64;
    fn read_blocks(&mut self, lba: u64, count: u16, buf: &mut [u8]) -> bool;
    fn write_blocks(&mut self, lba: u64, count: u16, buf: &[u8]) -> bool;
    fn flush(&mut self) -> bool;
}

// ── NVMe Driver ──────────────────────────────────────────────────────────

const QUEUE_DEPTH: usize = 64;

pub struct NvmeController {
    mmio:       usize,
    admin_sq:   [SqEntry;  QUEUE_DEPTH],
    admin_cq:   [CqEntry;  QUEUE_DEPTH],
    io_sq:      [SqEntry;  QUEUE_DEPTH],
    io_cq:      [CqEntry;  QUEUE_DEPTH],
    admin_sq_t: u16,
    admin_cq_h: u16,
    io_sq_t:    u16,
    io_cq_h:    u16,
    phase:      bool,
    ns_blocks:  u64,
    ns_blk_sz:  usize,
    cid:        u16,
}

impl NvmeController {
    pub fn new(mmio: usize) -> Self {
        Self {
            mmio,
            admin_sq:   core::array::from_fn(|_| unsafe { core::mem::zeroed() }),
            admin_cq:   core::array::from_fn(|_| unsafe { core::mem::zeroed() }),
            io_sq:      core::array::from_fn(|_| unsafe { core::mem::zeroed() }),
            io_cq:      core::array::from_fn(|_| unsafe { core::mem::zeroed() }),
            admin_sq_t: 0, admin_cq_h: 0,
            io_sq_t:    0, io_cq_h:    0,
            phase:      true,
            ns_blocks:  0, ns_blk_sz: 512,
            cid:        0,
        }
    }

    pub fn probe(vendor: u16, class: u8, subclass: u8, prog_if: u8) -> bool {
        class == 0x01 && subclass == 0x08 && prog_if == 0x02
    }

    pub fn init(&mut self) -> bool {
        // 1. Disable controller
        self.write32(REG_CC, 0);
        // Wait for RDY to clear
        for _ in 0..100_000 {
            if self.read32(REG_CSTS) & CSTS_RDY == 0 { break; }
        }
        // 2. Set up Admin Queue addresses and sizes
        let asq_phys = self.admin_sq.as_ptr() as u64;
        let acq_phys = self.admin_cq.as_ptr() as u64;
        self.write32(REG_AQA, ((QUEUE_DEPTH as u32 - 1) << 16) | (QUEUE_DEPTH as u32 - 1));
        self.write64(REG_ASQ, asq_phys);
        self.write64(REG_ACQ, acq_phys);
        // 3. Enable controller
        let cc = CC_ENABLE | CC_CSS | CC_MPS | CC_AMS | CC_SHN | CC_IOSQES | CC_IOCQES;
        self.write32(REG_CC, cc);
        // 4. Wait for RDY
        let mut ready = false;
        for _ in 0..1_000_000 {
            let csts = self.read32(REG_CSTS);
            if csts & CSTS_CFS != 0 { return false; }
            if csts & CSTS_RDY != 0 { ready = true; break; }
        }
        if !ready { return false; }
        // 5. Create IO queues and identify namespace (simplified)
        self.ns_blocks  = 0x100000; // 512 MB placeholder
        self.ns_blk_sz  = 512;
        true
    }

    fn submit_admin(&mut self, entry: SqEntry) -> bool {
        let t = self.admin_sq_t as usize;
        self.admin_sq[t] = entry;
        self.admin_sq_t = ((t + 1) % QUEUE_DEPTH) as u16;
        self.write32(SQ0TDBL, self.admin_sq_t as u32);
        // Poll completion
        for _ in 0..100_000 {
            let cq = &self.admin_cq[self.admin_cq_h as usize];
            if (cq.phase & 1 != 0) == self.phase {
                let ok = (cq.dw3() >> 17) == 0;
                self.admin_cq_h = ((self.admin_cq_h as usize + 1) % QUEUE_DEPTH) as u16;
                if self.admin_cq_h == 0 { self.phase = !self.phase; }
                return ok;
            }
        }
        false
    }

    fn read32(&self, off: usize) -> u32 {
        unsafe { ((self.mmio + off) as *const volatile u32).read_volatile() }
    }
    fn write32(&self, off: usize, v: u32) {
        unsafe { ((self.mmio + off) as *mut volatile u32).write_volatile(v) }
    }
    fn write64(&self, off: usize, v: u64) {
        self.write32(off,     (v & 0xFFFF_FFFF) as u32);
        self.write32(off + 4, (v >> 32) as u32);
    }
    fn next_cid(&mut self) -> u16 { self.cid = self.cid.wrapping_add(1); self.cid }
}

trait Dw3 { fn dw3(&self) -> u32; }
impl Dw3 for CqEntry { fn dw3(&self) -> u32 { self.phase as u32 } }

impl BlockDevice for NvmeController {
    fn block_size(&self)  -> usize { self.ns_blk_sz }
    fn block_count(&self) -> u64   { self.ns_blocks  }

    fn read_blocks(&mut self, lba: u64, count: u16, buf: &mut [u8]) -> bool {
        if buf.len() < (count as usize) * self.ns_blk_sz { return false; }
        let mut entry: SqEntry = unsafe { core::mem::zeroed() };
        entry.opc  = IO_READ;
        entry.nsid = 1;
        entry.prp1 = buf.as_ptr() as u64;
        entry.cdw10 = (lba & 0xFFFF_FFFF) as u32;
        entry.cdw11 = (lba >> 32) as u32;
        entry.cdw12 = (count as u32).wrapping_sub(1);
        self.submit_admin(entry)
    }

    fn write_blocks(&mut self, lba: u64, count: u16, buf: &[u8]) -> bool {
        if buf.len() < (count as usize) * self.ns_blk_sz { return false; }
        let mut entry: SqEntry = unsafe { core::mem::zeroed() };
        entry.opc  = IO_WRITE;
        entry.nsid = 1;
        entry.prp1 = buf.as_ptr() as u64;
        entry.cdw10 = (lba & 0xFFFF_FFFF) as u32;
        entry.cdw11 = (lba >> 32) as u32;
        entry.cdw12 = (count as u32).wrapping_sub(1);
        self.submit_admin(entry)
    }

    fn flush(&mut self) -> bool {
        let mut entry: SqEntry = unsafe { core::mem::zeroed() };
        entry.opc  = 0x00; // Flush opcode
        entry.nsid = 1;
        self.submit_admin(entry)
    }
}
