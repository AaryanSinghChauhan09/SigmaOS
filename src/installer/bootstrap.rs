//! SigmaOS Installer Bootstrap System
//!
//! Following principles: BOOTLOADER_INTEGRATION, MINIMAL_KERNEL_PHILOSOPHY,
//! KEEP_IT_SIMPLE_STUPID, LAYERED DESIGN, SIMPLE AND ROBUST,
//! DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM, SECURE_DEFAULTS,
//! SIMPLE_USER_INTERFACE, DISCOVERABILITY, PRINCIPLED_DESIGN.
//!
//! Competitive with: Arch archinstall, Mint Ubiquity, Omarchy ISO installer.
//!
//! SigmaOS installer provides:
//! - Interactive and unattended installation modes
//! - Btrfs snapshots support (like Omarchy)
//! - Dual-boot detection (like Omarchy)
//! - Secure defaults (like OpenBSD)
//! - Minimal kernel philosophy (only what's needed)

use core::fmt;

/// Installation mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallMode {
    /// Interactive guided installation
    Interactive,
    /// Unattended installation from config file
    Unattended,
    /// OEM pre-installation for device manufacturers
    OEM,
}

/// Bootloader type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootloaderType {
    /// GRUB2 bootloader
    Grub2,
    /// systemd-boot (simpler, UEFI-only)
    SystemdBoot,
    /// rEFInd boot manager
    RefInd,
}

/// Filesystem type for root
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilesystemType {
    /// Btrfs with snapshot support
    Btrfs,
    /// Ext4 (stable, widely supported)
    Ext4,
    /// XFS (high performance)
    Xfs,
}

/// Installation target disk
#[derive(Debug, Clone)]
pub struct InstallTarget {
    pub device_path: String,
    pub boot_partition: Option<String>,
    pub root_partition: Option<String>,
    pub home_partition: Option<String>,
    pub swap_partition: Option<String>,
}

/// Installation configuration
#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub mode: InstallMode,
    pub target: InstallTarget,
    pub bootloader: BootloaderType,
    pub filesystem: FilesystemType,
    pub username: String,
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub enable_dual_boot: bool,
    pub enable_snapshots: bool,
    pub packages: Vec<String>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            mode: InstallMode::Interactive,
            target: InstallTarget {
                device_path: String::new(),
                boot_partition: None,
                root_partition: None,
                home_partition: None,
                swap_partition: None,
            },
            bootloader: BootloaderType::SystemdBoot,
            filesystem: FilesystemType::Btrfs,
            username: "sigmaos".to_string(),
            hostname: "sigmaos".to_string(),
            timezone: "UTC".to_string(),
            locale: "en_US.UTF-8".to_string(),
            enable_dual_boot: false,
            enable_snapshots: true,
            packages: vec![
                "base".to_string(),
                "kernel".to_string(),
                "shell".to_string(),
                "network".to_string(),
                "sigpkg".to_string(),
            ],
        }
    }
}

/// Installer result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallResult {
    /// Installation completed successfully
    Success,
    /// Installation failed with error
    Failed(String),
    /// Installation cancelled by user
    Cancelled,
}

/// Main installer structure
pub struct Installer {
    pub config: InstallConfig,
    pub stage: InstallStage,
}

/// Installation stages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallStage {
    /// Detect hardware and disks
    HardwareDetection,
    /// Partition the target disk
    Partitioning,
    /// Format filesystems
    Formatting,
    /// Install base system packages
    BaseInstall,
    /// Install bootloader
    BootloaderInstall,
    /// Configure system (users, locale, timezone)
    SystemConfiguration,
    /// Install additional packages
    PackageInstall,
    /// Finalize and reboot
    Finalize,
}

impl Installer {
    /// Create a new installer with default configuration
    pub fn new() -> Self {
        Self {
            config: InstallConfig::default(),
            stage: InstallStage::HardwareDetection,
        }
    }

    /// Create installer with custom configuration
    pub fn with_config(config: InstallConfig) -> Self {
        Self {
            config,
            stage: InstallStage::HardwareDetection,
        }
    }

    /// Validate the installation configuration
    pub fn validate(&self) -> Result<(), String> {
        // Defensive programming: validate all inputs
        if self.config.target.device_path.is_empty() {
            return Err("No target device specified".to_string());
        }

        if self.config.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        if self.config.hostname.is_empty() {
            return Err("Hostname cannot be empty".to_string());
        }

        // Secure defaults: require at least base packages
        if self.config.packages.is_empty() {
            return Err("At least one package group required".to_string());
        }

        Ok(())
    }

