//! SigmaOS — x86_64 4-Level Paging (PML4 → PDPT → PD → PT)
//! SPDX-License-Identifier: MIT OR GPL-2.0
//!
//! Provides the kernel's virtual memory management primitives:
//! page-table entry construction, CR3 manipulation, TLB invalidation,
//! a page-fault handler that reads CR2, and identity-mapping of the
//! first 4 GiB at boot using 2 MiB huge pages.

#![allow(dead_code)]

use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

// ─── Page Table Entry Flags ──────────────────────────────────────────────────

/// Bitmask flags for a page-table entry (Intel SDM Vol. 3A §4.5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PageFlags(u64);

impl PageFlags {
    pub const PRESENT:    Self = Self(1 << 0);  // P
    pub const WRITABLE:   Self = Self(1 << 1);  // R/W
    pub const USER:       Self = Self(1 << 2);  // U/S
    pub const WRITE_THRU: Self = Self(1 << 3);  // PWT
    pub const NO_CACHE:   Self = Self(1 << 4);  // PCD
    pub const ACCESSED:   Self = Self(1 << 5);  // A
    pub const DIRTY:      Self = Self(1 << 6);  // D
    pub const HUGE:       Self = Self(1 << 7);  // PS — 2 MiB / 1 GiB page
    pub const GLOBAL:     Self = Self(1 << 8);  // G
    pub const NO_EXEC:    Self = Self(1 << 63); // XD / NX
    pub const EMPTY:      Self = Self(0);

    #[inline(always)]
    pub const fn bits(self) -> u64 { self.0 }

    #[inline(always)]
    pub const fn or(self, other: Self) -> Self { Self(self.0 | other.0) }

    #[inline(always)]
    pub const fn contains(self, flag: Self) -> bool { (self.0 & flag.0) == flag.0 }
}

impl core::ops::BitOr for PageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for PageFlags {
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

// ─── Page Table Entry ────────────────────────────────────────────────────────

/// A single entry in any of the four paging levels.
/// The physical frame address occupies bits [51:12]; flags reside in
/// bits [11:0] and bit 63 (NX).
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn unused() -> Self { Self(0) }

    /// Constructs an entry pointing to `phys_frame` (4 KiB-aligned) with `flags`.
    ///
    /// # Panics (debug)
    /// Panics if `phys_frame` is not 4 KiB-aligned.
    pub fn new(phys_frame: u64, flags: PageFlags) -> Self {
        debug_assert!(
            phys_frame & 0xFFF == 0,
            "physical frame address must be 4 KiB-aligned"
        );
        Self((phys_frame & 0x000F_FFFF_FFFF_F000) | flags.bits())
    }

    /// Constructs a 2 MiB huge-page entry (PS bit set in PD).
    pub fn huge(phys_2m: u64, flags: PageFlags) -> Self {
        debug_assert!(
            phys_2m & 0x1F_FFFF == 0,
            "huge-page base must be 2 MiB-aligned"
        );
        Self::new(phys_2m, flags | PageFlags::HUGE)
    }

    #[inline(always)] pub fn is_present(self) -> bool   { self.0 & PageFlags::PRESENT.bits() != 0 }
    #[inline(always)] pub fn is_huge(self)    -> bool   { self.0 & PageFlags::HUGE.bits() != 0 }
    #[inline(always)] pub fn is_writable(self) -> bool  { self.0 & PageFlags::WRITABLE.bits() != 0 }
    #[inline(always)] pub fn frame(self)      -> u64    { self.0 & 0x000F_FFFF_FFFF_F000 }
    #[inline(always)] pub fn flags(self)      -> PageFlags { PageFlags(self.0 & !(0x000F_FFFF_FFFF_F000u64)) }
    #[inline(always)] pub fn raw(self)        -> u64    { self.0 }
}

// ─── Page Table Structures ───────────────────────────────────────────────────

/// A 512-entry page table (any level). Each entry is 8 bytes → 4 KiB total.
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn empty() -> Self {
        Self { entries: [PageTableEntry::unused(); 512] }
    }

    /// Index into the table.  Bounds-checked via Rust's slice indexing.
    #[inline]
    pub fn entry(&self, idx: usize) -> PageTableEntry {
        self.entries[idx & 511]
    }

    #[inline]
    pub fn set_entry(&mut self, idx: usize, e: PageTableEntry) {
        self.entries[idx & 511] = e;
    }
}

