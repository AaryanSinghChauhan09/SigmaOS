use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// 1. Debian / Ubuntu netselect-apt Fast Mirror Latency & Throughput Ranker
#[derive(Debug, Clone)]
pub struct AptMirrorRecord {
    pub mirror_url: String,
    pub latency_ms: u32,
    pub bandwidth_kbps: u32,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct DebianAptFastMirrorRanker {
    pub candidate_mirrors: Vec<AptMirrorRecord>,
}

impl DebianAptFastMirrorRanker {
    pub fn new() -> Self {
        Self {
            candidate_mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u32, bandwidth_kbps: u32) {
        let score = (bandwidth_kbps as f32) / (latency_ms as f32 + 1.0);
        self.candidate_mirrors.push(AptMirrorRecord {
            mirror_url: url.to_string(),
            latency_ms,
            bandwidth_kbps,
            score,
        });
    }

    pub fn rank_best_mirrors(&mut self) -> Vec<AptMirrorRecord> {
        self.candidate_mirrors
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        self.candidate_mirrors.clone()
    }
}

/// 2. Arch Linux pacman xdelta Package Patch Reconstruction Engine
#[derive(Debug, Clone)]
pub struct PacmanDeltaPatch {
    pub pkgname: String,
    pub old_ver: String,
    pub new_ver: String,
    pub patch_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ArchPacmanDeltaSyncEngine;

impl ArchPacmanDeltaSyncEngine {
    /// Creates a delta patch between old and new package binaries
    pub fn create_delta_patch(
        pkgname: &str,
        old_ver: &str,
        new_ver: &str,
        old_bytes: &[u8],
        new_bytes: &[u8],
    ) -> PacmanDeltaPatch {
        let diff: Vec<u8> = new_bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ old_bytes.get(i).copied().unwrap_or(0))
            .collect();

        PacmanDeltaPatch {
            pkgname: pkgname.to_string(),
            old_ver: old_ver.to_string(),
            new_ver: new_ver.to_string(),
            patch_bytes: diff,
        }
    }

    /// Reconstructs new package binary from old binary and delta patch
    pub fn apply_delta_patch(old_bytes: &[u8], patch: &PacmanDeltaPatch) -> Vec<u8> {
        patch
            .patch_bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ old_bytes.get(i).copied().unwrap_or(0))
            .collect()
    }
}

/// 3. Fedora DNF5 Package Group Comps Solver (`dnf groupinstall` parity)
#[derive(Debug, Clone)]
pub struct CompsGroup {
    pub group_id: String,
    pub name: String,
    pub mandatory_packages: Vec<String>,
    pub default_packages: Vec<String>,
    pub optional_packages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FedoraDnfGroupInstallSolver {
    pub groups: BTreeMap<String, CompsGroup>,
}

impl FedoraDnfGroupInstallSolver {
    pub fn new() -> Self {
        let mut groups = BTreeMap::new();
        groups.insert(
            "development-tools".to_string(),
            CompsGroup {
                group_id: "development-tools".to_string(),
                name: "Development Tools".to_string(),
                mandatory_packages: vec!["gcc".to_string(), "make".to_string(), "gdb".to_string()],
                default_packages: vec!["git".to_string(), "autoconf".to_string()],
                optional_packages: vec!["clang".to_string(), "ninja-build".to_string()],
            },
        );

        Self { groups }
    }

    pub fn resolve_group_packages(
        &self,
        group_id: &str,
        include_optional: bool,
    ) -> Result<Vec<String>, &'static str> {
        let group = self
            .groups
            .get(group_id)
            .ok_or("Comps group not found in DNF registry")?;

        let mut pkgs = group.mandatory_packages.clone();
        pkgs.extend(group.default_packages.clone());

        if include_optional {
            pkgs.extend(group.optional_packages.clone());
        }

        Ok(pkgs)
    }
}

/// 4. FreeBSD `pkg(8)` Base RootFS System Upgrade & Boot Environment Manager
#[derive(Debug, Clone)]
pub struct FreeBsdPkgBaseRootfsEngine {
    pub installed_base_version: String,
    pub active_boot_env: String,
    pub available_boot_envs: Vec<String>,
}

impl FreeBsdPkgBaseRootfsEngine {
    pub fn new(current_version: &str) -> Self {
        Self {
            installed_base_version: current_version.to_string(),
            active_boot_env: "default".to_string(),
            available_boot_envs: vec!["default".to_string()],
        }
    }

