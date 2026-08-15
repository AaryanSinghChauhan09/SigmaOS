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
        }
    }

    /// Dynamically switches active running kernel profile with safety fallback checks
    pub fn switch_kernel(&mut self, target: ManjaroKernelRelease) -> Result<String, &'static str> {
        if !self.available_kernels.contains_key(&target) {
            return Err("Target kernel release is not certified or configured on host.");
        }
        if self.active_kernel == target {
            return Err("Target kernel is already loaded and active.");
        }

        self.active_kernel = target;
        self.hot_swaps_completed += 1;
        Ok(self.available_kernels.get(&target).unwrap().clone())
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
}

impl PamacPackageManager {
    pub fn new() -> Self {
        Self {
            registered_modules: Vec::new(),
            compiled_modules_for_kernels: HashMap::new(),
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
}

impl Default for PamacPackageManager {
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
}

impl ManjaroSettingsManager {
    pub fn new() -> Self {
        Self {
            system_language: "en_US.UTF-8".to_string(),
            kernel_driver_warnings_enabled: true,
            optimal_thermal_fan_speed_rpm: 2400,
        }
    }

    /// Rebuilds and recompiles registered modules dynamically for target kernel version
    pub fn trigger_rebuild(&mut self, kernel_version: &str) -> usize {
        let mut compiled = Vec::new();
        for module in &self.registered_modules {
            compiled.push(module.clone());
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

        msm.set_language("de_DE.UTF-8");
        assert_eq!(msm.system_language, "de_DE.UTF-8");

        msm.configure_thermal_profile(true);
        assert_eq!(msm.optimal_thermal_fan_speed_rpm, 4500);
    }
}
