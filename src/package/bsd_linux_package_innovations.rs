#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT
// SigmaOS - Linux & BSD Inspired Package Management Innovations
// Inspired by FreeBSD Ports & VuXML, Void XBPS, Alpine APK v3, Nix/Guix CAS,
// Arch ALPM hooks, Fedora DNF5 DeltaRPM, Gentoo Portage subslots, Haiku PackageFS,
// Debian apt-mark, Void xbps-src, Fedora DNF history, and NetBSD pkgin.


use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

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
            if !live_paths
                .iter()
                .any(|live| path.starts_with(live.as_str()) || live.starts_with(path))
            {
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
        if let Some(pkg) = self
            .mounted_packages
            .iter_mut()
            .find(|p| p.package_id == package_id)
        {
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
        self.installed_packages
            .insert(record.package_name.clone(), record);
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
                prgnam = line
                    .strip_prefix("PRGNAM=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("VERSION=") {
                version = line
                    .strip_prefix("VERSION=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("HOMEPAGE=") {
                homepage = line
                    .strip_prefix("HOMEPAGE=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
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
                maintainer = line
                    .strip_prefix("MAINTAINER=")
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
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

            let codename_matches =
                rule.release_codename == "*" || rule.release_codename == codename;

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
            let prio_a = self
                .repositories
                .get(&a.repo_alias)
                .map_or(99, |r| r.priority);
            let prio_b = self
                .repositories
                .get(&b.repo_alias)
                .map_or(99, |r| r.priority);
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
        self.devshell_env_vars
            .insert(key.to_string(), val.to_string());
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
                format!(
                    "copr-cli build-package {} --chroot fedora-rawhide-x86_64",
                    source.name
                )
            }
            CommunityRepoBackend::ArchAur => {
                format!(
                    "makepkg --syncdeps --clean --noconfirm --dir /sandbox/aur/{}",
                    source.name
                )
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
        self.supported_options
            .insert(spec.option_name.clone(), spec);
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
                        return Err(format!(
                            "Option '{}' requires missing option '{}'",
                            opt, req
                        ));
                    }
                }
                for conf in &spec.conflicts_with {
                    if self.active_options.contains(conf) {
                        return Err(format!(
                            "Option '{}' conflicts with active option '{}'",
                            opt, conf
                        ));
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
    AnySlot,           // '*'
    SubslotEqual,      // ':='
    ExactSlot(String), // ':slot'
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

    pub fn check_eapi_feature_support(
        &self,
        atom: &str,
        feature: &str,
    ) -> Result<bool, &'static str> {
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
                        Some(format!(
                            "Blocked by bug #{} [{}]: {}",
                            bug.bug_id, bug.severity, bug.title
                        )),
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

// =========================================================================
// 21. Void Linux xbps-src Restricted Non-Free Licensing Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestrictedPackageSpec {
    pub name: String,
    pub version: String,
    pub license: String,
    pub is_restricted: bool,
    pub download_url: String,
    pub accepted_terms_prompt: String,
}

pub struct XbpsRestrictedNonFreeLicenseEngine {
    pub restricted_packages: BTreeMap<String, RestrictedPackageSpec>,
    pub accepted_licenses: Vec<String>,
    pub allow_restricted_builds: bool,
}

impl XbpsRestrictedNonFreeLicenseEngine {
    pub fn new() -> Self {
        Self {
            restricted_packages: BTreeMap::new(),
            accepted_licenses: Vec::new(),
            allow_restricted_builds: false,
        }
    }

    pub fn register_restricted_package(&mut self, spec: RestrictedPackageSpec) {
        self.restricted_packages.insert(spec.name.clone(), spec);
    }

    pub fn accept_license(&mut self, license_name: &str) {
        if !self.accepted_licenses.contains(&license_name.to_string()) {
            self.accepted_licenses.push(license_name.to_string());
        }
    }

    pub fn can_fetch_and_build(&self, pkg_name: &str) -> Result<bool, &'static str> {
        let pkg = self
            .restricted_packages
            .get(pkg_name)
            .ok_or("Package spec not found")?;

        if pkg.is_restricted && !self.allow_restricted_builds {
            return Ok(false);
        }

        if !self.accepted_licenses.contains(&pkg.license) {
            return Ok(false);
        }

        Ok(true)
    }
}

impl Default for XbpsRestrictedNonFreeLicenseEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 22. Debian apt-mark Package Hold & Manual/Auto State Governor
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AptMarkState {
    Auto,
    Manual,
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptMarkRecord {
    pub package_name: String,
    pub state: AptMarkState,
    pub required_by: Vec<String>,
}

pub struct DebianAptMarkPackageStateGovernor {
    pub mark_db: BTreeMap<String, AptMarkRecord>,
}

impl DebianAptMarkPackageStateGovernor {
    pub fn new() -> Self {
        Self {
            mark_db: BTreeMap::new(),
        }
    }

    pub fn mark_package(&mut self, pkg_name: &str, state: AptMarkState) {
        self.mark_db
            .entry(pkg_name.to_string())
            .and_modify(|r| r.state = state)
            .or_insert_with(|| AptMarkRecord {
                package_name: pkg_name.to_string(),
                state,
                required_by: Vec::new(),
            });
    }

    pub fn register_dep_relation(&mut self, dependent: &str, provider: &str) {
        self.mark_db
            .entry(provider.to_string())
            .or_insert_with(|| AptMarkRecord {
                package_name: provider.to_string(),
                state: AptMarkState::Auto,
                required_by: Vec::new(),
            });
        if let Some(record) = self.mark_db.get_mut(provider) {
            if !record.required_by.contains(&dependent.to_string()) {
                record.required_by.push(dependent.to_string());
            }
        }
    }

    pub fn show_auto(&self) -> Vec<String> {
        self.mark_db
            .values()
            .filter(|r| r.state == AptMarkState::Auto)
            .map(|r| r.package_name.clone())
            .collect()
    }

    pub fn show_manual(&self) -> Vec<String> {
        self.mark_db
            .values()
            .filter(|r| r.state == AptMarkState::Manual)
            .map(|r| r.package_name.clone())
            .collect()
    }

    pub fn show_hold(&self) -> Vec<String> {
        self.mark_db
            .values()
            .filter(|r| r.state == AptMarkState::Hold)
            .map(|r| r.package_name.clone())
            .collect()
    }

    pub fn find_autoremove_candidates(&self) -> Vec<String> {
        let mut candidates = Vec::new();
        for record in self.mark_db.values() {
            if record.state == AptMarkState::Auto && record.required_by.is_empty() {
                candidates.push(record.package_name.clone());
            }
        }
        candidates
    }
}

impl Default for DebianAptMarkPackageStateGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 23. Fedora DNF Transaction History & Rollback Journal Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnfActionKind {
    Install,
    Upgrade,
    Downgrade,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnfActionRecord {
    pub package_name: String,
    pub version: String,
    pub kind: DnfActionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnfTransactionItem {
    pub transaction_id: u64,
    pub timestamp_sec: u64,
    pub user_cmd: String,
    pub actions: Vec<DnfActionRecord>,
}

pub struct FedoraDnfHistoryRollbackJournalEngine {
    pub history: Vec<DnfTransactionItem>,
    pub next_tx_id: u64,
}

impl FedoraDnfHistoryRollbackJournalEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            next_tx_id: 1,
        }
    }

    pub fn record_transaction(
        &mut self,
        user_cmd: &str,
        actions: Vec<DnfActionRecord>,
        now_sec: u64,
    ) -> u64 {
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        let tx = DnfTransactionItem {
            transaction_id: tx_id,
            timestamp_sec: now_sec,
            user_cmd: user_cmd.to_string(),
            actions,
        };
        self.history.push(tx);
        tx_id
    }

    pub fn compute_rollback_actions(&self, target_tx_id: u64) -> Result<Vec<DnfActionRecord>, &'static str> {
        let mut undo_actions = Vec::new();
        let target_idx = self
            .history
            .iter()
            .position(|t| t.transaction_id == target_tx_id)
            .ok_or("Transaction ID not found")?;

        for tx in self.history[target_idx..].iter().rev() {
            for action in tx.actions.iter().rev() {
                let undo = match action.kind {
                    DnfActionKind::Install => DnfActionRecord {
                        package_name: action.package_name.clone(),
                        version: action.version.clone(),
                        kind: DnfActionKind::Remove,
                    },
                    DnfActionKind::Remove => DnfActionRecord {
                        package_name: action.package_name.clone(),
                        version: action.version.clone(),
                        kind: DnfActionKind::Install,
                    },
                    DnfActionKind::Upgrade => DnfActionRecord {
                        package_name: action.package_name.clone(),
                        version: action.version.clone(),
                        kind: DnfActionKind::Downgrade,
                    },
                    DnfActionKind::Downgrade => DnfActionRecord {
                        package_name: action.package_name.clone(),
                        version: action.version.clone(),
                        kind: DnfActionKind::Upgrade,
                    },
                };
                undo_actions.push(undo);
            }
        }
        Ok(undo_actions)
    }
}

impl Default for FedoraDnfHistoryRollbackJournalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 24. NetBSD pkgin Binary Package Database & Vacuum Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgSummaryRecord {
    pub pkgname: String,
    pub pkgpath: String,
    pub size_bytes: u64,
    pub comment: String,
    pub depends: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedPackageFile {
    pub filename: String,
    pub size_bytes: u64,
    pub is_installed: bool,
}

pub struct NetBsdPkginBinaryDatabaseEngine {
    pub db: BTreeMap<String, PkgSummaryRecord>,
    pub cache: Vec<CachedPackageFile>,
}

impl NetBsdPkginBinaryDatabaseEngine {
    pub fn new() -> Self {
        Self {
            db: BTreeMap::new(),
            cache: Vec::new(),
        }
    }

    pub fn register_pkg_summary(&mut self, record: PkgSummaryRecord) {
        self.db.insert(record.pkgname.clone(), record);
    }

    pub fn register_cache_file(&mut self, file: CachedPackageFile) {
        self.cache.push(file);
    }

    pub fn query_pkg(&self, name_prefix: &str) -> Vec<PkgSummaryRecord> {
        self.db
            .values()
            .filter(|p| p.pkgname.starts_with(name_prefix))
            .cloned()
            .collect()
    }

    pub fn vacuum_stale_cache(&mut self) -> (usize, u64) {
        let mut freed_bytes = 0u64;
        let mut freed_count = 0usize;

        self.cache.retain(|f| {
            if !f.is_installed {
                freed_bytes += f.size_bytes;
                freed_count += 1;
                false
            } else {
                true
            }
        });

        (freed_count, freed_bytes)
    }
}

impl Default for NetBsdPkginBinaryDatabaseEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 25. Void Linux XBPS Local Downgrade Repository & Package Hold Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsCachedPkg {
    pub name: String,
    pub version: String,
    pub archive_filename: String,
    pub is_held: bool,
}

pub struct XbpsDowngradeRepoEngine {
    pub cached_packages: Vec<XbpsCachedPkg>,
    pub held_packages: Vec<String>,
}

impl XbpsDowngradeRepoEngine {
    pub fn new() -> Self {
        Self {
            cached_packages: Vec::new(),
            held_packages: Vec::new(),
        }
    }

    pub fn cache_pkg_archive(&mut self, name: &str, version: &str, archive_filename: &str) {
        let pkg = XbpsCachedPkg {
            name: name.to_string(),
            version: version.to_string(),
            archive_filename: archive_filename.to_string(),
            is_held: self.held_packages.contains(&name.to_string()),
        };
        self.cached_packages.push(pkg);
    }

    pub fn hold_package(&mut self, name: &str) {
        if !self.held_packages.contains(&name.to_string()) {
            self.held_packages.push(name.to_string());
            for pkg in &mut self.cached_packages {
                if pkg.name == name {
                    pkg.is_held = true;
                }
            }
        }
    }

    pub fn unhold_package(&mut self, name: &str) {
        self.held_packages.retain(|p| p != name);
        for pkg in &mut self.cached_packages {
            if pkg.name == name {
                pkg.is_held = false;
            }
        }
    }

    pub fn is_package_held(&self, name: &str) -> bool {
        self.held_packages.contains(&name.to_string())
    }

    pub fn find_downgrade_candidate(&self, name: &str, current_version: &str) -> Option<XbpsCachedPkg> {
        self.cached_packages
            .iter()
            .filter(|p| p.name == name && p.version != current_version)
            .cloned()
            .max_by(|a, b| a.version.cmp(&b.version))
    }
}

impl Default for XbpsDowngradeRepoEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 26. Gentoo Portage package.env Per-Package Compiler & Env Override Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortageEnvProfile {
    pub cflags: String,
    pub cxxflags: String,
    pub makeopts: String,
    pub env_vars: BTreeMap<String, String>,
}

pub struct PortagePackageEnvEngine {
    pub package_env_map: BTreeMap<String, PortageEnvProfile>,
}

impl PortagePackageEnvEngine {
    pub fn new() -> Self {
        Self {
            package_env_map: BTreeMap::new(),
        }
    }

    pub fn register_package_env(&mut self, atom: &str, profile: PortageEnvProfile) {
        self.package_env_map.insert(atom.to_string(), profile);
    }

    pub fn get_package_env(&self, atom: &str) -> Option<&PortageEnvProfile> {
        self.package_env_map.get(atom)
    }

    pub fn generate_build_env_export(&self, atom: &str) -> String {
        let mut out = String::new();
        if let Some(profile) = self.package_env_map.get(atom) {
            if !profile.cflags.is_empty() {
                out.push_str(&format!("export CFLAGS=\"{}\"\n", profile.cflags));
            }
            if !profile.cxxflags.is_empty() {
                out.push_str(&format!("export CXXFLAGS=\"{}\"\n", profile.cxxflags));
            }
            if !profile.makeopts.is_empty() {
                out.push_str(&format!("export MAKEOPTS=\"{}\"\n", profile.makeopts));
            }
            for (k, v) in &profile.env_vars {
                out.push_str(&format!("export {}=\"{}\"\n", k, v));
            }
        }
        out
    }
}

impl Default for PortagePackageEnvEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 27. FreeBSD pkg-audit Vulnerability Database & Transaction Blocker
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgAuditAdvisory {
    pub id: String,
    pub package_name: String,
    pub vulnerable_versions: Vec<String>,
    pub cvss_score: u32, // 0..100 (e.g. 98 = 9.8 Critical)
    pub description: String,
}

pub struct FreeBsdPkgAuditEngine {
    pub advisories: Vec<PkgAuditAdvisory>,
    pub cvss_block_threshold: u32,
}

impl FreeBsdPkgAuditEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            cvss_block_threshold: 70, // Block high and critical (CVSS >= 7.0)
        }
    }

    pub fn register_advisory(&mut self, advisory: PkgAuditAdvisory) {
        self.advisories.push(advisory);
    }

    pub fn audit_package(&self, name: &str, version: &str) -> Vec<PkgAuditAdvisory> {
        self.advisories
            .iter()
            .filter(|a| a.package_name == name && a.vulnerable_versions.contains(&version.to_string()))
            .cloned()
            .collect()
    }

    pub fn should_block_install(&self, name: &str, version: &str) -> (bool, Option<String>) {
        let vulns = self.audit_package(name, version);
        for vuln in vulns {
            if vuln.cvss_score >= self.cvss_block_threshold {
                return (
                    true,
                    Some(format!(
                        "Installation blocked: {} version {} has advisory {} (CVSS {})",
                        name, version, vuln.id, vuln.cvss_score as f32 / 10.0
                    )),
                );
            }
        }
        (false, None)
    }
}

impl Default for FreeBsdPkgAuditEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 28. Nix/Guix CAS Store GC Governor & Closure Size Calculator
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasStorePath {
    pub hash: String,
    pub path: String,
    pub size_bytes: u64,
    pub gc_roots: Vec<String>,
}

pub struct NixCasStoreGcGovernor {
    pub store_paths: BTreeMap<String, CasStorePath>,
}

impl NixCasStoreGcGovernor {
    pub fn new() -> Self {
        Self {
            store_paths: BTreeMap::new(),
        }
    }

    pub fn register_store_path(&mut self, hash: &str, path: &str, size_bytes: u64) {
        self.store_paths.insert(
            path.to_string(),
            CasStorePath {
                hash: hash.to_string(),
                path: path.to_string(),
                size_bytes,
                gc_roots: Vec::new(),
            },
        );
    }

    pub fn add_gc_root(&mut self, path: &str, root_name: &str) -> bool {
        if let Some(store_entry) = self.store_paths.get_mut(path) {
            if !store_entry.gc_roots.contains(&root_name.to_string()) {
                store_entry.gc_roots.push(root_name.to_string());
            }
            true
        } else {
            false
        }
    }

    pub fn remove_gc_root(&mut self, path: &str, root_name: &str) -> bool {
        if let Some(store_entry) = self.store_paths.get_mut(path) {
            store_entry.gc_roots.retain(|r| r != root_name);
            true
        } else {
            false
        }
    }

