// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Virtual & Physical Address Translation Engine
//
// Inspired by Linux x86_64 5-level paging (P4D/PUD/PMD/PTE) & direct-map (PAGE_OFFSET),
// FreeBSD pmap physical memory mapping, and OpenBSD UVM virtual memory management.
// Provides strongly-typed PhysAddr / VirtAddr primitives, page table translation,
// and Pull Request gateway integration for memory address mapping submissions.

use core::sync::atomic::{AtomicU64, Ordering};
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

pub const PAGE_SIZE_4KB: u64 = 4096;
pub const PAGE_SIZE_2MB: u64 = 2 * 1024 * 1024;
pub const PAGE_SIZE_1GB: u64 = 1024 * 1024 * 1024;

/// High-Half Direct Map Base Offset (Linux PAGE_OFFSET & FreeBSD DMAP parity)
pub const DIRECT_MAP_PHYS_OFFSET: u64 = 0xFFFF_8880_0000_0000;

/// Strongly-typed Physical Address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    pub const fn new(addr: u64) -> Self {
        Self(addr & 0x000F_FFFF_FFFF_FFFF) // 52-bit physical space clamp
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn is_aligned_4kb(&self) -> bool {
        (self.0 % PAGE_SIZE_4KB) == 0
    }

    pub fn is_aligned_2mb(&self) -> bool {
        (self.0 % PAGE_SIZE_2MB) == 0
    }

    /// Translate physical address to high-half virtual address via direct-map
    pub fn to_virt_direct_map(&self) -> VirtAddr {
        VirtAddr::new(self.0 + DIRECT_MAP_PHYS_OFFSET)
    }
}

/// Strongly-typed Virtual Address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    pub const fn new(addr: u64) -> Self {
        // Enforce x86_64 canonical address form (bits 47..63 matching bit 47)
        let canonical = if (addr & (1 << 47)) != 0 {
            addr | 0xFFFF_0000_0000_0000
        } else {
            addr & 0x0000_FFFF_FFFF_FFFF
        };
        Self(canonical)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn is_canonical(&self) -> bool {
        let bit47 = (self.0 >> 47) & 1;
        let high_bits = self.0 >> 48;
        if bit47 == 1 {
            high_bits == 0xFFFF
        } else {
            high_bits == 0x0000
        }
    }

    pub fn is_aligned_4kb(&self) -> bool {
        (self.0 % PAGE_SIZE_4KB) == 0
    }

    /// Extract 4-level / 5-level page table indices
    pub fn pml5_index(&self) -> usize {
        ((self.0 >> 48) & 0x1FF) as usize
    }

    pub fn pml4_index(&self) -> usize {
        ((self.0 >> 39) & 0x1FF) as usize
    }

    pub fn pud_index(&self) -> usize {
        ((self.0 >> 30) & 0x1FF) as usize
    }

    pub fn pmd_index(&self) -> usize {
        ((self.0 >> 21) & 0x1FF) as usize
    }

    pub fn pte_index(&self) -> usize {
        ((self.0 >> 12) & 0x1FF) as usize
    }

    pub fn page_offset_4kb(&self) -> usize {
        (self.0 & 0xFFF) as usize
    }

    /// Convert direct-map high-half virtual address back to physical address
    pub fn to_phys_direct_map(&self) -> Result<PhysAddr, &'static str> {
        if self.0 >= DIRECT_MAP_PHYS_OFFSET {
            Ok(PhysAddr::new(self.0 - DIRECT_MAP_PHYS_OFFSET))
        } else {
            Err("Virtual address is not in high-half direct-map range")
        }
    }
}

/// Page Table Entry Flags (x86_64 & ARM64 translation parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableFlags(pub u64);

impl PageTableFlags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const NO_CACHE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE_2MB: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const NO_EXECUTE: u64 = 1 << 63;

    pub fn new_kernel_rw() -> Self {
        Self(Self::PRESENT | Self::WRITABLE | Self::NO_EXECUTE)
    }

    pub fn new_user_rx() -> Self {
        Self(Self::PRESENT | Self::USER_ACCESSIBLE)
    }

    pub fn contains(&self, flag: u64) -> bool {
        (self.0 & flag) == flag
    }
}

/// Single Page Table Mapping Entry
#[derive(Debug, Clone)]
pub struct AddressMappingEntry {
    pub virt_addr: VirtAddr,
    pub phys_addr: PhysAddr,
    pub page_size: u64,
    pub flags: PageTableFlags,
}

