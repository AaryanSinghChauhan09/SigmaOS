//! Hardware compatibility database for SigmaOS.
//!
//! Contains the built-in compatibility matrix of tested hardware, indexed by
//! PCI vendor/device ID.  Used by the installer, driver manager, and the
//! `sigma hardware` CLI to report compatibility before and after installation.

/// Support tier for a hardware device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SupportTier {
    /// Fully supported and regression-tested. Release-blocking if broken.
    Tier1FullySupported,
    /// Community-tested. Should work; not release-blocking.
    Tier2CommunityTested,
    /// Best effort. May require manual configuration or nonfree drivers.
    Tier3BestEffort,
    /// Known not to work in v1. Will not be fixed.
    Unsupported,
}

impl SupportTier {
    /// Short label for display.
    pub fn label(self) -> &'static str {
        match self {
            Self::Tier1FullySupported  => "Tier 1",
            Self::Tier2CommunityTested => "Tier 2",
            Self::Tier3BestEffort      => "Tier 3",
            Self::Unsupported          => "Unsupported",
        }
    }
}

/// Broad hardware class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareClass {
    Cpu,
    Gpu,
    Wifi,
    Bluetooth,
    Audio,
    Storage,
    Display,
    Input,
    Printer,
    Usb,
}

/// A single entry in the compatibility matrix.
#[derive(Debug, Clone)]
pub struct HardwareEntry {
    /// PCI vendor ID (0 = any/N/A).
    pub vendor_id: u16,
    /// PCI device ID (0 = any/N/A).
    pub device_id: u16,
    /// Human-readable device name.
    pub name: String,
    /// Hardware class.
    pub class: HardwareClass,
    /// Support tier.
    pub tier: SupportTier,
    /// Kernel driver module name.
    pub driver: String,
    /// Additional notes (nonfree requirement, quirks, etc.).
    pub notes: String,
}

impl HardwareEntry {
    fn new(
        vendor_id: u16, device_id: u16, name: &str,
        class: HardwareClass, tier: SupportTier, driver: &str, notes: &str,
    ) -> Self {
        Self {
            vendor_id, device_id,
            name: name.to_string(),
            class, tier,
            driver: driver.to_string(),
            notes: notes.to_string(),
        }
    }
}

/// The hardware compatibility database.
pub struct CompatibilityMatrix {
    entries: Vec<HardwareEntry>,
}

impl CompatibilityMatrix {
    /// Build the matrix with all built-in entries.
    pub fn new() -> Self {
        let mut m = Self { entries: Vec::new() };
        m.populate();
        m
    }

    /// Look up a device by vendor and device ID.
    pub fn lookup(&self, vendor_id: u16, device_id: u16) -> Option<&HardwareEntry> {
        self.entries.iter().find(|e| e.vendor_id == vendor_id && e.device_id == device_id)
    }

    /// Look up devices by name (case-insensitive substring match).
    pub fn lookup_by_name(&self, name: &str) -> Vec<&HardwareEntry> {
        let lower = name.to_lowercase();
        self.entries.iter().filter(|e| e.name.to_lowercase().contains(&lower)).collect()
    }

    /// Return all Tier 1 devices.
    pub fn get_tier1_devices(&self) -> Vec<&HardwareEntry> {
        self.entries.iter().filter(|e| e.tier == SupportTier::Tier1FullySupported).collect()
    }

    /// Return all devices of a given hardware class.
    pub fn get_all_by_class(&self, class: HardwareClass) -> Vec<&HardwareEntry> {
        self.entries.iter().filter(|e| e.class == class).collect()
    }

    /// Total number of entries in the matrix.
    pub fn len(&self) -> usize { self.entries.len() }

