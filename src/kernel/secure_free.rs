// Secure Free Detection - Linux-style memory sanitization
// Prevents information disclosure by clearing freed memory

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizationLevel {
    None,       // No sanitization
    Partial,    // Zero only sensitive data
    Full,       // Zero entire allocation
    Pattern,    // Fill with detectable pattern
}

#[derive(Debug, Clone)]
pub struct AllocationRecord {
    pub address: usize,
    pub size: usize,
    pub is_sensitive: bool,
    pub freed: bool,
}

pub struct SecureFreeDetector {
    allocations: BTreeMap<usize, AllocationRecord>,
    sanitization_level: SanitizationLevel,
    detection_enabled: bool,
    pattern: u8, // Pattern for pattern-based sanitization
}

impl SecureFreeDetector {
    pub fn new() -> Self {
        Self {
            allocations: BTreeMap::new(),
            sanitization_level: SanitizationLevel::Full,
            detection_enabled: true,
            pattern: 0xDE,
        }
    }

    /// Register an allocation
    pub fn register_allocation(&mut self, address: usize, size: usize, is_sensitive: bool) {
        let record = AllocationRecord {
            address,
            size,
            is_sensitive,
            freed: false,
        };
        self.allocations.insert(address, record);
    }

    /// Securely free memory
    pub fn secure_free(&mut self, address: usize, ptr: *mut u8) -> Result<(), &'static str> {
        let record = self.allocations.get_mut(&address)
            .ok_or("Allocation not found")?;

        if record.freed {
            return Err("Double free detected");
        }

        // Sanitize based on level
        match self.sanitization_level {
            SanitizationLevel::None => {
                // No sanitization
            }
            SanitizationLevel::Partial => {
                if record.is_sensitive {
                    self.sanitize_memory(ptr, record.size, 0);
                }
            }
            SanitizationLevel::Full => {
                self.sanitize_memory(ptr, record.size, 0);
            }
            SanitizationLevel::Pattern => {
                self.fill_pattern(ptr, record.size);
            }
        }

