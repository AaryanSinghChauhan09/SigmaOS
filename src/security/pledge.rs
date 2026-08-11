// SigmaOS Pledge and Unveil - Process Privilege Reduction Mechanisms
// Inspired by OpenBSD's security sandboxing models but capability-based

use crate::security::capability::{CapabilityGate, CapabilityToken, Permission};
use crate::klib::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

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
        self.permissions.as_slice()
    }
}

/// Pledge errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgeError {
    AlreadyActive,
    InvalidPermission,
    Violation,
}

/// Represents an unveiled directory or file path with permitted permissions (OpenBSD style)
pub struct UnveiledPath {
    pub path: [u8; 64],
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub create: bool,
}

impl UnveiledPath {
    pub fn new(path: &[u8], permissions: &[u8]) -> Self {
        let mut path_arr = [0u8; 64];
        let len = path.len().min(63);
        path_arr[..len].copy_from_slice(&path[..len]);

        let mut read = false;
        let mut write = false;
        let mut execute = false;
        let mut create = false;

        for &b in permissions {
            match b {
                b'r' => read = true,
                b'w' => write = true,
                b'x' => execute = true,
                b'c' => create = true,
                _ => {}
            }
        }

        UnveiledPath {
            path: path_arr,
            read,
            write,
            execute,
            create,
        }
    }

    /// Returns the length of the string path
    pub fn path_len(&self) -> usize {
        self.path.iter().position(|&b| b == 0).unwrap_or(64)
    }

    /// Checks if a requested path starts with this unveiled prefix
    pub fn matches_prefix(&self, target_path: &[u8]) -> bool {
        let len = self.path_len();
        if target_path.len() < len {
            return false;
        }
        &target_path[..len] == &self.path[..len]
    }
}

/// Process pledge and unveil manager
pub struct PledgeManager {
    /// Current pledge promise
    pledge: Option<PledgePromise>,
    /// Capability gate for validation
    gate: CapabilityGate,
    /// Unveiled files/directories list (OpenBSD style)
    pub unveiled_paths: Vec<UnveiledPath>,
}

impl PledgeManager {
    /// Create new pledge manager
    pub fn new() -> Self {
        Self {
            pledge: None,
            gate: CapabilityGate::new(),
            unveiled_paths: Vec::new(),
        }
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
                    Permission::FileRead => token = token.allow_read("/"),
                    Permission::FileWrite => token = token.allow_write("/tmp"),
                    Permission::ProcessExec => token = token.allow_exec(),
                    Permission::Ipc => token = token.allow_ipc(),
                }
            }
            self.gate.set_capability(token);
        }

        Ok(())
    }

    /// Register an unveiled path with its permitted permission set (e.g. "rw")
    pub fn unveil(&mut self, path: &[u8], permissions: &[u8]) {
        self.unveiled_paths.push(UnveiledPath::new(path, permissions));
    }

    /// Validates requested path access.
    /// Rules (OpenBSD spec):
    /// 1. If no paths are unveiled, standard UNIX/Capability rules apply (allow path).
    /// 2. If at least one path is unveiled, the namespace is locked down: any path that does NOT match an unveiled prefix is forbidden!
    pub fn validate_path_access(&self, target_path: &[u8], is_write: bool) -> bool {
        if self.unveiled_paths.is_empty() {
            return true; // No unveil lockdown active
        }

        // Find the most specific (longest) matching prefix
        let mut best_match: Option<&UnveiledPath> = None;
        let mut best_len = 0;

        for i in 0..self.unveiled_paths.len() {
            let entry = &self.unveiled_paths[i];
            if entry.matches_prefix(target_path) {
                let len = entry.path_len();
                if len > best_len {
                    best_len = len;
                    best_match = Some(entry);
                }
            }
        }

        if let Some(entry) = best_match {
            if is_write {
                entry.write
            } else {
                entry.read
            }
        } else {
            false // Path is completely hidden / forbidden outside the unveiled namespace
        }
    }

    /// Validate syscall against pledge
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

/// Enhanced pledge promises inspired by OpenBSD 7.9 security improvements
pub mod promises {
    use super::{Permission, PledgePromise};
    use crate::klib::Vec;

