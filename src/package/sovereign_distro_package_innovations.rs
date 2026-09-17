// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Distro Package Innovations Suite
// Inspired by OpenBSD Signify & Pledge/Unveil, Alpine APK v3 & LBU, Gentoo Portage EAPI 8,
// FreeBSD Ports & VuXML & Poudriere, NixOS & GNU Guix CAS, Fedora DNF5 & RPM-OSTree,
// Arch Linux Pacman & CachyOS Microarchitecture, and Void Linux XBPS.

#[cfg(all(feature = "standalone_test", not(test)))]
extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use std::collections::BTreeMap;
#[cfg(not(feature = "standalone_test"))]
use std::format;
#[cfg(not(feature = "standalone_test"))]
use std::string::{String, ToString};
#[cfg(not(feature = "standalone_test"))]
use std::vec::Vec;

#[cfg(feature = "standalone_test")]
use std::collections::BTreeMap;
#[cfg(feature = "standalone_test")]
use std::format;
#[cfg(feature = "standalone_test")]
use std::string::{String, ToString};
#[cfg(feature = "standalone_test")]
use std::vec::Vec;

// =========================================================================
// 1. Sovereign OpenBSD Signify PQC & Pledge/Unveil Scriptlet Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBsdSignifyKey {
    pub key_id: String,
    pub pubkey: String,
    pub is_pqc_dilithium: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptletSandboxPolicy {
    pub package_name: String,
    pub pledge_promises: Vec<String>,
    pub unveil_paths: Vec<(String, String)>, // (path, permissions "r", "rw", etc)
}

pub struct SovereignOpenBsdSignifyPledgeEngine {
    pub trusted_keys: BTreeMap<String, OpenBsdSignifyKey>,
    pub package_policies: BTreeMap<String, ScriptletSandboxPolicy>,
}

impl SovereignOpenBsdSignifyPledgeEngine {
    pub fn new() -> Self {
        Self {
            trusted_keys: BTreeMap::new(),
            package_policies: BTreeMap::new(),
        }
    }

    pub fn register_key(&mut self, key: OpenBsdSignifyKey) {
        self.trusted_keys.insert(key.key_id.clone(), key);
    }

    pub fn register_sandbox_policy(&mut self, policy: ScriptletSandboxPolicy) {
        self.package_policies
            .insert(policy.package_name.clone(), policy);
    }

    pub fn verify_package_signature(&self, key_id: &str, signature: &str) -> bool {
        if let Some(key) = self.trusted_keys.get(key_id) {
            if key.is_pqc_dilithium {
                signature.starts_with("pqc-dilithium-v1:") && signature.contains(&key.pubkey)
            } else {
                signature.starts_with("untrusted comment: verify with ") && signature.contains(&key.pubkey)
            }
        } else {
            false
        }
    }

    pub fn generate_scriptlet_pledge_unveil(&self, pkg_name: &str) -> String {
        if let Some(policy) = self.package_policies.get(pkg_name) {
            let promises = policy.pledge_promises.join(" ");
            let mut unveil_str = String::new();
            for (path, perm) in &policy.unveil_paths {
                unveil_str.push_str(&format!("unveil(\"{}\", \"{}\");\n", path, perm));
            }
            format!(
                "# OpenBSD Pledge/Unveil Sandbox Guard\n{}\npledge(\"{}\", NULL);",
                unveil_str, promises
            )
        } else {
            "pledge(\"stdio rpath\", NULL);".to_string()
        }
    }
}

impl Default for SovereignOpenBsdSignifyPledgeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Sovereign Alpine APK v3 & LBU RAM Overlay State Governor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkIndexEntry {
    pub name: String,
    pub version: String,
    pub checksum_sha256: String,
    pub depends: Vec<String>,
}

pub struct SovereignAlpineApkLbuGovernorEngine {
    pub apk_index: BTreeMap<String, ApkIndexEntry>,
    pub world_pinned_packages: Vec<String>,
    pub lbu_overlay_files: Vec<String>,
    pub index_signature_verified: bool,
}

impl SovereignAlpineApkLbuGovernorEngine {
    pub fn new() -> Self {
        Self {
            apk_index: BTreeMap::new(),
            world_pinned_packages: Vec::new(),
            lbu_overlay_files: Vec::new(),
            index_signature_verified: false,
        }
    }

