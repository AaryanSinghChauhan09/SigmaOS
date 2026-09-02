//! # Ubuntu Common Drivers & DKMS Kernel-ABI Autoloader Engine
//!
//! Inspired by Ubuntu/Debian common driver management infrastructure:
//! - `ubuntu-drivers-common` & `jockey` Additional Drivers utility
//! - Dynamic Kernel Module Support (`dkms`) automated ABI rebuilds
//! - Canonical Livepatch kernel driver hot-patching
//! - Hardware PCI ID auto-matching for NVIDIA/AMD GPUs, Broadcom Wi-Fi, Realtek NICs

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::klib::HashMap;

/// Driver License Category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverLicense {
    GplCompatible,
    Proprietary,        // e.g. NVIDIA, Broadcom STA
    OpenSourceFallback, // e.g. Nouveau, b43, r8169
}

/// Driver Hardware Class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverHardwareCategory {
    GpuDisplay,
    WirelessNetwork,
    EthernetNetwork,
    ProcessorMicrocode,
    VirtualizationGuest,
    StorageController,
}

/// Ubuntu Driver Package Metadata
#[derive(Debug, Clone)]
pub struct UbuntuDriverPackage {
    pub name: String,
    pub version: String,
    pub category: DriverHardwareCategory,
    pub license: DriverLicense,
    pub vendor_id: u16,
    pub device_id: u16,
    pub is_recommended: bool,
    pub free_software: bool,
    pub dkms_module_name: Option<String>,
}

impl UbuntuDriverPackage {
    pub fn new(
        name: &str,
        version: &str,
        category: DriverHardwareCategory,
        license: DriverLicense,
        vendor_id: u16,
        device_id: u16,
    ) -> Self {
        let free_software =
            license == DriverLicense::GplCompatible || license == DriverLicense::OpenSourceFallback;
        Self {
            name: name.to_string(),
            version: version.to_string(),
            category,
            license,
            vendor_id,
            device_id,
            is_recommended: true,
            free_software,
            dkms_module_name: None,
        }
    }

    pub fn with_dkms(mut self, module_name: &str) -> Self {
        self.dkms_module_name = Some(module_name.to_string());
        self
    }
}

/// DKMS Module Specification
#[derive(Debug, Clone)]
pub struct DkmsModuleSpec {
    pub module_name: String,
    pub module_version: String,
    pub target_kernel_abi: String, // e.g. "6.8.0-45-generic"
    pub source_path: String,       // e.g. "/usr/src/nvidia-550.120"
    pub auto_install: bool,
    pub is_built: bool,
    pub is_installed: bool,
}

impl DkmsModuleSpec {
    pub fn new(name: &str, version: &str, kernel_abi: &str) -> Self {
        Self {
            module_name: name.to_string(),
            module_version: version.to_string(),
            target_kernel_abi: kernel_abi.to_string(),
            source_path: format!("/usr/src/{}-{}", name, version),
            auto_install: true,
            is_built: false,
            is_installed: false,
        }
    }
}

/// DKMS Kernel-ABI Rebuild Engine
pub struct DkmsAbiRebuildEngine {
    pub active_kernel_version: String,
    pub dkms_modules: HashMap<String, DkmsModuleSpec>,
    pub rebuild_history: Vec<String>,
}

impl DkmsAbiRebuildEngine {
    pub fn new(kernel_version: &str) -> Self {
        Self {
            active_kernel_version: kernel_version.to_string(),
            dkms_modules: HashMap::new(),
            rebuild_history: Vec::new(),
        }
    }

    pub fn register_module(&mut self, spec: DkmsModuleSpec) {
        self.dkms_modules.insert(spec.module_name.clone(), spec);
    }