    /// `true` if the matrix is empty.
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }

    // --- private: populate the database ---

    fn add(&mut self, e: HardwareEntry) { self.entries.push(e); }

    fn populate(&mut self) {
        use HardwareClass::*;
        use SupportTier::*;

        // --- GPUs ---
        self.add(HardwareEntry::new(0x8086, 0x56A0, "Intel Arc A770",
            Gpu, Tier1FullySupported, "i915", "Vulkan 1.3, VA-API"));
        self.add(HardwareEntry::new(0x8086, 0x56A1, "Intel Arc A750",
            Gpu, Tier1FullySupported, "i915", "Vulkan 1.3, VA-API"));
        self.add(HardwareEntry::new(0x8086, 0x9A49, "Intel Iris Xe (laptop)",
            Gpu, Tier1FullySupported, "i915", "Vulkan 1.2, VA-API"));
        self.add(HardwareEntry::new(0x8086, 0x4680, "Intel UHD 770",
            Gpu, Tier1FullySupported, "i915", "Vulkan 1.2, VA-API"));
        self.add(HardwareEntry::new(0x8086, 0x3EA0, "Intel UHD 620",
            Gpu, Tier1FullySupported, "i915", "Partial Vulkan, VA-API"));
        self.add(HardwareEntry::new(0x1002, 0x744C, "AMD Radeon RX 7900 XTX",
            Gpu, Tier1FullySupported, "amdgpu", "Vulkan 1.3, VA-API, ROCm"));
        self.add(HardwareEntry::new(0x1002, 0x7480, "AMD Radeon RX 7700 XT",
            Gpu, Tier1FullySupported, "amdgpu", "Vulkan 1.3, VA-API"));
        self.add(HardwareEntry::new(0x1002, 0x73DF, "AMD Radeon RX 6700 XT",
            Gpu, Tier1FullySupported, "amdgpu", "Vulkan 1.3, VA-API"));
        self.add(HardwareEntry::new(0x1002, 0x1900, "AMD Radeon 780M (iGPU)",
            Gpu, Tier1FullySupported, "amdgpu", "Vulkan 1.3, VA-API"));
        self.add(HardwareEntry::new(0x1002, 0x67DF, "AMD Radeon RX 580",
            Gpu, Tier2CommunityTested, "amdgpu", "Vulkan 1.2, VA-API"));
        self.add(HardwareEntry::new(0x10DE, 0x2684, "NVIDIA GeForce RTX 4090",
            Gpu, Tier2CommunityTested, "nvidia", "Requires sigma-nonfree; proprietary driver"));
        self.add(HardwareEntry::new(0x10DE, 0x2484, "NVIDIA GeForce RTX 3070",
            Gpu, Tier2CommunityTested, "nvidia", "Requires sigma-nonfree; proprietary driver"));
        self.add(HardwareEntry::new(0x10DE, 0x2182, "NVIDIA GeForce GTX 1660 Ti",
            Gpu, Tier2CommunityTested, "nvidia", "Requires sigma-nonfree; no VDPAU"));
        self.add(HardwareEntry::new(0x1AF4, 0x1050, "VirtIO GPU",
            Gpu, Tier1FullySupported, "virtio-gpu", "QEMU/KVM only; no Vulkan"));

        // --- WiFi ---
        self.add(HardwareEntry::new(0x8086, 0x2725, "Intel Wi-Fi 6E AX211",
            Wifi, Tier1FullySupported, "iwlwifi", "WiFi 6E, Bluetooth 5.3"));
        self.add(HardwareEntry::new(0x8086, 0x2723, "Intel Wi-Fi 6 AX200",
            Wifi, Tier1FullySupported, "iwlwifi", "WiFi 6, Bluetooth 5.2"));
        self.add(HardwareEntry::new(0x8086, 0x9DF0, "Intel Wi-Fi 5 9560",
            Wifi, Tier1FullySupported, "iwlwifi", "WiFi 5"));
        self.add(HardwareEntry::new(0x14C3, 0x7922, "MediaTek MT7922",
            Wifi, Tier1FullySupported, "mt7921e", "WiFi 6E"));
        self.add(HardwareEntry::new(0x14C3, 0x7961, "MediaTek MT7921",
            Wifi, Tier1FullySupported, "mt7921e", "WiFi 6"));
        self.add(HardwareEntry::new(0x10EC, 0x8852, "Realtek RTW8852AE",
            Wifi, Tier2CommunityTested, "rtw89", "Firmware from linux-firmware required"));
        self.add(HardwareEntry::new(0x10EC, 0xC821, "Realtek RTL8821CE",
            Wifi, Tier2CommunityTested, "rtl8821ce", "DKMS out-of-tree module"));
        self.add(HardwareEntry::new(0x14E4, 0x43A0, "Broadcom BCM4360",
            Wifi, Tier3BestEffort, "wl", "Proprietary wl driver; sigma-nonfree"));
        self.add(HardwareEntry::new(0x14E4, 0x4727, "Broadcom BCM4313",
            Wifi, Unsupported, "brcmsmac", "Pre-2012 chip; driver EOL"));

        // --- Audio ---
        self.add(HardwareEntry::new(0x8086, 0x7AD0, "Intel HDA (Alder/Raptor Lake)",
            Audio, Tier1FullySupported, "snd_hda_intel", "SOF firmware required"));
        self.add(HardwareEntry::new(0x8086, 0x02C8, "Intel HDA (Comet Lake)",
            Audio, Tier1FullySupported, "snd_hda_intel", ""));
        self.add(HardwareEntry::new(0x1002, 0x1637, "AMD HDA (Renoir/Rembrandt)",
            Audio, Tier1FullySupported, "snd_hda_intel", ""));
        self.add(HardwareEntry::new(0x0000, 0x0001, "USB Audio Class 2.0",
            Audio, Tier1FullySupported, "snd_usb_audio", "Any UAC2-compliant device"));
        self.add(HardwareEntry::new(0x0000, 0x0002, "USB Audio Class 1.0",
            Audio, Tier1FullySupported, "snd_usb_audio", "Any UAC1-compliant device"));
    }
}

