//! Address Sanitizer (ASan) Memory Error Detection
//!
//! ASan-inspired memory error detection with redzone-based overflow detection,
//! use-after-free tracking, stack corruption detection, and shadow memory mapping.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Memory region with redzone protection
#[derive(Debug, Clone)]
pub struct AsanRegion {
    base_address: u64,
    size: usize,
    redzone_size: usize,
    shadow_offset: u64,
    is_allocated: bool,
    pub start: u64,
    pub end: u64,
}

impl AsanRegion {
    pub fn new(base_address: u64, size: usize, redzone_size: usize, shadow_offset: u64) -> Self {
        let start = base_address;
        let end = base_address + size as u64 + 2 * redzone_size as u64;
        Self {
            base_address,
            size,
            redzone_size,
            shadow_offset,
            is_allocated: true,
            start,
            end,
        }
    }

    pub fn base_address(&self) -> u64 {
        self.base_address
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn redzone_size(&self) -> usize {
        self.redzone_size
    }

    pub fn shadow_offset(&self) -> u64 {
        self.shadow_offset
    }

    pub fn is_allocated(&self) -> bool {
        self.is_allocated
    }

    pub fn deallocate(&mut self) {
        self.is_allocated = false;
    }

    /// Check if address is within valid range (excluding redzones)
    pub fn is_valid_access(&self, address: u64, access_size: usize) -> bool {
        if !self.is_allocated {
            return false;
        }

        let start = self.base_address + self.redzone_size as u64;
        let end = start + self.size as u64;

        // Check if access is within bounds
        if address < start || address + access_size as u64 > end {
            return false;
        }

        true
    }
}

/// Shadow memory entry for tracking memory state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowState {
    Free,
    Allocated,
    Freed,
    Redzone,
    Corrupted,
}

/// Address Sanitizer configuration
#[derive(Debug, Clone)]
pub struct AsanConfig {
    pub redzone_size: usize,
    pub shadow_scale: u8,
    pub shadow_offset: u64,
    pub quarantine_size: usize,
}

impl Default for AsanConfig {
    fn default() -> Self {
        Self {
            redzone_size: 16, // 16-byte redzones
            shadow_scale: 3,   // 1:8 shadow mapping
            shadow_offset: 0,
            quarantine_size: 1024 * 1024, // 1MB quarantine
        }
    }
}

impl From<usize> for AsanConfig {
    fn from(redzone_size: usize) -> Self {
        Self {
            redzone_size,
            ..Default::default()
        }
    }
}

/// Address Sanitizer for memory error detection
#[derive(Debug)]
pub struct AddressSanitizer {
    config: AsanConfig,
    regions: HashMap<u64, AsanRegion>,
    shadow_memory: HashMap<u64, ShadowState>,
    next_address: AtomicU64,
    quarantine: Vec<u64>,
    allocation_count: u64,
    deallocation_count: u64,
    error_count: u64,
}

impl AddressSanitizer {
    pub fn new(config: impl Into<AsanConfig>) -> Self {
        Self {
            config: config.into(),
            regions: HashMap::new(),
            shadow_memory: HashMap::new(),
            next_address: AtomicU64::new(0x10000),
            quarantine: Vec::new(),
            allocation_count: 0,
            deallocation_count: 0,
            error_count: 0,
        }
    }

    pub fn region_count(&self) -> usize {
        self.regions.values().filter(|r| r.is_allocated()).count()
    }

    /// Allocate a memory region with redzones
    pub fn allocate(&mut self, size: usize) -> Result<u64, String> {
        let redzone = self.config.redzone_size;
        let total_size = size + 2 * redzone;
        let base_address = self.next_address.fetch_add(total_size as u64, Ordering::SeqCst);

        let region = AsanRegion::new(base_address, size, redzone, self.config.shadow_offset);
        let ptr = base_address + redzone as u64;

        // Shadow left redzone
        for i in 0..redzone {
            self.shadow_memory.insert(base_address + i as u64, ShadowState::Redzone);
        }
        // Shadow accessible region
        for i in 0..size {
            self.shadow_memory.insert(ptr + i as u64, ShadowState::Allocated);
        }
        // Shadow right redzone
        let right_rz = ptr + size as u64;
        for i in 0..redzone {
            self.shadow_memory.insert(right_rz + i as u64, ShadowState::Redzone);
        }

        self.regions.insert(ptr, region);
        self.allocation_count += 1;

        Ok(ptr)
    }

