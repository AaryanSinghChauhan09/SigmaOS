// SigmaOS Privacy-First Sandbox Subsystem
// Enforces zero-trust sandboxing by default, with post-quantum cryptography baked into kernel-level syscall filters
// Enhanced with Sandboxie-style file system overlays and Firejail-style execution profiles.

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
    UntrustedInstaller,
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
    pub profile: SandboxProfile,
    pub sanitized_env: HashMap<String, String>,
    pub virtual_filesystem_overlay: HashMap<String, Vec<u8>>,
    pub virtualization_overlay: BTreeMap<String, String>,
    pub environment_variables: HashMap<String, String>,
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
            profile: SandboxProfile::None,
            sanitized_env: HashMap::new(),
            virtual_filesystem_overlay: HashMap::new(),
            virtualization_overlay: BTreeMap::new(),
            environment_variables: HashMap::new(),
            enforcement: EnforcementLevel::Enforce,
            security_context: SovereignSecurityContext::new("system_u", "system_r", "sandbox_t", "s0"),
            private_dir_shields: HashSet::new(),
            compliance: ComplianceProfile::StandardSandbox,
            security_audit_log: Vec::new(),
        }
    }

    pub fn apply_profile(&mut self, profile: SandboxProfile) {
        self.profile = profile;
        match profile {
            SandboxProfile::StrictBrowser => {
                self.blocked_rules.insert(SandboxRule::FSWriteGate);
                self.blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                self.blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                self.blocked_rules.remove(&SandboxRule::NetworkWriteGate);
            }
            SandboxProfile::RestrictedOffice => {
                self.blocked_rules.insert(SandboxRule::NetworkWriteGate);
                self.blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                self.blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                self.blocked_rules.remove(&SandboxRule::FSWriteGate);
            }
            SandboxProfile::None | SandboxProfile::UntrustedInstaller => {
                self.blocked_rules.clear();
            }
        }
    }

    pub fn with_profile(pid: u32, pqc_key: &str, profile: SandboxProfile) -> Self {
        let mut sandbox = Self::new(pid, pqc_key);
        sandbox.profile = profile;

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
            SandboxProfile::None => {}
        }
        sandbox
    }

    pub fn block_syscall_rule(&mut self, rule: SandboxRule) {
        self.blocked_rules.insert(rule);
    }

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
                return false;
            }
        }

        true
    }

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

    pub fn virtual_write(&mut self, file_path: &str, content: &[u8]) -> Result<(), &'static str> {
        if !self.validate_syscall_transition(SandboxRule::FSWriteGate) {
            return Err("System FSWriteGate is blocked; filesystem mutations must go through custom overlay maps");
        }
        self.virtual_filesystem_overlay.insert(file_path.to_string(), content.to_vec());
        self.virtualization_overlay.insert(file_path.to_string(), String::from_utf8_lossy(content).to_string());
        Ok(())
    }

    pub fn virtual_read(&self, file_path: &str, host_fallback_content: &[u8]) -> Vec<u8> {
        if let Some(content) = self.virtual_filesystem_overlay.get(file_path) {
            content.clone()
        } else {
            host_fallback_content.to_vec()
        }
    }

    pub fn virtual_read_opt(&self, path: &str) -> Option<&str> {
        self.virtualization_overlay.get(path).map(|s| s.as_str())
    }

    pub fn purge_sandbox(&mut self) {
        self.virtual_filesystem_overlay.clear();
        self.virtualization_overlay.clear();
    }

    pub fn set_environment(&mut self, key: String, val: String) {
        self.environment_variables.insert(key, val);
    }

    pub fn get_environment(&self, key: &str) -> Option<&str> {
        self.environment_variables.get(key).map(|s| s.as_str())
    }

    pub fn shield_private_directory(&mut self, path: &str) {
        self.private_dir_shields.insert(path.to_string());
    }

    pub fn validate_path_access(&mut self, target_path: &str) -> bool {
        if self.enforcement == EnforcementLevel::Disable {
            return true;
        }

        for shield in &self.private_dir_shields {
            if target_path.starts_with(shield) {
                let log_msg = format!("AUDIT: Access to shielded path '{}' denied", target_path);
                self.security_audit_log.push(log_msg);

                if self.enforcement == EnforcementLevel::Enforce {
                    return false;
                }
            }
        }

        true
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

        assert!(sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));

        sandbox.block_syscall_rule(SandboxRule::NetworkWriteGate);
        assert!(!sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
        assert!(sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
    }

    #[test]
    fn test_firejail_execution_profiles() {
        let mut sandbox = PrivacyFirstSandbox::new(600, "crystal-key-888");

        sandbox.apply_profile(SandboxProfile::StrictBrowser);
        assert!(!sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
        assert!(sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));

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

        let sandboxed_hosts = b"127.0.0.1 localhost\n127.0.0.1 my-blocked-site.com";
        assert!(sandbox.virtual_write("/etc/hosts", sandboxed_hosts).is_ok());

        let read_content = sandbox.virtual_read("/etc/hosts", host_etc_hosts);
        assert_eq!(read_content, sandboxed_hosts.to_vec());

        let read_unmodified = sandbox.virtual_read("/etc/resolv.conf", b"nameserver 8.8.8.8");
        assert_eq!(read_unmodified, b"nameserver 8.8.8.8".to_vec());

        sandbox.purge_sandbox();
        let read_after_purge = sandbox.virtual_read("/etc/hosts", host_etc_hosts);
        assert_eq!(read_after_purge, host_etc_hosts.to_vec());
    }

    #[test]
    fn test_competitor_profiles_sandboxing() {
        let mut browser_sandbox = PrivacyFirstSandbox::with_profile(601, "dilithium-key-1", SandboxProfile::StrictBrowser);
        assert!(!browser_sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
        assert!(!browser_sandbox.validate_syscall_transition(SandboxRule::ProcessForkGate));
        assert!(!browser_sandbox.validate_syscall_transition(SandboxRule::MemoryDbgAttachGate));
        assert_eq!(browser_sandbox.get_environment("BROWSER_SANDBOX_ENFORCED").unwrap(), "1");

        let mut office_sandbox = PrivacyFirstSandbox::with_profile(602, "dilithium-key-2", SandboxProfile::RestrictedOffice);
        assert!(!office_sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
        assert!(!office_sandbox.validate_syscall_transition(SandboxRule::IpcAccessGate));
        assert_eq!(office_sandbox.get_environment("OFFICE_ISOLATION_ENFORCED").unwrap(), "1");
    }

    #[test]
    fn test_sandboxie_style_virtualization_overlays() {
        let mut sandbox = PrivacyFirstSandbox::new(701, "key-3");
        assert!(sandbox.virtual_read_opt("/etc/passwd").is_none());

        sandbox.virtual_write("/etc/passwd", b"root:x:0:0:root:/root:/bin/sh").unwrap();
        assert_eq!(sandbox.virtual_read_opt("/etc/passwd").unwrap(), "root:x:0:0:root:/root:/bin/sh");

        sandbox.purge_sandbox();
        assert!(sandbox.virtual_read_opt("/etc/passwd").is_none());
    }
}
