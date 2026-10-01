// Memory Protection Features for SigmaOS
// Implements ASLR (Address Space Layout Randomization) and stack canaries

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Memory protection modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProtectionMode {
    None = 0,
    StackCanaries = 1,
    ASLR = 2,
    Full = 3, // Both ASLR and stack canaries
}

/// Address Space Layout Randomization configuration
#[derive(Debug, Clone)]
pub struct AslrConfig {
    pub enabled: bool,
    pub randomization_bits: u8, // Number of bits to randomize
    pub page_alignment: u64,
}

impl Default for AslrConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            randomization_bits: 16, // 16-bit randomization by default
            page_alignment: 4096, // 4KB page alignment
        }
    }
}

/// Stack canary configuration
#[derive(Debug, Clone)]
pub struct StackCanaryConfig {
    pub enabled: bool,
    pub canary_value: u64,
    pub check_interval: u32,
}

impl Default for StackCanaryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            canary_value: 0xDEADBEEFCAFEBABE, // Default canary value
            check_interval: 1000, // Check every 1000 operations
        }
    }
}

/// Memory protection manager
pub struct MemoryProtectionManager {
    pub mode: MemoryProtectionMode,
    pub aslr_config: AslrConfig,
    pub stack_canary_config: StackCanaryConfig,
    pub aslr_offset: AtomicU64,
    pub stack_canary_map: HashMap<usize, u64>,
}

impl MemoryProtectionManager {
    pub fn new(mode: MemoryProtectionMode) -> Self {
        let aslr_config = AslrConfig::default();
        let stack_canary_config = StackCanaryConfig::default();

        // Generate random ASLR offset if enabled
        let aslr_offset = if mode == MemoryProtectionMode::ASLR || mode == MemoryProtectionMode::Full {
            AtomicU64::new(Self::generate_random_offset(&aslr_config))
        } else {
            AtomicU64::new(0)
        };

        Self {
            mode,
            aslr_config,
            stack_canary_config,
            aslr_offset,
            stack_canary_map: HashMap::new(),
        }
    }

    /// Generate a random offset for ASLR
    fn generate_random_offset(config: &AslrConfig) -> u64 {
        // In production, use a cryptographically secure RNG
        // For now, use a simple hash-based approach
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let mask = (1u64 << config.randomization_bits) - 1;
        let random = (timestamp as u64) & mask;

        // Align to a power-of-two boundary; saturating the configurable input
        // above avoids zero-alignment underflow and invalid shifts.
        random & !(alignment - 1)
        let mask = (1u64 << config.randomization_bits) - 1;
        let random = (timestamp as u64) & mask;

        // Align to page boundary
        (random + config.page_alignment) & !(config.page_alignment - 1)
    }

    /// Apply ASLR to an address
    pub fn apply_aslr(&self, base_address: u64) -> u64 {
        if self.mode == MemoryProtectionMode::ASLR || self.mode == MemoryProtectionMode::Full {
            let offset = self.aslr_offset.load(Ordering::SeqCst);
            base_address.wrapping_add(offset)
        } else {
            base_address
        }
    }

    /// Generate a stack canary for a stack frame
    pub fn generate_stack_canary(&mut self, stack_pointer: usize) -> u64 {
        if self.mode == MemoryProtectionMode::StackCanaries || self.mode == MemoryProtectionMode::Full {
            let canary = self.stack_canary_config.canary_value.wrapping_add(stack_pointer as u64);
            self.stack_canary_map.insert(stack_pointer, canary);
            canary
        } else {
            0
        }
    }

    /// Verify stack canary for a stack frame
    pub fn verify_stack_canary(&self, stack_pointer: usize, canary: u64) -> bool {
        if self.mode == MemoryProtectionMode::StackCanaries || self.mode == MemoryProtectionMode::Full {
            if let Some(&expected) = self.stack_canary_map.get(&stack_pointer) {
                expected == canary
            } else {
                false
            }
        } else {
            true
        }
    }

    /// Update ASLR offset (for dynamic re-randomization)
    pub fn re_randomize_aslr(&mut self) {
        if self.mode == MemoryProtectionMode::ASLR || self.mode == MemoryProtectionMode::Full {
            let new_offset = Self::generate_random_offset(&self.aslr_config);
            self.aslr_offset.store(new_offset, Ordering::SeqCst);
        }
    }

    /// Set memory protection mode
    pub fn set_mode(&mut self, mode: MemoryProtectionMode) {
        self.mode = mode;

        // Re-initialize ASLR offset if enabling ASLR
        if mode == MemoryProtectionMode::ASLR || mode == MemoryProtectionMode::Full {
            let new_offset = Self::generate_random_offset(&self.aslr_config);
            self.aslr_offset.store(new_offset, Ordering::SeqCst);
        }
    }

    /// Get current ASLR offset
    pub fn get_aslr_offset(&self) -> u64 {
        self.aslr_offset.load(Ordering::SeqCst)
    }

    /// Get stack canary count
    pub fn stack_canary_count(&self) -> usize {
        self.stack_canary_map.len()
    }

    /// Clear stack canary map
    pub fn clear_stack_canaries(&mut self) {
        self.stack_canary_map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aslr_offset_generation() {
        let config = AslrConfig::default();
        let offset = MemoryProtectionManager::generate_random_offset(&config);
        assert!(offset >= config.page_alignment);
        assert!(offset % config.page_alignment == 0);
    }

    #[test]
    fn test_aslr_application() {
        let manager = MemoryProtectionManager::new(MemoryProtectionMode::ASLR);
        let base = 0x10000000;
        let randomized = manager.apply_aslr(base);
        assert_ne!(base, randomized);
    }

    #[test]
    fn test_stack_canary_generation() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::StackCanaries);
        let sp = 0x7FFFFF000;
        let canary = manager.generate_stack_canary(sp);
        assert_ne!(canary, 0);
        assert_eq!(manager.stack_canary_count(), 1);
    }

    #[test]
    fn test_stack_canary_verification() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::StackCanaries);
        let sp = 0x7FFFFF000;
        let canary = manager.generate_stack_canary(sp);
        assert!(manager.verify_stack_canary(sp, canary));
        assert!(!manager.verify_stack_canary(sp, canary.wrapping_add(1)));
    }

    #[test]
    fn test_mode_switching() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::None);
        assert_eq!(manager.get_aslr_offset(), 0);

        manager.set_mode(MemoryProtectionMode::ASLR);
        assert_ne!(manager.get_aslr_offset(), 0);
    }

    #[test]
    fn test_re_randomization() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::ASLR);
        let initial_offset = manager.get_aslr_offset();
        manager.re_randomize_aslr();
        let new_offset = manager.get_aslr_offset();
        assert_ne!(initial_offset, new_offset);
    }
}