    /// Free a memory region
    pub fn free(&mut self, ptr: u64) -> Result<(), String> {
        let region = self
            .regions
            .get_mut(&ptr)
            .ok_or_else(|| format!("Address 0x{:x} not allocated", ptr))?;

        if !region.is_allocated() {
            self.error_count += 1;
            return Err(format!("Double free detected at 0x{:x}", ptr));
        }

        region.deallocate();
        self.deallocation_count += 1;

        // Mark payload as Freed in shadow memory
        for i in 0..region.size() {
            self.shadow_memory.insert(ptr + i as u64, ShadowState::Freed);
        }

        // Add to quarantine
        if self.quarantine.len() < self.config.quarantine_size {
            self.quarantine.push(ptr);
        }

        Ok(())
    }

    /// Check if an address and range is valid
    pub fn is_valid_access(&self, ptr: u64, access_size: usize) -> bool {
        if access_size == 0 {
            return true;
        }

        for i in 0..access_size {
            let addr = ptr + i as u64;
            match self.shadow_memory.get(&addr) {
                Some(ShadowState::Allocated) => continue,
                _ => return false,
            }
        }

        true
    }

    /// Check access and return Result
    pub fn check_access(&mut self, ptr: u64, access_size: usize) -> Result<(), String> {
        if self.is_valid_access(ptr, access_size) {
            Ok(())
        } else {
            self.error_count += 1;
            Err(format!("Invalid memory access at 0x{:x} (size: {})", ptr, access_size))
        }
    }

    /// Get shadow memory state for an address
    pub fn get_shadow(&self, address: u64) -> Option<ShadowState> {
        self.shadow_memory.get(&address).copied()
    }

    /// Clear quarantine
    pub fn clear_quarantine(&mut self) {
        self.quarantine.clear();
    }

    /// Detect stack corruption using canary values
    pub fn check_stack_canary(&mut self, canary: u64, expected: u64) -> Result<(), String> {
        if canary != expected {
            self.error_count += 1;
            return Err(format!(
                "Stack canary corruption detected: expected 0x{:x}, got 0x{:x}",
                expected, canary
            ));
        }
        Ok(())
    }

    /// Detect buffer overflow by checking redzones
    pub fn detect_buffer_overflow(&mut self, address: u64, access_size: usize) -> bool {
        !self.is_valid_access(address, access_size)
    }

    /// Detect use-after-free
    pub fn detect_use_after_free(&mut self, address: u64) -> bool {
        self.get_shadow(address) == Some(ShadowState::Freed)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> AsanStatistics {
        AsanStatistics {
            allocation_count: self.allocation_count,
            deallocation_count: self.deallocation_count,
            error_count: self.error_count,
            active_regions: self.region_count(),
            quarantine_size: self.quarantine.len(),
            shadow_memory_size: self.shadow_memory.len(),
        }
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.allocation_count = 0;
        self.deallocation_count = 0;
        self.error_count = 0;
    }
}

/// ASan statistics
#[derive(Debug, Clone)]
pub struct AsanStatistics {
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub error_count: u64,
    pub active_regions: usize,
    pub quarantine_size: usize,
    pub shadow_memory_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();
        assert!(ptr > 0);
        assert_eq!(asan.region_count(), 1);
    }

    #[test]
    fn test_free() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();
        assert!(asan.free(ptr).is_ok());
        assert_eq!(asan.region_count(), 0);
    }

    #[test]
    fn test_valid_access() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();
        assert!(asan.is_valid_access(ptr, 50));
    }

    #[test]
    fn test_invalid_access_redzone() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();
        // Access left redzone
        assert!(!asan.is_valid_access(ptr - 1, 1));
        // Access right redzone
        assert!(!asan.is_valid_access(ptr + 100, 1));
    }

    #[test]
    fn test_shadow_memory() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();

        // Accessible region should be ShadowState::Allocated
        assert_eq!(asan.get_shadow(ptr), Some(ShadowState::Allocated));

        // Redzone should be ShadowState::Redzone
        assert_eq!(asan.get_shadow(ptr - 1), Some(ShadowState::Redzone));
    }

    #[test]
    fn test_use_after_free() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();
        asan.free(ptr).unwrap();

        // Access after free should be invalid
        assert!(!asan.is_valid_access(ptr, 1));
    }
}

/// Alias for backward compatibility
pub type MemoryRegion = AsanRegion;
