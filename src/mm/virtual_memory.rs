#![no_std]
#![no_main]

/// OOP-based Virtual Memory Manager for SigmaOS
/// Implements virtual memory management using OOP principles with traits and structs
/// No dependency on external memory management libraries

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Page size (4KB)
const PAGE_SIZE: usize = 4096;

/// Virtual address
pub type VirtualAddress = usize;

/// Physical address
pub type PhysicalAddress = usize;

/// Page table entry flags
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntryFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub write_through: bool,
    pub cache_disabled: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub global: bool,
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
        flags
    }
}

/// Page table entry
#[repr(C)]
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
        self.entries[index] = None;
        Ok(())
    }

    pub unsafe fn protect_page(&mut self, virtual_addr: VirtualAddress, flags: PageTableEntryFlags) -> Result<(), MemoryError> {
        if !self.capability.can_protect {
            return Err(MemoryError::PermissionDenied);
        }

        let index = (virtual_addr / PAGE_SIZE) % 512;
        if let Some(ref mut entry) = self.entries[index] {
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
#[derive(Debug, Clone, Copy)]
pub struct MemoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl MemoryPermissions {
    pub fn new() -> Self {
        MemoryPermissions {
            read: false,
            write: false,
            execute: false,
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
        self.permissions = permissions;
        Ok(())
    }
}

/// Memory error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MemoryError {
    Success = 0,
    OutOfMemory = 1,
    InvalidAddress = 2,
    PermissionDenied = 3,
    NotMapped = 4,
    AlreadyMapped = 5,
    AlignmentError = 6,
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
        for region_option in &self.memory_regions {
            if let Some(region_ptr) = *region_option {
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

        unsafe {
            let mut index = None;
            for (i, region_option) in self.memory_regions.iter().enumerate() {
                if let Some(region_ptr) = *region_option {
                    let region = &*region_ptr.as_ptr();
                    if region.start == addr {
                        index = Some(i);
                        break;
                    }
                }
            }

            if let Some(i) = index {
                if let Some(region_ptr) = self.memory_regions[i] {
                    core::ptr::drop_in_place(region_ptr.as_ptr());
                    free(region_ptr.as_ptr() as *mut u8);
                }
                self.memory_regions[i] = None;
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
            };

            for offset in (0..aligned_size).step_by(PAGE_SIZE) {
                let page_table_index = (virtual_addr + offset) / (PAGE_SIZE * 512);
                
                while page_table_index >= self.page_tables.len() {
                    let page_table = PageTable::new(PageTableCapability::full());
                    let pt_ptr = alloc(mem::size_of::<PageTable>()) as *mut PageTable;
                    if pt_ptr.is_null() {
                        return Err(MemoryError::OutOfMemory);
                    }
                    core::ptr::write(pt_ptr, page_table);
                    self.page_tables.push(Some(NonNull::new_unchecked(pt_ptr)));
                }

                if let Some(pt_ptr) = self.page_tables[page_table_index] {
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

        unsafe {
            if let Some(region) = self.find_region(virtual_addr) {
                let size = region.size();
                
                for offset in (0..size).step_by(PAGE_SIZE) {
                    let page_table_index = (virtual_addr + offset) / (PAGE_SIZE * 512);
                    
                    if page_table_index < self.page_tables.len() {
                        if let Some(pt_ptr) = self.page_tables[page_table_index] {
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
                };

                let size = region.size();
                for offset in (0..size).step_by(PAGE_SIZE) {
                    let page_table_index = (virtual_addr + offset) / (PAGE_SIZE * 512);
                    
                    if page_table_index < self.page_tables.len() {
                        if let Some(pt_ptr) = self.page_tables[page_table_index] {
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

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
