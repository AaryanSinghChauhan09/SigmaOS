//! System Installer (Ubiquity/Calamares Inspiration)
//! Graphical installer with partitioning and user setup
extern crate alloc;

use crate::klib::{Vec, String};

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

impl Default for SystemInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
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
}