    /// Triggers automated DKMS rebuild when kernel version changes
    pub fn handle_kernel_upgrade(&mut self, new_kernel_version: &str) -> usize {
        self.active_kernel_version = new_kernel_version.to_string();
        let mut rebuilt_count = 0;

        let module_names: Vec<String> = self.dkms_modules.keys().cloned().collect();

        for name in module_names {
            if let Some(module) = self.dkms_modules.get_mut(&name) {
                if module.target_kernel_abi != new_kernel_version {
                    module.target_kernel_abi = new_kernel_version.to_string();
                    module.is_built = true;
                    module.is_installed = true;
                    rebuilt_count += 1;
                    self.rebuild_history.push(format!(
                        "DKMS rebuilt [{}-{}] for kernel [{}]",
                        module.module_name, module.module_version, new_kernel_version
                    ));
                }
            }
        }

        rebuilt_count
    }
}

/// Ubuntu Additional Drivers (`jockey` / `ubuntu-drivers-common` parity) Registry
pub struct UbuntuAdditionalDriversRegistry {
    pub available_drivers: Vec<UbuntuDriverPackage>,
    pub installed_drivers: HashMap<String, UbuntuDriverPackage>,
    pub allow_proprietary: bool,
}

impl UbuntuAdditionalDriversRegistry {
    pub fn new() -> Self {
        Self {
            available_drivers: Vec::new(),
            installed_drivers: HashMap::new(),
            allow_proprietary: true,
        }
    }

    /// Populate standard driver repository catalog
    pub fn populate_ubuntu_driver_db(&mut self) {
        self.available_drivers = vec![
            // NVIDIA GPU Drivers
            UbuntuDriverPackage::new(
                "nvidia-driver-550",
                "550.120",
                DriverHardwareCategory::GpuDisplay,
                DriverLicense::Proprietary,
                0x10DE, // NVIDIA Vendor ID
                0x2484, // RTX 3070
            )
            .with_dkms("nvidia"),
            UbuntuDriverPackage::new(
                "xserver-xorg-video-nouveau",
                "1.0.17",
                DriverHardwareCategory::GpuDisplay,
                DriverLicense::OpenSourceFallback,
                0x10DE,
                0x2484,
            ),
            // Broadcom Wi-Fi
            UbuntuDriverPackage::new(
                "bcmwl-kernel-source",
                "6.30.223.271",
                DriverHardwareCategory::WirelessNetwork,
                DriverLicense::Proprietary,
                0x14E4, // Broadcom
                0x43A0, // BCM4360
            )
            .with_dkms("wl"),
            UbuntuDriverPackage::new(
                "firmware-b43-installer",
                "1:019-4",
                DriverHardwareCategory::WirelessNetwork,
                DriverLicense::GplCompatible,
                0x14E4,
                0x43A0,
            ),
            // Realtek Ethernet
            UbuntuDriverPackage::new(
                "r8168-dkms",
                "8.052.01",
                DriverHardwareCategory::EthernetNetwork,
                DriverLicense::GplCompatible,
                0x10EC, // Realtek
                0x8168, // RTL8111/8168
            )
            .with_dkms("r8168"),
            // Intel CPU Microcode
            UbuntuDriverPackage::new(
                "intel-microcode",
                "3.20240813.1",
                DriverHardwareCategory::ProcessorMicrocode,
                DriverLicense::Proprietary,
                0x8086, // Intel
                0x9A49,
            ),
        ];
    }

    /// Detect recommended drivers for detected PCI hardware
    pub fn detect_drivers_for_pci(
        &self,
        vendor_id: u16,
        device_id: u16,
    ) -> Vec<UbuntuDriverPackage> {
        self.available_drivers
            .iter()
            .filter(|drv| drv.vendor_id == vendor_id && drv.device_id == device_id)
            .filter(|drv| self.allow_proprietary || drv.free_software)
            .cloned()
            .collect()
    }

    /// Install selected driver
    pub fn install_driver(&mut self, driver_name: &str) -> Result<(), &'static str> {
        let drv = self
            .available_drivers
            .iter()
            .find(|d| d.name == driver_name)
            .cloned()
            .ok_or("UbuntuDrivers: Driver package not found")?;

        if !self.allow_proprietary && !drv.free_software {
            return Err("UbuntuDrivers: Proprietary drivers disabled by software policy");
        }

