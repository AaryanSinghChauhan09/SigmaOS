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
}

impl AsanRegion {
    pub fn new(base_address: u64, size: usize, redzone_size: usize, shadow_offset: u64) -> Self {
        Self {
            base_address,
            size,
            redzone_size,
            shadow_offset,
            is_allocated: true,
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

        let start = self.base_address;
        let end = self.base_address + self.size as u64;

        // Check if access is within bounds
        if address < start || address + access_size as u64 > end {
            return false;
        }

        true
    }
}

/// Shadow memory entry for tracking memory state
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn new(config: AsanConfig) -> Self {
        Self {
            config,
            regions: HashMap::new(),
            shadow_memory: HashMap::new(),
            next_address: AtomicU64::new(0x1000),
            quarantine: Vec::new(),
            allocation_count: 0,
            deallocation_count: 0,
            error_count: 0,
        }
    }

    pub fn new_default() -> Self {
        Self::new(AsanConfig::default())
    }

    /// Allocate memory with redzone protection
    pub fn allocate(&mut self, size: usize) -> Result<u64, String> {
        let base_address = self.next_address.fetch_add((size + self.config.redzone_size * 2) as u64, Ordering::SeqCst);
        let redzone_size = self.config.redzone_size;
        let shadow_offset = self.config.shadow_offset + base_address;

        // Create region with redzones
        let total_size = size + redzone_size * 2;
        let region = AsanRegion::new(base_address + redzone_size as u64, size, redzone_size, shadow_offset);

        // Initialize shadow memory
        self.initialize_shadow_memory(base_address, total_size, &region);

        self.regions.insert(base_address + redzone_size as u64, region);
        self.allocation_count += 1;

        Ok(base_address + redzone_size as u64)
    }

    fn initialize_shadow_memory(&mut self, base: u64, total_size: usize, region: &AsanRegion) {
        for i in 0..total_size {
            let addr = base + i as u64;
            let offset = i as u64;

            if offset < region.redzone_size() as u64 || offset >= (region.redzone_size() + region.size()) as u64 {
                self.shadow_memory.insert(addr, ShadowState::Redzone);
            } else {
                self.shadow_memory.insert(addr, ShadowState::Allocated);
            }
        }
    }

    /// Check valid access
    pub fn check_access(&mut self, address: u64, size: usize) -> Result<(), String> {
        // Find region containing this address
        let region = self.find_region(address)
            .ok_or_else(|| format!("Address 0x{:x} not in any allocated region", address))?;

        if !region.is_valid_access(address, size) {
            self.error_count += 1;
            return Err(format!("Invalid access to address 0x{:x} with size {}", address, size));
        }

        // Check shadow memory for corruption
        for i in 0..size {
            let addr = address + i as u64;
            if let Some(&state) = self.shadow_memory.get(&addr) {
                if state == ShadowState::Freed {
                    self.error_count += 1;
                    return Err(format!("Use-after-free detected at address 0x{:x}", addr));
                }
                if state == ShadowState::Redzone {
                    self.error_count += 1;
                    return Err(format!("Redzone overflow detected at address 0x{:x}", addr));
                }
                if state == ShadowState::Corrupted {
                    self.error_count += 1;
                    return Err(format!("Memory corruption detected at address 0x{:x}", addr));
                }
            }
        }

        Ok(())
    }

    fn find_region(&self, address: u64) -> Option<&AsanRegion> {
        self.regions.values().find(|r| {
            let start = r.base_address();
            let end = r.base_address() + (r.size() + r.redzone_size() * 2) as u64;
            address >= start && address < end
        })
    }