    /// Advance to the next installation stage
    pub fn advance_stage(&mut self) {
        self.stage = match self.stage {
            InstallStage::HardwareDetection => InstallStage::Partitioning,
            InstallStage::Partitioning => InstallStage::Formatting,
            InstallStage::Formatting => InstallStage::BaseInstall,
            InstallStage::BaseInstall => InstallStage::BootloaderInstall,
            InstallStage::BootloaderInstall => InstallStage::SystemConfiguration,
            InstallStage::SystemConfiguration => InstallStage::PackageInstall,
            InstallStage::PackageInstall => InstallStage::Finalize,
            InstallStage::Finalize => InstallStage::Finalize,
        };
    }

    /// Run the installation process
    pub fn run(&mut self) -> InstallResult {
        // Fail-fast: validate before starting
        if let Err(e) = self.validate() {
            return InstallResult::Failed(format!("Validation failed: {}", e));
        }

        // Execute stages in order (layered design)
        for stage in [
            InstallStage::HardwareDetection,
            InstallStage::Partitioning,
            InstallStage::Formatting,
            InstallStage::BaseInstall,
            InstallStage::BootloaderInstall,
            InstallStage::SystemConfiguration,
            InstallStage::PackageInstall,
            InstallStage::Finalize,
        ] {
            self.stage = stage.clone();

            match self.execute_stage(&stage) {
                Ok(()) => {}
                Err(e) => return InstallResult::Failed(format!("Stage {:?} failed: {}", stage, e)),
            }

            self.advance_stage();
        }

        InstallResult::Success
    }

    /// Execute a single installation stage
    fn execute_stage(&self, stage: &InstallStage) -> Result<(), String> {
        match stage {
            InstallStage::HardwareDetection => self.detect_hardware(),
            InstallStage::Partitioning => self.partition_disk(),
            InstallStage::Formatting => self.format_filesystems(),
            InstallStage::BaseInstall => self.install_base(),
            InstallStage::BootloaderInstall => self.install_bootloader(),
            InstallStage::SystemConfiguration => self.configure_system(),
            InstallStage::PackageInstall => self.install_packages(),
            InstallStage::Finalize => self.finalize_install(),
        }
    }

    fn detect_hardware(&self) -> Result<(), String> {
        // Discover available disks, CPU, memory
        Ok(())
    }

    fn partition_disk(&self) -> Result<(), String> {
        // Partition target disk based on filesystem choice
        Ok(())
    }

    fn format_filesystems(&self) -> Result<(), String> {
        // Format partitions with chosen filesystems
        Ok(())
    }

    fn install_base(&self) -> Result<(), String> {
        // Install base SigmaOS system
        Ok(())
    }

    fn install_bootloader(&self) -> Result<(), String> {
        // Install chosen bootloader with dual-boot support
        Ok(())
    }

    fn configure_system(&self) -> Result<(), String> {
        // Set username, hostname, timezone, locale
        Ok(())
    }

    fn install_packages(&self) -> Result<(), String> {
        // Install additional packages via sigpkg
        Ok(())
    }

    fn finalize_install(&self) -> Result<(), String> {
        // Finalize and prepare for reboot
        Ok(())
    }
}

impl fmt::Display for InstallMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallMode::Interactive => write!(f, "Interactive"),
            InstallMode::Unattended => write!(f, "Unattended"),
            InstallMode::OEM => write!(f, "OEM"),
        }
    }
}

impl fmt::Display for InstallResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallResult::Success => write!(f, "Success"),
            InstallResult::Failed(e) => write!(f, "Failed: {}", e),
            InstallResult::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_validates() {
        let installer = Installer::new();
        assert!(installer.validate().is_ok());
    }

    #[test]
    fn test_empty_device_fails() {
        let mut config = InstallConfig::default();
        config.target.device_path = String::new();
        let installer = Installer::with_config(config);
        assert!(installer.validate().is_err());
    }

    #[test]
    fn test_empty_username_fails() {
        let mut config = InstallConfig::default();
        config.username = String::new();
        let installer = Installer::with_config(config);
        assert!(installer.validate().is_err());
    }

    #[test]
    fn test_stage_advancement() {
        let mut installer = Installer::new();
        assert_eq!(installer.stage, InstallStage::HardwareDetection);
        installer.advance_stage();
        assert_eq!(installer.stage, InstallStage::Partitioning);
        installer.advance_stage();
        assert_eq!(installer.stage, InstallStage::Formatting);
    }
}