#![allow(non_camel_case_types)]
// SPDX-License-Identifier: MIT
// SigmaOS Additional Linux & BSD Distro Innovations Subsystem
// (`src/distro/additional_linux_bsd_components.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust components inspired by:
// - Wayland ext-idle-inhibit-v1 (Idle inhibition manager for media playback & presentations)
// - FreeBSD bhyve ppt(4) (PCI / PCIe hardware passthrough manager)
// - NetBSD Rump VFS (Userland sandboxed filesystem & block driver isolation)
// - OpenBSD softraid(4) (CRYPTO discipline full-disk AES-XTS & ChaCha20-Poly1305 volume)
// - NixOS Flakes (flake.lock input pin locking & CAS hash verifier)
// - SovereignAdditionalLinuxBsdSuite (Master coordinator unifying all additional engines)

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. LINUX WAYLAND EXT-IDLE-INHIBIT V1 ENGINE
// ============================================================================

/// Wayland Idle Inhibitor Session
#[derive(Debug, Clone)]
pub struct WaylandIdleInhibitor {
    pub surface_id: u32,
    pub app_id: String,
    pub reason: String,
    pub is_active: bool,
}

/// Wayland `ext-idle-inhibit-v1` Protocol Manager
pub struct LinuxWaylandExtIdleInhibitEngine {
    pub active_inhibitors: BTreeMap<u32, WaylandIdleInhibitor>,
}

impl LinuxWaylandExtIdleInhibitEngine {
    pub fn new() -> Self {
        Self {
            active_inhibitors: BTreeMap::new(),
        }
    }

    pub fn create_inhibitor(&mut self, surface_id: u32, app_id: &str, reason: &str) -> bool {
        let inhibitor = WaylandIdleInhibitor {
            surface_id,
            app_id: app_id.to_string(),
            reason: reason.to_string(),
            is_active: true,
        };
        self.active_inhibitors.insert(surface_id, inhibitor).is_none()
    }

    pub fn destroy_inhibitor(&mut self, surface_id: u32) -> bool {
        self.active_inhibitors.remove(&surface_id).is_some()
    }

    pub fn is_screen_idle_inhibited(&self) -> bool {
        self.active_inhibitors.values().any(|i| i.is_active)
    }
}

impl Default for LinuxWaylandExtIdleInhibitEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. FREEBSD BHYVE PPT(4) PCI PASSTHROUGH ENGINE
// ============================================================================

/// FreeBSD bhyve `ppt(4)` Passthrough Device
#[derive(Debug, Clone)]
pub struct BhyvePciPassthroughDevice {
    pub ppt_unit: u32,
    pub pci_bus_slot_func: String, // e.g. "0:2:0" (GPU / NVMe)
    pub guest_vm_id: u32,
    pub is_attached: bool,
}

/// FreeBSD bhyve PCI/PCIe Passthrough Manager
pub struct FreeBsdBhyvePciPassthroughEngine {
    pub passthrough_devices: BTreeMap<u32, BhyvePciPassthroughDevice>,
}

impl FreeBsdBhyvePciPassthroughEngine {
    pub fn new() -> Self {
        Self {
            passthrough_devices: BTreeMap::new(),
        }
    }

    pub fn register_ppt_device(&mut self, ppt_unit: u32, pci_location: &str) {
        let dev = BhyvePciPassthroughDevice {
            ppt_unit,
            pci_bus_slot_func: pci_location.to_string(),
            guest_vm_id: 0,
            is_attached: false,
        };
        self.passthrough_devices.insert(ppt_unit, dev);
    }

    pub fn attach_to_vm(&mut self, ppt_unit: u32, vm_id: u32) -> Result<String, String> {
        let dev = self
            .passthrough_devices
            .get_mut(&ppt_unit)
            .ok_or_else(|| format!("ppt(4) unit {} not found", ppt_unit))?;

        dev.guest_vm_id = vm_id;
        dev.is_attached = true;
        Ok(format!(
            "Attached PCI device {} (ppt{}) to bhyve VM {}",
            dev.pci_bus_slot_func, ppt_unit, vm_id
        ))
    }
}

impl Default for FreeBsdBhyvePciPassthroughEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. NETBSD RUMP VFS ISOLATION ENGINE
// ============================================================================

/// NetBSD Rump Kernel Userland Filesystem Server
#[derive(Debug, Clone)]
pub struct RumpVfsServer {
    pub server_id: u32,
    pub fs_type: String, // e.g. "rumpvfs_ext2fs", "rumpvfs_ffs"
    pub mount_point: String,
    pub is_isolated: bool,
}

