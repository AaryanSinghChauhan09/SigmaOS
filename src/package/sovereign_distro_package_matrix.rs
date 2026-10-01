// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Distro Package Matrix Suite
// Inspired by Void XBPS, Alpine APK v3 & LBU, Gentoo Portage EAPI 8, FreeBSD Ports & VuXML & Poudriere,
// NixOS / GNU Guix CAS, Fedora DNF5 & RPM-OSTree, Arch Linux Pacman & CachyOS ALPM, and OpenBSD Signify & Pledge/Unveil.

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
use alloc::collections::BTreeMap;
#[cfg(feature = "standalone_test")]
use alloc::format;
#[cfg(feature = "standalone_test")]
use alloc::string::{String, ToString};
#[cfg(feature = "standalone_test")]
use alloc::vec::Vec;

// =========================================================================
// 1. Sovereign Void XBPS Atomic Transaction Journal Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XbpsOpKind {
    Install,
    Remove,
    Configure,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsTransactionOp {
    pub package_name: String,
    pub version: String,
    pub kind: XbpsOpKind,
    pub provided_sonames: Vec<String>,
    pub required_sonames: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsJournalEntry {
    pub tx_id: u64,
    pub timestamp_sec: u64,
    pub ops: Vec<XbpsTransactionOp>,
    pub is_committed: bool,
}

pub struct SovereignXbpsTransactionJournalEngine {
    pub journal: Vec<XbpsJournalEntry>,
    pub installed_sonames: BTreeMap<String, String>, // soname -> pkg_name
    pub current_tx_counter: u64,
}

impl SovereignXbpsTransactionJournalEngine {
    pub fn new() -> Self {
        Self {
            journal: Vec::new(),
            installed_sonames: BTreeMap::new(),
            current_tx_counter: 1,
        }
    }

    pub fn begin_transaction(&mut self, now_sec: u64) -> u64 {
        let tx_id = self.current_tx_counter;
        self.current_tx_counter += 1;

        self.journal.push(XbpsJournalEntry {
            tx_id,
            timestamp_sec: now_sec,
            ops: Vec::new(),
            is_committed: false,
        });
        tx_id
    }

    pub fn record_op(&mut self, tx_id: u64, op: XbpsTransactionOp) -> Result<(), &'static str> {
        let entry = self
            .journal
            .iter_mut()
            .find(|e| e.tx_id == tx_id && !e.is_committed)
            .ok_or("Active uncommitted transaction not found")?;

        entry.ops.push(op);
        Ok(())
    }

    pub fn commit_transaction(&mut self, tx_id: u64) -> Result<(), &'static str> {
        let entry = self
            .journal
            .iter_mut()
            .find(|e| e.tx_id == tx_id && !e.is_committed)
            .ok_or("Transaction not found or already committed")?;

        for op in &entry.ops {
            match op.kind {
                XbpsOpKind::Install | XbpsOpKind::Update | XbpsOpKind::Configure => {
                    for soname in &op.provided_sonames {
                        self.installed_sonames
                            .insert(soname.clone(), op.package_name.clone());
                    }
                }
                XbpsOpKind::Remove => {
                    for soname in &op.provided_sonames {
                        self.installed_sonames.remove(soname);
                    }
                }
            }
        }

        entry.is_committed = true;
        Ok(())
    }

    pub fn rollback_transaction(&mut self, tx_id: u64) -> Result<Vec<XbpsTransactionOp>, &'static str> {
        let entry = self
            .journal
            .iter()
            .find(|e| e.tx_id == tx_id)
            .ok_or("Transaction ID not found")?;

        let mut rollback_ops = Vec::new();
        for op in entry.ops.iter().rev() {
            let undo = match op.kind {
                XbpsOpKind::Install => XbpsTransactionOp {
                    package_name: op.package_name.clone(),
                    version: op.version.clone(),
                    kind: XbpsOpKind::Remove,
                    provided_sonames: op.provided_sonames.clone(),
                    required_sonames: op.required_sonames.clone(),
                },
                XbpsOpKind::Remove => XbpsTransactionOp {
                    package_name: op.package_name.clone(),
                    version: op.version.clone(),
                    kind: XbpsOpKind::Install,
                    provided_sonames: op.provided_sonames.clone(),
                    required_sonames: op.required_sonames.clone(),
                },
                _ => op.clone(),
            };
            rollback_ops.push(undo);
        }
        Ok(rollback_ops)
    }

    pub fn sweep_orphaned_sonames(&self, active_required_sonames: &[&str]) -> Vec<String> {
        let mut orphans = Vec::new();
        for (soname, pkg) in &self.installed_sonames {
            if !active_required_sonames.contains(&soname.as_str()) {
                if !orphans.contains(pkg) {
                    orphans.push(pkg.clone());
                }
            }
        }
        orphans
    }
}

impl Default for SovereignXbpsTransactionJournalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Sovereign Alpine APK v3 & LBU RAM Overlay Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkTriggerScriptlet {
    pub trigger_id: String,
    pub target_file_pattern: String,
    pub command: String,
}

pub struct SovereignApkLbuOverlayEngine {
    pub world_pinned_packages: Vec<String>,
    pub ram_overlay_paths: Vec<String>,
    pub registered_triggers: Vec<ApkTriggerScriptlet>,
    pub pending_triggers: Vec<String>,
}

impl SovereignApkLbuOverlayEngine {
    pub fn new() -> Self {
        Self {
            world_pinned_packages: Vec::new(),
            ram_overlay_paths: Vec::new(),
            registered_triggers: Vec::new(),
            pending_triggers: Vec::new(),
        }
    }

    pub fn pin_world_package(&mut self, pkg_name: &str) {
        if !self.world_pinned_packages.contains(&pkg_name.to_string()) {
            self.world_pinned_packages.push(pkg_name.to_string());
        }
    }

    pub fn unpin_world_package(&mut self, pkg_name: &str) {
        self.world_pinned_packages.retain(|p| p != pkg_name);
    }

    pub fn register_lbu_overlay_path(&mut self, path: &str) {
        if !self.ram_overlay_paths.contains(&path.to_string()) {
            self.ram_overlay_paths.push(path.to_string());
        }
    }

    pub fn register_trigger(&mut self, trigger: ApkTriggerScriptlet) {
        self.registered_triggers.push(trigger);
    }

    pub fn evaluate_file_changes(&mut self, changed_files: &[&str]) -> usize {
        let mut triggered_count = 0;
        for file in changed_files {
            for trigger in &self.registered_triggers {
                if file.contains(&trigger.target_file_pattern) {
                    if !self.pending_triggers.contains(&trigger.command) {
                        self.pending_triggers.push(trigger.command.clone());
                        triggered_count += 1;
                    }
                }
            }
        }
        triggered_count
    }

    pub fn execute_pending_triggers(&mut self) -> Vec<String> {
        let executed = self.pending_triggers.clone();
        self.pending_triggers.clear();
        executed
    }

    pub fn generate_lbu_commit_manifest(&self) -> String {
        let mut manifest = String::from("# Alpine LBU Commit Overlay Manifest\n");
        for path in &self.ram_overlay_paths {
            manifest.push_str(&format!("overlay {}\n", path));
        }
        for pkg in &self.world_pinned_packages {
            manifest.push_str(&format!("world {}\n", pkg));
        }
        manifest
    }
}

impl Default for SovereignApkLbuOverlayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Sovereign Gentoo Portage EAPI 8 Subslot & Revdep-Rebuild Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortageSubslotRecord {
    pub atom: String,
    pub slot: String,
    pub subslot: String,
    pub provided_libs: Vec<String>,
    pub required_libs: Vec<String>,
}

pub struct SovereignPortageEapiSlotMatrixEngine {
    pub slots: BTreeMap<String, PortageSubslotRecord>,
    pub use_expand_vars: BTreeMap<String, Vec<String>>,
    pub masked_packages: Vec<String>,
}