impl Default for CompatibilityMatrix {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_intel_arc_a770() {
        // Arrange
        let matrix = CompatibilityMatrix::new();
        // Act
        let entry = matrix.lookup(0x8086, 0x56A0);
        // Assert
        assert!(entry.is_some());
        let e = entry.unwrap();
        assert_eq!(e.tier, SupportTier::Tier1FullySupported);
        assert_eq!(e.driver, "i915");
    }

    #[test]
    fn test_lookup_nonexistent_returns_none() {
        let matrix = CompatibilityMatrix::new();
        assert!(matrix.lookup(0xDEAD, 0xBEEF).is_none());
    }

    #[test]
    fn test_lookup_by_name_substring() {
        let matrix = CompatibilityMatrix::new();
        let results = matrix.lookup_by_name("AX200");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.name.contains("AX200")));
    }

    #[test]
    fn test_get_tier1_devices_nonempty() {
        let matrix = CompatibilityMatrix::new();
        let tier1 = matrix.get_tier1_devices();
        assert!(!tier1.is_empty());
        assert!(tier1.iter().all(|e| e.tier == SupportTier::Tier1FullySupported));
    }

    #[test]
    fn test_get_all_gpus() {
        let matrix = CompatibilityMatrix::new();
        let gpus = matrix.get_all_by_class(HardwareClass::Gpu);
        assert!(gpus.len() >= 5, "expected at least 5 GPU entries");
    }

    #[test]
    fn test_nvidia_requires_nonfree() {
        let matrix = CompatibilityMatrix::new();
        let nvidia = matrix.lookup(0x10DE, 0x2484).unwrap();
        assert!(nvidia.notes.contains("nonfree") || nvidia.driver == "nvidia");
        assert_eq!(nvidia.tier, SupportTier::Tier2CommunityTested);
    }

    #[test]
    fn test_matrix_is_non_empty() {
        let matrix = CompatibilityMatrix::new();
        assert!(matrix.len() > 10);
    }
}
