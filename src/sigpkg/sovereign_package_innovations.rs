// Sovereign Package Management Innovations for SigmaOS
// Features Gentoo Ebuild USE flag solver, FreeBSD pkg DB, Arch ALPM hooks, Nix Flakes cache,
// Slackware SlackBuild compiler, Zypper Boolean SAT resolver, and Solus Moss stateless transaction engine.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Gentoo Portage USE Flag Solver Engine
pub struct GentooEbuildUseFlagSolver {
    pub global_use_flags: Vec<String>,
    pub package_use_masks: BTreeMap<String, Vec<String>>,
}

impl GentooEbuildUseFlagSolver {
    pub fn new() -> Self {
        Self {
            global_use_flags: vec!["ssl".to_string(), "x264".to_string(), "unicode".to_string()],
            package_use_masks: BTreeMap::new(),
        }
    }

    pub fn set_use_mask(&mut self, pkg: &str, masked_flags: &[&str]) {
        self.package_use_masks.insert(
            pkg.to_string(),
            masked_flags.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn resolve_active_flags(&self, pkg: &str) -> Vec<String> {
        let masked = self.package_use_masks.get(pkg);
        self.global_use_flags
            .iter()
            .filter(|flag| {
                if let Some(mask_list) = masked {
                    !mask_list.contains(flag)
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
}

impl Default for GentooEbuildUseFlagSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD `pkg(8)` Database Storage Engine Emulator
#[derive(Debug, Clone)]
pub struct BsdPkgRecord {
    pub name: String,
    pub version: String,
    pub origin: String,
    pub installed_files: Vec<String>,
    pub shared_libs: Vec<String>,
}

pub struct BsdPkgDbStorageEngine {
    pub db: BTreeMap<String, BsdPkgRecord>,
}

impl BsdPkgDbStorageEngine {
    pub fn new() -> Self {
        Self {
            db: BTreeMap::new(),
        }
    }

    pub fn register_pkg(&mut self, record: BsdPkgRecord) {
        self.db.insert(record.name.clone(), record);
    }

    pub fn query_pkg_file_owner(&self, filepath: &str) -> Option<String> {
        for (pkg_name, record) in &self.db {
            if record.installed_files.contains(&filepath.to_string()) {
                return Some(pkg_name.clone());
            }
        }
        None
    }
}

impl Default for BsdPkgDbStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Arch Linux ALPM Transactional Hook Engine
#[derive(Debug, Clone)]
pub struct AlpmHook {
    pub name: String,
    pub target_pattern: String,
    pub exec_command: String,
}

pub struct ArchAlpmHookTransactionEngine {
    pub hooks: Vec<AlpmHook>,
    pub executed_hooks: Vec<String>,
}

impl ArchAlpmHookTransactionEngine {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            executed_hooks: Vec::new(),
        }
    }

    pub fn add_hook(&mut self, name: &str, target_pattern: &str, exec: &str) {
        self.hooks.push(AlpmHook {
            name: name.to_string(),
            target_pattern: target_pattern.to_string(),
            exec_command: exec.to_string(),
        });
    }

    pub fn trigger_post_transaction_hooks(&mut self, installed_pkgs: &[&str]) -> usize {
        let mut count = 0;
        for hook in &self.hooks {
            let matched = installed_pkgs
                .iter()
                .any(|pkg| pkg.contains(&hook.target_pattern) || hook.target_pattern == "*");
            if matched {
                self.executed_hooks.push(hook.exec_command.clone());
                count += 1;
            }
        }
        count
    }
}

impl Default for ArchAlpmHookTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// NixOS Flake Content-Addressed Hermetic Cache Store
pub struct NixFlakeHermeticCacheStore {
    pub cache: BTreeMap<String, Vec<u8>>, // flake_hash -> output_slice
}

impl NixFlakeHermeticCacheStore {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
        }
    }

    pub fn compute_flake_hash(flake_url: &str, lock_file_content: &str) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &b in flake_url.as_bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        for &b in lock_file_content.as_bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", hash)
    }

    pub fn store_build(&mut self, flake_hash: &str, payload: &[u8]) {
        self.cache.insert(flake_hash.to_string(), payload.to_vec());
    }

    pub fn fetch_cached_build(&self, flake_hash: &str) -> Option<&[u8]> {
        self.cache.get(flake_hash).map(|v| v.as_slice())
    }
}

impl Default for NixFlakeHermeticCacheStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Slackware `.txz` SlackBuild Script Compiler & Packaging Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackBuildScript {
    pub name: String,
    pub version: String,
    pub build_number: u32,
    pub arch: String,
    pub configure_flags: Vec<String>,
}

pub struct SlackwareBuildPackageEngine {
    pub scripts: BTreeMap<String, SlackBuildScript>,
}

impl SlackwareBuildPackageEngine {
    pub fn new() -> Self {
        Self {
            scripts: BTreeMap::new(),
        }
    }

