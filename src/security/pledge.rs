use crate::klib::BTreeMap;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

use crate::security::capability::{CapabilityGate, CapabilityToken, Permission};

use core::sync::atomic::{AtomicBool, Ordering};

/// Per-thread sub-pledge context enabling fine-grained worker thread isolation
#[derive(Debug, Clone)]
pub struct ThreadSubPledgeContext {
    pub tid: u64,
    pub sub_promise: PledgePromise,
}

/// Pledge promise representing process permissions
#[derive(Debug)]
pub struct PledgePromise {
    /// Allowed permissions
    permissions: Vec<Permission>,
    /// Whether pledge is active
    active: AtomicBool,
}

impl Clone for PledgePromise {
    fn clone(&self) -> Self {
        Self {
            permissions: self.permissions.clone(),
            active: AtomicBool::new(self.active.load(Ordering::SeqCst)),
        }
    }
}

impl PledgePromise {
    /// Create new pledge promise with specified permissions
    pub fn new(permissions: Vec<Permission>) -> Self {
        Self {
            permissions,
            active: AtomicBool::new(false),
        }
    }

    /// Activate the pledge (can only be done once)
    pub fn activate(&self) -> Result<(), PledgeError> {
        if self.active.load(Ordering::SeqCst) {
            return Err(PledgeError::AlreadyActive);
        }
        self.active.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Check if permission is allowed
    pub fn allows(&self, permission: Permission) -> bool {
        if !self.active.load(Ordering::SeqCst) {
            return true; // Not activated yet, allow everything
        }
        self.permissions.contains(&permission)
    }

    /// Get all allowed permissions
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }
}

/// Pledge errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgeError {
    AlreadyActive,
    InvalidPermission,
    Violation,
}

#[derive(Debug, Clone)]
pub struct UnveilEntry {
    pub path: String,
    pub permissions: String, // e.g., "r", "rw", "rx"
}

/// Process pledge manager supporting process-level and thread-level sub-pledges
pub struct PledgeManager {
    /// Current pledge promise
    pledge: Option<PledgePromise>,
    /// Pre-configured pledge promise for exec child process
    exec_pledge: Option<PledgePromise>,
    /// Capability gate for validation
    gate: CapabilityGate,
    /// Unveiled paths for filesystem sandboxing
    unveiled_paths: Vec<UnveilEntry>,
    /// Thread-specific sub-pledges
    thread_sub_pledges: BTreeMap<u64, ThreadSubPledgeContext>,
}

impl PledgeManager {
    /// Create new pledge manager
    pub fn new() -> Self {
        Self {
            pledge: None,
            exec_pledge: None,
            gate: CapabilityGate::new(),
            unveiled_paths: Vec::new(),
            thread_sub_pledges: BTreeMap::new(),
        }
    }

    /// Assign a sub-pledge promise to a worker thread (must be a subset of main process pledge)
    pub fn sub_pledge_thread(
        &mut self,
        tid: u64,
        sub_promise: PledgePromise,
    ) -> Result<(), PledgeError> {
        if let Some(ref main_pledge) = self.pledge {
            // Verify that thread sub-pledge does not exceed process pledge
            for perm in sub_promise.permissions() {
                if !main_pledge.allows(*perm) {
                    return Err(PledgeError::Violation);
                }
            }
        }
        sub_promise.activate()?;
        self.thread_sub_pledges
            .insert(tid, ThreadSubPledgeContext { tid, sub_promise });
        Ok(())
    }

