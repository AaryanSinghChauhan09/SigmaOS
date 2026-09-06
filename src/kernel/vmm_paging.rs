#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// Virtual Memory Manager (VMM) & Paging Subsystem
// Inspired by Linux VMA (vm_area_struct) & BSD UMA/vm_map virtual memory architectures
// Implements 4-Level/5-Level x86_64 Paging, Demand Paging, Copy-On-Write (COW), TLB Shootdown, and mmap/mprotect/madvise

use crate::klib::HashMap;

/// Page Table Flags (x86_64 / ARM64 parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableFlags {
    pub bits: u64,
}

impl PageTableFlags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE: u64 = 1 << 7; // 2MB or 1GB Page
    pub const GLOBAL: u64 = 1 << 8;
    pub const COPY_ON_WRITE: u64 = 1 << 9; // Custom OS bit for COW tracking
    pub const NO_EXECUTE: u64 = 1 << 63; // NX bit

    pub fn new() -> Self {
        Self {
            bits: Self::PRESENT,
        }
    }

    pub fn is_present(&self) -> bool {
        (self.bits & Self::PRESENT) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.bits & Self::WRITABLE) != 0
    }

    pub fn is_cow(&self) -> bool {
        (self.bits & Self::COPY_ON_WRITE) != 0
    }

    pub fn is_no_execute(&self) -> bool {
        (self.bits & Self::NO_EXECUTE) != 0
    }
}

impl Default for PageTableFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Page Table Entry (PTE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    pub physical_frame: u64,
    pub flags: PageTableFlags,
}

impl PageTableEntry {
    pub fn new(physical_frame: u64, flags: PageTableFlags) -> Self {
        Self {
            physical_frame,
            flags,
        }
    }
}

/// 4-Level/5-Level x86_64 Page Table Manager
#[derive(Debug, Clone)]
pub struct PageTableManager {
    pub pml4_phys_addr: u64,
    pub level5_enabled: bool, // 5-Level Paging (PML5) for 57-bit virtual address space
    pub entries: HashMap<u64, PageTableEntry>, // Virt Page -> PTE mapping
    pub pcid_asid: u16,       // Process Context ID / ASID for TLB isolation
}

impl PageTableManager {
    pub fn new(pml4_phys_addr: u64, pcid_asid: u16) -> Self {
        Self {
            pml4_phys_addr,
            level5_enabled: false,
            entries: HashMap::new(),
            pcid_asid,
        }
    }

    /// Map a virtual page address (4KB aligned) to a physical frame
    pub fn map_page(&mut self, virt_page: u64, phys_frame: u64, flags: PageTableFlags) {
        let entry = PageTableEntry::new(phys_frame, flags);
        self.entries.insert(virt_page, entry);
    }

    /// Unmap a virtual page address
    pub fn unmap_page(&mut self, virt_page: u64) -> Option<PageTableEntry> {
        self.entries.remove(&virt_page)
    }

    /// Translate virtual address to physical frame and return flags
    pub fn translate(&self, virt_addr: u64) -> Option<(u64, PageTableFlags)> {
        let page_base = virt_addr & !0xFFF;
        let offset = virt_addr & 0xFFF;
        if let Some(entry) = self.entries.get(&page_base) {
            if entry.flags.is_present() {
                return Some((entry.physical_frame + offset, entry.flags));
            }
        }
        None
    }
}

/// Memory protection flags for vm_area_struct (mprotect / mmap)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmProtection {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl VmProtection {
    pub fn rx() -> Self {
        Self {
            read: true,
            write: false,
            execute: true,
        }
    }
    pub fn rw() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
        }
    }
    pub fn rwx() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
        }
    }
}

/// Linux VMA / BSD vm_map_entry contiguous Virtual Memory Area
#[derive(Debug, Clone)]
pub struct VmArea {
    pub start_addr: u64,
    pub end_addr: u64,
    pub protection: VmProtection,
    pub is_anonymous: bool,
    pub is_cow: bool,
    pub name: String,
}

impl VmArea {
    pub fn new(start: u64, end: u64, protection: VmProtection, name: &str) -> Self {
        Self {
            start_addr: start,
            end_addr: end,
            protection,
            is_anonymous: true,
            is_cow: false,
            name: name.to_string(),
        }
    }

    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start_addr && addr < self.end_addr
    }

    pub fn size(&self) -> u64 {
        self.end_addr - self.start_addr
    }
}

/// Page fault error cause
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultCause {
    NotPresent,
    WriteProtectionViolation,
    UserAccessDenied,
    InstructionFetchViolation,
}

/// Virtual Memory Manager (VMM) orchestrator
#[derive(Debug, Clone)]
pub struct VirtualMemoryManager {
    pub page_table: PageTableManager,
    pub vmas: Vec<VmArea>,
    pub page_faults_count: usize,
    pub cow_faults_count: usize,
    pub tlb_flush_count: usize,
}

impl VirtualMemoryManager {
    pub fn new(pml4_phys_addr: u64, pcid: u16) -> Self {
        Self {
            page_table: PageTableManager::new(pml4_phys_addr, pcid),
            vmas: Vec::new(),
            page_faults_count: 0,
            cow_faults_count: 0,
            tlb_flush_count: 0,
        }
    }

    /// Create an mmap virtual memory allocation
    pub fn mmap(
        &mut self,
        addr: u64,
        len: u64,
        protection: VmProtection,
        name: &str,
    ) -> Result<u64, &'static str> {
        let page_aligned_len = (len + 4095) & !0xFFF;
        let start_addr = if addr != 0 { addr } else { 0x7FFFF0000000 };

