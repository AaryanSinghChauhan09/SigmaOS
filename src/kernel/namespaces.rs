//! # Namespace Infrastructure Module
//!
//! This module provides the core namespace infrastructure for process isolation in SigmaOS.
//! It supports various namespace types including PID namespaces for process isolation.
//!
//! ## Architecture
//!
//! - **KernelNamespace trait**: Generic interface for all namespace types
//! - **NamespaceRegistry**: Central registry for namespace management
//! - **Specific namespace implementations**: PID, IPC, Network, etc. (PID implemented here)

use std::sync::atomic::{AtomicU64, Ordering};
use std::string::String;

/// Maximum number of namespaces in the system
pub const MAX_NAMESPACES: usize = 1024;

/// Maximum processes per PID namespace
pub const MAX_PIDS_PER_NAMESPACE: u32 = 32768;

/// Unique namespace identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespaceId(u64);

impl NamespaceId {
    /// Create a new namespace ID
    pub fn new(id: u64) -> Self {
        NamespaceId(id)
    }

    /// Get the raw ID value
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// Types of namespaces supported by SigmaOS (for trait-based namespace system)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelNamespaceType {
    Pid,
    Ipc,
    Network,
    Uts,  // UTS (hostname/domainname)
    User,
    Cgroup,
    Mount,
}

impl KernelNamespaceType {
    /// Get a string representation of the namespace type
    pub fn as_str(&self) -> &'static str {
        match self {
            KernelNamespaceType::Pid => "pid",
            KernelNamespaceType::Ipc => "ipc",
            KernelNamespaceType::Network => "network",
            KernelNamespaceType::Uts => "uts",
            KernelNamespaceType::User => "user",
            KernelNamespaceType::Cgroup => "cgroup",
            KernelNamespaceType::Mount => "mount",
        }
    }
}

/// Generic namespace trait defining the interface all namespaces must implement
pub trait KernelNamespace: Send + Sync {
    /// Get the unique namespace ID
    fn namespace_id(&self) -> NamespaceId;

    /// Get the namespace type
    fn namespace_type(&self) -> KernelNamespaceType;

    /// Get the reference count (how many processes use this namespace)
    fn ref_count(&self) -> u32;

    /// Increment reference count (when a process enters this namespace)
    fn increment_ref(&self);

    /// Decrement reference count (when a process leaves this namespace)
    fn decrement_ref(&self);

    /// Check if this namespace is equal to another
    fn equals(&self, other: &dyn KernelNamespace) -> bool {
        self.namespace_id() == other.namespace_id()
    }

    /// Get namespace metadata as a string
    fn metadata(&self) -> String;
}

/// Namespace creation configuration
#[derive(Debug, Clone)]
pub struct NamespaceConfig {
    pub namespace_type: KernelNamespaceType,
    pub inherit_from: Option<NamespaceId>,
}

impl NamespaceConfig {
    /// Create a new namespace configuration
    pub fn new(namespace_type: KernelNamespaceType) -> Self {
        NamespaceConfig {
            namespace_type,
            inherit_from: None,
        }
    }

    /// Create configuration that inherits from an existing namespace
    pub fn inherit(namespace_type: KernelNamespaceType, parent_id: NamespaceId) -> Self {
        NamespaceConfig {
            namespace_type,
            inherit_from: Some(parent_id),
        }
    }
}

/// Namespace error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceError {
    InvalidNamespaceId,
    NamespaceFull,
    NamespaceTypeNotSupported,
    ProcessNotInNamespace,
    InsufficientPermissions,
    AlreadyInNamespace,
}

impl NamespaceError {
    /// Get error message
    pub fn message(&self) -> &'static str {
        match self {
            NamespaceError::InvalidNamespaceId => "Invalid namespace ID",
            NamespaceError::NamespaceFull => "Namespace is full",
            NamespaceError::NamespaceTypeNotSupported => "Namespace type not supported",
            NamespaceError::ProcessNotInNamespace => "Process not in namespace",
            NamespaceError::InsufficientPermissions => "Insufficient permissions",
            NamespaceError::AlreadyInNamespace => "Process already in this namespace type",
        }
    }
}

/// Global namespace ID generator
pub struct NamespaceIdGenerator {
    next_id: AtomicU64,
}

impl NamespaceIdGenerator {
    /// Create a new ID generator
    pub const fn new() -> Self {
        NamespaceIdGenerator {
            next_id: AtomicU64::new(1),
        }
    }

    /// Generate the next namespace ID
    pub fn next(&self) -> NamespaceId {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        NamespaceId(id)
    }
}

/// Global namespace ID generator instance
static NAMESPACE_ID_GEN: NamespaceIdGenerator = NamespaceIdGenerator::new();

/// Get the next global namespace ID
pub fn next_namespace_id() -> NamespaceId {
    NAMESPACE_ID_GEN.next()
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_id_creation() {
        let id1 = NamespaceId::new(42);
        assert_eq!(id1.raw(), 42);

        let id2 = NamespaceId::new(42);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_namespace_id_generator() {
        let id1 = next_namespace_id();
        let id2 = next_namespace_id();

        assert!(id1.raw() < id2.raw());
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_namespace_type_as_str() {
        assert_eq!(KernelNamespaceType::Pid.as_str(), "pid");
        assert_eq!(KernelNamespaceType::Ipc.as_str(), "ipc");
        assert_eq!(KernelNamespaceType::Network.as_str(), "network");
    }

    #[test]
    fn test_namespace_config() {
        let config = NamespaceConfig::new(KernelNamespaceType::Pid);
        assert_eq!(config.namespace_type, KernelNamespaceType::Pid);
        assert_eq!(config.inherit_from, None);

        let parent_id = NamespaceId::new(1);
        let config = NamespaceConfig::inherit(KernelNamespaceType::Pid, parent_id);
        assert_eq!(config.inherit_from, Some(parent_id));
    }

    #[test]
    fn test_namespace_error_messages() {
        assert!(!NamespaceError::InvalidNamespaceId.message().is_empty());
        assert!(!NamespaceError::NamespaceFull.message().is_empty());
    }
}
