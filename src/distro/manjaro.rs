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

use klib::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurPackage {
    pub name: String,
    pub pkgbuild_url: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatpakPackage {
    pub app_id: String,
    pub runtime_version: String,
    pub sandbox_permissions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapPackage {
    pub name: String,
    pub channel: String,
    pub confinement: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType {
    IntelIntegrated,
    AmdRadeon,
    NvidiaDiscrete,
    HybridIntelNvidia,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MhwdDriverConfig {
    pub name: String,
    pub version: String,
    pub open_source: bool,
    pub hybrid_supported: bool,
}

#[derive(Debug, Clone)]
pub struct ManjaroHardwareDetection {
    pub detected_gpus: Vec<GpuType>,
    pub installed_drivers: Vec<MhwdDriverConfig>,
}

impl ManjaroHardwareDetection {
    pub fn new() -> Self {
        Self {
            detected_gpus: Vec::new(),
            installed_drivers: Vec::new(),
        }
    }

    pub fn scan_pci_bus(&mut self, gpus: &[GpuType]) {
        self.detected_gpus = gpus.to_vec();
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManjaroKernelRelease {
    LinuxStable,
    LinuxLts,
    LinuxRealtimeRt,
    LinuxExperimental,
}

pub struct ManjaroKernelSwitcher {
    pub active_release: ManjaroKernelRelease,
    pub installed_releases: Vec<ManjaroKernelRelease>,
}

impl Default for ManjaroKernelSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ManjaroKernelSwitcher {
    pub fn new() -> Self {
        Self {
            active_release: ManjaroKernelRelease::LinuxLts,
            installed_releases: vec![ManjaroKernelRelease::LinuxLts, ManjaroKernelRelease::LinuxStable],
        }
    }

    pub fn switch_kernel(&mut self, release: ManjaroKernelRelease) -> Result<(), &'static str> {
        if self.installed_releases.contains(&release) {
            self.active_release = release;
            Ok(())
        } else {
            Err("Kernel release not installed")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanMirror {
    pub country: String,
    pub url: String,
    pub ping_ms: u32,
}

pub struct PamacPackageManager {
    pub mirrors: Vec<PacmanMirror>,
    pub installed_apps: Vec<String>,
}

impl Default for PamacPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PamacPackageManager {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
            installed_apps: Vec::new(),
        }
    }

    pub fn rank_mirrors(&mut self) {
        self.mirrors.sort_by_key(|m| m.ping_ms);
    }
}

pub struct ManjaroSettingsManager {
    pub locale: String,
    pub timezone: String,
    pub kernel_switcher: ManjaroKernelSwitcher,
}

impl Default for ManjaroSettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ManjaroSettingsManager {
    pub fn new() -> Self {
        Self {
            locale: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            kernel_switcher: ManjaroKernelSwitcher::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MhwdDkmsRebuilder {
    pub registered_modules: Vec<String>,
    pub compiled_modules_for_kernels: HashMap<String, Vec<String>>,
}

impl Default for MhwdDkmsRebuilder {
    fn default() -> Self {
        Self::new()
    }
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
