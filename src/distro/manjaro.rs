#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Manjaro Distro Integration Module
// Models advanced rolling-release, automatic hardware configuration,
// kernel switching, and mirror-ranked transactional packaging.

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
    pub channel: String, // stable, beta, edge
    pub confinement: String, // classic, strict
}

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
    pub channel: String, // stable, beta, edge
    pub confinement: String, // classic, strict
}

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
    pub channel: String, // stable, beta, edge
    pub confinement: String, // classic, strict
}

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
    pub channel: String, // stable, beta, edge
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
        self.compiled_modules_for_kernels.insert(kernel_version.to_string(), compiled);
        count
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
                .unwrap_or(std::cmp::Ordering::Equal)
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
            if !self.installed_packages.contains_key(dep) && !self.installed_aur_packages.contains_key(dep) {
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
        let packs = self.language_packs.get(locale).ok_or("Locale not found in language pack index.")?;
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
        assert!(msm.langpack_installer.installed_packs.contains(&"firefox-i18n-de".to_string()));

        msm.configure_thermal_profile(true);
        assert_eq!(msm.optimal_thermal_fan_speed_rpm, 4500);
        assert_eq!(msm.power_governor.current_profile, PowerProfile::Performance);
        assert_eq!(msm.power_governor.target_cpu_freq_mhz, 4800);
    }

    #[test]
    fn test_mhwd_power_governor() {
        let mut gov = MhwdPowerGovernor::new();
        assert_eq!(gov.current_profile, PowerProfile::Balanced);

        gov.set_profile(PowerProfile::PowerSaver);
        assert_eq!(gov.target_cpu_freq_mhz, 1200);
        assert!(gov.pci_power_suspended);

        gov.toggle_prime_offload(true);
        assert!(gov.prime_offload_enabled);
    }

    #[test]
    fn test_pamac_aur_and_sandboxes() {
        let mut pamac = PamacPackageManager::new();
        let aur_pkg = AurPackage {
            name: "spotify".to_string(),
            pkgbuild_url: "https://aur.archlinux.org/spotify.git".to_string(),
            dependencies: vec!["libcurl".to_string()],
        };

        // Installing without resolved dependency should fail
        let res = pamac.build_and_install_aur(aur_pkg.clone());
        assert!(res.is_err());

        // Now install the dependency
        pamac.installed_packages.insert("libcurl".to_string(), "8.2.1-1".to_string());
        pamac.build_and_install_aur(aur_pkg).unwrap();
        assert!(pamac.installed_aur_packages.contains_key("spotify"));

        // Flatpak install
        let flat_app = FlatpakPackage {
            app_id: "org.gimp.GIMP".to_string(),
            runtime_version: "23.08".to_string(),
            sandbox_permissions: vec!["--share=ipc".to_string()],
        };
        pamac.install_flatpak(flat_app);
        assert!(pamac.installed_flatpaks.contains_key("org.gimp.GIMP"));

        // Snap install
        let snap_app = SnapPackage {
            name: "vlc".to_string(),
            channel: "stable".to_string(),
            confinement: "strict".to_string(),
        };
        pamac.install_snap(snap_app);
        assert!(pamac.installed_snaps.contains_key("vlc"));
    }

    #[test]
    fn test_mhwd_dkms_rebuilder() {
        let mut dkms = MhwdDkmsRebuilder::new();
        dkms.register_module("nvidia-proprietary");
        dkms.register_module("virtualbox-host-dkms");

        let compiled_count = dkms.trigger_rebuild("6.23-rc3");
        assert_eq!(compiled_count, 2);
        let modules = dkms.compiled_modules_for_kernels.get("6.23-rc3").unwrap();
        assert!(modules.contains(&"nvidia-proprietary".to_string()));
    }

    #[test]
    fn test_msm_language_packs() {
        let mut installer = MsmLanguagePackInstaller::new();
        let registered_count = installer.install_packs_for_locale("ja_JP").unwrap();
        assert_eq!(registered_count, 3);
        assert!(installer.installed_packs.contains(&"fcitx-mozc".to_string()));
    }
}
