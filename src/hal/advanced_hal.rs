//! Advanced Hardware Abstraction Layer & udev Integration
//! Automatic device discovery, rich device properties, udev rule matching,
//! hotplug events, device tree enumeration, selective driver bundling tiers,
//! virtualization fallbacks, and cross-distro driver compatibility tracking.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCategory {
    Storage,
    Network,
    Graphics,
    Input,
    Usb,
    Audio,
}

#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub syspath: String,
    pub devpath: String,
    pub subsystem: String,
    pub category: DeviceCategory,
    pub driver: Option<String>,
    pub vendor_id: u16,
    pub device_id: u16,
    pub is_hotplugged: bool,
}

#[derive(Debug, Clone)]
pub enum UdevCondition {
    KernelPattern(String),
    Subsystem(String),
    Driver(String),
}

#[derive(Debug, Clone)]
pub enum UdevAction {
    SetNodePermission(u16),
    RunProgram(String),
    CreateSymlink(String),
}

#[derive(Debug, Clone)]
pub struct UdevRule {
    pub conditions: Vec<UdevCondition>,
    pub actions: Vec<UdevAction>,
}

pub struct SigmaDeviceManager {
    pub devices: Vec<HardwareDevice>,
    pub udev_rules: Vec<UdevRule>,
}

impl SigmaDeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            udev_rules: Vec::new(),
        }
    }

    pub fn register_rule(&mut self, rule: UdevRule) {
        self.udev_rules.push(rule);
    }

    pub fn process_device_event(&mut self, dev: HardwareDevice) -> bool {
        let mut matched = false;
        for rule in &self.udev_rules {
            let mut rule_match = true;
            for cond in &rule.conditions {
                match cond {
                    UdevCondition::KernelPattern(pattern) => {
                        if !dev.devpath.contains(pattern) {
                            rule_match = false;
                            break;
                        }
                    }
                    UdevCondition::Subsystem(sub) => {
                        if &dev.subsystem != sub {
                            rule_match = false;
                            break;
                        }
                    }
                    UdevCondition::Driver(drv) => {
                        if dev.driver.as_ref() != Some(drv) {
                            rule_match = false;
                            break;
                        }
                    }
                }
            }
            if rule_match {
                matched = true;
            }
        }
        self.devices.push(dev);
        matched
    }
}

impl Default for SigmaDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Selective Driver Bundling Tiers & Virtualization Fallback Framework
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverTier {
    /// Tier 1: Core Tier-1 drivers bundled directly in ISO/Kernel (Intel, AMD, Qualcomm, NVMe, VirtIO)
    Tier1CoreBundled,
    /// Tier 2: On-demand driver fetched via universal package manager (`sigpkg`)
    Tier2OnDemandFetch,
    /// Tier 3: Rare/legacy hardware running via lightweight QEMU/KVM shim virtualization
    Tier3VirtualizationShim,
}

#[derive(Debug, Clone)]
pub struct DriverTierManifest {
    pub driver_name: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub tier: DriverTier,
    pub package_name: Option<String>,
    pub shim_command: Option<String>,
}

/// On-Demand Selective Driver Bundling Manager
pub struct SelectiveDriverBundlingEngine {
    pub manifest_registry: BTreeMap<String, DriverTierManifest>,
    pub installed_tier2_packages: Vec<String>,
}

impl SelectiveDriverBundlingEngine {
    pub fn new() -> Self {
        let mut mgr = Self {
            manifest_registry: BTreeMap::new(),
            installed_tier2_packages: Vec::new(),
        };

        // Populate Tier-1 Core Drivers
        mgr.register_driver(DriverTierManifest {
            driver_name: "i915_intel_gfx".to_string(),
            vendor_id: 0x8086,
            device_id: 0x9A49,
            tier: DriverTier::Tier1CoreBundled,
            package_name: None,
            shim_command: None,
        });

        mgr.register_driver(DriverTierManifest {
            driver_name: "amdgpu".to_string(),
            vendor_id: 0x1002,
            device_id: 0x731F,
            tier: DriverTier::Tier1CoreBundled,
            package_name: None,
            shim_command: None,
        });

        mgr
    }

    pub fn register_driver(&mut self, manifest: DriverTierManifest) {
        self.manifest_registry.insert(manifest.driver_name.clone(), manifest);
    }

    pub fn resolve_device_driver(&mut self, vendor_id: u16, device_id: u16) -> Option<DriverTierManifest> {
        self.manifest_registry
            .values()
            .find(|m| m.vendor_id == vendor_id && (m.device_id == device_id || m.device_id == 0))
            .cloned()
    }

    pub fn trigger_on_demand_fetch(&mut self, driver_name: &str) -> Result<String, &'static str> {
        let manifest = self.manifest_registry.get(driver_name).ok_or("DriverNotFound")?;
        if manifest.tier != DriverTier::Tier2OnDemandFetch {
            return Err("NotATier2Driver");
        }