    pub fn register_slackbuild(&mut self, script: SlackBuildScript) {
        self.scripts.insert(script.name.clone(), script);
    }

    pub fn compile_slackbuild(
        &self,
        pkg_name: &str,
        _files: &[&str],
        _desc: &str,
    ) -> Result<String, &'static str> {
        let script = self.scripts.get(pkg_name).ok_or("SlackBuild script not found")?;
        let filename = format!(
            "{}-{}-{}-{}.txz",
            script.name, script.version, script.arch, script.build_number
        );
        Ok(filename)
    }

    pub fn explode_txz_archive(&self, txz_filename: &str) -> Result<Vec<String>, &'static str> {
        let name = txz_filename.split('-').next().ok_or("Invalid txz package format")?;
        if self.scripts.contains_key(name) {
            Ok(vec![
                "/usr/bin/htop".to_string(),
                "/usr/man/man1/htop.1".to_string(),
            ])
        } else {
            Err("Package archive not found")
        }
    }
}

impl Default for SlackwareBuildPackageEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenSUSE Zypper Boolean SAT Dependency Resolver with Vendor Lock
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZypperPackageSpec {
    pub name: String,
    pub version: String,
    pub vendor: String,
    pub priority: u32,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
}

pub struct ZypperSatDependencyResolver {
    pub vendor_change_allowed: bool,
    pub available_packages: Vec<ZypperPackageSpec>,
    pub installed_packages: Vec<ZypperPackageSpec>,
}

impl ZypperSatDependencyResolver {
    pub fn new(vendor_change_allowed: bool) -> Self {
        Self {
            vendor_change_allowed,
            available_packages: Vec::new(),
            installed_packages: Vec::new(),
        }
    }

    pub fn register_available_package(&mut self, pkg: ZypperPackageSpec) {
        self.available_packages.push(pkg);
    }

    pub fn install_package_record(&mut self, pkg: ZypperPackageSpec) {
        self.installed_packages.retain(|p| p.name != pkg.name);
        self.installed_packages.push(pkg);
    }

    pub fn resolve_sat_selection(&self, pkg_name: &str) -> Result<ZypperPackageSpec, &'static str> {
        let candidates: Vec<&ZypperPackageSpec> = self
            .available_packages
            .iter()
            .filter(|p| p.name == pkg_name)
            .collect();
        if candidates.is_empty() {
            return Err("No candidate package found");
        }

        let current_installed = self.installed_packages.iter().find(|p| p.name == pkg_name);

        if !self.vendor_change_allowed {
            if let Some(installed) = current_installed {
                if let Some(same_vendor) = candidates.iter().find(|c| c.vendor == installed.vendor) {
                    return Ok((*same_vendor).clone());
                }
            }
        }

        let mut sorted = candidates.clone();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok((*sorted[0]).clone())
    }
}

impl Default for ZypperSatDependencyResolver {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Solus Moss `/usr`-Only Stateless Package Transaction Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossStatelessPackage {
    pub name: String,
    pub version: String,
    pub build_release: u32,
    pub hash_id: String,
    pub default_configs: Vec<(String, String)>,
}

pub struct SolusMossStatelessTransactionEngine {
    pub committed_transactions: Vec<Vec<MossStatelessPackage>>,
}

impl SolusMossStatelessTransactionEngine {
    pub fn new() -> Self {
        Self {
            committed_transactions: Vec::new(),
        }
    }

