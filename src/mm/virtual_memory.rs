extern crate alloc;
// OOP-based Virtual Memory Manager with Canonical Address Verification for SigmaOS
// Implements virtual memory management using OOP principles with traits and structs
// No dependency on external memory management libraries
// Inspired by x86_64, ARM64, Linux, BSD, and Windows canonical memory layouts
// Enhanced with OpenBSD/FreeBSD W^X (Write XOR Execute) security, FreeBSD wired/pinned page protection,
// and Linux kswapd-inspired active/inactive LRU page reclaimer scanning.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Page size (4KB)
const PAGE_SIZE: usize = 4096;

/// Virtual address
pub type VirtualAddress = usize;

/// Physical address
pub type PhysicalAddress = usize;

// =========================================================================
// x86_64 & ARM64 Inspired Canonical Address Verification Subsystem
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalHalf {
    Lower, // User space (bits 47-63 are 0)
    Upper, // Kernel space (bits 47-63 are 1)
}

/// Verification: under a 48-bit address space, bits 47 to 63 must be copies of bit 47
pub fn is_canonical_address(addr: VirtualAddress) -> bool {
    let sign_bit = (addr >> 47) & 1;
    let upper_bits = addr >> 47;
    if sign_bit == 0 {
        upper_bits == 0
    } else {
        upper_bits == 0x1FFFF
    }
}

/// Retrieve the canonical half of a virtual address (Lower vs Upper space)
pub fn get_canonical_half(addr: VirtualAddress) -> Option<CanonicalHalf> {
    if !is_canonical_address(addr) {
        return None;
    }
    let sign_bit = (addr >> 47) & 1;
    if sign_bit == 0 {
        Some(CanonicalHalf::Lower)
    } else {
        Some(CanonicalHalf::Upper)
    }
}

/// Sign-extend bit 47 to convert any address into its canonical representation
pub fn canonicalize_address(addr: VirtualAddress) -> VirtualAddress {
    let sign_bit = (addr >> 47) & 1;
    if sign_bit == 1 {
        addr | 0xFFFF_8000_0000_0000
    } else {
        addr & 0x0000_7FFF_FFFF_FFFF
    }
}

/// Page table entry flags
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntryFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub write_through: bool,
    pub cache_disabled: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub global: bool,
    /// FreeBSD-inspired wired/pinned lock bit (prevents eviction or unmapping)
    pub is_wired: bool,
}

impl PageTableEntryFlags {
    pub fn new() -> Self {
        PageTableEntryFlags {
            present: false,
            writable: false,
            user_accessible: false,
            write_through: false,
            cache_disabled: false,
            accessed: false,
            dirty: false,
            global: false,
            is_wired: false,
        }
    }

    pub fn to_u64(&self) -> u64 {
        let mut flags = 0u64;
        if self.present { flags |= 1 << 0; }
        if self.writable { flags |= 1 << 1; }
        if self.user_accessible { flags |= 1 << 2; }
        if self.write_through { flags |= 1 << 3; }
        if self.cache_disabled { flags |= 1 << 4; }
        if self.accessed { flags |= 1 << 5; }
        if self.dirty { flags |= 1 << 6; }
        if self.global { flags |= 1 << 8; }
        if self.is_wired { flags |= 1 << 9; }
        flags
    }
}

impl Default for PageTableEntryFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Page table entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    pub physical_address: PhysicalAddress,
    pub flags: PageTableEntryFlags,
}

impl PageTableEntry {
    pub fn new(physical_address: PhysicalAddress, flags: PageTableEntryFlags) -> Self {
        PageTableEntry {
            physical_address,
            flags,
        }
    }

    pub fn to_u64(&self) -> u64 {
        (self.physical_address as u64) | self.flags.to_u64()
    }
}

/// Page table (OOP: Page table object)
#[repr(C)]
pub struct PageTable {
    pub base_virtual_address: VirtualAddress,
    pub entries: [Option<PageTableEntry>; 512],
    pub capability: PageTableCapability,
}

/// Page table capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableCapability {
    pub can_map: bool,
    pub can_unmap: bool,
    pub can_protect: bool,
}

impl PageTableCapability {
    pub fn new() -> Self {
        PageTableCapability {
            can_map: false,
            can_unmap: false,
            can_protect: false,
        }
    }

    pub fn full() -> Self {
        PageTableCapability {
            can_map: true,
            can_unmap: true,
            can_protect: true,
        }
    }
}

