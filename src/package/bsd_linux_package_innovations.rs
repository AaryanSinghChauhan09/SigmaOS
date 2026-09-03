// SPDX-License-Identifier: MIT
// SigmaOS - Linux & BSD Inspired Package Management Innovations
// Inspired by FreeBSD Ports & VuXML, Void XBPS, Alpine APK v3, Nix/Guix CAS,
// Arch ALPM hooks, Fedora DNF5 DeltaRPM, Gentoo Portage subslots, and Haiku PackageFS.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// =========================================================================
// 1. FreeBSD Ports Flavours & VuXML Package Vulnerability Auditor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortFlavour {
    pub name: String,
    pub base_package: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlEntry {
    pub vuln_id: String,
    pub cve_id: String,
    pub package_name: String,
    pub vulnerable_range_min: String,
    pub vulnerable_range_max: String,
    pub severity: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlAuditReport {
    pub package_name: String,
    pub installed_version: String,
    pub vuln_id: String,
    pub cve_id: String,
    pub severity: String,
    pub summary: String,
}

pub struct FreeBsdPortsFlavoursAndVuxmlEngine {
    pub flavours: BTreeMap<String, Vec<PortFlavour>>,
    pub vuxml_database: Vec<VuXmlEntry>,
}

impl FreeBsdPortsFlavoursAndVuxmlEngine {
    pub fn new() -> Self {
        Self {
            flavours: BTreeMap::new(),
            vuxml_database: Vec::new(),
        }
    }

    pub fn register_flavour(&mut self, base_package: &str, flavour_name: &str, options: &[&str]) {
        let entry = PortFlavour {
            name: flavour_name.to_string(),
            base_package: base_package.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
        };
        self.flavours
            .entry(base_package.to_string())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    pub fn register_vuxml_entry(&mut self, entry: VuXmlEntry) {
        self.vuxml_database.push(entry);
    }

    pub fn audit_installed_packages(&self, installed: &[(&str, &str)]) -> Vec<VuXmlAuditReport> {
        let mut reports = Vec::new();
        for &(pkg_name, pkg_ver) in installed {
            for vuln in &self.vuxml_database {
                if vuln.package_name == pkg_name {
                    if pkg_ver >= vuln.vulnerable_range_min.as_str()
                        && pkg_ver <= vuln.vulnerable_range_max.as_str()
                    {
                        reports.push(VuXmlAuditReport {
                            package_name: pkg_name.to_string(),
                            installed_version: pkg_ver.to_string(),
                            vuln_id: vuln.vuln_id.clone(),
                            cve_id: vuln.cve_id.clone(),
                            severity: vuln.severity.clone(),
                            summary: vuln.summary.clone(),
                        });
                    }
                }
            }
        }
        reports
    }
}

impl Default for FreeBsdPortsFlavoursAndVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Void Linux XBPS Soname Library Dependency Tracker & Orphan Package Resolver
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsPackageRecord {
    pub name: String,
    pub version: String,
    pub provided_sonames: Vec<String>,
    pub required_sonames: Vec<String>,
    pub is_explicitly_installed: bool,
}

pub struct XbpsSonameAndOrphanEngine {
    pub installed_packages: BTreeMap<String, XbpsPackageRecord>,
}

impl XbpsSonameAndOrphanEngine {
    pub fn new() -> Self {
        Self {
            installed_packages: BTreeMap::new(),
        }
    }

    pub fn register_package(&mut self, record: XbpsPackageRecord) {
        self.installed_packages.insert(record.name.clone(), record);
    }

    /// Finds missing sonames across installed packages
    pub fn find_missing_sonames(&self) -> Vec<(String, String)> {
        let mut missing = Vec::new();
        let mut available_sonames = Vec::new();
        for pkg in self.installed_packages.values() {
            for soname in &pkg.provided_sonames {
                available_sonames.push(soname.clone());
            }
        }

        for pkg in self.installed_packages.values() {
            for req in &pkg.required_sonames {
                if !available_sonames.contains(req) {
                    missing.push((pkg.name.clone(), req.clone()));
                }
            }
        }
        missing
    }

    /// Identifies orphan packages installed as dependencies that are no longer required
    pub fn find_orphan_packages(&self) -> Vec<String> {
        let mut required_by_others = Vec::new();

        // Collect all required sonames
        for pkg in self.installed_packages.values() {
            for req in &pkg.required_sonames {
                required_by_others.push(req.clone());
            }
        }

        let mut orphans = Vec::new();
        for (pkg_name, pkg) in &self.installed_packages {
            if !pkg.is_explicitly_installed {
                let provides_needed_soname = pkg
                    .provided_sonames
                    .iter()
                    .any(|so| required_by_others.contains(so));
                if !provides_needed_soname {
                    orphans.push(pkg_name.clone());
                }
            }
        }
        orphans
    }
}

impl Default for XbpsSonameAndOrphanEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Alpine APK v3 Declarative World Rules & Ephemeral Virtual Build Deps
// =========================================================================

pub struct AlpineApkWorldAndVirtualPkgEngine {
    pub world_packages: Vec<String>,
    pub installed_virtual_groups: BTreeMap<String, Vec<String>>,
}

impl AlpineApkWorldAndVirtualPkgEngine {
    pub fn new() -> Self {
        Self {
            world_packages: Vec::new(),
            installed_virtual_groups: BTreeMap::new(),
        }
    }

    pub fn add_to_world(&mut self, package: &str) {
        if !self.world_packages.contains(&package.to_string()) {
            self.world_packages.push(package.to_string());
        }
    }

    pub fn remove_from_world(&mut self, package: &str) {
        self.world_packages.retain(|p| p != package);
    }

    pub fn add_virtual_group(&mut self, group_name: &str, packages: &[&str]) {
        self.installed_virtual_groups.insert(
            group_name.to_string(),
            packages.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn remove_virtual_group(&mut self, group_name: &str) -> Option<Vec<String>> {
        self.installed_virtual_groups.remove(group_name)
    }

    pub fn generate_world_file(&self) -> String {
        let mut out = String::new();
        for pkg in &self.world_packages {
            out.push_str(pkg);
            out.push('\n');
        }
        out
    }
}

impl Default for AlpineApkWorldAndVirtualPkgEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Nix/Guix Content-Addressed Storage (CAS) GC Root Scanner & Profile Switcher
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileGeneration {
    pub gen_number: u32,
    pub store_path: String,
    pub timestamp_sec: u64,
}

pub struct NixGuixCasGcProfileEngine {
    pub gc_roots: Vec<String>,
    pub profile_generations: Vec<ProfileGeneration>,
    pub active_generation: Option<u32>,
}

impl NixGuixCasGcProfileEngine {
    pub fn new() -> Self {
        Self {
            gc_roots: Vec::new(),
            profile_generations: Vec::new(),
            active_generation: None,
        }
    }

    pub fn register_gc_root(&mut self, path: &str) {
        if !self.gc_roots.contains(&path.to_string()) {
            self.gc_roots.push(path.to_string());
        }
    }

    pub fn create_generation(&mut self, store_path: &str, now_sec: u64) -> u32 {
        let gen_number = (self.profile_generations.len() as u32) + 1;
        let gen = ProfileGeneration {
            gen_number,
            store_path: store_path.to_string(),
            timestamp_sec: now_sec,
        };
        self.profile_generations.push(gen);
        self.active_generation = Some(gen_number);
        gen_number
    }

    pub fn switch_profile(&mut self, gen_number: u32) -> Result<String, &'static str> {
        let gen = self
            .profile_generations
            .iter()
            .find(|g| g.gen_number == gen_number)
            .ok_or("Profile generation not found")?;
        self.active_generation = Some(gen_number);
        Ok(gen.store_path.clone())
    }

    /// Identifies unreferenced store paths for garbage collection
    pub fn scan_dead_store_paths(&self, all_store_paths: &[&str]) -> Vec<String> {
        let mut dead = Vec::new();
        let mut live_paths = self.gc_roots.clone();
        for gen in &self.profile_generations {
            live_paths.push(gen.store_path.clone());
        }

        for &path in all_store_paths {
            if !live_paths.iter().any(|live| path.starts_with(live.as_str()) || live.starts_with(path)) {
                dead.push(path.to_string());
            }
        }
        dead
    }
}

impl Default for NixGuixCasGcProfileEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Arch Split-Package Generator & ALPM Path Hook Runner
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitSubPackage {
    pub name: String,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpmPathHook {
    pub name: String,
    pub trigger_paths: Vec<String>,
    pub action_command: String,
}

pub struct ArchSplitPackageHookRunnerEngine {
    pub hooks: Vec<AlpmPathHook>,
    pub executed_hooks: Vec<String>,
}

impl ArchSplitPackageHookRunnerEngine {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            executed_hooks: Vec::new(),
        }
    }

    pub fn register_hook(&mut self, hook: AlpmPathHook) {
        self.hooks.push(hook);
    }

    pub fn trigger_path_hooks(&mut self, modified_paths: &[&str]) -> usize {
        let mut count = 0;
        for hook in &self.hooks {
            let matches = hook.trigger_paths.iter().any(|trig| {
                modified_paths
                    .iter()
                    .any(|mod_path| mod_path.starts_with(trig.as_str()))
            });
            if matches {
                self.executed_hooks.push(hook.name.clone());
                count += 1;
            }
        }
        count
    }
}

impl Default for ArchSplitPackageHookRunnerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Fedora DNF5 Advisory Filtering & Binary Delta RPMs
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5Advisory {
    pub advisory_id: String,
    pub severity: String, // e.g. "Security", "Bugfix", "Enhancement"
    pub target_package: String,
    pub fixed_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaRpmPatch {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub delta_size_bytes: u64,
    pub full_size_bytes: u64,
}

pub struct FedoraDnf5AdvisoryAndDeltaRpmEngine {
    pub advisories: Vec<Dnf5Advisory>,
    pub available_deltas: Vec<DeltaRpmPatch>,
}

impl FedoraDnf5AdvisoryAndDeltaRpmEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            available_deltas: Vec::new(),
        }
    }

    pub fn add_advisory(&mut self, advisory: Dnf5Advisory) {
        self.advisories.push(advisory);
    }

    pub fn add_delta_patch(&mut self, patch: DeltaRpmPatch) {
        self.available_deltas.push(patch);
    }

    pub fn filter_advisories_by_severity(&self, severity: &str) -> Vec<Dnf5Advisory> {
        self.advisories
            .iter()
            .filter(|a| a.severity.eq_ignore_ascii_case(severity))
            .cloned()
            .collect()
    }

    pub fn calculate_bandwidth_savings(&self) -> u64 {
        let mut savings = 0u64;
        for delta in &self.available_deltas {
            if delta.full_size_bytes > delta.delta_size_bytes {
                savings += delta.full_size_bytes - delta.delta_size_bytes;
            }
        }
        savings
    }
}