    /// Stdio promise - basic I/O only
    pub fn stdio() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// Network promise - network access
    pub fn network() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::NetworkTcp);
        p.push(Permission::NetworkUdp);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// DNS promise - DNS resolution only (OpenBSD 7.9 inspired)
    pub fn dns() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Dns);
        p.push(Permission::NetworkUdp);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// Unix domain sockets promise (OpenBSD 7.9 inspired)
    pub fn unix() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Unix);
        p.push(Permission::Ipc);
        PledgePromise::new(p)
    }

    /// TTY promise - terminal access (OpenBSD 7.9 inspired)
    pub fn tty() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Tty);
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// Process promise - process operations (OpenBSD 7.9 inspired)
    pub fn proc() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Proc);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// Exec promise - can execute processes
    pub fn exec() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::ProcessExec);
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// ID promise - user/group ID operations (OpenBSD 7.9 inspired)
    pub fn id() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Id);
        PledgePromise::new(p)
    }

    /// Settime promise - time setting (OpenBSD 7.9 inspired)
    pub fn settime() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Settime);
        PledgePromise::new(p)
    }

    /// PF promise - packet filter access (OpenBSD 7.9 inspired)
    pub fn pf() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Pf);
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// Route promise - routing table access (OpenBSD 7.9 inspired)
    pub fn route() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Route);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// Wroute promise - write routing table (OpenBSD 7.9 inspired)
    pub fn wroute() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Wroute);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// Audio promise - audio device access (OpenBSD 7.9 inspired)
    pub fn audio() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Audio);
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// Video promise - video device access (OpenBSD 7.9 inspired)
    pub fn video() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Video);
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        PledgePromise::new(p)
    }

    /// BPF promise - Berkeley Packet Filter access (OpenBSD 7.9 inspired)
    pub fn bpf() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Bpf);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// IPC promise - inter-process communication
    pub fn ipc() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::Ipc);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// SigmaOS AI capability promise - AI/ML operations
    pub fn ai_capability() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::AICapability);
        p.push(Permission::FileRead);
        p.push(Permission::ProcessExec);
        PledgePromise::new(p)
    }

    /// SigmaOS shard access promise - Shard system access
    pub fn shard_access() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::ShardAccess);
        p.push(Permission::Ipc);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// SigmaOS quantum crypto promise - Post-quantum crypto operations
    pub fn quantum_crypto() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::QuantumCrypto);
        p.push(Permission::FileRead);
        PledgePromise::new(p)
    }

    /// Full promise - all permissions
    pub fn full() -> PledgePromise {
        let mut p = Vec::new();
        p.push(Permission::NetworkTcp);
        p.push(Permission::NetworkUdp);
        p.push(Permission::FileRead);
        p.push(Permission::FileWrite);
        p.push(Permission::ProcessExec);
        p.push(Permission::Ipc);
        p.push(Permission::Dns);
        p.push(Permission::Unix);
        p.push(Permission::Tty);
        p.push(Permission::Proc);
        p.push(Permission::Id);
        p.push(Permission::Settime);
        p.push(Permission::Pf);
        p.push(Permission::Route);
        p.push(Permission::Wroute);
        p.push(Permission::Audio);
        p.push(Permission::Video);
        p.push(Permission::Bpf);
        p.push(Permission::AICapability);
        p.push(Permission::ShardAccess);
        p.push(Permission::QuantumCrypto);
        PledgePromise::new(p)
    }
}

#[cfg(test)]
mod tests {
    use super::promises::*;
    use super::*;

    #[test]
    fn test_pledge_creation() {
        let mut p = Vec::new();
        p.push(Permission::FileRead);
        let promise = PledgePromise::new(p);
        assert!(!promise.active.load(Ordering::SeqCst));
    }

    #[test]
    fn test_pledge_activation() {
        let mut p = Vec::new();
        p.push(Permission::FileRead);
        let promise = PledgePromise::new(p);
        assert!(promise.activate().is_ok());
        assert!(promise.activate().is_err());
    }

    #[test]
    fn test_pledge_permission_check() {
        let mut p = Vec::new();
        p.push(Permission::FileRead);
        let promise = PledgePromise::new(p);
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
    fn test_openbsd_unveil_sandboxing() {
        let mut manager = PledgeManager::new();

        // 1. By default, with no unveiled paths, access is permitted
        assert!(manager.validate_path_access(b"/etc/passwd", false));
        assert!(manager.validate_path_access(b"/tmp/session.lock", true));

        // 2. Unveil a specific read-only prefix and a read-write prefix
        manager.unveil(b"/tmp", b"rw");
        manager.unveil(b"/usr/local", b"r");

        // 3. Namespace should be locked down: unrelated paths are blocked / hidden!
        assert!(!manager.validate_path_access(b"/etc/passwd", false));

        // 4. Mapped prefixes should satisfy requested read/write constraints
        assert!(manager.validate_path_access(b"/tmp/file.txt", true));  // ReadWrite allowed
        assert!(manager.validate_path_access(b"/usr/local/bin/rustc", false)); // Read allowed
        assert!(!manager.validate_path_access(b"/usr/local/bin/rustc", true)); // Write rejected
    }
}