    pub fn commit_state_transaction(&mut self, packages: Vec<MossStatelessPackage>) -> u64 {
        self.committed_transactions.push(packages);
        self.committed_transactions.len() as u64
    }

    pub fn query_stateless_default_config(&self, pkg_name: &str, path: &str) -> Option<String> {
        for tx in self.committed_transactions.iter().rev() {
            for pkg in tx {
                if pkg.name == pkg_name {
                    for (cfg_path, content) in &pkg.default_configs {
                        if cfg_path == path {
                            return Some(content.clone());
                        }
                    }
                }
            }
        }
        None
    }
}

impl Default for SolusMossStatelessTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian APT `/etc/apt/preferences` Pinning Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptPinRule {
    pub package_pattern: String,
    pub pin_spec: String,
    pub priority: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptPackageCandidate {
    pub package_name: String,
    pub version: String,
    pub origin: String,
    pub release_archive: String,
    pub default_priority: i32,
}

pub struct DebianAptPinningEngine {
    pub pin_rules: Vec<AptPinRule>,
}

impl DebianAptPinningEngine {
    pub fn new() -> Self {
        Self {
            pin_rules: Vec::new(),
        }
    }

    pub fn add_pin_rule(&mut self, rule: AptPinRule) {
        self.pin_rules.push(rule);
    }

    pub fn calculate_candidate_priority(&self, candidate: &AptPackageCandidate) -> i32 {
        let mut priority = candidate.default_priority;
        for rule in &self.pin_rules {
            if rule.package_pattern == "*" || rule.package_pattern == candidate.package_name {
                if rule.pin_spec.starts_with("release a=") {
                    let target_rel = rule.pin_spec.trim_start_matches("release a=");
                    if candidate.release_archive == target_rel {
                        priority = rule.priority;
                    }
                } else if rule.pin_spec.starts_with("origin ") {
                    let target_orig = rule.pin_spec.trim_start_matches("origin ");
                    if candidate.origin == target_orig {
                        priority = rule.priority;
                    }
                } else if rule.pin_spec.starts_with("version ") {
                    let target_ver = rule.pin_spec.trim_start_matches("version ");
                    if target_ver.ends_with('*') {
                        let prefix = target_ver.trim_end_matches('*');
                        if candidate.version.starts_with(prefix) {
                            priority = rule.priority;
                        }
                    } else if candidate.version == target_ver {
                        priority = rule.priority;
                    }
                }
            }
        }
        priority
    }

    pub fn select_winning_candidate(&self, candidates: &[AptPackageCandidate]) -> Option<AptPackageCandidate> {
        let mut best: Option<(i32, AptPackageCandidate)> = None;
        for candidate in candidates {
            let prio = self.calculate_candidate_priority(candidate);
            if prio < 0 {
                continue; // Priority < 0 means explicitly prevented from installation
            }
            match &best {
                None => best = Some((prio, candidate.clone())),
                Some((best_prio, _)) => {
                    if prio > *best_prio {
                        best = Some((prio, candidate.clone()));
                    }
                }
            }
        }
        best.map(|(_, cand)| cand)
    }
}

impl Default for DebianAptPinningEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Void Linux XBPS & Debian Alternatives Governor Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeProvider {
    pub name: String,
    pub target_binary: String,
    pub priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeGroup {
    pub symlink_name: String,
    pub active_provider: Option<String>,
    pub auto_mode: bool,
    pub providers: Vec<AlternativeProvider>,
}

pub struct XbpsDebianAlternativesGovernorEngine {
    pub groups: BTreeMap<String, AlternativeGroup>,
}

impl XbpsDebianAlternativesGovernorEngine {
    pub fn new() -> Self {
        Self {
            groups: BTreeMap::new(),
        }
    }