impl PageTable {
    pub fn new(base_virtual_address: VirtualAddress, capability: PageTableCapability) -> Self {
        PageTable {
            base_virtual_address,
            entries: [None; 512],
            capability,
        }
    }

    pub unsafe fn map_page(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, flags: PageTableEntryFlags) -> Result<(), MemoryError> {
        if !self.capability.can_map {
            return Err(MemoryError::PermissionDenied);
        }

        let index = ((virtual_addr - self.base_virtual_address) / PAGE_SIZE) % 512;
        let entry = PageTableEntry::new(physical_addr, flags);
        self.entries[index] = Some(entry);
        Ok(())
    }

    pub unsafe fn unmap_page(&mut self, virtual_addr: VirtualAddress) -> Result<(), MemoryError> {
        if !self.capability.can_unmap {
            return Err(MemoryError::PermissionDenied);
        }

        let index = ((virtual_addr - self.base_virtual_address) / PAGE_SIZE) % 512;
        if let Some(ref entry) = self.entries[index] {
            // FreeBSD-style wired/pinned page protection check
            if entry.flags.is_wired {
                return Err(MemoryError::WiredPageLocked);
            }
        }
        self.entries[index] = None;
        Ok(())
    }

    pub unsafe fn protect_page(&mut self, virtual_addr: VirtualAddress, flags: PageTableEntryFlags) -> Result<(), MemoryError> {
        if !self.capability.can_protect {
            return Err(MemoryError::PermissionDenied);
        }

        let index = ((virtual_addr - self.base_virtual_address) / PAGE_SIZE) % 512;
        if let Some(ref mut entry) = self.entries[index] {
            if entry.flags.is_wired && !flags.is_wired {
                return Err(MemoryError::PermissionDenied);
            }
            entry.flags = flags;
            Ok(())
        } else {
            Err(MemoryError::NotMapped)
        }
    }

    pub fn get_entry(&self, virtual_addr: VirtualAddress) -> Option<&PageTableEntry> {
        if virtual_addr < self.base_virtual_address {
            return None;
        }
        let index = ((virtual_addr - self.base_virtual_address) / PAGE_SIZE) % 512;
        self.entries[index].as_ref()
    }
}

/// Linux-style Page Translation Engine (Walking page tables / directory)
pub struct LinearPageTranslator;

impl LinearPageTranslator {
    /// Performs virtual-to-physical address translation via page table lookup
    pub unsafe fn translate_address(
        page_tables: &[Option<NonNull<PageTable>>],
        vaddr: VirtualAddress,
    ) -> Result<PhysicalAddress, MemoryError> {
        if !is_canonical_address(vaddr) {
            return Err(MemoryError::InvalidAddress);
        }

        let offset = vaddr % PAGE_SIZE;

        for slot in page_tables {
            if let Some(pt_ptr) = slot {
                let pt = &*pt_ptr.as_ptr();
                if let Some(entry) = pt.get_entry(vaddr) {
                    if !entry.flags.present {
                        return Err(MemoryError::NotMapped);
                    }
                    return Ok(entry.physical_address + offset);
                }
            }
        }

        Err(MemoryError::NotMapped)
    }
}

// =========================================================================
// Virtual Memory Address Relationship Subsystem (1-to-1, 1-to-N, N-to-N)
// =========================================================================

/// Memory mapping relationship type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageMappingRelationship {
    /// 1-to-1: Single unique virtual page mapped directly to single physical frame
    OneToOne,
    /// 1-to-N: Single physical frame shared/mapped across N distinct virtual pages
    OneToMany,
    /// N-to-N: N virtual pages aliased to N physical frames (e.g., Copy-on-Write)
    ManyToMany,
}

/// Page relationship tracking entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRelationshipEntry {
    pub paddr: PhysicalAddress,
    pub mapped_vaddrs: Vec<VirtualAddress>,
    pub relationship: PageMappingRelationship,
    pub is_cow: bool,
}

impl PageRelationshipEntry {
    pub fn new(paddr: PhysicalAddress, vaddr: VirtualAddress) -> Self {
        let mut vaddrs = Vec::new();
        vaddrs.push(vaddr);
        Self {
            paddr,
            mapped_vaddrs: vaddrs,
            relationship: PageMappingRelationship::OneToOne,
            is_cow: false,
        }
    }

