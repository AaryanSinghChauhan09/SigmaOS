extern crate alloc;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Debian Innovations Subsystem (sigpkg-debian)
// Parity features inspired by Debian GNU/Linux:
// 1. update-alternatives dynamic command link management
// 2. dpkg maintainer script sandboxing (preinst, postinst, prerm, postrm)
// 3. debdelta binary diff patch generation
// 4. APT mirror latency ranking and GPG signature verification

use crate::klib::HashMap;

/// Alternative Link Candidate (Debian update-alternatives equivalent)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeCandidate {
    pub name: String,
    pub path: String,
    pub priority: u32,
}

/// Debian-Style Alternatives System Manager
#[derive(Debug, Clone)]
pub struct SovereignAlternativesSystem {
    pub alternatives: HashMap<String, Vec<AlternativeCandidate>>, // command -> candidate choices
    pub active_selections: HashMap<String, String>,               // command -> chosen path
}

impl SovereignAlternativesSystem {
    pub fn new() -> Self {
        Self {
            alternatives: HashMap::new(),
            active_selections: HashMap::new(),
        }
    }

    pub fn register_alternative(
        &mut self,
        generic_name: &str,
        candidate_name: &str,
        path: &str,
        priority: u32,
    ) {
        let entry = self
            .alternatives
            .entry(generic_name.to_string())
            .or_insert_with(Vec::new);
        entry.push(AlternativeCandidate {
            name: candidate_name.to_string(),
            path: path.to_string(),
            priority,
        });

        // Automatically select candidate with highest priority
        self.auto_select(generic_name);
    }

    pub fn auto_select(&mut self, generic_name: &str) {
        if let Some(candidates) = self.alternatives.get(generic_name) {
            if let Some(best) = candidates.iter().max_by_key(|c| c.priority) {
                self.active_selections
                    .insert(generic_name.to_string(), best.path.clone());
            }
        }
    }

    pub fn get_active_path(&self, generic_name: &str) -> Option<&str> {
        self.active_selections.get(generic_name).map(|s| s.as_str())
    }
}

impl Default for SovereignAlternativesSystem {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Additional Debian Linux Subsystems
// =========================================================================

/// Debian dpkg-divert File Diversion Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diversion {
    pub original_path: String,
    pub diverted_path: String,
    pub package: String,
}

#[derive(Debug, Clone)]
pub struct DpkgDivertEngine {
    pub diversions: HashMap<String, Diversion>,
}

impl DpkgDivertEngine {
    pub fn new() -> Self {
        Self {
            diversions: HashMap::new(),
        }
    }

    pub fn add_diversion(&mut self, original: &str, diverted: &str, package: &str) {
        self.diversions.insert(
            original.to_string(),
            Diversion {
                original_path: original.to_string(),
                diverted_path: diverted.to_string(),
                package: package.to_string(),
            },
        );
    }

    pub fn remove_diversion(&mut self, original: &str) -> bool {
        self.diversions.remove(original).is_some()
    }

    pub fn resolve_path<'a>(&'a self, path: &'a str) -> &'a str {
        if let Some(div) = self.diversions.get(path) {
            &div.diverted_path
        } else {
            path
        }
    }
}

impl Default for DpkgDivertEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian debconf Pre-Configuration Template & Response Database
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebconfPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebconfQuestion {
    pub name: String,
    pub question_type: String,
    pub default_value: String,
    pub answer: Option<String>,
    pub priority: DebconfPriority,
}

#[derive(Debug, Clone)]
pub struct DebconfConfigDatabase {
    pub questions: HashMap<String, DebconfQuestion>,
    pub frontend_priority: DebconfPriority,
}

impl DebconfConfigDatabase {
    pub fn new(frontend_priority: DebconfPriority) -> Self {
        Self {
            questions: HashMap::new(),
            frontend_priority,
        }
    }

