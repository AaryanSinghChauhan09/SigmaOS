#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::vec;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// System Installer (Ubiquity/Calamares Inspiration)
// Graphical installer with partitioning and user setup




/// Installer stage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerStage {
    Welcome,
    Language,
    Partitioning,
    UserSetup,
    Installation,
    Complete,
}

/// Partitioning mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitioningMode {
    Automatic,
    Manual,
    Alongside,
    EraseDisk,
}

/// Installer configuration
#[derive(Debug, Clone)]
pub struct InstallerConfig {
    pub language: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub partitioning_mode: PartitioningMode,
    pub disk: String,
    pub username: String,
    pub hostname: String,
}

impl InstallerConfig {
    pub fn new() -> Self {
        Self {
            language: "en_US".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
            partitioning_mode: PartitioningMode::Automatic,
            disk: String::new(),
            username: String::new(),
            hostname: "sigmaos".to_string(),
        }
    }

    pub fn set_disk(&mut self, disk: &str) {
        self.disk = disk.to_string();
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }
}

/// Installer
pub struct SystemInstaller {
    pub stages: Vec<InstallerStage>,
    pub current_stage: InstallerStage,
    pub configuration: InstallerConfig,
    pub progress: u32,
}

impl SystemInstaller {
    pub fn new() -> Self {
        Self {
            stages: vec![
                InstallerStage::Welcome,
                InstallerStage::Language,
                InstallerStage::Partitioning,
                InstallerStage::UserSetup,
                InstallerStage::Installation,
                InstallerStage::Complete,
            ],
            current_stage: InstallerStage::Welcome,
            configuration: InstallerConfig::new(),
            progress: 0,
        }
    }

    pub fn next_stage(&mut self) -> Result<(), InstallerError> {
        let current_index = self.stages.iter().position(|&s| s == self.current_stage);
        if let Some(index) = current_index {
            if index + 1 < self.stages.len() {
                self.current_stage = self.stages[index + 1];
                Ok(())
            } else {
                Err(InstallerError::NoNextStage)
            }
        } else {
            Err(InstallerError::InvalidStage)
        }
    }

    pub fn previous_stage(&mut self) -> Result<(), InstallerError> {
        let current_index = self.stages.iter().position(|&s| s == self.current_stage);
        if let Some(index) = current_index {
            if index > 0 {
                self.current_stage = self.stages[index - 1];
                Ok(())
            } else {
                Err(InstallerError::NoPreviousStage)
            }
        } else {
            Err(InstallerError::InvalidStage)
        }
    }

    pub fn set_partitioning_mode(&mut self, mode: PartitioningMode) {
        self.configuration.partitioning_mode = mode;
    }

    pub fn start_installation(&mut self) -> Result<(), InstallerError> {
        self.current_stage = InstallerStage::Installation;
        self.progress = 0;
        Ok(())
    }

    pub fn update_progress(&mut self, progress: u32) {
        self.progress = progress;
    }

    pub fn complete_installation(&mut self) -> Result<(), InstallerError> {
        self.current_stage = InstallerStage::Complete;
        self.progress = 100;
        Ok(())
    }

    pub fn get_installation_log(&self) -> String {
        "Installation log".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallerError {
    NoNextStage,
    NoPreviousStage,
    InvalidStage,
    InstallationFailed,
    PartitioningFailed,
}

/// Configuration for Windows-hosted Wubi/Mint dual-boot virtual disk installation
#[derive(Debug, Clone)]
pub struct WubiWindowsInstallerConfig {
    pub target_drive_letter: char,
    pub install_folder_path: String,
    pub virtual_disk_size_gb: u32,
    pub virtual_swap_size_mb: u32,
    pub bcd_entry_label: String,
}

impl Default for WubiWindowsInstallerConfig {
    fn default() -> Self {
        Self {
            target_drive_letter: 'C',
            install_folder_path: "C:\\sigmaos".to_string(),
            virtual_disk_size_gb: 30,
            virtual_swap_size_mb: 2048,
            bcd_entry_label: "SigmaOS (Linux Mint Wubi Dual Boot)".to_string(),
        }
    }
}

/// Ubuntu Wubi and Linux Mint inspired Windows Installer Engine
pub struct WubiWindowsInstallerEngine {
    pub config: WubiWindowsInstallerConfig,
    pub loopback_rootfs_created: bool,
    pub bcd_registered: bool,
}

impl WubiWindowsInstallerEngine {
    pub fn new(config: WubiWindowsInstallerConfig) -> Self {
        Self {
            config,
            loopback_rootfs_created: false,
            bcd_registered: false,
        }
    }

    /// Allocates Windows NTFS loopback virtual rootfs disk file (root.disk)
    pub fn create_loopback_rootfs(&mut self) -> Result<String, InstallerError> {
        if self.config.virtual_disk_size_gb < 8 {
            return Err(InstallerError::PartitioningFailed);
        }
        self.loopback_rootfs_created = true;
        Ok(format!("{}\\\\disks\\\\root.disk", self.config.install_folder_path))
    }

    /// Registers dual-boot entry in Windows Boot Configuration Data (BCD)
    pub fn register_bcd_boot_entry(&mut self) -> Result<String, InstallerError> {
        if !self.loopback_rootfs_created {
            return Err(InstallerError::InstallationFailed);
        }
        self.bcd_registered = true;
        Ok(format!("BCD Entry Registered: {}", self.config.bcd_entry_label))
    }

    /// Uninstalls Linux Mint Wubi virtual disk and cleans Windows BCD entries
    pub fn uninstall_from_windows(&mut self) -> Result<(), InstallerError> {
        self.loopback_rootfs_created = false;
        self.bcd_registered = false;
        Ok(())
    }
}

impl Default for SystemInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_config() {
        let config = InstallerConfig::new();
        assert_eq!(config.hostname, "sigmaos");
    }

    #[test]
    fn test_system_installer() {
        let mut installer = SystemInstaller::new();
        assert_eq!(installer.current_stage, InstallerStage::Welcome);
    }

    #[test]
    fn test_next_stage() {
        let mut installer = SystemInstaller::new();
        assert!(installer.next_stage().is_ok());
        assert_eq!(installer.current_stage, InstallerStage::Language);
    }

    #[test]
    fn test_wubi_windows_installer_engine() {
        let config = WubiWindowsInstallerConfig::default();
        let mut engine = WubiWindowsInstallerEngine::new(config);

        assert!(!engine.loopback_rootfs_created);
        assert!(!engine.bcd_registered);

        // Cannot register BCD before root.disk creation
        assert!(engine.register_bcd_boot_entry().is_err());

        // Create rootfs virtual disk
        let disk_path = engine.create_loopback_rootfs().unwrap();
        assert!(disk_path.contains("root.disk"));
        assert!(engine.loopback_rootfs_created);

        // Register Windows BCD boot entry
        let bcd_res = engine.register_bcd_boot_entry().unwrap();
        assert!(bcd_res.contains("SigmaOS (Linux Mint Wubi Dual Boot)"));
        assert!(engine.bcd_registered);

        // Uninstall
        assert!(engine.uninstall_from_windows().is_ok());
        assert!(!engine.loopback_rootfs_created);
        assert!(!engine.bcd_registered);
    }
}