        record.freed = true;
        Ok(())
    }

    /// Sanitize memory by zeroing
    fn sanitize_memory(&self, ptr: *mut u8, size: usize, value: u8) {
        unsafe {
            for i in 0..size {
                *ptr.add(i) = value;
            }
        }
    }

    /// Fill memory with detectable pattern
    fn fill_pattern(&self, ptr: *mut u8, size: usize) {
        unsafe {
            for i in 0..size {
                *ptr.add(i) = self.pattern;
            }
        }
    }

    /// Check for use-after-free
    pub fn check_use_after_free(&self, address: usize) -> bool {
        if let Some(record) = self.allocations.get(&address) {
            return record.freed;
        }
        false
    }

    /// Detect information disclosure (unfreed sensitive data)
    pub fn detect_information_disclosure(&self) -> Vec<usize> {
        self.allocations.iter()
            .filter(|(_, record)| record.is_sensitive && !record.freed)
            .map(|(addr, _)| *addr)
            .collect()
    }

    /// Set sanitization level
    pub fn set_sanitization_level(&mut self, level: SanitizationLevel) {
        self.sanitization_level = level;
    }

    /// Get sanitization level
    pub fn sanitization_level(&self) -> SanitizationLevel {
        self.sanitization_level
    }

    /// Enable/disable detection
    pub fn set_detection_enabled(&mut self, enabled: bool) {
        self.detection_enabled = enabled;
    }

    /// Set pattern for pattern-based sanitization
    pub fn set_pattern(&mut self, pattern: u8) {
        self.pattern = pattern;
    }

    /// Get allocation count
    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }

    /// Get freed allocation count
    pub fn freed_count(&self) -> usize {
        self.allocations.values().filter(|r| r.freed).count()
    }

    /// Get sensitive allocation count
    pub fn sensitive_count(&self) -> usize {
        self.allocations.values().filter(|r| r.is_sensitive).count()
    }

    /// Cleanup old allocation records
    pub fn cleanup_records(&mut self, max_age: u64) {
        // In a real implementation, this would remove old records
        // For now, we keep all records for demonstration
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SecureFreeStats {
        SecureFreeStats {
            total_allocations: self.allocation_count(),
            freed_allocations: self.freed_count(),
            sensitive_allocations: self.sensitive_count(),
            unfreed_sensitive: self.detect_information_disclosure().len(),
            sanitization_level: self.sanitization_level,
            detection_enabled: self.detection_enabled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecureFreeStats {
    pub total_allocations: usize,
    pub freed_allocations: usize,
    pub sensitive_allocations: usize,
    pub unfreed_sensitive: usize,
    pub sanitization_level: SanitizationLevel,
    pub detection_enabled: bool,
}

impl Default for SecureFreeDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_allocation() {
        let mut detector = SecureFreeDetector::new();
        
        detector.register_allocation(0x1000, 512, true);
        assert_eq!(detector.allocation_count(), 1);
    }

    #[test]
    fn test_secure_free() {
        let mut detector = SecureFreeDetector::new();
        
        detector.register_allocation(0x1000, 512, true);
        
        let mut buffer = [0u8; 512];
        let result = detector.secure_free(0x1000, buffer.as_mut_ptr());
        
        assert!(result.is_ok());
        assert_eq!(detector.freed_count(), 1);
    }

    #[test]
    fn test_double_free_detection() {
        let mut detector = SecureFreeDetector::new();
        
        detector.register_allocation(0x1000, 512, true);
        
        let mut buffer = [0u8; 512];
        detector.secure_free(0x1000, buffer.as_mut_ptr()).unwrap();
        
        let result = detector.secure_free(0x1000, buffer.as_mut_ptr());
        assert!(result.is_err());
    }

    #[test]
    fn test_use_after_free_detection() {
        let mut detector = SecureFreeDetector::new();
        
        detector.register_allocation(0x1000, 512, true);
        
        let mut buffer = [0u8; 512];
        detector.secure_free(0x1000, buffer.as_mut_ptr()).unwrap();
        
        assert!(detector.check_use_after_free(0x1000));
    }

    #[test]
    fn test_information_disclosure_detection() {
        let mut detector = SecureFreeDetector::new();
        
        detector.register_allocation(0x1000, 512, true);
        detector.register_allocation(0x2000, 512, false);
        
        let disclosures = detector.detect_information_disclosure();
        assert_eq!(disclosures.len(), 1);
        assert_eq!(disclosures[0], 0x1000);
    }

    #[test]
    fn test_sanitization_levels() {
        let mut detector = SecureFreeDetector::new();
        
        detector.set_sanitization_level(SanitizationLevel::None);
        assert_eq!(detector.sanitization_level(), SanitizationLevel::None);
        
        detector.set_sanitization_level(SanitizationLevel::Full);
        assert_eq!(detector.sanitization_level(), SanitizationLevel::Full);
    }

    #[test]
    fn test_pattern_sanitization() {
        let mut detector = SecureFreeDetector::new();
        
        detector.set_sanitization_level(SanitizationLevel::Pattern);
        detector.set_pattern(0xAB);
        
        detector.register_allocation(0x1000, 512, true);
        
        let mut buffer = [0u8; 512];
        detector.secure_free(0x1000, buffer.as_mut_ptr()).unwrap();
        
        // Check that buffer is filled with pattern
        assert_eq!(buffer[0], 0xAB);
    }

    #[test]
    fn test_statistics() {
        let mut detector = SecureFreeDetector::new();
        
        detector.register_allocation(0x1000, 512, true);
        detector.register_allocation(0x2000, 512, false);
        
        let mut buffer1 = [0u8; 512];
        detector.secure_free(0x1000, buffer1.as_mut_ptr()).unwrap();
        
        let stats = detector.get_statistics();
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.freed_allocations, 1);
        assert_eq!(stats.sensitive_allocations, 1);
    }
}
