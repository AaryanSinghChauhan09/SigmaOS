//! SigmaOS — AArch64 MMU: 4KB Granule, 4-Level Paging
//! SPDX-License-Identifier: MIT OR GPL-2.0
//!
//! Implements TTBR0/TTBR1, TCR_EL1, MAIR_EL1 setup and TLB management.
//! Reference: ARM Architecture Reference Manual ARMv8-A, D5 (VMSAv8-64).
#![allow(dead_code, unused)]

use core::arch::asm;
use core::sync::atomic::{fence, Ordering};

// ─── Memory Attribute indices (MAIR_EL1) ─────────────────────────────────────
pub const MAIR_ATTR_NORMAL: u64 = 0xFF; // Normal, Inner/Outer WB RAWA
pub const MAIR_ATTR_DEVICE_NGNRNE: u64 = 0x00; // Device-nGnRnE
pub const MAIR_ATTR_NORMAL_NC: u64 = 0x44; // Normal non-cacheable

/// MAIR_EL1 encoding: attr0=Normal, attr1=Device, attr2=Normal-NC
pub const MAIR_EL1_VALUE: u64 = MAIR_ATTR_NORMAL
    | (MAIR_ATTR_DEVICE_NGNRNE << 8)
    | (MAIR_ATTR_NORMAL_NC << 16);

/// AttrIndx values that index into MAIR_EL1
pub const ATTRIDX_NORMAL: u64 = 0;
pub const ATTRIDX_DEVICE: u64 = 1;
pub const ATTRIDX_NORMAL_NC: u64 = 2;

// ─── TCR_EL1 fields ───────────────────────────────────────────────────────────
/// T0SZ=16 → 48-bit VA for TTBR0 (user space)
pub const TCR_T0SZ: u64 = 16;
/// T1SZ=16 → 48-bit VA for TTBR1 (kernel)
pub const TCR_T1SZ: u64 = 16;
/// TG0=00 → 4KB granule for TTBR0
pub const TCR_TG0_4KB: u64 = 0b00 << 14;
/// TG1=10 → 4KB granule for TTBR1
pub const TCR_TG1_4KB: u64 = 0b10 << 30;
/// Inner shareable (SH=11)
pub const TCR_SH0_INNER: u64 = 0b11 << 12;
pub const TCR_SH1_INNER: u64 = 0b11 << 28;
/// ORGN/IRGN = Write-Back, Write-Allocate (01)
pub const TCR_ORGN0_WBWA: u64 = 0b01 << 10;
pub const TCR_IRGN0_WBWA: u64 = 0b01 << 8;
pub const TCR_ORGN1_WBWA: u64 = 0b01 << 26;
pub const TCR_IRGN1_WBWA: u64 = 0b01 << 24;
/// IPS=101 → 48-bit physical address space
pub const TCR_IPS_48BIT: u64 = 0b101 << 32;

pub const TCR_EL1_VALUE: u64 = TCR_T0SZ
    | (TCR_T1SZ << 16)
    | TCR_TG0_4KB
    | TCR_TG1_4KB
    | TCR_SH0_INNER
    | TCR_SH1_INNER
    | TCR_ORGN0_WBWA
    | TCR_IRGN0_WBWA
    | TCR_ORGN1_WBWA
    | TCR_IRGN1_WBWA
    | TCR_IPS_48BIT;

// ─── Page Table Entry flags ───────────────────────────────────────────────────
/// AArch64 page descriptor flags (4KB granule, level-3 descriptor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmPageFlags(pub u64);

impl ArmPageFlags {
    pub const VALID: u64 = 1 << 0; // Entry is valid
    pub const TABLE: u64 = 1 << 1; // Points to next-level table (for L0–L2)
    pub const PAGE: u64 = 1 << 1; // L3 page descriptor (same bit, different level)
    pub const AF: u64 = 1 << 10; // Access Flag — must be set or takes AF fault
    pub const NG: u64 = 1 << 11; // Not Global — ASID-tagged TLB entry
    pub const SH_INNER: u64 = 0b11 << 8; // Inner Shareable
    pub const AP_RW_EL1: u64 = 0b00 << 6; // RW at EL1, no access at EL0
    pub const AP_RW_ALL: u64 = 0b01 << 6; // RW at EL0 and EL1
    pub const AP_RO_EL1: u64 = 0b10 << 6; // RO at EL1, no access at EL0
    pub const AP_RO_ALL: u64 = 0b11 << 6; // RO at EL0 and EL1
    pub const UXN: u64 = 1 << 54; // Unprivileged Execute Never
    pub const PXN: u64 = 1 << 53; // Privileged Execute Never

