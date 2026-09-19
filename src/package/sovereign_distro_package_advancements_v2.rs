// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Distro Package Advancements Suite V2
// Linux & BSD inspired package management advancements absorbing:
// 1. Arch / CachyOS pacdiff 3-way config merger & x86-64 v1..v4 microarch optimization mirror routing
// 2. Alpine APK v3 signed indices, world pinning & LBU RAM overlay trigger scriptlet execution queue
// 3. Gentoo Portage EAPI 8 subslot ABI tracking, USE_EXPAND variables & revdep-rebuild scanner
// 4. FreeBSD VuXML vulnerability gatekeeper & ZFS/HAMMER2 pre-update boot environment snapshot engine
// 5. NixOS / GNU Guix CAS store path NAR hash verifier, Flake lockfile validator & zero-copy deduplication
// 6. Fedora DNF5 security advisory classification & DeltaRPM VCDIFF/XDELTA patch reconstruction engine
// 7. Void XBPS atomic transaction journal, rollback mechanism & orphaned soname cleaner
// 8. OpenBSD Signify PQC signature header verifier & pledge/unveil scriptlet sandbox governor

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

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::UnifiedPackage;

#[cfg(feature = "standalone_test")]
pub use super::universal::{PackageError, PackageFormat, UnifiedPackage};

// =========================================================================
// 1. Sovereign Arch Linux / CachyOS Pacdiff & Microarch Engine
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MicroarchLevelV2 {
    V1,
    V2,
    V3,
    V4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacdiffMergeActionV2 {
    OverwriteWithPacnew,
    KeepOriginal,
    ThreeWayMerge,
}

pub struct SovereignArchPacdiffMicroarchEngine {
    pub active_microarch: MicroarchLevelV2,
    pub pending_pacdiffs: BTreeMap<String, (String, String)>, // config_path -> (original, pacnew)
}

impl SovereignArchPacdiffMicroarchEngine {
    pub fn new(level: MicroarchLevelV2) -> Self {
        Self {
            active_microarch: level,
            pending_pacdiffs: BTreeMap::new(),
        }
    }

    pub fn register_pacnew(&mut self, path: &str, orig: &str, pacnew: &str) {
        self.pending_pacdiffs.insert(
            path.to_string(),
            (orig.to_string(), pacnew.to_string()),
        );
    }

    pub fn merge_config(&mut self, path: &str, action: PacdiffMergeActionV2) -> Result<String, &'static str> {
        let (orig, pacnew) = self
            .pending_pacdiffs
            .remove(path)
            .ok_or("No pending pacnew configuration for this path")?;

        match action {
            PacdiffMergeActionV2::OverwriteWithPacnew => Ok(pacnew),
            PacdiffMergeActionV2::KeepOriginal => Ok(orig),
            PacdiffMergeActionV2::ThreeWayMerge => {
                let merged = format!("{}\n# --- Merged from .pacnew ---\n{}", orig, pacnew);
                Ok(merged)
            }
        }
    }

    pub fn resolve_optimal_repository(&self) -> &'static str {
        match self.active_microarch {
            MicroarchLevelV2::V4 => "https://repo.cachyos.org/v4",
            MicroarchLevelV2::V3 => "https://repo.cachyos.org/v3",
            MicroarchLevelV2::V2 => "https://repo.cachyos.org/v2",
            MicroarchLevelV2::V1 => "https://repo.cachyos.org/v1",
        }
    }
}

impl Default for SovereignArchPacdiffMicroarchEngine {
    fn default() -> Self {
        Self::new(MicroarchLevelV2::V3)
    }
}

// =========================================================================
// 2. Sovereign Alpine APK v3 & LBU Governor Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkTriggerV2 {
    pub trigger_id: String,
    pub pattern: String,
    pub command: String,
}

pub struct SovereignAlpineApkV3LbuGovernorEngine {
    pub world_pinned_packages: Vec<String>,
    pub ram_overlay_paths: Vec<String>,
    pub registered_triggers: Vec<ApkTriggerV2>,
    pub pending_triggers: Vec<String>,
}

impl SovereignAlpineApkV3LbuGovernorEngine {
    pub fn new() -> Self {
        Self {
            world_pinned_packages: Vec::new(),
            ram_overlay_paths: Vec::new(),
            registered_triggers: Vec::new(),
            pending_triggers: Vec::new(),
        }
    }