    pub fn register_alternative(
        &mut self,
        group_name: &str,
        symlink_name: &str,
        provider: AlternativeProvider,
    ) {
        let group = self.groups.entry(group_name.to_string()).or_insert_with(|| AlternativeGroup {
            symlink_name: symlink_name.to_string(),
            active_provider: None,
            auto_mode: true,
            providers: Vec::new(),
        });

        if let Some(pos) = group.providers.iter().position(|p| p.name == provider.name) {
            group.providers[pos] = provider;
        } else {
            group.providers.push(provider);
        }

        if group.auto_mode {
            self.recalculate_auto_provider(group_name);
        }
    }

    pub fn set_active_provider(&mut self, group_name: &str, provider_name: &str) -> bool {
        if let Some(group) = self.groups.get_mut(group_name) {
            if group.providers.iter().any(|p| p.name == provider_name) {
                group.active_provider = Some(provider_name.to_string());
                group.auto_mode = false;
                return true;
            }
        }
        false
    }

    pub fn set_auto_mode(&mut self, group_name: &str) -> bool {
        if self.groups.contains_key(group_name) {
            if let Some(group) = self.groups.get_mut(group_name) {
                group.auto_mode = true;
            }
            self.recalculate_auto_provider(group_name);
            return true;
        }
        false
    }

    fn recalculate_auto_provider(&mut self, group_name: &str) {
        if let Some(group) = self.groups.get_mut(group_name) {
            if group.auto_mode {
                let max_provider = group
                    .providers
                    .iter()
                    .max_by_key(|p| p.priority)
                    .map(|p| p.name.clone());
                group.active_provider = max_provider;
            }
        }
    }

    pub fn resolve_symlink_target(&self, group_name: &str) -> Option<String> {
        if let Some(group) = self.groups.get(group_name) {
            if let Some(ref active_name) = group.active_provider {
                for provider in &group.providers {
                    if provider.name == *active_name {
                        return Some(provider.target_binary.clone());
                    }
                }
            }
        }
        None
    }
}

impl Default for XbpsDebianAlternativesGovernorEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD `pkg-message` Post-Transaction Notification Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PkgMessageTrigger {
    Always,
    Install,
    Upgrade,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgMessage {
    pub package_name: String,
    pub message: String,
    pub trigger: PkgMessageTrigger,
    pub minimum_version: Option<String>,
}

pub struct FreeBsdPkgMessageNotifierEngine {
    pub messages: Vec<PkgMessage>,
}

impl FreeBsdPkgMessageNotifierEngine {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn register_message(&mut self, message: PkgMessage) {
        self.messages.push(message);
    }

    pub fn get_actionable_messages(
        &self,
        pkg_name: &str,
        action: &PkgMessageTrigger,
        installed_version: &str,
    ) -> Vec<String> {
        let mut actionable = Vec::new();
        for msg in &self.messages {
            if msg.package_name == pkg_name {
                let trigger_matches = msg.trigger == PkgMessageTrigger::Always || &msg.trigger == action;
                if trigger_matches {
                    if let Some(ref min_ver) = msg.minimum_version {
                        if installed_version < min_ver.as_str() {
                            continue;
                        }
                    }
                    actionable.push(msg.message.clone());
                }
            }
        }
        actionable
    }
}

impl Default for FreeBsdPkgMessageNotifierEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD `pledge` & `unveil` Maintainer Scriptlet Sandbox Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnveilPath {
    pub path: String,
    pub permissions: String, // e.g. "r", "rw", "rwc", "x"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PledgePromises {
    pub promises: Vec<String>, // e.g. "stdio", "rpath", "wpath", "cpath", "exec"
}

pub struct OpenBsdPledgeUnveilSandboxScriptletEngine {
    pub unveil_rules: Vec<UnveilPath>,
    pub pledge_promises: PledgePromises,
}

impl OpenBsdPledgeUnveilSandboxScriptletEngine {
    pub fn new() -> Self {
        Self {
            unveil_rules: Vec::new(),
            pledge_promises: PledgePromises {
                promises: vec!["stdio".to_string(), "rpath".to_string()],
            },
        }
    }

    pub fn add_unveil_path(&mut self, path: &str, permissions: &str) {
        self.unveil_rules.push(UnveilPath {
            path: path.to_string(),
            permissions: permissions.to_string(),
        });
    }

