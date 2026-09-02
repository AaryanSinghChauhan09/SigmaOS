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
// 15. Arch Linux SvnToGit & Pkgctl Repo Maintainer Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PkgctlBranchChannel {
    Main,
    Testing,
    Staging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchGitPackageRepository {
    pub package_name: String,
    pub current_branch: PkgctlBranchChannel,
    pub pkgver: String,
    pub pkgrel: u32,
    pub maintainer_pgp_key: String,
    pub git_commit_hash: String,
    pub has_srcinfo: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvnLegacyPackageRecord {
    pub svn_path: String,
    pub trunk_pkgbuild: String,
    pub revision: u64,
}

pub struct ArchSvnToGitPkgctlMaintainerEngine {
    pub git_repositories: BTreeMap<String, ArchGitPackageRepository>,
    pub converted_svn_records: Vec<SvnLegacyPackageRecord>,
}

impl ArchSvnToGitPkgctlMaintainerEngine {
    pub fn new() -> Self {
        Self {
            git_repositories: BTreeMap::new(),
            converted_svn_records: Vec::new(),
        }
    }

    /// Converts a legacy SVN package layout into a modern Git repository structure (svntogit migration)
    pub fn convert_svn_layout_to_git_repo(
        &mut self,
        svn_record: SvnLegacyPackageRecord,
        package_name: &str,
        pkgver: &str,
        pkgrel: u32,
    ) -> ArchGitPackageRepository {
        let commit_hash = format!("{:016x}{:016x}", svn_record.revision, svn_record.revision ^ 0x9E3779B9);
        let repo = ArchGitPackageRepository {
            package_name: package_name.to_string(),
            current_branch: PkgctlBranchChannel::Main,
            pkgver: pkgver.to_string(),
            pkgrel,
            maintainer_pgp_key: "PGP_KEY_ARCH_MAINTAINER_42".to_string(),
            git_commit_hash: commit_hash,
            has_srcinfo: true,
        };
        self.converted_svn_records.push(svn_record);
        self.git_repositories.insert(package_name.to_string(), repo.clone());
        repo
    }

    /// `pkgctl repo switch` - Switches the package repository branch (e.g. Main -> Testing -> Staging)
    pub fn pkgctl_repo_switch(
        &mut self,
        package_name: &str,
        target_branch: PkgctlBranchChannel,
    ) -> Result<String, &'static str> {
        if let Some(repo) = self.git_repositories.get_mut(package_name) {
            repo.current_branch = target_branch.clone();
            Ok(format!("Switched {} to branch {:?}", package_name, target_branch))
        } else {
            Err("Package git repository not found")
        }
    }

    /// `pkgctl db release` - Generates signed tag and syncs package release into target repo DB
    pub fn pkgctl_db_release(
        &self,
        package_name: &str,
        pgp_signature: &str,
    ) -> Result<String, &'static str> {
        let repo = self
            .git_repositories
            .get(package_name)
            .ok_or("Package repository not found")?;

        if !pgp_signature.contains(&repo.maintainer_pgp_key) {
            return Err("Invalid maintainer PGP signature for pkgctl release");
        }

        let release_tag = format!("v{}-{}-{}", repo.pkgver, repo.pkgrel, &repo.git_commit_hash[..8]);
        Ok(format!(
            "Released {} [{}] to channel {:?} with tag {}",
            repo.package_name, release_tag, repo.current_branch, release_tag
        ))
    }

    /// Generates standard .SRCINFO file metadata for Arch Linux repository index
    pub fn generate_srcinfo_manifest(&self, package_name: &str) -> Option<String> {
        let repo = self.git_repositories.get(package_name)?;
        let mut out = String::from("# Generated by pkgctl srcinfo\n");
        out.push_str(&format!("pkgbase = {}\n", repo.package_name));
        out.push_str(&format!("\tpkgver = {}\n", repo.pkgver));
        out.push_str(&format!("\tpkgrel = {}\n", repo.pkgrel));
        out.push_str(&format!("pkgname = {}\n", repo.package_name));
        Some(out)
    }
}

impl Default for ArchSvnToGitPkgctlMaintainerEngine {
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
    fn test_arch_svntogit_pkgctl_maintainer_engine() {
        let mut engine = ArchSvnToGitPkgctlMaintainerEngine::new();
        let svn_rec = SvnLegacyPackageRecord {
            svn_path: "extra-x86_64/glibc".to_string(),
            trunk_pkgbuild: "pkgname=glibc\npkgver=2.38\npkgrel=1\n".to_string(),
            revision: 485120,
        };

        let repo = engine.convert_svn_layout_to_git_repo(svn_rec, "glibc", "2.38", 1);
        assert_eq!(repo.package_name, "glibc");
        assert_eq!(repo.current_branch, PkgctlBranchChannel::Main);

        assert!(engine
            .pkgctl_repo_switch("glibc", PkgctlBranchChannel::Testing)
            .is_ok());

        let release = engine.pkgctl_db_release("glibc", "signed_by_PGP_KEY_ARCH_MAINTAINER_42");
        assert!(release.is_ok());
        assert!(release.unwrap().contains("Released glibc"));

        let srcinfo = engine.generate_srcinfo_manifest("glibc");
        assert!(srcinfo.is_some());
        assert!(srcinfo.unwrap().contains("pkgbase = glibc"));
    }
}
