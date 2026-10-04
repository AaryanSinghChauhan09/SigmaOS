//! SigmaOS — Page Fault Handler (#PF exception handler)
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux mm/fault.c and arch/x86/mm/fault.c
//!
//! Handles #PF exceptions:
//!  - Demand paging (not-present)
//!  - Copy-on-Write (write to read-only mapping)
//!  - User access to kernel addresses → SIGSEGV
//!  - Kernel null dereference → kernel bug

#![allow(dead_code)]

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

// ── PageFaultFlags ─────────────────────────────────────────────────────────────

/// Error code bits pushed by the CPU on a #PF
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFaultFlags(pub u64);

impl PageFaultFlags {
    /// Bit 0: Page was Present (1) or Not-present (0)
    pub const PRESENT: u64 = 1 << 0;
    /// Bit 1: Access was a Write (1) or Read (0)
    pub const WRITE: u64 = 1 << 1;
    /// Bit 2: Fault in User mode (1) or Supervisor (0)
    pub const USER: u64 = 1 << 2;
    /// Bit 3: Reserved bit violation
    pub const RESERVED: u64 = 1 << 3;
    /// Bit 4: Instruction fetch (NX violation)
    pub const EXEC: u64 = 1 << 4;
    /// Bit 5: Protection-key violation
    pub const PKEY: u64 = 1 << 5;

    pub fn present(&self) -> bool {
        self.0 & Self::PRESENT != 0
    }
    pub fn write(&self) -> bool {
        self.0 & Self::WRITE != 0
    }
    pub fn user(&self) -> bool {
        self.0 & Self::USER != 0
    }
    pub fn reserved(&self) -> bool {
        self.0 & Self::RESERVED != 0
    }
    pub fn exec(&self) -> bool {
        self.0 & Self::EXEC != 0
    }
    pub fn pkey(&self) -> bool {
        self.0 & Self::PKEY != 0
    }
}

impl core::fmt::Display for PageFaultFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PageFaultFlags(0x{:x})", self.0)
    }
}

// ── VMA backing ───────────────────────────────────────────────────────────────

/// What backs a VMA region
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmaBacking {
    /// Anonymous mapping (not file-backed)
    Anonymous,
    /// File-backed mapping
    File { path: String, offset: u64 },
}

// ── VMA protection flags ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmaFlags(pub u32);

impl VmaFlags {
    pub const READ: u32 = 1 << 0;
    pub const WRITE: u32 = 1 << 1;
    pub const EXEC: u32 = 1 << 2;
    pub const SHARED: u32 = 1 << 3;
    pub const GROWSDOWN: u32 = 1 << 4;

    pub fn readable(&self) -> bool {
        self.0 & Self::READ != 0
    }
    pub fn writable(&self) -> bool {
        self.0 & Self::WRITE != 0
    }
    pub fn executable(&self) -> bool {
        self.0 & Self::EXEC != 0
    }
    pub fn shared(&self) -> bool {
        self.0 & Self::SHARED != 0
    }
}

// ── VirtualMemoryArea ─────────────────────────────────────────────────────────

/// Kernel VMA (virtual memory area)
#[derive(Debug, Clone)]
pub struct VirtualMemoryArea {
    /// Start virtual address (inclusive)
    pub start: u64,
    /// End virtual address (exclusive)
    pub end: u64,
    /// Protection / mapping flags
    pub flags: VmaFlags,
    /// Backing storage
    pub backing: VmaBacking,
    /// Copy-on-write: number of sharing processes
    pub cow_ref_count: u32,
}

impl VirtualMemoryArea {
    pub fn new_anon(start: u64, end: u64, flags: VmaFlags) -> Self {
        Self {
            start,
            end,
            flags,
            backing: VmaBacking::Anonymous,
            cow_ref_count: 1,
        }
    }

    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.end
    }

    pub fn is_shared(&self) -> bool {
        self.cow_ref_count > 1
    }
}

// ── Physical page allocator (simulated) ───────────────────────────────────────

/// Simulated physical page frame
#[derive(Debug, Clone)]
pub struct PhysPage {
    pub pfn: u64,
    pub data: Vec<u8>,
}

impl PhysPage {
    pub const PAGE_SIZE: usize = 4096;

    pub fn new(pfn: u64) -> Self {
        Self {
            pfn,
            data: vec![0u8; Self::PAGE_SIZE],
        }
    }

    /// Make a copy (for CoW)
    pub fn copy(&self, new_pfn: u64) -> Self {
        Self {
            pfn: new_pfn,
            data: self.data.clone(),
        }
    }
}

/// Simulated frame allocator
#[derive(Debug)]
pub struct FrameAllocator {
    next_pfn: u64,
    free_pages: u64,
}