        self.installed_drivers.insert(drv.name.clone(), drv);
        Ok(())
    }
}

impl Default for UbuntuAdditionalDriversRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Ubuntu Livepatch Hot-Patching Driver Hook
pub struct UbuntuLivepatchDriverHook {
    pub kernel_version: String,
    pub livepatch_active: bool,
    pub active_patches: Vec<String>,
}

impl UbuntuLivepatchDriverHook {
    pub fn new(kernel_version: &str) -> Self {
        Self {
            kernel_version: kernel_version.to_string(),
            livepatch_active: true,
            active_patches: Vec::new(),
        }
    }

    pub fn apply_hotpatch(&mut self, cve_id: &str, patch_name: &str) -> bool {
        if !self.livepatch_active {
            return false;
        }
        self.active_patches
            .push(format!("{}: {}", cve_id, patch_name));
        true
    }
}

/// Main Ubuntu Common Drivers Engine
pub struct UbuntuCommonDriverEngine {
    pub registry: UbuntuAdditionalDriversRegistry,
    pub dkms_engine: DkmsAbiRebuildEngine,
    pub livepatch_hook: UbuntuLivepatchDriverHook,
}

impl UbuntuCommonDriverEngine {
    pub fn new(kernel_version: &str) -> Self {
        let mut registry = UbuntuAdditionalDriversRegistry::new();
        registry.populate_ubuntu_driver_db();

        Self {
            registry,
            dkms_engine: DkmsAbiRebuildEngine::new(kernel_version),
            livepatch_hook: UbuntuLivepatchDriverHook::new(kernel_version),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_pci_matching() {
        let mut registry = UbuntuAdditionalDriversRegistry::new();
        registry.populate_ubuntu_driver_db();

        // Query NVIDIA GPU (0x10DE, 0x2484)
        let nvidia_drivers = registry.detect_drivers_for_pci(0x10DE, 0x2484);
        assert_eq!(nvidia_drivers.len(), 2);
        assert!(nvidia_drivers.iter().any(|d| d.name == "nvidia-driver-550"));
        assert!(nvidia_drivers
            .iter()
            .any(|d| d.name == "xserver-xorg-video-nouveau"));
    }

    #[test]
    fn test_proprietary_driver_policy_gating() {
        let mut registry = UbuntuAdditionalDriversRegistry::new();
        registry.populate_ubuntu_driver_db();
        registry.allow_proprietary = false; // Block proprietary drivers

        let drivers = registry.detect_drivers_for_pci(0x10DE, 0x2484);
        assert_eq!(drivers.len(), 1);
        assert_eq!(drivers[0].name, "xserver-xorg-video-nouveau");

        // Attempting to install proprietary driver fails
        assert!(registry.install_driver("nvidia-driver-550").is_err());
    }

    #[test]
    fn test_dkms_kernel_abi_rebuild() {
        let mut dkms = DkmsAbiRebuildEngine::new("6.8.0-40-generic");
        dkms.register_module(DkmsModuleSpec::new("nvidia", "550.120", "6.8.0-40-generic"));
        dkms.register_module(DkmsModuleSpec::new("r8168", "8.052.01", "6.8.0-40-generic"));

        assert_eq!(dkms.dkms_modules.len(), 2);

        // Kernel upgraded to 6.8.0-45-generic -> triggers DKMS rebuild
        let rebuilt = dkms.handle_kernel_upgrade("6.8.0-45-generic");
        assert_eq!(rebuilt, 2);
        assert_eq!(dkms.rebuild_history.len(), 2);
        assert!(dkms.dkms_modules.get("nvidia").unwrap().is_built);
    }

    #[test]
    fn test_ubuntu_livepatch_hook() {
        let mut livepatch = UbuntuLivepatchDriverHook::new("6.8.0-45-generic");
        assert!(livepatch.apply_hotpatch("CVE-2024-12345", "DRM memory leak hotfix"));
        assert_eq!(livepatch.active_patches.len(), 1);
    }
}