    pub fn load_apk_index(&mut self, index_entries: Vec<ApkIndexEntry>, signature_valid: bool) {
        self.index_signature_verified = signature_valid;
        if signature_valid {
            for entry in index_entries {
                self.apk_index.insert(entry.name.clone(), entry);
            }
        }
    }

    pub fn pin_world_package(&mut self, pkg_name: &str) {
        if !self.world_pinned_packages.contains(&pkg_name.to_string()) {
            self.world_pinned_packages.push(pkg_name.to_string());
        }
    }

    pub fn track_lbu_file(&mut self, filepath: &str) {
        if !self.lbu_overlay_files.contains(&filepath.to_string()) {
            self.lbu_overlay_files.push(filepath.to_string());
        }
    }

    pub fn generate_apkovl_tarball_manifest(&self) -> String {
        let mut manifest = String::from("# Alpine LBU apkovl overlay manifest\n");
        for file in &self.lbu_overlay_files {
            manifest.push_str(&format!("file: {}\n", file));
        }
        manifest.push_str("# Pinned World Packages\n");
        for pkg in &self.world_pinned_packages {
            manifest.push_str(&format!("world: {}\n", pkg));
        }
        manifest
    }
}

impl Default for SovereignAlpineApkLbuGovernorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Sovereign Gentoo Portage EAPI 8 USE_EXPAND & Revdep-Rebuild Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EbuildSlotMetadata {
    pub atom: String,
    pub slot: String,
    pub subslot: String,
    pub provided_sonames: Vec<String>,
    pub needed_sonames: Vec<String>,
}

pub struct SovereignGentooPortageEapi8Engine {
    pub registered_slots: BTreeMap<String, EbuildSlotMetadata>,
    pub use_expand_map: BTreeMap<String, Vec<String>>, // e.g. "PYTHON_TARGETS" -> ["python3_11", "python3_12"]
    pub masked_atoms: Vec<String>,
}

impl SovereignGentooPortageEapi8Engine {
    pub fn new() -> Self {
        Self {
            registered_slots: BTreeMap::new(),
            use_expand_map: BTreeMap::new(),
            masked_atoms: Vec::new(),
        }
    }

    pub fn register_ebuild_slot(&mut self, metadata: EbuildSlotMetadata) {
        self.registered_slots.insert(metadata.atom.clone(), metadata);
    }

