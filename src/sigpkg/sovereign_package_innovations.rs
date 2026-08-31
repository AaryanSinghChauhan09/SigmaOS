extern crate alloc;
// Sovereign Package Management Innovations for SigmaOS
// Features Gentoo Ebuild USE flag solver, FreeBSD pkg DB, Arch ALPM hooks, and Nix Flakes cache

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
            let matched = installed_pkgs.iter().any(|pkg| pkg.contains(&hook.target_pattern) || hook.target_pattern == "*");
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

/// Slackware SlackBuild Package Explosion & Compilation Engine
#[derive(Debug, Clone)]
pub struct SlackBuildScript {
    pub name: String,
    pub version: String,
    pub build_number: u32,
    pub arch: String,
    pub configure_flags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SlackwarePackageArchive {
    pub name: String,
    pub txz_filename: String,
    pub file_list: Vec<String>,
    pub slack_desc: String,
    pub doinst_sh: Option<String>,
}

pub struct SlackwareBuildPackageEngine {
    pub build_scripts: Vec<SlackBuildScript>,
    pub generated_archives: Vec<SlackwarePackageArchive>,
}

impl SlackwareBuildPackageEngine {
    pub fn new() -> Self {
        Self {
            build_scripts: Vec::new(),
            generated_archives: Vec::new(),
        }
    }

    pub fn register_slackbuild(&mut self, script: SlackBuildScript) {
        self.build_scripts.push(script);
    }

    pub fn compile_slackbuild(
        &mut self,
        pkg_name: &str,
        files: &[&str],
        slack_desc: &str,
    ) -> Result<String, &'static str> {
        let script = self
            .build_scripts
            .iter()
            .find(|s| s.name == pkg_name)
            .ok_or("SlackBuild script not found")?;

        let txz_filename = format!(
            "{}-{}-{}-{}.txz",
            script.name, script.version, script.arch, script.build_number
        );
        let archive = SlackwarePackageArchive {
            name: script.name.clone(),
            txz_filename: txz_filename.clone(),
            file_list: files.iter().map(|s| s.to_string()).collect(),
            slack_desc: slack_desc.to_string(),
            doinst_sh: Some("/sbin/makepkg".to_string()),
        };

        self.generated_archives.push(archive);
        Ok(txz_filename)
    }

    pub fn explode_txz_archive(&self, txz_filename: &str) -> Option<Vec<String>> {
        self.generated_archives
            .iter()
            .find(|a| a.txz_filename == txz_filename)
            .map(|a| a.file_list.clone())
    }
}

impl Default for SlackwareBuildPackageEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// openSUSE Libzypp SAT Boolean Solver & Vendor Lock Engine
#[derive(Debug, Clone)]
pub struct ZypperPackageSpec {
    pub name: String,
    pub version: String,
    pub vendor: String,
    pub priority: i32,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
}

pub struct ZypperSatDependencyResolver {
    pub available_packages: Vec<ZypperPackageSpec>,
    pub installed_packages: Vec<ZypperPackageSpec>,
    pub allow_vendor_change: bool,
}

impl ZypperSatDependencyResolver {
    pub fn new(allow_vendor_change: bool) -> Self {
        Self {
            available_packages: Vec::new(),
            installed_packages: Vec::new(),
            allow_vendor_change,
        }
    }

    pub fn register_available_package(&mut self, pkg: ZypperPackageSpec) {
        self.available_packages.push(pkg);
    }

    pub fn install_package_record(&mut self, pkg: ZypperPackageSpec) {
        self.installed_packages.push(pkg);
    }

    pub fn resolve_sat_selection(
        &self,
        target_pkg_name: &str,
    ) -> Result<ZypperPackageSpec, &'static str> {
        let candidates: Vec<&ZypperPackageSpec> = self
            .available_packages
            .iter()
            .filter(|p| p.name == target_pkg_name)
            .collect();

        if candidates.is_empty() {
            return Err("SAT resolution failed: no matching package candidate found");
        }

        // Check for vendor lock/stickiness if package is currently installed
        let current_installed = self
            .installed_packages
            .iter()
            .find(|p| p.name == target_pkg_name);

        if let Some(installed) = current_installed {
            if !self.allow_vendor_change {
                let same_vendor_candidates: Vec<&&ZypperPackageSpec> = candidates
                    .iter()
                    .filter(|c| c.vendor == installed.vendor)
                    .collect();

                if let Some(best) = same_vendor_candidates.iter().max_by_key(|c| c.priority) {
                    return Ok((***best).clone());
                } else {
                    return Err("SAT resolution failed: vendor change restricted by policy");
                }
            }
        }

        let best = candidates
            .iter()
            .max_by_key(|c| c.priority)
            .ok_or("SAT candidate selection error")?;

        Ok((*best).clone())
    }
}

