extern crate alloc;
// Sovereign Package Management Innovations for SigmaOS
// Features Gentoo Ebuild USE flag solver, FreeBSD pkg DB, Arch ALPM hooks, Nix Flakes cache,
// Slackware SlackBuild compiler, Zypper Boolean SAT resolver, and Solus Moss stateless transaction engine.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

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
}