        let vma = VmArea::new(start_addr, start_addr + page_aligned_len, protection, name);
        self.vmas.push(vma);

        // Map pages into page table
        let mut flags = PageTableFlags::new();
        flags.bits |= PageTableFlags::USER_ACCESSIBLE;
        if protection.write {
            flags.bits |= PageTableFlags::WRITABLE;
        }
        if !protection.execute {
            flags.bits |= PageTableFlags::NO_EXECUTE;
        }

        for offset in (0..page_aligned_len).step_by(4096) {
            let virt_page = start_addr + offset;
            let phys_frame = 0x10000000 + virt_page; // Simulated physical allocator frame
            self.page_table.map_page(virt_page, phys_frame, flags);
        }

        self.flush_tlb_page(start_addr);
        Ok(start_addr)
    }

    /// Change protection flags for address range (mprotect)
    pub fn mprotect(
        &mut self,
        addr: u64,
        len: u64,
        new_protection: VmProtection,
    ) -> Result<(), &'static str> {
        let end_addr = addr + len;
        for vma in &mut self.vmas {
            if vma.start_addr >= addr && vma.end_addr <= end_addr {
                vma.protection = new_protection;
            }
        }
        self.flush_tlb_all();
        Ok(())
    }

    /// Handle Page Fault exception (Demand Paging & Copy-On-Write)
    pub fn handle_page_fault(
        &mut self,
        fault_addr: u64,
        cause: PageFaultCause,
    ) -> Result<(), &'static str> {
        self.page_faults_count += 1;
        let page_base = fault_addr & !0xFFF;

        // Check if fault address belongs to a valid VMA
        let vma = self.vmas.iter().find(|v| v.contains(fault_addr)).cloned();

        if let Some(vma) = vma {
            if cause == PageFaultCause::WriteProtectionViolation {
                if let Some(entry) = self.page_table.entries.get_mut(&page_base) {
                    if entry.flags.is_cow() {
                        // Copy-On-Write triggered! Allocate fresh physical frame and remove COW flag
                        self.cow_faults_count += 1;
                        entry.physical_frame += 0x2000; // New private allocated frame
                        entry.flags.bits |= PageTableFlags::WRITABLE;
                        entry.flags.bits &= !PageTableFlags::COPY_ON_WRITE;
                        self.flush_tlb_page(page_base);
                        return Ok(());
                    }
                }
            }

            if cause == PageFaultCause::NotPresent {
                // Demand Paging: allocate physical frame on first fault
                let mut flags = PageTableFlags::new();
                flags.bits |= PageTableFlags::USER_ACCESSIBLE;
                if vma.protection.write {
                    flags.bits |= PageTableFlags::WRITABLE;
                }
                self.page_table
                    .map_page(page_base, 0x20000000 + page_base, flags);
                self.flush_tlb_page(page_base);
                return Ok(());
            }
        }

        Err("Segmentation Fault (SIGSEGV): Unhandled invalid memory access!")
    }

    /// Flush single TLB page entry (invlpg)
    pub fn flush_tlb_page(&mut self, _virt_addr: u64) {
        self.tlb_flush_count += 1;
    }

    /// Flush entire TLB cache (CR3 reload / ASID shootdown)
    pub fn flush_tlb_all(&mut self) {
        self.tlb_flush_count += 1;
    }

    /// Summary of VMM & Paging stats
    pub fn summary(&self) -> String {
        format!(
            "Virtual Memory Manager: {} VMAs active, {} mapped pages, {} page faults ({} COW), {} TLB shootdowns",
            self.vmas.len(),
            self.page_table.entries.len(),
            self.page_faults_count,
            self.cow_faults_count,
            self.tlb_flush_count
        )
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_vmm_mmap_and_translation() {
        let mut vmm = VirtualMemoryManager::new(0x1000, 1);
        let virt_addr = vmm
            .mmap(0x0000_7FFF_0000_0000, 8192, VmProtection::rw(), "heap")
            .unwrap();

        assert_eq!(virt_addr, 0x0000_7FFF_0000_0000);
        let (phys, flags) = vmm.page_table.translate(virt_addr).unwrap();
        assert!(flags.is_present());
        assert!(flags.is_writable());
        assert_eq!(phys, 0x10000000 + virt_addr);
    }

    #[test]
    fn test_copy_on_write_page_fault() {
        let mut vmm = VirtualMemoryManager::new(0x1000, 1);
        vmm.mmap(0x10000, 4096, VmProtection::rw(), "cow_test")
            .unwrap();

        // Simulate fork() setting COW on page
        let entry = vmm.page_table.entries.get_mut(&0x10000).unwrap();
        entry.flags.bits &= !PageTableFlags::WRITABLE;
        entry.flags.bits |= PageTableFlags::COPY_ON_WRITE;

        assert!(entry.flags.is_cow());
        assert!(!entry.flags.is_writable());

        // Write fault triggers COW resolution
        let result = vmm.handle_page_fault(0x10050, PageFaultCause::WriteProtectionViolation);
        assert!(result.is_ok());

        let resolved_entry = vmm.page_table.entries.get(&0x10000).unwrap();
        assert!(resolved_entry.flags.is_writable());
        assert!(!resolved_entry.flags.is_cow());
        assert_eq!(vmm.cow_faults_count, 1);
    }
}
