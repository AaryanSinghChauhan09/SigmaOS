// SigmaOS Virtual Memory Manager
// User/kernel virtual address space isolation, mapping protections, swap/reclaim triggers, and OOM handling.

use super::MmError;
use core::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::vec::Vec;

pub const USER_SPACE_START: u64 = 0x0000_0000_0000_1000;
pub const USER_SPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF;
pub const KERNEL_SPACE_START: u64 = 0xFFFF_8000_0000_0000;
pub const KERNEL_SPACE_END: u64 = 0xFFFF_FFFF_FFFF_FFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmaProtection {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

impl VmaProtection {
    pub fn validate_wx(&self) -> bool {
        // Enforce W^X (Write XOR Execute) policy
        !(self.write && self.execute)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmaRegion {
    pub start_addr: u64,
    pub end_addr: u64,
    pub protection: VmaProtection,
    pub name: std::string::String,
}

impl VmaRegion {
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start_addr && addr < self.end_addr
    }
}

#[derive(Debug)]
pub struct VirtualMemoryManager {
    vmas: Mutex<Vec<VmaRegion>>,
    total_mapped_bytes: AtomicUsize,
    max_mapped_bytes_limit: usize,
    max_vma_count: usize,
}

impl VirtualMemoryManager {
    pub fn new(max_mapped_bytes_limit: usize, max_vma_count: usize) -> Self {
        Self {
            vmas: Mutex::new(Vec::new()),
            total_mapped_bytes: AtomicUsize::new(0),
            max_mapped_bytes_limit,
            max_vma_count,
        }
    }

    /// Allocate and map a new Virtual Memory Area
    pub fn create_vma(&self, start: u64, length: u64, protection: VmaProtection, name: &str) -> Result<VmaRegion, MmError> {
        if length == 0 {
            return Err(MmError::InvalidParam);
        }

        let end = start.checked_add(length).ok_or(MmError::InvalidParam)?;

        // Enforce W^X protection policy
        if !protection.validate_wx() {
            return Err(MmError::PermissionDenied);
        }

        // Validate address space range (User vs Kernel)
        let is_user = start >= USER_SPACE_START && end <= USER_SPACE_END;
        let is_kernel = start >= KERNEL_SPACE_START && end <= KERNEL_SPACE_END;

        if !is_user && !is_kernel {
            return Err(MmError::InvalidParam);
        }

        if protection.user && !is_user {
            return Err(MmError::PermissionDenied);
        }

        let mut list = self.vmas.lock().unwrap();

        if list.len() >= self.max_vma_count {
            return Err(MmError::OutOfMemory);
        }

        let current_mapped = self.total_mapped_bytes.load(Ordering::SeqCst);
        if current_mapped + (length as usize) > self.max_mapped_bytes_limit {
            return Err(MmError::OutOfMemory);
        }

        // Check overlapping VMAs
        for existing in list.iter() {
            if (start < existing.end_addr) && (end > existing.start_addr) {
                return Err(MmError::InvalidParam);
            }
        }

        let vma = VmaRegion {
            start_addr: start,
            end_addr: end,
            protection,
            name: name.into(),
        };

        list.push(vma.clone());
        self.total_mapped_bytes.fetch_add(length as usize, Ordering::SeqCst);

        Ok(vma)
    }

    /// Update protection flags of an existing VMA
    pub fn update_vma_protection(&self, start: u64, new_protection: VmaProtection) -> Result<(), MmError> {
        if !new_protection.validate_wx() {
            return Err(MmError::PermissionDenied);
        }

        let mut list = self.vmas.lock().unwrap();
        for vma in list.iter_mut() {
            if vma.start_addr == start {
                if new_protection.user && !(vma.start_addr >= USER_SPACE_START && vma.end_addr <= USER_SPACE_END) {
                    return Err(MmError::PermissionDenied);
                }
                vma.protection = new_protection;
                return Ok(());
            }
        }
        Err(MmError::NotFound)
    }

    /// Remove and unmap a VMA
    pub fn remove_vma(&self, start: u64) -> Result<(), MmError> {
        let mut list = self.vmas.lock().unwrap();
        let mut found_idx = None;

        for (idx, vma) in list.iter().enumerate() {
            if vma.start_addr == start {
                found_idx = Some((idx, vma.end_addr - vma.start_addr));
                break;
            }
        }

        if let Some((idx, len)) = found_idx {
            list.remove(idx);
            self.total_mapped_bytes.fetch_sub(len as usize, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MmError::NotFound)
        }
    }

    /// Check read/write/execute permission for a given virtual address
    pub fn check_permission(&self, addr: u64, write_req: bool, exec_req: bool, is_user_mode: bool) -> Result<(), MmError> {
        let list = self.vmas.lock().unwrap();
        for vma in list.iter() {
            if vma.contains(addr) {
                if is_user_mode && !vma.protection.user {
                    return Err(MmError::PermissionDenied);
                }
                if write_req && !vma.protection.write {
                    return Err(MmError::PermissionDenied);
                }
                if exec_req && !vma.protection.execute {
                    return Err(MmError::PermissionDenied);
                }
                return Ok(());
            }
        }
        Err(MmError::NotFound)
    }

    pub fn total_mapped_bytes(&self) -> usize {
        self.total_mapped_bytes.load(Ordering::SeqCst)
    }
}