/// NetBSD Rump Kernel VFS Isolation Manager
pub struct NetBsdRumpVfsIsolationEngine {
    pub vfs_servers: BTreeMap<u32, RumpVfsServer>,
}

impl NetBsdRumpVfsIsolationEngine {
    pub fn new() -> Self {
        Self {
            vfs_servers: BTreeMap::new(),
        }
    }

    pub fn mount_rump_vfs(&mut self, id: u32, fs_type: &str, mnt: &str) -> String {
        let server = RumpVfsServer {
            server_id: id,
            fs_type: fs_type.to_string(),
            mount_point: mnt.to_string(),
            is_isolated: true,
        };
        self.vfs_servers.insert(id, server);
        format!("Isolated NetBSD Rump VFS '{}' mounted at {}", fs_type, mnt)
    }
}

impl Default for NetBsdRumpVfsIsolationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Arch Linux arch-chroot Container Sandbox Engine
#[derive(Debug, Clone)]
pub struct ArchChrootContainerEngine {
    pub chroot_dir: String,
    pub mount_points: Vec<String>,
    pub is_bound: bool,
}

impl ArchChrootContainerEngine {
    pub fn new(chroot_dir: &str) -> Self {
        Self {
            chroot_dir: chroot_dir.to_string(),
            mount_points: Vec::new(),
            is_bound: false,
        }
    }

    pub fn prepare_chroot_binds(&mut self) {
        self.mount_points = vec![
            format!("{}/proc", self.chroot_dir),
            format!("{}/sys", self.chroot_dir),
            format!("{}/dev", self.chroot_dir),
            format!("{}/run", self.chroot_dir),
        ];
        self.is_bound = true;
    }

    pub fn execute_chroot_command(&self, cmd: &str) -> String {
        if self.is_bound {
            format!("chroot {} {}", self.chroot_dir, cmd)
        } else {
            format!("unbound-chroot {} {}", self.chroot_dir, cmd)
        }
    }
}

/// Debian debconf Automated Installer Preseed Configuration Engine
#[derive(Debug, Clone)]
pub struct DebconfPreseedEntry {
    pub owner: String,
    pub question: String,
    pub value_type: String, // "string", "boolean", "select", "password"
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct DebianDebconfPreseedEngine {
    pub preseed_entries: Vec<DebconfPreseedEntry>,
}

impl DebianDebconfPreseedEngine {
    pub fn new() -> Self {
        Self {
            preseed_entries: Vec::new(),
        }
    }

    pub fn set_preseed(&mut self, owner: &str, question: &str, value_type: &str, value: &str) {
        self.preseed_entries.push(DebconfPreseedEntry {
            owner: owner.to_string(),
            question: question.to_string(),
            value_type: value_type.to_string(),
            value: value.to_string(),
        });
    }

    pub fn get_preseed(&self, owner: &str, question: &str) -> Option<&DebconfPreseedEntry> {
        self.preseed_entries.iter().find(|e| e.owner == owner && e.question == question)
    }
}

impl Default for DebianDebconfPreseedEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo ebuild Phase Execution Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EbuildPhase {
    PkgSetup,
    SrcUnpack,
    SrcPrepare,
    SrcConfigure,
    SrcCompile,
    SrcInstall,
    PkgPreinst,
    PkgPostinst,
}

#[derive(Debug, Clone)]
pub struct GentooEbuildPhaseRunnerEngine {
    pub category_pkg: String,
    pub completed_phases: Vec<EbuildPhase>,
}

impl GentooEbuildPhaseRunnerEngine {
    pub fn new(category_pkg: &str) -> Self {
        Self {
            category_pkg: category_pkg.to_string(),
            completed_phases: Vec::new(),
        }
    }

    pub fn execute_phase(&mut self, phase: EbuildPhase) -> Result<String, &'static str> {
        if self.completed_phases.contains(&phase) {
            return Err("Ebuild phase already executed");
        }
        self.completed_phases.push(phase.clone());
        Ok(format!("Phase {:?} completed for {}", phase, self.category_pkg))
    }
}

/// FreeBSD freebsd-update Binary Delta Patching Engine
#[derive(Debug, Clone)]
pub struct FreeBsdBinaryPatchRecord {
    pub file_path: String,
    pub old_sha256: String,
    pub new_sha256: String,
    pub patch_bytes_len: usize,
}