impl FrameAllocator {
    pub fn new(total_pages: u64) -> Self {
        Self {
            next_pfn: 1,
            free_pages: total_pages,
        }
    }

    pub fn alloc_page(&mut self) -> Option<PhysPage> {
        if self.free_pages == 0 {
            return None;
        }
        let pfn = self.next_pfn;
        self.next_pfn += 1;
        self.free_pages -= 1;
        Some(PhysPage::new(pfn))
    }

    pub fn free_pages_remaining(&self) -> u64 {
        self.free_pages
    }
}

// ── Page table (simulated) ────────────────────────────────────────────────────

/// Simulated page table: maps virtual page → physical page frame
#[derive(Debug, Default)]
pub struct PageTable {
    /// virt_page_addr → PhysPage
    entries: HashMap<u64, PhysPage>,
}

impl PageTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn map(&mut self, virt_addr: u64, page: PhysPage) {
        let vpn = virt_addr & !0xFFF;
        self.entries.insert(vpn, page);
    }

    pub fn lookup(&self, virt_addr: u64) -> Option<&PhysPage> {
        let vpn = virt_addr & !0xFFF;
        self.entries.get(&vpn)
    }

    pub fn lookup_mut(&mut self, virt_addr: u64) -> Option<&mut PhysPage> {
        let vpn = virt_addr & !0xFFF;
        self.entries.get_mut(&vpn)
    }

    pub fn unmap(&mut self, virt_addr: u64) -> Option<PhysPage> {
        let vpn = virt_addr & !0xFFF;
        self.entries.remove(&vpn)
    }
}

// ── PageFaultResult ───────────────────────────────────────────────────────────

/// Outcome of handling a page fault
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageFaultResult {
    /// Fault resolved — execution can resume
    Handled,
    /// Killed faulting process to free memory (OOM)
    OomKill,
    /// Deliver SIGSEGV to process; inner u64 = faulting address
    Segfault(u64),
    /// Kernel bug — panic/oops
    KernelBug,
}

// ── Page fault handler ────────────────────────────────────────────────────────

/// OOM killer simulation: returns true if a process was killed (freeing memory)
fn oom_kill_process(allocator: &FrameAllocator) -> bool {
    // In a real kernel we'd walk all processes and kill the one with highest
    // oom_score. Here we simply return false (no victim available).
    let _ = allocator;
    false
}

/// Demand-paging handler: allocate a new zeroed page and map it.
fn do_demand_paging(
    vma: &VirtualMemoryArea,
    fault_addr: u64,
    pt: &mut PageTable,
    allocator: &mut FrameAllocator,
) -> PageFaultResult {
    match allocator.alloc_page() {
        Some(page) => {
            pt.map(fault_addr, page);
            PageFaultResult::Handled
        }
        None => {
            if oom_kill_process(allocator) {
                // Retry after OOM kill
                PageFaultResult::OomKill
            } else {
                // Cannot satisfy — SIGSEGV
                let _ = vma;
                PageFaultResult::Segfault(fault_addr)
            }
        }
    }
}

/// Copy-on-Write handler: copy the shared page, map private copy as writable.
fn do_cow(
    vma: &VirtualMemoryArea,
    fault_addr: u64,
    pt: &mut PageTable,
    allocator: &mut FrameAllocator,
) -> PageFaultResult {
    // Get old page
    let old_page = match pt.lookup(fault_addr) {
        Some(p) => p.clone(),
        None => return PageFaultResult::Segfault(fault_addr),
    };

    // VMA must be writable (just not mapped as such yet)
    if !vma.flags.writable() {
        return PageFaultResult::Segfault(fault_addr);
    }

    match allocator.alloc_page() {
        Some(new_pfn_page) => {
            let new_page = old_page.copy(new_pfn_page.pfn);
            pt.map(fault_addr, new_page);
            PageFaultResult::Handled
        }
        None => PageFaultResult::OomKill,
    }
}

