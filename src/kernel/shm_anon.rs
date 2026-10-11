// SPDX-License-Identifier: MIT
// SigmaOS - POSIX Shared Memory with Anonymous Mmap Backing
// Inspired by Linux shm with anonymous mmap backing (mm/shmem.c)
// Provides physical page backing for shared memory segments via anonymous mmap

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::kernel::shm::{ShmManager, ShmPerm, ShmSegment};

// Local mmap types to avoid dependency on mm module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmapError {
    InvalidArgs,
    NoMemory,
    Permission,
    BadFd,
    Exists,
    NotMapped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MmapProt(pub u32);

impl MmapProt {
    pub const NONE: u32 = 0;
    pub const READ: u32 = 1;
    pub const WRITE: u32 = 2;
    pub const EXEC: u32 = 4;

    pub fn readable(&self) -> bool {
        self.0 & Self::READ != 0
    }
    pub fn writable(&self) -> bool {
        self.0 & Self::WRITE != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MmapFlags(pub u32);

impl MmapFlags {
    pub const SHARED: u32 = 0x01;
    pub const PRIVATE: u32 = 0x02;
    pub const FIXED: u32 = 0x10;
    pub const ANONYMOUS: u32 = 0x20;
    pub const POPULATE: u32 = 0x8000;
    pub const LOCKED: u32 = 0x2000;
    pub const GROWSDOWN: u32 = 0x0100;

    pub fn shared(&self) -> bool {
        self.0 & Self::SHARED != 0
    }
    pub fn private(&self) -> bool {
        self.0 & Self::PRIVATE != 0
    }
}

/// Anonymous shared memory mapping
#[derive(Debug, Clone)]
pub struct ShmAnonMapping {
    pub shm_id: u64,
    pub virt_addr: usize,
    pub size: usize,
    pub prot: MmapProt,
    pub flags: MmapFlags,
    pub ref_count: usize,
}

/// Anonymous shared memory manager
pub struct ShmAnonManager {
    pub shm_manager: ShmManager,
    pub mappings: BTreeMap<usize, ShmAnonMapping>, // virt_addr -> mapping
    pub next_virt_addr: usize,
}

impl ShmAnonManager {
    pub fn new() -> Self {
        Self {
            shm_manager: ShmManager::new(),
            mappings: BTreeMap::new(),
            next_virt_addr: 0x7fff00000000usize, // High virtual address range
        }
    }
    
    /// Create a shared memory segment with anonymous mmap backing
    pub fn shm_open_anon(&mut self, name: &str, size: usize, perm: ShmPerm) -> Result<u64, String> {
        let key = name.len() as i32;
        let shm_id = self.shm_manager.create_segment(key, size, perm);
        Ok(shm_id)
    }
    
    /// Map a shared memory segment into process address space (anonymous mmap)
    pub fn mmap_shm(&mut self, shm_id: u64, prot: MmapProt, flags: MmapFlags) -> Result<usize, MmapError> {
        // Get the shared memory segment
        let segment = self.shm_manager.get_segment(shm_id)
            .ok_or(MmapError::BadFd)?;
        
        // Allocate virtual address
        let virt_addr = self.next_virt_addr;
        self.next_virt_addr += segment.size;
        
        // Create mapping
        let mapping = ShmAnonMapping {
            shm_id,
            virt_addr,
            size: segment.size,
            prot,
            flags,
            ref_count: 1,
        };
        
        self.mappings.insert(virt_addr, mapping);
        
        // Attach to shared memory segment
        self.shm_manager.attach(shm_id).map_err(|_| MmapError::Permission)?;
        
        Ok(virt_addr)
    }
    
    /// Unmap a shared memory mapping
    pub fn munmap_shm(&mut self, virt_addr: usize) -> Result<(), MmapError> {
        let mapping = self.mappings.remove(&virt_addr)
            .ok_or(MmapError::NotMapped)?;
        
        // Detach from shared memory segment
        self.shm_manager.detach(mapping.shm_id).map_err(|_| MmapError::Permission)?;
        
        Ok(())
    }
    
    /// Get shared memory segment by virtual address
    pub fn get_shm_by_addr(&self, virt_addr: usize) -> Option<ShmSegment> {
        let mapping = self.mappings.get(&virt_addr)?;
        self.shm_manager.get_segment(mapping.shm_id)
    }
    
    /// Read from shared memory via virtual address
    pub fn read_shm(&self, virt_addr: usize, offset: usize, count: usize) -> Result<Vec<u8>, MmapError> {
        let mapping = self.mappings.get(&virt_addr)
            .ok_or(MmapError::NotMapped)?;
        
        if !mapping.prot.readable() {
            return Err(MmapError::Permission);
        }
        
        self.shm_manager.read(mapping.shm_id, offset, count)
            .map_err(|_| MmapError::Permission)
    }
    
    /// Write to shared memory via virtual address
    pub fn write_shm(&mut self, virt_addr: usize, offset: usize, data: &[u8]) -> Result<(), MmapError> {
        let mapping = self.mappings.get(&virt_addr)
            .ok_or(MmapError::NotMapped)?;
        
        if !mapping.prot.writable() {
            return Err(MmapError::Permission);
        }
        
        self.shm_manager.write(mapping.shm_id, offset, data)
            .map_err(|_| MmapError::Permission)
    }
    
    /// Protect shared memory mapping (change permissions)
    pub fn mprotect_shm(&mut self, virt_addr: usize, new_prot: MmapProt) -> Result<(), MmapError> {
        let mapping = self.mappings.get_mut(&virt_addr)
            .ok_or(MmapError::NotMapped)?;
        
        mapping.prot = new_prot;
        Ok(())
    }
    
    /// Increase reference count for forked processes
    pub fn fork_mapping(&mut self, virt_addr: usize) -> Result<(), MmapError> {
        let mapping = self.mappings.get_mut(&virt_addr)
            .ok_or(MmapError::NotMapped)?;
        
        mapping.ref_count += 1;
        Ok(())
    }
    
    /// Decrease reference count (process exit)
    pub fn unref_mapping(&mut self, virt_addr: usize) -> Result<usize, MmapError> {
        let mapping = self.mappings.get_mut(&virt_addr)
            .ok_or(MmapError::NotMapped)?;
        
        mapping.ref_count -= 1;
        Ok(mapping.ref_count)
    }
    
    /// Remove shared memory segment by ID
    pub fn shm_unlink(&mut self, shm_id: u64) -> Result<(), String> {
        self.shm_manager.remove_segment(shm_id)
    }
}

impl Default for ShmAnonManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global anonymous shared memory manager
static mut SHM_ANON_MANAGER: Option<ShmAnonManager> = None;

/// Initialize the anonymous shared memory manager
pub fn init_shm_anon() {
    unsafe {
        SHM_ANON_MANAGER = Some(ShmAnonManager::new());
    }
}

/// Get the global anonymous shared memory manager
pub fn get_shm_anon_manager() -> Option<&'static mut ShmAnonManager> {
    unsafe {
        SHM_ANON_MANAGER.as_mut()
    }
}

/// POSIX shm_open - open or create a shared memory object
pub fn shm_open(name: &str, oflag: u32, mode: u16) -> Result<i32, String> {
    let manager = get_shm_anon_manager().ok_or_else(|| "shm anon manager not initialized".to_string())?;
    
    let size = 4096; // Default page size
    let perm = ShmPerm::new().with_read().with_write();
    
    let shm_id = manager.shm_open_anon(name, size, perm)?;
    Ok(shm_id as i32)
}

/// POSIX shm_close - close a shared memory object
pub fn shm_close(shm_fd: i32) -> Result<(), String> {
    // In this implementation, shm_fd is the shm_id
    // Close is a no-op as segments persist until shm_unlink
    Ok(())
}

/// POSIX shm_unlink - remove a shared memory object
pub fn shm_unlink(name: &str) -> Result<(), String> {
    let manager = get_shm_anon_manager().ok_or_else(|| "shm anon manager not initialized".to_string())?;
    
    let key = name.len() as i32;
    let segment = manager.shm_manager.get_segment_by_key(key)
        .ok_or_else(|| format!("Shared memory {} not found", name))?;
    
    manager.shm_unlink(segment.id)
}

/// mmap a shared memory object
pub fn mmap_shm(shm_fd: i32, prot: MmapProt, flags: MmapFlags) -> Result<usize, MmapError> {
    let manager = get_shm_anon_manager().ok_or(MmapError::Permission)?;
    
    let shm_id = shm_fd as u64;
    manager.mmap_shm(shm_id, prot, flags)
}

/// munmap a shared memory mapping
pub fn munmap_shm(virt_addr: usize) -> Result<(), MmapError> {
    let manager = get_shm_anon_manager().ok_or(MmapError::Permission)?;
    manager.munmap_shm(virt_addr)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shm_anon_manager_init() {
        let manager = ShmAnonManager::new();
        assert_eq!(manager.mappings.len(), 0);
    }
    
    #[test]
    fn test_shm_open_anon() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        assert!(shm_id > 0);
    }
    
