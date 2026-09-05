// SPDX-License-Identifier: MIT
/// Memory Protection (mprotect)
/// Manages virtual memory page permissions for user space processes

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::sync::atomic::Mutex;

/// Memory Protection Flags
pub mod prot_flags {
    pub const PROT_NONE: u32 = 0x00;      // No permissions
    pub const PROT_READ: u32 = 0x01;      // Read permission
    pub const PROT_WRITE: u32 = 0x02;     // Write permission
    pub const PROT_EXEC: u32 = 0x04;      // Execute permission
}

use prot_flags::*;

/// Page Protection Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageProtection {
    pub address: u64,
    pub size: u64,
    pub flags: u32,
}

impl PageProtection {
    /// Create new page protection entry
    pub fn new(address: u64, size: u64, flags: u32) -> Self {
        Self {
            address,
            size,
            flags,
        }
    }
    
    /// Check if page has read permission
    pub fn is_readable(&self) -> bool {
        (self.flags & PROT_READ) != 0
    }
    
    /// Check if page has write permission
    pub fn is_writable(&self) -> bool {
        (self.flags & PROT_WRITE) != 0
    }
    
    /// Check if page has execute permission
    pub fn is_executable(&self) -> bool {
        (self.flags & PROT_EXEC) != 0
    }
    
    /// Check if address falls within this protection region
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.address && addr < (self.address + self.size)
    }
}

/// Memory Protection Table for a process
/// Maps address ranges to protection flags
pub struct MemoryProtectionTable {
    /// Protection entries indexed by start address
    entries: BTreeMap<u64, PageProtection>,
}

impl MemoryProtectionTable {
    /// Create new empty protection table
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
    
    /// Set protection for memory region
    /// 
    /// # Arguments
    /// * `address` - Start address (must be page-aligned)
    /// * `size` - Size of region (must be multiple of page size)
    /// * `flags` - Protection flags (PROT_READ | PROT_WRITE | PROT_EXEC)
    pub fn mprotect(
        &mut self,
        address: u64,
        size: u64,
        flags: u32,
    ) -> Result<(), &'static str> {
        // Validate page alignment (assume 4KB pages)
        if address % 4096 != 0 {
            return Err("Address not page-aligned");
        }
        
        if size == 0 || size % 4096 != 0 {
            return Err("Size must be multiple of page size");
        }
        
        // Validate flags
        if flags > (PROT_READ | PROT_WRITE | PROT_EXEC) {
            return Err("Invalid protection flags");
        }
        
        // Check for overlapping regions and split if necessary
        self.handle_overlap(address, size, flags)?;
        
        // Insert new protection entry
        let prot = PageProtection::new(address, size, flags);
        self.entries.insert(address, prot);
        
        Ok(())
    }
    
    /// Handle overlapping regions
    fn handle_overlap(
        &mut self,
        address: u64,
        size: u64,
        _flags: u32,
    ) -> Result<(), &'static str> {
        let end = address + size;
        
        // Find overlapping entries
        let overlapping: Vec<_> = self.entries
            .values()
            .filter(|entry| {
                let entry_end = entry.address + entry.size;
                !(end <= entry.address || address >= entry_end)
            })
            .copied()
            .collect();
        
        // Remove overlapping entries (will be replaced or split)
        for entry in overlapping {
            self.entries.remove(&entry.address);
        }
        
        Ok(())
    }
    
    /// Get protection for address
    pub fn get_protection(&self, address: u64) -> Option<PageProtection> {
        for entry in self.entries.values() {
            if entry.contains(address) {
                return Some(*entry);
            }
        }
        None
    }
    
    /// Check if address can be read
    pub fn can_read(&self, address: u64) -> bool {
        self.get_protection(address)
            .map(|p| p.is_readable())
            .unwrap_or(false)
    }
    
    /// Check if address can be written
    pub fn can_write(&self, address: u64) -> bool {
        self.get_protection(address)
            .map(|p| p.is_writable())
            .unwrap_or(false)
    }
    
    /// Check if address can be executed
    pub fn can_execute(&self, address: u64) -> bool {
        self.get_protection(address)
            .map(|p| p.is_executable())
            .unwrap_or(false)
    }
}

impl Default for MemoryProtectionTable {
    fn default() -> Self {
        Self::new()
    }
}