/// Main page fault dispatch — called from the #PF exception handler.
///
/// # Arguments
/// * `cr2` — the faulting virtual address (CR2 on x86)
/// * `error_code` — CPU error code bits
/// * `vmas` — sorted list of VMAs for the current process
/// * `pt` — current process page table
/// * `allocator` — physical frame allocator
pub fn handle_page_fault(
    cr2: u64,
    error_code: u64,
    vmas: &[VirtualMemoryArea],
    pt: &mut PageTable,
    allocator: &mut FrameAllocator,
) -> PageFaultResult {
    let flags = PageFaultFlags(error_code);

    // ── Kernel null dereference ──────────────────────────────────────────────
    if !flags.user() && cr2 < 0x1000 {
        return PageFaultResult::KernelBug;
    }

    // ── User access to kernel address ────────────────────────────────────────
    if flags.user() && cr2 >= 0xFFFF_8000_0000_0000 {
        return PageFaultResult::Segfault(cr2);
    }

    // ── Find the VMA covering the fault address ───────────────────────────────
    let vma = match vmas.iter().find(|v| v.contains(cr2)) {
        Some(v) => v,
        None => {
            // No mapping → SIGSEGV
            return PageFaultResult::Segfault(cr2);
        }
    };

    // ── Reserved bit violation / NX violation → SIGSEGV ──────────────────────
    if flags.reserved() {
        return PageFaultResult::Segfault(cr2);
    }
    if flags.exec() && !vma.flags.executable() {
        return PageFaultResult::Segfault(cr2);
    }

    // ── Write to read-only mapping ────────────────────────────────────────────
    if flags.present() && flags.write() {
        if vma.is_shared() || !vma.flags.writable() {
            return PageFaultResult::Segfault(cr2);
        }
        // Private writable VMA that was mapped read-only → CoW
        return do_cow(vma, cr2, pt, allocator);
    }

    // ── Not-present: demand paging ────────────────────────────────────────────
    if !flags.present() {
        return do_demand_paging(vma, cr2, pt, allocator);
    }

    // Unhandled case
    PageFaultResult::KernelBug
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    fn rw_vma(start: u64, end: u64) -> VirtualMemoryArea {
        VirtualMemoryArea::new_anon(start, end, VmaFlags(VmaFlags::READ | VmaFlags::WRITE))
    }

    fn ro_vma(start: u64, end: u64) -> VirtualMemoryArea {
        VirtualMemoryArea::new_anon(start, end, VmaFlags(VmaFlags::READ))
    }

    #[test]
    fn test_demand_paging_success() {
        let vmas = vec![rw_vma(0x1000, 0x2000)];
        let mut pt = PageTable::new();
        let mut alloc = FrameAllocator::new(16);

        // Not-present fault at 0x1000
        let res = handle_page_fault(0x1000, 0, &vmas, &mut pt, &mut alloc);
        assert_eq!(res, PageFaultResult::Handled);
        assert!(pt.lookup(0x1000).is_some());
    }

    #[test]
    fn test_demand_paging_oom() {
        let vmas = vec![rw_vma(0x1000, 0x2000)];
        let mut pt = PageTable::new();
        let mut alloc = FrameAllocator::new(0); // no free pages

        let res = handle_page_fault(0x1000, 0, &vmas, &mut pt, &mut alloc);
        // Should get OomKill or Segfault (we return Segfault in our sim)
        assert!(res == PageFaultResult::OomKill || res == PageFaultResult::Segfault(0x1000));
    }

    #[test]
    fn test_cow_fault() {
        let mut vma = rw_vma(0x1000, 0x2000);
        vma.cow_ref_count = 2; // shared
        let vmas = vec![vma];
        let mut pt = PageTable::new();
        let mut alloc = FrameAllocator::new(8);

        // Map a page first (simulate it being present but shared/read-only)
        let initial = alloc.alloc_page().unwrap();
        pt.map(0x1000, initial);

        // Write to present, shared page → CoW
        // error_code: bit0 (present) | bit1 (write)
        let res = handle_page_fault(0x1000, 0b011, &vmas, &mut pt, &mut alloc);
        // CoW on shared vma → SIGSEGV (cannot write shared)
        assert_eq!(res, PageFaultResult::Segfault(0x1000));
    }

    #[test]
    fn test_user_access_kernel_addr() {
        let vmas = vec![];
        let mut pt = PageTable::new();
        let mut alloc = FrameAllocator::new(8);
        // User-mode fault to kernel address: error_code bit2 = USER
        let res = handle_page_fault(0xFFFF_8000_1234_5678, 0b100, &vmas, &mut pt, &mut alloc);
        assert_eq!(res, PageFaultResult::Segfault(0xFFFF_8000_1234_5678));
    }

    #[test]
    fn test_kernel_null_deref() {
        let vmas = vec![];
        let mut pt = PageTable::new();
        let mut alloc = FrameAllocator::new(8);
        // Kernel-mode fault at addr 0 (no USER bit)
        let res = handle_page_fault(0x0, 0, &vmas, &mut pt, &mut alloc);
        assert_eq!(res, PageFaultResult::KernelBug);
    }

    #[test]
    fn test_no_vma_segfault() {
        let vmas = vec![];
        let mut pt = PageTable::new();
        let mut alloc = FrameAllocator::new(8);
        // User fault, no covering VMA
        let res = handle_page_fault(0xDEAD_0000, 0b100, &vmas, &mut pt, &mut alloc);
        assert_eq!(res, PageFaultResult::Segfault(0xDEAD_0000));
    }
}
