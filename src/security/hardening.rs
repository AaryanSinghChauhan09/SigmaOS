// SigmaOS Security Hardening Module
// W^X enforcement, stack protection, and memory security
// Inspired by OpenBSD and Linux security mitigations

use alloc::vec::Vec;

/// Memory protection flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPermission {
    Read,
    Write,
    Execute,
    ReadWrite,
    ReadExecute,
    ReadWriteExecute,
}

/// Memory protection state
#[derive(Debug, Clone)]
pub struct MemoryProtectionState {
    pub enforce_wx: bool, // W^X enforcement
    pub stack_guard_pages: usize,
    pub heap_guard_pages: usize,
}

impl MemoryProtectionState {
    pub fn new() -> Self {
        Self {
            enforce_wx: true,
            stack_guard_pages: 1,
            heap_guard_pages: 1,
        }
    }

    /// Check if W^X violation would occur
    pub fn check_wx_violation(&self, current: MemoryPermission, requested: MemoryPermission) -> bool {
        if !self.enforce_wx {
            return false;
        }

        let has_write = matches!(current, MemoryPermission::Write | MemoryPermission::ReadWrite | MemoryPermission::ReadWriteExecute);
        let has_execute = matches!(requested, MemoryPermission::Execute | MemoryPermission::ReadExecute | MemoryPermission::ReadWriteExecute);
        
        has_write && has_execute
    }

    /// Apply W^X enforcement to permission request
    pub fn apply_wx(&self, current: MemoryPermission, requested: MemoryPermission) -> Result<MemoryPermission, &'static str> {
        if self.check_wx_violation(current, requested) {
            Err("W^X violation: cannot add execute permission to writable memory")
        } else {
            Ok(requested)
        }

        let mut expected_prev: u64 = 0x1337_C0DE_FA11_FACE;
        for i in 0..self.logs.len() {
            let log = &self.logs[i];
            if log.previous_hash != expected_prev {
                return false; // Chain broken! Tampering detected!
            }

            let payload: u64 = log.process_id
                ^ (log.permission as u64)
                ^ (if log.status_allowed { 1u64 } else { 0u64 });
            let calculated_hash = (expected_prev ^ payload).wrapping_mul(1099511628211_u64);

            if log.entry_hash != calculated_hash {
                return false; // Entry hash mismatch! Tampering detected!
            }

            expected_prev = log.entry_hash;
        }

        true
    }
}

impl Default for MemoryProtectionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack canary for overflow detection
#[derive(Debug, Clone)]
pub struct StackCanary {
    pub canary_value: u64,
    pub generation: u64,
}

impl StackCanary {
    pub fn new() -> Self {
        Self {
            canary_value: 0xDEADBEEFCAFEBABE,
            generation: 0,
        }
    }

    /// Generate new canary value
    pub fn generate(&mut self) -> u64 {
        self.generation = self.generation.wrapping_add(1);
        // In production, this should use CSPRNG
        self.canary_value = 0xDEADBEEFCAFEBABE ^ (self.generation * 0x9E3779B97F4A7C15);
        self.canary_value
    }

    /// Verify canary integrity
    pub fn verify(&self, expected: u64) -> bool {
        self.canary_value == expected
    }
}

impl Default for StackCanary {
    fn default() -> Self {
        Self::new()
    }
}

/// RELRO (Read-Only After Relocation) state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelroState {
    Disabled,
    Partial,
    Full,
}

/// Security hardening configuration
#[derive(Debug, Clone)]
pub struct SecurityHardeningConfig {
    pub memory_protection: MemoryProtectionState,
    pub stack_canary: StackCanary,
    pub relro_state: RelroState,
    pub pie_enabled: bool,
    pub aslr_enabled: bool,
}

impl SecurityHardeningConfig {
    pub fn new() -> Self {
        Self {
            memory_protection: MemoryProtectionState::new(),
            stack_canary: StackCanary::new(),
            relro_state: RelroState::Full,
            pie_enabled: true,
            aslr_enabled: true,
        }
    }

    /// Apply full hardening configuration
    pub fn apply_full_hardening(&mut self) {
        self.memory_protection.enforce_wx = true;
        self.memory_protection.stack_guard_pages = 1;
        self.memory_protection.heap_guard_pages = 1;
        self.relro_state = RelroState::Full;
        self.pie_enabled = true;
        self.aslr_enabled = true;
    }
}

impl Default for SecurityHardeningConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wx_enforcement() {
        let state = MemoryProtectionState::new();
        
        // Should allow read -> read/write
        assert!(!state.check_wx_violation(MemoryPermission::Read, MemoryPermission::ReadWrite));
        
        // Should reject write -> read/write/execute
        assert!(state.check_wx_violation(MemoryPermission::Write, MemoryPermission::ReadWriteExecute));
    }

    #[test]
    fn test_stack_canary() {
        let mut canary = StackCanary::new();
        let value1 = canary.generate();
        let value2 = canary.generate();
        
        assert_ne!(value1, value2);
        assert!(canary.verify(value2));
    }

    #[test]
    fn test_security_config() {
        let mut config = SecurityHardeningConfig::new();
        config.apply_full_hardening();
        
        assert!(config.memory_protection.enforce_wx);
        assert_eq!(config.relro_state, RelroState::Full);
        assert!(config.pie_enabled);
        assert!(config.aslr_enabled);
    }
}
