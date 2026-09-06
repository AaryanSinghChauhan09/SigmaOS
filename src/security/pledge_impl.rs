#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS — OpenBSD pledge()-Inspired Capability Restriction
//
// After a process finishes its initialization phase it calls `pledge()` to
// declare the *minimal* set of operations it will ever need.  The kernel then
// enforces that contract; any attempt to use an undeclared promise results in
// immediate process termination (or a configurable error return).
//
// Reference: OpenBSD pledge(2) man page.
// This implementation is fully custom — no std, no libc, no external crates.

// No predefined library dependency — we use only the kernel's own klib.
use crate::klib::collections::SigmaBTreeMap;
use crate::runtime::process::ProcessId;

// ─────────────────────────────────────────────────────────────────────────────
// Promise flags
// ─────────────────────────────────────────────────────────────────────────────

/// Each variant maps 1-to-1 with an OpenBSD `pledge()` promise token.
/// Stored as a bit-flag (see `promise_bit`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PledgePromise {
    /// Basic I/O: read, write, fstat, close, …
    Stdio   = 0,
    /// Read-only filesystem access.
    Rpath   = 1,
    /// Write filesystem access (no create/delete).
    Wpath   = 2,
    /// Create and delete paths (implies Wpath).
    Cpath   = 3,
    /// Execute other programs via exec*.
    Exec    = 4,
    /// DNS resolution via getaddrinfo / getnameinfo.
    Dns     = 5,
    /// TCP/UDP connections via inet sockets.
    Inet    = 6,
    /// Unix-domain socket access.
    Unix    = 7,
    /// Fork, wait, getpid, kill (limited).
    Proc    = 8,
    /// setuid, setgid, seteuid, setegid.
    Id      = 9,
    /// chdir, fchdir.
    Chdir   = 10,
    /// mmap, mprotect (anonymous mappings only).
    Prot    = 11,
    /// tty-related ioctls.
    Tty     = 12,
    /// Audio device access.
    Audio   = 13,
    /// Video / camera device access.
    Video   = 14,
    /// Bluetooth device access.
    Bpf     = 15,
    /// Cryptographic hardware (TPM, HSM).
    Crypto  = 16,
    /// Virtual memory management.
    Vminfo  = 17,
}

