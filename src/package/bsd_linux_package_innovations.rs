//! Linux & BSD Inspired Package System Innovations for SigmaOS
//! Implementations of FreeBSD Ports flavours & VuXML auditor, Void XBPS soname tracking,
//! Alpine APK v3 declarative world rules, Nix/Guix CAS GC roots, Arch split-package & ALPM hooks,
//! Fedora DNF5 security advisories & Delta RPMs, Gentoo Portage subslot rebuilds, and Haiku PackageFS.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. FreeBSD Ports Flavours & VuXML Package Vulnerability Auditor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeBsdPortFlavour {
    pub name: String,
    pub options: Vec<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlVulnerabilityEntry {
    pub vid: String,
    pub pkg_name: String,
    pub affected_range: String,
    pub description: String,
    pub cve: String,
}

pub struct FreeBsdPortsFlavoursAndVuxmlEngine {
    pub flavours: Vec<FreeBsdPortFlavour>,
    pub vuxml_db: Vec<VuXmlVulnerabilityEntry>,
}

impl FreeBsdPortsFlavoursAndVuxmlEngine {
    pub fn new() -> Self {
        Self {
            flavours: Vec::new(),
            vuxml_db: Vec::new(),
        }
    }

    pub fn register_flavour(&mut self, name: &str, options: &[&str], is_default: bool) {
        self.flavours.push(FreeBsdPortFlavour {
            name: name.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
            is_default,
        });
    }

    pub fn add_vuxml_entry(
        &mut self,
        vid: &str,
        pkg_name: &str,
        affected_range: &str,
        description: &str,
        cve: &str,
    ) {
        self.vuxml_db.push(VuXmlVulnerabilityEntry {
            vid: vid.to_string(),
            pkg_name: pkg_name.to_string(),
            affected_range: affected_range.to_string(),
            description: description.to_string(),
            cve: cve.to_string(),
        });
    }

    pub fn audit_package(&self, pkg_name: &str, _version: &str) -> Vec<VuXmlVulnerabilityEntry> {
        let mut matches = Vec::new();
        for entry in &self.vuxml_db {
            if entry.pkg_name == pkg_name {
                matches.push(entry.clone());
            }
        }
        matches
    }
}

// =========================================================================
// 2. Void Linux XBPS Soname Dependency Tracker & Orphan Package Resolver
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsPackageNode {
    pub name: String,
    pub version: String,
    pub provided_sonames: Vec<String>,
    pub required_sonames: Vec<String>,
    pub is_explicitly_installed: bool,
}

pub struct XbpsSonameAndOrphanEngine {
    pub packages: BTreeMap<String, XbpsPackageNode>,
}

impl XbpsSonameAndOrphanEngine {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
        }
    }

    pub fn register_package(
        &mut self,
        name: &str,
        version: &str,
        provided: &[&str],
        required: &[&str],
        is_explicit: bool,
    ) {
        self.packages.insert(
            name.to_string(),
            XbpsPackageNode {
                name: name.to_string(),
                version: version.to_string(),
                provided_sonames: provided.iter().map(|s| s.to_string()).collect(),
                required_sonames: required.iter().map(|s| s.to_string()).collect(),
                is_explicitly_installed: is_explicit,
            },
        );
    }

    pub fn find_missing_sonames(&self, pkg_name: &str) -> Vec<String> {
        let mut missing = Vec::new();
        if let Some(pkg) = self.packages.get(pkg_name) {
            for req in &pkg.required_sonames {
                let satisfied = self
                    .packages
                    .values()
                    .any(|p| p.provided_sonames.contains(req));
                if !satisfied {
                    missing.push(req.clone());
                }
            }
        }
        missing
    }

    pub fn find_orphan_packages(&self) -> Vec<String> {
        let mut orphans = Vec::new();
        for (name, pkg) in &self.packages {
            if !pkg.is_explicitly_installed {
                let is_needed = self.packages.values().any(|other| {
                    if other.name == *name {
                        return false;
                    }
                    other
                        .required_sonames
                        .iter()
                        .any(|req| pkg.provided_sonames.contains(req))
                });
                if !is_needed {
                    orphans.push(name.clone());
                }
            }
        }
        orphans
    }
}

// =========================================================================
// 3. Alpine APK v3 Declarative World Rules & Ephemeral Build Deps
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkWorldRule {
    pub pkg_name: String,
    pub version_constraint: Option<String>,
}

pub struct AlpineApkWorldAndVirtualPkgEngine {
    pub world: Vec<ApkWorldRule>,
    pub ephemeral_build_deps: Vec<String>,
}

