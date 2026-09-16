// SigmaOS Separation of Policy and Mechanism Operating System Architecture
// Implements core principles of OS design: Separation of Policy and Mechanism,
// Protection & Isolation, Optimization for the Common Case, Privilege Levels, and Interrupt Handling.

use crate::security::CapabilityToken;
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    UserMode = 0,
    KernelMode = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyError {
    Success = 0,
    QuotaExceeded = 1,
    PermissionDenied = 2,
    InvalidPrivilege = 3,
}

/// 1. Separation of Policy and Mechanism (Mechanism)
/// ResourceBroker manages raw system allocations (how the allocation is made).
pub struct ResourceBroker {
    allocated_units: AtomicUsize,
}

impl ResourceBroker {
    pub const fn new() -> Self {
        Self {
            allocated_units: AtomicUsize::new(0),
        }
    }

    /// Mechanism: Performs raw unit allocation
    pub fn allocate_raw(&self, units: usize) {
        self.allocated_units.fetch_add(units, Ordering::SeqCst);
    }

    /// Mechanism: Performs raw unit deallocation
    pub fn release_raw(&self, units: usize) {
        self.allocated_units.fetch_sub(units, Ordering::SeqCst);
    }

    pub fn current_usage(&self) -> usize {
        self.allocated_units.load(Ordering::SeqCst)
    }
}

/// 1. Separation of Policy and Mechanism (Policy)
/// PolicyManager defines allocation limits, quotas, and permissions (who gets what and when).
pub struct PolicyManager {
    max_quota_limit: usize,
    required_token_bit: u64,
}

impl PolicyManager {
    pub const fn new(limit: usize, required_bit: u64) -> Self {
        Self {
            max_quota_limit: limit,
            required_token_bit: required_bit,
        }
    }

    /// Policy: Validates if a resource request conforms to current policies
    pub fn enforce_allocation_policy(
        &self,
        broker: &ResourceBroker,
        request_units: usize,
        token: &CapabilityToken,
    ) -> Result<(), PolicyError> {
        // Policy A: Quota Limit
        if broker.current_usage() + request_units > self.max_quota_limit {
            return Err(PolicyError::QuotaExceeded);
        }

        // Policy B: Privilege Verification via CapabilityToken
        if (token.bits() & self.required_token_bit) == 0 {
            return Err(PolicyError::PermissionDenied);
        }

        Ok(())
    }
}

/// 2. Protection and Isolation (ProtectionDomain)
/// Encapsulates compartmentalized address spaces and capabilities.
pub struct ProtectionDomain {
    pub id: usize,
    pub start_address: u64,
    pub end_address: u64,
    pub capabilities: CapabilityToken,
}

impl ProtectionDomain {
    pub const fn new(id: usize, start: u64, end: u64, caps: CapabilityToken) -> Self {
        Self {
            id,
            start_address: start,
            end_address: end,
            capabilities: caps,
        }
    }

    /// Checks if a memory access falls within this isolated protection domain
    pub fn is_memory_isolated(&self, target_addr: u64) -> bool {
        target_addr >= self.start_address && target_addr <= self.end_address
    }
}

/// 3. Interrupt Handling & Privilege Levels (InterruptMechanism)
/// Routes interrupts and validates execution privilege modes.
pub struct InterruptMechanism {
    registered_handlers: [Option<usize>; 256], // Mock function pointer offsets
}

impl InterruptMechanism {
    pub const fn new() -> Self {
        Self {
            registered_handlers: [None; 256],
        }
    }

    pub fn register_handler(&mut self, vector: u8, handler_offset: usize) {
        self.registered_handlers[vector as usize] = Some(handler_offset);
    }

    /// Handles interrupt and enforces privilege transition
    pub fn dispatch_interrupt(
        &self,
        vector: u8,
        caller_privilege: PrivilegeLevel,
    ) -> Result<PrivilegeLevel, PolicyError> {
        if self.registered_handlers[vector as usize].is_none() {
            return Err(PolicyError::PermissionDenied);
        }

        // Enforce transition to KernelMode upon interrupt handling
        if caller_privilege == PrivilegeLevel::UserMode {
            Ok(PrivilegeLevel::KernelMode)
        } else {
            Ok(PrivilegeLevel::KernelMode)
        }
    }
}

/// 4. Optimization for the Common Case (FastPathIpc)
/// Standard IPC uses heavy table lookup routing; Common case local IPC uses a lock-free fast-path.
pub struct FastPathIpc {
    buffered_message: AtomicUsize,
}

