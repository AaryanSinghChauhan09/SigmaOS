// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: NATIVE SOVEREIGN — NVMe Storage Driver (Rust, no_std)
//! =========================================================================
//!
//! Replaces: drivers/sigma/sigma_nvme.cpp
//! Language: Rust  #![no_std]  #![no_builtins]
//!
//! Bare-metal NVMe 1.4 host controller driver.
//! ZERO standard library. ZERO predefined functions. ZERO external crates.
//! All MMIO register access via raw volatile pointer dereferences.
//! All data structures are stack-resident — no heap allocation.
//!
//! Selected at build time with: TARGET_OS=sigma
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); }
    }
}

// ── Primitive types (sovereign aliases) ────────────────────────────────────
type U8   = u8;
type U16  = u16;
type U32  = u32;
type U64  = u64;
type I32  = i32;
type Bool = bool;

// ═══════════════════════════════════════════════════════════════════════════
// § 1. NVMe specification constants (NVM Express 1.4)
//      Defined from the specification document — not from any header.
// ═══════════════════════════════════════════════════════════════════════════

const NVME_REG_CAP  : U32 = 0x0000; // Controller Capabilities (64-bit)
const NVME_REG_VS   : U32 = 0x0008; // Version
const NVME_REG_INTMS: U32 = 0x000C; // Interrupt Mask Set
const NVME_REG_INTMC: U32 = 0x0010; // Interrupt Mask Clear
const NVME_REG_CC   : U32 = 0x0014; // Controller Configuration
const NVME_REG_CSTS : U32 = 0x001C; // Controller Status
const NVME_REG_NSSR : U32 = 0x0020; // NVM Subsystem Reset
const NVME_REG_AQA  : U32 = 0x0024; // Admin Queue Attributes
const NVME_REG_ASQ  : U32 = 0x0028; // Admin Submission Queue Base (64-bit)
const NVME_REG_ACQ  : U32 = 0x0030; // Admin Completion Queue Base (64-bit)
const NVME_REG_CMBLOC: U32 = 0x0038; // Controller Memory Buffer Location

// CC register bit fields
const NVME_CC_EN    : U32 = 1 << 0;  // Enable
const NVME_CC_CSS   : U32 = 0 << 4;  // Command Set Selected: NVM
const NVME_CC_MPS   : U32 = 0 << 7;  // Memory Page Size: 4 KiB
const NVME_CC_AMS   : U32 = 0 << 11; // Arbitration Mechanism: Round Robin
const NVME_CC_SHN   : U32 = 0 << 14; // Shutdown Notification: None
const NVME_CC_IOSQES: U32 = 6 << 16; // I/O SQ Entry Size: 64 bytes (2^6)
const NVME_CC_IOCQES: U32 = 4 << 20; // I/O CQ Entry Size: 16 bytes (2^4)

// CSTS register bit fields
const NVME_CSTS_RDY : U32 = 1 << 0;  // Ready
const NVME_CSTS_CFS : U32 = 1 << 1;  // Controller Fatal Status
const NVME_CSTS_SHST: U32 = 3 << 2;  // Shutdown Status mask

// NVM Subsystem Reset value (write to NSSR to trigger)
const NVME_NSSR_NSSRC: U32 = 0x4E56_4D65; // 'NVMe'

// Maximum spin iterations before declaring timeout
const NVME_POLL_MAX: U32 = 500_000;

// Admin queue depth (entries per queue — spec min: 2, max: 4096)
const NVME_ADMIN_Q_DEPTH: usize = 64;

// Submission Queue Entry size: 64 bytes
const NVME_SQE_SIZE: usize = 64;
// Completion Queue Entry size: 16 bytes
const NVME_CQE_SIZE: usize = 16;

// ═══════════════════════════════════════════════════════════════════════════
// § 2. NVMe Submission Queue Entry (SQE) — 64 bytes, NVM Express 1.4 §4.6
// ═══════════════════════════════════════════════════════════════════════════

/// NVMe Admin/IO Submission Queue Entry.
/// Laid out exactly as the specification mandates — 64 bytes total.
#[repr(C, packed)]
struct NvmeSqe {
    cdw0 : U32, // Command Dword 0: opcode, fuse, psdt, CID
    nsid : U32, // Namespace Identifier
    cdw2 : U32, // Reserved
    cdw3 : U32, // Reserved
    mptr : U64, // Metadata Pointer
    prp1 : U64, // PRP Entry 1 (data pointer)
    prp2 : U64, // PRP Entry 2 (second page or PRP list)
    cdw10: U32, // Command-specific Dword 10
    cdw11: U32, // Command-specific Dword 11
    cdw12: U32, // Command-specific Dword 12
    cdw13: U32, // Command-specific Dword 13
    cdw14: U32, // Command-specific Dword 14
    cdw15: U32, // Command-specific Dword 15
}

/// Compile-time size assertion — SQE must be exactly 64 bytes.
const _: () = assert!(core::mem::size_of::<NvmeSqe>() == NVME_SQE_SIZE);

