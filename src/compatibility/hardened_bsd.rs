// SPDX-License-Identifier: GPL-2.0-or-later
// HardenedBSD security hardening integration for SigmaOS.
// Implements SafeStack, CFI policies, ASLR entropy, and W^X enforcement
// aligned with HardenedBSD's hardening feature set.

use alloc::vec::Vec;

/// SafeStack configuration — shadow stack for ROP mitigation.
#[derive(Debug, Clone)]
pub struct SafeStackConfig {
    pub enabled: bool,
    pub shadow_stack_size: usize,
}

impl SafeStackConfig {
    pub fn new(enabled: bool, shadow_stack_size: usize) -> Self {
        Self { enabled, shadow_stack_size }
    }
}

/// Control-Flow Integrity policy variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfiPolicy {
    Crosscall,
    Icall,
    Vcall,
}

/// ASLR entropy bits per memory region type.
#[derive(Debug, Clone)]
pub struct AslrEntropy {
    pub stack_bits: u8,
    pub heap_bits: u8,
    pub mmap_bits: u8,
}

impl AslrEntropy {
    pub fn new(stack_bits: u8, heap_bits: u8, mmap_bits: u8) -> Self {
        Self { stack_bits, heap_bits, mmap_bits }
    }

    /// Minimal recommended entropy for 64-bit systems.
    pub fn recommended() -> Self {
        Self::new(24, 24, 28)
    }
}

/// Enforces the W^X (Write XOR Execute) page-table policy.
pub struct WxEnforcer;

impl WxEnforcer {
    /// Returns true if the PTE is valid (not simultaneously writable AND executable).
    /// Bit 1 = Writable, Bit 2 = User-Executable (simplified model).
    pub fn check_pte_flags(pte: u64) -> bool {
        let writable   = (pte >> 1) & 1;
        let executable = (pte >> 2) & 1;
        !(writable == 1 && executable == 1)
    }
}

/// Aggregated HardenedBSD security profile.
#[derive(Debug, Clone)]
pub struct HardenedBsdProfile {
    pub safestack: SafeStackConfig,
    pub cfi_policies: Vec<CfiPolicy>,
    pub aslr: AslrEntropy,
    pub applied: bool,
}

impl HardenedBsdProfile {
    pub fn new() -> Self {
        Self {
            safestack: SafeStackConfig::new(true, 4096),
            cfi_policies: alloc::vec![CfiPolicy::Icall, CfiPolicy::Vcall],
            aslr: AslrEntropy::recommended(),
            applied: false,
        }
    }

    /// Apply the security profile (marks it as active).
    pub fn apply(&mut self) -> Result<(), &'static str> {
        if self.applied {
            return Err("Profile already applied");
        }
        self.applied = true;
        Ok(())
    }
}

impl Default for HardenedBsdProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wx_check_passes_for_valid_ptes() {
        // Readable only (bit0=1, bit1=0, bit2=0) — OK
        assert!(WxEnforcer::check_pte_flags(0b001));
        // Writable but not executable — OK
        assert!(WxEnforcer::check_pte_flags(0b011));
        // Executable but not writable — OK
        assert!(WxEnforcer::check_pte_flags(0b101));
        // Both writable AND executable — FAIL (W^X violation)
        assert!(!WxEnforcer::check_pte_flags(0b111));
    }

    #[test]
    fn test_profile_apply() {
        let mut profile = HardenedBsdProfile::new();
        assert!(!profile.applied);
        assert!(profile.apply().is_ok());
        assert!(profile.applied);
        // Double apply should fail
        assert!(profile.apply().is_err());
    }

    #[test]
    fn test_aslr_entropy_recommended() {
        let e = AslrEntropy::recommended();
        assert!(e.stack_bits >= 20);
        assert!(e.heap_bits >= 20);
        assert!(e.mmap_bits >= 20);
    }
}