    #[test]
    fn test_mmap_shm() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = manager.mmap_shm(shm_id, prot, flags).unwrap();
        assert!(virt_addr > 0);
        assert!(manager.mappings.contains_key(&virt_addr));
    }
    
    #[test]
    fn test_munmap_shm() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = manager.mmap_shm(shm_id, prot, flags).unwrap();
        manager.munmap_shm(virt_addr).unwrap();
        
        assert!(!manager.mappings.contains_key(&virt_addr));
    }
    
    #[test]
    fn test_read_write_shm() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = manager.mmap_shm(shm_id, prot, flags).unwrap();
        manager.write_shm(virt_addr, 0, b"Hello").unwrap();
        
        let data = manager.read_shm(virt_addr, 0, 5).unwrap();
        assert_eq!(data, b"Hello");
    }
    
    #[test]
    fn test_mprotect_shm() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = manager.mmap_shm(shm_id, prot, flags).unwrap();
        let new_prot = MmapProt(MmapProt::READ);
        
        manager.mprotect_shm(virt_addr, new_prot).unwrap();
        
        let mapping = manager.mappings.get(&virt_addr).unwrap();
        assert_eq!(mapping.prot.0, MmapProt::READ);
    }
    
    #[test]
    fn test_fork_mapping() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = manager.mmap_shm(shm_id, prot, flags).unwrap();
        manager.fork_mapping(virt_addr).unwrap();
        
        let mapping = manager.mappings.get(&virt_addr).unwrap();
        assert_eq!(mapping.ref_count, 2);
    }
    
    #[test]
    fn test_unref_mapping() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = manager.mmap_shm(shm_id, prot, flags).unwrap();
        manager.fork_mapping(virt_addr).unwrap();
        
        let ref_count = manager.unref_mapping(virt_addr).unwrap();
        assert_eq!(ref_count, 1);
    }
    
    #[test]
    fn test_shm_unlink() {
        let mut manager = ShmAnonManager::new();
        let perm = ShmPerm::new().with_read().with_write();
        
        let shm_id = manager.shm_open_anon("test_shm", 4096, perm).unwrap();
        manager.shm_unlink(shm_id).unwrap();
        
        assert!(manager.shm_manager.get_segment(shm_id).is_none());
    }
    
    #[test]
    fn test_shm_open_close_unlink() {
        init_shm_anon();
        
        let shm_fd = shm_open("test_shm", 0, 0o600).unwrap();
        assert!(shm_fd > 0);
        
        shm_close(shm_fd).unwrap();
        shm_unlink("test_shm").unwrap();
    }
    
    #[test]
    fn test_shm_mmap_munmap() {
        init_shm_anon();
        
        let shm_fd = shm_open("test_shm", 0, 0o600).unwrap();
        let prot = MmapProt(MmapProt::READ | MmapProt::WRITE);
        let flags = MmapFlags(MmapFlags::PRIVATE | MmapFlags::ANONYMOUS);
        
        let virt_addr = mmap_shm(shm_fd, prot, flags).unwrap();
        assert!(virt_addr > 0);
        
        munmap_shm(virt_addr).unwrap();
    }
}
