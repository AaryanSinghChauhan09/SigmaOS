#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// SigmaOS Manjaro Distro Integration Module
// Models advanced rolling-release, automatic hardware configuration,
// kernel switching, and mirror-ranked transactional packaging.

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// An Arch User Repository (AUR) package representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurPackage {
    pub name: String,
    pub pkgbuild_url: String,
    pub dependencies: Vec<String>,
}

/// A Flatpak sandboxed application representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatpakPackage {
    pub app_id: String,
    pub runtime_version: String,
    pub sandbox_permissions: Vec<String>,
}

/// A Snap sandboxed application representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapPackage {
    pub name: String,
    pub channel: String,     // stable, beta, edge
    pub confinement: String, // classic, strict
}

/// Hardware GPU types detected on the system bus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType {
    IntelIntegrated,
    AmdRadeon,
    NvidiaDiscrete,
    HybridIntelNvidia,
}

/// A driver module configuration managed by MHWD
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MhwdDriverConfig {
    pub name: String,
    pub version: String,
    pub open_source: bool,
    pub hybrid_supported: bool,
}

/// Manjaro Hardware Detection (MHWD) - Auto-detects optimal open/proprietary drivers
#[derive(Debug, Clone)]
pub struct ManjaroHardwareDetection {
    pub detected_gpus: Vec<GpuType>,
    pub installed_drivers: Vec<MhwdDriverConfig>,
}

impl ManjaroHardwareDetection {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            detected_gpus: Vec::new(),
            installed_drivers: Vec::new(),
        }
    }

    pub fn scan_pci_bus(&mut self, gpus: &[GpuType]) {
        self.detected_gpus = gpus.to_vec();
    }

    /// Auto-configures and installs optimal driver configurations
    pub fn auto_configure(&mut self) -> Result<usize, &'static str> {
        if self.detected_gpus.is_empty() {
            return Err("No compatible graphic processing units detected on PCI bus.");
        }

        let mut config_count = 0;
        for gpu in &self.detected_gpus {
            match gpu {
                GpuType::IntelIntegrated => {
                    self.installed_drivers.push(MhwdDriverConfig {
                        name: "video-linux-intel".to_string(),
                        version: "2026.04".to_string(),
                        open_source: true,
                        hybrid_supported: false,
                    });
                    config_count += 1;
                }
                GpuType::AmdRadeon => {
                    self.installed_drivers.push(MhwdDriverConfig {
                        name: "video-mesa-amdgpu".to_string(),
                        version: "2026.04".to_string(),
                        open_source: true,
                        hybrid_supported: false,
                    });
                    config_count += 1;
                }
                GpuType::NvidiaDiscrete => {
                    self.installed_drivers.push(MhwdDriverConfig {
                        name: "video-nvidia-proprietary".to_string(),
                        version: "555.22".to_string(),
                        open_source: false,
                        hybrid_supported: true,
                    });
                    config_count += 1;
                }
                GpuType::HybridIntelNvidia => {
                    self.installed_drivers.push(MhwdDriverConfig {
                        name: "video-hybrid-intel-nvidia-prime".to_string(),
                        version: "555.22-prime".to_string(),
                        open_source: false,
                        hybrid_supported: true,
                    });
                    config_count += 1;
                }
            }
        }
        Ok(config_count)
    }
}

impl Default for ManjaroHardwareDetection {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 1. Manjaro Branch Switcher & Staging Repository Manager
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManjaroBranch {
    Stable,
    Testing,
    Unstable,
}

pub struct ManjaroBranchManager {
    pub current_branch: ManjaroBranch,
    pub mirror_url: String,
    pub package_hold_list: Vec<String>,
}

impl ManjaroBranchManager {
    pub fn new() -> Self {
        Self {
            current_branch: ManjaroBranch::Stable,
            mirror_url: "https://repo.manjaro.org/stable".to_string(),
            package_hold_list: Vec::new(),
        }
    }

