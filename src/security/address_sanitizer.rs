//! Address Sanitizer (ASan) Memory Error Detection
//!
//! ASan-inspired memory error detection with redzone-based overflow detection,
//! use-after-free tracking, stack corruption detection, and shadow memory mapping.

use std::collections::HashMap;

/// Memory region with redzone protection
#[derive(Debug, Clone)]
pub struct AsanRegion {
    pub base_address: u64,
    pub size: usize,
    pub redzone_size: usize,
    pub shadow_offset: u64,
    pub is_allocated: bool,
    pub start: u64,
    pub end: u64,
    pub canary: u64,
}

impl AsanRegion {
    pub fn new(base_address: u64, size: usize, redzone_size: usize, shadow_offset: u64) -> Self {
        Self {
            base_address,
            size,
            redzone_size,
            shadow_offset,
            is_allocated: true,
            start: base_address,
            end: base_address + size as u64 + 2 * redzone_size as u64,
            canary: 0,
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
        self.allocated = false;
    }

    /// Check if address is within valid range (excluding redzones)
    pub fn is_valid_access(&self, address: u64, access_size: usize) -> bool {
        if !self.is_allocated {
            return false;
        }

        let start = self.base_address + self.redzone_size as u64;
        let end = start + self.size as u64;

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
    shadow_memory: HashMap<u64, u8>,
    quarantine: Vec<u64>,
    allocation_count: u64,
    deallocation_count: u64,
    error_count: u64,
    pub redzone_size: usize,
    pub canary_seed: u64,
    pub next_region_id: u64,
    pub allocated: usize,
}

impl AddressSanitizer {
    pub fn new<C: Into<AsanConfig>>(config: C) -> Self {
        let cfg = config.into();
        let r_size = cfg.redzone_size;
        Self {
            config: cfg,
            regions: HashMap::new(),
            shadow_memory: HashMap::new(),
            quarantine: Vec::new(),
            allocation_count: 0,
            deallocation_count: 0,
            error_count: 0,
            redzone_size: r_size,
            canary_seed: 0xDEADBEEF,
            next_region_id: 1,
            allocated: 0,
        }
    }

    pub fn region_count(&self) -> usize {
        self.regions.values().filter(|r| r.is_allocated).count()
    }

    /// Allocate a memory region with redzones
    pub fn allocate(&mut self, size: usize) -> Result<u64, &'static str> {
        let region_id = self.next_region_id;
        self.next_region_id += 1;
        let total_size = size + 2 * self.redzone_size;

        let start = region_id * 10000 + 0x1000; // Simulated base address
        let end = start + total_size as u64;

        let canary = self.generate_canary(region_id);

        let mut region = AsanRegion::new(start, size, self.redzone_size, 0);
        region.canary = canary;

        for i in 0..self.redzone_size {
            self.shadow_memory.insert(start + i as u64, ShadowState::Redzone); // Left redzone
            self.shadow_memory.insert(end - i as u64 - 1, ShadowState::Redzone); // Right redzone
        }

        for i in self.redzone_size..(self.redzone_size + size) {
            self.shadow_memory.insert(start + i as u64, ShadowState::Allocated); // Accessible
        }

        let user_ptr = start + self.redzone_size as u64;
        self.regions.insert(user_ptr, region);
        self.allocation_count += 1;
        self.allocated += size;

        Ok(user_ptr) // Return pointer to data (after left redzone)
    }

    /// Free a memory region
    pub fn free(&mut self, ptr: u64) -> Result<(), &'static str> {
        let region = self.regions.get(&ptr).cloned();
        if let Some(mut region) = region {
            if !region.is_allocated {
                return Err("Double free or region already freed");
            }

            // Check canary
            if !self.check_canary(&region) {
                self.error_count += 1;
                return Err("Stack corruption detected: canary mismatch");
            }

            region.deallocate();
            if let Some(r) = self.regions.get_mut(&ptr) {
                r.deallocate();
            }

            // Mark region as freed in shadow memory
            for addr in region.start..region.end {
                self.shadow_memory.insert(addr, ShadowState::Freed); // Freed
            }

            self.deallocation_count += 1;
            if self.quarantine.len() < self.config.quarantine_size {
                self.quarantine.push(ptr);
            }
            region.deallocate();
            self.deallocation_count += 1;
            Ok(())
        } else {
            Err("Region not found")
        }
    }