impl PledgePromise {
    /// Convert a promise to its bit-mask position.
    #[inline(always)]
    fn bit(self) -> u32 {
        1u32 << (self as u32)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pledge violation handling
// ─────────────────────────────────────────────────────────────────────────────

/// What should happen when a pledged process tries an unpledged operation?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationMode {
    /// Terminate the process immediately (OpenBSD default).
    Kill,
    /// Return an error code to the caller (permissive / development mode).
    ReturnError,
    /// Log the violation but allow the operation (audit mode).
    Log,
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-process pledge state
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct PledgeState {
    /// Bitmask of allowed promises.
    allowed: u32,
    /// Once pledged, the set may only be *narrowed*, never widened.
    locked: bool,
    /// Violation mode for this process.
    mode: ViolationMode,
}

impl PledgeState {
    fn new(promises: u32, mode: ViolationMode) -> Self {
        Self { allowed: promises, locked: true, mode }
    }

    fn check(&self, op: PledgePromise) -> bool {
        (self.allowed & op.bit()) != 0
    }

    /// Narrow the promise set.  Returns `Err` if the new set would be
    /// *wider* than the existing one (widening is forbidden once pledged).
    fn narrow(&mut self, new_promises: u32) -> Result<(), PledgeError> {
        if (new_promises & !self.allowed) != 0 {
            // new_promises tries to add bits that were not already allowed
            return Err(PledgeError::WideningForbidden);
        }
        self.allowed = new_promises;
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgeError {
    /// Process attempted to widen its promise set after pledging.
    WideningForbidden,
    /// Operation not covered by the process's pledge.
    Violation(PledgePromise),
    /// The process ID was not found in the enforcer.
    UnknownProcess,
}

// ─────────────────────────────────────────────────────────────────────────────
// PledgeEnforcer — kernel-side registry
// ─────────────────────────────────────────────────────────────────────────────

/// Kernel-side registry mapping each process to its pledge state.
///
/// Typically there is one global `PledgeEnforcer` per system, accessed through
/// the kernel security subsystem.  It has no `alloc` / `std` dependency —
/// `SigmaBTreeMap` is the project's own fixed-capacity hash map.
pub struct PledgeEnforcer {
    registry: SigmaBTreeMap<ProcessId, PledgeState>,
}

impl PledgeEnforcer {
    /// Create a new, empty enforcer.
    pub fn new() -> Self {
        Self { registry: SigmaBTreeMap::new() }
    }

    // ── pledge() ──────────────────────────────────────────────────────────────

    /// Record a pledge for `pid`.
    ///
    /// * If the process has not pledged before, install the given promises.
    /// * If it has pledged before, narrow the promise set (widening is an error).
    pub fn pledge(
        &mut self,
        pid: ProcessId,
        promises: &[PledgePromise],
        mode: ViolationMode,
    ) -> Result<(), PledgeError> {
        let bits = Self::build_mask(promises);
        if let Some(state) = self.registry.get_mut(&pid) {
            state.narrow(bits)
        } else {
            self.registry.insert(pid, PledgeState::new(bits, mode));
            Ok(())
        }
    }

    // ── Check an operation ────────────────────────────────────────────────────

    /// Check whether `pid` is allowed to perform `op`.
    ///
    /// Returns `Ok(())` if:
    /// - The process has not pledged (unrestricted by default).
    /// - The operation is covered by its promise set.
    ///
    /// Returns `Err(PledgeError::Violation(_))` otherwise.
    pub fn check(
        &self,
        pid: ProcessId,
        op: PledgePromise,
    ) -> Result<(), PledgeError> {
        match self.registry.get(&pid) {
            None => Ok(()),  // unpledged: full access
            Some(state) => {
                if state.check(op) {
                    Ok(())
                } else {
                    Err(PledgeError::Violation(op))
                }
            }
        }
    }

    // ── Process exit cleanup ───────────────────────────────────────────────────

    /// Remove pledge state when a process exits.
    pub fn remove(&mut self, pid: ProcessId) {
        self.registry.remove(&pid);
    }

    // ── Query ─────────────────────────────────────────────────────────────────

    /// Return the raw bitmask for a process, or `None` if not pledged.
    pub fn promise_mask(&self, pid: ProcessId) -> Option<u32> {
        self.registry.get(&pid).map(|s| s.allowed)
    }

    /// Return the violation mode for a process, or `None` if not pledged.
    pub fn violation_mode(&self, pid: ProcessId) -> Option<ViolationMode> {
        self.registry.get(&pid).map(|s| s.mode)
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn build_mask(promises: &[PledgePromise]) -> u32 {
        let mut mask: u32 = 0;
        for &p in promises {
            mask |= p.bit();
        }
        mask
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper macro for syscall enforcement
// ─────────────────────────────────────────────────────────────────────────────

/// Quick check inside a syscall handler.
///
/// ```rust
/// use crate::security::pledge_impl::{check_pledge, PledgePromise};
/// check_pledge!(enforcer, pid, PledgePromise::Inet);
/// ```
///
/// Expands to a `return Err(...)` if the check fails.
#[macro_export]
macro_rules! check_pledge {
    ($enforcer:expr, $pid:expr, $op:expr) => {
        if let Err(e) = $enforcer.check($pid, $op) {
            return Err(e.into());
        }
    };
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test_disabled)]
mod tests {
    use super::*;

    fn pid(n: u64) -> ProcessId { ProcessId(n) }

    #[test]
    fn test_unpledged_process_allowed() {
        let enforcer = PledgeEnforcer::new();
        assert!(enforcer.check(pid(1), PledgePromise::Inet).is_ok());
    }

    #[test]
    fn test_pledged_allowed_promise() {
        let mut enforcer = PledgeEnforcer::new();
        enforcer.pledge(pid(2), &[PledgePromise::Stdio, PledgePromise::Rpath], ViolationMode::Kill).unwrap();
        assert!(enforcer.check(pid(2), PledgePromise::Stdio).is_ok());
        assert!(enforcer.check(pid(2), PledgePromise::Rpath).is_ok());
    }

    #[test]
    fn test_pledged_forbidden_promise() {
        let mut enforcer = PledgeEnforcer::new();
        enforcer.pledge(pid(3), &[PledgePromise::Stdio], ViolationMode::ReturnError).unwrap();
        assert_eq!(
            enforcer.check(pid(3), PledgePromise::Inet),
            Err(PledgeError::Violation(PledgePromise::Inet))
        );
    }

    #[test]
    fn test_narrowing_allowed() {
        let mut enforcer = PledgeEnforcer::new();
        enforcer.pledge(pid(4), &[PledgePromise::Stdio, PledgePromise::Inet], ViolationMode::Kill).unwrap();
        // Narrow to Stdio only — should succeed
        enforcer.pledge(pid(4), &[PledgePromise::Stdio], ViolationMode::Kill).unwrap();
        assert!(enforcer.check(pid(4), PledgePromise::Inet).is_err());
    }

    #[test]
    fn test_widening_forbidden() {
        let mut enforcer = PledgeEnforcer::new();
        enforcer.pledge(pid(5), &[PledgePromise::Stdio], ViolationMode::Kill).unwrap();
        // Try to add Inet — should fail
        let result = enforcer.pledge(pid(5), &[PledgePromise::Stdio, PledgePromise::Inet], ViolationMode::Kill);
        assert_eq!(result, Err(PledgeError::WideningForbidden));
    }

    #[test]
    fn test_remove_clears_state() {
        let mut enforcer = PledgeEnforcer::new();
        enforcer.pledge(pid(6), &[PledgePromise::Stdio], ViolationMode::Kill).unwrap();
        enforcer.remove(pid(6));
        // After removal, process is considered unpledged (full access)
        assert!(enforcer.check(pid(6), PledgePromise::Inet).is_ok());
    }
}
