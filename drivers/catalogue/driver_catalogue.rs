// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/catalogue/driver_catalogue.rs — Driver Warehouse Catalogue
//
// The central metadata database for every Linux driver ever written.
// Each entry tracks: hardware IDs, status (active/deprecated/removed),
// kernel version range, upstream source path, SigmaOS compat status,
// dependencies, and searchable tags.
//
// This module does NOT contain driver source code — it only stores
// metadata. Actual driver sources are fetched on-demand from upstream
// repositories (kernel.org git) or the SigmaOS driver CDN.
//
// Language: Rust (std — runs in userspace as part of sigma-drivers CLI)

#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════
// § 1. Hardware Identification
// ═══════════════════════════════════════════════════════════════════════════

/// Unified hardware identifier supporting multiple bus types.
/// Mirrors Linux's MODULE_DEVICE_TABLE entries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HardwareId {
    /// PCI/PCIe device (vendor:device, optional subsystem)
    Pci {
        vendor: u16,
        device: u16,
        subvendor: Option<u16>,
        subdevice: Option<u16>,
        class: Option<u32>,      // PCI class code (3 bytes)
    },
    /// USB device (vendor:product)
    Usb {
        vendor: u16,
        product: u16,
        device_class: Option<u8>,
        interface_class: Option<u8>,
    },
    /// ACPI device (Hardware ID string)
    Acpi {
        hid: String,             // e.g., "PNP0C09", "ACPI0003"
    },
    /// Platform device (name string, used for SoC-integrated peripherals)
    Platform {
        name: String,            // e.g., "bcm2835-mmc"
    },
    /// Device Tree / Open Firmware compatible string
    Of {
        compatible: String,      // e.g., "brcm,bcm2835-sdhost"
    },
    /// I2C device
    I2c {
        name: String,
    },
    /// SPI device
    Spi {
        modalias: String,
    },
    /// SDIO device
    Sdio {
        class: u8,
        vendor: u16,
        device: u16,
    },
    /// Virtio device
    Virtio {
        device_id: u32,
        vendor_id: u32,
    },
}

impl fmt::Display for HardwareId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HardwareId::Pci { vendor, device, .. } =>
                write!(f, "PCI {:04x}:{:04x}", vendor, device),
            HardwareId::Usb { vendor, product, .. } =>
                write!(f, "USB {:04x}:{:04x}", vendor, product),
            HardwareId::Acpi { hid } =>
                write!(f, "ACPI {}", hid),
            HardwareId::Platform { name } =>
                write!(f, "PLAT {}", name),
            HardwareId::Of { compatible } =>
                write!(f, "OF {}", compatible),
            HardwareId::I2c { name } =>
                write!(f, "I2C {}", name),
            HardwareId::Spi { modalias } =>
                write!(f, "SPI {}", modalias),
            HardwareId::Sdio { vendor, device, .. } =>
                write!(f, "SDIO {:04x}:{:04x}", vendor, device),
            HardwareId::Virtio { device_id, .. } =>
                write!(f, "VIRTIO {:04x}", device_id),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 2. Driver Status & Compatibility
// ═══════════════════════════════════════════════════════════════════════════

/// Upstream Linux kernel status of this driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverStatus {
    /// Currently in mainline Linux kernel (drivers/)
    Active,
    /// In mainline but marked for removal (drivers/staging/ or DEPRECATED)
    Deprecated,
    /// Removed from mainline kernel (recoverable from git history)
    Removed,
    /// In drivers/staging/ — not yet fully merged
    Staging,
    /// Experimental / out-of-tree driver
    Experimental,
    /// Native SigmaOS driver (not from Linux)
    SigmaNative,
}

impl fmt::Display for DriverStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverStatus::Active      => write!(f, "🟢 Active"),
            DriverStatus::Deprecated  => write!(f, "🟡 Deprecated"),
            DriverStatus::Removed     => write!(f, "🔴 Removed"),
            DriverStatus::Staging     => write!(f, "🟠 Staging"),
            DriverStatus::Experimental=> write!(f, "🔵 Experimental"),
            DriverStatus::SigmaNative => write!(f, "Σ  Native"),
        }
    }
}