    pub fn set_use_expand(&mut self, category: &str, flags: &[&str]) {
        self.use_expand_map.insert(
            category.to_string(),
            flags.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn mask_atom(&mut self, atom: &str) {
        if !self.masked_atoms.contains(&atom.to_string()) {
            self.masked_atoms.push(atom.to_string());
        }
    }

    pub fn scan_revdep_broken_packages(&self) -> Vec<(String, String)> {
        let mut available_sonames: Vec<String> = Vec::new();
        for meta in self.registered_slots.values() {
            for soname in &meta.provided_sonames {
                available_sonames.push(soname.clone());
            }
        }

        let mut broken: Vec<(String, String)> = Vec::new();
        for (atom, meta) in &self.registered_slots {
            for needed in &meta.needed_sonames {
                if !available_sonames.contains(needed) {
                    broken.push((atom.clone(), needed.clone()));
                }
            }
        }
        broken
    }
}

impl Default for SovereignGentooPortageEapi8Engine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Sovereign FreeBSD Ports VuXML & Poudriere BE Snapshot Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlVulnRecord {
    pub vuxml_id: String,
    pub cve: String,
    pub package_pattern: String,
    pub cvss_score_x10: u32, // e.g. 95 = 9.5
}

pub struct SovereignFreeBsdPoudriereVuxmlEngine {
    pub vulnerabilities: Vec<VuXmlVulnRecord>,
    pub cvss_critical_threshold_x10: u32,
    pub boot_environments: Vec<String>,
}

impl SovereignFreeBsdPoudriereVuxmlEngine {
    pub fn new() -> Self {
        Self {
            vulnerabilities: Vec::new(),
            cvss_critical_threshold_x10: 75, // 7.5
            boot_environments: vec!["default".to_string()],
        }
    }

    pub fn add_vuxml_record(&mut self, record: VuXmlVulnRecord) {
        self.vulnerabilities.push(record);
    }

    pub fn check_package_vulnerability(&self, pkg_name: &str) -> (bool, Option<VuXmlVulnRecord>) {
        for vuln in &self.vulnerabilities {
            if pkg_name.contains(&vuln.package_pattern) {
                return (vuln.cvss_score_x10 >= self.cvss_critical_threshold_x10, Some(vuln.clone()));
            }
        }
        (false, None)
    }

    pub fn create_bectl_snapshot(&mut self, be_name: &str) -> String {
        self.boot_environments.push(be_name.to_string());
        format!("bectl create {}", be_name)
    }
}

impl Default for SovereignFreeBsdPoudriereVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Sovereign NixOS & GNU Guix Hermetic CAS Verification Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixStoreDerivationPath {
    pub store_path: String, // /nix/store/hash-name
    pub nar_sha256: String,
    pub references: Vec<String>,
    pub size_bytes: u64,
}

pub struct SovereignNixGuixCasVerificationEngine {
    pub store_paths: BTreeMap<String, NixStoreDerivationPath>,
    pub gc_roots: Vec<String>,
}

impl SovereignNixGuixCasVerificationEngine {
    pub fn new() -> Self {
        Self {
            store_paths: BTreeMap::new(),
            gc_roots: Vec::new(),
        }
    }

    pub fn register_store_path(&mut self, path: NixStoreDerivationPath) {
        self.store_paths.insert(path.store_path.clone(), path);
    }

    pub fn add_gc_root(&mut self, path: &str) {
        if !self.gc_roots.contains(&path.to_string()) {
            self.gc_roots.push(path.to_string());
        }
    }

    pub fn verify_nar_sha256(&self, store_path: &str, expected_hash: &str) -> bool {
        if let Some(drv) = self.store_paths.get(store_path) {
            drv.nar_sha256.eq_ignore_ascii_case(expected_hash)
        } else {
            false
        }
    }

    pub fn detect_cas_duplicates(&self) -> Vec<(String, String, u64)> {
        let mut duplicates = Vec::new();
        let paths: Vec<&NixStoreDerivationPath> = self.store_paths.values().collect();
        for i in 0..paths.len() {
            for j in (i + 1)..paths.len() {
                if paths[i].nar_sha256 == paths[j].nar_sha256 {
                    duplicates.push((
                        paths[i].store_path.clone(),
                        paths[j].store_path.clone(),
                        paths[i].size_bytes,
                    ));
                }
            }
        }
        duplicates
    }
}

impl Default for SovereignNixGuixCasVerificationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Sovereign Fedora DNF5 Security Advisory & RPM-OSTree Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dnf5AdvisorySeverity {
    Critical,
    Important,
    Moderate,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5Advisory {
    pub advisory_id: String,
    pub cve: String,
    pub severity: Dnf5AdvisorySeverity,
    pub target_package: String,
}

pub struct SovereignFedoraDnf5RpmOstreeEngine {
    pub advisories: Vec<Dnf5Advisory>,
    pub current_ostree_deployment: String,
    pub deployment_history: Vec<String>,
}

impl SovereignFedoraDnf5RpmOstreeEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            current_ostree_deployment: "deploy-base-v1".to_string(),
            deployment_history: vec!["deploy-base-v1".to_string()],
        }
    }

    pub fn register_advisory(&mut self, advisory: Dnf5Advisory) {
        self.advisories.push(advisory);
    }

    pub fn filter_critical_advisories(&self) -> Vec<Dnf5Advisory> {
        self.advisories
            .iter()
            .filter(|a| a.severity == Dnf5AdvisorySeverity::Critical || a.severity == Dnf5AdvisorySeverity::Important)
            .cloned()
            .collect()
    }

    pub fn stage_layered_deployment(&mut self, deploy_id: &str) {
        self.current_ostree_deployment = deploy_id.to_string();
        self.deployment_history.push(deploy_id.to_string());
    }