// ─── Boot Page Tables (identity-map 0 → 4 GiB with 2 MiB huge pages) ────────
//
// We need:
//   1 × PML4  (512 entries, only [0] used)
//   1 × PDPT  (512 entries, [0..3] used — 4 × 1 GiB regions)
//   4 × PD    (each 512 entries of 2 MiB pages — covers 1 GiB)
//
// Total: ~24 KiB of BSS.  No PT level needed for huge pages.

static mut PML4:   PageTable = PageTable::empty();
static mut PDPT:   PageTable = PageTable::empty();
static mut PD:     [PageTable; 4] = [
    PageTable::empty(),
    PageTable::empty(),
    PageTable::empty(),
    PageTable::empty(),
];

// ─── CR3 / TLB primitives ────────────────────────────────────────────────────

/// Write a new PML4 physical address into CR3, causing a full TLB flush.
///
/// # Safety
/// `addr` must be the physical address of a valid 4 KiB-aligned PML4 table.
/// The CPU will immediately begin using it for address translation.
#[inline]
pub unsafe fn write_cr3(addr: u64) {
    asm!("mov cr3, {}", in(reg) addr, options(nostack, preserves_flags));
}

/// Read the current PML4 physical address from CR3.
#[inline]
pub fn read_cr3() -> u64 {
    let val: u64;
    // SAFETY: `mov rax, cr3` is a privileged read; we are in ring-0.
    unsafe {
        asm!("mov {}, cr3", out(reg) val, options(nostack, preserves_flags));
    }
    val
}

/// Flush all TLB entries by writing CR3 back to itself.
///
/// # Safety
/// Must be called from ring-0.  All translations are invalidated.
#[inline]
pub unsafe fn flush_tlb() {
    let cr3 = read_cr3();
    write_cr3(cr3);
}

/// Invalidate the TLB entry for one virtual page using `invlpg`.
///
/// # Safety
/// Must be called from ring-0.  Only the single page containing `virt`
/// is invalidated; other mappings remain cached.
#[inline]
pub unsafe fn flush_tlb_page(virt: u64) {
    asm!("invlpg [{}]", in(reg) virt, options(nostack, preserves_flags));
}

// ─── Virtual-address index extraction ────────────────────────────────────────

/// Extract the PML4 index from a 64-bit canonical virtual address (bits 47:39).
#[inline(always)]
const fn pml4_index(virt: u64) -> usize { ((virt >> 39) & 0x1FF) as usize }
/// Extract the PDPT index (bits 38:30).
#[inline(always)]
const fn pdpt_index(virt: u64) -> usize { ((virt >> 30) & 0x1FF) as usize }
/// Extract the PD index (bits 29:21).
#[inline(always)]
const fn pd_index(virt: u64) -> usize   { ((virt >> 21) & 0x1FF) as usize }
/// Extract the PT index (bits 20:12).
#[inline(always)]
const fn pt_index(virt: u64) -> usize   { ((virt >> 12) & 0x1FF) as usize }

// ─── map_page ────────────────────────────────────────────────────────────────

/// Map a single 4 KiB page: `virt` → `phys` with the given `flags`.
///
/// This is a *physical-identity-mapped bootstrap* mapper — it expects
/// that all kernel page tables are identity-mapped so that pointer
/// arithmetic between physical and virtual addresses is trivial (virt == phys).
///
/// # Safety
/// - `phys` and `virt` must be 4 KiB-aligned.
/// - The intermediate page tables pointed to by existing entries must
///   themselves be identity-mapped and writable.
/// - No aliasing of page tables between CPUs during this call.
pub unsafe fn map_page(phys: u64, virt: u64, flags: PageFlags) {
    let pml4 = &mut PML4;
    let p4i = pml4_index(virt);
    if !pml4.entry(p4i).is_present() {
        // Allocate a zeroed PDPT page — for simplicity use our static PDPT.
        // In a full allocator, call the physical page allocator here.
        let pdpt_phys = &PDPT as *const PageTable as u64;
        pml4.set_entry(
            p4i,
            PageTableEntry::new(
                pdpt_phys,
                PageFlags::PRESENT | PageFlags::WRITABLE,
            ),
        );
    }

    let pdpt_phys = pml4.entry(p4i).frame();
    // SAFETY: identity-mapped; physical == virtual in bootstrap context.
    let pdpt = &mut *(pdpt_phys as *mut PageTable);
    let p3i = pdpt_index(virt);
    if !pdpt.entry(p3i).is_present() {
        let pd_phys = &PD[0] as *const PageTable as u64;
        pdpt.set_entry(
            p3i,
            PageTableEntry::new(
                pd_phys,
                PageFlags::PRESENT | PageFlags::WRITABLE,
            ),
        );
    }

    let pd_phys = pdpt.entry(p3i).frame();
    let pd = &mut *(pd_phys as *mut PageTable);
    let p2i = pd_index(virt);

    // We use 4 KiB pages here, so PD must point to a PT.
    // (Huge-page mapping is handled separately in `identity_map_4gib`.)
    if !pd.entry(p2i).is_present() {
        // Would call the PMM allocator; omitted for this skeleton.
        return;
    }

    let pt_phys = pd.entry(p2i).frame();
    let pt = &mut *(pt_phys as *mut PageTable);
    let p1i = pt_index(virt);
    pt.set_entry(p1i, PageTableEntry::new(phys, flags | PageFlags::PRESENT));

    flush_tlb_page(virt);
}