/// SigmaOS-specific compatibility status for this driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatStatus {
    /// Fully ported to SigmaOS SDF — runs natively
    Native,
    /// Runs via compatibility shim (UDTL/linux_compat)
    Shimmed,
    /// Listed in catalogue but not tested on SigmaOS
    Untested,
    /// Known to fail on SigmaOS (needs porting work)
    Broken,
    /// AI-ported skeleton exists, needs manual verification
    AiPorted,
}

impl fmt::Display for CompatStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompatStatus::Native   => write!(f, "✅ Native"),
            CompatStatus::Shimmed  => write!(f, "🔄 Shimmed"),
            CompatStatus::Untested => write!(f, "⬜ Untested"),
            CompatStatus::Broken   => write!(f, "❌ Broken"),
            CompatStatus::AiPorted => write!(f, "🤖 AI-Ported"),
        }
    }
}

/// Hardware category for organizational grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverCategory {
    Network,         // Ethernet controllers
    Wireless,        // Wi-Fi, cellular
    Bluetooth,
    Storage,         // NVMe, AHCI, SCSI, eMMC
    Gpu,             // Graphics processing
    Display,         // Framebuffer, KMS
    Audio,           // Sound cards, codecs
    Input,           // HID, keyboard, mouse, touchscreen
    Usb,             // USB host/device controllers
    Serial,          // UART, tty
    I2c,
    Spi,
    Sensor,          // Accelerometer, gyroscope, etc.
    Camera,          // V4L2, UVC
    Crypto,          // Hardware crypto accelerators
    Watchdog,
    Power,           // ACPI, battery, thermal
    Platform,        // SoC-specific platform drivers
    Virtio,          // Virtualization drivers
    Firmware,        // Firmware interface drivers (EFI, BIOS)
    Infiniband,
    Misc,
}

impl fmt::Display for DriverCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 3. Kernel Version
// ═══════════════════════════════════════════════════════════════════════════

/// Linux kernel version for tracking driver compatibility ranges.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KernelVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl KernelVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    /// Parse from string like "6.1.0" or "5.15"
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.is_empty() { return None; }
        Some(Self {
            major: parts.get(0)?.parse().ok()?,
            minor: parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0),
            patch: parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0),
        })
    }

    /// Well-known kernel versions for reference
    pub const V2_6_0:  Self = Self::new(2, 6, 0);
    pub const V3_0_0:  Self = Self::new(3, 0, 0);
    pub const V4_0_0:  Self = Self::new(4, 0, 0);
    pub const V5_0_0:  Self = Self::new(5, 0, 0);
    pub const V5_15_0: Self = Self::new(5, 15, 0);
    pub const V6_0_0:  Self = Self::new(6, 0, 0);
    pub const V6_1_0:  Self = Self::new(6, 1, 0);
    pub const V6_6_0:  Self = Self::new(6, 6, 0);
    pub const V6_12_0: Self = Self::new(6, 12, 0);
}

impl fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 4. Driver Entry — The core metadata record
// ═══════════════════════════════════════════════════════════════════════════

/// Upstream source location and git recovery info.
#[derive(Debug, Clone)]
pub struct UpstreamSource {
    /// Path in the Linux kernel tree (e.g., "drivers/net/ethernet/intel/e1000/")
    pub kernel_path: String,
    /// Git commit hash where this driver was last present (for removed drivers)
    pub last_commit: Option<String>,
    /// Git commit hash where this driver was removed
    pub removal_commit: Option<String>,
    /// Date of removal (ISO 8601)
    pub removal_date: Option<String>,
    /// Kernel version where driver was removed
    pub removed_in: Option<KernelVersion>,
}

/// Firmware or external dependency required by the driver.
#[derive(Debug, Clone)]
pub struct DriverDependency {
    /// Dependency name (e.g., "iwlwifi-ty-a0-gf-a0-89.ucode")
    pub name: String,
    /// Dependency type
    pub kind: DependencyKind,
    /// Where to obtain (URL or package name)
    pub source: String,
    /// Whether this is required or optional
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Firmware,       // Binary firmware blob (/lib/firmware/)
    KernelConfig,   // Required Kconfig option (e.g., CONFIG_PCI)
    KernelModule,   // Another kernel module dependency
    Library,        // Userspace library
}

/// The master metadata record for a single driver.
#[derive(Debug, Clone)]
pub struct DriverEntry {
    /// Unique identifier (e.g., "sigma-e1000", "linux-r8169", "legacy-3c59x")
    pub id: String,

    /// Human-readable display name
    pub display_name: String,