// ═══════════════════════════════════════════════════════════════════════════
// § 3. NVMe Completion Queue Entry (CQE) — 16 bytes, NVM Express 1.4 §4.7
// ═══════════════════════════════════════════════════════════════════════════

/// NVMe Admin/IO Completion Queue Entry.
#[repr(C, packed)]
struct NvmeCqe {
    dw0: U32, // Command-specific result
    dw1: U32, // Reserved
    sqhd_sqid: U32, // SQ Head Pointer (15:0) + SQ Identifier (31:16)
    p_status_cid: U32, // Phase tag (0), Status Field (14:1), CID (31:16)
}

const _: () = assert!(core::mem::size_of::<NvmeCqe>() == NVME_CQE_SIZE);

// ═══════════════════════════════════════════════════════════════════════════
// § 4. Volatile MMIO helpers (inline — no external module dependency)
// ═══════════════════════════════════════════════════════════════════════════

#[inline(always)]
unsafe fn read32(base: U64, off: U32) -> U32 {
    core::ptr::read_volatile((base + off as U64) as *const U32)
}

#[inline(always)]
unsafe fn write32(base: U64, off: U32, val: U32) {
    core::ptr::write_volatile((base + off as U64) as *mut U32, val);
}

#[inline(always)]
unsafe fn read64(base: U64, off: U32) -> U64 {
    core::ptr::read_volatile((base + off as U64) as *const U64)
}

#[inline(always)]
unsafe fn write64(base: U64, off: U32, val: U64) {
    core::ptr::write_volatile((base + off as U64) as *mut U64, val);
}