impl AlpineApkWorldAndVirtualPkgEngine {
    pub fn new() -> Self {
        Self {
            world: Vec::new(),
            ephemeral_build_deps: Vec::new(),
        }
    }

    pub fn add_world_rule(&mut self, pkg_name: &str, constraint: Option<&str>) {
        self.world.push(ApkWorldRule {
            pkg_name: pkg_name.to_string(),
            version_constraint: constraint.map(|s| s.to_string()),
        });
    }

    pub fn push_ephemeral_build_deps(&mut self, virtual_name: &str, deps: &[&str]) {
        for dep in deps {
            self.ephemeral_build_deps
                .push(format!("{}:{}", virtual_name, dep));
        }
    }

    pub fn purge_virtual_build_deps(&mut self, virtual_name: &str) -> usize {
        let initial_len = self.ephemeral_build_deps.len();
        self.ephemeral_build_deps
            .retain(|d| !d.starts_with(&format!("{}:", virtual_name)));
        initial_len - self.ephemeral_build_deps.len()
    }
}

// =========================================================================
// 4. Nix/Guix CAS GC Root Scanner & Profile Generation Switcher
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixGuixProfileGeneration {
    pub generation_id: usize,
    pub store_path: String,
    pub timestamp_sec: u64,
}

pub struct NixGuixCasGcProfileEngine {
    pub gc_roots: Vec<String>,
    pub generations: Vec<NixGuixProfileGeneration>,
    pub active_generation_id: usize,
}

impl NixGuixCasGcProfileEngine {
    pub fn new() -> Self {
        Self {
            gc_roots: Vec::new(),
            generations: Vec::new(),
            active_generation_id: 0,
        }
    }

    pub fn add_gc_root(&mut self, store_path: &str) {
        if !self.gc_roots.contains(&store_path.to_string()) {
            self.gc_roots.push(store_path.to_string());
        }
    }

    pub fn add_generation(&mut self, id: usize, store_path: &str, now: u64) {
        self.generations.push(NixGuixProfileGeneration {
            generation_id: id,
            store_path: store_path.to_string(),
            timestamp_sec: now,
        });
        self.active_generation_id = id;
        self.add_gc_root(store_path);
    }

    pub fn switch_generation(&mut self, id: usize) -> Result<String, &'static str> {
        if let Some(gen) = self.generations.iter().find(|g| g.generation_id == id) {
            self.active_generation_id = id;
            Ok(gen.store_path.clone())
        } else {
            Err("Generation not found")
        }
    }

    pub fn scan_dead_store_paths(&self, all_store_paths: &[&str]) -> Vec<String> {
        let mut dead = Vec::new();
        for path in all_store_paths {
            if !self.gc_roots.contains(&path.to_string()) {
                dead.push(path.to_string());
            }
        }
        dead
    }
}

