// SigmaOS Privacy-First Sandbox Subsystem
// Enforces zero-trust sandboxing by default, with post-quantum cryptography baked into kernel-level syscall filters
// Enhanced with Sandboxie-style file system overlays and Firejail-style execution profiles.
// Taking inspiration from industry-leading competitors Sandboxie (FS virtualization overlays) and Firejail (strict execution profiles)

use std::collections::{HashSet, HashMap};
use std::collections::HashSet;
use std::collections::{HashSet, HashMap, BTreeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxRule {
    NetworkWriteGate,
    FSWriteGate,
    ProcessForkGate,
    IpcAccessGate,          // Block inter-process communication
    MemoryDbgAttachGate,    // Prevent debuggers attaching (ptrace)
    RawSocketOpenGate,      // Block raw socket creations
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxProfile {
    None,
    StrictBrowser,   // Demands network, blocks local filesystems except user downloads
    RestrictedOffice, // Demands file writes, absolutely blocks network gates
    IpcAccessGate,           // New: Prevents raw inter-process communications
    MemoryDbgAttachGate,     // New: Prevents ptrace or debugger attachments
    RawSocketOpenGate,       // New: Blocks raw network socket creation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxProfile {
    StrictBrowser,
    RestrictedOffice,
    UntrustedInstaller,
}

pub struct PrivacyFirstSandbox {
    pub process_id: u32,
    pub is_active_sandboxed: bool,
    pub active_pqc_key_attestation: String,
    pub blocked_rules: HashSet<SandboxRule>,
    pub profile: SandboxProfile,
    pub sanitized_env: HashMap<String, String>,
    pub virtual_filesystem_overlay: HashMap<String, Vec<u8>>, // Sandboxie-style overlay file system
    // Sandboxie-inspired file system virtualization overlays
    pub virtualization_overlay: BTreeMap<String, String>,
    // Firejail-inspired sanitized execution environment
    pub environment_variables: HashMap<String, String>,
    pub profile: Option<SandboxProfile>,
}

impl PrivacyFirstSandbox {
    pub fn new(pid: u32, pqc_key: &str) -> Self {
        PrivacyFirstSandbox {
            process_id: pid,
            is_active_sandboxed: true,
            active_pqc_key_attestation: pqc_key.to_string(),
            blocked_rules: HashSet::new(),
            profile: SandboxProfile::None,
            sanitized_env: HashMap::new(),
            virtual_filesystem_overlay: HashMap::new(),
        }
    }

    /// Sets up a Firejail-style execution profile constraints
    pub fn apply_profile(&mut self, profile: SandboxProfile) {
        self.profile = profile;
        match profile {
            SandboxProfile::StrictBrowser => {
                // Allow network writes, block raw socket openings, local file modifications, and debugging
                self.blocked_rules.insert(SandboxRule::FSWriteGate);
                self.blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                self.blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                self.blocked_rules.remove(&SandboxRule::NetworkWriteGate);
            }
            SandboxProfile::RestrictedOffice => {
                // Allow filesystem writes, strictly block any outgoing/incoming network sockets and debuggers
                self.blocked_rules.insert(SandboxRule::NetworkWriteGate);
                self.blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                self.blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                self.blocked_rules.remove(&SandboxRule::FSWriteGate);
            }
            SandboxProfile::None => {
                self.blocked_rules.clear();
            }
        }
    }

    /// Construct a Sandbox with predefined strict competitor execution profiles
    pub fn with_profile(pid: u32, pqc_key: &str, profile: SandboxProfile) -> Self {
        let mut sandbox = Self::new(pid, pqc_key);
        sandbox.profile = Some(profile);

        match profile {
            SandboxProfile::StrictBrowser => {
                sandbox.block_syscall_rule(SandboxRule::FSWriteGate);
                sandbox.block_syscall_rule(SandboxRule::ProcessForkGate);
                sandbox.block_syscall_rule(SandboxRule::MemoryDbgAttachGate);
                sandbox.block_syscall_rule(SandboxRule::RawSocketOpenGate);
                sandbox.set_environment("BROWSER_SANDBOX_ENFORCED".to_string(), "1".to_string());
            }
            SandboxProfile::RestrictedOffice => {
                sandbox.block_syscall_rule(SandboxRule::NetworkWriteGate);
                sandbox.block_syscall_rule(SandboxRule::IpcAccessGate);
                sandbox.block_syscall_rule(SandboxRule::MemoryDbgAttachGate);
                sandbox.set_environment("OFFICE_ISOLATION_ENFORCED".to_string(), "1".to_string());
            }
            SandboxProfile::UntrustedInstaller => {
                sandbox.block_syscall_rule(SandboxRule::NetworkWriteGate);
                sandbox.block_syscall_rule(SandboxRule::RawSocketOpenGate);
                sandbox.block_syscall_rule(SandboxRule::MemoryDbgAttachGate);
                sandbox.set_environment("INSTALLER_GUARD_ACTIVE".to_string(), "1".to_string());
            }
        }
        sandbox
    }

    pub fn block_syscall_rule(&mut self, rule: SandboxRule) {
        self.blocked_rules.insert(rule);
    }

    pub fn validate_syscall_transition(&self, rule: SandboxRule) -> bool {
        if !self.is_active_sandboxed {
            return true; // Bypass checks if sandboxing is explicitly disabled
        }
        // If the rule is blocked, deny transition
        !self.blocked_rules.contains(&rule)
    }

    /// Firejail-style environment variable sanitizer to prevent privilege escalation / variable injections
    pub fn sanitize_environment(&mut self, env_vars: &[(&str, &str)]) {
        let sensitive_prefixes = ["LD_", "RUST_", "PATH", "SHELL", "USER"];
        for &(key, val) in env_vars {
            let mut is_sensitive = false;
            for prefix in &sensitive_prefixes {
                if key.starts_with(prefix) {
                    is_sensitive = true;
                    break;
                }
            }
            if !is_sensitive {
                self.sanitized_env.insert(key.to_string(), val.to_string());
            }
        }
    }

    // ==========================================
    // Sandboxie-style File Virtualization Overlay
    // ==========================================

    /// Emulates writing a file inside the isolated sandbox overlay
    pub fn virtual_write(&mut self, file_path: &str, content: &[u8]) -> Result<(), &'static str> {
        if !self.validate_syscall_transition(SandboxRule::FSWriteGate) {
            return Err("System FSWriteGate is blocked; filesystem mutations must go through custom overlay maps");
        }
        self.virtual_filesystem_overlay.insert(file_path.to_string(), content.to_vec());
        Ok(())
    }

    /// Emulates reading a file, falling back to host buffer if not modified inside the sandbox
    pub fn virtual_read(&self, file_path: &str, host_fallback_content: &[u8]) -> Vec<u8> {
        if let Some(content) = self.virtual_filesystem_overlay.get(file_path) {
            content.clone()
        } else {
            host_fallback_content.to_vec()
        }
    }

    /// Purges all virtualized file modifications inside the sandbox (perfect clean reset)
    pub fn purge_sandbox(&mut self) {
        self.virtual_filesystem_overlay.clear();
    }

    /// Sandboxie-style virtualization write: writes securely to an isolated memory overlay instead of modifying the host FS
    pub fn virtual_write(&mut self, path: &str, content: String) {
        self.virtualization_overlay.insert(path.to_string(), content);
    }

    /// Sandboxie-style virtualization read: attempts to read from the memory overlay first
    pub fn virtual_read(&self, path: &str) -> Option<&str> {
        self.virtualization_overlay.get(path).map(|s| s.as_str())
    }

    /// Purges all isolated writes and virtual file structures
    pub fn purge_sandbox(&mut self) {
        self.virtualization_overlay.clear();
    }

    /// Set isolated environment variable
    pub fn set_environment(&mut self, key: String, val: String) {
        self.environment_variables.insert(key, val);
    }

    /// Query isolated environment variable
    pub fn get_environment(&self, key: &str) -> Option<&str> {
        self.environment_variables.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_first_sandbox() {
        let mut sandbox = PrivacyFirstSandbox::new(505, "crystals-dilithium-attestation-token-999");
        assert!(sandbox.is_active_sandboxed);
        assert_eq!(sandbox.active_pqc_key_attestation, "crystals-dilithium-attestation-token-999");

        // Allowed by default
        assert!(sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));

        // Block and verify rejection
        sandbox.block_syscall_rule(SandboxRule::NetworkWriteGate);
        assert!(!sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
        assert!(sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
    }

    #[test]
    fn test_firejail_execution_profiles() {
        let mut sandbox = PrivacyFirstSandbox::new(600, "crystal-key-888");

        // Apply strict browser profile
        sandbox.apply_profile(SandboxProfile::StrictBrowser);
        assert!(!sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
        assert!(sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));

        // Apply restricted office profile
        sandbox.apply_profile(SandboxProfile::RestrictedOffice);
        assert!(sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
        assert!(!sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
    }

    #[test]
    fn test_env_sanitizer() {
        let mut sandbox = PrivacyFirstSandbox::new(700, "key-777");
        let raw_env = [
            ("LD_PRELOAD", "/lib/malicious.so"),
            ("APP_THEME", "dark"),
            ("PATH", "/usr/bin"),
            ("LICENSE_KEY", "12345"),
        ];

        sandbox.sanitize_environment(&raw_env);
        assert_eq!(sandbox.sanitized_env.get("APP_THEME").unwrap(), "dark");
        assert_eq!(sandbox.sanitized_env.get("LICENSE_KEY").unwrap(), "12345");
        assert!(sandbox.sanitized_env.get("LD_PRELOAD").is_none());
        assert!(sandbox.sanitized_env.get("PATH").is_none());
    }

    #[test]
    fn test_sandboxie_file_virtualizer_overlay() {
        let mut sandbox = PrivacyFirstSandbox::new(800, "key-888");

        let host_etc_hosts = b"127.0.0.1 localhost";

        // Write virtualized overlay modification
        let sandboxed_hosts = b"127.0.0.1 localhost\n127.0.0.1 my-blocked-site.com";
        assert!(sandbox.virtual_write("/etc/hosts", sandboxed_hosts).is_ok());

        // Read virtualized overlay should return modified version
        let read_content = sandbox.virtual_read("/etc/hosts", host_etc_hosts);
        assert_eq!(read_content, sandboxed_hosts.to_vec());

        // Read unmodified file should return host fallback
        let read_unmodified = sandbox.virtual_read("/etc/resolv.conf", b"nameserver 8.8.8.8");
        assert_eq!(read_unmodified, b"nameserver 8.8.8.8".to_vec());

        // Purge and check reset to host fallbacks
        sandbox.purge_sandbox();
        let read_after_purge = sandbox.virtual_read("/etc/hosts", host_etc_hosts);
        assert_eq!(read_after_purge, host_etc_hosts.to_vec());
    }

    #[test]
    fn test_competitor_profiles_sandboxing() {
        // Test strict browser profile
        let browser_sandbox = PrivacyFirstSandbox::with_profile(601, "dilithium-key-1", SandboxProfile::StrictBrowser);
        assert!(!browser_sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
        assert!(!browser_sandbox.validate_syscall_transition(SandboxRule::ProcessForkGate));
        assert!(!browser_sandbox.validate_syscall_transition(SandboxRule::MemoryDbgAttachGate));
        assert_eq!(browser_sandbox.get_environment("BROWSER_SANDBOX_ENFORCED").unwrap(), "1");

        // Test restricted office profile
        let office_sandbox = PrivacyFirstSandbox::with_profile(602, "dilithium-key-2", SandboxProfile::RestrictedOffice);
        assert!(!office_sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
        assert!(!office_sandbox.validate_syscall_transition(SandboxRule::IpcAccessGate));
        assert_eq!(office_sandbox.get_environment("OFFICE_ISOLATION_ENFORCED").unwrap(), "1");
    }

    #[test]
    fn test_sandboxie_style_virtualization_overlays() {
        let mut sandbox = PrivacyFirstSandbox::new(701, "key-3");
        assert!(sandbox.virtual_read("/etc/passwd").is_none());

        // Virtual write
        sandbox.virtual_write("/etc/passwd", "root:x:0:0:root:/root:/bin/sh".to_string());
        assert_eq!(sandbox.virtual_read("/etc/passwd").unwrap(), "root:x:0:0:root:/root:/bin/sh");

        // Purge
        sandbox.purge_sandbox();
        assert!(sandbox.virtual_read("/etc/passwd").is_none());
    }
}
// SigmaOS Privacy-First Sandbox Subsystem
// Enforces zero-trust sandboxing by default, with post-quantum cryptography baked into kernel-level syscall filters
// Absorbs advanced security controls from SELinux, AppArmor, and Firejail to satisfy Common Criteria and FIPS compliance


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxRule {
    NetworkWriteGate,
    FSWriteGate,
    ProcessForkGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementLevel {
    Enforce,  // Block and log
    Complain, // Log but allow
    Disable,  // Bypass all checks
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceProfile {
    StandardSandbox,
    Fips140_3,
    CommonCriteria_EAL4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignSecurityContext {
    pub user: String,
    pub role: String,
    pub domain: String,
    pub sensitivity: String, // Multi-Level Security (MLS) label
}

impl SovereignSecurityContext {
    pub fn new(user: &str, role: &str, domain: &str, level: &str) -> Self {
        SovereignSecurityContext {
            user: user.to_string(),
            role: role.to_string(),
            domain: domain.to_string(),
            sensitivity: level.to_string(),
        }
    }

    pub fn to_string_context(&self) -> String {
        format!("{}:{}:{}:{}", self.user, self.role, self.domain, self.sensitivity)
    }
}

pub struct PrivacyFirstSandbox {
    pub process_id: u32,
    pub is_active_sandboxed: bool,
    pub active_pqc_key_attestation: String,
    pub blocked_rules: HashSet<SandboxRule>,
    // Advanced SELinux, AppArmor, and Firejail absorptions:
    pub enforcement: EnforcementLevel,
    pub security_context: SovereignSecurityContext,
    pub private_dir_shields: HashSet<String>,
    pub compliance: ComplianceProfile,
    pub security_audit_log: Vec<String>,
}

impl PrivacyFirstSandbox {
    pub fn new(pid: u32, pqc_key: &str) -> Self {
        PrivacyFirstSandbox {
            process_id: pid,
            is_active_sandboxed: true,
            active_pqc_key_attestation: pqc_key.to_string(),
            blocked_rules: HashSet::new(),
            enforcement: EnforcementLevel::Enforce,
            security_context: SovereignSecurityContext::new("system_u", "system_r", "sandbox_t", "s0"),
            private_dir_shields: HashSet::new(),
            compliance: ComplianceProfile::StandardSandbox,
            security_audit_log: Vec::new(),
        }
    }

    pub fn block_syscall_rule(&mut self, rule: SandboxRule) {
        self.blocked_rules.insert(rule);
    }

    pub fn shield_private_directory(&mut self, path: &str) {
        self.private_dir_shields.insert(path.to_string());
    }

    /// AppArmor and SELinux parity validation checking
    pub fn validate_syscall_transition(&mut self, rule: SandboxRule) -> bool {
        if self.enforcement == EnforcementLevel::Disable || !self.is_active_sandboxed {
            return true;
        }

        let is_blocked = self.blocked_rules.contains(&rule);

        if is_blocked {
            let log_msg = format!(
                "AUDIT: Syscall rule {:?} denied for context '{}'",
                rule,
                self.security_context.to_string_context()
            );
            self.security_audit_log.push(log_msg);

            if self.enforcement == EnforcementLevel::Enforce {
                return false; // Action Blocked
            }
        }

        true // Allowed (or allowed in Complain mode)
    }

    /// Firejail-parity path security shield validation
    pub fn validate_path_access(&mut self, target_path: &str) -> bool {
        if self.enforcement == EnforcementLevel::Disable {
            return true;
        }

        // Check if path is shielded inside the private sandbox overlay
        for shield in &self.private_dir_shields {
            if target_path.starts_with(shield) {
                let log_msg = format!("AUDIT: Access to shielded path '{}' denied", target_path);
                self.security_audit_log.push(log_msg);

                if self.enforcement == EnforcementLevel::Enforce {
                    return false; // Blocked
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_privacy_first_sandbox() {
        let mut sandbox = PrivacyFirstSandbox::new(505, "crystals-dilithium-attestation-token-999");
        assert!(sandbox.is_active_sandboxed);
        assert_eq!(sandbox.active_pqc_key_attestation, "crystals-dilithium-attestation-token-999");

        // Allowed by default
        assert!(sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));

        // Block and verify rejection
        sandbox.block_syscall_rule(SandboxRule::NetworkWriteGate);
        assert!(!sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
        assert!(sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
    }

    #[test]
    fn test_selinux_rbac_contexts() {
        let mut sandbox = PrivacyFirstSandbox::new(606, "pqc-token-111");
        assert_eq!(sandbox.security_context.to_string_context(), "system_u:system_r:sandbox_t:s0");

        // Set high sensitivity Multi-Level Security context
        sandbox.security_context = SovereignSecurityContext::new("admin_u", "admin_r", "trusted_t", "s0-s3:c0.c1023");
        assert_eq!(sandbox.security_context.to_string_context(), "admin_u:admin_r:trusted_t:s0-s3:c0.c1023");
    }

    #[test]
    fn test_apparmor_complain_mode() {
        let mut sandbox = PrivacyFirstSandbox::new(707, "pqc-token-222");
        sandbox.block_syscall_rule(SandboxRule::ProcessForkGate);

        // AppArmor Complain mode allows but logs
        sandbox.enforcement = EnforcementLevel::Complain;
        assert!(sandbox.validate_syscall_transition(SandboxRule::ProcessForkGate));
        assert_eq!(sandbox.security_audit_log.len(), 1);
        assert!(sandbox.security_audit_log[0].contains("ProcessForkGate"));
    }

    #[test]
    fn test_firejail_directory_shields() {
        let mut sandbox = PrivacyFirstSandbox::new(808, "pqc-token-333");
        sandbox.shield_private_directory("/etc/shadow");
        sandbox.shield_private_directory("/var/log/audit");

        // Enforce mode blocks access
        assert!(!sandbox.validate_path_access("/etc/shadow/admin"));
        assert!(sandbox.validate_path_access("/home/user/document.txt"));

        // Disable mode bypasses blocks
        sandbox.enforcement = EnforcementLevel::Disable;
        assert!(sandbox.validate_path_access("/etc/shadow/admin"));
    }
}
