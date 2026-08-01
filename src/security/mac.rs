#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

/// Security context ID
pub type ContextID = usize;

/// Security level
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

/// Security domain
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityDomain {
    System = 0,
    User = 1,
    Network = 2,
    Storage = 3,
    Custom = 4,
}

/// Security context (OOP: Context object)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SecurityContext {
    pub id: ContextID,
    pub level: SecurityLevel,
    pub domain: SecurityDomain,
    pub capability: ContextCapability,
}

/// Context capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextCapability {
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
}

impl ContextCapability {
    pub fn new() -> Self {
        ContextCapability {
            can_read: false,
            can_write: false,
            can_execute: false,
        }
    }

    pub fn full() -> Self {
        ContextCapability {
            can_read: true,
            can_write: true,
            can_execute: true,
        }
    }
}

impl Default for ContextCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityContext {
    pub fn new(
        id: ContextID,
        level: SecurityLevel,
        domain: SecurityDomain,
        capability: ContextCapability,
    ) -> Self {
        SecurityContext {
            id,
            level,
            domain,
            capability,
        }
    }
}

/// MAC policy trait (OOP interface)
pub trait MACPolicy {
    /// Check if operation is allowed
    fn check(&self, context: &SecurityContext, operation: SecurityOperation) -> bool;
    /// Get policy info
    fn info(&self) -> PolicyInfo;
}

/// Security operation
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityOperation {
    Read = 0,
    Write = 1,
    Execute = 2,
    Create = 3,
    Delete = 4,
    Modify = 5,
}

/// Policy info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PolicyInfo {
    pub policy_type: PolicyType,
    pub strictness: SecurityLevel,
    pub capability: PolicyCapability,
}

impl PolicyInfo {
    pub fn new(policy_type: PolicyType) -> Self {
        PolicyInfo {
            policy_type,
            strictness: SecurityLevel::Medium,
            capability: PolicyCapability::new(),
        }
    }
}

/// Policy type
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    MLS = 0,  // Multi-Level Security
    Biba = 1, // Integrity
    RBAC = 2, // Role-Based
    Custom = 3,
}

/// Policy capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyCapability {
    pub can_enforce: bool,
    pub can_modify: bool,
}

impl PolicyCapability {
    pub fn new() -> Self {
        PolicyCapability {
            can_enforce: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        PolicyCapability {
            can_enforce: true,
            can_modify: true,
        }
    }
}

impl Default for PolicyCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// MLS policy (OOP: Concrete policy class)
pub struct MLSPolicy {
    pub policy_type: PolicyType,
    pub strictness: SecurityLevel,
    pub capability: PolicyCapability,
}

impl MLSPolicy {
    pub fn new(strictness: SecurityLevel, capability: PolicyCapability) -> Self {
        MLSPolicy {
            policy_type: PolicyType::MLS,
            strictness,
            capability,
        }
    }
}

impl MACPolicy for MLSPolicy {
    fn check(&self, context: &SecurityContext, operation: SecurityOperation) -> bool {
        if !self.capability.can_enforce {
            return true; // Allow if not enforcing
        }

        // MLS: Simple level check - context must meet or exceed policy strictness
        match operation {
            SecurityOperation::Read => context.level >= self.strictness,
            SecurityOperation::Write => {
                context.level >= self.strictness && context.capability.can_write
            }
            SecurityOperation::Execute => {
                context.level >= self.strictness && context.capability.can_execute
            }
            SecurityOperation::Create => context.level >= self.strictness,
            SecurityOperation::Delete => context.level >= SecurityLevel::High,
            SecurityOperation::Modify => {
                context.level >= self.strictness && context.capability.can_write
            }
        }
    }

    fn info(&self) -> PolicyInfo {
        PolicyInfo {
            policy_type: self.policy_type,
            strictness: self.strictness,
            capability: self.capability,
        }
    }
}

/// MAC engine trait (OOP interface)
pub trait MACEngine {
    /// Register policy
    fn register_policy(&mut self, policy: Box<dyn MACPolicy>) -> Result<usize, MACError>;
    /// Unregister policy
    fn unregister_policy(&mut self, id: usize) -> Result<(), MACError>;
    /// Create security context
    fn create_context(
        &mut self,
        level: SecurityLevel,
        domain: SecurityDomain,
        capability: ContextCapability,
    ) -> Result<ContextID, MACError>;
    /// Destroy security context
    fn destroy_context(&mut self, id: ContextID) -> Result<(), MACError>;
    /// Check access
    fn check_access(&self, context_id: ContextID, operation: SecurityOperation) -> bool;
    /// Get engine statistics
    fn stats(&self) -> MACStats;
}

/// MAC error types
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MACError {
    Success = 0,
    PolicyNotFound = 1,
    ContextNotFound = 2,
    PermissionDenied = 3,
    InvalidLevel = 4,
}

/// MAC statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MACStats {
    pub total_policies: usize,
    pub total_contexts: usize,
    pub access_checks: u64,
    pub access_denied: u64,
}