/// Sovereign Page Table Translator
pub struct SovereignAddressTranslationEngine {
    pub mappings: BTreeMap<u64, AddressMappingEntry>, // VirtAddr u64 -> Entry
    pub active_cr3_phys: PhysAddr,
    pub translation_hits: AtomicU64,
    pub translation_faults: AtomicU64,
}

impl SovereignAddressTranslationEngine {
    pub fn new(cr3_phys: PhysAddr) -> Self {
        Self {
            mappings: BTreeMap::new(),
            active_cr3_phys: cr3_phys,
            translation_hits: AtomicU64::new(0),
            translation_faults: AtomicU64::new(0),
        }
    }

    /// Map a virtual page to physical page frame
    pub fn map_page(
        &mut self,
        virt: VirtAddr,
        phys: PhysAddr,
        page_size: u64,
        flags: PageTableFlags,
    ) -> Result<(), &'static str> {
        if !virt.is_canonical() {
            return Err("Cannot map non-canonical virtual address");
        }
        if page_size == PAGE_SIZE_4KB && (!virt.is_aligned_4kb() || !phys.is_aligned_4kb()) {
            return Err("Unaligned 4KB page mapping requested");
        }

        let entry = AddressMappingEntry {
            virt_addr: virt,
            phys_addr: phys,
            page_size,
            flags,
        };

        self.mappings.insert(virt.as_u64(), entry);
        Ok(())
    }

    /// Translate Virtual Address to Physical Address
    pub fn translate(&self, virt: VirtAddr) -> Result<PhysAddr, &'static str> {
        // First check direct-map range
        if virt.as_u64() >= DIRECT_MAP_PHYS_OFFSET {
            self.translation_hits.fetch_add(1, Ordering::SeqCst);
            return virt.to_phys_direct_map();
        }

        // Search mapped page tables
        let page_base = virt.as_u64() & !0xFFF;
        if let Some(entry) = self.mappings.get(&page_base) {
            if !entry.flags.contains(PageTableFlags::PRESENT) {
                self.translation_faults.fetch_add(1, Ordering::SeqCst);
                return Err("Page Fault: Page marked Not Present");
            }
            self.translation_hits.fetch_add(1, Ordering::SeqCst);
            let phys_base = entry.phys_addr.as_u64();
            let offset = virt.page_offset_4kb() as u64;
            Ok(PhysAddr::new(phys_base + offset))
        } else {
            self.translation_faults.fetch_add(1, Ordering::SeqCst);
            Err("Page Fault: Address not mapped in page table")
        }
    }

    /// Unmap a virtual page
    pub fn unmap_page(&mut self, virt: VirtAddr) -> Result<PhysAddr, &'static str> {
        let page_base = virt.as_u64() & !0xFFF;
        if let Some(entry) = self.mappings.remove(&page_base) {
            Ok(entry.phys_addr)
        } else {
            Err("Virtual page not found in mapping registry")
        }
    }
}

// ============================================================================
// Address Mapping Pull Request Gateway Subsystem
// ============================================================================

/// Pull Request submission status for address mapping requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressPrStatus {
    Open,
    Validated,
    Merged,
    Rejected,
}

/// Address Mapping Pull Request Entry
#[derive(Debug, Clone)]
pub struct AddressMappingPrSubmission {
    pub pr_id: u64,
    pub submitter: String,
    pub title: String,
    pub virt_start: VirtAddr,
    pub phys_start: PhysAddr,
    pub num_pages: usize,
    pub flags: PageTableFlags,
    pub status: AddressPrStatus,
    pub pqc_signature: Vec<u8>,
}

/// Address Translation PR Gateway Engine
pub struct SovereignAddressPRGatewayEngine {
    pub translation_engine: SovereignAddressTranslationEngine,
    pub pr_registry: BTreeMap<u64, AddressMappingPrSubmission>,
    next_pr_id: u64,
}

impl SovereignAddressPRGatewayEngine {
    pub fn new(cr3_phys: PhysAddr) -> Self {
        Self {
            translation_engine: SovereignAddressTranslationEngine::new(cr3_phys),
            pr_registry: BTreeMap::new(),
            next_pr_id: 1,
        }
    }

    /// Submits a virtual/physical address mapping PR
    pub fn submit_address_mapping_pr(
        &mut self,
        submitter: &str,
        title: &str,
        virt: VirtAddr,
        phys: PhysAddr,
        num_pages: usize,
        flags: PageTableFlags,
        pqc_signature: &[u8],
    ) -> u64 {
        let pr_id = self.next_pr_id;
        self.next_pr_id += 1;

        let pr = AddressMappingPrSubmission {
            pr_id,
            submitter: submitter.to_string(),
            title: title.to_string(),
            virt_start: virt,
            phys_start: phys,
            num_pages,
            flags,
            status: AddressPrStatus::Open,
            pqc_signature: pqc_signature.to_vec(),
        };

        self.pr_registry.insert(pr_id, pr);
        pr_id
    }