impl Default for ZypperSatDependencyResolver {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Solus & Serpent OS Moss-inspired Stateless Package Engine
#[derive(Debug, Clone)]
pub struct MossStatelessPackage {
    pub name: String,
    pub version: String,
    pub build_release: u32,
    pub hash_id: String,
    pub default_configs: Vec<(String, String)>, // (path, content)
}

#[derive(Debug, Clone)]
pub struct MossStateTransaction {
    pub state_id: u32,
    pub active_packages: Vec<MossStatelessPackage>,
    pub timestamp: u64,
}

pub struct SolusMossStatelessTransactionEngine {
    pub transactions: Vec<MossStateTransaction>,
    pub active_state_id: u32,
    pub system_usr_overlay: BTreeMap<String, String>,
}

impl SolusMossStatelessTransactionEngine {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            active_state_id: 0,
            system_usr_overlay: BTreeMap::new(),
        }
    }

    pub fn commit_state_transaction(&mut self, pkgs: Vec<MossStatelessPackage>) -> u32 {
        self.active_state_id += 1;
        let state_id = self.active_state_id;

        for pkg in &pkgs {
            for (path, content) in &pkg.default_configs {
                // Statutory stateless overlay under /usr/share/defaults/
                let default_path = format!("/usr/share/defaults/{}", path.trim_start_matches('/'));
                self.system_usr_overlay
                    .insert(default_path, content.clone());
            }
        }

        self.transactions.push(MossStateTransaction {
            state_id,
            active_packages: pkgs,
            timestamp: self.transactions.len() as u64 + 1,
        });

        state_id
    }

    pub fn rollback_to_state(&mut self, state_id: u32) -> Result<(), &'static str> {
        let tx = self
            .transactions
            .iter()
            .find(|t| t.state_id == state_id)
            .ok_or("Target state ID not found")?;

        self.active_state_id = tx.state_id;
        self.system_usr_overlay.clear();

        for pkg in &tx.active_packages {
            for (path, content) in &pkg.default_configs {
                let default_path = format!("/usr/share/defaults/{}", path.trim_start_matches('/'));
                self.system_usr_overlay
                    .insert(default_path, content.clone());
            }
        }

        Ok(())
    }
}

impl Default for SolusMossStatelessTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD / OpenBSD Inspired pkg(8) Manifest & Signature Auditor
#[derive(Debug, Clone)]
pub struct BsdPkgManifestSpec {
    pub name: String,
    pub version: String,
    pub origin: String,
    pub arch_abi: String,
    pub shlibs_required: Vec<String>,
    pub signature_dilithium: String,
}

pub struct BsdPkgManifestSignatureAuditor {
    pub trusted_abi: String,
    pub trusted_keys: Vec<String>,
}

impl BsdPkgManifestSignatureAuditor {
    pub fn new(trusted_abi: &str) -> Self {
        Self {
            trusted_abi: trusted_abi.to_string(),
            trusted_keys: Vec::new(),
        }
    }

    pub fn register_trusted_key(&mut self, key: &str) {
        self.trusted_keys.push(key.to_string());
    }

    pub fn audit_manifest(&self, manifest: &BsdPkgManifestSpec) -> Result<bool, &'static str> {
        if manifest.arch_abi != self.trusted_abi && manifest.arch_abi != "any" {
            return Err("ABI mismatch: package not compatible with system ABI");
        }

        if manifest.signature_dilithium.is_empty() {
            return Err("Unsigned package manifest");
        }

        let is_signed_by_trusted = self
            .trusted_keys
            .iter()
            .any(|k| manifest.signature_dilithium.contains(k));
        if !is_signed_by_trusted {
            return Err("Untrusted cryptographic signature");
        }

        Ok(true)
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
        assert_eq!(db.query_pkg_file_owner("/usr/bin/zsh"), Some("zsh".to_string()));
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
            .compile_slackbuild(
                "htop",
                &["/usr/bin/htop", "/usr/man/man1/htop.1"],
                "htop process viewer",
            )
            .unwrap();
        assert_eq!(txz, "htop-3.2.1-x86_64-1.txz");

        let exploded = engine.explode_txz_archive(&txz).unwrap();
        assert_eq!(exploded.len(), 2);
        assert!(exploded.contains(&"/usr/bin/htop".to_string()));
    }

    #[test]
    fn test_zypper_sat_resolver_vendor_lock() {
        let mut resolver = ZypperSatDependencyResolver::new(false); // Vendor lock enabled

        let pkg_open_suse = ZypperPackageSpec {
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

        resolver.register_available_package(pkg_open_suse.clone());
        resolver.register_available_package(pkg_packman.clone());

        // Currently installed from openSUSE
        resolver.install_package_record(pkg_open_suse.clone());
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

        assert_eq!(
            moss.system_usr_overlay
                .get("/usr/share/defaults/etc/nanorc")
                .map(|s| s.as_str()),
            Some("set syntaxon")
        );
    }

    #[test]
    fn test_bsd_pkg_manifest_auditor() {
        let mut auditor = BsdPkgManifestSignatureAuditor::new("freebsd:14:x86:64");
        auditor.register_trusted_key("dilithium_key_sigos_official");

        let valid_manifest = BsdPkgManifestSpec {
            name: "git".to_string(),
            version: "2.42.0".to_string(),
            origin: "devel/git".to_string(),
            arch_abi: "freebsd:14:x86:64".to_string(),
            shlibs_required: vec!["libcrypto.so.30".to_string()],
            signature_dilithium: "dilithium_key_sigos_official:sig_data_xyz".to_string(),
        };

        assert_eq!(auditor.audit_manifest(&valid_manifest), Ok(true));

        let bad_abi_manifest = BsdPkgManifestSpec {
            arch_abi: "openbsd:7.4:aarch64".to_string(),
            ..valid_manifest.clone()
        };
        assert!(auditor.audit_manifest(&bad_abi_manifest).is_err());
        let hash = NixFlakeHermeticCacheStore::compute_flake_hash("github:nixos/nixpkgs", "lock_data");

        store.store_build(&hash, b"HERMETIC_NIX_OUTPUT");
        assert_eq!(store.fetch_cached_build(&hash).unwrap(), b"HERMETIC_NIX_OUTPUT");
    }
}