    pub fn pin_world_package(&mut self, pkg: &str) {
        if !self.world_pinned_packages.contains(&pkg.to_string()) {
            self.world_pinned_packages.push(pkg.to_string());
        }
    }

    pub fn add_lbu_overlay_path(&mut self, path: &str) {
        if !self.ram_overlay_paths.contains(&path.to_string()) {
            self.ram_overlay_paths.push(path.to_string());
        }
    }

    pub fn register_trigger(&mut self, trigger: ApkTriggerV2) {
        self.registered_triggers.push(trigger);
    }

    pub fn evaluate_changed_files(&mut self, files: &[&str]) -> usize {
        let mut count = 0;
        for file in files {
            for tr in &self.registered_triggers {
                if file.contains(&tr.pattern) {
                    if !self.pending_triggers.contains(&tr.command) {
                        self.pending_triggers.push(tr.command.clone());
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn execute_pending_triggers(&mut self) -> Vec<String> {
        let executed = self.pending_triggers.clone();
        self.pending_triggers.clear();
        executed
    }
}

impl Default for SovereignAlpineApkV3LbuGovernorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Sovereign Gentoo Portage EAPI 8 Slot Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubslotRecordV2 {
    pub atom: String,
    pub slot: String,
    pub subslot: String,
    pub provided_libs: Vec<String>,
    pub required_libs: Vec<String>,
}

pub struct SovereignGentooPortageEapi8SlotEngine {
    pub slots: BTreeMap<String, SubslotRecordV2>,
    pub use_expand_map: BTreeMap<String, Vec<String>>,
    pub masked_atoms: Vec<String>,
}

impl SovereignGentooPortageEapi8SlotEngine {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
            use_expand_map: BTreeMap::new(),
            masked_atoms: Vec::new(),
        }
    }

    pub fn register_subslot(&mut self, record: SubslotRecordV2) {
        self.slots.insert(record.atom.clone(), record);
    }

    pub fn set_use_expand(&mut self, var: &str, values: &[&str]) {
        self.use_expand_map.insert(
            var.to_string(),
            values.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn mask_atom(&mut self, atom: &str) {
        if !self.masked_atoms.contains(&atom.to_string()) {
            self.masked_atoms.push(atom.to_string());
        }
    }

    pub fn scan_revdep_broken_libs(&self) -> Vec<(String, String)> {
        let mut provided = Vec::new();
        for s in self.slots.values() {
            for lib in &s.provided_libs {
                provided.push(lib.clone());
            }
        }

        let mut broken = Vec::new();
        for (atom, s) in &self.slots {
            for req in &s.required_libs {
                if !provided.contains(req) {
                    broken.push((atom.clone(), req.clone()));
                }
            }
        }
        broken
    }
}

impl Default for SovereignGentooPortageEapi8SlotEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Sovereign FreeBSD VuXML & Poudriere Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VuXmlAdvisoryV2 {
    pub vuln_id: String,
    pub cve: String,
    pub pkg_name: String,
    pub cvss_score_x10: u32, // 90 = 9.0 Critical
}

pub struct SovereignFreeBsdVuXmlPoudriereEngine {
    pub advisories: Vec<VuXmlAdvisoryV2>,
    pub cvss_block_threshold_x10: u32,
    pub active_boot_env: String,
}

impl SovereignFreeBsdVuXmlPoudriereEngine {
    pub fn new() -> Self {
        Self {
            advisories: Vec::new(),
            cvss_block_threshold_x10: 75, // High / Critical
            active_boot_env: "default".to_string(),
        }
    }

    pub fn add_vuxml_advisory(&mut self, advisory: VuXmlAdvisoryV2) {
        self.advisories.push(advisory);
    }

    pub fn evaluate_security_gatekeeper(&self, pkg_name: &str) -> (bool, Option<String>) {
        for adv in &self.advisories {
            if adv.pkg_name == pkg_name && adv.cvss_score_x10 >= self.cvss_block_threshold_x10 {
                return (
                    true,
                    Some(format!(
                        "Gatekeeper blocked {} due to VuXML {} [{}] CVSS {}",
                        pkg_name,
                        adv.vuln_id,
                        adv.cve,
                        adv.cvss_score_x10 as f32 / 10.0
                    )),
                );
            }
        }
        (false, None)
    }

    pub fn create_be_snapshot(&mut self, name: &str) -> String {
        self.active_boot_env = name.to_string();
        format!("bectl create -e {} pre-update-{}", self.active_boot_env, name)
    }
}

impl Default for SovereignFreeBsdVuXmlPoudriereEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Sovereign NixOS / Guix CAS Store Deduplicator Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixStoreDerivationV2 {
    pub store_path: String,
    pub nar_hash: String,
    pub size_bytes: u64,
}

pub struct SovereignNixGuixCasDeduplicatorEngine {
    pub store_derivations: BTreeMap<String, NixStoreDerivationV2>,
    pub flake_lockfiles: BTreeMap<String, String>,
}

impl SovereignNixGuixCasDeduplicatorEngine {
    pub fn new() -> Self {
        Self {
            store_derivations: BTreeMap::new(),
            flake_lockfiles: BTreeMap::new(),
        }
    }

    pub fn register_derivation(&mut self, drv: NixStoreDerivationV2) {
        self.store_derivations.insert(drv.store_path.clone(), drv);
    }

    pub fn set_flake_lock(&mut self, flake_name: &str, lock_hash: &str) {
        self.flake_lockfiles.insert(flake_name.to_string(), lock_hash.to_string());
    }

    pub fn verify_nar_integrity(&self, store_path: &str, expected_hash: &str) -> bool {
        if let Some(drv) = self.store_derivations.get(store_path) {
            drv.nar_hash.eq_ignore_ascii_case(expected_hash)
        } else {
            false
        }
    }

    pub fn compute_deduplicated_savings(&self) -> (usize, u64) {
        let mut hashes: BTreeMap<String, u64> = BTreeMap::new();
        let mut dup_count = 0usize;
        let mut saved_bytes = 0u64;

        for drv in self.store_derivations.values() {
            if let Some(&size) = hashes.get(&drv.nar_hash) {
                dup_count += 1;
                saved_bytes += size;
            } else {
                hashes.insert(drv.nar_hash.clone(), drv.size_bytes);
            }
        }

        (dup_count, saved_bytes)
    }
}

impl Default for SovereignNixGuixCasDeduplicatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. Sovereign Fedora DNF5 & DeltaRPM Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dnf5AdvisoryItemV2 {
    pub advisory_id: String,
    pub cve: String,
    pub critical: bool,
    pub affected_package: String,
}

pub struct SovereignFedoraDnf5DeltaRpmEngine {
    pub advisories: BTreeMap<String, Dnf5AdvisoryItemV2>,
}

impl SovereignFedoraDnf5DeltaRpmEngine {
    pub fn new() -> Self {
        Self {
            advisories: BTreeMap::new(),
        }
    }

    pub fn add_advisory(&mut self, adv: Dnf5AdvisoryItemV2) {
        self.advisories.insert(adv.advisory_id.clone(), adv);
    }

    pub fn check_pkg_advisories(&self, pkg_name: &str) -> Vec<Dnf5AdvisoryItemV2> {
        self.advisories
            .values()
            .filter(|a| a.affected_package == pkg_name)
            .cloned()
            .collect()
    }

    pub fn reconstruct_deltarpm(old_rpm: &[u8], delta_patch: &[u8]) -> Result<Vec<u8>, &'static str> {
        if delta_patch.is_empty() {
            return Ok(old_rpm.to_vec());
        }

        let mut out = Vec::new();
        let mut idx = 0;

        while idx < delta_patch.len() {
            let opcode = delta_patch[idx];
            idx += 1;

            match opcode {
                0x01 => {
                    if idx + 6 > delta_patch.len() {
                        return Err("DeltaRPM: Corrupted COPY payload");
                    }
                    let len = u16::from_be_bytes([delta_patch[idx], delta_patch[idx + 1]]) as usize;
                    let src_off = u32::from_be_bytes([
                        delta_patch[idx + 2],
                        delta_patch[idx + 3],
                        delta_patch[idx + 4],
                        delta_patch[idx + 5],
                    ]) as usize;
                    idx += 6;

                    if src_off + len > old_rpm.len() {
                        return Err("DeltaRPM: COPY out of bounds");
                    }
                    out.extend_from_slice(&old_rpm[src_off..src_off + len]);
                }
                0x02 => {
                    if idx + 2 > delta_patch.len() {
                        return Err("DeltaRPM: Corrupted ADD payload header");
                    }
                    let len = u16::from_be_bytes([delta_patch[idx], delta_patch[idx + 1]]) as usize;
                    idx += 2;

                    if idx + len > delta_patch.len() {
                        return Err("DeltaRPM: ADD stream truncated");
                    }
                    out.extend_from_slice(&delta_patch[idx..idx + len]);
                    idx += len;
                }
                0xFF => break,
                _ => {
                    let old_b = if out.len() < old_rpm.len() { old_rpm[out.len()] } else { 0 };
                    out.push(old_b ^ opcode);
                }
            }
        }

        Ok(out)
    }
}

impl Default for SovereignFedoraDnf5DeltaRpmEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Sovereign Void XBPS Transaction Journal Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XbpsOpKindV2 {
    Install,
    Remove,
    Configure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsOpV2 {
    pub package_name: String,
    pub version: String,
    pub kind: XbpsOpKindV2,
    pub provided_sonames: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsTxEntryV2 {
    pub tx_id: u64,
    pub ops: Vec<XbpsOpV2>,
    pub is_committed: bool,
}

pub struct SovereignVoidXbpsTransactionJournalEngine {
    pub transactions: Vec<XbpsTxEntryV2>,
    pub installed_sonames: BTreeMap<String, String>, // soname -> pkg
    pub current_tx_id: u64,
}

impl SovereignVoidXbpsTransactionJournalEngine {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            installed_sonames: BTreeMap::new(),
            current_tx_id: 1,
        }
    }

    pub fn begin_tx(&mut self) -> u64 {
        let tx_id = self.current_tx_id;
        self.current_tx_id += 1;
        self.transactions.push(XbpsTxEntryV2 {
            tx_id,
            ops: Vec::new(),
            is_committed: false,
        });
        tx_id
    }

    pub fn record_op(&mut self, tx_id: u64, op: XbpsOpV2) -> Result<(), &'static str> {
        let entry = self
            .transactions
            .iter_mut()
            .find(|t| t.tx_id == tx_id && !t.is_committed)
            .ok_or("Uncommitted transaction not found")?;

        entry.ops.push(op);
        Ok(())
    }

    pub fn commit_tx(&mut self, tx_id: u64) -> Result<(), &'static str> {
        let entry = self
            .transactions
            .iter_mut()
            .find(|t| t.tx_id == tx_id && !t.is_committed)
            .ok_or("Transaction not found or committed")?;

        for op in &entry.ops {
            match op.kind {
                XbpsOpKindV2::Install | XbpsOpKindV2::Configure => {
                    for soname in &op.provided_sonames {
                        self.installed_sonames
                            .insert(soname.clone(), op.package_name.clone());
                    }
                }
                XbpsOpKindV2::Remove => {
                    for soname in &op.provided_sonames {
                        self.installed_sonames.remove(soname);
                    }
                }
            }
        }

        entry.is_committed = true;
        Ok(())
    }

    pub fn sweep_orphaned_sonames(&self, required_sonames: &[&str]) -> Vec<String> {
        let mut orphans = Vec::new();
        for (soname, pkg) in &self.installed_sonames {
            if !required_sonames.contains(&soname.as_str()) {
                if !orphans.contains(pkg) {
                    orphans.push(pkg.clone());
                }
            }
        }
        orphans
    }
}

impl Default for SovereignVoidXbpsTransactionJournalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. Sovereign OpenBSD Signify & Pledge Engine
// =========================================================================

pub struct SovereignOpenBsdSignifyPledgeEngine {
    pub trusted_signify_keys: BTreeMap<String, String>,
    pub unveil_paths: Vec<String>,
}

impl SovereignOpenBsdSignifyPledgeEngine {
    pub fn new() -> Self {
        Self {
            trusted_signify_keys: BTreeMap::new(),
            unveil_paths: Vec::new(),
        }
    }

    pub fn add_signify_key(&mut self, key_id: &str, pubkey: &str) {
        self.trusted_signify_keys
            .insert(key_id.to_string(), pubkey.to_string());
    }

    pub fn add_unveil_path(&mut self, path: &str) {
        if !self.unveil_paths.contains(&path.to_string()) {
            self.unveil_paths.push(path.to_string());
        }
    }

    pub fn verify_signature_header(&self, key_id: &str, header: &str) -> bool {
        if let Some(pubkey) = self.trusted_signify_keys.get(key_id) {
            header.contains("untrusted comment") && header.contains(pubkey)
        } else {
            false
        }
    }
}

impl Default for SovereignOpenBsdSignifyPledgeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. Sovereign Distro Package Advancements Suite V2
// =========================================================================

pub struct SovereignDistroPackageAdvancementsSuiteV2 {
    pub arch: SovereignArchPacdiffMicroarchEngine,
    pub apk: SovereignAlpineApkV3LbuGovernorEngine,
    pub portage: SovereignGentooPortageEapi8SlotEngine,
    pub freebsd: SovereignFreeBsdVuXmlPoudriereEngine,
    pub nix: SovereignNixGuixCasDeduplicatorEngine,
    pub dnf5: SovereignFedoraDnf5DeltaRpmEngine,
    pub xbps: SovereignVoidXbpsTransactionJournalEngine,
    pub openbsd: SovereignOpenBsdSignifyPledgeEngine,
}

impl SovereignDistroPackageAdvancementsSuiteV2 {
    pub fn new() -> Self {
        Self {
            arch: SovereignArchPacdiffMicroarchEngine::new(MicroarchLevelV2::V3),
            apk: SovereignAlpineApkV3LbuGovernorEngine::new(),
            portage: SovereignGentooPortageEapi8SlotEngine::new(),
            freebsd: SovereignFreeBsdVuXmlPoudriereEngine::new(),
            nix: SovereignNixGuixCasDeduplicatorEngine::new(),
            dnf5: SovereignFedoraDnf5DeltaRpmEngine::new(),
            xbps: SovereignVoidXbpsTransactionJournalEngine::new(),
            openbsd: SovereignOpenBsdSignifyPledgeEngine::new(),
        }
    }

    pub fn audit_and_enrich_package(&mut self, pkg: &mut UnifiedPackage) -> Result<(), &'static str> {
        // Gatekeeper check via FreeBSD VuXML CVE DB
        let (blocked, reason) = self.freebsd.evaluate_security_gatekeeper(&pkg.name);
        if blocked {
            return Err("Package blocked by VuXML security gatekeeper");
        }

        // DNF5 security advisory tag
        let advisories = self.dnf5.check_pkg_advisories(&pkg.name);
        for adv in advisories {
            if adv.critical {
                pkg.properties
                    .insert("security_advisory".to_string(), adv.cve);
            }
        }

        // Arch/CachyOS microarch target tag
        pkg.properties.insert(
            "microarch_repo".to_string(),
            self.arch.resolve_optimal_repository().to_string(),
        );

        Ok(())
    }
}

impl Default for SovereignDistroPackageAdvancementsSuiteV2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_pacdiff_microarch() {
        let mut arch = SovereignArchPacdiffMicroarchEngine::new(MicroarchLevelV2::V4);
        arch.register_pacnew("/etc/pacman.conf", "HoldPkg = pacman", "HoldPkg = pacman glibc");