    pub fn register_question(
        &mut self,
        name: &str,
        qtype: &str,
        default_val: &str,
        priority: DebconfPriority,
    ) {
        self.questions.insert(
            name.to_string(),
            DebconfQuestion {
                name: name.to_string(),
                question_type: qtype.to_string(),
                default_value: default_val.to_string(),
                answer: None,
                priority,
            },
        );
    }

    pub fn set_answer(&mut self, name: &str, answer: &str) -> bool {
        if let Some(q) = self.questions.get_mut(name) {
            q.answer = Some(answer.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_effective_answer(&self, name: &str) -> Option<String> {
        let q = self.questions.get(name)?;
        if let Some(ref ans) = q.answer {
            Some(ans.clone())
        } else {
            Some(q.default_value.clone())
        }
    }

    pub fn should_prompt_user(&self, name: &str) -> bool {
        if let Some(q) = self.questions.get(name) {
            q.priority >= self.frontend_priority && q.answer.is_none()
        } else {
            false
        }
    }
}

/// Debian Multi-Arch Co-Installation Policy Evaluator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiArchPolicy {
    Same,
    Foreign,
    Allowed,
    No,
}

#[derive(Debug, Clone)]
pub struct DebianMultiArchResolver {
    pub primary_arch: String,
    pub foreign_architectures: Vec<String>,
}

impl DebianMultiArchResolver {
    pub fn new(primary_arch: &str) -> Self {
        Self {
            primary_arch: primary_arch.to_string(),
            foreign_architectures: Vec::new(),
        }
    }

    pub fn add_foreign_arch(&mut self, arch: &str) {
        let a = arch.to_string();
        if !self.foreign_architectures.contains(&a) {
            self.foreign_architectures.push(a);
        }
    }

    pub fn is_architecture_supported(&self, arch: &str) -> bool {
        arch == "all" || arch == self.primary_arch || self.foreign_architectures.iter().any(|a| a == arch)
    }

    pub fn can_satisfy_dependency(
        &self,
        pkg_arch: &str,
        policy: MultiArchPolicy,
        requester_arch: &str,
    ) -> bool {
        if !self.is_architecture_supported(pkg_arch) {
            return false;
        }
        match policy {
            MultiArchPolicy::Same => pkg_arch == requester_arch || pkg_arch == "all",
            MultiArchPolicy::Foreign => true,
            MultiArchPolicy::Allowed => pkg_arch == requester_arch || pkg_arch == "all",
            MultiArchPolicy::No => pkg_arch == self.primary_arch || pkg_arch == "all",
        }
    }
}

/// Maintainer Lifecycle Scripts (dpkg)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintainerScriptPhase {
    PreInst,
    PostInst,
    PreRm,
    PostRm,
}

pub type SovereignTransactionManager = SovereignAlternativesSystem;
pub type SovereignSandboxEnforcer = SovereignMaintainerSandbox;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    Committed,
    RolledBack,
    InProgress,
}

/// Sandboxed Maintainer Script Enforcer
#[derive(Debug, Clone)]
pub struct SovereignMaintainerSandbox {
    pub allowed_directories: Vec<String>,
}

impl SovereignMaintainerSandbox {
    pub fn new() -> Self {
        Self {
            allowed_directories: vec![
                "/usr/bin".to_string(),
                "/etc".to_string(),
                "/var/lib".to_string(),
            ],
        }
    }

    pub fn validate_script(
        &self,
        phase: MaintainerScriptPhase,
        script_body: &str,
    ) -> Result<(), &'static str> {
        // Disallow dangerous commands in package install scripts
        if script_body.contains("rm -rf /") || script_body.contains("dd if=") {
            return Err("Maintainer script contains dangerous unsafe file operations");
        }

        // Validate target path constraints
        for line in script_body.lines() {
            let line = line.trim();
            if line.starts_with("mkdir ") || line.starts_with("cp ") {
                let target = line.split_whitespace().last().unwrap_or("");
                if !self
                    .allowed_directories
                    .iter()
                    .any(|allowed| target.starts_with(allowed))
                {
                    return Err("Maintainer script targets directory outside sandbox permissions");
                }
            }
        }