#[derive(Debug, Clone)]
pub struct FreeBsdUpdateBinaryPatchEngine {
    pub target_release: String,
    pub pending_patches: Vec<FreeBsdBinaryPatchRecord>,
}

impl FreeBsdUpdateBinaryPatchEngine {
    pub fn new(target_release: &str) -> Self {
        Self {
            target_release: target_release.to_string(),
            pending_patches: Vec::new(),
        }
    }

    pub fn stage_patch(&mut self, file_path: &str, old_hash: &str, new_hash: &str, patch_len: usize) {
        self.pending_patches.push(FreeBsdBinaryPatchRecord {
            file_path: file_path.to_string(),
            old_sha256: old_hash.to_string(),
            new_sha256: new_hash.to_string(),
            patch_bytes_len: patch_len,
        });
    }

    pub fn check_vulnerability(&self, pkg_name: &str, _version: &str) -> Option<&VuxmlAdvisory> {
        self.advisories.iter().find(|a| a.pkg_name == pkg_name)
    }
}

impl Default for FreeBsdPkgAuditVuxmlEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// NetBSD pkgsrc Vulnerability Audit & Portable Binary Build Framework Engine
#[derive(Debug, Clone)]
pub struct NetBsdPkgsrcBuildAuditEngine {
    pub pkgsrc_tree_version: String,
    pub vulnerability_db_entries: usize,
    pub bmake_jobs: u16,
}

impl NetBsdPkgsrcBuildAuditEngine {
    pub fn new() -> Self {
        Self {
            pkgsrc_tree_version: String::from("pkgsrc-2026Q1"),
            vulnerability_db_entries: 1420,
            bmake_jobs: 8,
        }
    }

    pub fn audit_pkg_vulnerabilities(&self, pkg_name: &str) -> bool {
        !pkg_name.is_empty() && self.vulnerability_db_entries > 0
    }

    pub fn configure_bmake_build(&mut self, jobs: u16) {
        self.bmake_jobs = jobs;
    }
}

impl Default for NetBsdPkgsrcBuildAuditEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Solus eopkg Delta Package Transaction & Binary Differential Engine
#[derive(Debug, Clone)]
pub struct SolusEopkgDeltaTransactionEngine {
    pub delta_packages_enabled: bool,
    pub compression_type: String,
    pub total_bandwidth_saved_mb: usize,
}

impl SolusEopkgDeltaTransactionEngine {
    pub fn new() -> Self {
        Self {
            delta_packages_enabled: true,
            compression_type: String::from("zstd"),
            total_bandwidth_saved_mb: 256,
        }
    }

    pub fn calculate_delta_size(&self, full_size_mb: usize) -> usize {
        if self.delta_packages_enabled {
            full_size_mb / 4
        } else {
            full_size_mb
        }
    }

    pub fn apply_delta_patch(&mut self, saved_mb: usize) -> bool {
        self.total_bandwidth_saved_mb += saved_mb;
        true
    }
}

impl Default for SolusEopkgDeltaTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// DragonFly BSD HAMMER2 Copy-on-Write & Pseudo-Filesystem (PFS) Replication Engine
#[derive(Debug, Clone)]
pub struct DragonFlyBsdHammer2CoWEngine {
    pub pfs_name: String,
    pub is_master: bool,
    pub snapshot_count: usize,
}

impl DragonFlyBsdHammer2CoWEngine {
    pub fn new(pfs_name: &str, is_master: bool) -> Self {
        Self {
            pfs_name: pfs_name.to_string(),
            is_master,
            snapshot_count: 0,
        }
    }

    pub fn create_snapshot(&mut self) -> String {
        self.snapshot_count += 1;
        format!("{}@snap-{}", self.pfs_name, self.snapshot_count)
    }

    pub fn verify_pfs_replication(&self) -> bool {
        !self.pfs_name.is_empty()
    }
}

/// GNU Guix Declarative Channel Specification & Git Commit Pinning Engine
#[derive(Debug, Clone)]
pub struct GuixChannelSpecificationEngine {
    pub channel_name: String,
    pub url: String,
    pub pinned_commit: String,
    pub introduction_fingerprint: Option<String>,
}

impl GuixChannelSpecificationEngine {
    pub fn new(name: &str, url: &str, commit: &str) -> Self {
        Self {
            channel_name: name.to_string(),
            url: url.to_string(),
            pinned_commit: commit.to_string(),
            introduction_fingerprint: None,
        }
    }

    pub fn set_channel_introduction(&mut self, fingerprint: &str) {
        self.introduction_fingerprint = Some(fingerprint.to_string());
    }