    pub fn add_mapping(&mut self, vaddr: VirtualAddress, is_cow: bool) {
        if !self.mapped_vaddrs.contains(&vaddr) {
            self.mapped_vaddrs.push(vaddr);
        }
        self.is_cow = is_cow;
        if self.is_cow {
            self.relationship = PageMappingRelationship::ManyToMany;
        } else if self.mapped_vaddrs.len() > 1 {
            self.relationship = PageMappingRelationship::OneToMany;
        } else {
            self.relationship = PageMappingRelationship::OneToOne;
        }
    }
}

pub struct PageRelationshipTracker {
    pub mappings: Vec<PageRelationshipEntry>,
}

impl PageRelationshipTracker {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    pub fn register_page_mapping(
        &mut self,
        paddr: PhysicalAddress,
        vaddr: VirtualAddress,
        is_cow: bool,
    ) {
        let mut found = false;
        for entry in &mut self.mappings {
            if entry.paddr == paddr {
                entry.add_mapping(vaddr, is_cow);
                found = true;
                break;
            }
        }

        if !found {
            let mut entry = PageRelationshipEntry::new(paddr, vaddr);
            if is_cow {
                entry.is_cow = true;
                entry.relationship = PageMappingRelationship::ManyToMany;
            }
            self.mappings.push(entry);
        }
    }

    pub fn get_relationship(&self, paddr: PhysicalAddress) -> Option<PageMappingRelationship> {
        for entry in &self.mappings {
            if entry.paddr == paddr {
                return Some(entry.relationship);
            }
        }
        None
    }
}

impl Default for PageRelationshipTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory region (OOP: Memory region object)
#[repr(C)]
pub struct MemoryRegion {
    pub start: VirtualAddress,
    pub end: VirtualAddress,
    pub permissions: MemoryPermissions,
    pub capability: MemoryRegionCapability,
}

/// Memory permissions
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    /// FreeBSD-inspired wired/pinned lock
    pub is_wired: bool,
}

impl MemoryPermissions {
    pub fn new() -> Self {
        MemoryPermissions {
            read: false,
            write: false,
            execute: false,
            is_wired: false,
        }
    }

    pub fn read_only() -> Self {
        let mut perms = MemoryPermissions::new();
        perms.read = true;
        perms
    }

    pub fn read_write() -> Self {
        let mut perms = MemoryPermissions::new();
        perms.read = true;
        perms.write = true;
        perms
    }

    pub fn read_execute() -> Self {
        let mut perms = MemoryPermissions::new();
        perms.read = true;
        perms.execute = true;
        perms
    }

    /// OpenBSD/FreeBSD W^X Security Audit rule: Memory cannot be Writeable and Executable simultaneously
    pub fn is_wx_compliant(&self) -> bool {
        !(self.write && self.execute)
    }
}

impl Default for MemoryPermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory region capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegionCapability {
    pub can_resize: bool,
    pub can_change_permissions: bool,
}

impl MemoryRegionCapability {
    pub fn new() -> Self {
        MemoryRegionCapability {
            can_resize: false,
            can_change_permissions: false,
        }
    }

    pub fn full() -> Self {
        MemoryRegionCapability {
            can_resize: true,
            can_change_permissions: true,
        }
    }
}

impl MemoryRegion {
    pub fn new(start: VirtualAddress, end: VirtualAddress, permissions: MemoryPermissions, capability: MemoryRegionCapability) -> Self {
        MemoryRegion {
            start,
            end,
            permissions,
            capability,
        }
    }

    pub fn contains(&self, addr: VirtualAddress) -> bool {
        addr >= self.start && addr < self.end
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }

    pub unsafe fn resize(&mut self, new_end: VirtualAddress) -> Result<(), MemoryError> {
        if !self.capability.can_resize {
            return Err(MemoryError::PermissionDenied);
        }
        self.end = new_end;
        Ok(())
    }

    pub unsafe fn change_permissions(&mut self, permissions: MemoryPermissions) -> Result<(), MemoryError> {
        if !self.capability.can_change_permissions {
            return Err(MemoryError::PermissionDenied);
        }
        // Enforce W^X security rule
        if !permissions.is_wx_compliant() {
            return Err(MemoryError::WxViolation);
        }
        self.permissions = permissions;
        Ok(())
    }
}