    pub fn switch_branch(&mut self, target_branch: ManjaroBranch) -> String {
        self.current_branch = target_branch;
        let branch_str = match target_branch {
            ManjaroBranch::Stable => "stable",
            ManjaroBranch::Testing => "testing",
            ManjaroBranch::Unstable => "unstable",
        };
        self.mirror_url = format!("https://repo.manjaro.org/{}", branch_str);
        self.mirror_url.clone()
    }

    pub fn hold_package(&mut self, pkg_name: &str) {
        if !self.package_hold_list.contains(&pkg_name.to_string()) {
            self.package_hold_list.push(pkg_name.to_string());
        }
    }
}

impl Default for ManjaroBranchManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Manjaro Btrfs Timeshift Auto-Snapshots
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotMode {
    Btrfs,
    Rsync,
}

#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub snapshot_id: usize,
    pub comments: String,
    pub timestamp_sec: u64,
    pub mode: SnapshotMode,
}

pub struct ManjaroTimeshiftAutoSnap {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub snapshot_mode: SnapshotMode,
    pub auto_snap_before_pacman: bool,
}

impl ManjaroTimeshiftAutoSnap {
    pub fn new(mode: SnapshotMode) -> Self {
        Self {
            snapshots: Vec::new(),
            snapshot_mode: mode,
            auto_snap_before_pacman: true,
        }
    }

    pub fn create_pre_transaction_snapshot(&mut self, comments: &str, timestamp: u64) -> usize {
        let snapshot_id = self.snapshots.len() + 1;
        self.snapshots.push(TimeshiftSnapshot {
            snapshot_id,
            comments: comments.to_string(),
            timestamp_sec: timestamp,
            mode: self.snapshot_mode,
        });
        snapshot_id
    }

    pub fn generate_grub_btrfs_entries(&self) -> Vec<String> {
        let mut entries = Vec::new();
        for snap in &self.snapshots {
            entries.push(format!(
                "menuentry 'Manjaro Timeshift Snapshot #{} ({})' {{ linux /timeshift/btrfs/{}/vmlinuz }}",
                snap.snapshot_id, snap.comments, snap.snapshot_id
            ));
        }
        entries
    }
}

impl Default for ManjaroTimeshiftAutoSnap {
    fn default() -> Self {
        Self::new(SnapshotMode::Btrfs)
    }
}

// ============================================================================
// 3. MHWD Kernel Driver Autobuilder & PRIME Offloading
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimeOffloadMode {
    IntelOnly,
    NvidiaOnly,
    OnDemandPrime,
}

pub struct MhwdKernelDriverAutobuilder {
    pub prime_mode: PrimeOffloadMode,
    pub dkms_modules: Vec<String>,
    pub auto_patch_kernel_switches: bool,
}

impl MhwdKernelDriverAutobuilder {
    pub fn new() -> Self {
        Self {
            prime_mode: PrimeOffloadMode::OnDemandPrime,
            dkms_modules: Vec::new(),
            auto_patch_kernel_switches: true,
        }
    }

    pub fn register_dkms_module(&mut self, module_name: &str) {
        if !self.dkms_modules.contains(&module_name.to_string()) {
            self.dkms_modules.push(module_name.to_string());
        }
    }

    pub fn patch_drivers_for_new_kernel(&self, kernel_ver: &str) -> usize {
        if !self.auto_patch_kernel_switches {
            return 0;
        }
        println!("MHWD Autobuilder: Patching {} DKMS drivers for kernel {}", self.dkms_modules.len(), kernel_ver);
        self.dkms_modules.len()
    }
}

impl Default for MhwdKernelDriverAutobuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Pamac Unified Multi-Backend Search Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultBackend {
    Pacman,
    Aur,
    Flatpak,
    Snap,
    AppImage,
}

#[derive(Debug, Clone)]
pub struct PackageSearchResult {
    pub name: String,
    pub version: String,
    pub description: String,
    pub backend: SearchResultBackend,
}

pub struct PamacUnifiedSearchEngine {
    pub search_index: Vec<PackageSearchResult>,
}

impl PamacUnifiedSearchEngine {
    pub fn new() -> Self {
        let mut engine = Self { search_index: Vec::new() };
        engine.populate_default_index();
        engine
    }