    /// Driver description
    pub description: String,

    /// Hardware category
    pub category: DriverCategory,

    /// Upstream kernel status
    pub status: DriverStatus,

    /// SigmaOS compatibility status
    pub compat_status: CompatStatus,

    /// All hardware IDs this driver supports
    pub hardware_ids: Vec<HardwareId>,

    /// Kernel version range where this driver exists/existed
    pub min_kernel: KernelVersion,
    pub max_kernel: Option<KernelVersion>,  // None = still in latest

    /// Upstream source info
    pub upstream: UpstreamSource,

    /// Dependencies
    pub dependencies: Vec<DriverDependency>,

    /// Vendor name
    pub vendor: String,

    /// Chipset/hardware family (e.g., "Intel 82540", "Realtek RTL8111")
    pub chipset_family: String,

    /// SigmaOS SDF module name (if ported)
    pub sigma_module: Option<String>,

    /// File size of packaged driver (bytes)
    pub package_size: Option<u64>,

    /// SHA-256 checksum of the packaged driver
    pub package_sha256: Option<String>,

    /// Searchable tags
    pub tags: Vec<String>,

    /// SPDX license identifier
    pub license: String,

    /// Maintainer name/email
    pub maintainer: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// § 5. Driver Catalogue — The queryable database
// ═══════════════════════════════════════════════════════════════════════════

/// Query filter for searching the catalogue.
#[derive(Debug, Default)]
pub struct CatalogueQuery {
    pub hardware_id: Option<HardwareId>,
    pub vendor: Option<String>,
    pub category: Option<DriverCategory>,
    pub status: Option<DriverStatus>,
    pub compat_status: Option<CompatStatus>,
    pub kernel_version: Option<KernelVersion>,
    pub search_text: Option<String>,
    pub include_removed: bool,
    pub include_deprecated: bool,
}

/// The master driver catalogue — holds all driver metadata entries.
pub struct DriverCatalogue {
    /// All driver entries, keyed by driver ID
    entries: HashMap<String, DriverEntry>,

    /// Index: hardware ID → driver IDs (for fast hardware matching)
    hw_index: HashMap<String, Vec<String>>,

    /// Index: category → driver IDs
    category_index: HashMap<DriverCategory, Vec<String>>,