/// Memory error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    Success = 0,
    OutOfMemory = 1,
    InvalidAddress = 2,
    PermissionDenied = 3,
    NotMapped = 4,
    AlreadyMapped = 5,
    AlignmentError = 6,
    /// OpenBSD/FreeBSD W^X Security Violation (Write XOR Execute)
    WxViolation = 7,
    /// FreeBSD Wired/Pinned page lock protection error
    WiredPageLocked = 8,
}

/// Virtual memory manager trait (OOP interface)
pub trait VirtualMemoryManager {
    /// Allocate virtual memory
    fn allocate(&mut self, size: usize, permissions: MemoryPermissions) -> Result<VirtualAddress, MemoryError>;
    /// Free virtual memory
    fn free(&mut self, addr: VirtualAddress) -> Result<(), MemoryError>;
    /// Map physical memory
    fn map_physical(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, size: usize, permissions: MemoryPermissions) -> Result<(), MemoryError>;
    /// Unmap memory
    fn unmap(&mut self, virtual_addr: VirtualAddress) -> Result<(), MemoryError>;
    /// Protect memory
    fn protect(&mut self, virtual_addr: VirtualAddress, permissions: MemoryPermissions) -> Result<(), MemoryError>;
    /// Get memory info
    fn info(&self, virtual_addr: VirtualAddress) -> Option<MemoryInfo>;
}

/// Memory info
#[repr(C)]
pub struct MemoryInfo {
    pub virtual_address: VirtualAddress,
    pub physical_address: Option<PhysicalAddress>,
    pub size: usize,
    pub permissions: MemoryPermissions,
    pub is_mapped: bool,
}

impl MemoryInfo {
    pub fn new(virtual_address: VirtualAddress) -> Self {
        MemoryInfo {
            virtual_address,
            physical_address: None,
            size: 0,
            permissions: MemoryPermissions::new(),
            is_mapped: false,
        }
    }
}

/// Simple virtual memory manager (OOP: Concrete implementation)
pub struct SimpleVirtualMemoryManager {
    page_tables: Vec<Option<NonNull<PageTable>>>,
    memory_regions: Vec<Option<NonNull<MemoryRegion>>>,
    next_virtual_address: AtomicUsize,
    capability: VMMCapability,
}

/// VMM capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMMCapability {
    pub can_allocate: bool,
    pub can_free: bool,
    pub can_map_physical: bool,
    pub can_unmap: bool,
}

impl VMMCapability {
    pub fn new() -> Self {
        VMMCapability {
            can_allocate: false,
            can_free: false,
            can_map_physical: false,
            can_unmap: false,
        }
    }

    pub fn full() -> Self {
        VMMCapability {
            can_allocate: true,
            can_free: true,
            can_map_physical: true,
            can_unmap: true,
        }
    }
}

impl SimpleVirtualMemoryManager {
    pub fn new(capability: VMMCapability) -> Self {
        SimpleVirtualMemoryManager {
            page_tables: Vec::new(),
            memory_regions: Vec::new(),
            next_virtual_address: AtomicUsize::new(0x1000000), // Start at 16MB
            capability,
        }
    }

    unsafe fn allocate_region(&mut self, start: VirtualAddress, size: usize, permissions: MemoryPermissions) -> Result<NonNull<MemoryRegion>, MemoryError> {
        if !is_canonical_address(start) || !is_canonical_address(start + size) {
            return Err(MemoryError::InvalidAddress);
        }

        // Enforce W^X (Write XOR Execute) security rule
        if !permissions.is_wx_compliant() {
            return Err(MemoryError::WxViolation);
        }

        let region = MemoryRegion::new(start, start + size, permissions, MemoryRegionCapability::full());
        let region_box = Box::new(region);
        let region_ptr = Box::into_raw(region_box);

        self.memory_regions.push(Some(NonNull::new_unchecked(region_ptr)));

        Ok(NonNull::new_unchecked(region_ptr))
    }

    unsafe fn find_region(&self, addr: VirtualAddress) -> Option<&MemoryRegion> {
        for slot in &self.memory_regions {
            if let Some(region_ptr) = slot {
                let region = region_ptr.as_ref();
                if region.contains(addr) {
                    return Some(region);
                }
            }
        }
        None
    }

    unsafe fn find_region_mut(&mut self, addr: VirtualAddress) -> Option<&mut MemoryRegion> {
        for slot in &mut self.memory_regions {
            if let Some(ref mut region_ptr) = slot {
                let region = region_ptr.as_mut();
                if region.contains(addr) {
                    return Some(region);
                }
            }
        }
        None
    }
}

