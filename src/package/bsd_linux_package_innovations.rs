// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Package Management Innovations
// Inspired by Linux & BSD distributions: FreeBSD Ports & VuXML, Void XBPS, Alpine APK v3,
// Nix/Guix Functional Store, Arch ALPM, Fedora DNF5, Gentoo Portage, Haiku HPKG & Solus Moss.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// ============================================================================
// 1. FreeBSD Ports Flavours & VuXML Package Vulnerability Audit Engine
// ============================================================================

/// FreeBSD Ports Flavour Specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortFlavourSpec {
    pub origin: String,             // e.g., "devel/py-certifi"
    pub selected_flavour: String,   // e.g., "py311", "py312", "qt6"
    pub available_flavours: Vec<String>,
    pub default_options: Vec<String>,
    pub flavour_options: Vec<String>,
    pub flavour_dependencies: Vec<String>,
}

/// Vulnerability severity level according to FreeBSD VuXML
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnerabilitySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// VuXML Vulnerability Record
#[derive(Debug, Clone)]
pub struct VuXmlVulnerabilityRecord {
    pub vid: String,               // VuXML UUID / ID
    pub affected_package: String,  // e.g., "openssl"
    pub vulnerable_range_min: String,
    pub vulnerable_range_max: String,
    pub severity: VulnerabilitySeverity,
    pub cve_ids: Vec<String>,
    pub description: String,
    pub fixed_version: String,
}

/// FreeBSD Ports Flavours & VuXML Package Audit Engine
pub struct FreeBsdPortsFlavoursAndVuxmlEngine {
    pub ports_catalog: BTreeMap<String, PortFlavourSpec>,
    pub vuxml_db: Vec<VuXmlVulnerabilityRecord>,
}

impl FreeBsdPortsFlavoursAndVuxmlEngine {
    pub fn new() -> Self {
        Self {
            ports_catalog: BTreeMap::new(),
            vuxml_db: Vec::new(),
        }
    }

    pub fn register_port(&mut self, spec: PortFlavourSpec) {
        self.ports_catalog.insert(spec.origin.clone(), spec);
    }

    pub fn register_vulnerability(&mut self, record: VuXmlVulnerabilityRecord) {
        self.vuxml_db.push(record);
    }

    /// Select a specific port flavour and return resolved build dependencies
    pub fn select_flavour(&mut self, origin: &str, flavour: &str) -> Result<Vec<String>, &'static str> {
        let spec = self.ports_catalog.get_mut(origin).ok_or("Port origin not found")?;
        if !spec.available_flavours.contains(&flavour.to_string()) {
            return Err("Requested flavour is not available for this port");
        }
        spec.selected_flavour = flavour.to_string();

        let mut resolved_deps = spec.flavour_dependencies.clone();
        resolved_deps.push(format!("{}-{}", origin, flavour));
        Ok(resolved_deps)
    }

    /// Audit installed packages against the VuXML vulnerability database
    pub fn audit_installed_packages(&self, installed_pkgs: &[(&str, &str)]) -> Vec<VuXmlVulnerabilityRecord> {
        let mut matches = Vec::new();
        for (pkg_name, pkg_version) in installed_pkgs {
            for vuln in &self.vuxml_db {
                if vuln.affected_package == *pkg_name {
                    // Check if pkg_version is within vulnerable range or matches affected package
                    if pkg_version >= &vuln.vulnerable_range_min.as_str()
                        && pkg_version <= &vuln.vulnerable_range_max.as_str()
                    {
                        matches.push(vuln.clone());
                    }
                }
            }
        }
        matches
    }
}

impl Default for FreeBsdPortsFlavoursAndVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Void Linux XBPS `soname` Library Dependency & Orphan Auto-Purging Engine
// ============================================================================

/// Dynamic ELF shared library dependency record (`soname`)
#[derive(Debug, Clone)]
pub struct ElfSonameRecord {
    pub package_name: String,
    pub provided_sonames: Vec<String>,  // e.g., ["libssl.so.3", "libcrypto.so.3"]
    pub required_sonames: Vec<String>,  // e.g., ["libc.so.6", "libm.so.6"]
    pub is_explicitly_installed: bool,
}