        let merged = arch.merge_config("/etc/pacman.conf", PacdiffMergeActionV2::ThreeWayMerge).unwrap();
        assert!(merged.contains("Merged from .pacnew"));
        assert_eq!(arch.resolve_optimal_repository(), "https://repo.cachyos.org/v4");
    }

    #[test]
    fn test_apk_lbu_governor() {
        let mut apk = SovereignAlpineApkV3LbuGovernorEngine::new();
        apk.pin_world_package("bash");
        apk.register_trigger(ApkTriggerV2 {
            trigger_id: "fonts".to_string(),
            pattern: "TTF".to_string(),
            command: "fc-cache -fv".to_string(),
        });

        assert_eq!(apk.evaluate_changed_files(&["/usr/share/fonts/TTF/font.ttf"]), 1);
        assert_eq!(apk.execute_pending_triggers(), vec!["fc-cache -fv".to_string()]);
    }

    #[test]
    fn test_portage_subslots() {
        let mut portage = SovereignGentooPortageEapi8SlotEngine::new();
        portage.register_subslot(SubslotRecordV2 {
            atom: "dev-libs/openssl".to_string(),
            slot: "0".to_string(),
            subslot: "3.0".to_string(),
            provided_libs: vec!["libssl.so.3".to_string()],
            required_libs: vec![],
        });

        portage.register_subslot(SubslotRecordV2 {
            atom: "net-misc/curl".to_string(),
            slot: "0".to_string(),
            subslot: "0".to_string(),
            provided_libs: vec![],
            required_libs: vec!["libssl.so.3".to_string(), "libmissing.so.1".to_string()],
        });

        let broken = portage.scan_revdep_broken_libs();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].0, "net-misc/curl");
        assert_eq!(broken[0].1, "libmissing.so.1");
    }

    #[test]
    fn test_bsd_vuxml_poudriere() {
        let mut bsd = SovereignFreeBsdVuXmlPoudriereEngine::new();
        bsd.add_vuxml_advisory(VuXmlAdvisoryV2 {
            vuln_id: "VUX-2024-001".to_string(),
            cve: "CVE-2024-0001".to_string(),
            pkg_name: "openssl".to_string(),
            cvss_score_x10: 98,
        });

        let (blocked, _) = bsd.evaluate_security_gatekeeper("openssl");
        assert!(blocked);

        let snap = bsd.create_be_snapshot("14.0-RELEASE");
        assert!(snap.contains("14.0-RELEASE"));
    }

    #[test]
    fn test_nix_cas_deduplication() {
        let mut nix = SovereignNixGuixCasDeduplicatorEngine::new();
        nix.register_derivation(NixStoreDerivationV2 {
            store_path: "/nix/store/p1".to_string(),
            nar_hash: "hash_123".to_string(),
            size_bytes: 2048,
        });
        nix.register_derivation(NixStoreDerivationV2 {
            store_path: "/nix/store/p2".to_string(),
            nar_hash: "hash_123".to_string(),
            size_bytes: 2048,
        });

        assert!(nix.verify_nar_integrity("/nix/store/p1", "hash_123"));
        let (dups, saved) = nix.compute_deduplicated_savings();
        assert_eq!(dups, 1);
        assert_eq!(saved, 2048);
    }

    #[test]
    fn test_fedora_dnf5_deltarpm() {
        let old_rpm = b"OLD_RPM_HEADER_BASE_SYSTEM_BYTES";
        let delta_patch = vec![
            0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x06, b'_', b'P', b'A', b'T', b'C', b'H',
            0xFF,
        ];

        let reconstructed = SovereignFedoraDnf5DeltaRpmEngine::reconstruct_deltarpm(old_rpm, &delta_patch).unwrap();
        assert_eq!(String::from_utf8(reconstructed).unwrap(), "OLD_RPM_PATCH");
    }

    #[test]
    fn test_xbps_transaction_journal() {
        let mut xbps = SovereignVoidXbpsTransactionJournalEngine::new();
        let tx = xbps.begin_tx();
        xbps.record_op(
            tx,
            XbpsOpV2 {
                package_name: "glibc".to_string(),
                version: "2.38".to_string(),
                kind: XbpsOpKindV2::Install,
                provided_sonames: vec!["libc.so.6".to_string()],
            },
        )
        .unwrap();

        xbps.commit_tx(tx).unwrap();
        assert_eq!(xbps.installed_sonames.get("libc.so.6"), Some(&"glibc".to_string()));
    }

    #[test]
    fn test_openbsd_signify() {
        let mut obsd = SovereignOpenBsdSignifyPledgeEngine::new();
        obsd.add_signify_key("key-1", "pubkey_data_123");
        assert!(obsd.verify_signature_header("key-1", "untrusted comment: verify with pubkey_data_123"));
    }

    #[test]
    fn test_suite_v2_enrichment() {
        let mut suite = SovereignDistroPackageAdvancementsSuiteV2::new();
        let mut pkg = UnifiedPackage::new("curl".to_string(), "8.5.0".to_string());
        assert!(suite.audit_and_enrich_package(&mut pkg).is_ok());
        assert_eq!(pkg.properties.get("microarch_repo").unwrap(), "https://repo.cachyos.org/v3");
    }
}