    /// Upgrades base system via pkg base and creates a new ZFS boot environment (`bectl create`)
    pub fn upgrade_pkg_base_with_be(
        &mut self,
        target_version: &str,
        be_name: &str,
    ) -> Result<String, &'static str> {
        if self.available_boot_envs.contains(&be_name.to_string()) {
            return Err("Boot environment name already exists");
        }

        self.available_boot_envs.push(be_name.to_string());
        self.installed_base_version = target_version.to_string();
        self.active_boot_env = be_name.to_string();

        Ok(format!(
            "Successfully upgraded FreeBSD base rootfs to {} inside new Boot Environment '{}'",
            target_version, be_name
        ))
    }
}

/// 5. Debian AppStream XML/YAML Desktop Application Catalog Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppStreamEntry {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub categories: Vec<String>,
    pub package_name: String,
    pub icon_url: String,
}

#[derive(Debug, Clone)]
pub struct DebianAppStreamCatalogEngine {
    pub entries: BTreeMap<String, AppStreamEntry>,
}

impl DebianAppStreamCatalogEngine {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn register_app(&mut self, app: AppStreamEntry) {
        self.entries.insert(app.id.clone(), app);
    }

    pub fn search_by_category(&self, category: &str) -> Vec<AppStreamEntry> {
        self.entries
            .values()
            .filter(|a| a.categories.iter().any(|c| c.eq_ignore_ascii_case(category)))
            .cloned()
            .collect()
    }

    pub fn search_by_keyword(&self, keyword: &str) -> Vec<AppStreamEntry> {
        let kw = keyword.to_lowercase();
        self.entries
            .values()
            .filter(|a| {
                a.name.to_lowercase().contains(&kw)
                    || a.summary.to_lowercase().contains(&kw)
                    || a.package_name.to_lowercase().contains(&kw)
            })
            .cloned()
            .collect()
    }
}

impl Default for DebianAppStreamCatalogEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 6. Arch Linux pacstrap Base System Bootstrapping & Chroot Seeder Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacstrapSeedTask {
    pub target_rootfs: String,
    pub base_packages: Vec<String>,
    pub copy_host_keyring: bool,
    pub copy_host_mirrorlist: bool,
}

#[derive(Debug, Clone)]
pub struct ArchPacstrapSeederEngine {
    pub active_seedings: Vec<PacstrapSeedTask>,
}

impl ArchPacstrapSeederEngine {
    pub fn new() -> Self {
        Self {
            active_seedings: Vec::new(),
        }
    }

    pub fn prepare_bootstrap_task(
        &mut self,
        target_rootfs: &str,
        packages: &[&str],
        copy_keyring: bool,
        copy_mirrorlist: bool,
    ) -> PacstrapSeedTask {
        let task = PacstrapSeedTask {
            target_rootfs: target_rootfs.to_string(),
            base_packages: packages.iter().map(|s| s.to_string()).collect(),
            copy_host_keyring: copy_keyring,
            copy_host_mirrorlist: copy_mirrorlist,
        };
        self.active_seedings.push(task.clone());
        task
    }

    pub fn generate_pacstrap_command(&self, task: &PacstrapSeedTask) -> String {
        let mut cmd = format!("pacstrap -K {}", task.target_rootfs);
        for pkg in &task.base_packages {
            cmd.push(' ');
            cmd.push_str(pkg);
        }
        cmd
    }
}