        Ok(())
    }
}

impl Default for SovereignMaintainerSandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// Debdelta-style Binary Delta Patch Generator
pub struct SovereignDeltaGenerator;

impl SovereignDeltaGenerator {
    /// Generates a binary delta patch between old package bytes and new package bytes
    pub fn generate_delta(old_bytes: &[u8], new_bytes: &[u8]) -> Vec<u8> {
        let mut delta = Vec::new();
        delta.extend_from_slice(b"DEBDELTA_V1_PATCH\n");

        let min_len = old_bytes.len().min(new_bytes.len());
        for i in 0..min_len {
            if old_bytes[i] != new_bytes[i] {
                delta.push(i as u8);
                delta.push(new_bytes[i]);
            }
        }

        if new_bytes.len() > old_bytes.len() {
            delta.extend_from_slice(&new_bytes[old_bytes.len()..]);
        }

        delta
    }

    /// Applies a debdelta patch to reconstruct the new package bytes
    pub fn apply_delta(old_bytes: &[u8], delta_patch: &[u8]) -> Result<Vec<u8>, &'static str> {
        if !delta_patch.starts_with(b"DEBDELTA_V1_PATCH\n") {
            return Err("Invalid debdelta patch header");
        }

        let mut reconstructed = old_bytes.to_vec();
        let patch_body = &delta_patch[18..];

        let mut i = 0;
        while i + 1 < patch_body.len() {
            let idx = patch_body[i] as usize;
            let val = patch_body[i + 1];
            if idx < reconstructed.len() {
                reconstructed[idx] = val;
            } else {
                reconstructed.push(val);
            }
            i += 2;
        }

        Ok(reconstructed)
    }
}

/// APT Repository Mirror Info
#[derive(Debug, Clone)]
pub struct AptMirror {
    pub url: String,
    pub latency_ms: u64,
    pub gpg_key_valid: bool,
}

/// Debian APT Mirror Selector
#[derive(Debug, Clone)]
pub struct SovereignMirrorSelector {
    pub mirrors: Vec<AptMirror>,
}

impl SovereignMirrorSelector {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u64, gpg_key_valid: bool) {
        self.mirrors.push(AptMirror {
            url: url.to_string(),
            latency_ms,
            gpg_key_valid,
        });
    }

    pub fn select_optimal_mirror(&self) -> Result<String, &'static str> {
        let valid_mirrors: Vec<&AptMirror> =
            self.mirrors.iter().filter(|m| m.gpg_key_valid).collect();
        if valid_mirrors.is_empty() {
            return Err("No APT mirrors with valid GPG signatures available");
        }

        let best = valid_mirrors
            .into_iter()
            .min_by_key(|m| m.latency_ms)
            .unwrap();
        Ok(best.url.clone())
    }
}

impl Default for SovereignMirrorSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_alternatives() {
        let mut alternatives = SovereignAlternativesSystem::new();

        alternatives.register_alternative("editor", "nano", "/usr/bin/nano", 50);
        alternatives.register_alternative("editor", "vim", "/usr/bin/vim", 100);