    fn populate_default_index(&mut self) {
        self.search_index.push(PackageSearchResult {
            name: "firefox".to_string(),
            version: "120.0".to_string(),
            description: "Web Browser".to_string(),
            backend: SearchResultBackend::Pacman,
        });
        self.search_index.push(PackageSearchResult {
            name: "spotify".to_string(),
            version: "1.2.20".to_string(),
            description: "Music Streaming Client".to_string(),
            backend: SearchResultBackend::Flatpak,
        });
        self.search_index.push(PackageSearchResult {
            name: "visual-studio-code-bin".to_string(),
            version: "1.85.0".to_string(),
            description: "VS Code binary build".to_string(),
            backend: SearchResultBackend::Aur,
        });
    }

    pub fn search(&self, query: &str) -> Vec<&PackageSearchResult> {
        self.search_index
            .iter()
            .filter(|r| r.name.contains(query) || r.description.contains(query))
            .collect()
    }
}

impl Default for PamacUnifiedSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod manjaro_tests {
    use super::*;

    #[test]
    fn test_manjaro_branch_manager() {
        let mut branch_mgr = ManjaroBranchManager::new();
        assert_eq!(branch_mgr.current_branch, ManjaroBranch::Stable);

        let new_url = branch_mgr.switch_branch(ManjaroBranch::Testing);
        assert_eq!(branch_mgr.current_branch, ManjaroBranch::Testing);
        assert!(new_url.contains("testing"));

        branch_mgr.hold_package("linux612");
        assert_eq!(branch_mgr.package_hold_list, vec!["linux612"]);
    }

    #[test]
    fn test_manjaro_timeshift_auto_snap() {
        let mut timeshift = ManjaroTimeshiftAutoSnap::new(SnapshotMode::Btrfs);
        let id = timeshift.create_pre_transaction_snapshot("pre-pacman sysupdate", 1700000000);
        assert_eq!(id, 1);

        let grub_entries = timeshift.generate_grub_btrfs_entries();
        assert_eq!(grub_entries.len(), 1);
        assert!(grub_entries[0].contains("Timeshift Snapshot #1"));
    }

    #[test]
    fn test_mhwd_autobuilder_and_pamac_search() {
        let mut mhwd = MhwdKernelDriverAutobuilder::new();
        mhwd.register_dkms_module("nvidia-proprietary");
        mhwd.register_dkms_module("broadcom-wl");

        let patched_count = mhwd.patch_drivers_for_new_kernel("6.12.0-SIGMA");
        assert_eq!(patched_count, 2);

        let search_engine = PamacUnifiedSearchEngine::new();
        let results = search_engine.search("code");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].backend, SearchResultBackend::Aur);
    }
}

/// Represents different available kernel releases to switch dynamically
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManjaroKernelRelease {
    LinuxStable,
    LinuxLts,
    LinuxRealtimeRt,
    LinuxExperimental,
}

/// Manjaro-inspired: Dynamic Kernel Module Support (DKMS) auto-module rebuilder on host kernel swaps
#[derive(Debug, Clone)]
pub struct MhwdDkmsRebuilder {
    pub registered_modules: Vec<String>,
    pub compiled_modules_for_kernels: HashMap<String, Vec<String>>,
}

impl MhwdDkmsRebuilder {
    pub fn new() -> Self {
        Self {
            registered_modules: Vec::new(),
            compiled_modules_for_kernels: HashMap::new(),
        }
    }

    pub fn register_module(&mut self, module_name: &str) {
        if !self.registered_modules.contains(&module_name.to_string()) {
            self.registered_modules.push(module_name.to_string());
        }
    }

    /// Rebuilds and recompiles registered modules dynamically for target kernel version
    pub fn trigger_rebuild(&mut self, kernel_version: &str) -> usize {
        let mut compiled = Vec::new();
        for module in &self.registered_modules {
            compiled.push(module.clone());
        }
        let count = compiled.len();
        self.compiled_modules_for_kernels
            .insert(kernel_version.to_string(), compiled);
        count
    }
}