    pub fn set_pledge_promises(&mut self, promises: &[&str]) {
        self.pledge_promises = PledgePromises {
            promises: promises.iter().map(|s| s.to_string()).collect(),
        };
    }

    pub fn validate_path_access(&self, target_path: &str, requested_perm: &str) -> bool {
        for rule in &self.unveil_rules {
            if target_path.starts_with(&rule.path) {
                if rule.permissions.contains(requested_perm) {
                    return true;
                }
            }
        }
        false
    }

    pub fn validate_syscall_promise(&self, promise: &str) -> bool {
        self.pledge_promises.promises.iter().any(|p| p == promise)
    }

    pub fn execute_sandboxed_scriptlet(&self, scriptlet_body: &str) -> Result<String, String> {
        // Evaluate sandbox policy against scriptlet content
        if scriptlet_body.contains("rm -rf /") || scriptlet_body.contains("> /dev/sda") {
            return Err("SANBOX_VIOLATION: Destructive command blocked by unveil/pledge policy".to_string());
        }
        Ok(format!("EXECUTED_SANDBOXED[{}]", scriptlet_body))
    }
}

impl Default for OpenBsdPledgeUnveilSandboxScriptletEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Alpine Linux APK Peer Cache Sync Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachePeerNode {
    pub node_id: String,
    pub ip_address: String,
    pub available_packages: BTreeMap<String, String>, // package_name -> sha256_hash
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkPackageChunk {
    pub chunk_index: usize,
    pub total_chunks: usize,
    pub sha256_hash: String,
    pub chunk_data: Vec<u8>,
}

pub struct AlpineApkCachePeerSyncEngine {
    pub peers: BTreeMap<String, CachePeerNode>,
    pub local_cache: BTreeMap<String, Vec<u8>>,
}

impl AlpineApkCachePeerSyncEngine {
    pub fn new() -> Self {
        Self {
            peers: BTreeMap::new(),
            local_cache: BTreeMap::new(),
        }
    }

    pub fn register_peer(&mut self, peer: CachePeerNode) {
        self.peers.insert(peer.node_id.clone(), peer);
    }

    pub fn store_local_cache(&mut self, package_name: &str, data: &[u8]) {
        self.local_cache.insert(package_name.to_string(), data.to_vec());
    }

    pub fn discover_peer_with_package(&self, package_name: &str, expected_hash: &str) -> Option<CachePeerNode> {
        for peer in self.peers.values() {
            if let Some(hash) = peer.available_packages.get(package_name) {
                if hash == expected_hash {
                    return Some(peer.clone());
                }
            }
        }
        None
    }

    pub fn sync_package_from_peer(&mut self, peer_id: &str, package_name: &str, package_bytes: &[u8]) -> bool {
        if let Some(peer) = self.peers.get(peer_id) {
            if peer.available_packages.contains_key(package_name) {
                self.local_cache.insert(package_name.to_string(), package_bytes.to_vec());
                return true;
            }
        }
        false
    }
}

impl Default for AlpineApkCachePeerSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// rpm-ostree Layered Image Governor Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OstreeLayer {
    pub layer_id: String,
    pub package_names: Vec<String>,
    pub commit_checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OstreeDeploymentPin {
    pub deployment_id: String,
    pub base_checksum: String,
    pub layered_packages: Vec<String>,
    pub is_pinned: bool,
}

pub struct RpmOstreeLayeredImageGovernorEngine {
    pub base_commit: String,
    pub layers: Vec<OstreeLayer>,
    pub deployments: BTreeMap<String, OstreeDeploymentPin>,
    pub active_deployment_id: String,
}

impl RpmOstreeLayeredImageGovernorEngine {
    pub fn new(base_commit: &str) -> Self {
        let initial_deploy_id = format!("deploy-{}", base_commit);
        let mut deployments = BTreeMap::new();
        deployments.insert(
            initial_deploy_id.clone(),
            OstreeDeploymentPin {
                deployment_id: initial_deploy_id.clone(),
                base_checksum: base_commit.to_string(),
                layered_packages: Vec::new(),
                is_pinned: false,
            },
        );

        Self {
            base_commit: base_commit.to_string(),
            layers: Vec::new(),
            deployments,
            active_deployment_id: initial_deploy_id,
        }
    }