/// Spin-poll a 32-bit MMIO register until `(reg & mask) == expected`.
/// Returns `true` on match, `false` on timeout.
#[inline]
unsafe fn poll32(base: U64, off: U32, mask: U32, expected: U32) -> Bool {
    let mut i: U32 = 0;
    while i < NVME_POLL_MAX {
        if (read32(base, off) & mask) == expected { return true; }
        core::arch::asm!("pause", options(nomem, nostack, preserves_flags));
        i += 1;
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════
// § 5. SigmaNvme driver state — stack-resident, no heap
// ═══════════════════════════════════════════════════════════════════════════

/// Admin Submission Queue — stack-resident ring buffer.
#[repr(C, align(4096))]
struct AdminSQ {
    entries: [NvmeSqe; NVME_ADMIN_Q_DEPTH],
    tail: usize,
}

impl AdminSQ {
    const fn new() -> Self {
        AdminSQ {
            entries: unsafe { core::mem::zeroed() },
            tail: 0,
        }
    }
}

/// Admin Completion Queue — stack-resident ring buffer.
#[repr(C, align(4096))]
struct AdminCQ {
    entries: [NvmeCqe; NVME_ADMIN_Q_DEPTH],
    head: usize,
    phase: U32,
}

impl AdminCQ {
    const fn new() -> Self {
        AdminCQ {
            entries: unsafe { core::mem::zeroed() },
            head: 0,
            phase: 1,
        }
    }
}

/// Full NVMe driver state — everything lives on the stack / BSS.
pub struct SigmaNvme {
    mmio_base  : U64,
    admin_sq   : AdminSQ,
    admin_cq   : AdminCQ,
    read_cmds  : U64,
    write_cmds : U64,
    initialized: Bool,
}

impl SigmaNvme {
    /// Construct an uninitialised driver instance.
    pub const fn new() -> Self {
        SigmaNvme {
            mmio_base  : 0,
            admin_sq   : AdminSQ::new(),
            admin_cq   : AdminCQ::new(),
            read_cmds  : 0,
            write_cmds : 0,
            initialized: false,
        }
    }

    // ── § 5a. init ─────────────────────────────────────────────────────────

    /// Attach the driver to the NVMe controller at `mmio_base`.
    ///
    /// Sequence (NVM Express 1.4 §3.5.1):
    ///   1. Disable controller (CC.EN = 0), wait CSTS.RDY = 0
    ///   2. Configure Admin queue attributes (AQA)
    ///   3. Set Admin SQ / CQ base addresses (ASQ, ACQ)
    ///   4. Enable controller (CC.EN = 1), wait CSTS.RDY = 1
    ///
    /// # Safety
    /// `mmio_base` must be a valid, identity-mapped MMIO address for the
    /// NVMe controller's BAR0.
    pub unsafe fn init(&mut self, mmio_base: U64) -> I32 {
        self.mmio_base = mmio_base;

        // Step 1: Disable controller
        let cc = read32(mmio_base, NVME_REG_CC);
        write32(mmio_base, NVME_REG_CC, cc & !NVME_CC_EN);
        if !poll32(mmio_base, NVME_REG_CSTS, NVME_CSTS_RDY, 0) {
            return -4; // SIGMA_TIMEOUT
        }

        // Step 2: Admin Queue Attributes
        //   ASQS (15:0)  = NVME_ADMIN_Q_DEPTH - 1
        //   ACQS (31:16) = NVME_ADMIN_Q_DEPTH - 1
        let aqa: U32 = ((NVME_ADMIN_Q_DEPTH as U32 - 1) << 16)
                      | (NVME_ADMIN_Q_DEPTH as U32 - 1);
        write32(mmio_base, NVME_REG_AQA, aqa);

        // Step 3: Admin SQ / CQ physical base addresses
        let sq_phys = self.admin_sq.entries.as_ptr() as U64;
        let cq_phys = self.admin_cq.entries.as_ptr() as U64;
        write64(mmio_base, NVME_REG_ASQ, sq_phys);
        write64(mmio_base, NVME_REG_ACQ, cq_phys);

        // Step 4: Enable controller with standard CSS / MPS / AMS
        let new_cc: U32 = NVME_CC_EN
            | NVME_CC_CSS
            | NVME_CC_MPS
            | NVME_CC_AMS
            | NVME_CC_IOSQES
            | NVME_CC_IOCQES;
        write32(mmio_base, NVME_REG_CC, new_cc);
        if !poll32(mmio_base, NVME_REG_CSTS, NVME_CSTS_RDY, NVME_CSTS_RDY) {
            return -4; // SIGMA_TIMEOUT
        }

        // Check for controller fatal status
        if (read32(mmio_base, NVME_REG_CSTS) & NVME_CSTS_CFS) != 0 {
            return -1; // SIGMA_ERR
        }

        self.initialized = true;
        0 // SIGMA_OK
    }

    // ── § 5b. Async read ───────────────────────────────────────────────────

    /// Submit an asynchronous read for `count` 512-byte blocks at `lba`
    /// into the physical buffer at `buf_phys`.
    ///
    /// Returns a positive command identifier, or a negative error code.
    pub unsafe fn async_read(&mut self, lba: U64, count: U16, buf_phys: U64) -> I32 {
        if !self.initialized || count == 0 || buf_phys == 0 { return -3; }

        let tail = self.admin_sq.tail;
        let sqe = &mut self.admin_sq.entries[tail % NVME_ADMIN_Q_DEPTH];

        // Opcode 0x02 = Read (NVM Command Set, NVM Express 1.4 §6.8)
        sqe.cdw0  = 0x02 | ((self.read_cmds as U32 & 0xFFFF) << 16);
        sqe.nsid  = 1; // Namespace 1
        sqe.prp1  = buf_phys;
        sqe.prp2  = 0;
        sqe.cdw10 = lba as U32;
        sqe.cdw11 = (lba >> 32) as U32;
        sqe.cdw12 = (count as U32).wrapping_sub(1); // 0-based count

        self.admin_sq.tail = tail.wrapping_add(1);
        self.read_cmds = self.read_cmds.wrapping_add(1);

        // Ring doorbell — SQ Tail Doorbell at offset 0x1000 + 8*(2*qid)
        write32(self.mmio_base, 0x1000, self.admin_sq.tail as U32);

        (self.read_cmds & 0xFFFF) as I32
    }

    // ── § 5c. Async write ──────────────────────────────────────────────────

    /// Submit an asynchronous write of `count` 512-byte blocks at `lba`
    /// from the physical buffer at `buf_phys`.
    pub unsafe fn async_write(&mut self, lba: U64, count: U16, buf_phys: U64) -> I32 {
        if !self.initialized || count == 0 || buf_phys == 0 { return -3; }

        let tail = self.admin_sq.tail;
        let sqe = &mut self.admin_sq.entries[tail % NVME_ADMIN_Q_DEPTH];

        // Opcode 0x01 = Write (NVM Express 1.4 §6.11)
        sqe.cdw0  = 0x01 | ((self.write_cmds as U32 & 0xFFFF) << 16);
        sqe.nsid  = 1;
        sqe.prp1  = buf_phys;
        sqe.prp2  = 0;
        sqe.cdw10 = lba as U32;
        sqe.cdw11 = (lba >> 32) as U32;
        sqe.cdw12 = (count as U32).wrapping_sub(1);

        self.admin_sq.tail = tail.wrapping_add(1);
        self.write_cmds = self.write_cmds.wrapping_add(1);
        write32(self.mmio_base, 0x1000, self.admin_sq.tail as U32);

        (self.write_cmds & 0xFFFF) as I32
    }

    /// Return number of read commands submitted.
    pub fn read_cmds(&self) -> U64 { self.read_cmds }
    /// Return number of write commands submitted.
    pub fn write_cmds(&self) -> U64 { self.write_cmds }
}

// ── Global singleton (BSS-resident) ───────────────────────────────────────
static mut G_NVME: SigmaNvme = SigmaNvme::new();

// ── C bridge ──────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_nvme_init(mmio_base: U64) -> I32 {
    G_NVME.init(mmio_base)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_nvme_read(lba: U64, count: U32, buf_phys: U64) -> I32 {
    G_NVME.async_read(lba, count as U16, buf_phys)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_nvme_write(lba: U64, count: U32, buf_phys: U64) -> I32 {
    G_NVME.async_write(lba, count as U16, buf_phys)
}