impl Default for MhwdDkmsRebuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Manjaro Settings Manager (MSM) Kernel Switcher
#[derive(Debug, Clone)]
pub struct ManjaroKernelSwitcher {
    pub available_kernels: HashMap<ManjaroKernelRelease, String>,
    pub active_kernel: ManjaroKernelRelease,
    pub hot_swaps_completed: usize,
    pub dkms: MhwdDkmsRebuilder,
}

impl ManjaroKernelSwitcher {
    pub fn new(active: ManjaroKernelRelease) -> Self {
        let mut available = HashMap::new();
        available.insert(ManjaroKernelRelease::LinuxStable, "6.22-stable".to_string());
        available.insert(ManjaroKernelRelease::LinuxLts, "6.12-lts".to_string());
        available.insert(
            ManjaroKernelRelease::LinuxRealtimeRt,
            "6.12-rt-rt15".to_string(),
        );
        available.insert(
            ManjaroKernelRelease::LinuxExperimental,
            "6.23-rc3".to_string(),
        );

        Self {
            available_kernels: available,
            active_kernel: active,
            hot_swaps_completed: 0,
            dkms: MhwdDkmsRebuilder::new(),
        }
    }

    /// Dynamically switches active running kernel profile with safety fallback checks and auto-triggers DKMS module compilation
    pub fn switch_kernel(&mut self, target: ManjaroKernelRelease) -> Result<String, &'static str> {
        if !self.available_kernels.contains_key(&target) {
            return Err("Target kernel release is not certified or configured on host.");
        }
        if self.active_kernel == target {
            return Err("Target kernel is already loaded and active.");
        }

        self.active_kernel = target;
        self.hot_swaps_completed += 1;
        let version = self.available_kernels.get(&target).unwrap().clone();

        // Auto-recompile dynamic kernel modules via DKMS
        self.dkms.trigger_rebuild(&version);

        Ok(version)
    }
}

/// A mirror server location for package downloads
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanMirror {
    pub url: String,
    pub country: String,
    pub latency_ms: u32,
    pub reliability_score: u8, // 1 - 100
}

/// Pamac Package Manager - Unified rolling-release and mirror-ranked transactional packaging
#[derive(Debug, Clone)]
pub struct PamacPackageManager {
    pub mirrors: Vec<PacmanMirror>,
    pub installed_packages: HashMap<String, String>, // pkg -> version
    pub installed_aur_packages: HashMap<String, AurPackage>,
    pub installed_flatpaks: HashMap<String, FlatpakPackage>,
    pub installed_snaps: HashMap<String, SnapPackage>,
}

impl PamacPackageManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            installed_packages: HashMap::new(),
            installed_aur_packages: HashMap::new(),
            installed_flatpaks: HashMap::new(),
            installed_snaps: HashMap::new(),
        }
    }

    pub fn add_mirror(&mut self, mirror: PacmanMirror) {
        self.mirrors.push(mirror);
    }

    /// Ranks mirrors dynamically based on latency and reliability score
    pub fn rank_mirrors(&mut self) {
        self.mirrors.sort_by(|a, b| {
            let score_a = (a.latency_ms as f64) / (a.reliability_score as f64);
            let score_b = (b.latency_ms as f64) / (b.reliability_score as f64);
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(core::cmp::Ordering::Equal)
        });
    }

    /// Simulates transaction-based safe rolling package upgrade
    pub fn transaction_upgrade(
        &mut self,
        package_name: &str,
        version: &str,
    ) -> Result<(), &'static str> {
        if self.mirrors.is_empty() {
            return Err("Cannot perform upgrade. Mirror database list is empty.");
        }
        self.installed_packages
            .insert(package_name.to_string(), version.to_string());
        Ok(())
    }

    /// Pamac-unified: Simulates user-space secure sandbox compilation and installation of an AUR package
    pub fn build_and_install_aur(&mut self, pkg: AurPackage) -> Result<(), &'static str> {
        // First resolve dependencies in user-space
        for dep in &pkg.dependencies {
            if !self.installed_packages.contains_key(dep)
                && !self.installed_aur_packages.contains_key(dep)
            {
                return Err("Missing required AUR build dependency.");
            }
        }
        self.installed_aur_packages.insert(pkg.name.clone(), pkg);
        Ok(())
    }

    /// Pamac-unified: Install sandboxed Flatpak package
    pub fn install_flatpak(&mut self, app: FlatpakPackage) {
        self.installed_flatpaks.insert(app.app_id.clone(), app);
    }

    /// Pamac-unified: Install sandboxed Snap package
    pub fn install_snap(&mut self, app: SnapPackage) {
        self.installed_snaps.insert(app.name.clone(), app);
    }
}

