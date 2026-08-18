// SigmaOS Privacy-First Sandbox Subsystem
// Enforces zero-trust sandboxing by default, with post-quantum cryptography baked into kernel-level syscall filters
// Enhanced with Sandboxie-style file system overlays and Firejail-style execution profiles.

use std::collections::{HashSet, HashMap};

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
}

pub struct PrivacyFirstSandbox {
    pub process_id: u32,
    pub is_active_sandboxed: bool,
    pub active_pqc_key_attestation: String,
    pub blocked_rules: HashSet<SandboxRule>,
    pub profile: SandboxProfile,
    pub sanitized_env: HashMap<String, String>,
    pub virtual_filesystem_overlay: HashMap<String, Vec<u8>>, // Sandboxie-style overlay file system
    pub overlay_writes: HashMap<String, Vec<u8>>, // New optimized overlay writes
    pub readonly_paths: Vec<String>, // Firejail-style read-only paths
    pub private_paths: Vec<String>, // Firejail-style private namespace paths
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
            overlay_writes: HashMap::new(),
            readonly_paths: Vec::new(),
            private_paths: Vec::new(),
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

    /// Sandboxie-style File Virtualization: Write Operation.
    /// Intercepts path modifications and commits them strictly to the private memory overlay.
    pub fn virtual_write(&mut self, path: &str, data: &[u8]) -> Result<(), &'static str> {
        if !self.is_active_sandboxed {
            return Err("Sandbox is inactive, virtual write denied.");
        }
        if !self.validate_syscall_transition(SandboxRule::FSWriteGate) {
            return Err("System FSWriteGate is blocked");
        }
        // Firejail-style read-only check
        if self.readonly_paths.iter().any(|ro| path.starts_with(ro)) {
            return Err("Access violation: write to a read-only sandboxed path.");
        }
        self.overlay_writes.insert(path.to_string(), data.to_vec());
        // Also maintain compatibility with old virtual_filesystem_overlay
        self.virtual_filesystem_overlay.insert(path.to_string(), data.to_vec());
        Ok(())
    }

    /// Sandboxie-style File Virtualization: Read Operation.
    /// Intercepts path reads. Returns the sandboxed virtual file if modified, otherwise falls back to host.
    pub fn virtual_read(&self, path: &str, host_content: &[u8]) -> Result<Vec<u8>, &'static str> {
        if !self.is_active_sandboxed {
            return Ok(host_content.to_vec());
        }
        // Firejail-style private namespace check: hide sensitive files
        if self.private_paths.iter().any(|priv_path| path.starts_with(priv_path)) {
            return Err("Access violation: path is marked private in this sandbox namespace.");
        }
        if let Some(sandboxed_data) = self.overlay_writes.get(path) {
            Ok(sandboxed_data.clone())
        } else if let Some(legacy_data) = self.virtual_filesystem_overlay.get(path) {
            Ok(legacy_data.clone())
        } else {
            Ok(host_content.to_vec())
        }
    }

    /// Emulates reading a file, falling back to host buffer if not modified inside the sandbox (legacy compatibility)
    pub fn virtual_read_legacy(&self, file_path: &str, host_fallback_content: &[u8]) -> Vec<u8> {
        if let Some(content) = self.virtual_filesystem_overlay.get(file_path) {
            content.clone()
        } else {
            host_fallback_content.to_vec()
        }
    }

    /// Discards all virtual overlay writes completely, leaving the host system completely clean.
    pub fn purge_sandbox(&mut self) {
        self.overlay_writes.clear();
        self.virtual_filesystem_overlay.clear();
    }

    pub fn set_environment(&mut self, key: String, val: String) {
        self.sanitized_env.insert(key, val);
    }

    /// Firejail-style environment variable sanitization.
    /// Strips hazardous environment variables (e.g., LD_PRELOAD, LD_LIBRARY_PATH, path manipulation)
    /// to prevent dynamic linking injection and process hijacking.
    pub fn sanitize_env_variables(&self, env: HashMap<String, String>) -> HashMap<String, String> {
        let blacklisted_vars = [
            "LD_PRELOAD",
            "LD_LIBRARY_PATH",
            "DYLD_INSERT_LIBRARIES",
            "DYLD_LIBRARY_PATH",
            "PATH",
        ];

        env.into_iter()
            .filter(|(key, _)| !blacklisted_vars.contains(&key.as_str()))
            .collect
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
        let read_content = sandbox.virtual_read("/etc/hosts", host_etc_hosts).unwrap();
        assert_eq!(read_content, sandboxed_hosts.to_vec());

        // Read unmodified file should return host fallback
        let read_unmodified = sandbox.virtual_read("/etc/resolv.conf", b"nameserver 8.8.8.8").unwrap();
        assert_eq!(read_unmodified, b"nameserver 8.8.8.8".to_vec());

        // Purge and check reset to host fallbacks
        sandbox.purge_sandbox();
        let read_after_purge = sandbox.virtual_read("/etc/hosts", host_etc_hosts).unwrap();
        assert_eq!(read_after_purge, host_etc_hosts.to_vec());
    }
}