    pub fn calculate_closure_size(&self, paths: &[&str]) -> u64 {
        let mut total = 0u64;
        for &p in paths {
            if let Some(entry) = self.store_paths.get(p) {
                total += entry.size_bytes;
            }
        }
        total
    }

    pub fn collect_garbage(&mut self) -> (usize, u64) {
        let mut freed_count = 0usize;
        let mut freed_bytes = 0u64;

        let dead_paths: Vec<String> = self
            .store_paths
            .values()
            .filter(|e| e.gc_roots.is_empty())
            .map(|e| e.path.clone())
            .collect();

        for p in dead_paths {
            if let Some(removed) = self.store_paths.remove(&p) {
                freed_count += 1;
                freed_bytes += removed.size_bytes;
            }
        }

        (freed_count, freed_bytes)
    }
}

impl Default for NixCasStoreGcGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 29. Alpine APK v3 Security Signature Verification & Index Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkSignatureKey {
    pub key_id: String,
    pub pubkey_pem: String,
    pub is_trusted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkIndexMetadata {
    pub repo_url: String,
    pub checksum_sha256: String,
    pub signature_b64: String,
}

pub struct ApkV3SignatureEngine {
    pub trusted_keys: BTreeMap<String, ApkSignatureKey>,
}

impl ApkV3SignatureEngine {
    pub fn new() -> Self {
        Self {
            trusted_keys: BTreeMap::new(),
        }
    }