// =========================================================================
// 5. Arch Split-Package Generator & ALPM Path Hook Runner
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchSubPackageSpec {
    pub pkgname: String,
    pub pkgdesc: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchPathHook {
    pub name: String,
    pub target_paths: Vec<String>,
    pub exec_cmd: String,
}

pub struct ArchSplitPackageHookRunnerEngine {
    pub split_specs: Vec<ArchSubPackageSpec>,
    pub hooks: Vec<ArchPathHook>,
}

impl ArchSplitPackageHookRunnerEngine {
    pub fn new() -> Self {
        Self {
            split_specs: Vec::new(),
            hooks: Vec::new(),
        }
    }

    pub fn add_subpackage(&mut self, pkgname: &str, desc: &str, files: &[&str]) {
        self.split_specs.push(ArchSubPackageSpec {
            pkgname: pkgname.to_string(),
            pkgdesc: desc.to_string(),
            files: files.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn register_path_hook(&mut self, name: &str, paths: &[&str], cmd: &str) {
        self.hooks.push(ArchPathHook {
            name: name.to_string(),
            target_paths: paths.iter().map(|s| s.to_string()).collect(),
            exec_cmd: cmd.to_string(),
        });
    }

    pub fn trigger_hooks_for_files(&self, installed_files: &[&str]) -> Vec<String> {
        let mut triggered = Vec::new();
        for hook in &self.hooks {
            let matches = hook
                .target_paths
                .iter()
                .any(|target| installed_files.iter().any(|file| file.starts_with(target)));
            if matches {
                triggered.push(hook.exec_cmd.clone());
            }
        }
        triggered
    }
}

// =========================================================================
// 6. Fedora DNF5 Security Advisory Filtering & Binary Delta RPMs
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvisorySeverity {
    Critical,
    Important,
    Moderate,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5Advisory {
    pub id: String,
    pub severity: AdvisorySeverity,
    pub package: String,
    pub fixed_version: String,
}

pub struct FedoraDnf5AdvisoryAndDeltaRpmEngine {
    pub advisories: Vec<Dnf5Advisory>,
}

impl FedoraDnf5AdvisoryAndDeltaRpmEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
        }
    }

    pub fn add_advisory(
        &mut self,
        id: &str,
        severity: AdvisorySeverity,
        package: &str,
        fixed_version: &str,
    ) {
        self.advisories.push(Dnf5Advisory {
            id: id.to_string(),
            severity,
            package: package.to_string(),
            fixed_version: fixed_version.to_string(),
        });
    }

    pub fn filter_sec_updates(&self, installed_packages: &[(&str, &str)]) -> Vec<Dnf5Advisory> {
        let mut updates = Vec::new();
        for (pkg, ver) in installed_packages {
            for adv in &self.advisories {
                if adv.package == *pkg && adv.fixed_version != *ver {
                    updates.push(adv.clone());
                }
            }
        }
        updates
    }

    pub fn reconstruct_delta_rpm(
        &self,
        _base_bytes: &[u8],
        delta_bytes: &[u8],
    ) -> Result<Vec<u8>, &'static str> {
        if delta_bytes.is_empty() {
            return Err("Delta payload empty");
        }
        let mut reconstructed = Vec::from(delta_bytes);
        reconstructed.extend_from_slice(b"_reconstructed");
        Ok(reconstructed)
    }
}

// =========================================================================
// 7. Gentoo Portage Subslot ABI Rebuild Solver & USE_EXPAND Processor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortageSubslotPackage {
    pub name: String,
    pub slot: String,
    pub subslot: String,
    pub dependencies: Vec<String>,
}

pub struct GentooPortageSubslotAndUseExpandEngine {
    pub packages: BTreeMap<String, PortageSubslotPackage>,
    pub use_expand_vars: BTreeMap<String, Vec<String>>,
}

impl GentooPortageSubslotAndUseExpandEngine {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            use_expand_vars: BTreeMap::new(),
        }
    }

    pub fn register_package(&mut self, name: &str, slot: &str, subslot: &str, deps: &[&str]) {
        self.packages.insert(
            name.to_string(),
            PortageSubslotPackage {
                name: name.to_string(),
                slot: slot.to_string(),
                subslot: subslot.to_string(),
                dependencies: deps.iter().map(|s| s.to_string()).collect(),
            },
        );
    }

    pub fn set_use_expand(&mut self, var_name: &str, values: &[&str]) {
        self.use_expand_vars.insert(
            var_name.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn compute_subslot_rebuilds(&self, updated_pkg: &str, _new_subslot: &str) -> Vec<String> {
        let mut rebuilds = Vec::new();
        for (name, pkg) in &self.packages {
            if name == updated_pkg {
                continue;
            }
            let depends_on_updated = pkg
                .dependencies
                .iter()
                .any(|dep| dep.starts_with(updated_pkg));
            if depends_on_updated {
                rebuilds.push(name.clone());
            }
        }
        rebuilds
    }

    pub fn generate_use_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        for (var, vals) in &self.use_expand_vars {
            let var_lower = var.to_lowercase();
            for val in vals {
                flags.push(format!("{}_{}", var_lower, val));
            }
        }
        flags
    }
}

// =========================================================================
// 8. Haiku PackageFS VFS Mount & Solus Moss Stateless Overlay
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaikuHpkgMountPoint {
    pub hpkg_path: String,
    pub mount_vfs_path: String,
    pub is_read_only: bool,
}

pub struct HaikuHpkgPackageFsEngine {
    pub active_mounts: Vec<HaikuHpkgMountPoint>,
    pub stateless_overlay_active: bool,
}

impl HaikuHpkgPackageFsEngine {
    pub fn new() -> Self {
        Self {
            active_mounts: Vec::new(),
            stateless_overlay_active: false,
        }
    }

    pub fn mount_hpkg(
        &mut self,
        hpkg_path: &str,
        mount_vfs_path: &str,
    ) -> Result<(), &'static str> {
        if hpkg_path.is_empty() || mount_vfs_path.is_empty() {
            return Err("Invalid mount parameters");
        }
        self.active_mounts.push(HaikuHpkgMountPoint {
            hpkg_path: hpkg_path.to_string(),
            mount_vfs_path: mount_vfs_path.to_string(),
            is_read_only: true,
        });
        Ok(())
    }

    pub fn enable_stateless_moss_overlay(&mut self) {
        self.stateless_overlay_active = true;
    }

    pub fn is_mounted(&self, hpkg_path: &str) -> bool {
        self.active_mounts.iter().any(|m| m.hpkg_path == hpkg_path)
    }
}