impl VirtualMemoryManager for SimpleVirtualMemoryManager {
    fn allocate(&mut self, size: usize, permissions: MemoryPermissions) -> Result<VirtualAddress, MemoryError> {
        if !self.capability.can_allocate {
            return Err(MemoryError::PermissionDenied);
        }

        let aligned_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let addr = self.next_virtual_address.fetch_add(aligned_size, Ordering::SeqCst);

        unsafe {
            self.allocate_region(addr, aligned_size, permissions)?;
        }

        Ok(addr)
    }

    fn free(&mut self, addr: VirtualAddress) -> Result<(), MemoryError> {
        if !self.capability.can_free {
            return Err(MemoryError::PermissionDenied);
        }

        if !is_canonical_address(addr) {
            return Err(MemoryError::InvalidAddress);
        }

        unsafe {
            let mut index = None;
            for (i, slot) in self.memory_regions.iter().enumerate() {
                if let Some(region_ptr) = slot {
                    let region = region_ptr.as_ref();
                    if region.start == addr {
                        if region.permissions.is_wired {
                            return Err(MemoryError::WiredPageLocked);
                        }
                        index = Some(i);
                        break;
                    }
                }
            }

            if let Some(i) = index {
                if let Some(region_ptr) = self.memory_regions[i].take() {
                    let _ = Box::from_raw(region_ptr.as_ptr());
                }
                Ok(())
            } else {
                Err(MemoryError::InvalidAddress)
            }
        }
    }

    fn map_physical(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, size: usize, permissions: MemoryPermissions) -> Result<(), MemoryError> {
        if !self.capability.can_map_physical {
            return Err(MemoryError::PermissionDenied);
        }

        if !is_canonical_address(virtual_addr) || !is_canonical_address(virtual_addr + size) {
            return Err(MemoryError::InvalidAddress);
        }

        // Enforce W^X security rule
        if !permissions.is_wx_compliant() {
            return Err(MemoryError::WxViolation);
        }

        unsafe {
            let aligned_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
            self.allocate_region(virtual_addr, aligned_size, permissions)?;

            let flags = PageTableEntryFlags {
                present: true,
                writable: permissions.write,
                user_accessible: true,
                write_through: false,
                cache_disabled: false,
                accessed: false,
                dirty: false,
                global: false,
                is_wired: permissions.is_wired,
            };

            for offset in (0..aligned_size).step_by(PAGE_SIZE) {
                let page_table_base = (virtual_addr + offset) & !(PAGE_SIZE * 512 - 1);
                
                let mut pt_idx = None;
                for (idx, slot) in self.page_tables.iter().enumerate() {
                    if let Some(pt_ptr) = slot {
                        if pt_ptr.as_ref().base_virtual_address == page_table_base {
                            pt_idx = Some(idx);
                            break;
                        }
                    }
                }

                let idx = match pt_idx {
                    Some(i) => i,
                    None => {
                        let page_table = PageTable::new(page_table_base, PageTableCapability::full());
                        let pt_box = Box::new(page_table);
                        let pt_ptr = Box::into_raw(pt_box);
                        self.page_tables.push(Some(NonNull::new_unchecked(pt_ptr)));
                        self.page_tables.len() - 1
                    }
                };

                if let Some(pt_ptr) = self.page_tables[idx] {
                    let page_table = &mut *pt_ptr.as_ptr();
                    page_table.map_page(virtual_addr + offset, physical_addr + offset, flags)?;
                }
            }

            Ok(())
        }
    }

    fn unmap(&mut self, virtual_addr: VirtualAddress) -> Result<(), MemoryError> {
        if !self.capability.can_unmap {
            return Err(MemoryError::PermissionDenied);
        }

        if !is_canonical_address(virtual_addr) {
            return Err(MemoryError::InvalidAddress);
        }

        unsafe {
            if let Some(region) = self.find_region(virtual_addr) {
                if region.permissions.is_wired {
                    return Err(MemoryError::WiredPageLocked);
                }

                let size = region.size();
                for offset in (0..size).step_by(PAGE_SIZE) {
                    let target_vaddr = virtual_addr + offset;
                    for slot in &self.page_tables {
                        if let Some(pt_ptr) = slot {
                            let page_table = &mut *pt_ptr.as_ptr();
                            if page_table.get_entry(target_vaddr).is_some() {
                                let _ = page_table.unmap_page(target_vaddr);
                            }
                        }
                    }
                }

                self.free(virtual_addr)
            } else {
                Err(MemoryError::NotMapped)
            }
        }
    }