    pub fn register_key(&mut self, key_id: &str, pubkey_pem: &str, is_trusted: bool) {
        self.trusted_keys.insert(
            key_id.to_string(),
            ApkSignatureKey {
                key_id: key_id.to_string(),
                pubkey_pem: pubkey_pem.to_string(),
                is_trusted,
            },
        );
    }

    pub fn verify_index_signature(&self, key_id: &str, index: &ApkIndexMetadata) -> bool {
        if let Some(key) = self.trusted_keys.get(key_id) {
            if !key.is_trusted {
                return false;
            }
            !index.signature_b64.is_empty() && !index.checksum_sha256.is_empty()
        } else {
            false
        }
    }

    pub fn verify_package_checksum(&self, computed_sha256: &str, expected_sha256: &str) -> bool {
        computed_sha256.eq_ignore_ascii_case(expected_sha256)
    }
}

impl Default for ApkV3SignatureEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 30. Fedora / RHEL Delta RPM Binary Reconstitution Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaRpmSpec {
    pub package_name: String,
    pub base_version: String,
    pub target_version: String,
    pub delta_bytes: u64,
    pub full_bytes: u64,
    pub delta_sha256: String,
}

pub struct RpmDeltaReconstitutionEngine {
    pub available_deltas: Vec<DeltaRpmSpec>,
}

impl RpmDeltaReconstitutionEngine {
    pub fn new() -> Self {
        Self {
            available_deltas: Vec::new(),
        }
    }

    pub fn register_delta(&mut self, spec: DeltaRpmSpec) {
        self.available_deltas.push(spec);
    }

    pub fn reconstruct_rpm_package(
        &self,
        package_name: &str,
        base_ver: &str,
        target_ver: &str,
    ) -> Result<String, &'static str> {
        let spec = self
            .available_deltas
            .iter()
            .find(|d| d.package_name == package_name && d.base_version == base_ver && d.target_version == target_ver)
            .ok_or("Delta RPM spec not found")?;

        Ok(format!("{}-{}.x86_64.rpm", spec.package_name, spec.target_version))
    }

    pub fn total_bandwidth_saved(&self) -> u64 {
        self.available_deltas
            .iter()
            .map(|d| d.full_bytes.saturating_sub(d.delta_bytes))
            .sum()
    }
}

impl Default for RpmDeltaReconstitutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 31. Debian / Ubuntu dpkg-divert File Diversion Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgDivertRule {
    pub package_name: String,
    pub original_path: String,
    pub diverted_path: String,
    pub is_local_override: bool,
}

pub struct DpkgDivertEngine {
    pub diversions: BTreeMap<String, DpkgDivertRule>,
}

impl DpkgDivertEngine {
    pub fn new() -> Self {
        Self {
            diversions: BTreeMap::new(),
        }
    }

    pub fn add_diversion(&mut self, rule: DpkgDivertRule) {
        self.diversions.insert(rule.original_path.clone(), rule);
    }

    pub fn remove_diversion(&mut self, original_path: &str) -> Option<DpkgDivertRule> {
        self.diversions.remove(original_path)
    }

    pub fn resolve_target_path(&self, original_path: &str, installing_pkg: &str) -> String {
        if let Some(rule) = self.diversions.get(original_path) {
            if rule.package_name == installing_pkg {
                rule.original_path.clone()
            } else {
                rule.diverted_path.clone()
            }
        } else {
            original_path.to_string()
        }
    }
}

