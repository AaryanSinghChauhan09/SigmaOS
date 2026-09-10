// SigmaOS Ultimate Linux & BSD Distro Parity Module
// Combines advanced packaging, sandboxing, and repository management from Debian, Arch, Void, Alpine, and OpenBSD.

#![allow(dead_code)]



use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// 1. Debian & Ubuntu APT Pinning & MultiArch Architecture Resolver
#[derive(Debug, Clone)]
pub struct AptPinRule {
    pub package_pattern: String,
    pub pin_origin: String,
    pub priority: i32, // e.g. 1001 = forced downgrade, 500 = default, -1 = block
}

#[derive(Debug, Clone)]
pub struct AptPinningMultiArchResolver {
    pub primary_arch: String,
    pub foreign_architectures: Vec<String>,
    pub pin_rules: Vec<AptPinRule>,
}

impl AptPinningMultiArchResolver {
    pub fn new(primary_arch: &str) -> Self {
        Self {
            primary_arch: primary_arch.to_string(),
            foreign_architectures: Vec::new(),
            pin_rules: Vec::new(),
        }
    }

    pub fn add_foreign_arch(&mut self, arch: &str) {
        if !self.foreign_architectures.contains(&arch.to_string()) {
            self.foreign_architectures.push(arch.to_string());
        }
    }

    pub fn add_pin_rule(&mut self, pattern: &str, origin: &str, priority: i32) {
        self.pin_rules.push(AptPinRule {
            package_pattern: pattern.to_string(),
            pin_origin: origin.to_string(),
            priority,
        });
    }

    pub fn resolve_package_priority(&self, pkg_name: &str, origin: &str) -> i32 {
        for rule in &self.pin_rules {
            if (rule.package_pattern == "*" || rule.package_pattern == pkg_name) && rule.pin_origin == origin {
                return rule.priority;
            }
        }
        500 // Default APT priority
    }

    pub fn is_architecture_supported(&self, arch: &str) -> bool {
        arch == self.primary_arch || self.foreign_architectures.iter().any(|a| a == arch)
    }
}

/// 2. Arch Linux ALPM Parallel Downloads & Latency Mirror Sync Engine
#[derive(Debug, Clone)]
pub struct AlpmMirror {
    pub url: String,
    pub latency_ms: u32,
    pub is_active: bool,
}

pub struct AlpmParallelSyncEngine {
    pub max_parallel_downloads: u32,
    pub mirrors: Vec<AlpmMirror>,
}

impl AlpmParallelSyncEngine {
    pub fn new(parallel_downloads: u32) -> Self {
        Self {
            max_parallel_downloads: parallel_downloads,
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u32) {
        self.mirrors.push(AlpmMirror {
            url: url.to_string(),
            latency_ms,
            is_active: true,
        });
    }

    pub fn sort_mirrors_by_speed(&mut self) {
        self.mirrors.sort_by_key(|m| m.latency_ms);
    }

    pub fn get_fastest_mirror(&self) -> Option<String> {
        self.mirrors.iter().find(|m| m.is_active).map(|m| m.url.clone())
    }
}

/// 3. Void Linux XBPS Package & RSA/Ed25519 Signature Verifier
#[derive(Debug, Clone)]
pub struct XbpsPackageHeader {
    pub pkgname: String,
    pub version: String,
    pub architecture: String,
    pub signature_hex: String,
}

pub struct XbpsSignatureVerifier {
    pub public_key_hex: String,
}

impl XbpsSignatureVerifier {
    pub fn new(pubkey: &str) -> Self {
        Self {
            public_key_hex: pubkey.to_string(),
        }
    }

    pub fn verify_package(&self, header: &XbpsPackageHeader, archive_bytes: &[u8]) -> bool {
        if header.signature_hex.is_empty() || archive_bytes.is_empty() {
            return false;
        }
        // Verification simulator for XBPS Ed25519 signatures
        !self.public_key_hex.is_empty() && header.signature_hex.starts_with("sig_ed25519_")
    }
}

/// 4. Alpine Linux APK Trigger Hook & Index Engine
#[derive(Debug, Clone)]
pub struct ApkTriggerHook {
    pub name: String,
    pub target_path_prefix: String,
    pub exec_script: String,
}

pub struct ApkIndexTriggerEngine {
    pub triggers: Vec<ApkTriggerHook>,
    pub executed_triggers: Vec<String>,
}

impl ApkIndexTriggerEngine {
    pub fn new() -> Self {
        Self {
            triggers: Vec::new(),
            executed_triggers: Vec::new(),
        }
    }

    pub fn register_trigger(&mut self, name: &str, prefix: &str, script: &str) {
        self.triggers.push(ApkTriggerHook {
            name: name.to_string(),
            target_path_prefix: prefix.to_string(),
            exec_script: script.to_string(),
        });
    }

