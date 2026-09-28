// Address Sanitizer (ASan-inspired)
// Memory error detection for buffer overflows and use-after-free

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Memory region
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub allocated: bool,
    pub canary: u64,
}

/// Address sanitizer
pub struct AddressSanitizer {
    regions: HashMap<u64, MemoryRegion>,
    next_region_id: AtomicU64,
    shadow_memory: HashMap<u64, u8>, // Shadow memory for redzones
    redzone_size: usize,
    canary_seed: u64,
}

impl AddressSanitizer {
    pub fn new(redzone_size: usize) -> Self {
        Self {
            regions: HashMap::new(),
            next_region_id: AtomicU64::new(1),
            shadow_memory: HashMap::new(),
            redzone_size,
            canary_seed: 0xDEADBEEFCAFEBABE,
        }
    }

    /// Allocate a memory region with redzones
    pub fn allocate(&mut self, size: usize) -> Result<u64, &'static str> {
        let region_id = self.next_region_id.fetch_add(1, Ordering::SeqCst);
        let total_size = size + 2 * self.redzone_size;
        
        let start = region_id * 1000; // Simulated address
        let end = start + total_size as u64;
        
        let canary = self.generate_canary(region_id);
        
        let region = MemoryRegion {
            start,
            end,
            allocated: true,
            canary,
        };
        
        // Set redzones in shadow memory
        for i in 0..self.redzone_size {
            self.shadow_memory.insert(start + i as u64, 0xFA); // Left redzone
            self.shadow_memory.insert(end - i as u64 - 1, 0xFA); // Right redzone
        }
        
        // Set accessible region
        for i in self.redzone_size..(self.redzone_size + size) {
            self.shadow_memory.insert(start + i as u64, 0x00); // Accessible
        }
        
        self.regions.insert(region_id, region);
        
        Ok(start + self.redzone_size as u64) // Return pointer to data (after left redzone)
    }

    /// Free a memory region
    pub fn free(&mut self, ptr: u64) -> Result<(), &'static str> {
        let region_id = self.find_region_by_ptr(ptr)?;
        
        if let Some(region) = self.regions.remove(&region_id) {
            // Check canary
            if !self.check_canary(&region) {
                return Err("Stack corruption detected: canary mismatch");
            }
            
            // Mark region as freed in shadow memory
            for addr in region.start..region.end {
                self.shadow_memory.insert(addr, 0xFD); // Freed
            }
            
            Ok(())
        } else {
            Err("Region not found")
        }
    }

    /// Check if an address is valid
    pub fn is_valid_access(&self, ptr: u64, size: usize) -> bool {
        for region in self.regions.values() {
            if !region.allocated {
                continue;
            }
            
            let data_start = region.start + self.redzone_size as u64;
            let data_end = region.end - self.redzone_size as u64;
            
            if ptr >= data_start && ptr + size as u64 <= data_end {
                // Check shadow memory
                for i in 0..size {
                    if let Some(&shadow) = self.shadow_memory.get(&(ptr + i as u64)) {
                        if shadow != 0x00 {
                            return false; // Invalid access (redzone or freed)
                        }
                    }
                }
                return true;
            }
        }
        
        false
    }

    /// Generate canary
    fn generate_canary(&self, seed: u64) -> u64 {
        self.canary_seed.wrapping_add(seed).wrapping_mul(0x9E3779B97F4A7C15)
    }

    /// Check canary
    fn check_canary(&self, region: &MemoryRegion) -> bool {
        region.canary == self.generate_canary(region.start / 1000)
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
        Err("Region not found for pointer")
    }

    /// Get region count
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Get shadow memory value
    pub fn get_shadow(&self, addr: u64) -> Option<u8> {
        self.shadow_memory.get(&addr).copied()
    }
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
        
        // Accessible region should be 0x00
        assert_eq!(asan.get_shadow(ptr), Some(0x00));
        
        // Redzone should be 0xFA
        assert_eq!(asan.get_shadow(ptr - 1), Some(0xFA));
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