impl MACStats {
    pub fn new() -> Self {
        MACStats {
            total_policies: 0,
            total_contexts: 0,
            access_checks: 0,
            access_denied: 0,
        }
    }
}

impl Default for MACStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple MAC engine (OOP: Concrete engine class)
pub struct SimpleMACEngine {
    policies: Vec<Option<Box<dyn MACPolicy>>>,
    contexts: Vec<Option<SecurityContext>>,
    next_context_id: AtomicUsize,
    total_policies: AtomicUsize,
    total_contexts: AtomicUsize,
    access_checks: AtomicUsize,
    access_denied: AtomicUsize,
    capability: EngineCapability,
}

/// Engine capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineCapability {
    pub can_register_policies: bool,
    pub can_create_contexts: bool,
    pub can_enforce: bool,
}

impl EngineCapability {
    pub fn new() -> Self {
        EngineCapability {
            can_register_policies: false,
            can_create_contexts: false,
            can_enforce: false,
        }
    }

    pub fn full() -> Self {
        EngineCapability {
            can_register_policies: true,
            can_create_contexts: true,
            can_enforce: true,
        }
    }
}

impl SimpleMACEngine {
    pub fn new(capability: EngineCapability) -> Self {
        SimpleMACEngine {
            policies: Vec::new(),
            contexts: Vec::new(),
            next_context_id: AtomicUsize::new(1),
            total_policies: AtomicUsize::new(0),
            total_contexts: AtomicUsize::new(0),
            access_checks: AtomicUsize::new(0),
            access_denied: AtomicUsize::new(0),
            capability,
        }
    }

    fn get_context(&self, id: ContextID) -> Option<&SecurityContext> {
        for slot in &self.contexts {
            if let Some(ref context) = *slot {
                if context.id == id {
                    return Some(context);
                }
            }
        }
        None
    }
}

impl MACEngine for SimpleMACEngine {
    fn register_policy(&mut self, policy: Box<dyn MACPolicy>) -> Result<usize, MACError> {
        if !self.capability.can_register_policies {
            return Err(MACError::PermissionDenied);
        }

        let id = self.policies.len();
        self.policies.push(Some(policy));
        self.total_policies.fetch_add(1, Ordering::SeqCst);
        Ok(id)
    }

    fn unregister_policy(&mut self, id: usize) -> Result<(), MACError> {
        if !self.capability.can_register_policies {
            return Err(MACError::PermissionDenied);
        }

        if id < self.policies.len() {
            self.policies[id] = None;
            self.total_policies.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MACError::PolicyNotFound)
        }
    }

    fn create_context(
        &mut self,
        level: SecurityLevel,
        domain: SecurityDomain,
        capability: ContextCapability,
    ) -> Result<ContextID, MACError> {
        if !self.capability.can_create_contexts {
            return Err(MACError::PermissionDenied);
        }

        let id = self.next_context_id.fetch_add(1, Ordering::SeqCst);
        let context = SecurityContext::new(id, level, domain, capability);
        self.contexts.push(Some(context));
        self.total_contexts.fetch_add(1, Ordering::SeqCst);
        Ok(id)
    }

    fn destroy_context(&mut self, id: ContextID) -> Result<(), MACError> {
        if !self.capability.can_create_contexts {
            return Err(MACError::PermissionDenied);
        }

        let mut index = None;
        for (i, slot) in self.contexts.iter().enumerate() {
            if let Some(ref context) = *slot {
                if context.id == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.contexts[i] = None;
            self.total_contexts.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MACError::ContextNotFound)
        }
    }

    fn check_access(&self, context_id: ContextID, operation: SecurityOperation) -> bool {
        self.access_checks.fetch_add(1, Ordering::SeqCst);

        if !self.capability.can_enforce {
            return true;
        }

        if let Some(context) = self.get_context(context_id) {
            for slot in &self.policies {
                if let Some(ref policy) = *slot {
                    if !policy.check(context, operation) {
                        self.access_denied.fetch_add(1, Ordering::SeqCst);
                        return false;
                    }
                }
            }
            true
        } else {
            self.access_denied.fetch_add(1, Ordering::SeqCst);
            false
        }
    }

    fn stats(&self) -> MACStats {
        MACStats {
            total_policies: self.total_policies.load(Ordering::SeqCst),
            total_contexts: self.total_contexts.load(Ordering::SeqCst),
            access_checks: self.access_checks.load(Ordering::SeqCst) as u64,
            access_denied: self.access_denied.load(Ordering::SeqCst) as u64,
        }
    }
}

pub use MACPolicy as MacPolicy;
pub struct MacRule;
pub struct MacSecurity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_mac_engine() {
        let cap = EngineCapability::full();
        let mut engine = SimpleMACEngine::new(cap);
        let ctx_id = engine
            .create_context(
                SecurityLevel::Medium,
                SecurityDomain::System,
                ContextCapability::full(),
            )
            .unwrap();
        let policy_cap = PolicyCapability::full();
        let policy = MLSPolicy::new(SecurityLevel::Medium, policy_cap);
        engine.register_policy(Box::new(policy)).unwrap();

        assert!(engine.check_access(ctx_id, SecurityOperation::Read));
    }
}