    /// Free memory
    pub fn free(&mut self, address: u64) -> Result<(), String> {
        let region = self.regions.get_mut(&address)
            .ok_or_else(|| format!("Address 0x{:x} not allocated", address))?;

        if !region.is_allocated() {
            return Err(format!("Address 0x{:x} already freed", address));
        }

        // Mark shadow memory as freed
        let start = region.base_address();
        let total_size = region.size() + region.redzone_size() * 2;
        for i in 0..total_size {
            let addr = start + i as u64;
            self.shadow_memory.insert(addr, ShadowState::Freed);
        }

        region.deallocate();
        self.deallocation_count += 1;

        // Add to quarantine
        if self.quarantine.len() < self.config.quarantine_size {
            self.quarantine.push(address);
        }

        Ok(())
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
            return Err(format!("Stack canary corruption detected: expected 0x{:x}, got 0x{:x}", expected, canary));
        }
        Ok(())
    }

    /// Detect buffer overflow by checking redzones
    pub fn detect_buffer_overflow(&mut self, address: u64, access_size: usize) -> bool {
        if let Err(_) = self.check_access(address, access_size) {
            true
        } else {
            false
        }
    }

    /// Detect use-after-free
    pub fn detect_use_after_free(&mut self, address: u64) -> bool {
        if let Some(state) = self.get_shadow(address) {
            state == ShadowState::Freed
        } else {
            false
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> AsanStatistics {
        AsanStatistics {
            allocation_count: self.allocation_count,
            deallocation_count: self.deallocation_count,
            error_count: self.error_count,
            active_regions: self.regions.values().filter(|r| r.is_allocated()).count(),
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
    fn test_asan_region_creation() {
        let region = AsanRegion::new(0x1000, 100, 16, 0);
        assert_eq!(region.base_address(), 0x1000);
        assert_eq!(region.size(), 100);
        assert_eq!(region.redzone_size(), 16);
        assert!(region.is_allocated());
    }

    #[test]
    fn test_asan_region_valid_access() {
        let region = AsanRegion::new(0x1000, 100, 16, 0);
        assert!(region.is_valid_access(0x1000, 50));
        assert!(region.is_valid_access(0x1000, 100));
        assert!(!region.is_valid_access(0x1000, 101));
        assert!(!region.is_valid_access(0x0FF0, 50));
    }

    #[test]
    fn test_asan_allocate() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(addr >= 0x1000);
        assert_eq!(asan.get_statistics().allocation_count, 1);
    }

    #[test]
    fn test_asan_valid_access() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(asan.check_access(addr, 50).is_ok());
        assert!(asan.check_access(addr, 100).is_ok());
    }

    #[test]
    fn test_asan_invalid_access() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(asan.check_access(addr, 101).is_err());
        assert!(asan.check_access(addr + 50, 60).is_err());
    }

    #[test]
    fn test_asan_free() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(asan.free(addr).is_ok());
        assert_eq!(asan.get_statistics().deallocation_count, 1);
    }

    #[test]
    fn test_asan_double_free() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(asan.free(addr).is_ok());
        assert!(asan.free(addr).is_err());
    }

    #[test]
    fn test_asan_use_after_free() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(asan.free(addr).is_ok());
        assert!(asan.detect_use_after_free(addr));
    }

    #[test]
    fn test_asan_buffer_overflow() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        assert!(asan.detect_buffer_overflow(addr, 101));
        assert!(!asan.detect_buffer_overflow(addr, 100));
    }

    #[test]
    fn test_asan_shadow_memory() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        let state = asan.get_shadow(addr);
        assert_eq!(state, Some(ShadowState::Allocated));
    }

    #[test]
    fn test_asan_stack_canary() {
        let mut asan = AddressSanitizer::new_default();
        let canary = 0xDEADBEEF;
        assert!(asan.check_stack_canary(canary, canary).is_ok());
        assert!(asan.check_stack_canary(canary, 0x12345678).is_err());
    }

    #[test]
    fn test_asan_statistics() {
        let mut asan = AddressSanitizer::new_default();
        asan.allocate(100).unwrap();
        asan.allocate(200).unwrap();
        let addr = asan.allocate(50).unwrap();
        asan.free(addr).unwrap();

        let stats = asan.get_statistics();
        assert_eq!(stats.allocation_count, 3);
        assert_eq!(stats.deallocation_count, 1);
        assert_eq!(stats.active_regions, 2);
    }

    #[test]
    fn test_asan_quarantine() {
        let mut asan = AddressSanitizer::new_default();
        let addr = asan.allocate(100).unwrap();
        asan.free(addr).unwrap();
        assert_eq!(asan.get_statistics().quarantine_size, 1);
        asan.clear_quarantine();
        assert_eq!(asan.get_statistics().quarantine_size, 0);
    }
}