    /// Index: vendor name (lowercase) → driver IDs
    vendor_index: HashMap<String, Vec<String>>,
}

impl DriverCatalogue {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            hw_index: HashMap::new(),
            category_index: HashMap::new(),
            vendor_index: HashMap::new(),
        }
    }

    /// Load catalogue from a JSON manifest file.
    pub fn load_from_json(path: &str) -> Result<Self, String> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read catalogue: {}", e))?;

        let json: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse catalogue JSON: {}", e))?;

        let mut catalogue = Self::new();

        if let Some(drivers) = json.get("drivers").and_then(|d| d.as_array()) {
            for driver_json in drivers {
                if let Some(entry) = Self::parse_driver_entry(driver_json) {
                    catalogue.add_entry(entry);
                }
            }
        }

        Ok(catalogue)
    }

    /// Add a driver entry and update all indices.
    pub fn add_entry(&mut self, entry: DriverEntry) {
        let id = entry.id.clone();

        // Update hardware ID index
        for hw_id in &entry.hardware_ids {
            let key = format!("{}", hw_id);
            self.hw_index.entry(key).or_default().push(id.clone());
        }

        // Update category index
        self.category_index
            .entry(entry.category)
            .or_default()
            .push(id.clone());

        // Update vendor index
        let vendor_key = entry.vendor.to_lowercase();
        self.vendor_index
            .entry(vendor_key)
            .or_default()
            .push(id.clone());

        self.entries.insert(id, entry);
    }

    /// Get a driver entry by ID.
    pub fn get(&self, id: &str) -> Option<&DriverEntry> {
        self.entries.get(id)
    }

    /// Get total number of drivers in the catalogue.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if catalogue is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Search the catalogue with filters.
    pub fn query(&self, q: &CatalogueQuery) -> Vec<&DriverEntry> {
        self.entries.values().filter(|entry| {
            // Filter by status
            if !q.include_removed && entry.status == DriverStatus::Removed {
                return false;
            }
            if !q.include_deprecated && entry.status == DriverStatus::Deprecated {
                return false;
            }

            // Filter by specific status
            if let Some(ref status) = q.status {
                if entry.status != *status { return false; }
            }

            // Filter by compat status
            if let Some(ref compat) = q.compat_status {
                if entry.compat_status != *compat { return false; }
            }

            // Filter by category
            if let Some(ref category) = q.category {
                if entry.category != *category { return false; }
            }

            // Filter by vendor (case-insensitive substring)
            if let Some(ref vendor) = q.vendor {
                let vendor_lower = vendor.to_lowercase();
                if !entry.vendor.to_lowercase().contains(&vendor_lower) {
                    return false;
                }
            }

            // Filter by hardware ID
            if let Some(ref hw_id) = q.hardware_id {
                if !entry.hardware_ids.contains(hw_id) { return false; }
            }

            // Filter by kernel version compatibility
            if let Some(ref kver) = q.kernel_version {
                if *kver < entry.min_kernel { return false; }
                if let Some(ref max) = entry.max_kernel {
                    if *kver > *max { return false; }
                }
            }

            // Full-text search
            if let Some(ref text) = q.search_text {
                let text_lower = text.to_lowercase();
                let searchable = format!(
                    "{} {} {} {} {} {}",
                    entry.id,
                    entry.display_name,
                    entry.description,
                    entry.vendor,
                    entry.chipset_family,
                    entry.tags.join(" ")
                ).to_lowercase();
                if !searchable.contains(&text_lower) { return false; }
            }

            true
        }).collect()
    }

    /// Find drivers matching a specific hardware ID.
    pub fn find_by_hardware(&self, hw_id: &HardwareId) -> Vec<&DriverEntry> {
        let key = format!("{}", hw_id);
        match self.hw_index.get(&key) {
            Some(ids) => ids.iter()
                .filter_map(|id| self.entries.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Find all drivers for a specific category.
    pub fn find_by_category(&self, category: DriverCategory) -> Vec<&DriverEntry> {
        match self.category_index.get(&category) {
            Some(ids) => ids.iter()
                .filter_map(|id| self.entries.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Find all drivers from a specific vendor.
    pub fn find_by_vendor(&self, vendor: &str) -> Vec<&DriverEntry> {
        let key = vendor.to_lowercase();
        match self.vendor_index.get(&key) {
            Some(ids) => ids.iter()
                .filter_map(|id| self.entries.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

    /// List all deprecated/removed drivers.
    pub fn list_deprecated(&self) -> Vec<&DriverEntry> {
        self.entries.values()
            .filter(|e| matches!(e.status, DriverStatus::Deprecated | DriverStatus::Removed))
            .collect()
    }

    /// Get catalogue statistics.
    pub fn statistics(&self) -> CatalogueStats {
        let mut stats = CatalogueStats::default();
        for entry in self.entries.values() {
            stats.total += 1;
            match entry.status {
                DriverStatus::Active      => stats.active += 1,
                DriverStatus::Deprecated  => stats.deprecated += 1,
                DriverStatus::Removed     => stats.removed += 1,
                DriverStatus::Staging     => stats.staging += 1,
                DriverStatus::Experimental=> stats.experimental += 1,
                DriverStatus::SigmaNative => stats.sigma_native += 1,
            }
            match entry.compat_status {
                CompatStatus::Native   => stats.compat_native += 1,
                CompatStatus::Shimmed  => stats.compat_shimmed += 1,
                CompatStatus::Untested => stats.compat_untested += 1,
                CompatStatus::Broken   => stats.compat_broken += 1,
                CompatStatus::AiPorted => stats.compat_ai_ported += 1,
            }
        }
        stats
    }

    // ── JSON parsing helpers ──────────────────────────────────────────────

    fn parse_driver_entry(json: &serde_json::Value) -> Option<DriverEntry> {
        let id = json.get("id")?.as_str()?.to_string();
        let display_name = json.get("display_name")
            .and_then(|v| v.as_str())
            .unwrap_or(&id)
            .to_string();

        let status = match json.get("status").and_then(|v| v.as_str()).unwrap_or("active") {
            "active"       => DriverStatus::Active,
            "deprecated"   => DriverStatus::Deprecated,
            "removed"      => DriverStatus::Removed,
            "staging"      => DriverStatus::Staging,
            "experimental" => DriverStatus::Experimental,
            "sigma_native" => DriverStatus::SigmaNative,
            _              => DriverStatus::Active,
        };

        let compat = match json.get("compat_status").and_then(|v| v.as_str()).unwrap_or("untested") {
            "native"    => CompatStatus::Native,
            "shimmed"   => CompatStatus::Shimmed,
            "untested"  => CompatStatus::Untested,
            "broken"    => CompatStatus::Broken,
            "ai_ported" => CompatStatus::AiPorted,
            _           => CompatStatus::Untested,
        };

        let category = match json.get("category").and_then(|v| v.as_str()).unwrap_or("misc") {
            "network"    => DriverCategory::Network,
            "wireless"   => DriverCategory::Wireless,
            "bluetooth"  => DriverCategory::Bluetooth,
            "storage"    => DriverCategory::Storage,
            "gpu"        => DriverCategory::Gpu,
            "display"    => DriverCategory::Display,
            "audio"      => DriverCategory::Audio,
            "input"      => DriverCategory::Input,
            "usb"        => DriverCategory::Usb,
            "serial"     => DriverCategory::Serial,
            "sensor"     => DriverCategory::Sensor,
            "camera"     => DriverCategory::Camera,
            "crypto"     => DriverCategory::Crypto,
            "watchdog"   => DriverCategory::Watchdog,
            "power"      => DriverCategory::Power,
            "platform"   => DriverCategory::Platform,
            "virtio"     => DriverCategory::Virtio,
            "firmware"   => DriverCategory::Firmware,
            "infiniband" => DriverCategory::Infiniband,
            _            => DriverCategory::Misc,
        };

        let min_kernel = KernelVersion::parse(
            json.get("min_kernel").and_then(|v| v.as_str()).unwrap_or("2.6.0")
        ).unwrap_or(KernelVersion::V2_6_0);

        let max_kernel = json.get("max_kernel")
            .and_then(|v| v.as_str())
            .and_then(KernelVersion::parse);

        let hardware_ids = Self::parse_hardware_ids(
            json.get("hardware_ids").and_then(|v| v.as_array())
        );

        let dependencies = Self::parse_dependencies(
            json.get("dependencies").and_then(|v| v.as_array())
        );

        let tags = json.get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect())
            .unwrap_or_default();

        Some(DriverEntry {
            id,
            display_name,
            description: json.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            category,
            status,
            compat_status: compat,
            hardware_ids,
            min_kernel,
            max_kernel,
            upstream: UpstreamSource {
                kernel_path: json.get("kernel_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                last_commit: json.get("last_commit")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                removal_commit: json.get("removal_commit")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                removal_date: json.get("removal_date")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                removed_in: json.get("removed_in")
                    .and_then(|v| v.as_str())
                    .and_then(KernelVersion::parse),
            },
            dependencies,
            vendor: json.get("vendor")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string(),
            chipset_family: json.get("chipset_family")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            sigma_module: json.get("sigma_module")
                .and_then(|v| v.as_str())
                .map(String::from),
            package_size: json.get("package_size")
                .and_then(|v| v.as_u64()),
            package_sha256: json.get("package_sha256")
                .and_then(|v| v.as_str())
                .map(String::from),
            tags,
            license: json.get("license")
                .and_then(|v| v.as_str())
                .unwrap_or("GPL-2.0-only")
                .to_string(),
            maintainer: json.get("maintainer")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        })
    }

    fn parse_hardware_ids(arr: Option<&Vec<serde_json::Value>>) -> Vec<HardwareId> {
        let arr = match arr { Some(a) => a, None => return Vec::new() };
        arr.iter().filter_map(|v| {
            let bus = v.get("bus")?.as_str()?;
            match bus {
                "pci" => Some(HardwareId::Pci {
                    vendor: u16::from_str_radix(
                        v.get("vendor")?.as_str()?.trim_start_matches("0x"), 16
                    ).ok()?,
                    device: u16::from_str_radix(
                        v.get("device")?.as_str()?.trim_start_matches("0x"), 16
                    ).ok()?,
                    subvendor: v.get("subvendor")
                        .and_then(|s| s.as_str())
                        .and_then(|s| u16::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
                    subdevice: v.get("subdevice")
                        .and_then(|s| s.as_str())
                        .and_then(|s| u16::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
                    class: v.get("class")
                        .and_then(|s| s.as_str())
                        .and_then(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
                }),
                "usb" => Some(HardwareId::Usb {
                    vendor: u16::from_str_radix(
                        v.get("vendor")?.as_str()?.trim_start_matches("0x"), 16
                    ).ok()?,
                    product: u16::from_str_radix(
                        v.get("product")?.as_str()?.trim_start_matches("0x"), 16
                    ).ok()?,
                    device_class: v.get("device_class")
                        .and_then(|s| s.as_str())
                        .and_then(|s| u8::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
                    interface_class: v.get("interface_class")
                        .and_then(|s| s.as_str())
                        .and_then(|s| u8::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
                }),
                "acpi" => Some(HardwareId::Acpi {
                    hid: v.get("hid")?.as_str()?.to_string(),
                }),
                "platform" => Some(HardwareId::Platform {
                    name: v.get("name")?.as_str()?.to_string(),
                }),
                "of" => Some(HardwareId::Of {
                    compatible: v.get("compatible")?.as_str()?.to_string(),
                }),
                "virtio" => Some(HardwareId::Virtio {
                    device_id: v.get("device_id")?.as_u64()? as u32,
                    vendor_id: v.get("vendor_id")
                        .and_then(|s| s.as_u64())
                        .unwrap_or(0x1AF4) as u32,
                }),
                _ => None,
            }
        }).collect()
    }

    fn parse_dependencies(arr: Option<&Vec<serde_json::Value>>) -> Vec<DriverDependency> {
        let arr = match arr { Some(a) => a, None => return Vec::new() };
        arr.iter().filter_map(|v| {
            Some(DriverDependency {
                name: v.get("name")?.as_str()?.to_string(),
                kind: match v.get("kind")?.as_str()? {
                    "firmware"      => DependencyKind::Firmware,
                    "kernel_config" => DependencyKind::KernelConfig,
                    "kernel_module" => DependencyKind::KernelModule,
                    "library"       => DependencyKind::Library,
                    _               => DependencyKind::KernelModule,
                },
                source: v.get("source")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string(),
                required: v.get("required")
                    .and_then(|b| b.as_bool())
                    .unwrap_or(true),
            })
        }).collect()
    }
}

/// Catalogue statistics summary.
#[derive(Debug, Default)]
pub struct CatalogueStats {
    pub total: usize,
    pub active: usize,
    pub deprecated: usize,
    pub removed: usize,
    pub staging: usize,
    pub experimental: usize,
    pub sigma_native: usize,
    pub compat_native: usize,
    pub compat_shimmed: usize,
    pub compat_untested: usize,
    pub compat_broken: usize,
    pub compat_ai_ported: usize,
}

impl fmt::Display for CatalogueStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔══════════════════════════════════════╗")?;
        writeln!(f, "║    SigmaOS Driver Catalogue Stats    ║")?;
        writeln!(f, "╠══════════════════════════════════════╣")?;
        writeln!(f, "║  Total drivers:      {:>6}          ║", self.total)?;
        writeln!(f, "║  ─────────────────────────────────   ║")?;
        writeln!(f, "║  🟢 Active:          {:>6}          ║", self.active)?;
        writeln!(f, "║  🟡 Deprecated:      {:>6}          ║", self.deprecated)?;
        writeln!(f, "║  🔴 Removed:         {:>6}          ║", self.removed)?;
        writeln!(f, "║  🟠 Staging:         {:>6}          ║", self.staging)?;
        writeln!(f, "║  🔵 Experimental:    {:>6}          ║", self.experimental)?;
        writeln!(f, "║  Σ  SigmaNative:     {:>6}          ║", self.sigma_native)?;
        writeln!(f, "║  ─────────────────────────────────   ║")?;
        writeln!(f, "║  ✅ Native compat:   {:>6}          ║", self.compat_native)?;
        writeln!(f, "║  🔄 Shimmed:         {:>6}          ║", self.compat_shimmed)?;
        writeln!(f, "║  ⬜ Untested:        {:>6}          ║", self.compat_untested)?;
        writeln!(f, "║  ❌ Broken:          {:>6}          ║", self.compat_broken)?;
        writeln!(f, "║  🤖 AI-Ported:       {:>6}          ║", self.compat_ai_ported)?;
        writeln!(f, "╚══════════════════════════════════════╝")
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// § 6. Extern dependency stubs
// ═══════════════════════════════════════════════════════════════════════════
// The `serde_json` crate is listed in SigmaOS Cargo.toml.
// Re-export for use by other driver warehouse modules.
extern crate serde_json;