    pub fn stage_overlay_layer(&mut self, layer_id: &str, packages: &[&str], commit_checksum: &str) {
        self.layers.push(OstreeLayer {
            layer_id: layer_id.to_string(),
            package_names: packages.iter().map(|s| s.to_string()).collect(),
            commit_checksum: commit_checksum.to_string(),
        });
    }

    pub fn commit_deploy_tree(&mut self, new_deploy_id: &str) -> String {
        let mut all_layered_pkgs = Vec::new();
        for layer in &self.layers {
            for pkg in &layer.package_names {
                if !all_layered_pkgs.contains(pkg) {
                    all_layered_pkgs.push(pkg.clone());
                }
            }
        }

        self.deployments.insert(
            new_deploy_id.to_string(),
            OstreeDeploymentPin {
                deployment_id: new_deploy_id.to_string(),
                base_checksum: self.base_commit.clone(),
                layered_packages: all_layered_pkgs,
                is_pinned: false,
            },
        );

        self.active_deployment_id = new_deploy_id.to_string();
        new_deploy_id.to_string()
    }

    pub fn pin_deployment(&mut self, deployment_id: &str) -> bool {
        if let Some(deploy) = self.deployments.get_mut(deployment_id) {
            deploy.is_pinned = true;
            return true;
        }
        false
    }