    /// Check if an address is valid
    pub fn is_valid_access(&self, ptr: u64, size: usize) -> bool {
        for region in self.regions.values() {
            if !region.is_allocated {
                continue;
            }

            let data_start = region.start + self.redzone_size as u64;
            let data_end = data_start + region.size as u64;

            if ptr >= data_start && ptr + size as u64 <= data_end {
                for i in 0..size {
                    if let Some(&shadow) = self.shadow_memory.get(&(ptr + i as u64)) {
                        if shadow != ShadowState::Allocated {
                            return false; // Invalid access (redzone or freed)
                        }
                    }
                }
                return true;
            }
        }

        false
    }

    /// Check memory access
    pub fn check_access(&self, ptr: u64, size: usize) -> Result<(), &'static str> {
        if self.is_valid_access(ptr, size) {
            Ok(())
        } else {
            Err("Memory access violation")
        }
    }

    /// Generate canary
    fn generate_canary(&self, seed: u64) -> u64 {
        self.canary_seed
            .wrapping_add(seed)
            .wrapping_mul(0x9E3779B97F4A7C15)
    }

    fn check_canary(&self, region: &AsanRegion) -> bool {
        let expected = self.generate_canary(region.base_address / 10000);
        region.canary == expected || region.canary == 0
    }

        region.deallocate();
        self.deallocation_count += 1;

        // Add to quarantine
        if self.quarantine.len() < self.config.quarantine_size {
            self.quarantine.push(address);
        }

        Ok(())
    }

    /// Find region by pointer
    fn find_region_by_ptr(&self, ptr: u64) -> Result<u64, &'static str> {
        for (&id, region) in &self.regions {
            let data_start = region.start + self.redzone_size as u64;
            let data_end = region.end - self.redzone_size as u64;

            if ptr >= data_start && ptr < data_end {
                return Ok(id);
            }
        }

        Err("Pointer not within any allocated region")
    }

    pub fn get_shadow(&self, address: u64) -> Option<u8> {
        self.shadow_memory.get(&address).copied()
    }

    pub fn clear_quarantine(&mut self) {
        self.quarantine.clear();
    }

    pub fn check_stack_canary(&mut self, canary: u64, expected: u64) -> Result<(), String> {
        if canary != expected {
            self.error_count += 1;
            return Err(format!("Stack canary corruption detected: expected 0x{:x}, got 0x{:x}", expected, canary));
        }
        Ok(())
    }

    /// Detect buffer overflow by checking redzones
    pub fn detect_buffer_overflow(&mut self, address: u64, access_size: usize) -> bool {
        self.check_access(address, access_size).is_err()
    }

    /// Detect use-after-free
    pub fn detect_use_after_free(&mut self, address: u64) -> bool {
        if let Some(state) = self.get_shadow(address) {
            state == 0xFD
        } else {
            false
        }
    }

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

    pub fn reset_statistics(&mut self) {
        self.allocation_count = 0;
        self.deallocation_count = 0;
        self.error_count = 0;
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
        assert!(!asan.is_valid_access(ptr - 1, 1));
        assert!(!asan.is_valid_access(ptr + 100, 1));
    }

    #[test]
    fn test_shadow_memory() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();

        // Accessible region should be Allocated
        assert_eq!(asan.get_shadow(ptr), Some(ShadowState::Allocated));

        // Redzone should be Redzone
        assert_eq!(asan.get_shadow(ptr - 1), Some(ShadowState::Redzone));
    }

    #[test]
    fn test_use_after_free() {
        let mut asan = AddressSanitizer::new(16);

        let ptr = asan.allocate(100).unwrap();
        asan.free(ptr).unwrap();

        assert!(!asan.is_valid_access(ptr, 1));
    }
}

/// Alias for backward compatibility
pub type MemoryRegion = AsanRegion;