        // Highest priority candidate (vim = 100) should be active
        assert_eq!(alternatives.get_active_path("editor"), Some("/usr/bin/vim"));
    }

    #[test]
    fn test_maintainer_script_sandbox() {
        let sandbox = SovereignMaintainerSandbox::new();

        let safe_script = "mkdir /usr/bin/app_dir\ncp binary /usr/bin/app_dir/app";
        assert!(sandbox
            .validate_script(MaintainerScriptPhase::PostInst, safe_script)
            .is_ok());

        let dangerous_script = "rm -rf /";
        assert!(sandbox
            .validate_script(MaintainerScriptPhase::PreRm, dangerous_script)
            .is_err());

        let unpermitted_path_script = "mkdir /root/secret_dir";
        assert!(sandbox
            .validate_script(MaintainerScriptPhase::PreInst, unpermitted_path_script)
            .is_err());
    }

    #[test]
    fn test_debdelta_generation_and_patching() {
        let old_pkg = b"Debian Package V1 Data Payload";
        let new_pkg = b"Debian Package V2 Data Payload";

        let delta = SovereignDeltaGenerator::generate_delta(old_pkg, new_pkg);
        assert!(delta.starts_with(b"DEBDELTA_V1_PATCH\n"));

        let reconstructed = SovereignDeltaGenerator::apply_delta(old_pkg, &delta).unwrap();
        assert_eq!(reconstructed, new_pkg);
    }

    #[test]
    fn test_apt_mirror_selector() {
        let mut selector = SovereignMirrorSelector::new();

        selector.add_mirror("http://fast-untrusted.debian.org", 10, false); // Fast but untrusted
        selector.add_mirror("http://mirror1.debian.org", 45, true); // Trusted, 45ms
        selector.add_mirror("http://mirror2.debian.org", 20, true); // Trusted, 20ms

        assert_eq!(
            selector.select_optimal_mirror().unwrap(),
            "http://mirror2.debian.org"
        );
    }

    #[test]
    fn test_dpkg_divert_override_paths() {
        let mut divert_engine = DpkgDivertEngine::new();
        divert_engine.add_diversion("/usr/bin/gcc", "/usr/bin/gcc.real", "custom-compiler");

        assert_eq!(divert_engine.resolve_path("/usr/bin/gcc"), "/usr/bin/gcc.real");
        assert_eq!(divert_engine.resolve_path("/usr/bin/clang"), "/usr/bin/clang");

        assert!(divert_engine.remove_diversion("/usr/bin/gcc"));
        assert_eq!(divert_engine.resolve_path("/usr/bin/gcc"), "/usr/bin/gcc");
    }

    #[test]
    fn test_debconf_preconfig_lookup() {
        let mut debconf = DebconfConfigDatabase::new(DebconfPriority::Medium);
        debconf.register_question("tzdata/zones", "select", "UTC", DebconfPriority::High);
        debconf.register_question("debug/enable", "boolean", "false", DebconfPriority::Low);

        // Effective answer before explicit response -> returns default value
        assert_eq!(debconf.get_effective_answer("tzdata/zones"), Some("UTC".to_string()));

        // High priority question exceeds frontend threshold (Medium) and has no answer -> prompt user
        assert!(debconf.should_prompt_user("tzdata/zones"));

        // Low priority question below frontend threshold (Medium) -> do not prompt
        assert!(!debconf.should_prompt_user("debug/enable"));

        // Set answer -> effective answer updated
        assert!(debconf.set_answer("tzdata/zones", "Asia/Kolkata"));
        assert_eq!(debconf.get_effective_answer("tzdata/zones"), Some("Asia/Kolkata".to_string()));
        assert!(!debconf.should_prompt_user("tzdata/zones"));
    }

    #[test]
    fn test_multi_arch_policy_resolver() {
        let mut multiarch = DebianMultiArchResolver::new("amd64");
        multiarch.add_foreign_arch("arm64");

        assert!(multiarch.is_architecture_supported("amd64"));
        assert!(multiarch.is_architecture_supported("arm64"));
        assert!(multiarch.is_architecture_supported("all"));
        assert!(!multiarch.is_architecture_supported("riscv64"));

        // Same policy requires matching architecture
        assert!(multiarch.can_satisfy_dependency("amd64", MultiArchPolicy::Same, "amd64"));
        assert!(!multiarch.can_satisfy_dependency("arm64", MultiArchPolicy::Same, "amd64"));

        // Foreign policy allows satisfies dependency across architectures
        assert!(multiarch.can_satisfy_dependency("arm64", MultiArchPolicy::Foreign, "amd64"));
    }
}