/// XBPS `soname` Library & Orphan Package Resolver
pub struct XbpsSonameAndOrphanEngine {
    pub installed_packages: BTreeMap<String, ElfSonameRecord>,
}

impl XbpsSonameAndOrphanEngine {
    pub fn new() -> Self {
        Self {
            installed_packages: BTreeMap::new(),
        }
    }

    pub fn register_installed_package(&mut self, record: ElfSonameRecord) {
        self.installed_packages.insert(record.package_name.clone(), record);
    }

    /// Verify all dynamic library `soname` dependencies across installed packages
    pub fn verify_soname_integrity(&self) -> Result<usize, Vec<String>> {
        let mut all_provided: Vec<String> = Vec::new();
        for record in self.installed_packages.values() {
            for soname in &record.provided_sonames {
                all_provided.push(soname.clone());
            }
        }

        let mut missing_sonames = Vec::new();
        for record in self.installed_packages.values() {
            for req in &record.required_sonames {
                if !all_provided.contains(req) {
                    missing_sonames.push(format!("{}: missing {}", record.package_name, req));
                }
            }
        }

        if missing_sonames.is_empty() {
            Ok(self.installed_packages.len())
        } else {
            Err(missing_sonames)
        }
    }

    /// Recursively find orphan packages (dependencies no longer required by any explicit package)
    pub fn find_orphan_packages(&self) -> Vec<String> {
        let mut required_by_other: BTreeMap<String, bool> = BTreeMap::new();
        for name in self.installed_packages.keys() {
            required_by_other.insert(name.clone(), false);
        }

        // Mark packages that supply required sonames for explicitly installed packages
        for record in self.installed_packages.values() {
            if record.is_explicitly_installed {
                for req in &record.required_sonames {
                    for (provider_name, provider_record) in &self.installed_packages {
                        if provider_record.provided_sonames.contains(req) {
                            required_by_other.insert(provider_name.clone(), true);
                        }
                    }
                }
            }
        }

        let mut orphans = Vec::new();
        for (name, record) in &self.installed_packages {
            if !record.is_explicitly_installed {
                let is_needed = required_by_other.get(name).copied().unwrap_or(false);
                if !is_needed {
                    orphans.push(name.clone());
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

// ============================================================================
// 3. Alpine Linux APK v3 Declarative World & Ephemeral Virtual Package Engine
// ============================================================================

/// APK v3 World File Package Constraint Rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkWorldRule {
    pub package_name: String,
    pub version_constraint: Option<String>, // e.g., ">=3.18", "~2.0"
    pub pinned_repository_tag: Option<String>, // e.g., "@testing"
}

/// Ephemeral Virtual Package Set (`.build-deps`)
#[derive(Debug, Clone)]
pub struct EphemeralVirtualPackageSet {
    pub virtual_name: String, // e.g., ".build-deps-zlib"
    pub installed_members: Vec<String>,
    pub created_timestamp: u64,
}

/// Alpine APK v3 World & Ephemeral Package Engine
pub struct AlpineApkWorldAndVirtualPkgEngine {
    pub world_rules: Vec<ApkWorldRule>,
    pub virtual_sets: BTreeMap<String, EphemeralVirtualPackageSet>,
    pub installed_packages: Vec<String>,
}

impl AlpineApkWorldAndVirtualPkgEngine {
    pub fn new() -> Self {
        Self {
            world_rules: Vec::new(),
            virtual_sets: BTreeMap::new(),
            installed_packages: Vec::new(),
        }
    }

    pub fn add_world_rule(&mut self, rule: ApkWorldRule) {
        if !self.world_rules.iter().any(|r| r.package_name == rule.package_name) {
            self.world_rules.push(rule);
        }
    }

    /// Create an ephemeral virtual package group (`apk add .build-deps ...`)
    pub fn create_virtual_package_set(&mut self, virtual_name: &str, member_pkgs: &[&str], timestamp: u64) -> usize {
        let members: Vec<String> = member_pkgs.iter().map(|s| s.to_string()).collect();
        for m in &members {
            if !self.installed_packages.contains(m) {
                self.installed_packages.push(m.clone());
            }
        }

        let virt_set = EphemeralVirtualPackageSet {
            virtual_name: virtual_name.to_string(),
            installed_members: members,
            created_timestamp: timestamp,
        };
        self.virtual_sets.insert(virtual_name.to_string(), virt_set);
        self.installed_packages.len()
    }

    /// Atomically remove an ephemeral virtual package group (`apk del .build-deps`)
    pub fn purge_virtual_package_set(&mut self, virtual_name: &str) -> Result<Vec<String>, &'static str> {
        let virt_set = self.virtual_sets.remove(virtual_name).ok_or("Virtual package set not found")?;
        for member in &virt_set.installed_members {
            self.installed_packages.retain(|p| p != member);
        }
        Ok(virt_set.installed_members)
    }
}

impl Default for AlpineApkWorldAndVirtualPkgEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Guix & Nix Content-Addressed Store Garbage Collector & Profile Generations
// ============================================================================

/// Profile Generation
#[derive(Debug, Clone)]
pub struct ProfileGeneration {
    pub generation_id: u32,
    pub store_paths: Vec<String>, // e.g., "/nix/store/a1b2...-bash-5.2"
    pub created_at: u64,
}

/// Nix / Guix CAS Store Garbage Collector & Profile Generation Engine
pub struct NixGuixCasGcProfileEngine {
    pub store_objects: BTreeMap<String, Vec<u8>>, // store_path -> binary payload
    pub gc_roots: Vec<String>,                    // e.g., symlinks in /nix/var/nix/gcroots
    pub profile_generations: Vec<ProfileGeneration>,
    pub current_generation_id: u32,
}

impl NixGuixCasGcProfileEngine {
    pub fn new() -> Self {
        Self {
            store_objects: BTreeMap::new(),
            gc_roots: Vec::new(),
            profile_generations: Vec::new(),
            current_generation_id: 0,
        }
    }

    pub fn add_store_object(&mut self, store_path: &str, data: &[u8]) {
        self.store_objects.insert(store_path.to_string(), data.to_vec());
    }

    pub fn add_gc_root(&mut self, root_path: &str) {
        if !self.gc_roots.contains(&root_path.to_string()) {
            self.gc_roots.push(root_path.to_string());
        }
    }

    pub fn create_generation(&mut self, paths: &[&str], timestamp: u64) -> u32 {
        self.current_generation_id += 1;
        let gen_id = self.current_generation_id;
        let gen = ProfileGeneration {
            generation_id: gen_id,
            store_paths: paths.iter().map(|s| s.to_string()).collect(),
            created_at: timestamp,
        };
        self.profile_generations.push(gen);
        gen_id
    }

    /// Perform GC root tracing and prune unreferenced store objects (`nix-store --gc`)
    pub fn collect_garbage(&mut self) -> usize {
        let mut live_paths: Vec<String> = Vec::new();

        // 1. Mark paths referenced by GC roots
        for root in &self.gc_roots {
            live_paths.push(root.clone());
        }

        // 2. Mark paths referenced by active/historical profile generations
        for gen in &self.profile_generations {
            for path in &gen.store_paths {
                if !live_paths.contains(path) {
                    live_paths.push(path.clone());
                }
            }
        }

        // 3. Sweep dead store objects
        let dead_paths: Vec<String> = self
            .store_objects
            .keys()
            .filter(|p| !live_paths.contains(p))
            .cloned()
            .collect();

        let count = dead_paths.len();
        for dead in dead_paths {
            self.store_objects.remove(&dead);
        }
        count
    }
}

impl Default for NixGuixCasGcProfileEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Arch Linux Split-Package & ALPM Path Hook Execution Engine
// ============================================================================

/// PKGBUILD Split Package Specification
#[derive(Debug, Clone)]
pub struct PkgbuildSplitPackageSpec {
    pub base_pkgname: String,
    pub sub_packages: Vec<String>, // e.g., ["libfoo", "libfoo-devel", "libfoo-doc"]
    pub common_depends: Vec<String>,
}

/// ALPM Hook Action Timing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

/// Pacman ALPM Path Trigger Rule
#[derive(Debug, Clone)]
pub struct AlpmPathHookRule {
    pub name: String,
    pub when: HookWhen,
    pub path_trigger: String, // e.g., "usr/share/glib-2.0/schemas/"
    pub exec_command: String,
}

/// Arch Split-Package & Path Hook Runner Engine
pub struct ArchSplitPackageHookRunnerEngine {
    pub split_specs: Vec<PkgbuildSplitPackageSpec>,
    pub hook_rules: Vec<AlpmPathHookRule>,
    pub executed_hook_log: Vec<String>,
}

impl ArchSplitPackageHookRunnerEngine {
    pub fn new() -> Self {
        Self {
            split_specs: Vec::new(),
            hook_rules: Vec::new(),
            executed_hook_log: Vec::new(),
        }
    }

    pub fn register_split_pkg(&mut self, spec: PkgbuildSplitPackageSpec) {
        self.split_specs.push(spec);
    }

    pub fn register_hook(&mut self, hook: AlpmPathHookRule) {
        self.hook_rules.push(hook);
    }

    /// Trigger hooks matching modified file paths
    pub fn trigger_hooks_for_paths(&mut self, when: HookWhen, modified_paths: &[&str]) -> usize {
        let mut count = 0;
        for hook in &self.hook_rules {
            if hook.when == when {
                let matches = modified_paths.iter().any(|p| p.contains(&hook.path_trigger));
                if matches {
                    self.executed_hook_log.push(hook.exec_command.clone());
                    count += 1;
                }
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

// ============================================================================
// 6. Fedora DNF5 Security Advisory Filtering & Delta RPM Patch Engine
// ============================================================================

/// DNF5 Advisory Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dnf5AdvisoryKind {
    Security,
    Bugfix,
    Enhancement,
}

/// DNF5 Security Advisory Notice
#[derive(Debug, Clone)]
pub struct Dnf5AdvisoryNotice {
    pub advisory_id: String, // e.g., "FEDORA-2026-1234"
    pub kind: Dnf5AdvisoryKind,
    pub affected_package: String,
    pub new_version: String,
    pub cve_references: Vec<String>,
}

/// Delta RPM Patch Specification
#[derive(Debug, Clone)]
pub struct DeltaRpmPatchSpec {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub delta_size_bytes: usize,
    pub full_size_bytes: usize,
}

/// Fedora DNF5 Advisory & Delta RPM Engine
pub struct FedoraDnf5AdvisoryAndDeltaRpmEngine {
    pub advisories: Vec<Dnf5AdvisoryNotice>,
    pub available_deltas: Vec<DeltaRpmPatchSpec>,
}

impl FedoraDnf5AdvisoryAndDeltaRpmEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            available_deltas: Vec::new(),
        }
    }

    pub fn register_advisory(&mut self, advisory: Dnf5AdvisoryNotice) {
        self.advisories.push(advisory);
    }

    pub fn register_delta(&mut self, delta: DeltaRpmPatchSpec) {
        self.available_deltas.push(delta);
    }

    /// Filter upgrade packages strictly by security advisories (`dnf upgrade --security`)
    pub fn filter_security_updates(&self) -> Vec<Dnf5AdvisoryNotice> {
        self.advisories
            .iter()
            .filter(|a| a.kind == Dnf5AdvisoryKind::Security)
            .cloned()
            .collect()
    }

    /// Calculate bandwidth savings achieved via Delta RPMs
    pub fn calculate_delta_savings(&self) -> (usize, usize) {
        let total_full: usize = self.available_deltas.iter().map(|d| d.full_size_bytes).sum();
        let total_delta: usize = self.available_deltas.iter().map(|d| d.delta_size_bytes).sum();
        (total_full, total_delta)
    }
}

impl Default for FedoraDnf5AdvisoryAndDeltaRpmEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. Gentoo Portage Subslot ABI Rebuild & USE_EXPAND Engine
// ============================================================================

/// Portage Package Subslot Specification
#[derive(Debug, Clone)]
pub struct PortageSubslotSpec {
    pub package_atom: String, // e.g., "dev-libs/openssl"
    pub slot: String,         // e.g., "0"
    pub subslot: String,      // e.g., "3.0"
    pub reverse_dependents: Vec<String>, // e.g., ["net-misc/curl", "net-analyzer/nmap"]
}

/// Gentoo Portage Subslot & USE_EXPAND Engine
pub struct GentooPortageSubslotAndUseExpandEngine {
    pub installed_subslots: BTreeMap<String, PortageSubslotSpec>,
    pub use_expand_vars: BTreeMap<String, Vec<String>>, // e.g., "CPU_FLAGS_X86" -> ["avx2", "fma"]
}

impl GentooPortageSubslotAndUseExpandEngine {
    pub fn new() -> Self {
        Self {
            installed_subslots: BTreeMap::new(),
            use_expand_vars: BTreeMap::new(),
        }
    }

    pub fn register_subslot(&mut self, spec: PortageSubslotSpec) {
        self.installed_subslots.insert(spec.package_atom.clone(), spec);
    }

    pub fn set_use_expand(&mut self, var_name: &str, values: &[&str]) {
        self.use_expand_vars.insert(
            var_name.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }

    /// Detect reverse dependencies that require rebuilding when a subslot is updated
    pub fn check_subslot_rebuild_trigger(&mut self, pkg_atom: &str, new_subslot: &str) -> Vec<String> {
        if let Some(spec) = self.installed_subslots.get_mut(pkg_atom) {
            if spec.subslot != new_subslot {
                spec.subslot = new_subslot.to_string();
                return spec.reverse_dependents.clone();
            }
        }
        Vec::new()
    }

    /// Expand USE_EXPAND variables into package USE flags
    pub fn expand_use_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        for (var, values) in &self.use_expand_vars {
            let prefix = var.to_lowercase();
            for val in values {
                flags.push(format!("{}_{}", prefix, val));
            }
        }
        flags
    }
}

impl Default for GentooPortageSubslotAndUseExpandEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. Haiku HPKG PackageFS Mount & Solus Moss Stateless Overlay Engine
// ============================================================================

/// Haiku HPKG Package Container Spec
#[derive(Debug, Clone)]
pub struct HpkgContainerSpec {
    pub hpkg_filename: String,
    pub mounted_vfs_path: String, // e.g., "/system/packages/zsh-5.9.hpkg"
    pub contained_files: Vec<String>,
    pub is_mounted: bool,
}

/// Haiku HPKG & Solus Moss Stateless Engine
pub struct HaikuHpkgPackageFsEngine {
    pub package_containers: BTreeMap<String, HpkgContainerSpec>,
    pub stateless_defaults: BTreeMap<String, String>, // /usr/share/defaults/ -> content
}

impl HaikuHpkgPackageFsEngine {
    pub fn new() -> Self {
        Self {
            package_containers: BTreeMap::new(),
            stateless_defaults: BTreeMap::new(),
        }
    }

    pub fn mount_hpkg_package(&mut self, spec: HpkgContainerSpec) -> Result<String, &'static str> {
        let mut pkg = spec;
        pkg.is_mounted = true;
        let path = pkg.mounted_vfs_path.clone();
        self.package_containers.insert(pkg.hpkg_filename.clone(), pkg);
        Ok(path)
    }

    pub fn unmount_hpkg_package(&mut self, filename: &str) -> bool {
        if let Some(pkg) = self.package_containers.get_mut(filename) {
            pkg.is_mounted = false;
            true
        } else {
            false
        }
    }

    pub fn set_stateless_default_config(&mut self, rel_path: &str, content: &str) {
        let full_path = format!("/usr/share/defaults/{}", rel_path.trim_start_matches('/'));
        self.stateless_defaults.insert(full_path, content.to_string());
    }

    pub fn resolve_config_fallback(&self, user_etc_path: &str) -> Option<String> {
        let default_path = format!("/usr/share/defaults/{}", user_etc_path.trim_start_matches('/'));
        self.stateless_defaults.get(&default_path).cloned()
    }
}

impl Default for HaikuHpkgPackageFsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_ports_flavours_and_vuxml() {
        let mut engine = FreeBsdPortsFlavoursAndVuxmlEngine::new();
        engine.register_port(PortFlavourSpec {
            origin: "devel/py-certifi".to_string(),
            selected_flavour: "py311".to_string(),
            available_flavours: vec!["py311".to_string(), "py312".to_string()],
            default_options: vec!["DOCS".to_string()],
            flavour_options: vec![],
            flavour_dependencies: vec!["lang/python311".to_string()],
        });

        let deps = engine.select_flavour("devel/py-certifi", "py312").unwrap();
        assert!(deps.contains(&"lang/python311".to_string()));

        engine.register_vulnerability(VuXmlVulnerabilityRecord {
            vid: "vuxml-001".to_string(),
            affected_package: "openssl".to_string(),
            vulnerable_range_min: "3.0.0".to_string(),
            vulnerable_range_max: "3.0.8".to_string(),
            severity: VulnerabilitySeverity::Critical,
            cve_ids: vec!["CVE-2023-0286".to_string()],
            description: "Type confusion in X.509".to_string(),
            fixed_version: "3.0.9".to_string(),
        });

        let vulns = engine.audit_installed_packages(&[("openssl", "3.0.5")]);
        assert_eq!(vulns.len(), 1);
        assert_eq!(vulns[0].severity, VulnerabilitySeverity::Critical);
    }

    #[test]
    fn test_xbps_soname_and_orphan_engine() {
        let mut xbps = XbpsSonameAndOrphanEngine::new();

        xbps.register_installed_package(ElfSonameRecord {
            package_name: "openssl".to_string(),
            provided_sonames: vec!["libssl.so.3".to_string(), "libcrypto.so.3".to_string()],
            required_sonames: vec!["libc.so.6".to_string()],
            is_explicitly_installed: false,
        });

        xbps.register_installed_package(ElfSonameRecord {
            package_name: "curl".to_string(),
            provided_sonames: vec!["libcurl.so.4".to_string()],
            required_sonames: vec!["libssl.so.3".to_string(), "libc.so.6".to_string()],
            is_explicitly_installed: true,
        });

        xbps.register_installed_package(ElfSonameRecord {
            package_name: "glibc".to_string(),
            provided_sonames: vec!["libc.so.6".to_string()],
            required_sonames: vec![],
            is_explicitly_installed: true,
        });

        assert!(xbps.verify_soname_integrity().is_ok());

        // openssl is required by curl, so no orphans
        let orphans = xbps.find_orphan_packages();
        assert_eq!(orphans.len(), 0);
    }

    #[test]
    fn test_alpine_apk_world_and_virtual_pkg() {
        let mut apk = AlpineApkWorldAndVirtualPkgEngine::new();
        apk.add_world_rule(ApkWorldRule {
            package_name: "bash".to_string(),
            version_constraint: Some(">=5.2".to_string()),
            pinned_repository_tag: None,
        });
        assert_eq!(apk.world_rules.len(), 1);

        apk.create_virtual_package_set(".build-deps-zlib", &["make", "gcc"], 100);
        assert!(apk.installed_packages.contains(&"gcc".to_string()));

        let purged = apk.purge_virtual_package_set(".build-deps-zlib").unwrap();
        assert_eq!(purged.len(), 2);
        assert!(!apk.installed_packages.contains(&"gcc".to_string()));
    }

    #[test]
    fn test_nix_guix_cas_gc_profile_engine() {
        let mut store = NixGuixCasGcProfileEngine::new();

        store.add_store_object("/nix/store/pkg1-bash", b"BASH_PAYLOAD");
        store.add_store_object("/nix/store/pkg2-unused", b"UNUSED_PAYLOAD");

        store.add_gc_root("/nix/store/pkg1-bash");
        let pruned = store.collect_garbage();
        assert_eq!(pruned, 1);
        assert!(store.store_objects.contains_key("/nix/store/pkg1-bash"));
        assert!(!store.store_objects.contains_key("/nix/store/pkg2-unused"));
    }

    #[test]
    fn test_arch_split_package_and_hooks() {
        let mut arch = ArchSplitPackageHookRunnerEngine::new();
        arch.register_split_pkg(PkgbuildSplitPackageSpec {
            base_pkgname: "ffmpeg".to_string(),
            sub_packages: vec!["libavcodec".to_string(), "ffmpeg-cli".to_string()],
            common_depends: vec!["glibc".to_string()],
        });

        arch.register_hook(AlpmPathHookRule {
            name: "glib-schemas".to_string(),
            when: HookWhen::PostTransaction,
            path_trigger: "glib-2.0/schemas".to_string(),
            exec_command: "glib-compile-schemas /usr/share/glib-2.0/schemas".to_string(),
        });

        let triggered = arch.trigger_hooks_for_paths(HookWhen::PostTransaction, &["/usr/share/glib-2.0/schemas/org.gnome.gschema.xml"]);
        assert_eq!(triggered, 1);
    }

    #[test]
    fn test_fedora_dnf5_and_delta_rpm() {
        let mut dnf = FedoraDnf5AdvisoryAndDeltaRpmEngine::new();
        dnf.register_advisory(Dnf5AdvisoryNotice {
            advisory_id: "FEDORA-2026-001".to_string(),
            kind: Dnf5AdvisoryKind::Security,
            affected_package: "kernel".to_string(),
            new_version: "6.8.1".to_string(),
            cve_references: vec!["CVE-2026-0001".to_string()],
        });

        dnf.register_delta(DeltaRpmPatchSpec {
            package_name: "kernel".to_string(),
            old_version: "6.8.0".to_string(),
            new_version: "6.8.1".to_string(),
            delta_size_bytes: 5_000_000,
            full_size_bytes: 80_000_000,
        });

        let sec_updates = dnf.filter_security_updates();
        assert_eq!(sec_updates.len(), 1);

        let (full, delta) = dnf.calculate_delta_savings();
        assert_eq!(full, 80_000_000);
        assert_eq!(delta, 5_000_000);
    }

    #[test]
    fn test_gentoo_portage_subslot() {
        let mut portage = GentooPortageSubslotAndUseExpandEngine::new();
        portage.register_subslot(PortageSubslotSpec {
            package_atom: "dev-libs/openssl".to_string(),
            slot: "0".to_string(),
            subslot: "3.0".to_string(),
            reverse_dependents: vec!["net-misc/curl".to_string()],
        });

        portage.set_use_expand("CPU_FLAGS_X86", &["avx2", "fma"]);
        let flags = portage.expand_use_flags();
        assert!(flags.contains(&"cpu_flags_x86_avx2".to_string()));

        let rebuilds = portage.check_subslot_rebuild_trigger("dev-libs/openssl", "3.1");
        assert_eq!(rebuilds, vec!["net-misc/curl".to_string()]);
    }

    #[test]
    fn test_haiku_hpkg_and_moss_stateless() {
        let mut engine = HaikuHpkgPackageFsEngine::new();
        let path = engine.mount_hpkg_package(HpkgContainerSpec {
            hpkg_filename: "zsh-5.9.hpkg".to_string(),
            mounted_vfs_path: "/system/packages/zsh-5.9.hpkg".to_string(),
            contained_files: vec!["/bin/zsh".to_string()],
            is_mounted: false,
        }).unwrap();
        assert_eq!(path, "/system/packages/zsh-5.9.hpkg");

        engine.set_stateless_default_config("/etc/zshrc", "# Default zshrc");
        let fallback = engine.resolve_config_fallback("/etc/zshrc").unwrap();
        assert_eq!(fallback, "# Default zshrc");
    }
}