    pub fn rollback_ostree_deployment(&mut self) -> Result<String, &'static str> {
        if self.deployment_history.len() > 1 {
            self.deployment_history.pop();
            let previous = self.deployment_history.last().unwrap().clone();
            self.current_ostree_deployment = previous.clone();
            Ok(format!("rpm-ostree rollback to {}", previous))
        } else {
            Err("No previous ostree deployment to rollback to")
        }
    }
}

impl Default for SovereignFedoraDnf5RpmOstreeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Sovereign Arch Linux Pacman & CachyOS Microarchitecture Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicroarchTier {
    V1, // x86-64-v1
    V2, // x86-64-v2 (SSSE3, SSE4.2)
    V3, // x86-64-v3 (AVX2, BMI2)
    V4, // x86-64-v4 (AVX-512)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorLatencyRecord {
    pub url: String,
    pub latency_ms: u32,
}

pub struct SovereignArchCachyosMicroarchEngine {
    pub current_tier: MicroarchTier,
    pub mirrors: Vec<MirrorLatencyRecord>,
    pub pacdiff_conflicts: Vec<(String, String)>, // (original, pacnew)
}

impl SovereignArchCachyosMicroarchEngine {
    pub fn new(tier: MicroarchTier) -> Self {
        Self {
            current_tier: tier,
            mirrors: Vec::new(),
            pacdiff_conflicts: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u32) {
        self.mirrors.push(MirrorLatencyRecord {
            url: url.to_string(),
            latency_ms,
        });
    }

    pub fn select_fastest_mirror(&self) -> Option<String> {
        self.mirrors
            .iter()
            .min_by_key(|m| m.latency_ms)
            .map(|m| m.url.clone())
    }

    pub fn resolve_cachyos_repo_url(&self) -> &'static str {
        match self.current_tier {
            MicroarchTier::V4 => "https://repo.cachyos.org/v4",
            MicroarchTier::V3 => "https://repo.cachyos.org/v3",
            MicroarchTier::V2 => "https://repo.cachyos.org/v2",
            MicroarchTier::V1 => "https://repo.cachyos.org/v1",
        }
    }

    pub fn register_pacdiff(&mut self, config: &str, pacnew: &str) {
        self.pacdiff_conflicts.push((config.to_string(), pacnew.to_string()));
    }
}

impl Default for SovereignArchCachyosMicroarchEngine {
    fn default() -> Self {
        Self::new(MicroarchTier::V3)
    }
}

// =========================================================================
// 8. Sovereign Void XBPS Atomic Transaction Journal Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XbpsOpType {
    InstallPackage,
    RemovePackage,
    UpdatePackage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsTransactionItem {
    pub package_name: String,
    pub version: String,
    pub op_type: XbpsOpType,
    pub provided_sonames: Vec<String>,
}

pub struct SovereignVoidXbpsJournalEngine {
    pub transaction_log: Vec<XbpsTransactionItem>,
    pub installed_libraries: BTreeMap<String, String>, // soname -> package
}

impl SovereignVoidXbpsJournalEngine {
    pub fn new() -> Self {
        Self {
            transaction_log: Vec::new(),
            installed_libraries: BTreeMap::new(),
        }
    }

    pub fn record_transaction(&mut self, item: XbpsTransactionItem) {
        match item.op_type {
            XbpsOpType::InstallPackage | XbpsOpType::UpdatePackage => {
                for soname in &item.provided_sonames {
                    self.installed_libraries
                        .insert(soname.clone(), item.package_name.clone());
                }
            }
            XbpsOpType::RemovePackage => {
                for soname in &item.provided_sonames {
                    self.installed_libraries.remove(soname);
                }
            }
        }
        self.transaction_log.push(item);
    }

    pub fn generate_rollback_ops(&self) -> Vec<XbpsTransactionItem> {
        let mut rollbacks = Vec::new();
        for item in self.transaction_log.iter().rev() {
            let undo_op = match item.op_type {
                XbpsOpType::InstallPackage => XbpsOpType::RemovePackage,
                XbpsOpType::RemovePackage => XbpsOpType::InstallPackage,
                XbpsOpType::UpdatePackage => XbpsOpType::UpdatePackage,
            };
            rollbacks.push(XbpsTransactionItem {
                package_name: item.package_name.clone(),
                version: item.version.clone(),
                op_type: undo_op,
                provided_sonames: item.provided_sonames.clone(),
            });
        }
        rollbacks
    }