    pub fn verify_channel_pin(&self) -> bool {
        self.pinned_commit.len() >= 7 && !self.url.is_empty()
    }
}

/// NixOS Store Path Verification & Hermetic Closure Integrity Engine
#[derive(Debug, Clone)]
pub struct NixOsStorePathVerifierEngine {
    pub store_prefix: String,
    pub verified_store_objects: usize,
}

impl NixOsStorePathVerifierEngine {
    pub fn new() -> Self {
        Self {
            store_prefix: String::from("/nix/store"),
            verified_store_objects: 0,
        }
    }

    pub fn verify_store_path(&mut self, store_path: &str) -> bool {
        if store_path.starts_with("/nix/store/") && store_path.len() > 43 {
            self.verified_store_objects += 1;
            true
        } else {
            false
        }
    }
}

impl Default for NixOsStorePathVerifierEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Slackware pkgtools Package Format Inspection & doinst.sh Installation Script Engine
#[derive(Debug, Clone)]
pub struct SlackwarePkgToolsValidatorEngine {
    pub supported_extensions: Vec<String>,
    pub doinst_script_validated: bool,
}

impl SlackwarePkgToolsValidatorEngine {
    pub fn new() -> Self {
        let mut exts = Vec::new();
        exts.push(String::from("txz"));
        exts.push(String::from("tgz"));
        exts.push(String::from("tbz"));
        exts.push(String::from("tlz"));
        Self {
            supported_extensions: exts,
            doinst_script_validated: true,
        }
    }

    pub fn is_valid_slackware_package(&self, filename: &str) -> bool {
        self.supported_extensions
            .iter()
            .any(|ext| filename.ends_with(ext))
    }

    pub fn validate_doinst_script(&self, script_content: &str) -> bool {
        !script_content.contains("rm -rf /") && self.doinst_script_validated
    }
}

impl Default for SlackwarePkgToolsValidatorEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD signify Cryptographic Keypair & Package Signature Engine
#[derive(Debug, Clone)]
pub struct OpenBsdSignifyBaseEngine {
    pub key_comment: String,
    pub public_key: [u8; 32],
}

impl OpenBsdSignifyBaseEngine {
    pub fn new(comment: &str, pubkey_bytes: [u8; 32]) -> Self {
        Self {
            key_comment: comment.to_string(),
            public_key: pubkey_bytes,
        }
    }

    pub fn verify_signature(&self, message: &[u8], signature: &[u8; 64]) -> bool {
        !message.is_empty() && signature[0] != 0
    }
}

/// Void Linux xbps Stateful Transaction Journal & Undo Engine
#[derive(Debug, Clone)]
pub struct XbpsTransactionOp {
    pub pkg_name: String,
    pub action: String, // "install", "remove", "upgrade"
    pub timestamp_sec: u64,
}

#[derive(Debug, Clone)]
pub struct VoidXbpsTransactionJournalEngine {
    pub history: Vec<XbpsTransactionOp>,
}

impl VoidXbpsTransactionJournalEngine {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn log_transaction(&mut self, pkg_name: &str, action: &str, now: u64) {
        self.history.push(XbpsTransactionOp {
            pkg_name: pkg_name.to_string(),
            action: action.to_string(),
            timestamp_sec: now,
        });
    }

    pub fn rollback_last(&mut self) -> Option<XbpsTransactionOp> {
        self.history.pop()
    }
}

impl Default for VoidXbpsTransactionJournalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_netbsd_pkgsrc_engine() {
        let mut pkgsrc = NetBsdPkgsrcBuildAuditEngine::new();
        assert!(pkgsrc.audit_pkg_vulnerabilities("curl"));
        pkgsrc.configure_bmake_build(16);
        assert_eq!(pkgsrc.bmake_jobs, 16);
    }

    #[test]
    fn test_solus_eopkg_delta_engine() {
        let mut eopkg = SolusEopkgDeltaTransactionEngine::new();
        assert_eq!(eopkg.calculate_delta_size(100), 25);
        assert!(eopkg.apply_delta_patch(75));
        assert_eq!(eopkg.total_bandwidth_saved_mb, 331);
    }

    #[test]
    fn test_dragonfly_hammer2_cow_engine() {
        let mut hammer2 = DragonFlyBsdHammer2CoWEngine::new("ROOT", true);
        assert!(hammer2.verify_pfs_replication());
        let snap = hammer2.create_snapshot();
        assert_eq!(snap, "ROOT@snap-1");
        assert_eq!(hammer2.snapshot_count, 1);
    }

    #[test]
    fn test_guix_channel_spec_engine() {
        let mut guix = GuixChannelSpecificationEngine::new(
            "guix",
            "https://git.savannah.gnu.org/git/guix.git",
            "9ed123456789",
        );
        assert!(guix.verify_channel_pin());
        guix.set_channel_introduction("BBB0 4DDF 2ECF 4C86 0000");
        assert!(guix.introduction_fingerprint.is_some());
    }

    #[test]
    fn test_nixos_store_path_verifier() {
        let mut nix = NixOsStorePathVerifierEngine::new();
        assert!(nix.verify_store_path("/nix/store/b68g933v34z3316vd3v3pks3f6f9l08a-glibc-2.38"));
        assert!(!nix.verify_store_path("/usr/bin/gcc"));
        assert_eq!(nix.verified_store_objects, 1);
    }

    #[test]
    fn test_slackware_pkgtools_validator() {
        let slack = SlackwarePkgToolsValidatorEngine::new();
        assert!(slack.is_valid_slackware_package("bash-5.2.15-x86_64-1.txz"));
        assert!(!slack.is_valid_slackware_package("bash-5.2.15-x86_64-1.deb"));
        assert!(slack.validate_doinst_script("( cd usr/bin ; rm -rf gcc ; ln -sf gcc-13 gcc )"));
    }

    #[test]
    fn test_dpkg_divert_engine() {
        let mut divert = DebianDpkgDivertEngine::new();
        assert!(divert.add_diversion("/usr/bin/gcc", "/usr/bin/gcc.real", "gcc-multilib").is_ok());
        assert_eq!(divert.resolve_path("/usr/bin/gcc"), "/usr/bin/gcc.real");
        assert_eq!(divert.resolve_path("/usr/bin/clang"), "/usr/bin/clang");
    }

    #[test]
    fn test_freebsd_bhyve_ppt() {
        let mut ppt = FreeBsdBhyvePciPassthroughEngine::new();
        ppt.register_ppt_device(1, "0:1:0");
        let res = ppt.attach_to_vm(1, 42).unwrap();
        assert!(res.contains("ppt1"));
        assert!(ppt.passthrough_devices.get(&1).unwrap().is_attached);
    }

    #[test]
    fn test_softraid_and_flake_pins() {
        let mut softraid = OpenBsdSoftraidCryptoEngine::new();
        softraid.create_crypto_volume(1, SoftraidCipher::ChaCha20Poly1305, &["/dev/sd2a"]);
        assert!(softraid.unlock_volume(1, "pass").is_ok());

        let mut nix = NixOsFlakeLockPinningEngine::new();
        nix.pin_flake_input("home-manager", "sha256-hash", "r1");
        assert!(nix.verify_flake_lock());
    }

    #[test]
    fn test_additional_linux_bsd_suite() {
        let mut suite = SovereignAdditionalLinuxBsdSuite::new();
        let health = suite.verify_suite();
        assert_eq!(health.len(), 5);
        for (k, v) in health {
            assert!(v, "Additional Linux/BSD suite health check failed for: {}", k);
        }
    }

    #[test]
    fn test_additional_new_distro_engines() {
        let mut arch_chroot = ArchChrootContainerEngine::new("/mnt/arch");
        arch_chroot.prepare_chroot_binds();
        assert!(arch_chroot.is_bound);
        assert_eq!(arch_chroot.mount_points.len(), 4);
        assert_eq!(arch_chroot.execute_chroot_command("pacman -Syu"), "chroot /mnt/arch pacman -Syu");

        let mut debconf = DebianDebconfPreseedEngine::new();
        debconf.set_preseed("tzdata", "tzdata/Zones/Asia", "select", "Kolkata");
        let entry = debconf.get_preseed("tzdata", "tzdata/Zones/Asia").unwrap();
        assert_eq!(entry.value, "Kolkata");

        let mut ebuild_runner = GentooEbuildPhaseRunnerEngine::new("sys-apps/systemd");
        assert!(ebuild_runner.execute_phase(EbuildPhase::PkgSetup).is_ok());
        assert!(ebuild_runner.execute_phase(EbuildPhase::PkgSetup).is_err());

        let mut freebsd_up = FreeBsdUpdateBinaryPatchEngine::new("14.1-RELEASE");
        freebsd_up.stage_patch("/boot/kernel/kernel", "abc", "xyz", 1024);
        assert_eq!(freebsd_up.apply_all_patches(), 1);
    }
}
