// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Linux & BSD Inspired Package Management Innovations
// Features Void XBPS, Alpine APK v3 & LBU, Gentoo Portage EAPI 8, FreeBSD Ports/VuXML/Poudriere,
// Nix/Guix CAS & Flakes, Fedora DNF5 & RPM-OSTree, Arch/CachyOS Microarch, OpenBSD Signify/Pledge/Unveil.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. Void Linux XBPS Sovereign Transaction Journal Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XbpsOpType {
    Install,
    Update,
    Remove,
    Reconfigure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsTransactionAction {
    pub package_name: String,
    pub version: String,
    pub op_type: XbpsOpType,
    pub is_explicit: bool,
    pub provided_sonames: Vec<String>,
    pub required_sonames: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsTransactionSnapshot {
    pub tx_id: u64,
    pub timestamp_sec: u64,
    pub actions: Vec<XbpsTransactionAction>,
    pub signature_verified: bool,
}

pub struct SovereignXbpsTransactionJournalEngine {
    pub history: Vec<XbpsTransactionSnapshot>,
    pub installed_packages: BTreeMap<String, XbpsTransactionAction>,
    pub next_tx_id: u64,
}

impl SovereignXbpsTransactionJournalEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            installed_packages: BTreeMap::new(),
            next_tx_id: 1,
        }
    }

    pub fn commit_transaction(
        &mut self,
        actions: Vec<XbpsTransactionAction>,
        sig_verified: bool,
        now_sec: u64,
    ) -> Result<u64, &'static str> {
        if !sig_verified {
            return Err("Transaction signature verification failed; transaction aborted");
        }

        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        for action in &actions {
            match action.op_type {
                XbpsOpType::Install | XbpsOpType::Update | XbpsOpType::Reconfigure => {
                    self.installed_packages.insert(action.package_name.clone(), action.clone());
                }
                XbpsOpType::Remove => {
                    self.installed_packages.remove(&action.package_name);
                }
            }
        }

        let snapshot = XbpsTransactionSnapshot {
            tx_id,
            timestamp_sec: now_sec,
            actions,
            signature_verified: sig_verified,
        };
        self.history.push(snapshot);
        Ok(tx_id)
    }

    pub fn rollback_transaction(&mut self, tx_id: u64) -> Result<usize, &'static str> {
        let pos = self
            .history
            .iter()
            .position(|s| s.tx_id == tx_id)
            .ok_or("Transaction ID not found in journal")?;

        let snapshot = self.history.remove(pos);
        let mut count = 0;

        for action in snapshot.actions.iter().rev() {
            match action.op_type {
                XbpsOpType::Install | XbpsOpType::Update => {
                    self.installed_packages.remove(&action.package_name);
                    count += 1;
                }
                XbpsOpType::Remove => {
                    self.installed_packages.insert(action.package_name.clone(), action.clone());
                    count += 1;
                }
                XbpsOpType::Reconfigure => {}
            }
        }
        Ok(count)
    }

    pub fn find_orphaned_packages(&self) -> Vec<String> {
        let mut required_sonames = Vec::new();
        for pkg in self.installed_packages.values() {
            for req in &pkg.required_sonames {
                required_sonames.push(req.clone());
            }
        }

        let mut orphans = Vec::new();
        for (name, pkg) in &self.installed_packages {
            if !pkg.is_explicit {
                let provides_required = pkg.provided_sonames.iter().any(|so| required_sonames.contains(so));
                if !provides_required {
                    orphans.push(name.clone());
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
// 2. Alpine Linux APK v3 & LBU Sovereign Diskless Overlay Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkTriggerScriptlet {
    pub package_name: String,
    pub target_directory: String,
    pub trigger_script: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkWorldPinRule {
    pub package_name: String,
    pub version_pin: Option<String>,
}

pub struct SovereignApkLbuOverlayEngine {
    pub world_pins: Vec<ApkWorldPinRule>,
    pub pending_triggers: Vec<ApkTriggerScriptlet>,
    pub executed_triggers: Vec<String>,
    pub overlay_commit_history: Vec<String>,
}

impl SovereignApkLbuOverlayEngine {
    pub fn new() -> Self {
        Self {
            world_pins: Vec::new(),
            pending_triggers: Vec::new(),
            executed_triggers: Vec::new(),
            overlay_commit_history: Vec::new(),
        }
    }

    pub fn pin_package_in_world(&mut self, package: &str, version: Option<&str>) {
        if let Some(rule) = self.world_pins.iter_mut().find(|r| r.package_name == package) {
            rule.version_pin = version.map(|s| s.to_string());
        } else {
            self.world_pins.push(ApkWorldPinRule {
                package_name: package.to_string(),
                version_pin: version.map(|s| s.to_string()),
            });
        }
    }

    pub fn register_trigger(&mut self, trigger: ApkTriggerScriptlet) {
        if !self.pending_triggers.contains(&trigger) {
            self.pending_triggers.push(trigger);
        }
    }

    pub fn execute_pending_triggers(&mut self, modified_dirs: &[&str]) -> usize {
        let mut executed = 0;
        let mut remaining = Vec::new();

        for trigger in self.pending_triggers.drain(..) {
            let matches = modified_dirs
                .iter()
                .any(|dir| dir.starts_with(&trigger.target_directory) || trigger.target_directory == "*");

            if matches {
                self.executed_triggers.push(trigger.trigger_script.clone());
                executed += 1;
            } else {
                remaining.push(trigger);
            }
        }
        self.pending_triggers = remaining;
        executed
    }

    pub fn commit_lbu_overlay_snapshot(&mut self, commit_msg: &str) -> String {
        let commit_id = format!("lbu-commit-{}-{}", self.overlay_commit_history.len() + 1, commit_msg);
        self.overlay_commit_history.push(commit_id.clone());
        commit_id
    }
}

impl Default for SovereignApkLbuOverlayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Gentoo Portage EAPI 8 Sovereign Slot & USE Matrix Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortageEbuildAtom {
    pub category_pkg: String,
    pub slot: String,
    pub subslot: String,
    pub use_flags: Vec<String>,
    pub license: String,
    pub masked: bool,
}

pub struct SovereignPortageEapiSlotMatrixEngine {
    pub installed_atoms: BTreeMap<String, PortageEbuildAtom>,
    pub accepted_licenses: Vec<String>,
    pub use_expand_map: BTreeMap<String, Vec<String>>,
}

impl SovereignPortageEapiSlotMatrixEngine {
    pub fn new() -> Self {
        Self {
            installed_atoms: BTreeMap::new(),
            accepted_licenses: vec![
                "MIT".to_string(),
                "GPL-2.0-or-later".to_string(),
                "Apache-2.0".to_string(),
                "BSD-3-Clause".to_string(),
            ],
            use_expand_map: BTreeMap::new(),
        }
    }

    pub fn register_atom(&mut self, atom: PortageEbuildAtom) {
        self.installed_atoms.insert(atom.category_pkg.clone(), atom);
    }

    pub fn set_use_expand(&mut self, var: &str, values: &[&str]) {
        self.use_expand_map.insert(
            var.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn check_license_accepted(&self, pkg: &str) -> Result<bool, &'static str> {
        let atom = self.installed_atoms.get(pkg).ok_or("Package atom not found")?;
        Ok(self.accepted_licenses.contains(&atom.license) || self.accepted_licenses.contains(&"*".to_string()))
    }

    pub fn calculate_revdep_rebuilds(&self, changed_pkg: &str, new_subslot: &str) -> Vec<String> {
        let mut rebuilds = Vec::new();
        if let Some(target) = self.installed_atoms.get(changed_pkg) {
            if target.subslot != new_subslot {
                for (other_pkg, atom) in &self.installed_atoms {
                    if other_pkg != changed_pkg && atom.use_flags.iter().any(|u| u.contains(changed_pkg)) {
                        rebuilds.push(other_pkg.clone());
                    }
                }
            }
        }
        rebuilds
    }
}

impl Default for SovereignPortageEapiSlotMatrixEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. FreeBSD Ports, VuXML & Poudriere Sovereign Security Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlAdvisory {
    pub vuln_id: String,
    pub cve: String,
    pub package_name: String,
    pub min_ver: String,
    pub max_ver: String,
    pub cvss_score_x10: u32, // 98 = 9.8 Critical
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoudriereBuildTarget {
    pub jail_name: String,
    pub freebsd_version: String,
    pub target_arch: String,
}

pub struct SovereignBsdPoudriereVuxmlEngine {
    pub advisories: Vec<VuXmlAdvisory>,
    pub jails: Vec<PoudriereBuildTarget>,
    pub boot_environments: Vec<String>,
}

impl SovereignBsdPoudriereVuxmlEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            jails: Vec::new(),
            boot_environments: vec!["default".to_string()],
        }
    }

    pub fn register_advisory(&mut self, adv: VuXmlAdvisory) {
        self.advisories.push(adv);
    }

    pub fn add_poudriere_jail(&mut self, jail: PoudriereBuildTarget) {
        self.jails.push(jail);
    }

    pub fn audit_installed_package(&self, pkg_name: &str, ver: &str) -> Option<VuXmlAdvisory> {
        for adv in &self.advisories {
            if adv.package_name == pkg_name && ver >= adv.min_ver.as_str() && ver <= adv.max_ver.as_str() {
                return Some(adv.clone());
            }
        }
        None
    }

    pub fn create_cow_boot_environment(&mut self, be_name: &str) -> Result<String, &'static str> {
        if self.boot_environments.contains(&be_name.to_string()) {
            return Err("Boot environment name already exists");
        }
        self.boot_environments.push(be_name.to_string());
        Ok(format!("bectl create -e default {}", be_name))
    }
}

impl Default for SovereignBsdPoudriereVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. NixOS & GNU Guix Sovereign Content-Addressed Storage & Flake Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasStoreItem {
    pub store_path: String,
    pub nar_hash: String,
    pub size_bytes: u64,
    pub gc_roots: Vec<String>,
}

pub struct SovereignNixCasFlakeEngine {
    pub store: BTreeMap<String, CasStoreItem>,
}

impl SovereignNixCasFlakeEngine {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn register_store_item(&mut self, path: &str, nar_hash: &str, size_bytes: u64) {
        self.store.insert(
            path.to_string(),
            CasStoreItem {
                store_path: path.to_string(),
                nar_hash: nar_hash.to_string(),
                size_bytes,
                gc_roots: Vec::new(),
            },
        );
    }

    pub fn add_gc_root(&mut self, path: &str, root_name: &str) -> bool {
        if let Some(item) = self.store.get_mut(path) {
            if !item.gc_roots.contains(&root_name.to_string()) {
                item.gc_roots.push(root_name.to_string());
            }
            true
        } else {
            false
        }
    }

    pub fn run_garbage_collection(&mut self) -> (usize, u64) {
        let unreferenced: Vec<String> = self
            .store
            .values()
            .filter(|i| i.gc_roots.is_empty())
            .map(|i| i.store_path.clone())
            .collect();

        let mut freed_count = 0;
        let mut freed_bytes = 0;

        for path in unreferenced {
            if let Some(removed) = self.store.remove(&path) {
                freed_count += 1;
                freed_bytes += removed.size_bytes;
            }
        }
        (freed_count, freed_bytes)
    }

    pub fn deduplicate_store_entries(&self) -> Vec<(String, String, u64)> {
        let mut dups = Vec::new();
        let items: Vec<&CasStoreItem> = self.store.values().collect();

        for i in 0..items.len() {
            for j in (i + 1)..items.len() {
                if items[i].nar_hash == items[j].nar_hash && items[i].size_bytes > 0 {
                    dups.push((
                        items[i].store_path.clone(),
                        items[j].store_path.clone(),
                        items[i].size_bytes,
                    ));
                }
            }
        }
        dups
    }
}

impl Default for SovereignNixCasFlakeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Fedora DNF5 & RPM-OSTree Sovereign Advisory & Delta Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dnf5Severity {
    Security,
    Bugfix,
    Enhancement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5AdvisoryItem {
    pub advisory_id: String,
    pub package_name: String,
    pub severity: Dnf5Severity,
    pub cve_list: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaRpmPatchSpec {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub delta_size_bytes: u64,
    pub full_size_bytes: u64,
}

pub struct SovereignFedoraDnf5OstreeEngine {
    pub advisories: Vec<Dnf5AdvisoryItem>,
    pub delta_patches: Vec<DeltaRpmPatchSpec>,
}

impl SovereignFedoraDnf5OstreeEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            delta_patches: Vec::new(),
        }
    }

    pub fn register_advisory(&mut self, adv: Dnf5AdvisoryItem) {
        self.advisories.push(adv);
    }

    pub fn register_delta_patch(&mut self, patch: DeltaRpmPatchSpec) {
        self.delta_patches.push(patch);
    }

    pub fn filter_security_advisories(&self) -> Vec<Dnf5AdvisoryItem> {
        self.advisories
            .iter()
            .filter(|a| a.severity == Dnf5Severity::Security)
            .cloned()
            .collect()
    }

    pub fn calculate_bandwidth_savings(&self) -> u64 {
        self.delta_patches
            .iter()
            .map(|p| p.full_size_bytes.saturating_sub(p.delta_size_bytes))
            .sum()
    }
}

impl Default for SovereignFedoraDnf5OstreeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Arch Linux Pacman & CachyOS Sovereign Microarchitecture Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CpuMicroarchLevel {
    X86_64_V1,
    X86_64_V2,
    X86_64_V3,
    X86_64_V4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MicroarchRepoRoute {
    pub level: CpuMicroarchLevel,
    pub mirror_url: String,
    pub latency_ms: u32,
}

pub struct SovereignArchCachyosEngine {
    pub detected_level: CpuMicroarchLevel,
    pub routes: Vec<MicroarchRepoRoute>,
}

impl SovereignArchCachyosEngine {
    pub fn new() -> Self {
        Self {
            detected_level: CpuMicroarchLevel::X86_64_V1,
            routes: Vec::new(),
        }
    }

    pub fn detect_level_from_flags(&mut self, flags: &[&str]) -> CpuMicroarchLevel {
        let f: Vec<String> = flags.iter().map(|s| s.to_lowercase()).collect();

        let has_v2 = ["sse3", "ssse3", "sse4_1", "sse4_2", "popcnt"].iter().all(|k| f.contains(&k.to_string()));
        let has_v3 = has_v2 && ["avx", "avx2", "bmi1", "bmi2", "fma", "f16c"].iter().all(|k| f.contains(&k.to_string()));
        let has_v4 = has_v3 && ["avx512f", "avx512bw", "avx512cd", "avx512dq", "avx512vl"].iter().all(|k| f.contains(&k.to_string()));

        let level = if has_v4 {
            CpuMicroarchLevel::X86_64_V4
        } else if has_v3 {
            CpuMicroarchLevel::X86_64_V3
        } else if has_v2 {
            CpuMicroarchLevel::X86_64_V2
        } else {
            CpuMicroarchLevel::X86_64_V1
        };

        self.detected_level = level;
        level
    }

    pub fn register_repo_route(&mut self, level: CpuMicroarchLevel, url: &str, latency_ms: u32) {
        self.routes.push(MicroarchRepoRoute {
            level,
            mirror_url: url.to_string(),
            latency_ms,
        });
    }

    pub fn select_optimal_route(&self) -> Option<String> {
        self.routes
            .iter()
            .filter(|r| r.level <= self.detected_level)
            .min_by_key(|r| r.latency_ms)
            .map(|r| r.mirror_url.clone())
    }
}

impl Default for SovereignArchCachyosEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. OpenBSD Signify, Pledge & Unveil Sovereign Sandbox Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptletUnveilRule {
    pub path: String,
    pub mode: String, // "r", "rw", "rx"
}

pub struct SovereignOpenBsdPledgeUnveilEngine {
    pub trusted_signify_keys: BTreeMap<String, String>,
    pub unveil_rules: Vec<ScriptletUnveilRule>,
    pub pledge_promises: Vec<String>,
}

impl SovereignOpenBsdPledgeUnveilEngine {
    pub fn new() -> Self {
        Self {
            trusted_signify_keys: BTreeMap::new(),
            unveil_rules: Vec::new(),
            pledge_promises: vec!["stdio".to_string(), "rpath".to_string()],
        }
    }

    pub fn register_signify_key(&mut self, key_id: &str, pubkey: &str) {
        self.trusted_signify_keys.insert(key_id.to_string(), pubkey.to_string());
    }

    pub fn verify_signature(&self, key_id: &str, signature: &str) -> bool {
        if let Some(key) = self.trusted_signify_keys.get(key_id) {
            !signature.is_empty() && signature.contains(key)
        } else {
            false
        }
    }

    pub fn add_unveil_rule(&mut self, path: &str, mode: &str) {
        self.unveil_rules.push(ScriptletUnveilRule {
            path: path.to_string(),
            mode: mode.to_string(),
        });
    }

    pub fn validate_scriptlet_access(&self, target_path: &str, req_mode: &str) -> bool {
        for rule in &self.unveil_rules {
            if target_path.starts_with(&rule.path) && rule.mode.contains(req_mode) {
                return true;
            }
        }
        false
    }
}

impl Default for SovereignOpenBsdPledgeUnveilEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. Sovereign Distro Package Matrix Suite
// =========================================================================

#[derive(Debug, Clone)]
pub struct PackageSystemAuditReport {
    pub xbps_installed_count: usize,
    pub orphaned_package_count: usize,
    pub pending_triggers_count: usize,
    pub detected_cpu_microarch: CpuMicroarchLevel,
    pub optimal_repo_url: String,
    pub cas_store_items_count: usize,
    pub cas_dups_count: usize,
    pub bandwidth_saved_bytes: u64,
    pub security_advisories_count: usize,
}

pub struct SovereignDistroPackageMatrixSuite {
    pub xbps: SovereignXbpsTransactionJournalEngine,
    pub apk: SovereignApkLbuOverlayEngine,
    pub portage: SovereignPortageEapiSlotMatrixEngine,
    pub bsd: SovereignBsdPoudriereVuxmlEngine,
    pub nix: SovereignNixCasFlakeEngine,
    pub fedora: SovereignFedoraDnf5OstreeEngine,
    pub arch: SovereignArchCachyosEngine,
    pub openbsd: SovereignOpenBsdPledgeUnveilEngine,
}

impl SovereignDistroPackageMatrixSuite {
    pub fn new() -> Self {
        Self {
            xbps: SovereignXbpsTransactionJournalEngine::new(),
            apk: SovereignApkLbuOverlayEngine::new(),
            portage: SovereignPortageEapiSlotMatrixEngine::new(),
            bsd: SovereignBsdPoudriereVuxmlEngine::new(),
            nix: SovereignNixCasFlakeEngine::new(),
            fedora: SovereignFedoraDnf5OstreeEngine::new(),
            arch: SovereignArchCachyosEngine::new(),
            openbsd: SovereignOpenBsdPledgeUnveilEngine::new(),
        }
    }

    pub fn audit_and_optimize_package_system(&self) -> PackageSystemAuditReport {
        let orphans = self.xbps.find_orphaned_packages();
        let dups = self.nix.deduplicate_store_entries();
        let opt_route = self.arch.select_optimal_route().unwrap_or_else(|| "https://repo.sigmaos.org/core".to_string());

        PackageSystemAuditReport {
            xbps_installed_count: self.xbps.installed_packages.len(),
            orphaned_package_count: orphans.len(),
            pending_triggers_count: self.apk.pending_triggers.len(),
            detected_cpu_microarch: self.arch.detected_level,
            optimal_repo_url: opt_route,
            cas_store_items_count: self.nix.store.len(),
            cas_dups_count: dups.len(),
            bandwidth_saved_bytes: self.fedora.calculate_bandwidth_savings(),
            security_advisories_count: self.fedora.filter_security_advisories().len() + self.bsd.advisories.len(),
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
    fn test_xbps_journal_engine() {
        let mut xbps = SovereignXbpsTransactionJournalEngine::new();
        let actions = vec![
            XbpsTransactionAction {
                package_name: "glibc".to_string(),
                version: "2.39".to_string(),
                op_type: XbpsOpType::Install,
                is_explicit: true,
                provided_sonames: vec!["libc.so.6".to_string()],
                required_sonames: vec![],
            },
            XbpsTransactionAction {
                package_name: "libz".to_string(),
                version: "1.3.1".to_string(),
                op_type: XbpsOpType::Install,
                is_explicit: false,
                provided_sonames: vec!["libz.so.1".to_string()],
                required_sonames: vec!["libc.so.6".to_string()],
            },
        ];

        let tx_id = xbps.commit_transaction(actions, true, 1700000000).unwrap();
        assert_eq!(tx_id, 1);
        assert_eq!(xbps.installed_packages.len(), 2);

        let orphans = xbps.find_orphaned_packages();
        assert_eq!(orphans.len(), 1); // libz is an orphan as no installed package requires libz.so.1
        assert_eq!(orphans[0], "libz");

        let count = xbps.rollback_transaction(1).unwrap();
        assert_eq!(count, 2);
        assert_eq!(xbps.installed_packages.len(), 0);
    }

    #[test]
    fn test_apk_lbu_overlay_engine() {
        let mut apk = SovereignApkLbuOverlayEngine::new();
        apk.pin_package_in_world("bash", Some("5.2.21"));
        assert_eq!(apk.world_pins.len(), 1);

        apk.register_trigger(ApkTriggerScriptlet {
            package_name: "fontconfig".to_string(),
            target_directory: "/usr/share/fonts".to_string(),
            trigger_script: "fc-cache -s".to_string(),
        });

        let count = apk.execute_pending_triggers(&["/usr/share/fonts/TTF"]);
        assert_eq!(count, 1);
        assert_eq!(apk.executed_triggers.len(), 1);

        let commit_id = apk.commit_lbu_overlay_snapshot("sys-update");
        assert!(commit_id.contains("lbu-commit-1-sys-update"));
    }

    #[test]
    fn test_portage_matrix_engine() {
        let mut portage = SovereignPortageEapiSlotMatrixEngine::new();
        portage.register_atom(PortageEbuildAtom {
            category_pkg: "dev-libs/openssl".to_string(),
            slot: "0".to_string(),
            subslot: "3.0".to_string(),
            use_flags: vec!["ssl".to_string()],
            license: "Apache-2.0".to_string(),
            masked: false,
        });

        assert!(portage.check_license_accepted("dev-libs/openssl").unwrap());
    }

    #[test]
    fn test_bsd_poudriere_vuxml_engine() {
        let mut bsd = SovereignBsdPoudriereVuxmlEngine::new();
        bsd.register_advisory(VuXmlAdvisory {
            vuln_id: "vuln-1".to_string(),
            cve: "CVE-2024-0001".to_string(),
            package_name: "curl".to_string(),
            min_ver: "8.0.0".to_string(),
            max_ver: "8.5.0".to_string(),
            cvss_score_x10: 88,
        });

        let adv = bsd.audit_installed_package("curl", "8.2.0");
        assert!(adv.is_some());
        assert_eq!(adv.unwrap().cve, "CVE-2024-0001");

        let be = bsd.create_cow_boot_environment("be-14.1").unwrap();
        assert!(be.contains("be-14.1"));
    }

    #[test]
    fn test_nix_cas_flake_engine() {
        let mut nix = SovereignNixCasFlakeEngine::new();
        nix.register_store_item("/nix/store/pkg1", "hash123", 1000);
        nix.register_store_item("/nix/store/pkg2", "hash123", 1000);

        nix.add_gc_root("/nix/store/pkg1", "profile-root");

        let (freed_count, freed_bytes) = nix.run_garbage_collection();
        assert_eq!(freed_count, 1);
        assert_eq!(freed_bytes, 1000);

        let dups = nix.deduplicate_store_entries();
        assert_eq!(dups.len(), 0); // pkg2 was garbage collected
    }

    #[test]
    fn test_fedora_dnf5_ostree_engine() {
        let mut fedora = SovereignFedoraDnf5OstreeEngine::new();
        fedora.register_advisory(Dnf5AdvisoryItem {
            advisory_id: "FEDORA-2024-01".to_string(),
            package_name: "kernel".to_string(),
            severity: Dnf5Severity::Security,
            cve_list: vec!["CVE-2024-999".to_string()],
        });

        fedora.register_delta_patch(DeltaRpmPatchSpec {
            package_name: "kernel".to_string(),
            old_version: "6.8.0".to_string(),
            new_version: "6.8.1".to_string(),
            delta_size_bytes: 10_000_000,
            full_size_bytes: 100_000_000,
        });

        let sec = fedora.filter_security_advisories();
        assert_eq!(sec.len(), 1);
        assert_eq!(fedora.calculate_bandwidth_savings(), 90_000_000);
    }

    #[test]
    fn test_arch_cachyos_engine() {
        let mut arch = SovereignArchCachyosEngine::new();
        let flags = ["sse3", "ssse3", "sse4_1", "sse4_2", "popcnt", "avx", "avx2", "bmi1", "bmi2", "fma", "f16c"];
        let level = arch.detect_level_from_flags(&flags);
        assert_eq!(level, CpuMicroarchLevel::X86_64_V3);

        arch.register_repo_route(CpuMicroarchLevel::X86_64_V3, "https://repo.cachy.org/v3", 15);
        let route = arch.select_optimal_route().unwrap();
        assert_eq!(route, "https://repo.cachy.org/v3");
    }

    #[test]
    fn test_openbsd_pledge_unveil_engine() {
        let mut openbsd = SovereignOpenBsdPledgeUnveilEngine::new();
        openbsd.register_signify_key("key2024", "PUBKEY_BASE64_DATA");
        assert!(openbsd.verify_signature("key2024", "HEADER_PUBKEY_BASE64_DATA"));

        openbsd.add_unveil_rule("/etc/pkg", "r");
        assert!(openbsd.validate_scriptlet_access("/etc/pkg/config", "r"));
        assert!(!openbsd.validate_scriptlet_access("/var/run/secret", "w"));
    }

    #[test]
    fn test_distro_package_matrix_suite() {
        let suite = SovereignDistroPackageMatrixSuite::new();
        let report = suite.audit_and_optimize_package_system();
        assert_eq!(report.xbps_installed_count, 0);
        assert_eq!(report.detected_cpu_microarch, CpuMicroarchLevel::X86_64_V1);
    }
}