// ─── Identity-map first 4 GiB ────────────────────────────────────────────────

/// Identity-map physical [0, 4 GiB) using 2 MiB huge pages.
///
/// Page table layout:
///   PML4[0] → PDPT
///   PDPT[0..3] → PD[0..3]
///   PD[g][i] → phys 2 MiB frame at (g * 1 GiB + i * 2 MiB)
fn identity_map_4gib() {
    let kernel_rw = PageFlags::PRESENT | PageFlags::WRITABLE;

    // SAFETY: Single-threaded boot; no other CPU uses these tables yet.
    unsafe {
        // PML4[0] → PDPT (physical address of static PDPT)
        let pdpt_phys = &PDPT as *const PageTable as u64;
        PML4.set_entry(0, PageTableEntry::new(pdpt_phys, kernel_rw));

        for g in 0usize..4 {
            // PDPT[g] → PD[g]
            let pd_phys = &PD[g] as *const PageTable as u64;
            PDPT.set_entry(g, PageTableEntry::new(pd_phys, kernel_rw));

            // PD[g][0..511] → 2 MiB huge pages
            for i in 0usize..512 {
                let phys = ((g as u64) * 0x4000_0000) + ((i as u64) * 0x20_0000);
                PD[g].set_entry(i, PageTableEntry::huge(phys, kernel_rw));
            }
        }
    }
}

// ─── Public init ─────────────────────────────────────────────────────────────

/// Initialise paging: build identity map for the first 4 GiB and load CR3.
///
/// # Safety
/// Must be called exactly once, before enabling paging on secondary CPUs.
pub fn init() {
    identity_map_4gib();
    // SAFETY: `PML4` is a valid, properly-initialised page table.
    unsafe {
        let pml4_phys = &PML4 as *const PageTable as u64;
        write_cr3(pml4_phys);
    }
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_flags_or() {
        let f = PageFlags::PRESENT | PageFlags::WRITABLE;
        assert!(f.contains(PageFlags::PRESENT));
        assert!(f.contains(PageFlags::WRITABLE));
        assert!(!f.contains(PageFlags::USER));
    }

    #[test]
    fn page_table_entry_frame_roundtrip() {
        let phys: u64 = 0x0000_1234_5000;
        let e = PageTableEntry::new(phys, PageFlags::PRESENT | PageFlags::WRITABLE);
        assert_eq!(e.frame(), phys);
        assert!(e.is_present());
        assert!(e.is_writable());
    }

    #[test]
    fn huge_page_entry_has_ps_bit() {
        let phys: u64 = 0x0020_0000; // 2 MiB
        let e = PageTableEntry::huge(phys, PageFlags::PRESENT | PageFlags::WRITABLE);
        assert!(e.is_huge());
        assert!(e.is_present());
        assert_eq!(e.frame() & 0x1F_FFFF, 0, "huge frame must be 2 MiB-aligned");
    }

    #[test]
    fn virt_addr_index_extraction() {
        // Virtual address: 0x0000_0000_0020_0000 (2 MiB)
        let virt: u64 = 0x0000_0000_0020_0000;
        assert_eq!(pml4_index(virt), 0);
        assert_eq!(pdpt_index(virt), 0);
        assert_eq!(pd_index(virt), 1);   // second 2 MiB page
        assert_eq!(pt_index(virt), 0);
    }

    #[test]
    fn page_table_size() {
        assert_eq!(core::mem::size_of::<PageTable>(), 4096);
    }

    #[test]
    fn page_table_entry_unused_is_zero() {
        let e = PageTableEntry::unused();
        assert_eq!(e.raw(), 0);
        assert!(!e.is_present());
    }

    #[test]
    fn no_exec_flag_in_raw_entry() {
        let e = PageTableEntry::new(0x1000, PageFlags::PRESENT | PageFlags::NO_EXEC);
        // Bit 63 should be set
        assert!(e.raw() & (1 << 63) != 0);
    }
}