    /// Normal cached RW page (kernel)
    pub fn kernel_rw() -> Self {
        Self(
            Self::VALID
                | Self::PAGE
                | Self::AF
                | Self::SH_INNER
                | Self::AP_RW_EL1
                | Self::UXN
                | (ATTRIDX_NORMAL << 2),
        )
    }

    /// Normal cached RO page (kernel)
    pub fn kernel_ro() -> Self {
        Self(
            Self::VALID
                | Self::PAGE
                | Self::AF
                | Self::SH_INNER
                | Self::AP_RO_EL1
                | Self::UXN
                | Self::PXN
                | (ATTRIDX_NORMAL << 2),
        )
    }

    /// Device (MMIO) page — non-cached, non-buffered
    pub fn device() -> Self {
        Self(
            Self::VALID
                | Self::PAGE
                | Self::AF
                | Self::UXN
                | Self::PXN
                | (ATTRIDX_DEVICE << 2),
        )
    }
}

// ─── PageTableEntry ────────────────────────────────────────────────────────────

/// A single 64-bit AArch64 page table entry.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    /// An invalid (empty) entry.
    pub const EMPTY: Self = Self(0);

    /// Create a table descriptor pointing to the next-level page table.
    ///
    /// # Safety
    /// `next_table_pa` must be 4KB-aligned physical address of a valid PTE array.
    pub fn table(next_table_pa: u64) -> Self {
        debug_assert!(next_table_pa & 0xFFF == 0, "table PA must be 4KB aligned");
        Self((next_table_pa & 0x0000_FFFF_FFFF_F000) | ArmPageFlags::VALID | ArmPageFlags::TABLE)
    }

    /// Create a page (L3) descriptor.
    ///
    /// # Safety
    /// `phys` must be 4KB-aligned physical address.
    pub fn page(phys: u64, flags: ArmPageFlags) -> Self {
        debug_assert!(phys & 0xFFF == 0, "page PA must be 4KB aligned");
        Self((phys & 0x0000_FFFF_FFFF_F000) | flags.0)
    }

    #[inline(always)]
    pub fn is_valid(self) -> bool {
        self.0 & ArmPageFlags::VALID != 0
    }

    #[inline(always)]
    pub fn output_address(self) -> u64 {
        self.0 & 0x0000_FFFF_FFFF_F000
    }
}

// ─── System register accessors ────────────────────────────────────────────────

/// Write TTBR0_EL1 — user-space page table base.
///
/// # Safety
/// `addr` must be a valid physical address of a 4KB-aligned L0 page table.
#[inline]
pub unsafe fn write_ttbr0_el1(addr: u64) {
    unsafe {
        asm!(
            "msr ttbr0_el1, {0}",
            "isb",
            in(reg) addr,
            options(nostack, nomem),
        );
    }
}

/// Read TTBR0_EL1.
#[inline]
pub fn read_ttbr0_el1() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, ttbr0_el1", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Write TTBR1_EL1 — kernel page table base.
///
/// # Safety
/// `addr` must be a valid physical address of a 4KB-aligned L0 page table.
#[inline]
pub unsafe fn write_ttbr1_el1(addr: u64) {
    unsafe {
        asm!(
            "msr ttbr1_el1, {0}",
            "isb",
            in(reg) addr,
            options(nostack, nomem),
        );
    }
}

/// Read TTBR1_EL1.
#[inline]
pub fn read_ttbr1_el1() -> u64 {
    let val: u64;
    unsafe {
        asm!("mrs {0}, ttbr1_el1", out(reg) val, options(nostack, nomem));
    }
    val
}

/// Write TCR_EL1 — Translation Control Register.
///
/// # Safety
/// Modifies core memory translation behavior. Call before enabling the MMU.
#[inline]
pub unsafe fn write_tcr_el1(val: u64) {
    unsafe {
        asm!(
            "msr tcr_el1, {0}",
            "isb",
            in(reg) val,
            options(nostack, nomem),
        );
    }
}

/// Write MAIR_EL1 — Memory Attribute Indirection Register.
///
/// # Safety
/// Must be called before enabling the MMU.
#[inline]
pub unsafe fn write_mair_el1(val: u64) {
    unsafe {
        asm!("msr mair_el1, {0}", in(reg) val, options(nostack, nomem));
    }
}

/// Enable the MMU by setting SCTLR_EL1.M (bit 0).
///
/// # Safety
/// TTBR0/TTBR1, TCR_EL1, and MAIR_EL1 must all be configured correctly.
/// This is a point-of-no-return; invalid page tables cause an immediate fault.
#[inline]
pub unsafe fn enable_mmu() {
    unsafe {
        asm!(
            "dsb sy",
            "mrs x0, sctlr_el1",
            "orr x0, x0, #1",   // Set M bit
            "msr sctlr_el1, x0",
            "isb",
            out("x0") _,
            options(nostack),
        );
    }
}

