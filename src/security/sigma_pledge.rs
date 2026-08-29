//! # sigma_pledge - Syscall Filtering Framework
//!
//! Inspired by OpenBSD's pledge(), sigma_pledge allows processes to declare
//! exactly which syscall categories they need. All others are denied with EPERM.
//!
//! ## Usage
//!
//! ```rust,ignore
//! // A web server only needs networking and file I/O
//! sigma_pledge!(["inet", "rpath", "wpath", "proc"]);
//!
//! // A script interpreter needs only execution
//! sigma_pledge!(["exec", "rpath"]);
//! ```
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::vec;

extern crate alloc;

use sigma_types::{CapabilityToken, Result};

use alloc::collections::BTreeSet as HashSet;

/// Pledge namespaces representing different syscall categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PledgeNamespace {
    /// TCP/UDP socket creation
    Inet,
    /// Read filesystem paths
    Rpath,
    /// Write filesystem paths
    Wpath,
    /// Execute programs
    Exec,
    /// Process management
    Proc,
    /// AI inference via sigma-aid
    Ai,
    /// PQC operations
    Crypto,
    /// Terminal interaction
    Tty,
    /// DNS resolution
    Dns,
    /// sigma_unveil syscall
    Unveil,
    /// Audio operations
    Audio,
    /// Video operations
    Video,
    /// Network bind operations
    Bind,
}

/// Pledge promise - a set of allowed namespaces
#[derive(Debug, Clone)]
pub struct PledgePromise {
    allowed_namespaces: HashSet<PledgeNamespace>,
    is_pledged: bool,
}

impl PledgePromise {
    /// Create a new pledge promise with allowed namespaces
    pub fn new(namespaces: &[PledgeNamespace]) -> Self {
        PledgePromise {
            allowed_namespaces: namespaces.iter().cloned().collect(),
            is_pledged: false,
        }
    }

    /// Check if a namespace is allowed
    pub fn allows(&self, namespace: PledgeNamespace) -> bool {
        self.allowed_namespaces.contains(&namespace)
    }

    /// Mark as pledged (can only be done once)
    pub fn pledge(&mut self) -> Result<()> {
        if self.is_pledged {
            return Err("Already pledged");
        }
        self.is_pledged = true;
        Ok(())
    }

    /// Check if pledged
    pub fn is_pledged(&self) -> bool {
        self.is_pledged
    }

    /// Progressively narrow/restrict the allowed namespaces (cannot expand privileges)
    pub fn restrict(&mut self, new_namespaces: &[PledgeNamespace]) -> Result<()> {
        let requested_set: HashSet<PledgeNamespace> = new_namespaces.iter().cloned().collect();
        for ns in &requested_set {
            if !self.allowed_namespaces.contains(ns) {
                return Err("Namespace not permitted");
            }
        }
        self.allowed_namespaces = requested_set;
        Ok(())
    }
}

/// sigma_pledge macro for easy pledge declaration
#[macro_export]
macro_rules! sigma_pledge {
    ([$($namespace:ident),* $(,)?]) => {
        {
            let namespaces = vec![$($crate::security::sigma_pledge::PledgeNamespace::$namespace),*];
            let mut promise = $crate::security::sigma_pledge::PledgePromise::new(&namespaces);
            promise.pledge().expect("Failed to pledge");
            promise
        }
    };
}


/// Syscall filter that checks pledges
pub struct SyscallFilter {
    process_promises: alloc::collections::BTreeMap<u64, PledgePromise>,
}

impl SyscallFilter {
    /// Create a new syscall filter
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SyscallFilter {
            process_promises: alloc::collections::BTreeMap::new(),
        }
    }

    /// Register a process promise
    pub fn register_promise(&mut self, process_id: u64, promise: PledgePromise) {
        self.process_promises.insert(process_id, promise);
    }

    /// Check if a syscall is allowed for a process
    pub fn check_syscall(&self, process_id: u64, namespace: PledgeNamespace) -> Result<()> {
        match self.process_promises.get(&process_id) {
            Some(promise) => {
                if promise.allows(namespace) {
                    Ok(())
                } else {
                    Err("Syscall not pledged")
                }
            }
            None => {
                // Process hasn't pledged - deny by default
                Err("Process not pledged")
            }
        }
    }

    /// Remove process promise
    pub fn remove_promise(&mut self, process_id: u64) {
        self.process_promises.remove(&process_id);
    }
}

impl Default for SyscallFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_promise_creation() {
        let namespaces = vec![PledgeNamespace::Inet, PledgeNamespace::Rpath];
        let promise = PledgePromise::new(&namespaces);

        assert!(promise.allows(PledgeNamespace::Inet));
        assert!(promise.allows(PledgeNamespace::Rpath));
        assert!(!promise.allows(PledgeNamespace::Exec));
    }

    #[test]
    fn test_pledge_once() {
        let namespaces = vec![PledgeNamespace::Inet];
        let mut promise = PledgePromise::new(&namespaces);

        assert!(!promise.is_pledged());
        promise.pledge().unwrap();
        assert!(promise.is_pledged());

        // Second pledge should fail
        assert!(promise.pledge().is_err());
    }

    #[test]
    fn test_syscall_filter() {
        let mut filter = SyscallFilter::new();
        let namespaces = vec![PledgeNamespace::Inet, PledgeNamespace::Rpath];
        let promise = PledgePromise::new(&namespaces);

        filter.register_promise(1, promise);

        // Allowed syscall
        assert!(filter.check_syscall(1, PledgeNamespace::Inet).is_ok());

        // Denied syscall
        assert!(filter.check_syscall(1, PledgeNamespace::Exec).is_err());

        // Unregistered process
        assert!(filter.check_syscall(2, PledgeNamespace::Inet).is_err());
    }
}

// Placeholder types for compilation
mod sigma_types {
    pub type Result<T> = core::result::Result<T, &'static str>;

    #[derive(Debug, Clone)]
    pub struct CapabilityToken {
        pub id: u64,
    }
}