/// System-wide Memory Protection Manager
/// Maintains protection tables for all processes
pub struct MemoryProtectionManager {
    /// Protection tables per process ID
    protection_tables: Arc<Mutex<BTreeMap<u32, Arc<Mutex<MemoryProtectionTable>>>>>,
}

impl MemoryProtectionManager {
    /// Create new memory protection manager
    pub fn new() -> Self {
        Self {
            protection_tables: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
    
    /// Create protection table for new process
    pub fn create_process_table(&self, pid: u32) -> Result<(), &'static str> {
        let mut tables = self.protection_tables.lock().unwrap();
        tables.insert(pid, Arc::new(Mutex::new(MemoryProtectionTable::new())));
        Ok(())
    }
    
    /// Remove protection table when process exits
    pub fn remove_process_table(&self, pid: u32) -> Result<(), &'static str> {
        let mut tables = self.protection_tables.lock().unwrap();
        tables.remove(&pid)
            .ok_or("Process table not found")?;
        Ok(())
    }
    
    /// Apply mprotect syscall
    pub fn mprotect(
        &self,
        pid: u32,
        address: u64,
        size: u64,
        flags: u32,
    ) -> Result<(), &'static str> {
        let tables = self.protection_tables.lock().unwrap();
        let table = tables.get(&pid)
            .ok_or("Process table not found")?;
        
        let mut prot_table = table.lock().unwrap();
        prot_table.mprotect(address, size, flags)
    }
    
    /// Get protection for address in process
    pub fn get_protection(
        &self,
        pid: u32,
        address: u64,
    ) -> Result<Option<PageProtection>, &'static str> {
        let tables = self.protection_tables.lock().unwrap();
        let table = tables.get(&pid)
            .ok_or("Process table not found")?;
        
        let prot_table = table.lock().unwrap();
        Ok(prot_table.get_protection(address))
    }
    
    /// Check if access is allowed
    pub fn check_access(
        &self,
        pid: u32,
        address: u64,
        write: bool,
    ) -> Result<bool, &'static str> {
        let tables = self.protection_tables.lock().unwrap();
        let table = tables.get(&pid)
            .ok_or("Process table not found")?;
        
        let prot_table = table.lock().unwrap();
        Ok(if write {
            prot_table.can_write(address)
        } else {
            prot_table.can_read(address)
        })
    }
}

impl Default for MemoryProtectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_page_protection_creation() {
        let prot = PageProtection::new(0x1000, 4096, PROT_READ | PROT_WRITE);
        assert_eq!(prot.address, 0x1000);
        assert_eq!(prot.size, 4096);
        assert!(prot.is_readable());
        assert!(prot.is_writable());
        assert!(!prot.is_executable());
    }
    
    #[test]
    fn test_page_contains() {
        let prot = PageProtection::new(0x1000, 4096, PROT_READ);
        assert!(prot.contains(0x1000));
        assert!(prot.contains(0x1800));
        assert!(!prot.contains(0x2000));
        assert!(!prot.contains(0x0FFF));
    }
    
    #[test]
    fn test_mprotect() {
        let mut table = MemoryProtectionTable::new();
        
        // Should succeed with valid parameters
        assert!(table.mprotect(0x1000, 4096, PROT_READ | PROT_WRITE).is_ok());
        
        // Verify protection was set
        let prot = table.get_protection(0x1000).unwrap();
        assert!(prot.is_readable());
        assert!(prot.is_writable());
    }
    
    #[test]
    fn test_mprotect_invalid_alignment() {
        let mut table = MemoryProtectionTable::new();
        
        // Should fail with non-aligned address
        assert!(table.mprotect(0x1001, 4096, PROT_READ).is_err());
    }
    
    #[test]
    fn test_mprotect_invalid_size() {
        let mut table = MemoryProtectionTable::new();
        
        // Should fail with non-aligned size
        assert!(table.mprotect(0x1000, 4097, PROT_READ).is_err());
    }
    
    #[test]
    fn test_manager_create_process() {
        let manager = MemoryProtectionManager::new();
        assert!(manager.create_process_table(1).is_ok());
        assert!(manager.create_process_table(2).is_ok());
    }
    
    #[test]
    fn test_manager_mprotect() {
        let manager = MemoryProtectionManager::new();
        manager.create_process_table(1).unwrap();
        
        assert!(manager.mprotect(1, 0x1000, 4096, PROT_READ).is_ok());
        
        let prot = manager.get_protection(1, 0x1000).unwrap();
        assert!(prot.is_some());
    }
}