impl Default for FedoraDnf5AdvisoryAndDeltaRpmEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Gentoo Portage Subslot ABI Rebuild Solver & USE_EXPAND Processor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortagePackageSubslot {
    pub atom: String,
    pub slot: String,
    pub subslot: String,
    pub dependent_atoms: Vec<String>,
}

pub struct GentooPortageSubslotAndUseExpandEngine {
    pub installed_slots: BTreeMap<String, PortagePackageSubslot>,
    pub use_expand_vars: BTreeMap<String, Vec<String>>,
}

impl GentooPortageSubslotAndUseExpandEngine {
    pub fn new() -> Self {
        Self {
            installed_slots: BTreeMap::new(),
            use_expand_vars: BTreeMap::new(),
        }
    }

    pub fn register_subslot(&mut self, record: PortagePackageSubslot) {
        self.installed_slots.insert(record.atom.clone(), record);
    }

    pub fn set_use_expand(&mut self, var_name: &str, values: &[&str]) {
        self.use_expand_vars.insert(
            var_name.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Evaluates ABI breakage when a subslot changes and returns dependent packages requiring rebuild
    pub fn evaluate_abi_rebuilds(&self, atom: &str, new_subslot: &str) -> Vec<String> {
        let mut rebuilds = Vec::new();
        if let Some(existing) = self.installed_slots.get(atom) {
            if existing.subslot != new_subslot {
                for dep in &existing.dependent_atoms {
                    rebuilds.push(dep.clone());
                }
            }
        }
        rebuilds
    }
}

impl Default for GentooPortageSubslotAndUseExpandEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. Haiku PackageFS VFS Mount & Solus Moss Stateless Overlay
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HpkgMountedPackage {
    pub package_id: String,
    pub hpkg_path: String,
    pub mount_point: String,
    pub is_active: bool,
}

pub struct HaikuHpkgPackageFsEngine {
    pub mounted_packages: Vec<HpkgMountedPackage>,
    pub stateless_overlay_active: bool,
}

impl HaikuHpkgPackageFsEngine {
    pub fn new() -> Self {
        Self {
            mounted_packages: Vec::new(),
            stateless_overlay_active: true,
        }
    }

    pub fn mount_hpkg(&mut self, package_id: &str, hpkg_path: &str) -> String {
        let mount_point = format!("/system/packages/{}", package_id);
        let mounted = HpkgMountedPackage {
            package_id: package_id.to_string(),
            hpkg_path: hpkg_path.to_string(),
            mount_point: mount_point.clone(),
            is_active: true,
        };
        self.mounted_packages.push(mounted);
        mount_point
    }

    pub fn unmount_hpkg(&mut self, package_id: &str) -> bool {
        if let Some(pkg) = self.mounted_packages.iter_mut().find(|p| p.package_id == package_id) {
            pkg.is_active = false;
            true
        } else {
            false
        }
    }

    pub fn active_mount_count(&self) -> usize {
        self.mounted_packages.iter().filter(|p| p.is_active).count()
    }
}

impl Default for HaikuHpkgPackageFsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. Slackware Pkgtool & Sbopkg SlackBuild Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackBuildInfo {
    pub prgnam: String,
    pub version: String,
    pub homepage: String,
    pub download: Vec<String>,
    pub md5sum: Vec<String>,
    pub requires: Vec<String>,
    pub maintainer: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackPackageRecord {
    pub package_name: String,
    pub compressed_size_kb: u64,
    pub uncompressed_size_kb: u64,
    pub description: Vec<String>,
    pub installed_files: Vec<String>,
}

pub struct SlackwarePkgtoolSlackBuildEngine {
    pub slackbuilds: BTreeMap<String, SlackBuildInfo>,
    pub installed_packages: BTreeMap<String, SlackPackageRecord>,
}

impl SlackwarePkgtoolSlackBuildEngine {
    pub fn new() -> Self {
        Self {
            slackbuilds: BTreeMap::new(),
            installed_packages: BTreeMap::new(),
        }
    }

    pub fn register_slackbuild(&mut self, info: SlackBuildInfo) {
        self.slackbuilds.insert(info.prgnam.clone(), info);
    }

    pub fn install_package(&mut self, record: SlackPackageRecord) {
        self.installed_packages.insert(record.package_name.clone(), record);
    }

    pub fn remove_package(&mut self, package_name: &str) -> bool {
        self.installed_packages.remove(package_name).is_some()
    }

    pub fn parse_slackbuild_info(content: &str) -> Result<SlackBuildInfo, &'static str> {
        let mut prgnam = String::new();
        let mut version = String::new();
        let mut homepage = String::new();
        let mut download = Vec::new();
        let mut md5sum = Vec::new();
        let mut requires = Vec::new();
        let mut maintainer = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("PRGNAM=") {
                prgnam = line.strip_prefix("PRGNAM=").unwrap().trim_matches('"').to_string();
            } else if line.starts_with("VERSION=") {
                version = line.strip_prefix("VERSION=").unwrap().trim_matches('"').to_string();
            } else if line.starts_with("HOMEPAGE=") {
                homepage = line.strip_prefix("HOMEPAGE=").unwrap().trim_matches('"').to_string();
            } else if line.starts_with("DOWNLOAD=") {
                let urls = line.strip_prefix("DOWNLOAD=").unwrap().trim_matches('"');
                for u in urls.split_whitespace() {
                    download.push(u.to_string());
                }
            } else if line.starts_with("MD5SUM=") {
                let sums = line.strip_prefix("MD5SUM=").unwrap().trim_matches('"');
                for s in sums.split_whitespace() {
                    md5sum.push(s.to_string());
                }
            } else if line.starts_with("REQUIRES=") {
                let reqs = line.strip_prefix("REQUIRES=").unwrap().trim_matches('"');
                for r in reqs.split_whitespace() {
                    requires.push(r.to_string());
                }
            } else if line.starts_with("MAINTAINER=") {
                maintainer = line.strip_prefix("MAINTAINER=").unwrap().trim_matches('"').to_string();
            }
        }

        if prgnam.is_empty() {
            return Err("Missing PRGNAM in SlackBuild info file");
        }

        Ok(SlackBuildInfo {
            prgnam,
            version,
            homepage,
            download,
            md5sum,
            requires,
            maintainer,
        })
    }
}

impl Default for SlackwarePkgtoolSlackBuildEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 10. Ubuntu PPA & APT Pinning/Keyring Security Manager
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PpaRepository {
    pub ppa_owner: String,
    pub ppa_name: String,
    pub distro_codename: String,
    pub gpg_key_fingerprint: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptPinRule {
    pub package_pattern: String,
    pub release_codename: String,
    pub pin_priority: i32,
}

pub struct UbuntuPpaAptPinningEngine {
    pub ppas: BTreeMap<String, PpaRepository>,
    pub pin_rules: Vec<AptPinRule>,
    pub trusted_gpg_keys: Vec<String>,
}

impl UbuntuPpaAptPinningEngine {
    pub fn new() -> Self {
        Self {
            ppas: BTreeMap::new(),
            pin_rules: Vec::new(),
            trusted_gpg_keys: Vec::new(),
        }
    }

    pub fn add_ppa(&mut self, owner: &str, ppa_name: &str, codename: &str, key_fp: &str) -> String {
        let ppa_key = format!("ppa:{}/{}", owner, ppa_name);
        let repo = PpaRepository {
            ppa_owner: owner.to_string(),
            ppa_name: ppa_name.to_string(),
            distro_codename: codename.to_string(),
            gpg_key_fingerprint: key_fp.to_string(),
            is_enabled: true,
        };
        self.ppas.insert(ppa_key.clone(), repo);
        self.trusted_gpg_keys.push(key_fp.to_string());
        ppa_key
    }

    pub fn add_pin_rule(&mut self, pattern: &str, codename: &str, priority: i32) {
        self.pin_rules.push(AptPinRule {
            package_pattern: pattern.to_string(),
            release_codename: codename.to_string(),
            pin_priority: priority,
        });
    }

    pub fn resolve_effective_priority(&self, package: &str, codename: &str) -> i32 {
        let mut max_prio = 500; // Default APT priority
        for rule in &self.pin_rules {
            let pkg_matches = if rule.package_pattern == "*" {
                true
            } else if rule.package_pattern.ends_with('*') {
                package.starts_with(rule.package_pattern.trim_end_matches('*'))
            } else {
                rule.package_pattern == package
            };

            let codename_matches = rule.release_codename == "*" || rule.release_codename == codename;

            if pkg_matches && codename_matches {
                if rule.pin_priority > max_prio {
                    max_prio = rule.pin_priority;
                }
            }
        }
        max_prio
    }

    pub fn verify_key_trusted(&self, fingerprint: &str) -> bool {
        self.trusted_gpg_keys.contains(&fingerprint.to_string())
    }
}

impl Default for UbuntuPpaAptPinningEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 11. OpenSUSE Zypper Libzypp Multi-Repo Solver & Vendor Stickiness
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZypperRepository {
    pub repo_alias: String,
    pub name: String,
    pub priority: u32, // Lower number = higher priority (Zypper convention: 1..99)
    pub vendor: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZypperPackageOffer {
    pub package_name: String,
    pub version: String,
    pub vendor: String,
    pub repo_alias: String,
}

pub struct OpenSuseZypperVendorStickinessEngine {
    pub repositories: BTreeMap<String, ZypperRepository>,
    pub vendor_change_allowed: bool,
}

impl OpenSuseZypperVendorStickinessEngine {
    pub fn new() -> Self {
        Self {
            repositories: BTreeMap::new(),
            vendor_change_allowed: false, // Vendor stickiness enabled by default
        }
    }

    pub fn register_repo(&mut self, repo: ZypperRepository) {
        self.repositories.insert(repo.repo_alias.clone(), repo);
    }

    pub fn select_best_offer(
        &self,
        current_installed_vendor: Option<&str>,
        offers: &[ZypperPackageOffer],
    ) -> Result<ZypperPackageOffer, &'static str> {
        if offers.is_empty() {
            return Err("No package offers available");
        }

        let mut candidate_offers: Vec<&ZypperPackageOffer> = offers.iter().collect();

        // If vendor stickiness is active and we have an installed vendor, filter out differing vendors
        if !self.vendor_change_allowed {
            if let Some(cur_vendor) = current_installed_vendor {
                let same_vendor_offers: Vec<&ZypperPackageOffer> = candidate_offers
                    .iter()
                    .filter(|o| o.vendor == cur_vendor)
                    .cloned()
                    .collect();
                if !same_vendor_offers.is_empty() {
                    candidate_offers = same_vendor_offers;
                }
            }
        }

        // Sort candidates by repo priority (lower number = higher priority), then version
        candidate_offers.sort_by(|a, b| {
            let prio_a = self.repositories.get(&a.repo_alias).map_or(99, |r| r.priority);
            let prio_b = self.repositories.get(&b.repo_alias).map_or(99, |r| r.priority);
            prio_a.cmp(&prio_b).then_with(|| b.version.cmp(&a.version))
        });

        Ok((*candidate_offers[0]).clone())
    }
}

impl Default for OpenSuseZypperVendorStickinessEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. Nix Flakes Lockfile & Reproducible Devshell Resolver
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlakeInputLock {
    pub input_name: String,
    pub original_url: String,
    pub locked_nar_hash: String,
    pub rev: String,
}

pub struct NixFlakesDevshellResolverEngine {
    pub locked_inputs: BTreeMap<String, FlakeInputLock>,
    pub devshell_env_vars: BTreeMap<String, String>,
    pub devshell_packages: Vec<String>,
}

impl NixFlakesDevshellResolverEngine {
    pub fn new() -> Self {
        Self {
            locked_inputs: BTreeMap::new(),
            devshell_env_vars: BTreeMap::new(),
            devshell_packages: Vec::new(),
        }
    }

    pub fn lock_input(&mut self, lock: FlakeInputLock) {
        self.locked_inputs.insert(lock.input_name.clone(), lock);
    }

    pub fn export_devshell_var(&mut self, key: &str, val: &str) {
        self.devshell_env_vars.insert(key.to_string(), val.to_string());
    }

    pub fn add_devshell_package(&mut self, pkg: &str) {
        if !self.devshell_packages.contains(&pkg.to_string()) {
            self.devshell_packages.push(pkg.to_string());
        }
    }

    pub fn verify_lockfile_reproducibility(&self) -> bool {
        !self.locked_inputs.is_empty()
            && self
                .locked_inputs
                .values()
                .all(|l| !l.locked_nar_hash.is_empty() && !l.rev.is_empty())
    }

    pub fn generate_devshell_manifest(&self) -> String {
        let mut manifest = String::from("# Nix Flakes DevShell Environment\n");
        for (k, v) in &self.devshell_env_vars {
            manifest.push_str(&format!("export {}=\"{}\"\n", k, v));
        }
        manifest.push_str("# Packages:\n");
        for pkg in &self.devshell_packages {
            manifest.push_str(&format!("- {}\n", pkg));
        }
        manifest
    }
}

impl Default for NixFlakesDevshellResolverEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 13. OpenBSD Signify & Pkg_add Multi-Location Mirror Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBsdPkgAddSignifyEngine {
    pub signify_public_keys: BTreeMap<String, String>, // key_name -> pubkey_base64
    pub pkg_path_mirrors: Vec<String>,
}

impl OpenBsdPkgAddSignifyEngine {
    pub fn new() -> Self {
        Self {
            signify_public_keys: BTreeMap::new(),
            pkg_path_mirrors: Vec::new(),
        }
    }

    pub fn add_signify_key(&mut self, key_name: &str, pubkey_base64: &str) {
        self.signify_public_keys
            .insert(key_name.to_string(), pubkey_base64.to_string());
    }

    pub fn add_mirror(&mut self, mirror_url: &str) {
        if !self.pkg_path_mirrors.contains(&mirror_url.to_string()) {
            self.pkg_path_mirrors.push(mirror_url.to_string());
        }
    }

    pub fn verify_signify_signature(&self, key_name: &str, signature_header: &str) -> bool {
        if let Some(expected_key) = self.signify_public_keys.get(key_name) {
            signature_header.contains(expected_key)
        } else {
            false
        }
    }

    pub fn resolve_package_download_url(&self, package_tgz: &str) -> Result<String, &'static str> {
        if self.pkg_path_mirrors.is_empty() {
            return Err("PKG_PATH mirrors empty");
        }
        let primary_mirror = &self.pkg_path_mirrors[0];
        let url = if primary_mirror.ends_with('/') {
            format!("{}{}", primary_mirror, package_tgz)
        } else {
            format!("{}/{}", primary_mirror, package_tgz)
        };
        Ok(url)
    }
}

impl Default for OpenBsdPkgAddSignifyEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 14. Debian Debconf & Dpkg-Statoverride System Integration
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebconfQuestionType {
    String,
    Boolean,
    Select,
    Password,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebconfPreseedEntry {
    pub package_owner: String,
    pub question_template: String,
    pub qtype: DebconfQuestionType,
    pub answer_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgStatoverrideRule {
    pub owner_user: String,
    pub owner_group: String,
    pub mode_octal: u32,
    pub target_path: String,
}

pub struct DebianDebconfStatoverrideEngine {
    pub preseed_db: BTreeMap<String, DebconfPreseedEntry>,
    pub statoverrides: BTreeMap<String, DpkgStatoverrideRule>,
}

impl DebianDebconfStatoverrideEngine {
    pub fn new() -> Self {
        Self {
            preseed_db: BTreeMap::new(),
            statoverrides: BTreeMap::new(),
        }
    }

    pub fn add_preseed_answer(&mut self, entry: DebconfPreseedEntry) {
        let key = format!("{}/{}", entry.package_owner, entry.question_template);
        self.preseed_db.insert(key, entry);
    }

    pub fn add_statoverride(&mut self, rule: DpkgStatoverrideRule) {
        self.statoverrides.insert(rule.target_path.clone(), rule);
    }

    pub fn query_preseed_answer(&self, package: &str, question: &str) -> Option<String> {
        let key = format!("{}/{}", package, question);
        self.preseed_db.get(&key).map(|e| e.answer_value.clone())
    }

    pub fn get_statoverride_for_path(&self, path: &str) -> Option<&DpkgStatoverrideRule> {
        self.statoverrides.get(path)
    }
}

impl Default for DebianDebconfStatoverrideEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 15. Arch / CachyOS x86-64 Microarchitecture Level Optimization Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MicroarchitectureLevel {
    V1, // Generic x86-64
    V2, // CMPXCHG16B, LAHF-SAHF, POPCNT, SSE3, SSSE3, SSE4.1, SSE4.2
    V3, // AVX, AVX2, BMI1, BMI2, F16C, FMA, LZCNT, MOVBE, OSXSAVE
    V4, // AVX512F, AVX512BW, AVX512CD, AVX512DQ, AVX512VL
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicroarchRepoRoute {
    pub level: MicroarchitectureLevel,
    pub repo_url: String,
    pub is_enabled: bool,
}

pub struct ArchCachyosMicroarchOptimizationEngine {
    pub detected_level: MicroarchitectureLevel,
    pub cpu_flags: Vec<String>,
    pub repo_routes: Vec<MicroarchRepoRoute>,
}

impl ArchCachyosMicroarchOptimizationEngine {
    pub fn new() -> Self {
        Self {
            detected_level: MicroarchitectureLevel::V1,
            cpu_flags: Vec::new(),
            repo_routes: Vec::new(),
        }
    }

    pub fn detect_microarch_level(&mut self, flags: &[&str]) -> MicroarchitectureLevel {
        let mut flag_set = Vec::new();
        for &f in flags {
            flag_set.push(f.to_lowercase());
        }
        self.cpu_flags = flag_set.clone();

        let has_v2 = ["sse3", "ssse3", "sse4_1", "sse4_2", "popcnt"]
            .iter()
            .all(|f| flag_set.contains(&f.to_string()));

        let has_v3 = has_v2
            && ["avx", "avx2", "bmi1", "bmi2", "fma", "f16c", "lzcnt"]
                .iter()
                .all(|f| flag_set.contains(&f.to_string()));

        let has_v4 = has_v3
            && ["avx512f", "avx512bw", "avx512cd", "avx512dq", "avx512vl"]
                .iter()
                .all(|f| flag_set.contains(&f.to_string()));

        let level = if has_v4 {
            MicroarchitectureLevel::V4
        } else if has_v3 {
            MicroarchitectureLevel::V3
        } else if has_v2 {
            MicroarchitectureLevel::V2
        } else {
            MicroarchitectureLevel::V1
        };

        self.detected_level = level;
        level
    }

    pub fn register_repo_route(&mut self, level: MicroarchitectureLevel, repo_url: &str) {
        self.repo_routes.push(MicroarchRepoRoute {
            level,
            repo_url: repo_url.to_string(),
            is_enabled: true,
        });
    }

    pub fn resolve_optimal_repo(&self) -> String {
        let mut best_route: Option<&MicroarchRepoRoute> = None;
        for route in &self.repo_routes {
            if route.is_enabled && route.level <= self.detected_level {
                if best_route.map_or(true, |r| route.level > r.level) {
                    best_route = Some(route);
                }
            }
        }
        best_route
            .map(|r| r.repo_url.clone())
            .unwrap_or_else(|| "https://repo.sigmaos.org/core/x86_64".to_string())
    }
}

impl Default for ArchCachyosMicroarchOptimizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 16. Fedora COPR, Arch AUR & OBS Community Build Gateway Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunityRepoBackend {
    FedoraCopr,
    ArchAur,
    OpenSuseObs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunityPackageBuildSource {
    pub name: String,
    pub backend: CommunityRepoBackend,
    pub repository_owner: String,
    pub source_url: String,
    pub trust_score: u32, // 0..100
    pub is_sandboxed_build: bool,
}

pub struct CoprAurBuildRepositoryGatewayEngine {
    pub registered_sources: BTreeMap<String, CommunityPackageBuildSource>,
    pub min_trust_score_required: u32,
}

impl CoprAurBuildRepositoryGatewayEngine {
    pub fn new() -> Self {
        Self {
            registered_sources: BTreeMap::new(),
            min_trust_score_required: 50,
        }
    }

    pub fn register_source(&mut self, source: CommunityPackageBuildSource) {
        self.registered_sources.insert(source.name.clone(), source);
    }

    pub fn can_build_safely(&self, pkg_name: &str) -> Result<bool, &'static str> {
        let source = self
            .registered_sources
            .get(pkg_name)
            .ok_or("Community build source not registered")?;

        if !source.is_sandboxed_build {
            return Ok(false);
        }

        if source.trust_score < self.min_trust_score_required {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn generate_build_sandbox_cmd(&self, pkg_name: &str) -> Result<String, &'static str> {
        let source = self
            .registered_sources
            .get(pkg_name)
            .ok_or("Community build source not registered")?;

        let cmd = match source.backend {
            CommunityRepoBackend::FedoraCopr => {
                format!("copr-cli build-package {} --chroot fedora-rawhide-x86_64", source.name)
            }
            CommunityRepoBackend::ArchAur => {
                format!("makepkg --syncdeps --clean --noconfirm --dir /sandbox/aur/{}", source.name)
            }
            CommunityRepoBackend::OpenSuseObs => {
                format!("osc build --noservice {}", source.name)
            }
        };
        Ok(cmd)
    }
}

impl Default for CoprAurBuildRepositoryGatewayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 17. NetBSD Pkgsrc Options Framework Selector
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgsrcOptionSpec {
    pub option_name: String,
    pub description: String,
    pub requires_options: Vec<String>,
    pub conflicts_with: Vec<String>,
}

pub struct NetBsdPkgsrcOptionsFrameworkEngine {
    pub supported_options: BTreeMap<String, PkgsrcOptionSpec>,
    pub suggested_options: Vec<String>,
    pub active_options: Vec<String>,
}

impl NetBsdPkgsrcOptionsFrameworkEngine {
    pub fn new() -> Self {
        Self {
            supported_options: BTreeMap::new(),
            suggested_options: Vec::new(),
            active_options: Vec::new(),
        }
    }

    pub fn register_option(&mut self, spec: PkgsrcOptionSpec, suggested: bool) {
        if suggested {
            self.suggested_options.push(spec.option_name.clone());
            self.active_options.push(spec.option_name.clone());
        }
        self.supported_options.insert(spec.option_name.clone(), spec);
    }

    pub fn toggle_option(&mut self, option_str: &str) -> Result<(), &'static str> {
        if option_str.starts_with('+') {
            let opt = option_str.trim_start_matches('+');
            if !self.supported_options.contains_key(opt) {
                return Err("Unsupported option");
            }
            if !self.active_options.contains(&opt.to_string()) {
                self.active_options.push(opt.to_string());
            }
        } else if option_str.starts_with('-') {
            let opt = option_str.trim_start_matches('-');
            self.active_options.retain(|o| o != opt);
        } else {
            return Err("Option toggle must start with '+' or '-'");
        }
        Ok(())
    }

    pub fn validate_options(&self) -> Result<(), String> {
        for opt in &self.active_options {
            if let Some(spec) = self.supported_options.get(opt) {
                for req in &spec.requires_options {
                    if !self.active_options.contains(req) {
                        return Err(format!("Option '{}' requires missing option '{}'", opt, req));
                    }
                }
                for conf in &spec.conflicts_with {
                    if self.active_options.contains(conf) {
                        return Err(format!("Option '{}' conflicts with active option '{}'", opt, conf));
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for NetBsdPkgsrcOptionsFrameworkEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 18. Gentoo Portage EAPI Specification & Slot Operator Solver
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PortageEapiLevel {
    Eapi7,
    Eapi8,
    Eapi9,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotOperator {
    AnySlot,             // '*'
    SubslotEqual,        // ':='
    ExactSlot(String),   // ':slot'
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EbuildSlotRecord {
    pub atom: String,
    pub slot: String,
    pub subslot: String,
    pub eapi: PortageEapiLevel,
    pub slot_operator: SlotOperator,
}

pub struct GentooPortageEapiSlotOperatorEngine {
    pub slots: BTreeMap<String, EbuildSlotRecord>,
}

impl GentooPortageEapiSlotOperatorEngine {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
        }
    }

    pub fn register_ebuild_slot(&mut self, record: EbuildSlotRecord) {
        self.slots.insert(record.atom.clone(), record);
    }

    pub fn check_eapi_feature_support(&self, atom: &str, feature: &str) -> Result<bool, &'static str> {
        let record = self.slots.get(atom).ok_or("Atom not found")?;
        match feature {
            "BDEPEND" => Ok(record.eapi >= PortageEapiLevel::Eapi7),
            "IDEPEND" => Ok(record.eapi >= PortageEapiLevel::Eapi8),
            "PROPERTIES_ACCUMULATE" => Ok(record.eapi >= PortageEapiLevel::Eapi8),
            _ => Ok(true),
        }
    }

    pub fn requires_abi_rebuild(&self, atom: &str, target_subslot: &str) -> bool {
        if let Some(record) = self.slots.get(atom) {
            match record.slot_operator {
                SlotOperator::SubslotEqual => record.subslot != target_subslot,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Default for GentooPortageEapiSlotOperatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 19. DragonFly BSD DPorts & HAMMER2 Transactional Snapshot Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hammer2PfsSnapshot {
    pub snapshot_name: String,
    pub transaction_id: u64,
    pub timestamp_sec: u64,
    pub target_pkg: String,
    pub modified_files: Vec<String>,
}

pub struct DragonFlyDportsHammer2SnapshotEngine {
    pub snapshots: Vec<Hammer2PfsSnapshot>,
    pub current_tx_counter: u64,
}

impl DragonFlyDportsHammer2SnapshotEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            current_tx_counter: 1,
        }
    }

    pub fn create_pre_transaction_snapshot(
        &mut self,
        target_pkg: &str,
        modified_files: &[&str],
        now_sec: u64,
    ) -> String {
        let tx_id = self.current_tx_counter;
        self.current_tx_counter += 1;

        let snap_name = format!("@hammer2_snap_tx_{}_{}", tx_id, target_pkg);
        let snapshot = Hammer2PfsSnapshot {
            snapshot_name: snap_name.clone(),
            transaction_id: tx_id,
            timestamp_sec: now_sec,
            target_pkg: target_pkg.to_string(),
            modified_files: modified_files.iter().map(|s| s.to_string()).collect(),
        };
        self.snapshots.push(snapshot);
        snap_name
    }

    pub fn rollback_snapshot(&mut self, snap_name: &str) -> Result<Vec<String>, &'static str> {
        let idx = self
            .snapshots
            .iter()
            .position(|s| s.snapshot_name == snap_name)
            .ok_or("Snapshot not found")?;

        let snap = self.snapshots.remove(idx);
        Ok(snap.modified_files)
    }
}

impl Default for DragonFlyDportsHammer2SnapshotEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 20. Debian Dpkg Triggers & Apt-Listbugs Early Bug Guard
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DpkgTriggerKind {
    Interest,
    Activate,
    InterestNoAwait,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgTrigger {
    pub name: String,
    pub kind: DpkgTriggerKind,
    pub target_package: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptBugReport {
    pub bug_id: u32,
    pub package_name: String,
    pub severity: String, // "critical", "grave", "normal"
    pub title: String,
}

pub struct DebianDpkgTriggersAptListbugsGuardEngine {
    pub pending_triggers: Vec<DpkgTrigger>,
    pub known_bugs: Vec<AptBugReport>,
    pub block_on_critical_bugs: bool,
}

impl DebianDpkgTriggersAptListbugsGuardEngine {
    pub fn new() -> Self {
        Self {
            pending_triggers: Vec::new(),
            known_bugs: Vec::new(),
            block_on_critical_bugs: true,
        }
    }

    pub fn register_trigger(&mut self, trigger: DpkgTrigger) {
        if !self.pending_triggers.contains(&trigger) {
            self.pending_triggers.push(trigger);
        }
    }

    pub fn register_bug_report(&mut self, bug: AptBugReport) {
        self.known_bugs.push(bug);
    }

    pub fn should_block_installation(&self, package_name: &str) -> (bool, Option<String>) {
        if !self.block_on_critical_bugs {
            return (false, None);
        }

        for bug in &self.known_bugs {
            if bug.package_name == package_name {
                let sev = bug.severity.to_lowercase();
                if sev == "critical" || sev == "grave" {
                    return (
                        true,
                        Some(format!("Blocked by bug #{} [{}]: {}", bug.bug_id, bug.severity, bug.title)),
                    );
                }
            }
        }
        (false, None)
    }

    pub fn process_deferred_triggers(&mut self) -> usize {
        let count = self.pending_triggers.len();
        self.pending_triggers.clear();
        count
    }
}

impl Default for DebianDpkgTriggersAptListbugsGuardEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_vuxml_audit() {
        let mut engine = FreeBsdPortsFlavoursAndVuxmlEngine::new();
        engine.register_vuxml_entry(VuXmlEntry {
            vuln_id: "vuln-01".to_string(),
            cve_id: "CVE-2024-1234".to_string(),
            package_name: "openssl".to_string(),
            vulnerable_range_min: "3.0.0".to_string(),
            vulnerable_range_max: "3.0.8".to_string(),
            severity: "High".to_string(),
            summary: "Buffer overflow in OpenSSL SSL_read".to_string(),
        });

        let installed = [("openssl", "3.0.5"), ("curl", "8.2.1")];
        let reports = engine.audit_installed_packages(&installed);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].cve_id, "CVE-2024-1234");
    }

    #[test]
    fn test_xbps_soname_and_orphans() {
        let mut engine = XbpsSonameAndOrphanEngine::new();
        engine.register_package(XbpsPackageRecord {
            name: "glibc".to_string(),
            version: "2.38".to_string(),
            provided_sonames: vec!["libc.so.6".to_string(), "libm.so.6".to_string()],
            required_sonames: vec![],
            is_explicitly_installed: true,
        });

        engine.register_package(XbpsPackageRecord {
            name: "libpng".to_string(),
            version: "1.6.40".to_string(),
            provided_sonames: vec!["libpng16.so.16".to_string()],
            required_sonames: vec!["libc.so.6".to_string(), "libz.so.1".to_string()],
            is_explicitly_installed: false,
        });

        let missing = engine.find_missing_sonames();
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].1, "libz.so.1");

        let orphans = engine.find_orphan_packages();
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0], "libpng");
    }

    #[test]
    fn test_alpine_apk_world() {
        let mut engine = AlpineApkWorldAndVirtualPkgEngine::new();
        engine.add_to_world("bash");
        engine.add_to_world("curl");
        assert_eq!(engine.world_packages.len(), 2);

        engine.add_virtual_group(".build-deps", &["gcc", "make", "musl-dev"]);
        assert_eq!(engine.installed_virtual_groups.len(), 1);

        let removed = engine.remove_virtual_group(".build-deps").unwrap();
        assert_eq!(removed.len(), 3);
    }

    #[test]
    fn test_nix_gc_and_profiles() {
        let mut engine = NixGuixCasGcProfileEngine::new();
        engine.register_gc_root("/nix/var/nix/gcroots/booted-system");
        let gen1 = engine.create_generation("/nix/store/hash1-system-1.0", 1700000000);
        assert_eq!(gen1, 1);

        let dead = engine.scan_dead_store_paths(&[
            "/nix/store/hash1-system-1.0",
            "/nix/store/hash2-unused-lib",
        ]);
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0], "/nix/store/hash2-unused-lib");
    }

    #[test]
    fn test_arch_alpm_hooks() {
        let mut engine = ArchSplitPackageHookRunnerEngine::new();
        engine.register_hook(AlpmPathHook {
            name: "fontconfig.hook".to_string(),
            trigger_paths: vec!["usr/share/fonts/".to_string()],
            action_command: "fc-cache -s".to_string(),
        });

        let triggered = engine.trigger_path_hooks(&["usr/share/fonts/TTF/Roboto.ttf"]);
        assert_eq!(triggered, 1);
        assert_eq!(engine.executed_hooks[0], "fontconfig.hook");
    }

    #[test]
    fn test_fedora_dnf5_advisories() {
        let mut engine = FedoraDnf5AdvisoryAndDeltaRpmEngine::new();
        engine.add_advisory(Dnf5Advisory {
            advisory_id: "FEDORA-2024-001".to_string(),
            severity: "Security".to_string(),
            target_package: "kernel".to_string(),
            fixed_version: "6.8.1".to_string(),
        });

        engine.add_delta_patch(DeltaRpmPatch {
            package_name: "kernel".to_string(),
            old_version: "6.8.0".to_string(),
            new_version: "6.8.1".to_string(),
            delta_size_bytes: 15_000_000,
            full_size_bytes: 120_000_000,
        });

        let sec = engine.filter_advisories_by_severity("security");
        assert_eq!(sec.len(), 1);
        assert_eq!(engine.calculate_bandwidth_savings(), 105_000_000);
    }

    #[test]
    fn test_portage_subslots() {
        let mut engine = GentooPortageSubslotAndUseExpandEngine::new();
        engine.register_subslot(PortagePackageSubslot {
            atom: "dev-lang/perl".to_string(),
            slot: "0".to_string(),
            subslot: "5.38".to_string(),
            dependent_atoms: vec!["dev-perl/DBI".to_string(), "net-mail/mailx".to_string()],
        });

        let rebuilds = engine.evaluate_abi_rebuilds("dev-lang/perl", "5.40");
        assert_eq!(rebuilds.len(), 2);
    }

    #[test]
    fn test_haiku_packagefs() {
        let mut engine = HaikuHpkgPackageFsEngine::new();
        let mount = engine.mount_hpkg("haiku_base-r1", "/boot/system/packages/haiku_base.hpkg");
        assert_eq!(mount, "/system/packages/haiku_base-r1");
        assert_eq!(engine.active_mount_count(), 1);

        assert!(engine.unmount_hpkg("haiku_base-r1"));
        assert_eq!(engine.active_mount_count(), 0);
    }

    #[test]
    fn test_slackware_slackbuild_parser() {
        let info_text = r#"
PRGNAM="htop"
VERSION="3.3.0"
HOMEPAGE="https://htop.dev"
DOWNLOAD="https://github.com/htop-dev/htop/archive/3.3.0.tar.gz"
MD5SUM="abc123def456"
REQUIRES="ncurses"
MAINTAINER="SigmaOS"
"#;
        let info = SlackwarePkgtoolSlackBuildEngine::parse_slackbuild_info(info_text).unwrap();
        assert_eq!(info.prgnam, "htop");
        assert_eq!(info.version, "3.3.0");
        assert_eq!(info.requires, vec!["ncurses".to_string()]);

        let mut engine = SlackwarePkgtoolSlackBuildEngine::new();
        engine.register_slackbuild(info);
        assert_eq!(engine.slackbuilds.len(), 1);
    }

    #[test]
    fn test_ubuntu_ppa_apt_pinning() {
        let mut engine = UbuntuPpaAptPinningEngine::new();
        let ppa = engine.add_ppa("graphics-drivers", "ppa", "jammy", "12345678ABCD");
        assert_eq!(ppa, "ppa:graphics-drivers/ppa");
        assert!(engine.verify_key_trusted("12345678ABCD"));

        engine.add_pin_rule("nvidia-*", "jammy", 1001);
        assert_eq!(engine.resolve_effective_priority("nvidia-driver-535", "jammy"), 1001);
        assert_eq!(engine.resolve_effective_priority("curl", "jammy"), 500);
    }

    #[test]
    fn test_opensuse_zypper_vendor_stickiness() {
        let mut engine = OpenSuseZypperVendorStickinessEngine::new();
        engine.register_repo(ZypperRepository {
            repo_alias: "packman".to_string(),
            name: "Packman Repository".to_string(),
            priority: 90,
            vendor: "Packman".to_string(),
            is_enabled: true,
        });
        engine.register_repo(ZypperRepository {
            repo_alias: "openSUSE-OSS".to_string(),
            name: "openSUSE Main OSS".to_string(),
            priority: 99,
            vendor: "openSUSE".to_string(),
            is_enabled: true,
        });

        let offers = vec![
            ZypperPackageOffer {
                package_name: "ffmpeg".to_string(),
                version: "6.1.0".to_string(),
                vendor: "Packman".to_string(),
                repo_alias: "packman".to_string(),
            },
            ZypperPackageOffer {
                package_name: "ffmpeg".to_string(),
                version: "6.0.0".to_string(),
                vendor: "openSUSE".to_string(),
                repo_alias: "openSUSE-OSS".to_string(),
            },
        ];

        // Stickiness enabled: retains openSUSE vendor
        let offer = engine.select_best_offer(Some("openSUSE"), &offers).unwrap();
        assert_eq!(offer.vendor, "openSUSE");

        // Allow vendor change -> picks higher priority Packman repo
        engine.vendor_change_allowed = true;
        let offer2 = engine.select_best_offer(Some("openSUSE"), &offers).unwrap();
        assert_eq!(offer2.vendor, "Packman");
    }

    #[test]
    fn test_nix_flakes_devshell_resolver() {
        let mut engine = NixFlakesDevshellResolverEngine::new();
        engine.lock_input(FlakeInputLock {
            input_name: "nixpkgs".to_string(),
            original_url: "github:NixOS/nixpkgs/nixos-unstable".to_string(),
            locked_nar_hash: "sha256-nar-hash-123".to_string(),
            rev: "git-commit-rev-456".to_string(),
        });

        assert!(engine.verify_lockfile_reproducibility());

        engine.export_devshell_var("CC", "gcc");
        engine.add_devshell_package("pkg-config");

        let manifest = engine.generate_devshell_manifest();
        assert!(manifest.contains("export CC=\"gcc\""));
        assert!(manifest.contains("- pkg-config"));
    }

    #[test]
    fn test_openbsd_signify_pkg_add() {
        let mut engine = OpenBsdPkgAddSignifyEngine::new();
        engine.add_signify_key("openbsd-75-base", "pubkey_base64_data_xyz");
        engine.add_mirror("https://cdn.openbsd.org/pub/OpenBSD/7.5/packages/amd64/");

        assert!(engine.verify_signify_signature("openbsd-75-base", "signed_by_pubkey_base64_data_xyz"));

        let url = engine.resolve_package_download_url("zsh-5.9.tgz").unwrap();
        assert_eq!(url, "https://cdn.openbsd.org/pub/OpenBSD/7.5/packages/amd64/zsh-5.9.tgz");
    }

    #[test]
    fn test_debian_debconf_statoverride() {
        let mut engine = DebianDebconfStatoverrideEngine::new();
        engine.add_preseed_answer(DebconfPreseedEntry {
            package_owner: "tzdata".to_string(),
            question_template: "zones/default".to_string(),
            qtype: DebconfQuestionType::Select,
            answer_value: "UTC".to_string(),
        });

        engine.add_statoverride(DpkgStatoverrideRule {
            owner_user: "root".to_string(),
            owner_group: "shadow".to_string(),
            mode_octal: 0o4750,
            target_path: "/usr/bin/expiry".to_string(),
        });

        let ans = engine.query_preseed_answer("tzdata", "zones/default");
        assert_eq!(ans, Some("UTC".to_string()));

        let stat = engine.get_statoverride_for_path("/usr/bin/expiry").unwrap();
        assert_eq!(stat.mode_octal, 0o4750);
        assert_eq!(stat.owner_group, "shadow");
    }

    #[test]
    fn test_arch_cachyos_microarch() {
        let mut engine = ArchCachyosMicroarchOptimizationEngine::new();
        engine.register_repo_route(MicroarchitectureLevel::V1, "https://repo.sigmaos.org/core/x86_64");
        engine.register_repo_route(MicroarchitectureLevel::V3, "https://repo.sigmaos.org/cachyos/x86_64-v3");

        let level = engine.detect_microarch_level(&[
            "sse3", "ssse3", "sse4_1", "sse4_2", "popcnt", "avx", "avx2", "bmi1", "bmi2", "fma", "f16c", "lzcnt",
        ]);
        assert_eq!(level, MicroarchitectureLevel::V3);

        let repo = engine.resolve_optimal_repo();
        assert_eq!(repo, "https://repo.sigmaos.org/cachyos/x86_64-v3");
    }

    #[test]
    fn test_copr_aur_obs_gateway() {
        let mut engine = CoprAurBuildRepositoryGatewayEngine::new();
        engine.register_source(CommunityPackageBuildSource {
            name: "yay".to_string(),
            backend: CommunityRepoBackend::ArchAur,
            repository_owner: "Jguer".to_string(),
            source_url: "https://aur.archlinux.org/yay.git".to_string(),
            trust_score: 85,
            is_sandboxed_build: true,
        });

        assert!(engine.can_build_safely("yay").unwrap());

        let cmd = engine.generate_build_sandbox_cmd("yay").unwrap();
        assert!(cmd.contains("makepkg"));
    }

    #[test]
    fn test_pkgsrc_options_framework() {
        let mut engine = NetBsdPkgsrcOptionsFrameworkEngine::new();
        engine.register_option(
            PkgsrcOptionSpec {
                option_name: "ssl".to_string(),
                description: "Enable OpenSSL support".to_string(),
                requires_options: vec![],
                conflicts_with: vec![],
            },
            true,
        );

        engine.register_option(
            PkgsrcOptionSpec {
                option_name: "inet6".to_string(),
                description: "Enable IPv6 support".to_string(),
                requires_options: vec![],
                conflicts_with: vec![],
            },
            false,
        );

        assert!(engine.validate_options().is_ok());
        engine.toggle_option("+inet6").unwrap();
        assert_eq!(engine.active_options.len(), 2);
    }

    #[test]
    fn test_gentoo_portage_eapi_slots() {
        let mut engine = GentooPortageEapiSlotOperatorEngine::new();
        engine.register_ebuild_slot(EbuildSlotRecord {
            atom: "dev-libs/openssl".to_string(),
            slot: "0".to_string(),
            subslot: "3.0".to_string(),
            eapi: PortageEapiLevel::Eapi8,
            slot_operator: SlotOperator::SubslotEqual,
        });

        assert!(engine.check_eapi_feature_support("dev-libs/openssl", "IDEPEND").unwrap());
        assert!(engine.requires_abi_rebuild("dev-libs/openssl", "3.1"));
        assert!(!engine.requires_abi_rebuild("dev-libs/openssl", "3.0"));
    }

    #[test]
    fn test_hammer2_dports_snapshot() {
        let mut engine = DragonFlyDportsHammer2SnapshotEngine::new();
        let snap = engine.create_pre_transaction_snapshot(
            "nginx",
            &["/usr/local/sbin/nginx", "/etc/nginx/nginx.conf"],
            1700000000,
        );
        assert!(snap.contains("nginx"));

        let restored = engine.rollback_snapshot(&snap).unwrap();
        assert_eq!(restored.len(), 2);
    }

    #[test]
    fn test_debian_dpkg_triggers_apt_listbugs() {
        let mut engine = DebianDpkgTriggersAptListbugsGuardEngine::new();
        engine.register_bug_report(AptBugReport {
            bug_id: 1029384,
            package_name: "libglib2.0-0".to_string(),
            severity: "critical".to_string(),
            title: "Memory corruption in g_string_append".to_string(),
        });

        let (blocked, reason) = engine.should_block_installation("libglib2.0-0");
        assert!(blocked);
        assert!(reason.unwrap().contains("Blocked by bug #1029384"));

        engine.register_trigger(DpkgTrigger {
            name: "glib-compile-schemas".to_string(),
            kind: DpkgTriggerKind::InterestNoAwait,
            target_package: "libglib2.0-0".to_string(),
        });

        let processed = engine.process_deferred_triggers();
        assert_eq!(processed, 1);
    }
}
