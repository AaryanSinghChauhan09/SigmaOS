// OOP-based Virtual Memory Manager with Canonical Address Verification for SigmaOS
// Implements virtual memory management using OOP principles with traits and structs
// No dependency on external memory management libraries
// Inspired by x86_64, ARM64, Linux, BSD, and Windows canonical memory layouts
// Enhanced with OpenBSD/FreeBSD W^X (Write XOR Execute) security, FreeBSD wired/pinned page protection,
// and Linux kswapd-inspired active/inactive LRU page reclaimer scanning.

use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

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
    pub fn new(capability: PageTableCapability) -> Self {
        PageTable {
            entries: [None; 512],
            capability,
        }
    }

    pub unsafe fn map_page(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, flags: PageTableEntryFlags) -> Result<(), MemoryError> {
        if !self.capability.can_map {
            return Err(MemoryError::PermissionDenied);
        }

        let index = (virtual_addr / PAGE_SIZE) % 512;
        let entry = PageTableEntry::new(physical_addr, flags);
        self.entries[index] = Some(entry);
        Ok(())
    }

    pub unsafe fn unmap_page(&mut self, virtual_addr: VirtualAddress) -> Result<(), MemoryError> {
        if !self.capability.can_unmap {
            return Err(MemoryError::PermissionDenied);
        }

        let index = (virtual_addr / PAGE_SIZE) % 512;
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

        let index = (virtual_addr / PAGE_SIZE) % 512;
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
        let index = (virtual_addr / PAGE_SIZE) % 512;
        self.entries[index].as_ref()
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
        let region_ptr = alloc(mem::size_of::<MemoryRegion>()) as *mut MemoryRegion;

        if region_ptr.is_null() {
            return Err(MemoryError::OutOfMemory);
        }

        core::ptr::write(region_ptr, region);
        self.memory_regions.push(Some(NonNull::new_unchecked(region_ptr)));

        Ok(NonNull::new_unchecked(region_ptr))
    }

    unsafe fn find_region(&self, addr: VirtualAddress) -> Option<&MemoryRegion> {
        for i in 0..self.memory_regions.len {
            let slot = &*self.memory_regions.data.add(i);
            if let Some(region_ptr) = *slot {
                let region = &*region_ptr.as_ptr();
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
            for i in 0..self.memory_regions.len {
                let slot = &*self.memory_regions.data.add(i);
                if let Some(region_ptr) = *slot {
                    let region = &*region_ptr.as_ptr();
                    if region.start == addr {
                        // Check FreeBSD-style wired/pinned lock
                        if region.permissions.is_wired {
                            return Err(MemoryError::WiredPageLocked);
                        }
                        index = Some(i);
                        break;
                    }
                }
            }

            if let Some(i) = index {
                let slot = &mut *self.memory_regions.data.add(i);
                if let Some(region_ptr) = *slot {
                    core::ptr::drop_in_place(region_ptr.as_ptr());
                    free(region_ptr.as_ptr() as *mut u8);
                }
                *slot = None;
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
                let page_table_index = (virtual_addr + offset) / (PAGE_SIZE * 512);
                
                while page_table_index >= self.page_tables.len {
                    let page_table = PageTable::new(PageTableCapability::full());
                    let pt_ptr = alloc(mem::size_of::<PageTable>()) as *mut PageTable;
                    if pt_ptr.is_null() {
                        return Err(MemoryError::OutOfMemory);
                    }
                    core::ptr::write(pt_ptr, page_table);
                    self.page_tables.push(Some(NonNull::new_unchecked(pt_ptr)));
                }

                let slot = &mut *self.page_tables.data.add(page_table_index);
                if let Some(pt_ptr) = *slot {
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
                    let page_table_index = (virtual_addr + offset) / (PAGE_SIZE * 512);
                    
                    if page_table_index < self.page_tables.len {
                        let slot = &mut *self.page_tables.data.add(page_table_index);
                        if let Some(pt_ptr) = *slot {
                            let page_table = &mut *pt_ptr.as_ptr();
                            page_table.unmap_page(virtual_addr + offset)?;
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
            if let Some(region) = self.find_region(virtual_addr) {
                region.change_permissions(permissions)?;

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

                let size = region.size();
                for offset in (0..size).step_by(PAGE_SIZE) {
                    let page_table_index = (virtual_addr + offset) / (PAGE_SIZE * 512);
                    
                    if page_table_index < self.page_tables.len {
                        let slot = &mut *self.page_tables.data.add(page_table_index);
                        if let Some(pt_ptr) = *slot {
                            let page_table = &mut *pt_ptr.as_ptr();
                            page_table.protect_page(virtual_addr + offset, flags)?;
                        }
                    }
                }

                Ok(())
            } else {
                Err(MemoryError::NotMapped)
            }
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

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions / Test Shims
#[cfg(not(test))]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, 8))
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    if !ptr.is_null() {
        std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align_unchecked(1, 8));
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
        let mut pt = PageTable::new(PageTableCapability::full());

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
        let mut pt = PageTable::new(PageTableCapability::full());

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
}