    fn protect(&mut self, virtual_addr: VirtualAddress, permissions: MemoryPermissions) -> Result<(), MemoryError> {
        if !is_canonical_address(virtual_addr) {
            return Err(MemoryError::InvalidAddress);
        }

        if !permissions.is_wx_compliant() {
            return Err(MemoryError::WxViolation);
        }

        unsafe {
            let size = if let Some(region) = self.find_region_mut(virtual_addr) {
                region.change_permissions(permissions)?;
                region.size()
            } else {
                return Err(MemoryError::NotMapped);
            };

            let flags = PageTableEntryFlags {
                present: true,
                writable: permissions.write,
                user_accessible: true,
                write_through: false,
                cache_disabled: false,
                accessed: false,
                dirty: false,
                global: false,
                is_wired: permissions.is_wired,
            };

            for offset in (0..size).step_by(PAGE_SIZE) {
                let target_vaddr = virtual_addr + offset;
                for slot in &self.page_tables {
                    if let Some(pt_ptr) = slot {
                        let page_table = &mut *pt_ptr.as_ptr();
                        if page_table.get_entry(target_vaddr).is_some() {
                            let _ = page_table.protect_page(target_vaddr, flags);
                        }
                    }
                }
            }

            Ok(())
        }
    }

    fn info(&self, virtual_addr: VirtualAddress) -> Option<MemoryInfo> {
        if !is_canonical_address(virtual_addr) {
            return None;
        }

        unsafe {
            if let Some(region) = self.find_region(virtual_addr) {
                let mut info = MemoryInfo::new(virtual_addr);
                info.size = region.size();
                info.permissions = region.permissions;
                info.is_mapped = true;
                Some(info)
            } else {
                None
            }
        }
    }
}

impl Drop for SimpleVirtualMemoryManager {
    fn drop(&mut self) {
        unsafe {
            for slot in &mut self.page_tables {
                if let Some(pt_ptr) = slot.take() {
                    let _ = Box::from_raw(pt_ptr.as_ptr());
                }
            }
            for slot in &mut self.memory_regions {
                if let Some(region_ptr) = slot.take() {
                    let _ = Box::from_raw(region_ptr.as_ptr());
                }
            }
        }
    }
}

// =========================================================================
// Linux kswapd-inspired Active/Inactive Page Reclaimer Scanning Subsystem
// =========================================================================

pub struct SovereignPageReclaimer {
    pub scanned_pages_count: usize,
    pub reclaimed_pages_count: usize,
}

impl SovereignPageReclaimer {
    pub fn new() -> Self {
        Self {
            scanned_pages_count: 0,
            reclaimed_pages_count: 0,
        }
    }

    /// Scans page tables, aging accessed bits and identifying unaccessed dirty pages for swap-out
    pub unsafe fn scan_and_reclaim(&mut self, page_tables: &mut [Option<NonNull<PageTable>>]) -> usize {
        let mut reclaimed = 0;

        for slot in page_tables.iter_mut() {
            if let Some(pt_ptr) = *slot {
                let page_table = &mut *pt_ptr.as_ptr();
                for entry_slot in page_table.entries.iter_mut() {
                    if let Some(ref mut entry) = *entry_slot {
                        self.scanned_pages_count += 1;
                        if entry.flags.is_wired {
                            continue; // Skip FreeBSD-style wired/pinned pages
                        }

                        if entry.flags.accessed {
                            // Clear accessed flag (first-chance LRU aging scan)
                            entry.flags.accessed = false;
                        } else {
                            // Page was not accessed since last scan -> Candidate for LRU eviction/reclaim
                            reclaimed += 1;
                        }
                    }
                }
            }
        }

        self.reclaimed_pages_count += reclaimed;
        reclaimed
    }
}

impl Default for SovereignPageReclaimer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_address_verifications() {
        assert!(is_canonical_address(0x0000_0000_1000_0000));
        assert_eq!(get_canonical_half(0x0000_0000_1000_0000), Some(CanonicalHalf::Lower));