impl Default for PamacPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// MSM Localization Pack Installer - handles dynamic localization files and system dictionaries
#[derive(Debug, Clone)]
pub struct MsmLanguagePackInstaller {
    pub language_packs: HashMap<String, Vec<String>>,
    pub installed_packs: Vec<String>,
}

impl MsmLanguagePackInstaller {
    pub fn new() -> Self {
        let mut language_packs = HashMap::new();
        language_packs.insert(
            "de_DE".to_string(),
            vec![
                "firefox-i18n-de".to_string(),
                "manjaro-settings-manager-langpack-de".to_string(),
                "aspell-de".to_string(),
            ],
        );
        language_packs.insert(
            "fr_FR".to_string(),
            vec![
                "firefox-i18n-fr".to_string(),
                "manjaro-settings-manager-langpack-fr".to_string(),
                "aspell-fr".to_string(),
            ],
        );
        language_packs.insert(
            "es_ES".to_string(),
            vec![
                "firefox-i18n-es-es".to_string(),
                "manjaro-settings-manager-langpack-es".to_string(),
                "aspell-es".to_string(),
            ],
        );
        language_packs.insert(
            "ja_JP".to_string(),
            vec![
                "firefox-i18n-ja".to_string(),
                "manjaro-settings-manager-langpack-ja".to_string(),
                "fcitx-mozc".to_string(),
            ],
        );

        Self {
            language_packs,
            installed_packs: Vec::new(),
        }
    }

    pub fn register_language_pack(&mut self, locale: &str, packages: Vec<String>) {
        self.language_packs.insert(locale.to_string(), packages);
    }

    /// Installs packages corresponding to the given system locale
    pub fn install_packs_for_locale(&mut self, locale: &str) -> Result<usize, &'static str> {
        let packs = self
            .language_packs
            .get(locale)
            .ok_or("Locale not found in language pack index.")?;
        let mut count = 0;
        for pack in packs {
            if !self.installed_packs.contains(pack) {
                self.installed_packs.push(pack.clone());
                count += 1;
            }
        }
        Ok(count)
    }
}

impl Default for MsmLanguagePackInstaller {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced Hardware Power/Performance Profiles managed via MHWD
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
    HybridOnDemand,
}

/// MHWD Power Governor - configures CPU/GPU parameters and prime render offloading profiles
#[derive(Debug, Clone)]
pub struct MhwdPowerGovernor {
    pub current_profile: PowerProfile,
    pub prime_offload_enabled: bool,
    pub target_cpu_freq_mhz: u32,
    pub pci_power_suspended: bool,
}

impl MhwdPowerGovernor {
    pub fn new() -> Self {
        Self {
            current_profile: PowerProfile::Balanced,
            prime_offload_enabled: false,
            target_cpu_freq_mhz: 2400,
            pci_power_suspended: false,
        }
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.current_profile = profile;
        match profile {
            PowerProfile::Performance => {
                self.target_cpu_freq_mhz = 4800;
                self.pci_power_suspended = false;
            }
            PowerProfile::Balanced => {
                self.target_cpu_freq_mhz = 2400;
                self.pci_power_suspended = false;
            }
            PowerProfile::PowerSaver => {
                self.target_cpu_freq_mhz = 1200;
                self.pci_power_suspended = true;
            }
            PowerProfile::HybridOnDemand => {
                self.target_cpu_freq_mhz = 3200;
                self.pci_power_suspended = false;
            }
        }
    }

    pub fn toggle_prime_offload(&mut self, enable: bool) {
        self.prime_offload_enabled = enable;
    }
}