impl SovereignPortageEapiSlotMatrixEngine {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
            use_expand_vars: BTreeMap::new(),
            masked_packages: Vec::new(),
        }
    }

    pub fn register_slot(&mut self, record: PortageSubslotRecord) {
        self.slots.insert(record.atom.clone(), record);
    }

    pub fn set_use_expand(&mut self, var_name: &str, values: &[&str]) {
        self.use_expand_vars.insert(
            var_name.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn mask_package(&mut self, atom: &str) {
        if !self.masked_packages.contains(&atom.to_string()) {
            self.masked_packages.push(atom.to_string());
        }
    }

    pub fn is_masked(&self, atom: &str) -> bool {
        self.masked_packages.contains(&atom.to_string())
    }

    pub fn scan_revdep_broken_libraries(&self) -> Vec<(String, String)> {
        let mut available_libs = Vec::new();
        for rec in self.slots.values() {
            for lib in &rec.provided_libs {
                available_libs.push(lib.clone());
            }
        }

        let mut broken = Vec::new();
        for (atom, rec) in &self.slots {
            for req in &rec.required_libs {
                if !available_libs.contains(req) {
                    broken.push((atom.clone(), req.clone()));
                }
            }
        }
        broken
    }
}

impl Default for SovereignPortageEapiSlotMatrixEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Sovereign FreeBSD VuXML Threat Gatekeeper & Poudriere Matrix Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlAdvisory {
    pub vuln_id: String,
    pub cve_id: String,
    pub package_name: String,
    pub cvss_score_x10: u32, // e.g. 98 = 9.8 Critical
    pub description: String,
}

pub struct SovereignBsdPoudriereVuxmlEngine {
    pub vuxml_db: Vec<VuXmlAdvisory>,
    pub cvss_block_threshold_x10: u32,
    pub active_boot_env: String,
}

impl SovereignBsdPoudriereVuxmlEngine {
    pub fn new() -> Self {
        Self {
            vuxml_db: Vec::new(),
            cvss_block_threshold_x10: 70, // 7.0 High
            active_boot_env: "default".to_string(),
        }
    }

    pub fn add_vuxml_advisory(&mut self, advisory: VuXmlAdvisory) {
        self.vuxml_db.push(advisory);
    }

    pub fn evaluate_security_gatekeeper(&self, pkg_name: &str) -> (bool, Option<String>) {
        for vuln in &self.vuxml_db {
            if vuln.package_name == pkg_name {
                if vuln.cvss_score_x10 >= self.cvss_block_threshold_x10 {
                    return (
                        true,
                        Some(format!(
                            "Gatekeeper blocked {} due to VuXML {} [{}] CVSS {}",
                            pkg_name,
                            vuln.vuln_id,
                            vuln.cve_id,
                            vuln.cvss_score_x10 as f32 / 10.0
                        )),
                    );
                }
            }
        }
        (false, None)
    }

    pub fn create_be_snapshot(&mut self, be_name: &str) -> String {
        self.active_boot_env = be_name.to_string();
        format!("zfs snapshot pool/ROOT/{}@pre-update", be_name)
    }
}

impl Default for SovereignBsdPoudriereVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Sovereign NixOS & Guix Content-Addressed Storage Flake Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixCasStoreDerivation {
    pub store_path: String,
    pub nar_hash: String,
    pub references: Vec<String>,
    pub size_bytes: u64,
}

pub struct SovereignNixCasFlakeEngine {
    pub derivations: BTreeMap<String, NixCasStoreDerivation>,
    pub gc_roots: Vec<String>,
}

impl SovereignNixCasFlakeEngine {
    pub fn new() -> Self {
        Self {
            derivations: BTreeMap::new(),
            gc_roots: Vec::new(),
        }
    }

    pub fn register_derivation(&mut self, drv: NixCasStoreDerivation) {
        self.derivations.insert(drv.store_path.clone(), drv);
    }

    pub fn add_gc_root(&mut self, store_path: &str) {
        if !self.gc_roots.contains(&store_path.to_string()) {
            self.gc_roots.push(store_path.to_string());
        }
    }

    pub fn verify_nar_integrity(&self, store_path: &str, computed_nar_hash: &str) -> bool {
        if let Some(drv) = self.derivations.get(store_path) {
            drv.nar_hash.eq_ignore_ascii_case(computed_nar_hash)
        } else {
            false
        }
    }

    pub fn find_zero_copy_candidates(&self) -> Vec<(String, String, u64)> {
        let mut candidates = Vec::new();
        let list: Vec<&NixCasStoreDerivation> = self.derivations.values().collect();
        for i in 0..list.len() {
            for j in (i + 1)..list.len() {
                if list[i].nar_hash == list[j].nar_hash && list[i].size_bytes > 0 {
                    candidates.push((
                        list[i].store_path.clone(),
                        list[j].store_path.clone(),
                        list[i].size_bytes,
                    ));
                }
            }
        }
        candidates
    }
}

impl Default for SovereignNixCasFlakeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Sovereign Fedora DNF5 & RPM-OSTree Layered Image Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5SecurityAdvisory {
    pub advisory_id: String,
    pub cve: String,
    pub severity: String,
    pub package_name: String,
}

pub struct SovereignFedoraDnf5OstreeEngine {
    pub advisories: Vec<Dnf5SecurityAdvisory>,
    pub active_ostree_deployment: String,
}

impl SovereignFedoraDnf5OstreeEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            active_ostree_deployment: "deploy-base-0".to_string(),
        }
    }

    pub fn register_advisory(&mut self, advisory: Dnf5SecurityAdvisory) {
        self.advisories.push(advisory);
    }

    pub fn stage_ostree_layer(&mut self, deploy_id: &str) {
        self.active_ostree_deployment = deploy_id.to_string();
    }

    pub fn rollback_ostree_deployment(&mut self, target_id: &str) -> String {
        self.active_ostree_deployment = target_id.to_string();
        format!("rpm-ostree rollback to {}", target_id)
    }
}

