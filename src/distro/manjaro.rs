// SPDX-License-Identifier: MIT
// SigmaOS Manjaro Distro Integration Module

use crate::klib::collections::HashMap;
use crate::klib::{SigmaString, Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AurPackage {
    pub name: SigmaString,
    pub pkgbuild_url: SigmaString,
    pub dependencies: Vec<SigmaString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManjaroKernelRelease {
    pub version: SigmaString,
    pub is_lts: bool,
    pub dkms_modules: Vec<SigmaString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
    Virtio,
}

#[derive(Debug, Clone)]
pub struct MhwdDriverConfig {
    pub gpu_type: GpuType,
    pub driver_name: SigmaString,
    pub is_free_driver: bool,
}

pub struct ManjaroHardwareDetection {
    pub detected_gpus: Vec<MhwdDriverConfig>,
    pub installed_drivers: Vec<SigmaString>,
}

impl ManjaroHardwareDetection {
    pub fn new() -> Self {
        Self {
            detected_gpus: Vec::new(),
            installed_drivers: Vec::new(),
        }
    }

    pub fn auto_detect_hardware(&mut self, vendor_id: u16) {
        let config = match vendor_id {
            0x10DE => MhwdDriverConfig {
                gpu_type: GpuType::Nvidia,
                driver_name: SigmaString::from("video-nvidia"),
                is_free_driver: false,
            },
            0x1002 => MhwdDriverConfig {
                gpu_type: GpuType::Amd,
                driver_name: SigmaString::from("video-amdgpu"),
                is_free_driver: true,
            },
            _ => MhwdDriverConfig {
                gpu_type: GpuType::Intel,
                driver_name: SigmaString::from("video-modesetting"),
                is_free_driver: true,
            },
        };
        self.detected_gpus.push(config);
    }

    pub fn install_mhwd_driver(&mut self, driver_name: &str) -> Result<(), &'static str> {
        let sig = SigmaString::from(driver_name);
        if self.installed_drivers.contains(&sig) {
            return Err("Driver already installed");
        }
        self.installed_drivers.push(sig);
        Ok(())
    }
}

impl Default for ManjaroHardwareDetection {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MhwdDkmsRebuilder {
    pub active_kernel: ManjaroKernelRelease,
}

impl MhwdDkmsRebuilder {
    pub fn new(kernel_ver: &str, is_lts: bool) -> Self {
        Self {
            active_kernel: ManjaroKernelRelease {
                version: SigmaString::from(kernel_ver),
                is_lts,
                dkms_modules: Vec::new(),
            },
        }
    }

    pub fn register_dkms_module(&mut self, module_name: &str) {
        self.active_kernel.dkms_modules.push(SigmaString::from(module_name));
    }

    pub fn rebuild_all_dkms_modules(&self) -> usize {
        self.active_kernel.dkms_modules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mhwd_detection() {
        let mut mhwd = ManjaroHardwareDetection::new();
        mhwd.auto_detect_hardware(0x10DE);
        assert_eq!(mhwd.detected_gpus.len(), 1);
        assert_eq!(mhwd.detected_gpus[0].gpu_type, GpuType::Nvidia);

        assert!(mhwd.install_mhwd_driver("video-nvidia").is_ok());
        assert!(mhwd.install_mhwd_driver("video-nvidia").is_err());
    }

    #[test]
    fn test_dkms_rebuilder() {
        let mut dkms = MhwdDkmsRebuilder::new("linux65", true);
        dkms.register_dkms_module("nvidia-dkms");
        dkms.register_dkms_module("virtualbox-guest-dkms");

        assert_eq!(dkms.rebuild_all_dkms_modules(), 2);
    }
}