// ─── TLB maintenance ──────────────────────────────────────────────────────────

/// Flush all TLB entries (vmalle1 — all ASID, EL1 stage 1).
#[inline]
pub fn flush_tlb_all() {
    unsafe {
        asm!(
            "dsb ishst",
            "tlbi vmalle1",
            "dsb ish",
            "isb",
            options(nostack, nomem),
        );
    }
}

/// Flush TLB for a single virtual address (page-selective, all ASIDs).
///
/// `va` is the virtual address; only bits[63:12] matter.
#[inline]
pub fn flush_tlb_page(va: u64) {
    // ARM TLBI uses VA >> 12 (i.e., page-frame number) in the operand register.
    let va_pfn = va >> 12;
    unsafe {
        asm!(
            "dsb ishst",
            "tlbi vaae1, {0}",
            "dsb ish",
            "isb",
            in(reg) va_pfn,
            options(nostack, nomem),
        );
    }
}

// ─── Page mapping helper ──────────────────────────────────────────────────────

/// Map a single 4KB page: physical `phys` → virtual `virt` with `flags`.
///
/// This is a *direct-map* helper — it writes directly into the L3 page table
/// pointed to by `l3_table_va`. In production the kernel page-table walker
/// allocates intermediate tables; this function handles only the L3 leaf.
///
/// # Safety
/// - `l3_table_va` must point to an array of at least 512 `PageTableEntry`s
///   that is mapped and writable.
/// - `phys` must be 4KB-aligned.
/// - `virt` bits [47:39] select the L3 table slot (0..511).
#[inline]
pub unsafe fn map_page_4kb(
    l3_table_va: *mut PageTableEntry,
    phys: u64,
    virt: u64,
    flags: ArmPageFlags,
) {
    let index = ((virt >> 12) & 0x1FF) as usize; // bits[20:12]
    debug_assert!(index < 512);
    // Safety: caller guarantees table validity and index range.
    unsafe {
        let entry = l3_table_va.add(index);
        entry.write_volatile(PageTableEntry::page(phys, flags));
    }
    // Ensure the store is visible before a TLB fill could use this entry.
    fence(Ordering::SeqCst);
}

// ─── Init ─────────────────────────────────────────────────────────────────────

/// Configure MAIR_EL1 and TCR_EL1. Does NOT enable the MMU (that is done
/// after page tables are built).
///
/// # Safety
/// Must be called at EL1 before the MMU is enabled.
pub fn init() {
    unsafe {
        write_mair_el1(MAIR_EL1_VALUE);
        write_tcr_el1(TCR_EL1_VALUE);
    }
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_table_entry_valid_bit() {
        let entry = PageTableEntry::page(0x8000_0000, ArmPageFlags::kernel_rw());
        assert!(entry.is_valid());
    }

    #[test]
    fn test_page_table_entry_output_address() {
        let phys: u64 = 0x0000_0001_0000_0000;
        let entry = PageTableEntry::page(phys, ArmPageFlags::kernel_rw());
        assert_eq!(entry.output_address(), phys);
    }

    #[test]
    fn test_table_descriptor_valid() {
        let pa: u64 = 0x4000;
        let t = PageTableEntry::table(pa);
        assert!(t.is_valid());
        assert_eq!(t.output_address(), pa);
        // Bit 1 should be set (table descriptor)
        assert_ne!(t.0 & ArmPageFlags::TABLE, 0);
    }

    #[test]
    fn test_device_page_pxn_uxn() {
        let flags = ArmPageFlags::device();
        assert_ne!(flags.0 & ArmPageFlags::PXN, 0);
        assert_ne!(flags.0 & ArmPageFlags::UXN, 0);
    }

    #[test]
    fn test_mair_attr_indices() {
        // MAIR_EL1[7:0] = attr0 = Normal
        assert_eq!(MAIR_EL1_VALUE & 0xFF, MAIR_ATTR_NORMAL);
        // MAIR_EL1[15:8] = attr1 = Device
        assert_eq!((MAIR_EL1_VALUE >> 8) & 0xFF, MAIR_ATTR_DEVICE_NGNRNE);
        // MAIR_EL1[23:16] = attr2 = Normal-NC
        assert_eq!((MAIR_EL1_VALUE >> 16) & 0xFF, MAIR_ATTR_NORMAL_NC);
    }

    #[test]
    fn test_tcr_t0sz() {
        // T0SZ occupies bits[5:0]
        let t0sz = TCR_EL1_VALUE & 0x3F;
        assert_eq!(t0sz, 16);
    }

    #[test]
    fn test_kernel_rw_flags_af_set() {
        let flags = ArmPageFlags::kernel_rw();
        assert_ne!(flags.0 & ArmPageFlags::AF, 0, "AF must be set to avoid fault");
    }
}