impl Default for ArchPacstrapSeederEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 7. Gentoo Portage eclean Distfile Scrubber & revdep-rebuild Scanner
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistfileEntry {
    pub filename: String,
    pub size_bytes: u64,
    pub is_referenced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedLibRequirement {
    pub binary_path: String,
    pub needed_soname: String,
    pub is_broken: bool,
}

#[derive(Debug, Clone)]
pub struct GentooEcleanRevdepRebuildEngine {
    pub distfiles: Vec<DistfileEntry>,
    pub lib_requirements: Vec<SharedLibRequirement>,
}

impl GentooEcleanRevdepRebuildEngine {
    pub fn new() -> Self {
        Self {
            distfiles: Vec::new(),
            lib_requirements: Vec::new(),
        }
    }

    pub fn add_distfile(&mut self, filename: &str, size_bytes: u64, is_referenced: bool) {
        self.distfiles.push(DistfileEntry {
            filename: filename.to_string(),
            size_bytes,
            is_referenced,
        });
    }

    pub fn add_lib_requirement(&mut self, binary_path: &str, needed_soname: &str, is_broken: bool) {
        self.lib_requirements.push(SharedLibRequirement {
            binary_path: binary_path.to_string(),
            needed_soname: needed_soname.to_string(),
            is_broken,
        });
    }

    pub fn eclean_distfiles(&mut self) -> (usize, u64) {
        let mut count = 0usize;
        let mut freed_bytes = 0u64;
        self.distfiles.retain(|f| {
            if !f.is_referenced {
                count += 1;
                freed_bytes += f.size_bytes;
                false
            } else {
                true
            }
        });
        (count, freed_bytes)
    }

    pub fn scan_revdep_rebuild_candidates(&self) -> Vec<String> {
        let mut broken_binaries = Vec::new();
        for req in &self.lib_requirements {
            if req.is_broken && !broken_binaries.contains(&req.binary_path) {
                broken_binaries.push(req.binary_path.clone());
            }
        }
        broken_binaries
    }
}

impl Default for GentooEcleanRevdepRebuildEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 8. Nix Store Path Hash Integrity Verification Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorePathIntegrity {
    pub store_path: String,
    pub expected_nar_hash: String,
    pub computed_nar_hash: String,
    pub is_valid: bool,
}

#[derive(Debug, Clone)]
pub struct NixStorePathVerifyEngine {
    pub records: BTreeMap<String, StorePathIntegrity>,
}

impl NixStorePathVerifyEngine {
    pub fn new() -> Self {
        Self {
            records: BTreeMap::new(),
        }
    }

    pub fn register_store_path(&mut self, path: &str, expected_hash: &str, computed_hash: &str) -> bool {
        let is_valid = expected_hash.eq_ignore_ascii_case(computed_hash);
        self.records.insert(
            path.to_string(),
            StorePathIntegrity {
                store_path: path.to_string(),
                expected_nar_hash: expected_hash.to_string(),
                computed_nar_hash: computed_hash.to_string(),
                is_valid,
            },
        );
        is_valid
    }

    pub fn find_corrupted_paths(&self) -> Vec<String> {
        self.records
            .values()
            .filter(|r| !r.is_valid)
            .map(|r| r.store_path.clone())
            .collect()
    }
}

impl Default for NixStorePathVerifyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 9. HardenedBSD ELF Binary Mitigation & Hardening Auditor Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfBinaryMitigationStatus {
    pub binary_path: String,
    pub has_pie_aslr: bool,
    pub has_pax_cfi: bool,
    pub has_relro_full: bool,
    pub has_stack_canary: bool,
    pub is_fully_hardened: bool,
}

#[derive(Debug, Clone)]
pub struct HardenedBsdBinaryMitigationAuditorEngine {
    pub audited_binaries: BTreeMap<String, ElfBinaryMitigationStatus>,
}

impl HardenedBsdBinaryMitigationAuditorEngine {
    pub fn new() -> Self {
        Self {
            audited_binaries: BTreeMap::new(),
        }
    }

    pub fn audit_binary(
        &mut self,
        binary_path: &str,
        has_pie: bool,
        has_cfi: bool,
        has_relro: bool,
        has_canary: bool,
    ) -> ElfBinaryMitigationStatus {
        let is_fully = has_pie && has_cfi && has_relro && has_canary;
        let status = ElfBinaryMitigationStatus {
            binary_path: binary_path.to_string(),
            has_pie_aslr: has_pie,
            has_pax_cfi: has_cfi,
            has_relro_full: has_relro,
            has_stack_canary: has_canary,
            is_fully_hardened: is_fully,
        };
        self.audited_binaries.insert(binary_path.to_string(), status.clone());
        status
    }

    pub fn find_unhardened_binaries(&self) -> Vec<String> {
        self.audited_binaries
            .values()
            .filter(|s| !s.is_fully_hardened)
            .map(|s| s.binary_path.clone())
            .collect()
    }
}

impl Default for HardenedBsdBinaryMitigationAuditorEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 10. OpenBSD pkg_add -u Upgrade & Shared Library Relinker Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageUpgradeRecord {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub changed_shared_libs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OpenBsdPkgUpgradeRelinkerEngine {
    pub upgrades: Vec<PackageUpgradeRecord>,
}

impl OpenBsdPkgUpgradeRelinkerEngine {
    pub fn new() -> Self {
        Self { upgrades: Vec::new() }
    }

    pub fn record_upgrade(
        &mut self,
        pkg_name: &str,
        old_ver: &str,
        new_ver: &str,
        changed_libs: &[&str],
    ) {
        self.upgrades.push(PackageUpgradeRecord {
            package_name: pkg_name.to_string(),
            old_version: old_ver.to_string(),
            new_version: new_ver.to_string(),
            changed_shared_libs: changed_libs.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn generate_relink_tasks(&self) -> Vec<String> {
        let mut relinks = Vec::new();
        for upg in &self.upgrades {
            if !upg.changed_shared_libs.is_empty() {
                relinks.push(format!(
                    "relink-libs --pkg {} --libs {}",
                    upg.package_name,
                    upg.changed_shared_libs.join(",")
                ));
            }
        }
        relinks
    }
}

impl Default for OpenBsdPkgUpgradeRelinkerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 11. Alpine Linux LBU Diskless Package Overlay Governor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LbuOverlayFile {
    pub filepath: String,
    pub is_modified: bool,
    pub checksum_sha256: String,
}

#[derive(Debug, Clone)]
pub struct AlpineLbuOverlayStateGovernor {
    pub overlay_files: BTreeMap<String, LbuOverlayFile>,
    pub media_target: String,
}

impl AlpineLbuOverlayStateGovernor {
    pub fn new(media_target: &str) -> Self {
        Self {
            overlay_files: BTreeMap::new(),
            media_target: media_target.to_string(),
        }
    }

    pub fn track_file(&mut self, filepath: &str, checksum: &str, is_modified: bool) {
        self.overlay_files.insert(
            filepath.to_string(),
            LbuOverlayFile {
                filepath: filepath.to_string(),
                is_modified,
                checksum_sha256: checksum.to_string(),
            },
        );
    }

    pub fn modified_files_count(&self) -> usize {
        self.overlay_files.values().filter(|f| f.is_modified).count()
    }

    pub fn generate_commit_apkovl_cmd(&self) -> String {
        format!("lbu commit -d {}", self.media_target)
    }
}

impl Default for AlpineLbuOverlayStateGovernor {
    fn default() -> Self {
        Self::new("sda1")
    }
}

/// 12. Fedora DNF GPG Subkey Rotation & Revocation Validator
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnfGpgSubkey {
    pub subkey_id: String,
    pub master_key_id: String,
    pub is_revoked: bool,
    pub expiry_timestamp_sec: u64,
}

#[derive(Debug, Clone)]
pub struct FedoraDnfGpgKeyRotationEngine {
    pub subkeys: BTreeMap<String, DnfGpgSubkey>,
}

impl FedoraDnfGpgKeyRotationEngine {
    pub fn new() -> Self {
        Self {
            subkeys: BTreeMap::new(),
        }
    }

    pub fn register_subkey(&mut self, subkey: DnfGpgSubkey) {
        self.subkeys.insert(subkey.subkey_id.clone(), subkey);
    }

    pub fn revoke_subkey(&mut self, subkey_id: &str) -> bool {
        if let Some(key) = self.subkeys.get_mut(subkey_id) {
            key.is_revoked = true;
            true
        } else {
            false
        }
    }

    pub fn is_subkey_valid(&self, subkey_id: &str, current_time_sec: u64) -> bool {
        if let Some(key) = self.subkeys.get(subkey_id) {
            if key.is_revoked {
                return false;
            }
            if key.expiry_timestamp_sec != 0 && current_time_sec > key.expiry_timestamp_sec {
                return false;
            }
            true
        } else {
            false
        }
    }
}

impl Default for FedoraDnfGpgKeyRotationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_package_innovations() {
        let mut netselect = DebianAptFastMirrorRanker::new();
        netselect.add_mirror("http://mirror.us.debian.org", 20, 100000);
        netselect.add_mirror("http://mirror.slow.com", 200, 5000);
        let ranked = netselect.rank_best_mirrors();
        assert_eq!(ranked[0].mirror_url, "http://mirror.us.debian.org");

        let old_pkg = b"glibc-2.37-binary-data-stream";
        let new_pkg = b"glibc-2.38-binary-data-stream";
        let patch = ArchPacmanDeltaSyncEngine::create_delta_patch("glibc", "2.37", "2.38", old_pkg, new_pkg);
        let reconstructed = ArchPacmanDeltaSyncEngine::apply_delta_patch(old_pkg, &patch);
        assert_eq!(&reconstructed[..], new_pkg);

        let dnf_solver = FedoraDnfGroupInstallSolver::new();
        let dev_pkgs = dnf_solver.resolve_group_packages("development-tools", false).unwrap();
        assert!(dev_pkgs.contains(&"gcc".to_string()));

        let mut pkg_base = FreeBsdPkgBaseRootfsEngine::new("14.0-RELEASE");
        let upgrade_msg = pkg_base.upgrade_pkg_base_with_be("14.1-RELEASE", "be_14_1").unwrap();
        assert!(upgrade_msg.contains("14.1-RELEASE"));
        assert_eq!(pkg_base.active_boot_env, "be_14_1");
    }

    #[test]
    fn test_debian_appstream_catalog_engine() {
        let mut catalog = DebianAppStreamCatalogEngine::new();
        catalog.register_app(AppStreamEntry {
            id: "org.gnome.Gedit".to_string(),
            name: "gedit".to_string(),
            summary: "Text Editor".to_string(),
            categories: vec!["Utility".to_string(), "TextEditor".to_string()],
            package_name: "gedit".to_string(),
            icon_url: "https://hub.appstream.org/icons/gedit.png".to_string(),
        });

        let found_cat = catalog.search_by_category("texteditor");
        assert_eq!(found_cat.len(), 1);
        assert_eq!(found_cat[0].name, "gedit");

        let found_kw = catalog.search_by_keyword("editor");
        assert_eq!(found_kw.len(), 1);
    }

    #[test]
    fn test_arch_pacstrap_seeder_engine() {
        let mut seeder = ArchPacstrapSeederEngine::new();
        let task = seeder.prepare_bootstrap_task("/mnt", &["base", "linux", "linux-firmware"], true, true);
        assert_eq!(task.base_packages.len(), 3);

        let cmd = seeder.generate_pacstrap_command(&task);
        assert_eq!(cmd, "pacstrap -K /mnt base linux linux-firmware");
    }

    #[test]
    fn test_gentoo_eclean_revdep_rebuild_engine() {
        let mut engine = GentooEcleanRevdepRebuildEngine::new();
        engine.add_distfile("glibc-2.38.tar.xz", 15_000_000, true);
        engine.add_distfile("old-gcc-11.2.tar.xz", 80_000_000, false);

        let (freed_count, freed_bytes) = engine.eclean_distfiles();
        assert_eq!(freed_count, 1);
        assert_eq!(freed_bytes, 80_000_000);

        engine.add_lib_requirement("/usr/bin/curl", "libssl.so.1.1", true);
        engine.add_lib_requirement("/usr/bin/git", "libssl.so.3", false);

        let broken = engine.scan_revdep_rebuild_candidates();
        assert_eq!(broken, vec!["/usr/bin/curl".to_string()]);
    }

    #[test]
    fn test_nix_store_path_verify_engine() {
        let mut verifier = NixStorePathVerifyEngine::new();
        assert!(verifier.register_store_path("/nix/store/hash1-pkg", "sha256-abc", "sha256-abc"));
        assert!(!verifier.register_store_path("/nix/store/hash2-pkg", "sha256-xyz", "sha256-bad"));

        let corrupted = verifier.find_corrupted_paths();
        assert_eq!(corrupted, vec!["/nix/store/hash2-pkg".to_string()]);
    }

    #[test]
    fn test_hardenedbsd_binary_mitigation_auditor_engine() {
        let mut auditor = HardenedBsdBinaryMitigationAuditorEngine::new();
        let status1 = auditor.audit_binary("/sbin/init", true, true, true, true);
        assert!(status1.is_fully_hardened);

        let status2 = auditor.audit_binary("/tmp/legacy_tool", false, false, true, false);
        assert!(!status2.is_fully_hardened);

        let unhardened = auditor.find_unhardened_binaries();
        assert_eq!(unhardened, vec!["/tmp/legacy_tool".to_string()]);
    }

    #[test]
    fn test_openbsd_pkg_upgrade_relinker_engine() {
        let mut relinker = OpenBsdPkgUpgradeRelinkerEngine::new();
        relinker.record_upgrade("png", "1.6.39", "1.6.40", &["libpng16.so.16.0"]);

        let tasks = relinker.generate_relink_tasks();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].contains("relink-libs --pkg png"));
    }

    #[test]
    fn test_alpine_lbu_overlay_state_governor() {
        let mut lbu = AlpineLbuOverlayStateGovernor::new("sda1");
        lbu.track_file("/etc/network/interfaces", "hash1", true);
        lbu.track_file("/etc/hostname", "hash2", false);

        assert_eq!(lbu.modified_files_count(), 1);
        assert_eq!(lbu.generate_commit_apkovl_cmd(), "lbu commit -d sda1");
    }

    #[test]
    fn test_fedora_dnf_gpg_key_rotation_engine() {
        let mut rotation = FedoraDnfGpgKeyRotationEngine::new();
        rotation.register_subkey(DnfGpgSubkey {
            subkey_id: "KEY_2024_SUB1".to_string(),
            master_key_id: "MASTER_FEDORA_KEY".to_string(),
            is_revoked: false,
            expiry_timestamp_sec: 2000000000,
        });

        assert!(rotation.is_subkey_valid("KEY_2024_SUB1", 1700000000));
        assert!(rotation.revoke_subkey("KEY_2024_SUB1"));
        assert!(!rotation.is_subkey_valid("KEY_2024_SUB1", 1700000000));
    }
}