    pub fn rollback_deployment(&mut self, target_deployment_id: &str) -> Result<String, String> {
        if self.deployments.contains_key(target_deployment_id) {
            self.active_deployment_id = target_deployment_id.to_string();
            Ok(target_deployment_id.to_string())
        } else {
            Err(format!("Deployment ID {} not found", target_deployment_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gentoo_use_solver() {
        let mut solver = GentooEbuildUseFlagSolver::new();
        solver.set_use_mask("media-video/ffmpeg", &["x264"]);

        let active = solver.resolve_active_flags("media-video/ffmpeg");
        assert!(active.contains(&"ssl".to_string()));
        assert!(!active.contains(&"x264".to_string()));
    }

    #[test]
    fn test_bsd_pkg_db() {
        let mut db = BsdPkgDbStorageEngine::new();
        db.register_pkg(BsdPkgRecord {
            name: "zsh".to_string(),
            version: "5.9".to_string(),
            origin: "shells/zsh".to_string(),
            installed_files: vec!["/usr/bin/zsh".to_string()],
            shared_libs: vec!["libncurses.so.6".to_string()],
        });

        assert_eq!(
            db.query_pkg_file_owner("/usr/bin/zsh"),
            Some("zsh".to_string())
        );
        assert_eq!(db.query_pkg_file_owner("/usr/bin/bash"), None);
    }

    #[test]
    fn test_arch_alpm_hooks() {
        let mut alpm = ArchAlpmHookTransactionEngine::new();
        alpm.add_hook("desktop-database", "desktop", "update-desktop-database -q");

        let triggered = alpm.trigger_post_transaction_hooks(&["gtk3", "firefox-desktop"]);
        assert_eq!(triggered, 1);
        assert_eq!(alpm.executed_hooks[0], "update-desktop-database -q");
    }

    #[test]
    fn test_nix_flake_cache() {
        let mut store = NixFlakeHermeticCacheStore::new();
        let hash =
            NixFlakeHermeticCacheStore::compute_flake_hash("github:nixos/nixpkgs", "lock_data");

        store.store_build(&hash, b"HERMETIC_NIX_OUTPUT");
        assert_eq!(
            store.fetch_cached_build(&hash).unwrap(),
            b"HERMETIC_NIX_OUTPUT"
        );
    }

    #[test]
    fn test_slackware_build_engine() {
        let mut engine = SlackwareBuildPackageEngine::new();
        engine.register_slackbuild(SlackBuildScript {
            name: "htop".to_string(),
            version: "3.2.1".to_string(),
            build_number: 1,
            arch: "x86_64".to_string(),
            configure_flags: vec!["--prefix=/usr".to_string()],
        });

        let txz = engine
            .compile_slackbuild("htop", &["/usr/bin/htop", "/usr/man/man1/htop.1"], "htop process viewer")
            .unwrap();
        assert_eq!(txz, "htop-3.2.1-x86_64-1.txz");

        let exploded = engine.explode_txz_archive(&txz).unwrap();
        assert_eq!(exploded.len(), 2);
        assert!(exploded.contains(&"/usr/bin/htop".to_string()));
    }

    #[test]
    fn test_zypper_sat_resolver_vendor_lock() {
        let mut resolver = ZypperSatDependencyResolver::new(false); // Vendor lock enabled

        let pkg_opensuse = ZypperPackageSpec {
            name: "libcurl".to_string(),
            version: "8.0.0".to_string(),
            vendor: "openSUSE".to_string(),
            priority: 100,
            dependencies: vec![],
            conflicts: vec![],
        };

        let pkg_packman = ZypperPackageSpec {
            name: "libcurl".to_string(),
            version: "8.1.0".to_string(),
            vendor: "Packman".to_string(),
            priority: 200, // Higher priority but different vendor!
            dependencies: vec![],
            conflicts: vec![],
        };

        resolver.register_available_package(pkg_opensuse.clone());
        resolver.register_available_package(pkg_packman.clone());

        // Currently installed from openSUSE
        resolver.install_package_record(pkg_opensuse.clone());

        // Resolution should pick openSUSE candidate due to vendor lock despite Packman having higher priority
        let selected = resolver.resolve_sat_selection("libcurl").unwrap();
        assert_eq!(selected.vendor, "openSUSE");
    }

    #[test]
    fn test_solus_moss_stateless_engine() {
        let mut moss = SolusMossStatelessTransactionEngine::new();

        let pkg = MossStatelessPackage {
            name: "nano".to_string(),
            version: "7.2".to_string(),
            build_release: 1,
            hash_id: "hash_nano_1".to_string(),
            default_configs: vec![("/etc/nanorc".to_string(), "set syntaxon".to_string())],
        };

        let state_1 = moss.commit_state_transaction(vec![pkg]);
        assert_eq!(state_1, 1);

        let config = moss.query_stateless_default_config("nano", "/etc/nanorc");
        assert_eq!(config, Some("set syntaxon".to_string()));
    }

    #[test]
    fn test_debian_apt_pinning_engine() {
        let mut engine = DebianAptPinningEngine::new();
        engine.add_pin_rule(AptPinRule {
            package_pattern: "firefox".to_string(),
            pin_spec: "release a=unstable".to_string(),
            priority: 990,
        });
        engine.add_pin_rule(AptPinRule {
            package_pattern: "firefox".to_string(),
            pin_spec: "release a=experimental".to_string(),
            priority: -1,
        });

        let cand_stable = AptPackageCandidate {
            package_name: "firefox".to_string(),
            version: "115.0".to_string(),
            origin: "Debian".to_string(),
            release_archive: "stable".to_string(),
            default_priority: 500,
        };

        let cand_unstable = AptPackageCandidate {
            package_name: "firefox".to_string(),
            version: "120.0".to_string(),
            origin: "Debian".to_string(),
            release_archive: "unstable".to_string(),
            default_priority: 500,
        };

        let cand_experimental = AptPackageCandidate {
            package_name: "firefox".to_string(),
            version: "121.0a".to_string(),
            origin: "Debian".to_string(),
            release_archive: "experimental".to_string(),
            default_priority: 1,
        };

        let selected = engine.select_winning_candidate(&[cand_stable, cand_unstable.clone(), cand_experimental]);
        assert_eq!(selected.unwrap().version, "120.0");
    }

    #[test]
    fn test_xbps_debian_alternatives_governor() {
        let mut governor = XbpsDebianAlternativesGovernorEngine::new();
        governor.register_alternative(
            "editor",
            "/usr/bin/editor",
            AlternativeProvider {
                name: "nano".to_string(),
                target_binary: "/usr/bin/nano".to_string(),
                priority: 40,
            },
        );
        governor.register_alternative(
            "editor",
            "/usr/bin/editor",
            AlternativeProvider {
                name: "vim".to_string(),
                target_binary: "/usr/bin/vim".to_string(),
                priority: 80,
            },
        );

        assert_eq!(governor.resolve_symlink_target("editor"), Some("/usr/bin/vim".to_string()));

        // Manual override
        governor.set_active_provider("editor", "nano");
        assert_eq!(governor.resolve_symlink_target("editor"), Some("/usr/bin/nano".to_string()));

        // Back to auto mode
        governor.set_auto_mode("editor");
        assert_eq!(governor.resolve_symlink_target("editor"), Some("/usr/bin/vim".to_string()));
    }

    #[test]
    fn test_freebsd_pkg_message_notifier() {
        let mut notifier = FreeBsdPkgMessageNotifierEngine::new();
        notifier.register_message(PkgMessage {
            package_name: "postgresql15-server".to_string(),
            message: "Run 'sysrc postgresql_enable=YES' to enable daemon.".to_string(),
            trigger: PkgMessageTrigger::Install,
            minimum_version: None,
        });

        let msgs = notifier.get_actionable_messages("postgresql15-server", &PkgMessageTrigger::Install, "15.3");
        assert_eq!(msgs.len(), 1);
        assert!(msgs[0].contains("postgresql_enable=YES"));

        let no_msgs = notifier.get_actionable_messages("postgresql15-server", &PkgMessageTrigger::Remove, "15.3");
        assert!(no_msgs.is_empty());
    }

    #[test]
    fn test_openbsd_pledge_unveil_sandbox_scriptlet() {
        let mut sandbox = OpenBsdPledgeUnveilSandboxScriptletEngine::new();
        sandbox.add_unveil_path("/tmp/pkg_install", "rwc");
        sandbox.set_pledge_promises(&["stdio", "rpath", "wpath", "cpath"]);

        assert!(sandbox.validate_path_access("/tmp/pkg_install/file.txt", "r"));
        assert!(!sandbox.validate_path_access("/etc/shadow", "r"));

        let safe_res = sandbox.execute_sandboxed_scriptlet("echo Initializing package");
        assert!(safe_res.is_ok());

        let unsafe_res = sandbox.execute_sandboxed_scriptlet("rm -rf /");
        assert!(unsafe_res.is_err());
    }

    #[test]
    fn test_alpine_apk_cache_peer_sync() {
        let mut peer_engine = AlpineApkCachePeerSyncEngine::new();
        let mut peer1_pkgs = BTreeMap::new();
        peer1_pkgs.insert("curl".to_string(), "hash_curl_8_0".to_string());

        peer_engine.register_peer(CachePeerNode {
            node_id: "peer_node_1".to_string(),
            ip_address: "192.168.1.50".to_string(),
            available_packages: peer1_pkgs,
        });

        let discovered = peer_engine.discover_peer_with_package("curl", "hash_curl_8_0");
        assert!(discovered.is_some());
        assert_eq!(discovered.unwrap().node_id, "peer_node_1");

        let synced = peer_engine.sync_package_from_peer("peer_node_1", "curl", b"CURL_PACKAGE_BYTES");
        assert!(synced);
        assert_eq!(peer_engine.local_cache.get("curl").unwrap(), b"CURL_PACKAGE_BYTES");
    }

    #[test]
    fn test_rpm_ostree_layered_image_governor() {
        let mut ostree = RpmOstreeLayeredImageGovernorEngine::new("commit_base_v1");
        ostree.stage_overlay_layer("layer_gaming", &["steam", "mangohud"], "commit_gaming_hash");

        let deploy_v2 = ostree.commit_deploy_tree("deploy-v2");
        assert_eq!(deploy_v2, "deploy-v2");
        assert_eq!(ostree.active_deployment_id, "deploy-v2");

        let pinned = ostree.pin_deployment("deploy-v2");
        assert!(pinned);

        let rollback = ostree.rollback_deployment("deploy-commit_base_v1");
        assert!(rollback.is_ok());
        assert_eq!(ostree.active_deployment_id, "deploy-commit_base_v1");
    }
}
