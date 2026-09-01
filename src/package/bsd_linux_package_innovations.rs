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
}