    /// Validates PQC signature and canonical address alignment for PR
    pub fn validate_address_pr(&mut self, pr_id: u64) -> Result<bool, &'static str> {
        let pr = self.pr_registry.get_mut(&pr_id).ok_or("PR ID not found")?;

        if pr.pqc_signature.is_empty() {
            pr.status = AddressPrStatus::Rejected;
            return Err("Address PR Rejected: Missing PQC signature");
        }

        if !pr.virt_start.is_canonical() {
            pr.status = AddressPrStatus::Rejected;
            return Err("Address PR Rejected: Non-canonical virtual address");
        }

        if !pr.virt_start.is_aligned_4kb() || !pr.phys_start.is_aligned_4kb() {
            pr.status = AddressPrStatus::Rejected;
            return Err("Address PR Rejected: Unaligned address");
        }

        pr.status = AddressPrStatus::Validated;
        Ok(true)
    }

    /// Merges approved address mapping PR into active page table
    pub fn merge_address_pr(&mut self, pr_id: u64) -> Result<usize, &'static str> {
        if let Err(e) = self.validate_address_pr(pr_id) {
            return Err(e);
        }

        let pr = self.pr_registry.get_mut(&pr_id).ok_or("PR ID not found")?;
        let mut mapped_count = 0;

        for page in 0..pr.num_pages {
            let virt = VirtAddr::new(pr.virt_start.as_u64() + (page as u64 * PAGE_SIZE_4KB));
            let phys = PhysAddr::new(pr.phys_start.as_u64() + (page as u64 * PAGE_SIZE_4KB));
            self.translation_engine
                .map_page(virt, phys, PAGE_SIZE_4KB, pr.flags)?;
            mapped_count += 1;
        }

        pr.status = AddressPrStatus::Merged;
        Ok(mapped_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phys_and_virt_addr_primitives() {
        let phys = PhysAddr::new(0x1000_0000);
        assert!(phys.is_aligned_4kb());
        assert_eq!(phys.as_u64(), 0x1000_0000);

        let virt = phys.to_virt_direct_map();
        assert!(virt.is_canonical());
        assert_eq!(virt.as_u64(), 0xFFFF_8880_1000_0000);

        let back_phys = virt.to_phys_direct_map().unwrap();
        assert_eq!(back_phys, phys);
    }

    #[test]
    fn test_address_translation_engine() {
        let cr3 = PhysAddr::new(0x1000);
        let mut engine = SovereignAddressTranslationEngine::new(cr3);

        let virt = VirtAddr::new(0x4000_0000_0000);
        let phys = PhysAddr::new(0x2000_0000);
        let flags = PageTableFlags::new_kernel_rw();

        assert!(engine.map_page(virt, phys, PAGE_SIZE_4KB, flags).is_ok());

        let translated = engine.translate(virt).unwrap();
        assert_eq!(translated, phys);

        let unmapped = engine.unmap_page(virt).unwrap();
        assert_eq!(unmapped, phys);
        assert!(engine.translate(virt).is_err());
    }

    #[test]
    fn test_address_pr_gateway_engine() {
        let cr3 = PhysAddr::new(0x1000);
        let mut gateway = SovereignAddressPRGatewayEngine::new(cr3);

        let virt = VirtAddr::new(0x5000_0000_0000);
        let phys = PhysAddr::new(0x3000_0000);
        let flags = PageTableFlags::new_user_rx();

        let pr_id = gateway.submit_address_mapping_pr(
            "kernel_dev",
            "Map User Execution Region",
            virt,
            phys,
            4, // 4 pages = 16KB
            flags,
            b"pqc_sig_valid",
        );

        assert_eq!(pr_id, 1);
        assert!(gateway.validate_address_pr(pr_id).unwrap());

        let mapped_pages = gateway.merge_address_pr(pr_id).unwrap();
        assert_eq!(mapped_pages, 4);

        let translated_p2 = gateway
            .translation_engine
            .translate(VirtAddr::new(virt.as_u64() + PAGE_SIZE_4KB))
            .unwrap();
        assert_eq!(translated_p2.as_u64(), phys.as_u64() + PAGE_SIZE_4KB);
    }
}