impl FastPathIpc {
    pub const fn new() -> Self {
        Self {
            buffered_message: AtomicUsize::new(0),
        }
    }

    /// Fast-Path Local IPC: Directly exchanges a message unit in a lock-free, zero-copy loop
    pub fn fast_exchange(&self, message_val: usize) -> usize {
        // Exchange message in a single atomic cycle (optimized for the common case local IPC)
        self.buffered_message.swap(message_val, Ordering::SeqCst)
    }

    pub fn current_message(&self) -> usize {
        self.buffered_message.load(Ordering::SeqCst)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_mechanism_separation() {
        let broker = ResourceBroker::new();
        // Policy: Max quota of 100 units, requires capability bit 0x08
        let policy = PolicyManager::new(100, 0x08);

        let valid_token = CapabilityToken::from_bits(0x08);
        let invalid_token = CapabilityToken::from_bits(0x02);

        // Try allocating 50 units with invalid token - should fail
        assert_eq!(
            policy.enforce_allocation_policy(&broker, 50, &invalid_token),
            Err(PolicyError::PermissionDenied)
        );

        // Enforce with valid token - should succeed
        assert!(policy
            .enforce_allocation_policy(&broker, 50, &valid_token)
            .is_ok());

        // Mechanism: Apply raw allocation
        broker.allocate_raw(50);
        assert_eq!(broker.current_usage(), 50);

        // Try allocating another 60 units (total 110) - should fail quota
        assert_eq!(
            policy.enforce_allocation_policy(&broker, 60, &valid_token),
            Err(PolicyError::QuotaExceeded)
        );
    }

    #[test]
    fn test_protection_and_isolation() {
        let caps = CapabilityToken::from_bits(0x04);
        let domain = ProtectionDomain::new(1, 0x1000, 0x2000, caps);

        assert!(domain.is_memory_isolated(0x1500));
        assert!(!domain.is_memory_isolated(0x3000));
    }

    #[test]
    fn test_interrupts_and_privileges() {
        let mut interrupts = InterruptMechanism::new();
        interrupts.register_handler(0x80, 0xAA00); // System call interrupt

        let current_mode = PrivilegeLevel::UserMode;
        let new_mode = interrupts.dispatch_interrupt(0x80, current_mode).unwrap();

        assert_eq!(new_mode, PrivilegeLevel::KernelMode);
    }

    #[test]
    fn test_fast_path_optimization() {
        let ipc = FastPathIpc::new();
        let prev = ipc.fast_exchange(42);
        assert_eq!(prev, 0);
        assert_eq!(ipc.current_message(), 42);

        let prev_ex = ipc.fast_exchange(99);
        assert_eq!(prev_ex, 42);
        assert_eq!(ipc.current_message(), 99);
    }
}

pub trait KernelMechanism: Sync {
    fn name(&self) -> &'static str;
    fn execute(&self);
}

pub trait KernelPolicy: Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self) -> bool;
}

pub trait SovereignMechanism: Sync {
    fn name(&self) -> &'static str;
    fn activate(&self);
}

pub struct AdaptivePolicy {
    pub mechanism: &'static dyn KernelMechanism,
    pub policy: &'static dyn KernelPolicy,
}

impl AdaptivePolicy {
    pub const fn new(
        mechanism: &'static dyn KernelMechanism,
        policy: &'static dyn KernelPolicy,
    ) -> Self {
        Self { mechanism, policy }
    }

    pub fn apply(&self) {
        if self.policy.evaluate() {
            self.mechanism.execute();
        }
    }
}

pub struct PolicyMechanismCoordinator {
    pub policies: Vec<&'static dyn KernelPolicy>,
    pub mechanisms: Vec<&'static dyn KernelMechanism>,
}

impl PolicyMechanismCoordinator {
    pub const fn new() -> Self {
        Self {
            policies: Vec::new(),
            mechanisms: Vec::new(),
        }
    }

    pub fn register_policy(&mut self, policy: &'static dyn KernelPolicy) {
        self.policies.push(policy);
    }

    pub fn register_mechanism(&mut self, mechanism: &'static dyn KernelMechanism) {
        self.mechanisms.push(mechanism);
    }

    pub fn coordinate(&self) {
        for mechanism in &self.mechanisms {
            mechanism.execute();
        }
    }
}