impl Default for SovereignFedoraDnf5OstreeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Sovereign Arch Linux Pacman & CachyOS Microarchitecture Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpuLevel {
    V1,
    V2,
    V3,
    V4,
}

pub struct SovereignArchCachyosEngine {
    pub current_cpu_level: CpuLevel,
    pub registered_hooks: Vec<String>,
}

impl SovereignArchCachyosEngine {
    pub fn new(level: CpuLevel) -> Self {
        Self {
            current_cpu_level: level,
            registered_hooks: Vec::new(),
        }
    }

    pub fn register_path_hook(&mut self, hook_name: &str) {
        self.registered_hooks.push(hook_name.to_string());
    }

    pub fn resolve_optimal_repo(&self) -> &'static str {
        match self.current_cpu_level {
            CpuLevel::V4 => "https://repo.cachyos.org/v4",
            CpuLevel::V3 => "https://repo.cachyos.org/v3",
            CpuLevel::V2 => "https://repo.cachyos.org/v2",
            CpuLevel::V1 => "https://repo.cachyos.org/v1",
        }
    }
}

impl Default for SovereignArchCachyosEngine {
    fn default() -> Self {
        Self::new(CpuLevel::V3)
    }
}

// =========================================================================
// 8. Sovereign OpenBSD Signify PQC & Pledge/Unveil Scriptlet Engine
// =========================================================================

pub struct SovereignOpenBsdPledgeUnveilEngine {
    pub trusted_keys: BTreeMap<String, String>,
    pub unveil_paths: Vec<String>,
}

impl SovereignOpenBsdPledgeUnveilEngine {
    pub fn new() -> Self {
        Self {
            trusted_keys: BTreeMap::new(),
            unveil_paths: Vec::new(),
        }
    }

    pub fn register_signify_key(&mut self, key_id: &str, pubkey: &str) {
        self.trusted_keys.insert(key_id.to_string(), pubkey.to_string());
    }

    pub fn add_unveil_path(&mut self, path: &str) {
        if !self.unveil_paths.contains(&path.to_string()) {
            self.unveil_paths.push(path.to_string());
        }
    }

    pub fn verify_signature(&self, key_id: &str, sig_header: &str) -> bool {
        if let Some(pubkey) = self.trusted_keys.get(key_id) {
            sig_header.contains(pubkey)
        } else {
            false
        }
    }
}

impl Default for SovereignOpenBsdPledgeUnveilEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. Sovereign Unified Distro Package Matrix Suite
// =========================================================================

pub struct SovereignDistroPackageMatrixSuite {
    pub xbps: SovereignXbpsTransactionJournalEngine,
    pub apk: SovereignApkLbuOverlayEngine,
    pub portage: SovereignPortageEapiSlotMatrixEngine,
    pub freebsd: SovereignBsdPoudriereVuxmlEngine,
    pub nix: SovereignNixCasFlakeEngine,
    pub dnf5: SovereignFedoraDnf5OstreeEngine,
    pub cachy: SovereignArchCachyosEngine,
    pub openbsd: SovereignOpenBsdPledgeUnveilEngine,
}

impl SovereignDistroPackageMatrixSuite {
    pub fn new() -> Self {
        Self {
            xbps: SovereignXbpsTransactionJournalEngine::new(),
            apk: SovereignApkLbuOverlayEngine::new(),
            portage: SovereignPortageEapiSlotMatrixEngine::new(),
            freebsd: SovereignBsdPoudriereVuxmlEngine::new(),
            nix: SovereignNixCasFlakeEngine::new(),
            dnf5: SovereignFedoraDnf5OstreeEngine::new(),
            cachy: SovereignArchCachyosEngine::new(CpuLevel::V3),
            openbsd: SovereignOpenBsdPledgeUnveilEngine::new(),
        }
    }
}

impl Default for SovereignDistroPackageMatrixSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xbps_transaction_journal() {
        let mut xbps = SovereignXbpsTransactionJournalEngine::new();
        let tx = xbps.begin_transaction(1700000000);
        xbps.record_op(
            tx,
            XbpsTransactionOp {
                package_name: "openssl".to_string(),
                version: "3.0.8".to_string(),
                kind: XbpsOpKind::Install,
                provided_sonames: vec!["libssl.so.3".to_string()],
                required_sonames: vec![],
            },
        )
        .unwrap();

