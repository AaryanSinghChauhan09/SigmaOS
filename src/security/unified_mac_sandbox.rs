//! Unified Auditable MAC Sandbox & Mandatory Access Control Framework
//!
//! Inspired by AppArmor, SELinux, OpenBSD pledge/unveil, FreeBSD jails, and bubblewrap.
//! Provides a multi-layer security sandbox model combining:
//! - Syscall pledge filtering (restricting syscall promise categories: stdio, rpath, wpath, cpath, inet, exec, proc)
//! - VFS unveil path restrictions (path allowlist/denylist with read/write/exec/create permissions)
//! - Capability & Resource constraints (network sockets, memory bounds, IPC permissions)
//! - Auditable security event logging with structured, immutable logs for rule evaluations and violations.

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Mandatory Access Control Policy Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacPolicyMode {
    Enforcing,  // Block access violations and log audit events
    Permissive, // Allow access violations but log audit warnings
    Disabled,   // Bypass access control checks
}

/// OpenBSD-Style Syscall Pledge Promise Categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgePromise {
    StdIo,   // Standard input/output/pipes
    RPath,   // Read-only filesystem access
    WPath,   // Write filesystem access
    CPath,   // Create/delete filesystem nodes
    INet,    // Network socket operations
    Exec,    // Execute binary images
    Proc,    // Process management (fork/kill/ps)
    ProtExec,// Executable memory allocation (mprotect/mmap PROT_EXEC)
}

impl PledgePromise {
    pub fn as_str(&self) -> &str {
        match self {
            PledgePromise::StdIo => "stdio",
            PledgePromise::RPath => "rpath",
            PledgePromise::WPath => "wpath",
            PledgePromise::CPath => "cpath",
            PledgePromise::INet => "inet",
            PledgePromise::Exec => "exec",
            PledgePromise::Proc => "proc",
            PledgePromise::ProtExec => "protexec",
        }
    }
}

/// VFS Unveil Path Access Permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnveilPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub create: bool,
}

impl UnveilPermissions {
    pub fn read_only() -> Self {
        Self { read: true, write: false, execute: false, create: false }
    }

    pub fn read_write() -> Self {
        Self { read: true, write: true, execute: false, create: true }
    }

    pub fn read_execute() -> Self {
        Self { read: true, write: false, execute: true, create: false }
    }

    pub fn full_access() -> Self {
        Self { read: true, write: true, execute: true, create: true }
    }

    pub fn none() -> Self {
        Self { read: false, write: false, execute: false, create: false }
    }
}

/// Unveil Rule (Path Constraint)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnveilPathRule {
    pub path_pattern: String,
    pub permissions: UnveilPermissions,
    pub is_denylist: bool,
}

/// Auditable Security Event Log Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAuditEvent {
    pub timestamp_sec: u64,
    pub process_id: u32,
    pub app_name: String,
    pub policy_mode: MacPolicyMode,
    pub event_type: String, // "PLEDGE_VIOLATION", "UNVEIL_DENIED", "ACCESS_ALLOWED"
    pub target_resource: String,
    pub decision_action: String, // "BLOCKED", "AUDIT_ONLY", "ALLOWED"
}

/// Multi-Layer MAC Sandbox Profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacSandboxProfile {
    pub profile_id: String,
    pub app_name: String,
    pub mode: MacPolicyMode,
    pub pledged_promises: Vec<PledgePromise>,
    pub unveil_rules: Vec<UnveilPathRule>,
    pub allow_network: bool,
    pub max_memory_bytes: u64,
    pub allow_raw_sockets: bool,
}

impl MacSandboxProfile {
    pub fn new(profile_id: &str, app_name: &str) -> Self {
        Self {
            profile_id: profile_id.to_string(),
            app_name: app_name.to_string(),
            mode: MacPolicyMode::Enforcing,
            pledged_promises: vec![PledgePromise::StdIo],
            unveil_rules: Vec::new(),
            allow_network: false,
            max_memory_bytes: 512 * 1024 * 1024, // 512 MB default
            allow_raw_sockets: false,
        }
    }

    pub fn strict_browser_sandbox(profile_id: &str) -> Self {
        let mut prof = Self::new(profile_id, "Sovereign Browser");
        prof.pledged_promises = vec![PledgePromise::StdIo, PledgePromise::RPath, PledgePromise::INet];
        prof.allow_network = true;
        prof.unveil_rules.push(UnveilPathRule {
            path_pattern: "/home/user/Downloads".to_string(),
            permissions: UnveilPermissions::read_write(),
            is_denylist: false,
        });
        prof.unveil_rules.push(UnveilPathRule {
            path_pattern: "/etc/shadow".to_string(),
            permissions: UnveilPermissions::none(),
            is_denylist: true,
        });
        prof
    }

    /// Serializes profile to policy language specification format (JSON-like)
    pub fn to_policy_spec(&self) -> String {
        let promises_str: Vec<&str> = self.pledged_promises.iter().map(|p| p.as_str()).collect();
        format!(
            "{{\n  \"profile_id\": \"{}\",\n  \"app_name\": \"{}\",\n  \"mode\": \"{:?}\",\n  \"pledges\": [\"{}\"],\n  \"allow_network\": {}\n}}",
            self.profile_id,
            self.app_name,
            self.mode,
            promises_str.join("\", \""),
            self.allow_network
        )
    }
}

/// Unified Auditable MAC Sandbox Engine
pub struct UnifiedMacSandboxEngine {
    pub active_profiles: Vec<MacSandboxProfile>,
    pub audit_log: Vec<MacAuditEvent>,
    pub timestamp_clock_sec: u64,
}