impl Default for MhwdPowerGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// Manjaro Settings Manager (MSM) general localization and sensor profile settings
#[derive(Debug, Clone)]
pub struct ManjaroSettingsManager {
    pub system_language: String,
    pub kernel_driver_warnings_enabled: bool,
    pub optimal_thermal_fan_speed_rpm: u32,
    pub langpack_installer: MsmLanguagePackInstaller,
    pub power_governor: MhwdPowerGovernor,
}

impl ManjaroSettingsManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            system_language: "en_US.UTF-8".to_string(),
            kernel_driver_warnings_enabled: true,
            optimal_thermal_fan_speed_rpm: 2400,
            langpack_installer: MsmLanguagePackInstaller::new(),
            power_governor: MhwdPowerGovernor::new(),
        }
    }

    pub fn set_language(&mut self, lang: &str) -> Result<usize, &'static str> {
        self.system_language = lang.to_string();
        // Automatically attempt to install language packs matching locale
        let prefix = lang.split('.').next().unwrap_or(lang);
        self.langpack_installer.install_packs_for_locale(prefix)
    }

    pub fn configure_thermal_profile(&mut self, high_performance: bool) {
        if high_performance {
            self.optimal_thermal_fan_speed_rpm = 4500;
            self.power_governor.set_profile(PowerProfile::Performance);
        } else {
            self.optimal_thermal_fan_speed_rpm = 1800;
            self.power_governor.set_profile(PowerProfile::PowerSaver);
        }
    }
}

impl Default for ManjaroSettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Manjaro rolling update branch types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManjaroBranchType {
    Stable,
    Testing,
    Unstable,
}

/// Manjaro pacman-mirrors Branch Switcher & Fasttrack Mirror Engine
#[derive(Debug, Clone)]
pub struct ManjaroBranchSwitcherEngine {
    pub current_branch: ManjaroBranchType,
    pub fasttrack_mirrors_count: usize,
    pub sync_database_url: String,
}

impl ManjaroBranchSwitcherEngine {
    pub fn new() -> Self {
        Self {
            current_branch: ManjaroBranchType::Stable,
            fasttrack_mirrors_count: 10,
            sync_database_url: String::from("https://repo.manjaro.org/repo/stable/$repo/$arch"),
        }
    }

    pub fn switch_branch(&mut self, target_branch: ManjaroBranchType) -> String {
        self.current_branch = target_branch;
        match target_branch {
            ManjaroBranchType::Stable => {
                self.sync_database_url = String::from("https://repo.manjaro.org/repo/stable/$repo/$arch");
            }
            ManjaroBranchType::Testing => {
                self.sync_database_url = String::from("https://repo.manjaro.org/repo/testing/$repo/$arch");
            }
            ManjaroBranchType::Unstable => {
                self.sync_database_url = String::from("https://repo.manjaro.org/repo/unstable/$repo/$arch");
            }
        }
        self.sync_database_url.clone()
    }
}

impl Default for ManjaroBranchSwitcherEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Manjaro Architect CLI Net-Installer Profile & Subvolume Generator
#[derive(Debug, Clone)]
pub struct ManjaroArchitectProfileGenerator {
    pub desktop_edition: String, // "KDE", "XFCE", "GNOME", "Minimal"
    pub filesystem_type: String, // "btrfs", "zfs", "ext4", "f2fs"
    pub selected_packages: Vec<String>,
}