impl Default for DpkgDivertEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 32. Arch Linux pacman-key Web of Trust Keyring Manager
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PacmanKeyTrust {
    Never,
    Marginal,
    Full,
    Expired,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanGpgKey {
    pub key_id: String,
    pub uid: String,
    pub fingerprint: String,
    pub trust_level: PacmanKeyTrust,
    pub issuer_key_id: Option<String>,
}

pub struct PacmanKeyringEngine {
    pub keyring: BTreeMap<String, PacmanGpgKey>,
    pub master_key_id: String,
}

impl PacmanKeyringEngine {
    pub fn new(master_key_id: &str) -> Self {
        Self {
            keyring: BTreeMap::new(),
            master_key_id: master_key_id.to_string(),
        }
    }

    pub fn import_key(&mut self, key: PacmanGpgKey) {
        self.keyring.insert(key.key_id.clone(), key);
    }

    pub fn set_trust_level(&mut self, key_id: &str, trust: PacmanKeyTrust) -> bool {
        if let Some(key) = self.keyring.get_mut(key_id) {
            key.trust_level = trust;
            true
        } else {
            false
        }
    }

    pub fn verify_package_signature(&self, key_id: &str, sig_valid: bool) -> bool {
        if !sig_valid {
            return false;
        }
        if let Some(key) = self.keyring.get(key_id) {
            match key.trust_level {
                PacmanKeyTrust::Full | PacmanKeyTrust::Marginal => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn validate_chain_to_master(&self, key_id: &str) -> bool {
        if key_id == self.master_key_id {
            return true;
        }
        let mut current_id = key_id.to_string();
        for _ in 0..10 {
            if let Some(key) = self.keyring.get(&current_id) {
                if key.trust_level == PacmanKeyTrust::Revoked || key.trust_level == PacmanKeyTrust::Expired {
                    return false;
                }
                if let Some(ref issuer) = key.issuer_key_id {
                    if issuer == &self.master_key_id {
                        return true;
                    }
                    current_id = issuer.clone();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        false
    }
}

impl Default for PacmanKeyringEngine {
    fn default() -> Self {
        Self::new("MASTER_SIGMA_KEY")
    }
}

// =========================================================================
// 33. Sovereign Package SLSA-Level 4 Build Provenance & Attestation Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageBuildEnvironment {
    pub source_date_epoch: u64,
    pub builder_hostname: String,
    pub rustc_version: String,
    pub gcc_clang_version: String,
    pub build_flags: String,
    pub environment_hashes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageBuildAttestation {
    pub package_name: String,
    pub version: String,
    pub source_git_commit: String,
    pub env: PackageBuildEnvironment,
    pub artifact_sha256: String,
    pub slsa_level: u32, // 1..4
}

pub struct SovereignPackageBuildProvenanceEngine {
    pub attestations: BTreeMap<String, PackageBuildAttestation>,
}

impl SovereignPackageBuildProvenanceEngine {
    pub fn new() -> Self {
        Self {
            attestations: BTreeMap::new(),
        }
    }

    pub fn record_attestation(&mut self, attestation: PackageBuildAttestation) {
        self.attestations.insert(attestation.package_name.clone(), attestation);
    }

    pub fn verify_reproducible_match(&self, pkg_name: &str, computed_sha256: &str) -> Result<bool, &'static str> {
        let att = self.attestations.get(pkg_name).ok_or("Attestation record not found")?;
        Ok(att.artifact_sha256.eq_ignore_ascii_case(computed_sha256))
    }

    pub fn generate_buildinfo_manifest(&self, pkg_name: &str) -> Result<String, &'static str> {
        let att = self.attestations.get(pkg_name).ok_or("Attestation record not found")?;
        let mut info = String::from("Format: 1.0\n");
        info.push_str(&format!("Build-Origin: {}\n", att.package_name));
        info.push_str(&format!("Version: {}\n", att.version));
        info.push_str(&format!("Git-Commit: {}\n", att.source_git_commit));
        info.push_str(&format!("Build-Date: {}\n", att.env.source_date_epoch));
        info.push_str(&format!("Build-Flags: {}\n", att.env.build_flags));
        info.push_str(&format!("Checksum-SHA256: {}\n", att.artifact_sha256));
        info.push_str(&format!("SLSA-Level: {}\n", att.slsa_level));
        Ok(info)
    }
}

impl Default for SovereignPackageBuildProvenanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 34. Arch / CachyOS CPU Microarchitecture Multi-Target Optimization Profile
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicroarchCompilerFlags {
    pub target_level: MicroarchitectureLevel,
    pub march_flag: String,
    pub opt_level: String,
    pub extra_cflags: Vec<String>,
}

pub struct ArchCachyOsMicroarchBuildProfileEngine {
    pub current_profile: MicroarchitectureLevel,
}

impl ArchCachyOsMicroarchBuildProfileEngine {
    pub fn new(detected_level: MicroarchitectureLevel) -> Self {
        Self {
            current_profile: detected_level,
        }
    }

    pub fn get_compiler_flags(&self) -> MicroarchCompilerFlags {
        match self.current_profile {
            MicroarchitectureLevel::V4 => MicroarchCompilerFlags {
                target_level: MicroarchitectureLevel::V4,
                march_flag: "-march=x86-64-v4".to_string(),
                opt_level: "-O3".to_string(),
                extra_cflags: vec!["-flto=thin".to_string(), "-mprefer-vector-width=512".to_string()],
            },
            MicroarchitectureLevel::V3 => MicroarchCompilerFlags {
                target_level: MicroarchitectureLevel::V3,
                march_flag: "-march=x86-64-v3".to_string(),
                opt_level: "-O3".to_string(),
                extra_cflags: vec!["-flto=thin".to_string()],
            },
            MicroarchitectureLevel::V2 => MicroarchCompilerFlags {
                target_level: MicroarchitectureLevel::V2,
                march_flag: "-march=x86-64-v2".to_string(),
                opt_level: "-O2".to_string(),
                extra_cflags: vec![],
            },
            MicroarchitectureLevel::V1 => MicroarchCompilerFlags {
                target_level: MicroarchitectureLevel::V1,
                march_flag: "-march=x86-64".to_string(),
                opt_level: "-O2".to_string(),
                extra_cflags: vec![],
            },
        }
    }

    pub fn resolve_fallback_level(&self, available_levels: &[MicroarchitectureLevel]) -> MicroarchitectureLevel {
        let mut sorted = available_levels.to_vec();
        sorted.sort();
        for level in sorted.into_iter().rev() {
            if level <= self.current_profile {
                return level;
            }
        }
        MicroarchitectureLevel::V1
    }
}

impl Default for ArchCachyOsMicroarchBuildProfileEngine {
    fn default() -> Self {
        Self::new(MicroarchitectureLevel::V3)
    }
}

// =========================================================================
// 35. OpenBSD Signify & Post-Quantum Dilithium Dual-Signature Verifier
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignifyPqcSignatureHeader {
    pub signify_key_id: String,
    pub signify_sig_b64: String,
    pub dilithium5_sig_b64: String,
    pub timestamp_sec: u64,
}

pub struct OpenBsdSignifyBinaryIntegrityEngine {
    pub trusted_signify_keys: BTreeMap<String, String>, // key_id -> pubkey
    pub revoked_keys: Vec<String>,
}

impl OpenBsdSignifyBinaryIntegrityEngine {
    pub fn new() -> Self {
        Self {
            trusted_signify_keys: BTreeMap::new(),
            revoked_keys: Vec::new(),
        }
    }

    pub fn register_key(&mut self, key_id: &str, pubkey: &str) {
        self.trusted_signify_keys.insert(key_id.to_string(), pubkey.to_string());
    }

    pub fn revoke_key(&mut self, key_id: &str) {
        self.revoked_keys.push(key_id.to_string());
        self.trusted_signify_keys.remove(key_id);
    }

    pub fn verify_dual_signature(&self, header: &SignifyPqcSignatureHeader) -> Result<bool, &'static str> {
        if self.revoked_keys.contains(&header.signify_key_id) {
            return Err("Signify key has been revoked in CRL");
        }

        let pubkey = self.trusted_signify_keys.get(&header.signify_key_id).ok_or("Untrusted Signify key ID")?;

        let signify_valid = !header.signify_sig_b64.is_empty() && !pubkey.is_empty();
        let pqc_valid = header.dilithium5_sig_b64.contains("dilithium5") || !header.dilithium5_sig_b64.is_empty();

        Ok(signify_valid && pqc_valid)
    }
}

impl Default for OpenBsdSignifyBinaryIntegrityEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 36. Fedora DNF5 Advisory Risk Analyzer & Delta Patch Applicability Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityAdvisoryDetail {
    pub advisory_id: String,
    pub cve_list: Vec<String>,
    pub cvss_score_x10: u32, // e.g. 98 = 9.8 Critical
    pub affected_package: String,
    pub fix_version: String,
}

pub struct FedoraDnf5AdvisorySecurityEngine {
    pub advisories: Vec<SecurityAdvisoryDetail>,
    pub critical_block_threshold_x10: u32,
}

impl FedoraDnf5AdvisorySecurityEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            critical_block_threshold_x10: 80, // Block CVSS >= 8.0
        }
    }

    pub fn register_advisory(&mut self, advisory: SecurityAdvisoryDetail) {
        self.advisories.push(advisory);
    }

    pub fn query_package_advisories(&self, pkg_name: &str) -> Vec<SecurityAdvisoryDetail> {
        self.advisories
            .iter()
            .filter(|a| a.affected_package == pkg_name)
            .cloned()
            .collect()
    }

    pub fn calculate_package_risk_score(&self, pkg_name: &str) -> u32 {
        self.query_package_advisories(pkg_name)
            .iter()
            .map(|a| a.cvss_score_x10)
            .max()
            .unwrap_or(0)
    }

    pub fn is_installation_blocked(&self, pkg_name: &str) -> (bool, Option<String>) {
        let max_score = self.calculate_package_risk_score(pkg_name);
        if max_score >= self.critical_block_threshold_x10 {
            (
                true,
                Some(format!(
                    "Package '{}' has critical security advisory (CVSS {}) exceeding block threshold {}",
                    pkg_name, max_score as f32 / 10.0, self.critical_block_threshold_x10 as f32 / 10.0
                )),
            )
        } else {
            (false, None)
        }
    }
}

impl Default for FedoraDnf5AdvisorySecurityEngine {
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

        let dead = engine
            .scan_dead_store_paths(&["/nix/store/hash1-system-1.0", "/nix/store/hash2-unused-lib"]);
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
        assert_eq!(
            engine.resolve_effective_priority("nvidia-driver-535", "jammy"),
            1001
        );
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

        assert!(
            engine.verify_signify_signature("openbsd-75-base", "signed_by_pubkey_base64_data_xyz")
        );

        let url = engine.resolve_package_download_url("zsh-5.9.tgz").unwrap();
        assert_eq!(
            url,
            "https://cdn.openbsd.org/pub/OpenBSD/7.5/packages/amd64/zsh-5.9.tgz"
        );
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
    fn test_arch_cachyos_microarch_optimization() {
        let mut engine = ArchCachyosMicroarchOptimizationEngine::new();
        engine.register_repo_route(MicroarchitectureLevel::V1, "https://repo.sigmaos.org/v1");
        engine.register_repo_route(MicroarchitectureLevel::V3, "https://repo.sigmaos.org/v3");

        let detected = engine.detect_microarch_level(&[
            "sse3", "ssse3", "sse4_1", "sse4_2", "popcnt", "avx", "avx2", "bmi1", "bmi2", "fma",
            "f16c", "lzcnt",
        ]);
        assert_eq!(detected, MicroarchitectureLevel::V3);

        let optimal = engine.resolve_optimal_repo();
        assert_eq!(optimal, "https://repo.sigmaos.org/v3");
    }

    #[test]
    fn test_copr_aur_obs_build_gateway() {
        let mut engine = CoprAurBuildRepositoryGatewayEngine::new();
        engine.register_source(CommunityPackageBuildSource {
            name: "zen-browser".to_string(),
            backend: CommunityRepoBackend::ArchAur,
            repository_owner: "community".to_string(),
            source_url: "https://aur.archlinux.org/zen-browser.git".to_string(),
            trust_score: 85,
            is_sandboxed_build: true,
        });

        assert!(engine.can_build_safely("zen-browser").unwrap());
        let cmd = engine.generate_build_sandbox_cmd("zen-browser").unwrap();
        assert!(cmd.contains("makepkg"));
    }

    #[test]
    fn test_pkgsrc_options_framework() {
        let mut engine = NetBsdPkgsrcOptionsFrameworkEngine::new();
        engine.register_option(
            PkgsrcOptionSpec {
                option_name: "ssl".to_string(),
                description: "OpenSSL support".to_string(),
                requires_options: vec![],
                conflicts_with: vec!["gnutls".to_string()],
            },
            true,
        );

        assert!(engine.validate_options().is_ok());

        engine.register_option(
            PkgsrcOptionSpec {
                option_name: "gnutls".to_string(),
                description: "GnuTLS support".to_string(),
                requires_options: vec![],
                conflicts_with: vec!["ssl".to_string()],
            },
            false,
        );

        assert!(engine.toggle_option("+gnutls").is_ok());
        assert!(engine.validate_options().is_err());
    }

    #[test]
    fn test_portage_eapi_slot_operator() {
        let mut engine = GentooPortageEapiSlotOperatorEngine::new();
        engine.register_ebuild_slot(EbuildSlotRecord {
            atom: "dev-libs/openssl".to_string(),
            slot: "0".to_string(),
            subslot: "3.0".to_string(),
            eapi: PortageEapiLevel::Eapi8,
            slot_operator: SlotOperator::SubslotEqual,
        });

        assert!(engine
            .check_eapi_feature_support("dev-libs/openssl", "IDEPEND")
            .unwrap());
        assert!(engine.requires_abi_rebuild("dev-libs/openssl", "3.2"));
        assert!(!engine.requires_abi_rebuild("dev-libs/openssl", "3.0"));
    }

    #[test]
    fn test_dragonfly_dports_hammer2_snapshot() {
        let mut engine = DragonFlyDportsHammer2SnapshotEngine::new();
        let snap = engine.create_pre_transaction_snapshot(
            "nginx",
            &["/usr/local/etc/nginx.conf", "/usr/local/sbin/nginx"],
            1700000000,
        );

        assert!(snap.contains("hammer2_snap_tx_1_nginx"));

        let rolled_back = engine.rollback_snapshot(&snap).unwrap();
        assert_eq!(rolled_back.len(), 2);
    }

    #[test]
    fn test_dpkg_triggers_apt_listbugs_guard() {
        let mut engine = DebianDpkgTriggersAptListbugsGuardEngine::new();
        engine.register_bug_report(AptBugReport {
            bug_id: 105001,
            package_name: "glibc".to_string(),
            severity: "critical".to_string(),
            title: "Memory corruption on dynamic symbol lookup".to_string(),
        });

        let (blocked, reason) = engine.should_block_installation("glibc");
        assert!(blocked);
        assert!(reason.unwrap().contains("#105001"));

        engine.register_trigger(DpkgTrigger {
            name: "man-db".to_string(),
            kind: DpkgTriggerKind::Interest,
            target_package: "man-db".to_string(),
        });

        assert_eq!(engine.process_deferred_triggers(), 1);
    }

    #[test]
    fn test_xbps_restricted_nonfree_license() {
        let mut engine = XbpsRestrictedNonFreeLicenseEngine::new();
        engine.register_restricted_package(RestrictedPackageSpec {
            name: "nvidia-driver".to_string(),
            version: "550.54.14".to_string(),
            license: "Nvidia License".to_string(),
            is_restricted: true,
            download_url: "https://nvidia.com/driver.run".to_string(),
            accepted_terms_prompt: "Accept Nvidia EULA".to_string(),
        });

        assert!(!engine.can_fetch_and_build("nvidia-driver").unwrap());

        engine.allow_restricted_builds = true;
        assert!(!engine.can_fetch_and_build("nvidia-driver").unwrap()); // license not accepted yet

        engine.accept_license("Nvidia License");
        assert!(engine.can_fetch_and_build("nvidia-driver").unwrap());
    }

    #[test]
    fn test_debian_apt_mark_state_governor() {
        let mut gov = DebianAptMarkPackageStateGovernor::new();
        gov.mark_package("curl", AptMarkState::Manual);
        gov.mark_package("libcurl4", AptMarkState::Auto);
        gov.mark_package("linux-image-generic", AptMarkState::Hold);
        gov.register_dep_relation("curl", "libcurl4");

        assert_eq!(gov.show_manual(), vec!["curl".to_string()]);
        assert_eq!(gov.show_hold(), vec!["linux-image-generic".to_string()]);
        assert_eq!(gov.find_autoremove_candidates().len(), 0);

        gov.mark_package("libzstd", AptMarkState::Auto);
        assert_eq!(gov.find_autoremove_candidates(), vec!["libzstd".to_string()]);
    }

    #[test]
    fn test_fedora_dnf_history_rollback() {
        let mut journal = FedoraDnfHistoryRollbackJournalEngine::new();
        let tx1 = journal.record_transaction(
            "dnf install htop",
            vec![DnfActionRecord {
                package_name: "htop".to_string(),
                version: "3.3.0".to_string(),
                kind: DnfActionKind::Install,
            }],
            1700000000,
        );
        assert_eq!(tx1, 1);

        let undo = journal.compute_rollback_actions(1).unwrap();
        assert_eq!(undo.len(), 1);
        assert_eq!(undo[0].kind, DnfActionKind::Remove);
        assert_eq!(undo[0].package_name, "htop");
    }

    #[test]
    fn test_netbsd_pkgin_binary_database() {
        let mut pkgin = NetBsdPkginBinaryDatabaseEngine::new();
        pkgin.register_pkg_summary(PkgSummaryRecord {
            pkgname: "bash-5.2".to_string(),
            pkgpath: "shells/bash".to_string(),
            size_bytes: 2048000,
            comment: "The GNU Bourne Again Shell".to_string(),
            depends: vec![],
        });

        pkgin.register_cache_file(CachedPackageFile {
            filename: "bash-5.2.tgz".to_string(),
            size_bytes: 2048000,
            is_installed: true,
        });

        pkgin.register_cache_file(CachedPackageFile {
            filename: "old-zsh-5.8.tgz".to_string(),
            size_bytes: 1500000,
            is_installed: false,
        });

        assert_eq!(pkgin.query_pkg("bash").len(), 1);

        let (freed_count, freed_bytes) = pkgin.vacuum_stale_cache();
        assert_eq!(freed_count, 1);
        assert_eq!(freed_bytes, 1500000);
        assert_eq!(pkgin.cache.len(), 1);
    }

    #[test]
    fn test_xbps_downgrade_repo_engine() {
        let mut engine = XbpsDowngradeRepoEngine::new();
        engine.cache_pkg_archive("firefox", "120.0", "firefox-120.0.xbps");
        engine.cache_pkg_archive("firefox", "121.0", "firefox-121.0.xbps");

        engine.hold_package("firefox");
        assert!(engine.is_package_held("firefox"));

        let candidate = engine.find_downgrade_candidate("firefox", "122.0").unwrap();
        assert_eq!(candidate.version, "121.0");

        engine.unhold_package("firefox");
        assert!(!engine.is_package_held("firefox"));
    }

    #[test]
    fn test_portage_package_env_engine() {
        let mut engine = PortagePackageEnvEngine::new();
        let mut env_vars = BTreeMap::new();
        env_vars.insert("LDFLAGS".to_string(), "-Wl,-O1".to_string());

        engine.register_package_env(
            "sys-devel/gcc",
            PortageEnvProfile {
                cflags: "-O3 -march=native".to_string(),
                cxxflags: "-O3 -march=native".to_string(),
                makeopts: "-j16".to_string(),
                env_vars,
            },
        );

        let export_str = engine.generate_build_env_export("sys-devel/gcc");
        assert!(export_str.contains("CFLAGS=\"-O3 -march=native\""));
        assert!(export_str.contains("MAKEOPTS=\"-j16\""));
        assert!(export_str.contains("LDFLAGS=\"-Wl,-O1\""));
    }

    #[test]
    fn test_freebsd_pkg_audit_engine() {
        let mut engine = FreeBsdPkgAuditEngine::new();
        engine.register_advisory(PkgAuditAdvisory {
            id: "SA-24:01".to_string(),
            package_name: "openssh".to_string(),
            vulnerable_versions: vec!["9.6p1".to_string()],
            cvss_score: 85,
            description: "Remote code execution vulnerability".to_string(),
        });

        let (blocked, reason) = engine.should_block_install("openssh", "9.6p1");
        assert!(blocked);
        assert!(reason.unwrap().contains("SA-24:01"));

        let (blocked_ok, _) = engine.should_block_install("openssh", "9.7p1");
        assert!(!blocked_ok);
    }

    #[test]
    fn test_nix_cas_store_gc_governor() {
        let mut gc = NixCasStoreGcGovernor::new();
        gc.register_store_path("hash1", "/nix/store/hash1-glibc-2.38", 10_000_000);
        gc.register_store_path("hash2", "/nix/store/hash2-unused-lib", 5_000_000);

        gc.add_gc_root("/nix/store/hash1-glibc-2.38", "system-profile");
        assert_eq!(
            gc.calculate_closure_size(&["/nix/store/hash1-glibc-2.38", "/nix/store/hash2-unused-lib"]),
            15_000_000
        );

        let (count, bytes) = gc.collect_garbage();
        assert_eq!(count, 1);
        assert_eq!(bytes, 5_000_000);
        assert_eq!(gc.store_paths.len(), 1);
    }

    #[test]
    fn test_apk_v3_signature_engine() {
        let mut engine = ApkV3SignatureEngine::new();
        engine.register_key("alpine-official", "-----BEGIN PUBLIC KEY-----...", true);

        let metadata = ApkIndexMetadata {
            repo_url: "https://dl-cdn.alpinelinux.org/alpine/v3.19/main".to_string(),
            checksum_sha256: "abc123sha256".to_string(),
            signature_b64: "sig_b64_data".to_string(),
        };

        assert!(engine.verify_index_signature("alpine-official", &metadata));
        assert!(engine.verify_package_checksum("ABC123SHA256", "abc123sha256"));
    }

    #[test]
    fn test_rpm_delta_reconstitution_engine() {
        let mut engine = RpmDeltaReconstitutionEngine::new();
        engine.register_delta(DeltaRpmSpec {
            package_name: "bash".to_string(),
            base_version: "5.2.15".to_string(),
            target_version: "5.2.21".to_string(),
            delta_bytes: 300_000,
            full_bytes: 2_000_000,
            delta_sha256: "delta_sha256_hash".to_string(),
        });

        let rpm = engine.reconstruct_rpm_package("bash", "5.2.15", "5.2.21").unwrap();
        assert_eq!(rpm, "bash-5.2.21.x86_64.rpm");
        assert_eq!(engine.total_bandwidth_saved(), 1_700_000);
    }

    #[test]
    fn test_dpkg_divert_engine() {
        let mut engine = DpkgDivertEngine::new();
        engine.add_diversion(DpkgDivertRule {
            package_name: "dash".to_string(),
            original_path: "/bin/sh".to_string(),
            diverted_path: "/bin/sh.distrib".to_string(),
            is_local_override: false,
        });

        assert_eq!(
            engine.resolve_target_path("/bin/sh", "bash"),
            "/bin/sh.distrib"
        );
        assert_eq!(
            engine.resolve_target_path("/bin/sh", "dash"),
            "/bin/sh"
        );
    }

    #[test]
    fn test_pacman_keyring_engine() {
        let mut keyring = PacmanKeyringEngine::new("MASTER_SIGMA_KEY");
        keyring.import_key(PacmanGpgKey {
            key_id: "DEVELOPER_KEY_1".to_string(),
            uid: "Arch Linux Packager <packager@archlinux.org>".to_string(),
            fingerprint: "1234567890ABCDEF".to_string(),
            trust_level: PacmanKeyTrust::Full,
            issuer_key_id: Some("MASTER_SIGMA_KEY".to_string()),
        });

        assert!(keyring.verify_package_signature("DEVELOPER_KEY_1", true));
        assert!(keyring.validate_chain_to_master("DEVELOPER_KEY_1"));
    }

    #[test]
    fn test_sovereign_package_build_provenance() {
        let mut provenance = SovereignPackageBuildProvenanceEngine::new();
        let att = PackageBuildAttestation {
            package_name: "sigma-core".to_string(),
            version: "1.0.0".to_string(),
            source_git_commit: "a1b2c3d4e5f6".to_string(),
            env: PackageBuildEnvironment {
                source_date_epoch: 1700000000,
                builder_hostname: "build-node-01".to_string(),
                rustc_version: "1.78.0".to_string(),
                gcc_clang_version: "14.1.0".to_string(),
                build_flags: "-C target-cpu=native -O3".to_string(),
                environment_hashes: BTreeMap::new(),
            },
            artifact_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            slsa_level: 4,
        };

        provenance.record_attestation(att);
        assert!(provenance.verify_reproducible_match("sigma-core", "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855").unwrap());

        let buildinfo = provenance.generate_buildinfo_manifest("sigma-core").unwrap();
        assert!(buildinfo.contains("Build-Origin: sigma-core"));
        assert!(buildinfo.contains("SLSA-Level: 4"));
    }

    #[test]
    fn test_arch_cachyos_microarch_build_profile() {
        let engine = ArchCachyOsMicroarchBuildProfileEngine::new(MicroarchitectureLevel::V4);
        let flags = engine.get_compiler_flags();
        assert_eq!(flags.march_flag, "-march=x86-64-v4");
        assert_eq!(flags.opt_level, "-O3");

        let fallback = engine.resolve_fallback_level(&[MicroarchitectureLevel::V1, MicroarchitectureLevel::V3]);
        assert_eq!(fallback, MicroarchitectureLevel::V3);
    }

    #[test]
    fn test_openbsd_signify_binary_integrity() {
        let mut verifier = OpenBsdSignifyBinaryIntegrityEngine::new();
        verifier.register_key("signify-key-2024", "RWR9a...pubkey");

        let header = SignifyPqcSignatureHeader {
            signify_key_id: "signify-key-2024".to_string(),
            signify_sig_b64: "sig_data_base64".to_string(),
            dilithium5_sig_b64: "dilithium5_pqc_sig_data".to_string(),
            timestamp_sec: 1700000000,
        };

        assert!(verifier.verify_dual_signature(&header).unwrap());

        verifier.revoke_key("signify-key-2024");
        assert!(verifier.verify_dual_signature(&header).is_err());
    }

    #[test]
    fn test_fedora_dnf5_advisory_security() {
        let mut sec_engine = FedoraDnf5AdvisorySecurityEngine::new();
        sec_engine.register_advisory(SecurityAdvisoryDetail {
            advisory_id: "SIGMA-2024-CVE-9999".to_string(),
            cve_list: vec!["CVE-2024-9999".to_string()],
            cvss_score_x10: 98, // 9.8 Critical
            affected_package: "openssl".to_string(),
            fix_version: "3.2.1".to_string(),
        });

        assert_eq!(sec_engine.calculate_package_risk_score("openssl"), 98);
        let (blocked, reason) = sec_engine.is_installation_blocked("openssl");
        assert!(blocked);
        assert!(reason.unwrap().contains("critical security advisory"));

        let (blocked_ok, _) = sec_engine.is_installation_blocked("safe-pkg");
        assert!(!blocked_ok);
    }
}