    pub fn find_orphaned_sonames(&self, active_required_sonames: &[&str]) -> Vec<String> {
        let mut orphans: Vec<String> = Vec::new();
        for (soname, pkg) in &self.installed_libraries {
            if !active_required_sonames.contains(&soname.as_str()) {
                if !orphans.contains(pkg) {
                    orphans.push(pkg.clone());
                }
            }
        }
        orphans
    }
}

impl Default for SovereignVoidXbpsJournalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. Sovereign Distro Package Innovations Suite
// =========================================================================

pub struct SovereignDistroPackageInnovationsSuite {
    pub openbsd_engine: SovereignOpenBsdSignifyPledgeEngine,
    pub alpine_engine: SovereignAlpineApkLbuGovernorEngine,
    pub gentoo_engine: SovereignGentooPortageEapi8Engine,
    pub freebsd_engine: SovereignFreeBsdPoudriereVuxmlEngine,
    pub nix_engine: SovereignNixGuixCasVerificationEngine,
    pub fedora_engine: SovereignFedoraDnf5RpmOstreeEngine,
    pub cachy_engine: SovereignArchCachyosMicroarchEngine,
    pub xbps_engine: SovereignVoidXbpsJournalEngine,
}

impl SovereignDistroPackageInnovationsSuite {
    pub fn new() -> Self {
        Self {
            openbsd_engine: SovereignOpenBsdSignifyPledgeEngine::new(),
            alpine_engine: SovereignAlpineApkLbuGovernorEngine::new(),
            gentoo_engine: SovereignGentooPortageEapi8Engine::new(),
            freebsd_engine: SovereignFreeBsdPoudriereVuxmlEngine::new(),
            nix_engine: SovereignNixGuixCasVerificationEngine::new(),
            fedora_engine: SovereignFedoraDnf5RpmOstreeEngine::new(),
            cachy_engine: SovereignArchCachyosMicroarchEngine::new(MicroarchTier::V3),
            xbps_engine: SovereignVoidXbpsJournalEngine::new(),
        }
    }
}

impl Default for SovereignDistroPackageInnovationsSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openbsd_signify_pledge() {
        let mut engine = SovereignOpenBsdSignifyPledgeEngine::new();
        engine.register_key(OpenBsdSignifyKey {
            key_id: "key-1".to_string(),
            pubkey: "pubkey-123".to_string(),
            is_pqc_dilithium: true,
        });

        assert!(engine.verify_package_signature("key-1", "pqc-dilithium-v1:pubkey-123"));

        engine.register_sandbox_policy(ScriptletSandboxPolicy {
            package_name: "nginx".to_string(),
            pledge_promises: vec!["stdio".to_string(), "rpath".to_string(), "inet".to_string()],
            unveil_paths: vec![("/etc/nginx".to_string(), "r".to_string())],
        });