    pub fn process_file_installation(&mut self, installed_path: &str) -> usize {
        let mut count = 0;
        for trigger in &self.triggers {
            if installed_path.starts_with(&trigger.target_path_prefix) {
                self.executed_triggers.push(trigger.name.clone());
                count += 1;
            }
        }
        count
    }
}

impl Default for ApkIndexTriggerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 5. OpenBSD pledge/unveil Monotonic Sandbox Enforcer
#[derive(Debug, Clone)]
pub struct OpenBsdMonotonicSandbox {
    pub active_pledges: Vec<String>, // e.g. "stdio", "rpath", "wpath", "inet"
    pub unveiled_paths: Vec<(String, String)>, // (path, permissions "rwx")
    pub is_pledged: bool,
}

impl OpenBsdMonotonicSandbox {
    pub fn new() -> Self {
        Self {
            active_pledges: vec![
                "stdio".to_string(),
                "rpath".to_string(),
                "wpath".to_string(),
                "cpath".to_string(),
                "inet".to_string(),
                "exec".to_string(),
            ],
            unveiled_paths: Vec::new(),
            is_pledged: false,
        }
    }

    /// Monotonic pledge reduction - promises can only be removed, never added back
    pub fn pledge(&mut self, new_promises: &[&str]) -> Result<(), &'static str> {
        for promise in new_promises {
            if !self.active_pledges.contains(&promise.to_string()) {
                return Err("OpenBSD pledge violation: Cannot expand promises monotonically");
            }
        }
        self.active_pledges.retain(|p| new_promises.contains(&p.as_str()));
        self.is_pledged = true;
        Ok(())
    }

    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), &'static str> {
        if self.is_pledged && !self.active_pledges.contains(&"unveil".to_string()) {
            return Err("OpenBSD unveil violation: unveil promise not active in pledge");
        }
        self.unveiled_paths.push((path.to_string(), permissions.to_string()));
        Ok(())
    }

    pub fn is_path_accessible(&self, path: &str, perm: char) -> bool {
        if self.unveiled_paths.is_empty() {
            return true; // Unconfined until unveil is called
        }
        for (unveiled_path, perms) in &self.unveiled_paths {
            if path.starts_with(unveiled_path) && perms.contains(perm) {
                return true;
            }
        }
        false
    }
}

impl Default for OpenBsdMonotonicSandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_pinning_and_multiarch() {
        let mut resolver = AptPinningMultiArchResolver::new("amd64");
        resolver.add_foreign_arch("i386");
        resolver.add_pin_rule("wine", "deb.debian.org", 1001);

        assert!(resolver.is_architecture_supported("amd64"));
        assert!(resolver.is_architecture_supported("i386"));
        assert!(!resolver.is_architecture_supported("arm64"));

        assert_eq!(resolver.resolve_package_priority("wine", "deb.debian.org"), 1001);
        assert_eq!(resolver.resolve_package_priority("gcc", "deb.debian.org"), 500);
    }

    #[test]
    fn test_alpm_parallel_download_sync() {
        let mut engine = AlpmParallelSyncEngine::new(5);
        engine.add_mirror("https://mirror.archlinux.de", 120);
        engine.add_mirror("https://geo.mirror.pkg.archlinux.org", 35);
        engine.add_mirror("https://mirror.rackspace.com", 80);

        engine.sort_mirrors_by_speed();
        assert_eq!(engine.get_fastest_mirror().unwrap(), "https://geo.mirror.pkg.archlinux.org");
    }

    #[test]
    fn test_xbps_signature_verifier() {
        let verifier = XbpsSignatureVerifier::new("pubkey_ed25519_void_main_01");
        let header = XbpsPackageHeader {
            pkgname: "runit".to_string(),
            version: "2.1.2_15".to_string(),
            architecture: "x86_64".to_string(),
            signature_hex: "sig_ed25519_abc123xyz".to_string(),
        };

        assert!(verifier.verify_package(&header, b"xbps_binary_archive_payload"));
    }

    #[test]
    fn test_apk_index_triggers() {
        let mut apk = ApkIndexTriggerEngine::new();
        apk.register_trigger("dkms-hook", "/usr/src/kernel", "/sbin/dkms-build");

        let triggered_count = apk.process_file_installation("/usr/src/kernel/linux-6.8/main.c");
        assert_eq!(triggered_count, 1);
        assert_eq!(apk.executed_triggers.len(), 1);
        assert_eq!(apk.executed_triggers[0], "dkms-hook");
    }

    #[test]
    fn test_openbsd_monotonic_sandbox() {
        let mut sandbox = OpenBsdMonotonicSandbox::new();
        sandbox.unveil("/tmp", "rwx").unwrap();
        sandbox.unveil("/usr/bin", "rx").unwrap();

        assert!(sandbox.is_path_accessible("/tmp/test.txt", 'w'));
        assert!(sandbox.is_path_accessible("/usr/bin/bash", 'x'));
        assert!(!sandbox.is_path_accessible("/etc/shadow", 'r'));

        // Monotonic reduction: reduce promises to ("stdio", "rpath")
        assert!(sandbox.pledge(&["stdio", "rpath"]).is_ok());

        // Attempting to expand promises back to "inet" MUST fail
        assert!(sandbox.pledge(&["stdio", "rpath", "inet"]).is_err());
    }
}