impl ManjaroArchitectProfileGenerator {
    pub fn new(edition: &str, fs: &str) -> Self {
        Self {
            desktop_edition: edition.to_string(),
            filesystem_type: fs.to_string(),
            selected_packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, pkg: &str) {
        if !self.selected_packages.contains(&pkg.to_string()) {
            self.selected_packages.push(pkg.to_string());
        }
    }

    pub fn generate_installer_script(&self) -> String {
        let mut script = format!(
            "#!/bin/bash\n# Manjaro Architect Net-Installer Script\n# Edition: {}\n# Filesystem: {}\n\n",
            self.desktop_edition, self.filesystem_type
        );
        script.push_str("basestrap /mnt base linux612 manjaro-system\n");
        script.push_str("fstabgen -U /mnt >> /mnt/etc/fstab\n");
        if !self.selected_packages.is_empty() {
            script.push_str(&format!("manjaro-chroot /mnt pacman -S --noconfirm {}\n", self.selected_packages.join(" ")));
        }
        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manjaro_hardware_detection() {
        let mut mhwd = ManjaroHardwareDetection::new();
        mhwd.scan_pci_bus(&[GpuType::HybridIntelNvidia, GpuType::IntelIntegrated]);

        let configs = mhwd.auto_configure().unwrap();
        assert_eq!(configs, 2);
        assert_eq!(
            mhwd.installed_drivers[0].name,
            "video-hybrid-intel-nvidia-prime"
        );
        assert_eq!(mhwd.installed_drivers[1].name, "video-linux-intel");
    }

    #[test]
    fn test_manjaro_kernel_switcher() {
        let mut switcher = ManjaroKernelSwitcher::new(ManjaroKernelRelease::LinuxLts);
        assert_eq!(switcher.active_kernel, ManjaroKernelRelease::LinuxLts);

        let target_ver = switcher
            .switch_kernel(ManjaroKernelRelease::LinuxRealtimeRt)
            .unwrap();
        assert_eq!(target_ver, "6.12-rt-rt15");
        assert_eq!(
            switcher.active_kernel,
            ManjaroKernelRelease::LinuxRealtimeRt
        );
        assert_eq!(switcher.hot_swaps_completed, 1);
    }

    #[test]
    fn test_pamac_mirror_rank_and_upgrade() {
        let mut pamac = PamacPackageManager::new();
        pamac.add_mirror(PacmanMirror {
            url: "https://mirror.manjaro.org/germany".to_string(),
            country: "Germany".to_string(),
            latency_ms: 120,
            reliability_score: 95,
        });
        pamac.add_mirror(PacmanMirror {
            url: "https://mirror.manjaro.org/usa".to_string(),
            country: "USA".to_string(),
            latency_ms: 45,
            reliability_score: 98,
        });

        pamac.rank_mirrors();
        assert_eq!(pamac.mirrors[0].country, "USA"); // Lowest scored fraction wins

        pamac.transaction_upgrade("linux622", "6.22-3").unwrap();
        assert_eq!(pamac.installed_packages.get("linux622").unwrap(), "6.22-3");
    }

    #[test]
    fn test_manjaro_settings_manager() {
        let mut msm = ManjaroSettingsManager::new();
        assert_eq!(msm.system_language, "en_US.UTF-8");

        msm.set_language("de_DE.UTF-8").unwrap();
        assert_eq!(msm.system_language, "de_DE.UTF-8");
        assert!(msm
            .langpack_installer
            .installed_packs
            .contains(&"firefox-i18n-de".to_string()));
        assert!(msm
            .langpack_installer
            .installed_packs
            .contains(&"firefox-i18n-de".to_string()));

        msm.configure_thermal_profile(true);
        assert_eq!(msm.optimal_thermal_fan_speed_rpm, 4500);
        assert_eq!(
            msm.power_governor.current_profile,
            PowerProfile::Performance
        );
        assert_eq!(msm.power_governor.target_cpu_freq_mhz, 4800);
    }

    #[test]
    fn test_manjaro_branch_switcher_engine() {
        let mut switcher = ManjaroBranchSwitcherEngine::new();
        assert_eq!(switcher.current_branch, ManjaroBranchType::Stable);

        let testing_url = switcher.switch_branch(ManjaroBranchType::Testing);
        assert_eq!(switcher.current_branch, ManjaroBranchType::Testing);
        assert!(testing_url.contains("testing"));

        let unstable_url = switcher.switch_branch(ManjaroBranchType::Unstable);
        assert_eq!(switcher.current_branch, ManjaroBranchType::Unstable);
        assert!(unstable_url.contains("unstable"));
    }

    #[test]
    fn test_manjaro_architect_profile_generator() {
        let mut arch = ManjaroArchitectProfileGenerator::new("KDE", "btrfs");
        arch.add_package("pamac-gtk");
        arch.add_package("zsh");

        let script = arch.generate_installer_script();
        assert!(script.contains("Edition: KDE"));
        assert!(script.contains("Filesystem: btrfs"));
        assert!(script.contains("pamac-gtk zsh"));
    }
}
