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
    SmepSmap = 4,
    DepNx = 8,
    RopProtection = 16,
    Full = 31, // All protections enabled
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

/// SMEP and SMAP CPU configuration flags
#[derive(Debug, Clone)]
pub struct SmepSmapConfig {
    pub smep_enabled: bool, // Supervisor Mode Execution Prevention
    pub smap_enabled: bool, // Supervisor Mode Access Prevention
    pub allow_user_access_toggle: bool, // Temporary STAC/CLAC user access toggle
}

impl Default for SmepSmapConfig {
    fn default() -> Self {
        Self {
            smep_enabled: true,
            smap_enabled: true,
            allow_user_access_toggle: false,
        }
    }
}

/// DEP / NX (No-Execute) page permission configuration
#[derive(Debug, Clone)]
pub struct DepNxConfig {
    pub enabled: bool,
    pub enforce_w_xor_x: bool, // W^X (Write XOR Execute)
}

impl Default for DepNxConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enforce_w_xor_x: true,
        }
    }
}

/// Memory protection manager
pub struct MemoryProtectionManager {
    pub mode: MemoryProtectionMode,
    pub aslr_config: AslrConfig,
    pub stack_canary_config: StackCanaryConfig,
    pub smep_smap_config: SmepSmapConfig,
    pub dep_nx_config: DepNxConfig,
    pub aslr_offset: AtomicU64,
    pub stack_canary_map: HashMap<usize, u64>,
    pub shadow_stack: HashMap<usize, u64>, // ROP shadow stack tracking return addresses
}

impl MemoryProtectionManager {
    pub fn new(mode: MemoryProtectionMode) -> Self {
        let aslr_config = AslrConfig::default();
        let stack_canary_config = StackCanaryConfig::default();
        let smep_smap_config = SmepSmapConfig::default();
        let dep_nx_config = DepNxConfig::default();
        
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
            smep_smap_config,
            dep_nx_config,
            aslr_offset,
            stack_canary_map: HashMap::new(),
            shadow_stack: HashMap::new(),
        }
    }

    /// SMEP check: Verify supervisor mode cannot execute user-space code page
    pub fn verify_smep(&self, target_address: u64, is_user_page: bool) -> Result<(), &'static str> {
        if (self.mode == MemoryProtectionMode::SmepSmap || self.mode == MemoryProtectionMode::Full)
            && self.smep_smap_config.smep_enabled
            && is_user_page
        {
            return Err("SMEP Violation: Attempted supervisor execution of user-space memory page");
        }
        Ok(())
    }

    /// SMAP check: Verify supervisor mode cannot access user-space data page without STAC
    pub fn verify_smap(&self, target_address: u64, is_user_page: bool, user_access_allowed: bool) -> Result<(), &'static str> {
        if (self.mode == MemoryProtectionMode::SmepSmap || self.mode == MemoryProtectionMode::Full)
            && self.smep_smap_config.smap_enabled
            && is_user_page
            && !user_access_allowed
        {
            return Err("SMAP Violation: Unsanitized supervisor access to user-space memory page");
        }
        Ok(())
    }

    /// DEP / NX check: Verify page permissions conform to No-Execute and W^X policies
    pub fn verify_dep_nx(&self, is_writable: bool, is_executable: bool) -> Result<(), &'static str> {
        if (self.mode == MemoryProtectionMode::DepNx || self.mode == MemoryProtectionMode::Full)
            && self.dep_nx_config.enabled
        {
            if self.dep_nx_config.enforce_w_xor_x && is_writable && is_executable {
                return Err("DEP/NX W^X Violation: Memory page cannot be both writable and executable");
            }
        }
        Ok(())
    }

    /// ROP Shadow Stack: Push return address onto shadow stack
    pub fn push_shadow_stack(&mut self, thread_id: usize, return_address: u64) {
        if self.mode == MemoryProtectionMode::RopProtection || self.mode == MemoryProtectionMode::Full {
            self.shadow_stack.insert(thread_id, return_address);
        }
    }

    /// ROP Shadow Stack: Verify return address against shadow stack entry
    pub fn verify_shadow_stack(&mut self, thread_id: usize, actual_return_address: u64) -> Result<(), &'static str> {
        if self.mode == MemoryProtectionMode::RopProtection || self.mode == MemoryProtectionMode::Full {
            if let Some(expected_return_address) = self.shadow_stack.remove(&thread_id) {
                if expected_return_address != actual_return_address {
                    return Err("ROP Protection Violation: Return address mismatch on shadow stack");
                }
            } else {
                return Err("ROP Protection Violation: Missing shadow stack frame");
            }
        }
        Ok(())
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

    #[test]
    fn test_smep_smap_verification() {
        let manager = MemoryProtectionManager::new(MemoryProtectionMode::SmepSmap);
        assert!(manager.verify_smep(0x100000, false).is_ok());
        assert!(manager.verify_smep(0x100000, true).is_err()); // User page execution blocked

        assert!(manager.verify_smap(0x100000, false, false).is_ok());
        assert!(manager.verify_smap(0x100000, true, false).is_err()); // User page access without STAC blocked
        assert!(manager.verify_smap(0x100000, true, true).is_ok());  // STAC enabled
    }

    #[test]
    fn test_dep_nx_verification() {
        let manager = MemoryProtectionManager::new(MemoryProtectionMode::DepNx);
        assert!(manager.verify_dep_nx(false, true).is_ok());  // RX page
        assert!(manager.verify_dep_nx(true, false).is_ok());  // RW page
        assert!(manager.verify_dep_nx(true, true).is_err());   // RWX W^X violation
    }

    #[test]
    fn test_rop_shadow_stack() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::RopProtection);
        let thread_id = 1;
        let ret_addr = 0x40001000;

        manager.push_shadow_stack(thread_id, ret_addr);
        assert!(manager.verify_shadow_stack(thread_id, ret_addr).is_ok());

        manager.push_shadow_stack(thread_id, ret_addr);
        assert!(manager.verify_shadow_stack(thread_id, 0x40002000).is_err());
    }
}
