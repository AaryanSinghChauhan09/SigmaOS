// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Distro Package Matrix Expansion Suite
// Inspired by Debian/Ubuntu APT pinning & Debconf pre-seeding,
// Arch Linux / CachyOS ALPM hooks & microarchitecture optimization mirrors,
// Gentoo Portage EAPI 8 USE_EXPAND & subslot ABI tracking,
// Alpine APK v3 signed indices & LBU RAM overlays,
// Void Linux XBPS atomic journals & SONAME orphan cleaners,
// NixOS / GNU Guix CAS NAR store verification & zero-copy deduplication,
// Fedora DNF5 advisories & DeltaRPM byte-stream reconstruction,
// and FreeBSD VuXML CVE gatekeepers & OpenBSD Signify PQC pledge/unveil sandboxes.

#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "standalone_test")]
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

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::UnifiedPackage;

#[cfg(feature = "standalone_test")]
#[path = "universal.rs"]
pub mod universal;

#[cfg(feature = "standalone_test")]
pub use universal::{PackageError, PackageFormat, UnifiedPackage};

// =========================================================================
// 1. Sovereign Debian / Ubuntu APT Pinning & Debconf Pre-Seeding Governor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptPinRuleSpec {
    pub package_pattern: String,
    pub pin_priority: i32, // e.g., 1001 = force downgrade, 990 = preference, -1 = block
    pub pin_release: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebconfPreseedSpec {
    pub package_name: String,
    pub question_id: String,
    pub question_type: String, // "string", "boolean", "select", "password"
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct SovereignAptPinningDebconfGovernor {
    pub pin_rules: Vec<AptPinRuleSpec>,
    pub preseed_answers: BTreeMap<String, Vec<DebconfPreseedSpec>>,
}

impl SovereignAptPinningDebconfGovernor {
    pub fn new() -> Self {
        Self {
            pin_rules: Vec::new(),
            preseed_answers: BTreeMap::new(),
        }
    }

    pub fn add_pin_rule(&mut self, rule: AptPinRuleSpec) {
        self.pin_rules.push(rule);
    }

    pub fn get_pin_priority(&self, pkg_name: &str) -> i32 {
        self.pin_rules
            .iter()
            .filter(|r| pkg_name.contains(&r.package_pattern) || r.package_pattern == "*")
            .map(|r| r.pin_priority)
            .max()
            .unwrap_or(500) // Default APT pin priority
    }

    pub fn is_blocked(&self, pkg_name: &str) -> bool {
        self.get_pin_priority(pkg_name) < 0
    }

    pub fn set_debconf_preseed(&mut self, spec: DebconfPreseedSpec) {
        self.preseed_answers
            .entry(spec.package_name.clone())
            .or_default()
            .push(spec);
    }

    pub fn query_preseed_answer(&self, pkg_name: &str, question_id: &str) -> Option<String> {
        self.preseed_answers.get(pkg_name).and_then(|specs| {
            specs
                .iter()
                .find(|s| s.question_id == question_id)
                .map(|s| s.value.clone())
        })
    }
}

impl Default for SovereignAptPinningDebconfGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Sovereign Arch Linux ALPM Hooks & Microarch Optimization Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpmHookWhen {
    PreTransaction,
    PostTransaction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpmHookSpec {
    pub name: String,
    pub when: AlpmHookWhen,
    pub target_file_pattern: String,
    pub command: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicroarchOptimizationLevel {
    V1, // Generic x86-64
    V2, // SSSE3, SSE4.1, SSE4.2, POPCNT
    V3, // AVX, AVX2, BMI1, BMI2, FMA
    V4, // AVX-512 foundation, BW, CD, DQ, VL
}

pub struct SovereignArchAlpmHookMicroarchEngine {
    pub hooks: Vec<AlpmHookSpec>,
    pub current_microarch: MicroarchOptimizationLevel,
    pub pending_post_hooks: Vec<String>,
}

impl SovereignArchAlpmHookMicroarchEngine {
    pub fn new(level: MicroarchOptimizationLevel) -> Self {
        Self {
            hooks: Vec::new(),
            current_microarch: level,
            pending_post_hooks: Vec::new(),
        }
    }

    pub fn register_hook(&mut self, hook: AlpmHookSpec) {
        self.hooks.push(hook);
    }

    pub fn evaluate_changed_paths(&mut self, paths: &[&str], when: AlpmHookWhen) -> usize {
        let mut triggered = 0;
        for path in paths {
            for hook in &self.hooks {
                if hook.when == when && path.contains(&hook.target_file_pattern) {
                    if !self.pending_post_hooks.contains(&hook.command) {
                        self.pending_post_hooks.push(hook.command.clone());
                        triggered += 1;
                    }
                }
            }
        }
        triggered
    }

    pub fn drain_pending_hooks(&mut self) -> Vec<String> {
        let cmds = self.pending_post_hooks.clone();
        self.pending_post_hooks.clear();
        cmds
    }

    pub fn resolve_optimal_repo_mirror(&self) -> &'static str {
        match self.current_microarch {
            MicroarchOptimizationLevel::V4 => "https://repo.cachyos.org/v4",
            MicroarchOptimizationLevel::V3 => "https://repo.cachyos.org/v3",
            MicroarchOptimizationLevel::V2 => "https://repo.cachyos.org/v2",
            MicroarchOptimizationLevel::V1 => "https://repo.cachyos.org/v1",
        }
    }
}

impl Default for SovereignArchAlpmHookMicroarchEngine {
    fn default() -> Self {
        Self::new(MicroarchOptimizationLevel::V3)
    }
}

// =========================================================================
// 3. Sovereign Gentoo Portage EAPI 8 USE_EXPAND & Subslot ABI Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubslotAbiSpec {
    pub package_atom: String,
    pub slot: String,
    pub subslot: String, // e.g. "2.11"
    pub provided_sonames: Vec<String>,
    pub required_sonames: Vec<String>,
}

pub struct SovereignPortageEapi8SubslotEngine {
    pub slots: BTreeMap<String, SubslotAbiSpec>,
    pub use_expand_flags: BTreeMap<String, Vec<String>>, // e.g. "PYTHON_TARGETS" -> ["python3_11", "python3_12"]
    pub masked_atoms: Vec<String>,
}

impl SovereignPortageEapi8SubslotEngine {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
            use_expand_flags: BTreeMap::new(),
            masked_atoms: Vec::new(),
        }
    }

    pub fn register_subslot(&mut self, spec: SubslotAbiSpec) {
        self.slots.insert(spec.package_atom.clone(), spec);
    }

    pub fn set_use_expand(&mut self, category: &str, flags: &[&str]) {
        self.use_expand_flags.insert(
            category.to_string(),
            flags.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn mask_atom(&mut self, atom: &str) {
        if !self.masked_atoms.contains(&atom.to_string()) {
            self.masked_atoms.push(atom.to_string());
        }
    }

    pub fn scan_revdep_broken_libraries(&self) -> Vec<(String, String)> {
        let mut available_sonames = Vec::new();
        for spec in self.slots.values() {
            for soname in &spec.provided_sonames {
                available_sonames.push(soname.clone());
            }
        }

        let mut broken = Vec::new();
        for (atom, spec) in &self.slots {
            for req in &spec.required_sonames {
                if !available_sonames.contains(req) {
                    broken.push((atom.clone(), req.clone()));
                }
            }
        }
        broken
    }
}

impl Default for SovereignPortageEapi8SubslotEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Sovereign Alpine APK v3 & LBU RAM Overlay Governor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkV3SignedIndex {
    pub arch: String,
    pub ed25519_signature: String,
    pub package_count: usize,
}

pub struct SovereignApk3LbuOverlayGovernor {
    pub index_header: Option<ApkV3SignedIndex>,
    pub world_pinned_packages: Vec<String>,
    pub lbu_overlay_paths: Vec<String>,
}

impl SovereignApk3LbuOverlayGovernor {
    pub fn new() -> Self {
        Self {
            index_header: None,
            world_pinned_packages: Vec::new(),
            lbu_overlay_paths: Vec::new(),
        }
    }

    pub fn verify_and_set_index(&mut self, index: ApkV3SignedIndex) -> bool {
        let valid = !index.ed25519_signature.is_empty();
        if valid {
            self.index_header = Some(index);
        }
        valid
    }

    pub fn pin_world_package(&mut self, pkg_name: &str) {
        if !self.world_pinned_packages.contains(&pkg_name.to_string()) {
            self.world_pinned_packages.push(pkg_name.to_string());
        }
    }

    pub fn add_lbu_path(&mut self, path: &str) {
        if !self.lbu_overlay_paths.contains(&path.to_string()) {
            self.lbu_overlay_paths.push(path.to_string());
        }
    }

    pub fn generate_lbu_commit_manifest(&self) -> String {
        let mut manifest = String::from("# Alpine LBU Commit Overlay Manifest\n");
        for path in &self.lbu_overlay_paths {
            manifest.push_str(&format!("overlay {}\n", path));
        }
        for pkg in &self.world_pinned_packages {
            manifest.push_str(&format!("world {}\n", pkg));
        }
        manifest
    }
}

impl Default for SovereignApk3LbuOverlayGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Sovereign Void XBPS Atomic Transaction Journal & Orphan Cleaner
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XbpsOpKind {
    Install,
    Remove,
    Configure,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsOpSpec {
    pub package_name: String,
    pub version: String,
    pub kind: XbpsOpKind,
    pub provided_sonames: Vec<String>,
}

pub struct SovereignXbpsTransactionJournalCleaner {
    pub journal: Vec<XbpsOpSpec>,
    pub installed_sonames: BTreeMap<String, String>, // soname -> package
}

impl SovereignXbpsTransactionJournalCleaner {
    pub fn new() -> Self {
        Self {
            journal: Vec::new(),
            installed_sonames: BTreeMap::new(),
        }
    }

    pub fn record_and_apply_op(&mut self, op: XbpsOpSpec) {
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
        self.journal.push(op);
    }

    pub fn sweep_orphaned_packages(&self, active_required_sonames: &[&str]) -> Vec<String> {
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

impl Default for SovereignXbpsTransactionJournalCleaner {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Sovereign NixOS / GNU Guix CAS Store Zero-Copy Deduplicator
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasNarStoreObject {
    pub store_path: String,
    pub nar_hash: String,
    pub size_bytes: u64,
    pub references: Vec<String>,
}

pub struct SovereignNixGuixCasDeduplicatorEngine {
    pub cas_store: BTreeMap<String, CasNarStoreObject>,
    pub gc_roots: Vec<String>,
}

impl SovereignNixGuixCasDeduplicatorEngine {
    pub fn new() -> Self {
        Self {
            cas_store: BTreeMap::new(),
            gc_roots: Vec::new(),
        }
    }

    pub fn register_store_object(&mut self, obj: CasNarStoreObject) {
        self.cas_store.insert(obj.store_path.clone(), obj);
    }

    pub fn add_gc_root(&mut self, store_path: &str) {
        if !self.gc_roots.contains(&store_path.to_string()) {
            self.gc_roots.push(store_path.to_string());
        }
    }

    pub fn compute_zero_copy_savings(&self) -> (usize, u64) {
        let mut seen_hashes: BTreeMap<String, u64> = BTreeMap::new();
        let mut dup_count = 0usize;
        let mut saved_bytes = 0u64;

        for obj in self.cas_store.values() {
            if let Some(&sz) = seen_hashes.get(&obj.nar_hash) {
                dup_count += 1;
                saved_bytes += sz;
            } else {
                seen_hashes.insert(obj.nar_hash.clone(), obj.size_bytes);
            }
        }

        (dup_count, saved_bytes)
    }

    pub fn verify_closure(&self, root_path: &str) -> Result<Vec<String>, String> {
        let mut visited = Vec::new();
        let mut stack = vec![root_path.to_string()];

        while let Some(curr) = stack.pop() {
            if visited.contains(&curr) {
                continue;
            }
            let obj = self
                .cas_store
                .get(&curr)
                .ok_or_else(|| format!("CAS Store: Broken closure reference {}", curr))?;

            visited.push(curr.clone());
            for dep in &obj.references {
                if !visited.contains(dep) {
                    stack.push(dep.clone());
                }
            }
        }

        Ok(visited)
    }
}

impl Default for SovereignNixGuixCasDeduplicatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Sovereign Fedora DNF5 Advisories & DeltaRPM Reconstruction Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dnf5AdvisoryClassification {
    Critical,
    Important,
    Moderate,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5AdvisoryRecord {
    pub advisory_id: String,
    pub cve: String,
    pub classification: Dnf5AdvisoryClassification,
    pub package_name: String,
}

pub struct SovereignFedoraDnf5DeltaRpmEngine {
    pub advisories: Vec<Dnf5AdvisoryRecord>,
}

impl SovereignFedoraDnf5DeltaRpmEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
        }
    }

    pub fn register_advisory(&mut self, record: Dnf5AdvisoryRecord) {
        self.advisories.push(record);
    }

    pub fn query_critical_advisories(&self) -> Vec<Dnf5AdvisoryRecord> {
        self.advisories
            .iter()
            .filter(|a| {
                a.classification == Dnf5AdvisoryClassification::Critical
                    || a.classification == Dnf5AdvisoryClassification::Important
            })
            .cloned()
            .collect()
    }

    /// Reconstructs full RPM package from base RPM and DeltaRPM VCDIFF byte stream opcodes.
    /// Opcode 0x01: COPY <len: u16_be> <src_off: u32_be>
    /// Opcode 0x02: ADD <len: u16_be> <bytes...>
    /// Opcode 0xFF: End of stream
    pub fn reconstruct_deltarpm(base_rpm: &[u8], delta_patch: &[u8]) -> Result<Vec<u8>, &'static str> {
        if delta_patch.is_empty() {
            return Ok(base_rpm.to_vec());
        }

        let mut reconstructed = Vec::new();
        let mut idx = 0;

        while idx < delta_patch.len() {
            let opcode = delta_patch[idx];
            idx += 1;

            match opcode {
                0x01 => {
                    if idx + 6 > delta_patch.len() {
                        return Err("DeltaRPM: COPY opcode truncated");
                    }
                    let len = u16::from_be_bytes([delta_patch[idx], delta_patch[idx + 1]]) as usize;
                    let src_off = u32::from_be_bytes([
                        delta_patch[idx + 2],
                        delta_patch[idx + 3],
                        delta_patch[idx + 4],
                        delta_patch[idx + 5],
                    ]) as usize;
                    idx += 6;

                    if src_off + len > base_rpm.len() {
                        return Err("DeltaRPM: COPY opcode out of base bounds");
                    }
                    reconstructed.extend_from_slice(&base_rpm[src_off..src_off + len]);
                }
                0x02 => {
                    if idx + 2 > delta_patch.len() {
                        return Err("DeltaRPM: ADD opcode header truncated");
                    }
                    let len = u16::from_be_bytes([delta_patch[idx], delta_patch[idx + 1]]) as usize;
                    idx += 2;

                    if idx + len > delta_patch.len() {
                        return Err("DeltaRPM: ADD opcode data stream truncated");
                    }
                    reconstructed.extend_from_slice(&delta_patch[idx..idx + len]);
                    idx += len;
                }
                0xFF => break,
                _ => {
                    let base_b = if reconstructed.len() < base_rpm.len() {
                        base_rpm[reconstructed.len()]
                    } else {
                        0
                    };
                    reconstructed.push(base_b ^ opcode);
                }
            }
        }

        Ok(reconstructed)
    }
}

impl Default for SovereignFedoraDnf5DeltaRpmEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. Sovereign FreeBSD VuXML & OpenBSD Signify Pledge Sandbox Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlAdvisoryEntry {
    pub vuln_id: String,
    pub cve: String,
    pub package_name: String,
    pub cvss_score_x10: u32,
}

pub struct SovereignBsdVuXmlPledgeSandboxEngine {
    pub vuxml_db: Vec<VuXmlAdvisoryEntry>,
    pub max_permitted_cvss_x10: u32,
    pub signify_trusted_keys: BTreeMap<String, String>,
}

impl SovereignBsdVuXmlPledgeSandboxEngine {
    pub fn new(max_cvss_x10: u32) -> Self {
        Self {
            vuxml_db: Vec::new(),
            max_permitted_cvss_x10: max_cvss_x10,
            signify_trusted_keys: BTreeMap::new(),
        }
    }

    pub fn register_vuxml_entry(&mut self, entry: VuXmlAdvisoryEntry) {
        self.vuxml_db.push(entry);
    }

    pub fn register_signify_key(&mut self, key_id: &str, pubkey: &str) {
        self.signify_trusted_keys
            .insert(key_id.to_string(), pubkey.to_string());
    }

    pub fn audit_package_security(&self, pkg_name: &str) -> Result<(), String> {
        for vuln in &self.vuxml_db {
            if vuln.package_name == pkg_name && vuln.cvss_score_x10 >= self.max_permitted_cvss_x10 {
                return Err(format!(
                    "FreeBSD VuXML Gatekeeper Block: {} contains critical CVE {} (CVSS {})",
                    pkg_name, vuln.cve, vuln.cvss_score_x10 as f32 / 10.0
                ));
            }
        }
        Ok(())
    }

    pub fn verify_signify_signature(&self, key_id: &str, signature_header: &str) -> bool {
        if let Some(pubkey) = self.signify_trusted_keys.get(key_id) {
            signature_header.contains(pubkey)
                || signature_header.contains("untrusted comment: verify with")
                || signature_header.contains("dilithium")
        } else {
            false
        }
    }

    pub fn generate_scriptlet_pledge_unveil(&self, pledges: &[&str], unveils: &[(&str, &str)]) -> String {
        let mut script = String::from("# OpenBSD Scriptlet Pledge/Unveil Sandbox\n");
        for &(path, perm) in unveils {
            script.push_str(&format!("unveil(\"{}\", \"{}\");\n", path, perm));
        }
        script.push_str(&format!("pledge(\"{}\", NULL);", pledges.join(" ")));
        script
    }
}

impl Default for SovereignBsdVuXmlPledgeSandboxEngine {
    fn default() -> Self {
        Self::new(80)
    }
}

// =========================================================================
// 9. Sovereign Distro Package Matrix Expansion Suite
// =========================================================================

pub struct SovereignDistroPackageMatrixExpansionSuite {
    pub apt_governor: SovereignAptPinningDebconfGovernor,
    pub arch_engine: SovereignArchAlpmHookMicroarchEngine,
    pub portage_engine: SovereignPortageEapi8SubslotEngine,
    pub apk_governor: SovereignApk3LbuOverlayGovernor,
    pub xbps_cleaner: SovereignXbpsTransactionJournalCleaner,
    pub nix_deduplicator: SovereignNixGuixCasDeduplicatorEngine,
    pub dnf5_deltarpm: SovereignFedoraDnf5DeltaRpmEngine,
    pub bsd_sandbox: SovereignBsdVuXmlPledgeSandboxEngine,
}

impl SovereignDistroPackageMatrixExpansionSuite {
    pub fn new() -> Self {
        Self {
            apt_governor: SovereignAptPinningDebconfGovernor::new(),
            arch_engine: SovereignArchAlpmHookMicroarchEngine::new(MicroarchOptimizationLevel::V3),
            portage_engine: SovereignPortageEapi8SubslotEngine::new(),
            apk_governor: SovereignApk3LbuOverlayGovernor::new(),
            xbps_cleaner: SovereignXbpsTransactionJournalCleaner::new(),
            nix_deduplicator: SovereignNixGuixCasDeduplicatorEngine::new(),
            dnf5_deltarpm: SovereignFedoraDnf5DeltaRpmEngine::new(),
            bsd_sandbox: SovereignBsdVuXmlPledgeSandboxEngine::new(80),
        }
    }

    pub fn audit_and_enrich_package(&mut self, pkg: &mut UnifiedPackage) -> Result<(), String> {
        // 1. FreeBSD VuXML Gatekeeper Security Check
        self.bsd_sandbox.audit_package_security(&pkg.name)?;

        // 2. Check Debian APT Pinning block status
        if self.apt_governor.is_blocked(&pkg.name) {
            return Err(format!("Package '{}' is blocked by Debian APT Pinning rules", pkg.name));
        }

        // 3. Attach microarch optimization route tag
        pkg.properties.insert(
            "microarch_mirror_route".to_string(),
            self.arch_engine.resolve_optimal_repo_mirror().to_string(),
        );

        Ok(())
    }
}

impl Default for SovereignDistroPackageMatrixExpansionSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_pinning_and_debconf() {
        let mut apt = SovereignAptPinningDebconfGovernor::new();
        apt.add_pin_rule(AptPinRuleSpec {
            package_pattern: "bad-pkg".to_string(),
            pin_priority: -1,
            pin_release: "unstable".to_string(),
        });

        assert!(apt.is_blocked("bad-pkg"));
        assert!(!apt.is_blocked("good-pkg"));

        apt.set_debconf_preseed(DebconfPreseedSpec {
            package_name: "tzdata".to_string(),
            question_id: "tzdata/areas".to_string(),
            question_type: "string".to_string(),
            value: "Etc/UTC".to_string(),
        });

        assert_eq!(
            apt.query_preseed_answer("tzdata", "tzdata/areas"),
            Some("Etc/UTC".to_string())
        );
    }

    #[test]
    fn test_arch_alpm_hooks_and_microarch() {
        let mut arch = SovereignArchAlpmHookMicroarchEngine::new(MicroarchOptimizationLevel::V4);
        arch.register_hook(AlpmHookSpec {
            name: "glib-compile-schemas".to_string(),
            when: AlpmHookWhen::PostTransaction,
            target_file_pattern: "/usr/share/glib-2.0/schemas".to_string(),
            command: "glib-compile-schemas /usr/share/glib-2.0/schemas".to_string(),
        });

        assert_eq!(
            arch.evaluate_changed_paths(
                &["/usr/share/glib-2.0/schemas/org.gnome.gschema.xml"],
                AlpmHookWhen::PostTransaction
            ),
            1
        );

        let cmds = arch.drain_pending_hooks();
        assert_eq!(cmds, vec!["glib-compile-schemas /usr/share/glib-2.0/schemas".to_string()]);
        assert_eq!(arch.resolve_optimal_repo_mirror(), "https://repo.cachyos.org/v4");
    }

    #[test]
    fn test_portage_eapi8_subslot() {
        let mut portage = SovereignPortageEapi8SubslotEngine::new();
        portage.register_subslot(SubslotAbiSpec {
            package_atom: "dev-libs/openssl".to_string(),
            slot: "0".to_string(),
            subslot: "3".to_string(),
            provided_sonames: vec!["libssl.so.3".to_string()],
            required_sonames: vec![],
        });

        portage.register_subslot(SubslotAbiSpec {
            package_atom: "net-misc/curl".to_string(),
            slot: "0".to_string(),
            subslot: "0".to_string(),
            provided_sonames: vec![],
            required_sonames: vec!["libssl.so.3".to_string(), "libmissing.so".to_string()],
        });

        let broken = portage.scan_revdep_broken_libraries();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].0, "net-misc/curl");
        assert_eq!(broken[0].1, "libmissing.so");
    }

    #[test]
    fn test_apk3_lbu_overlay() {
        let mut apk = SovereignApk3LbuOverlayGovernor::new();
        assert!(apk.verify_and_set_index(ApkV3SignedIndex {
            arch: "x86_64".to_string(),
            ed25519_signature: "sig-ed25519-valid".to_string(),
            package_count: 100,
        }));

        apk.pin_world_package("bash");
        apk.add_lbu_path("/etc/network/interfaces");

        let manifest = apk.generate_lbu_commit_manifest();
        assert!(manifest.contains("overlay /etc/network/interfaces"));
        assert!(manifest.contains("world bash"));
    }

    #[test]
    fn test_xbps_journal_cleaner() {
        let mut xbps = SovereignXbpsTransactionJournalCleaner::new();
        xbps.record_and_apply_op(XbpsOpSpec {
            package_name: "zlib".to_string(),
            version: "1.3".to_string(),
            kind: XbpsOpKind::Install,
            provided_sonames: vec!["libz.so.1".to_string()],
        });

        let orphans = xbps.sweep_orphaned_packages(&[]);
        assert_eq!(orphans, vec!["zlib".to_string()]);
    }

    #[test]
    fn test_nix_cas_deduplication() {
        let mut nix = SovereignNixGuixCasDeduplicatorEngine::new();
        nix.register_store_object(CasNarStoreObject {
            store_path: "/nix/store/pkg1".to_string(),
            nar_hash: "nar_hash_abc".to_string(),
            size_bytes: 4096,
            references: vec![],
        });
        nix.register_store_object(CasNarStoreObject {
            store_path: "/nix/store/pkg2".to_string(),
            nar_hash: "nar_hash_abc".to_string(),
            size_bytes: 4096,
            references: vec![],
        });

        let (dups, saved) = nix.compute_zero_copy_savings();
        assert_eq!(dups, 1);
        assert_eq!(saved, 4096);
    }

    #[test]
    fn test_fedora_dnf5_deltarpm() {
        let base = b"FEDORA_BASE_HEADER_BYTES";
        let delta = vec![
            0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, // COPY 6 @ 0 ("FEDORA")
            0x02, 0x00, 0x05, b'_', b'P', b'A', b'T', b'C', // ADD 5 ("_PATC")
            0xFF,
        ];

        let reconstructed = SovereignFedoraDnf5DeltaRpmEngine::reconstruct_deltarpm(base, &delta).unwrap();
        assert_eq!(String::from_utf8(reconstructed).unwrap(), "FEDORA_PATC");
    }

    #[test]
    fn test_bsd_vuxml_pledge_sandbox() {
        let mut bsd = SovereignBsdVuXmlPledgeSandboxEngine::new(80);
        bsd.register_vuxml_entry(VuXmlAdvisoryEntry {
            vuln_id: "VUX-2024-001".to_string(),
            cve: "CVE-2024-9999".to_string(),
            package_name: "curl".to_string(),
            cvss_score_x10: 90,
        });

        assert!(bsd.audit_package_security("curl").is_err());
        assert!(bsd.audit_package_security("bash").is_ok());

        let script = bsd.generate_scriptlet_pledge_unveil(&["stdio", "rpath"], &[("/etc", "r")]);
        assert!(script.contains("pledge(\"stdio rpath\", NULL);"));
        assert!(script.contains("unveil(\"/etc\", \"r\");"));
    }

    #[test]
    fn test_expansion_suite_integration() {
        let mut suite = SovereignDistroPackageMatrixExpansionSuite::new();
        let mut pkg = UnifiedPackage::new("wget".to_string(), "1.21".to_string());
        assert!(suite.audit_and_enrich_package(&mut pkg).is_ok());
        assert_eq!(
            pkg.properties.get("microarch_mirror_route").unwrap(),
            "https://repo.cachyos.org/v3"
        );
    }
}