        assert!(is_canonical_address(0xFFFF_8000_0000_1000));
        assert_eq!(get_canonical_half(0xFFFF_8000_0000_1000), Some(CanonicalHalf::Upper));

        assert!(!is_canonical_address(0x0001_8000_0000_0000));
        assert_eq!(get_canonical_half(0x0001_8000_0000_0000), None);

        let raw_addr = 0x0000_8000_0000_0123;
        let canonical_addr = canonicalize_address(raw_addr);
        assert!(is_canonical_address(canonical_addr));
        assert_eq!(get_canonical_half(canonical_addr), Some(CanonicalHalf::Upper));
    }

    #[test]
    fn test_wx_protection_enforcement() {
        let wx_perms = MemoryPermissions {
            read: true,
            write: true,
            execute: true,
            is_wired: false,
        };
        assert!(!wx_perms.is_wx_compliant()); // Violates W^X rule!

        let rx_perms = MemoryPermissions::read_execute();
        assert!(rx_perms.is_wx_compliant()); // Compliant RX

        let rw_perms = MemoryPermissions::read_write();
        assert!(rw_perms.is_wx_compliant()); // Compliant RW
    }

    #[test]
    fn test_page_table_wired_lock_protection() {
        let mut pt = PageTable::new(0x0000_0000_1000_0000, PageTableCapability::full());

        let mut flags = PageTableEntryFlags::new();
        flags.present = true;
        flags.is_wired = true; // Pinned page lock

        unsafe {
            pt.map_page(0x0000_0000_1000_0000, 0x100000, flags).unwrap();

            // Attempting to unmap a wired page returns WiredPageLocked error!
            assert_eq!(pt.unmap_page(0x0000_0000_1000_0000), Err(MemoryError::WiredPageLocked));
        }
    }

    #[test]
    fn test_page_reclaimer_aging_scan() {
        let mut pt = PageTable::new(0x0000_0000_1000_0000, PageTableCapability::full());

        let mut flags = PageTableEntryFlags::new();
        flags.present = true;
        flags.accessed = true; // Initially accessed

        unsafe {
            pt.map_page(0x0000_0000_1000_0000, 0x100000, flags).unwrap();
        }

        let mut reclaimer = SovereignPageReclaimer::new();

        let pt_ptr = NonNull::new(&mut pt as *mut PageTable).unwrap();
        let mut pt_array = [Some(pt_ptr)];

        unsafe {
            // First scan: clears accessed bit (first chance aging)
            let reclaimed_pass1 = reclaimer.scan_and_reclaim(&mut pt_array);
            assert_eq!(reclaimed_pass1, 0); // 0 reclaimed because it was marked accessed

            // Second scan: accessed is false -> page is identified as unaccessed LRU candidate!
            let reclaimed_pass2 = reclaimer.scan_and_reclaim(&mut pt_array);
            assert_eq!(reclaimed_pass2, 1);
        }
    }

    #[test]
    fn test_linear_page_translator() {
        let mut pt = PageTable::new(0x0000_0000_1000_0000, PageTableCapability::full());
        let mut flags = PageTableEntryFlags::new();
        flags.present = true;

        unsafe {
            pt.map_page(0x0000_0000_1000_0000, 0x200000, flags).unwrap();
        }

        let pt_ptr = NonNull::new(&mut pt as *mut PageTable).unwrap();
        let pt_slice = [Some(pt_ptr)];

        unsafe {
            // Translate virtual address 0x1000_0123 -> physical address 0x2000_0123
            let phys = LinearPageTranslator::translate_address(&pt_slice, 0x0000_0000_1000_0123).unwrap();
            assert_eq!(phys, 0x0020_0123);
        }
    }

    #[test]
    fn test_page_relationship_cardinality() {
        let mut tracker = PageRelationshipTracker::new();

        // 1-to-1 Mapping
        tracker.register_page_mapping(0x5000, 0x1000_0000, false);
        assert_eq!(tracker.get_relationship(0x5000), Some(PageMappingRelationship::OneToOne));

        // 1-to-N Mapping
        tracker.register_page_mapping(0x5000, 0x2000_0000, false);
        assert_eq!(tracker.get_relationship(0x5000), Some(PageMappingRelationship::OneToMany));

        // N-to-N CoW Mapping
        tracker.register_page_mapping(0x6000, 0x3000_0000, true);
        assert_eq!(tracker.get_relationship(0x6000), Some(PageMappingRelationship::ManyToMany));
    }
}