        let script = engine.generate_scriptlet_pledge_unveil("nginx");
        assert!(script.contains("pledge(\"stdio rpath inet\", NULL);"));
        assert!(script.contains("unveil(\"/etc/nginx\", \"r\");"));
    }

    #[test]
    fn test_alpine_lbu_governor() {
        let mut engine = SovereignAlpineApkLbuGovernorEngine::new();
        engine.pin_world_package("busybox");
        engine.track_lbu_file("/etc/apk/world");

        let manifest = engine.generate_apkovl_tarball_manifest();
        assert!(manifest.contains("file: /etc/apk/world"));
        assert!(manifest.contains("world: busybox"));
    }

    #[test]
    fn test_gentoo_portage_eapi8() {
        let mut engine = SovereignGentooPortageEapi8Engine::new();
        engine.register_ebuild_slot(EbuildSlotMetadata {
            atom: "sys-libs/zlib".to_string(),
            slot: "0".to_string(),
            subslot: "1.3".to_string(),
            provided_sonames: vec!["libz.so.1".to_string()],
            needed_sonames: vec![],
        });

        engine.register_ebuild_slot(EbuildSlotMetadata {
            atom: "app-arch/tar".to_string(),
            slot: "0".to_string(),
            subslot: "0".to_string(),
            provided_sonames: vec![],
            needed_sonames: vec!["libz.so.1".to_string(), "libmissing.so".to_string()],
        });

        let broken = engine.scan_revdep_broken_packages();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].0, "app-arch/tar");
        assert_eq!(broken[0].1, "libmissing.so");
    }

    #[test]
    fn test_freebsd_vuxml_poudriere() {
        let mut engine = SovereignFreeBsdPoudriereVuxmlEngine::new();
        engine.add_vuxml_record(VuXmlVulnRecord {
            vuxml_id: "VUX-01".to_string(),
            cve: "CVE-2024-0001".to_string(),
            package_pattern: "openssl".to_string(),
            cvss_score_x10: 85,
        });

        let (is_vuln, rec) = engine.check_package_vulnerability("openssl");
        assert!(is_vuln);
        assert_eq!(rec.unwrap().cve, "CVE-2024-0001");

        let snap = engine.create_bectl_snapshot("14.0-RELEASE-p1");
        assert_eq!(snap, "bectl create 14.0-RELEASE-p1");
    }

    #[test]
    fn test_nix_cas_verification() {
        let mut engine = SovereignNixGuixCasVerificationEngine::new();
        engine.register_store_path(NixStoreDerivationPath {
            store_path: "/nix/store/111-pkg1".to_string(),
            nar_sha256: "sha256_hash_1".to_string(),
            references: vec![],
            size_bytes: 2048,
        });
        engine.register_store_path(NixStoreDerivationPath {
            store_path: "/nix/store/222-pkg2".to_string(),
            nar_sha256: "sha256_hash_1".to_string(),
            references: vec![],
            size_bytes: 2048,
        });

        assert!(engine.verify_nar_sha256("/nix/store/111-pkg1", "sha256_hash_1"));
        let dups = engine.detect_cas_duplicates();
        assert_eq!(dups.len(), 1);
        assert_eq!(dups[0].2, 2048);
    }

    #[test]
    fn test_fedora_dnf5_ostree() {
        let mut engine = SovereignFedoraDnf5RpmOstreeEngine::new();
        engine.register_advisory(Dnf5Advisory {
            advisory_id: "FEDORA-2024-01".to_string(),
            cve: "CVE-2024-1234".to_string(),
            severity: Dnf5AdvisorySeverity::Critical,
            target_package: "glibc".to_string(),
        });

        let crits = engine.filter_critical_advisories();
        assert_eq!(crits.len(), 1);

        engine.stage_layered_deployment("deploy-base-v2");
        let rollback_msg = engine.rollback_ostree_deployment().unwrap();
        assert!(rollback_msg.contains("deploy-base-v1"));
    }

    #[test]
    fn test_cachyos_microarch() {
        let mut engine = SovereignArchCachyosMicroarchEngine::new(MicroarchTier::V4);
        engine.add_mirror("https://mirror1.com", 50);
        engine.add_mirror("https://mirror2.com", 20);

        assert_eq!(engine.select_fastest_mirror(), Some("https://mirror2.com".to_string()));
        assert_eq!(engine.resolve_cachyos_repo_url(), "https://repo.cachyos.org/v4");
    }

    #[test]
    fn test_xbps_journal() {
        let mut engine = SovereignVoidXbpsJournalEngine::new();
        engine.record_transaction(XbpsTransactionItem {
            package_name: "curl".to_string(),
            version: "8.5.0".to_string(),
            op_type: XbpsOpType::InstallPackage,
            provided_sonames: vec!["libcurl.so.4".to_string()],
        });

        let rollbacks = engine.generate_rollback_ops();
        assert_eq!(rollbacks[0].op_type, XbpsOpType::RemovePackage);

        let orphans = engine.find_orphaned_sonames(&[]);
        assert_eq!(orphans, vec!["curl".to_string()]);
    }

    #[test]
    fn test_distro_innovations_suite() {
        let suite = SovereignDistroPackageInnovationsSuite::new();
        assert_eq!(suite.cachy_engine.resolve_cachyos_repo_url(), "https://repo.cachyos.org/v3");
    }
}
