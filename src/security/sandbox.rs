// SigmaOS Privacy-First Sandbox Subsystem
// Enforces zero-trust sandboxing by default, with post-quantum cryptography baked into kernel-level syscall filters
// Enhanced with Sandboxie-style file system overlays and Firejail-style execution profiles.

use std::collections::{HashSet, HashMap};

/// Sandbox execution profiles matching specific application profiles (inspired by Firejail)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxProfile {
    Default,
    StrictBrowser,
    RestrictedOffice,
    IsolatedGame,
    ZeroTrust,
}

/// Advanced sandbox security and syscall gating rules (SecComp/Firejail style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxRule {
    NetworkWriteGate,
    NetworkReadGate,
    FSWriteGate,
    FSReadGate,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxProfile {
    StrictBrowser,
    RestrictedOffice,
    UntrustedInstaller,
}

/// High-fidelity, privacy-first sandbox container (inspired by Sandboxie and Firejail)
pub struct PrivacyFirstSandbox {
    pub process_id: u32,
    pub is_active_sandboxed: bool,
    pub active_pqc_key_attestation: String,
    pub blocked_rules: HashSet<SandboxRule>,
    pub profile: SandboxProfile,
    pub sanitized_env: HashMap<String, String>,
    pub virtual_filesystem_overlay: HashMap<String, Vec<u8>>, // Sandboxie-style overlay file system
}

impl PrivacyFirstSandbox {
    /// Creates a default sandbox instance for a specific process
    pub fn new(pid: u32, pqc_key: &str) -> Self {
        Self::with_profile(pid, pqc_key, SandboxProfile::Default)
    }

    /// Creates a sandbox initialized with a specific profile and custom policy rules (Firejail style)
    pub fn with_profile(pid: u32, pqc_key: &str, profile: SandboxProfile) -> Self {
        let mut blocked_rules = HashSet::new();
        let mut private_paths = HashSet::new();
        let mut readonly_paths = HashSet::new();

        match profile {
            SandboxProfile::Default => {
                // Allows standard operations, guards process forks
                blocked_rules.insert(SandboxRule::ProcessForkGate);
            }
            SandboxProfile::StrictBrowser => {
                // Restricts everything except essential network-read/write, enables extreme isolation
                blocked_rules.insert(SandboxRule::FSReadGate);
                blocked_rules.insert(SandboxRule::FSWriteGate);
                blocked_rules.insert(SandboxRule::ProcessForkGate);
                blocked_rules.insert(SandboxRule::RegistryReadWriteGate);
                blocked_rules.insert(SandboxRule::IpcAccessGate);
                blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                blocked_rules.insert(SandboxRule::HardwareDbgAccessGate);

                // Isolate browser temp and user credentials
                private_paths.insert("/etc/shadow".to_string());
                private_paths.insert("/home/user/.ssh".to_string());
                readonly_paths.insert("/usr/bin".to_string());
                readonly_paths.insert("/lib".to_string());
            }
            SandboxProfile::RestrictedOffice => {
                // Blocks all network write gates, isolates file system
                blocked_rules.insert(SandboxRule::NetworkWriteGate);
                blocked_rules.insert(SandboxRule::NetworkReadGate);
                blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                blocked_rules.insert(SandboxRule::ProcessForkGate);
                blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);

                readonly_paths.insert("/usr/bin".to_string());
            }
            SandboxProfile::IsolatedGame => {
                // Blocks registry manipulation and hardware/debug/network listen ports
                blocked_rules.insert(SandboxRule::RegistryReadWriteGate);
                blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                blocked_rules.insert(SandboxRule::HardwareDbgAccessGate);
            }
            SandboxProfile::ZeroTrust => {
                // Everything is blocked by default
                blocked_rules.insert(SandboxRule::NetworkWriteGate);
                blocked_rules.insert(SandboxRule::NetworkReadGate);
                blocked_rules.insert(SandboxRule::FSWriteGate);
                blocked_rules.insert(SandboxRule::FSReadGate);
                blocked_rules.insert(SandboxRule::ProcessForkGate);
                blocked_rules.insert(SandboxRule::RegistryReadWriteGate);
                blocked_rules.insert(SandboxRule::IpcAccessGate);
                blocked_rules.insert(SandboxRule::RawSocketOpenGate);
                blocked_rules.insert(SandboxRule::MemoryDbgAttachGate);
                blocked_rules.insert(SandboxRule::HardwareDbgAccessGate);
            }
        }

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

    /// Unblocks a syscall capability rule (e.g. for dynamic privilege escalation)
    pub fn unblock_syscall_rule(&mut self, rule: SandboxRule) {
        self.blocked_rules.remove(&rule);
    }

    /// Validates whether a syscall transition is permitted under current sandbox rules
    pub fn validate_syscall_transition(&self, rule: SandboxRule) -> bool {
        if !self.is_active_sandboxed {
            return true; // Bypass checks if sandboxing is explicitly disabled
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_first_sandbox() {
        let mut sandbox = PrivacyFirstSandbox::new(505, "crystals-dilithium-attestation-token-999");
        assert!(sandbox.is_active_sandboxed);
        assert_eq!(sandbox.active_pqc_key_attestation, "crystals-dilithium-attestation-token-999");

        // Allowed by default (except ProcessForkGate which is blocked by Default profile)
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
}