    /// Unveil filesystem paths to restrict access (sigma_unveil)
    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), PledgeError> {
        self.unveiled_paths.push(UnveilEntry {
            path: path.to_string(),
            permissions: permissions.to_string(),
        });
        Ok(())
    }

    /// Validate path access against unveil permissions.
    ///
    /// Security hardening applied:
    /// - Rejects null bytes (CVE-class: null-byte injection)
    /// - Rejects `..` segments (directory traversal)
    /// - Rejects paths with encoded traversal sequences (`%2e%2e`, `%2F`)
    /// - Longest-prefix match with strict boundary check
    pub fn validate_unveil_access(&self, path: &str, requested_perm: char) -> bool {
        if self.unveiled_paths.is_empty() {
            return true; // No unveil restrictions — allow all.
        }

        // Reject null bytes — they can truncate paths in C-ABI syscall interop.
        if path.as_bytes().contains(&0u8) {
            return false;
        }

        // Reject URL-encoded traversal patterns (common in HTTP-facing code paths).
        let lower = {
            let mut buf = [0u8; 512];
            let bytes = path.as_bytes();
            let copy_len = bytes.len().min(buf.len());
            buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            // Lowercase the copy without alloc
            for b in &mut buf[..copy_len] {
                if *b >= b'A' && *b <= b'Z' {
                    *b += 32;
                }
            }
            buf
        };
        let lower_path = core::str::from_utf8(&lower[..path.len().min(512)]).unwrap_or("");
        if lower_path.contains("%2e%2e") || lower_path.contains("%2f") || lower_path.contains("%5c")
        {
            return false;
        }

        // Reject `..` segments — directory traversal mitigation.
        for segment in path.split(|c| c == '/' || c == '\\') {
            if segment == ".." || segment == "." {
                return false;
            }
        }

        // Find the most specific match (longest prefix match)
        let mut best_match: Option<&UnveilEntry> = None;
        for entry in &self.unveiled_paths {
            if path.starts_with(&entry.path) {
                let e_len = entry.path.len();
                // Ensure it is a valid boundary match (exact match, or followed by a separator, or suffix has slash)
                let is_boundary = path.len() == e_len
                    || path.as_bytes().get(e_len).copied() == Some(b'/')
                    || path.as_bytes().get(e_len).copied() == Some(b'\\')
                    || entry.path.ends_with('/')
                    || entry.path.ends_with('\\');

                if is_boundary {
                    match best_match {
                        None => best_match = Some(entry),
                        Some(best) => {
                            if entry.path.len() > best.path.len() {
                                best_match = Some(entry);
                            }
                        }
                    }
                }
            }
        }

        if let Some(entry) = best_match {
            entry.permissions.contains(requested_perm)
        } else {
            false // Not in unveiled paths, block access!
        }
    }

    /// Pre-configures execpledge promise for process child execution
    pub fn execpledge(&mut self, promise: PledgePromise) -> Result<(), PledgeError> {
        if self.exec_pledge.is_some() {
            return Err(PledgeError::AlreadyActive);
        }
        self.exec_pledge = Some(promise);
        Ok(())
    }

    /// Retrieves active exec_pledge promise if configured
    pub fn active_execpledge(&self) -> Option<&PledgePromise> {
        self.exec_pledge.as_ref()
    }

    /// Set pledge promise for process
    pub fn pledge(&mut self, promise: PledgePromise) -> Result<(), PledgeError> {
        if self.pledge.is_some() {
            return Err(PledgeError::AlreadyActive);
        }
        promise.activate()?;
        self.pledge = Some(promise);

        // Update capability gate based on pledge
        if let Some(ref pledge) = self.pledge {
            let mut token = CapabilityToken::new();
            for &perm in pledge.permissions() {
                match perm {
                    Permission::NetworkTcp => token = token.allow_network("tcp", 0),
                    Permission::NetworkUdp => token = token.allow_network("udp", 0),
                    Permission::FileRead => token = token.allow_read("/var/www"),
                    Permission::FileWrite => token = token.allow_write("/tmp"),
                    Permission::ProcessExec => token = token.allow_exec(),
                    Permission::Ipc => token = token.allow_ipc(),
                    Permission::AudioPlayback | Permission::DisplayAccess => {
                        token.allow_capability(1 << perm as u64)
                    }
                }
            }
            self.gate.set_capability(token);
        }

        Ok(())
    }

    /// Validate syscall against process or thread-specific pledge
    pub fn validate_thread(&self, tid: u64, permission: Permission) -> Result<(), PledgeError> {
        if let Some(thread_ctx) = self.thread_sub_pledges.get(&tid) {
            if !thread_ctx.sub_promise.allows(permission) {
                return Err(PledgeError::Violation);
            }
        }
        self.validate(permission)
    }

    /// Validate syscall against process pledge
    pub fn validate(&self, permission: Permission) -> Result<(), PledgeError> {
        if let Some(ref pledge) = self.pledge {
            if !pledge.allows(permission) {
                return Err(PledgeError::Violation);
            }
        }
        Ok(())
    }

    /// Get current capability gate
    pub fn gate(&self) -> &CapabilityGate {
        &self.gate
    }
}

impl Default for PledgeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Common pledge promises
pub mod promises {
    use super::{Permission, PledgePromise};

    /// Stdio promise - basic I/O only
    pub fn stdio() -> PledgePromise {
        PledgePromise::new(vec![Permission::FileRead, Permission::FileWrite])
    }

    /// Network promise - network access
    pub fn network() -> PledgePromise {
        PledgePromise::new(vec![
            Permission::NetworkTcp,
            Permission::NetworkUdp,
            Permission::FileRead,
        ])
    }

    /// Exec promise - can execute processes
    pub fn exec() -> PledgePromise {
        PledgePromise::new(vec![
            Permission::ProcessExec,
            Permission::FileRead,
            Permission::FileWrite,
        ])
    }

    /// IPC promise - inter-process communication
    pub fn ipc() -> PledgePromise {
        PledgePromise::new(vec![Permission::Ipc, Permission::FileRead])
    }

    /// Full promise - all permissions
    pub fn full() -> PledgePromise {
        PledgePromise::new(vec![
            Permission::NetworkTcp,
            Permission::NetworkUdp,
            Permission::FileRead,
            Permission::FileWrite,
            Permission::ProcessExec,
            Permission::Ipc,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::promises::*;
    use super::*;

    #[test]
    fn test_pledge_creation() {
        let promise = PledgePromise::new(vec![Permission::FileRead]);
        assert!(!promise.active.load(Ordering::SeqCst));
    }

    #[test]
    fn test_pledge_activation() {
        let promise = PledgePromise::new(vec![Permission::FileRead]);
        assert!(promise.activate().is_ok());
        assert!(promise.activate().is_err());
    }

    #[test]
    fn test_pledge_permission_check() {
        let promise = PledgePromise::new(vec![Permission::FileRead]);
        promise.activate().unwrap();
        assert!(promise.allows(Permission::FileRead));
        assert!(!promise.allows(Permission::FileWrite));
    }

    #[test]
    fn test_pledge_manager() {
        let mut manager = PledgeManager::new();
        let promise = stdio();
        assert!(manager.pledge(promise).is_ok());
        assert!(manager.validate(Permission::FileRead).is_ok());
        assert!(manager.validate(Permission::ProcessExec).is_err());
    }

    #[test]
    fn test_common_promises() {
        let stdio_promise = stdio();
        assert!(stdio_promise.allows(Permission::FileRead));

        let network_promise = network();
        assert!(network_promise.allows(Permission::NetworkTcp));

        let full_promise = full();
        assert!(full_promise.allows(Permission::ProcessExec));
    }

    #[test]
    fn test_execpledge_manager() {
        let mut manager = PledgeManager::new();
        assert!(manager.active_execpledge().is_none());

        let exec_p = stdio();
        assert!(manager.execpledge(exec_p).is_ok());
        assert!(manager.active_execpledge().is_some());
        assert!(manager.execpledge(stdio()).is_err()); // Already set
    }
}