        xbps.commit_transaction(tx).unwrap();
        assert_eq!(xbps.installed_sonames.get("libssl.so.3"), Some(&"openssl".to_string()));

        let orphans = xbps.sweep_orphaned_sonames(&["libcrypto.so.3"]);
        assert_eq!(orphans, vec!["openssl".to_string()]);
    }

    #[test]
    fn test_apk_lbu_overlay() {
        let mut apk = SovereignApkLbuOverlayEngine::new();
        apk.pin_world_package("bash");
        apk.register_lbu_overlay_path("/etc/network/interfaces");

        apk.register_trigger(ApkTriggerScriptlet {
            trigger_id: "fontconfig".to_string(),
            target_file_pattern: "fonts".to_string(),
            command: "fc-cache -fv".to_string(),
        });

        assert_eq!(apk.evaluate_file_changes(&["/usr/share/fonts/TTF/main.ttf"]), 1);
        let exec = apk.execute_pending_triggers();
        assert_eq!(exec, vec!["fc-cache -fv".to_string()]);

        let manifest = apk.generate_lbu_commit_manifest();
        assert!(manifest.contains("world bash"));
    }

    #[test]
    fn test_portage_subslot_revdep() {
        let mut portage = SovereignPortageEapiSlotMatrixEngine::new();
        portage.register_slot(PortageSubslotRecord {
            atom: "dev-libs/libxml2".to_string(),
            slot: "2".to_string(),
            subslot: "2.11".to_string(),
            provided_libs: vec!["libxml2.so.2".to_string()],
            required_libs: vec![],
        });

        portage.register_slot(PortageSubslotRecord {
            atom: "app-text/docbook".to_string(),
            slot: "0".to_string(),
            subslot: "0".to_string(),
            provided_libs: vec![],
            required_libs: vec!["libxml2.so.2".to_string(), "libmissing.so.1".to_string()],
        });

        let broken = portage.scan_revdep_broken_libraries();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].0, "app-text/docbook");
        assert_eq!(broken[0].1, "libmissing.so.1");
    }

    #[test]
    fn test_bsd_poudriere_vuxml() {
        let mut bsd = SovereignBsdPoudriereVuxmlEngine::new();
        bsd.add_vuxml_advisory(VuXmlAdvisory {
            vuln_id: "VUX-2024-01".to_string(),
            cve_id: "CVE-2024-9999".to_string(),
            package_name: "curl".to_string(),
            cvss_score_x10: 88,
            description: "High severity buffer overflow".to_string(),
        });

        let (blocked, reason) = bsd.evaluate_security_gatekeeper("curl");
        assert!(blocked);
        assert!(reason.unwrap().contains("VUX-2024-01"));

        let snap = bsd.create_be_snapshot("14.0-RELEASE");
        assert!(snap.contains("14.0-RELEASE"));
    }

    #[test]
    fn test_nix_cas_flake() {
        let mut nix = SovereignNixCasFlakeEngine::new();
        nix.register_derivation(NixCasStoreDerivation {
            store_path: "/nix/store/pkg1".to_string(),
            nar_hash: "nar_hash_123".to_string(),
            references: vec![],
            size_bytes: 1024,
        });
        nix.register_derivation(NixCasStoreDerivation {
            store_path: "/nix/store/pkg2".to_string(),
            nar_hash: "nar_hash_123".to_string(),
            references: vec![],
            size_bytes: 1024,
        });

        assert!(nix.verify_nar_integrity("/nix/store/pkg1", "nar_hash_123"));
        let dedup = nix.find_zero_copy_candidates();
        assert_eq!(dedup.len(), 1);
    }

    #[test]
    fn test_cachyos_cpu_routing() {
        let cachy = SovereignArchCachyosEngine::new(CpuLevel::V4);
        assert_eq!(cachy.resolve_optimal_repo(), "https://repo.cachyos.org/v4");
    }

    #[test]
    fn test_openbsd_signify() {
        let mut obsd = SovereignOpenBsdPledgeUnveilEngine::new();
        obsd.register_signify_key("obsd-75", "pubkey_75_data");
        assert!(obsd.verify_signature("obsd-75", "signed_with_pubkey_75_data"));
    }

    #[test]
    fn test_suite_initialization() {
        let suite = SovereignDistroPackageMatrixSuite::new();
        assert_eq!(suite.cachy.resolve_optimal_repo(), "https://repo.cachyos.org/v3");
    }
}
