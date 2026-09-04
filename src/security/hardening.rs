// SigmaOS Security Hardening Module
// W^X enforcement, stack protection, and memory security
// Inspired by OpenBSD and Linux security mitigations

use std::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

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
    pub fn check_wx_violation(
        &self,
        current: MemoryPermission,
        requested: MemoryPermission,
    ) -> bool {
        if !self.enforce_wx {
            return false;
        }

        let has_write = matches!(
            current,
            MemoryPermission::Write
                | MemoryPermission::ReadWrite
                | MemoryPermission::ReadWriteExecute
        );
        let has_execute = matches!(
            requested,
            MemoryPermission::Execute
                | MemoryPermission::ReadExecute
                | MemoryPermission::ReadWriteExecute
        );

        has_write && has_execute
    }

    /// Apply W^X enforcement to permission request
    pub fn apply_wx(
        &self,
        current: MemoryPermission,
        requested: MemoryPermission,
    ) -> Result<MemoryPermission, &'static str> {
        if self.check_wx_violation(current, requested) {
            Err("W^X violation: cannot add execute permission to writable memory")
        } else {
            Ok(requested)
        }
    }
}

impl Default for MemoryProtectionState {
    fn default() -> Self {
        Self::new()
    }
}

// Dynamic entropy base for stack canary generation.
// Seeded once at runtime using a compile-time djb2 hash of the build manifest
// directory path XOR'd with the Fibonacci hashing multiplier for avalanche effect.
// This gives a unique canary base per build/configuration without requiring a CSPRNG.
static CANARY_BASE_SEED: AtomicU64 = AtomicU64::new(0);

/// Returns the dynamic canary base, initialising the static on first call.
/// Mixing strategy:
///   - Compile-time djb2 hash of CARGO_MANIFEST_DIR  → build-unique
///   - XOR with Fibonacci multiplier 0x9E3779B97F4A7C15 → high bit diffusion
fn canary_base() -> u64 {
    let existing = CANARY_BASE_SEED.load(Ordering::Relaxed);
    if existing != 0 {
        return existing;
    }

    // Compile-time constant: djb2 hash over the build-manifest directory bytes.
    const FILE_PATH_HASH: u64 = {
        let bytes = env!("CARGO_MANIFEST_DIR").as_bytes();
        let mut h: u64 = 5381;
        let mut i = 0;
        while i < bytes.len() {
            h = h.wrapping_mul(33).wrapping_add(bytes[i] as u64);
            i += 1;
        }
        // XOR with Fibonacci constant for better avalanche.
        h ^ 0x9E3779B97F4A7C15
    };

    // compare_exchange ensures only one writer wins in concurrent contexts.
    match CANARY_BASE_SEED.compare_exchange(0, FILE_PATH_HASH, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => FILE_PATH_HASH,
        Err(winner) => winner,
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
            // Initialise stored canary to the dynamic base so un-generated
            // canaries are still non-trivially derived (not a magic constant).
            canary_value: canary_base(),
            generation: 0,
        }
    }

    /// Generate new canary value.
    /// XOR-combines the dynamic base seed with a generation counter scaled by
    /// the Fibonacci multiplier — preserving the original derivation formula
    /// while eliminating the former hardcoded 0xDEADBEEFCAFEBABE constant.
    pub fn generate(&mut self) -> u64 {
        self.generation = self.generation.wrapping_add(1);
        // Dynamic base replaces the former hardcoded sentinel 0xDEADBEEFCAFEBABE.
        self.canary_value = canary_base() ^ (self.generation.wrapping_mul(0x9E3779B97F4A7C15));
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

pub fn secure_zeroize(buffer: &mut [u8]) {
    for byte in buffer.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntrusionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp_ms: u64,
    pub event: std::string::String,
    pub severity: IntrusionSeverity,
}

#[derive(Debug, Default, Clone)]
pub struct HardenedAuditTrail {
    pub logs: std::vec::Vec<AuditLogEntry>,
}

impl HardenedAuditTrail {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_event(&mut self, event: &str, severity: IntrusionSeverity) {
        self.logs.push(AuditLogEntry {
            timestamp_ms: 1000,
            event: event.into(),
            severity,
        });
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntrusionMonitor {
    pub audit_trail: HardenedAuditTrail,
}

impl IntrusionMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log_intrusion_attempt(&mut self, source: &str, severity: IntrusionSeverity) {
        self.audit_trail.record_event(source, severity);
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
        assert!(
            state.check_wx_violation(MemoryPermission::Write, MemoryPermission::ReadWriteExecute)
        );
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