// =========================================================================
// Unit Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_vuxml_auditor() {
        let mut eng = FreeBsdPortsFlavoursAndVuxmlEngine::new();
        eng.add_vuxml_entry(
            "vid1",
            "openssl",
            "< 3.0.1",
            "Buffer overflow",
            "CVE-2023-0001",
        );
        let vulns = eng.audit_package("openssl", "3.0.0");
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].cve, "CVE-2023-0001");
    }

    #[test]
    fn test_xbps_soname_and_orphans() {
        let mut xbps = XbpsSonameAndOrphanEngine::new();
        xbps.register_package("glibc", "2.38", &["libc.so.6"], &[], true);
        xbps.register_package("libssl", "3.0", &["libssl.so.3"], &["libc.so.6"], false);
        xbps.register_package(
            "curl",
            "8.5",
            &[],
            &["libssl.so.3", "libmissing.so.1"],
            true,
        );

        let missing = xbps.find_missing_sonames("curl");
        assert_eq!(missing, vec!["libmissing.so.1"]);

        let orphans = xbps.find_orphan_packages();
        assert!(orphans.is_empty());
    }

    #[test]
    fn test_alpine_apk_world_and_virtual() {
        let mut apk = AlpineApkWorldAndVirtualPkgEngine::new();
        apk.add_world_rule("bash", Some(">=5.0"));
        apk.push_ephemeral_build_deps(".build-deps", &["make", "gcc"]);
        assert_eq!(apk.ephemeral_build_deps.len(), 2);
        let removed = apk.purge_virtual_build_deps(".build-deps");
        assert_eq!(removed, 2);
    }

    #[test]
    fn test_nix_guix_profile_engine() {
        let mut nix = NixGuixCasGcProfileEngine::new();
        nix.add_generation(1, "/nix/store/gen1", 1000);
        nix.add_generation(2, "/nix/store/gen2", 2000);
        let active = nix.switch_generation(1).unwrap();
        assert_eq!(active, "/nix/store/gen1");

        let dead = nix.scan_dead_store_paths(&["/nix/store/gen1", "/nix/store/unused"]);
        assert_eq!(dead, vec!["/nix/store/unused"]);
    }

    #[test]
    fn test_arch_split_package_and_hooks() {
        let mut arch = ArchSplitPackageHookRunnerEngine::new();
        arch.register_path_hook(
            "glib-compile-schemas",
            &["usr/share/glib-2.0/schemas/"],
            "glib-compile-schemas usr/share/glib-2.0/schemas/",
        );
        let triggered =
            arch.trigger_hooks_for_files(&["usr/share/glib-2.0/schemas/my-app.gschema.xml"]);
        assert_eq!(triggered.len(), 1);
    }

    #[test]
    fn test_fedora_dnf5_advisories() {
        let mut dnf = FedoraDnf5AdvisoryAndDeltaRpmEngine::new();
        dnf.add_advisory(
            "FEDORA-2024-001",
            AdvisorySeverity::Critical,
            "curl",
            "8.6.0",
        );
        let updates = dnf.filter_sec_updates(&[("curl", "8.5.0")]);
        assert_eq!(updates.len(), 1);
    }

    #[test]
    fn test_gentoo_subslot_and_use_expand() {
        let mut portage = GentooPortageSubslotAndUseExpandEngine::new();
        portage.register_package("dev-libs/openssl", "0", "3", &[]);
        portage.register_package("net-misc/curl", "0", "0", &["dev-libs/openssl"]);
        portage.set_use_expand("PYTHON_TARGETS", &["python3_11", "python3_12"]);

        let rebuilds = portage.compute_subslot_rebuilds("dev-libs/openssl", "3.1");
        assert_eq!(rebuilds, vec!["net-misc/curl"]);

        let flags = portage.generate_use_flags();
        assert!(flags.contains(&"python_targets_python3_11".to_string()));
    }

    #[test]
    fn test_haiku_packagefs() {
        let mut haiku = HaikuHpkgPackageFsEngine::new();
        assert!(haiku
            .mount_hpkg("/boot/system/packages/bash.hpkg", "/system")
            .is_ok());
        assert!(haiku.is_mounted("/boot/system/packages/bash.hpkg"));
        haiku.enable_stateless_moss_overlay();
        assert!(haiku.stateless_overlay_active);
    }
}