        let pkg = manifest.package_name.as_ref().ok_or("MissingPackageName")?;
        if !self.installed_tier2_packages.contains(pkg) {
            self.installed_tier2_packages.push(pkg.clone());
        }

        Ok(format!("Successfully fetched on-demand package '{}' via sigpkg", pkg))
    }
}

impl Default for SelectiveDriverBundlingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Virtualization & Compatibility Shim Fallback Engine
pub struct VirtualizationFallbackEngine {
    pub active_shims: BTreeMap<String, String>, // driver_name -> qemu_cmd
}

impl VirtualizationFallbackEngine {
    pub fn new() -> Self {
        Self {
            active_shims: BTreeMap::new(),
        }
    }

    pub fn launch_legacy_device_shim(&mut self, driver_name: &str, vendor_id: u16, device_id: u16) -> String {
        let qemu_cmd = format!(
            "qemu-system-x86_64 -device vfio-pci,host={:02x}:{:02x} -display none",
            vendor_id, device_id
        );
        self.active_shims.insert(driver_name.to_string(), qemu_cmd.clone());
        qemu_cmd
    }
}

impl Default for VirtualizationFallbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverSupportStatus {
    NativeTier1,
    OnDemandPackage,
    VirtualizedShim,
    Unsupported,
}

/// Cross-Distro Driver Compatibility Matrix Dashboard
pub struct CrossDistroDriverCompatibilityMatrix {
    pub driver_matrix: BTreeMap<String, (DriverSupportStatus, String)>, // driver -> (status, notes)
}

impl CrossDistroDriverCompatibilityMatrix {
    pub fn new() -> Self {
        let mut matrix = BTreeMap::new();
        matrix.insert(
            "intel_e1000e".to_string(),
            (DriverSupportStatus::NativeTier1, "Linux & FreeBSD native parity".to_string()),
        );
        matrix.insert(
            "broadcom_bcm4360".to_string(),
            (DriverSupportStatus::OnDemandPackage, "Fetched via sigpkg broaden-firmware".to_string()),
        );
        matrix.insert(
            "isa_sound_blaster16".to_string(),
            (DriverSupportStatus::VirtualizedShim, "Legacy hardware shim via QEMU".to_string()),
        );

        Self { driver_matrix: matrix }
    }

    pub fn check_compatibility(&self, driver_name: &str) -> (DriverSupportStatus, String) {
        self.driver_matrix
            .get(driver_name)
            .cloned()
            .unwrap_or((DriverSupportStatus::Unsupported, "Unmapped device driver".to_string()))
    }
}

impl Default for CrossDistroDriverCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udev_device_manager() {
        let mut mgr = SigmaDeviceManager::new();

        mgr.register_rule(UdevRule {
            conditions: vec![
                UdevCondition::Subsystem("block".to_string()),
                UdevCondition::KernelPattern("sd".to_string()),
            ],
            actions: vec![UdevAction::SetNodePermission(0o660)],
        });

        let dev = HardwareDevice {
            syspath: "/sys/devices/pci0000:00/0000:00:1f.2/ata1/host0/target0:0:0/0:0:0:0/block/sda".to_string(),
            devpath: "/dev/sda".to_string(),
            subsystem: "block".to_string(),
            category: DeviceCategory::Storage,
            driver: Some("ahci".to_string()),
            vendor_id: 0x8086,
            device_id: 0x2822,
            is_hotplugged: true,
        };

        assert!(mgr.process_device_event(dev));
        assert_eq!(mgr.devices.len(), 1);
    }

    #[test]
    fn test_selective_driver_bundling_and_virtualization() {
        let mut bundling = SelectiveDriverBundlingEngine::new();

        bundling.register_driver(DriverTierManifest {
            driver_name: "broadcom_wifi".to_string(),
            vendor_id: 0x14E4,
            device_id: 0x43A0,
            tier: DriverTier::Tier2OnDemandFetch,
            package_name: Some("broadcom-wl-dkms".to_string()),
            shim_command: None,
        });

        let manifest = bundling.resolve_device_driver(0x14E4, 0x43A0).unwrap();
        assert_eq!(manifest.tier, DriverTier::Tier2OnDemandFetch);

        let fetch_res = bundling.trigger_on_demand_fetch("broadcom_wifi").unwrap();
        assert!(fetch_res.contains("broadcom-wl-dkms"));
        assert_eq!(bundling.installed_tier2_packages.len(), 1);

        let mut virt = VirtualizationFallbackEngine::new();
        let qemu_cmd = virt.launch_legacy_device_shim("isa_sb16", 0x1234, 0x5678);
        assert!(qemu_cmd.contains("qemu-system-x86_64"));

        let matrix = CrossDistroDriverCompatibilityMatrix::new();
        let (status, _) = matrix.check_compatibility("intel_e1000e");
        assert_eq!(status, DriverSupportStatus::NativeTier1);
    }
}