impl UnifiedMacSandboxEngine {
    pub fn new() -> Self {
        Self {
            active_profiles: Vec::new(),
            audit_log: Vec::new(),
            timestamp_clock_sec: 1700000000,
        }
    }

    pub fn register_profile(&mut self, profile: MacSandboxProfile) {
        if let Some(pos) = self.active_profiles.iter().position(|p| p.profile_id == profile.profile_id) {
            self.active_profiles[pos] = profile;
        } else {
            self.active_profiles.push(profile);
        }
    }

    /// Checks if a process syscall promise is granted under its active profile
    pub fn evaluate_pledge_syscall(&mut self, profile_id: &str, pid: u32, promise: PledgePromise) -> bool {
        self.timestamp_clock_sec += 1;
        let profile = match self.active_profiles.iter().find(|p| p.profile_id == profile_id) {
            Some(p) => p,
            None => return true, // Default unconfined
        };

        if profile.mode == MacPolicyMode::Disabled {
            return true;
        }

        let is_granted = profile.pledged_promises.contains(&promise);

        let (action, result_bool) = if is_granted {
            ("ALLOWED".to_string(), true)
        } else if profile.mode == MacPolicyMode::Permissive {
            ("AUDIT_ONLY".to_string(), true)
        } else {
            ("BLOCKED".to_string(), false)
        };

        self.audit_log.push(MacAuditEvent {
            timestamp_sec: self.timestamp_clock_sec,
            process_id: pid,
            app_name: profile.app_name.clone(),
            policy_mode: profile.mode,
            event_type: "PLEDGE_EVALUATION".to_string(),
            target_resource: promise.as_str().to_string(),
            decision_action: action,
        });

        result_bool
    }

    /// Checks VFS unveil path permissions for read/write/exec access
    pub fn evaluate_unveil_access(
        &mut self,
        profile_id: &str,
        pid: u32,
        target_path: &str,
        req_read: bool,
        req_write: bool,
        req_exec: bool,
    ) -> bool {
        self.timestamp_clock_sec += 1;
        let profile = match self.active_profiles.iter().find(|p| p.profile_id == profile_id) {
            Some(p) => p,
            None => return true,
        };

        if profile.mode == MacPolicyMode::Disabled {
            return true;
        }

        let mut allowed = false;

        // 1. Check explicit denylist rules first
        for rule in &profile.unveil_rules {
            if rule.is_denylist && target_path.starts_with(&rule.path_pattern) {
                self.audit_log.push(MacAuditEvent {
                    timestamp_sec: self.timestamp_clock_sec,
                    process_id: pid,
                    app_name: profile.app_name.clone(),
                    policy_mode: profile.mode,
                    event_type: "UNVEIL_DENIED_EXPLICIT".to_string(),
                    target_resource: target_path.to_string(),
                    decision_action: if profile.mode == MacPolicyMode::Permissive { "AUDIT_ONLY".to_string() } else { "BLOCKED".to_string() },
                });
                return profile.mode == MacPolicyMode::Permissive;
            }
        }

        // 2. Check allowlist rules
        if profile.unveil_rules.iter().all(|r| r.is_denylist) {
            allowed = true; // No allowlist restrictions defined
        } else {
            for rule in &profile.unveil_rules {
                if !rule.is_denylist && target_path.starts_with(&rule.path_pattern) {
                    let perm = rule.permissions;
                    if (!req_read || perm.read) && (!req_write || perm.write) && (!req_exec || perm.execute) {
                        allowed = true;
                        break;
                    }
                }
            }
        }

        let (action, result_bool) = if allowed {
            ("ALLOWED".to_string(), true)
        } else if profile.mode == MacPolicyMode::Permissive {
            ("AUDIT_ONLY".to_string(), true)
        } else {
            ("BLOCKED".to_string(), false)
        };

        self.audit_log.push(MacAuditEvent {
            timestamp_sec: self.timestamp_clock_sec,
            process_id: pid,
            app_name: profile.app_name.clone(),
            policy_mode: profile.mode,
            event_type: "UNVEIL_PATH_EVALUATION".to_string(),
            target_resource: target_path.to_string(),
            decision_action: action,
        });

        result_bool
    }
}

impl Default for UnifiedMacSandboxEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_browser_sandbox_profile() {
        let profile = MacSandboxProfile::strict_browser_sandbox("prof.browser");
        assert_eq!(profile.app_name, "Sovereign Browser");
        assert!(profile.pledged_promises.contains(&PledgePromise::INet));
        assert!(profile.to_policy_spec().contains("prof.browser"));
    }

    #[test]
    fn test_unified_mac_sandbox_pledge_and_unveil_enforcement() {
        let mut engine = UnifiedMacSandboxEngine::new();
        let profile = MacSandboxProfile::strict_browser_sandbox("prof.browser");
        engine.register_profile(profile);

        // 1. Evaluate pledged syscall promises
        assert!(engine.evaluate_pledge_syscall("prof.browser", 1001, PledgePromise::INet));
        assert!(!engine.evaluate_pledge_syscall("prof.browser", 1001, PledgePromise::Exec));

        // 2. Evaluate unveil path restrictions
        assert!(engine.evaluate_unveil_access("prof.browser", 1001, "/home/user/Downloads/file.pdf", true, true, false));
        assert!(!engine.evaluate_unveil_access("prof.browser", 1001, "/etc/shadow", true, false, false));

        // 3. Verify auditable security event log
        assert_eq!(engine.audit_log.len(), 4);
        assert!(engine.audit_log.iter().any(|e| e.decision_action == "BLOCKED"));
    }
}
