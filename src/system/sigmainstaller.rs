//! SigmaInstaller - Graphical System Installer
//!
//! Inspired by Linux Mint's Ubiquity and Calamares, this module provides a comprehensive
//! graphical system installer with partition management, locale selection, and user configuration.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Installation state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallState {
    Welcome,
    Language,
    Location,
    Partitioning,
    UserSetup,
    Installation,
    Complete,
    Failed,
    Cancelled,
    Unknown,
}

/// Partitioning mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitioningMode {
    Automatic,
    Manual,
    Alongside,
    EraseDisk,
    Unknown,
}

/// Filesystem type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    Btrfs,
    Xfs,
    Swap,
    LVM,
    Unknown,
}

/// Installation configuration
#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub install_id: String,
    pub language: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub hostname: String,
    pub username: String,
    pub partitioning_mode: PartitioningMode,
    pub target_disk: String,
    pub filesystem: FilesystemType,
    pub state: InstallState,
    pub progress_percent: u8,
    pub error_message: String,
}

impl InstallConfig {
    /// Create a new installation configuration
    pub fn new(install_id: String) -> Self {
        Self {
            install_id,
            language: String::new(),
            timezone: String::new(),
            keyboard_layout: String::new(),
            hostname: String::new(),
            username: String::new(),
            partitioning_mode: PartitioningMode::Automatic,
            target_disk: String::new(),
            filesystem: FilesystemType::Ext4,
            state: InstallState::Welcome,
            progress_percent: 0,
            error_message: String::new(),
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: InstallState) {
        self.state = state;
    }

    /// Set progress
    pub fn set_progress(&mut self, progress: u8) {
        self.progress_percent = progress;
    }

    /// Set error message
    pub fn set_error(&mut self, error: String) {
        self.error_message = error;
        self.state = InstallState::Failed;
    }

    /// Check if installation is complete
    pub fn is_complete(&self) -> bool {
        self.state == InstallState::Complete
    }

    /// Check if installation failed
    pub fn is_failed(&self) -> bool {
        self.state == InstallState::Failed
    }

    /// Check if installation is active
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            InstallState::Language | InstallState::Partitioning | InstallState::Installation
        )
    }
}

/// Installer manager
pub struct SigmaInstaller {
    configs: BTreeMap<String, InstallConfig>,
    running: AtomicBool,
}

impl SigmaInstaller {
    /// Create a new installer manager
    pub fn new() -> Self {
        Self {
            configs: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the installer manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the installer manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Create an installation configuration
    pub fn create_config(&mut self, config: InstallConfig) -> bool {
        let install_id = config.install_id.clone();
        self.configs.insert(install_id, config);
        true
    }

    /// Get a configuration by ID
    pub fn get_config(&self, install_id: &str) -> Option<&InstallConfig> {
        self.configs.get(install_id)
    }

    /// List all configurations
    pub fn list_configs(&self) -> Vec<&InstallConfig> {
        self.configs.values().collect()
    }

    /// List active installations
    pub fn list_active(&self) -> Vec<&InstallConfig> {
        self.configs.values().filter(|c| c.is_active()).collect()
    }

    /// Start installation
    pub fn start_installation(&mut self, install_id: &str) -> bool {
        if let Some(config) = self.configs.get_mut(install_id) {
            config.set_state(InstallState::Language);
            true
        } else {
            false
        }
    }

    /// Advance to next step
    pub fn advance_step(&mut self, install_id: &str) -> bool {
        if let Some(config) = self.configs.get_mut(install_id) {
            config.set_state(match config.state {
                InstallState::Welcome => InstallState::Language,
                InstallState::Language => InstallState::Location,
                InstallState::Location => InstallState::Partitioning,
                InstallState::Partitioning => InstallState::UserSetup,
                InstallState::UserSetup => InstallState::Installation,
                _ => config.state,
            });
            true
        } else {
            false
        }
    }

    /// Update progress
    pub fn update_progress(&mut self, install_id: &str, progress: u8) -> bool {
        if let Some(config) = self.configs.get_mut(install_id) {
            config.set_progress(progress);
            true
        } else {
            false
        }
    }

    /// Complete installation
    pub fn complete_installation(&mut self, install_id: &str) -> bool {
        if let Some(config) = self.configs.get_mut(install_id) {
            config.set_state(InstallState::Complete);
            config.set_progress(100);
            true
        } else {
            false
        }
    }

    /// Cancel installation
    pub fn cancel_installation(&mut self, install_id: &str) -> bool {
        if let Some(config) = self.configs.get_mut(install_id) {
            config.set_state(InstallState::Cancelled);
            true
        } else {
            false
        }
    }

    /// Fail installation
    pub fn fail_installation(&mut self, install_id: &str, error: String) -> bool {
        if let Some(config) = self.configs.get_mut(install_id) {
            config.set_error(error);
            true
        } else {
            false
        }
    }

    /// Remove configuration
    pub fn remove_config(&mut self, install_id: &str) -> Option<InstallConfig> {
        self.configs.remove(install_id)
    }

    /// Get configuration count
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }
}

impl Default for SigmaInstaller {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaInstaller instance (placeholder for actual global state)
static mut SIGMAINSTALLER_INSTANCE: Option<SigmaInstaller> = None;

/// Initialize the global SigmaInstaller instance
pub fn init_sigmainstaller() {
    unsafe {
        SIGMAINSTALLER_INSTANCE = Some(SigmaInstaller::new());
    }
}

/// Get the global SigmaInstaller instance
pub fn get_sigmainstaller() -> Option<&'static SigmaInstaller> {
    unsafe { SIGMAINSTALLER_INSTANCE.as_ref() }
}

/// Get mutable global SigmaInstaller instance
pub fn get_sigmainstaller_mut() -> Option<&'static mut SigmaInstaller> {
    unsafe { SIGMAINSTALLER_INSTANCE.as_mut() }
}

/// Convenience function to create a configuration
pub fn create_install_config(config: InstallConfig) -> bool {
    if let Some(installer) = get_sigmainstaller_mut() {
        installer.create_config(config)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_config_creation() {
        let config = InstallConfig::new("install1".to_string());

        assert_eq!(config.install_id, "install1");
        assert_eq!(config.state, InstallState::Welcome);
        assert_eq!(config.progress_percent, 0);
    }

    #[test]
    fn test_install_config_state() {
        let mut config = InstallConfig::new("install1".to_string());

        config.set_state(InstallState::Installation);
        assert!(config.is_active());

        config.set_state(InstallState::Complete);
        assert!(config.is_complete());
    }

    #[test]
    fn test_sigmainstaller_creation() {
        let installer = SigmaInstaller::new();
        assert!(!installer.is_running());
        assert_eq!(installer.config_count(), 0);
    }

    #[test]
    fn test_sigmainstaller_start_stop() {
        let mut installer = SigmaInstaller::new();
        installer.start();
        assert!(installer.is_running());
        installer.stop();
        assert!(!installer.is_running());
    }

    #[test]
    fn test_sigmainstaller_create_config() {
        let mut installer = SigmaInstaller::new();
        let config = InstallConfig::new("install1".to_string());

        let created = installer.create_config(config);
        assert!(created);
        assert_eq!(installer.config_count(), 1);
    }

    #[test]
    fn test_sigmainstaller_start_installation() {
        let mut installer = SigmaInstaller::new();
        let config = InstallConfig::new("install1".to_string());

        installer.create_config(config);
        let started = installer.start_installation("install1");
        assert!(started);

        let config = installer.get_config("install1").unwrap();
        assert!(config.is_active());
    }

    #[test]
    fn test_sigmainstaller_advance_step() {
        let mut installer = SigmaInstaller::new();
        let config = InstallConfig::new("install1".to_string());

        installer.create_config(config);
        installer.start_installation("install1");

        let advanced = installer.advance_step("install1");
        assert!(advanced);

        let config = installer.get_config("install1").unwrap();
        assert_eq!(config.state, InstallState::Location);
    }

    #[test]
    fn test_sigmainstaller_update_progress() {
        let mut installer = SigmaInstaller::new();
        let config = InstallConfig::new("install1".to_string());

        installer.create_config(config);
        let updated = installer.update_progress("install1", 50);
        assert!(updated);

        let config = installer.get_config("install1").unwrap();
        assert_eq!(config.progress_percent, 50);
    }

    #[test]
    fn test_sigmainstaller_complete_installation() {
        let mut installer = SigmaInstaller::new();
        let config = InstallConfig::new("install1".to_string());

        installer.create_config(config);
        let completed = installer.complete_installation("install1");
        assert!(completed);

        let config = installer.get_config("install1").unwrap();
        assert!(config.is_complete());
        assert_eq!(config.progress_percent, 100);
    }

    #[test]
    fn test_sigmainstaller_cancel_installation() {
        let mut installer = SigmaInstaller::new();
        let config = InstallConfig::new("install1".to_string());

        installer.create_config(config);
        let cancelled = installer.cancel_installation("install1");
        assert!(cancelled);

        let config = installer.get_config("install1").unwrap();
        assert_eq!(config.state, InstallState::Cancelled);
    }

    #[test]
    fn test_global_instance() {
        init_sigmainstaller();

        let installer = get_sigmainstaller();
        assert!(installer.is_some());

        let mut installer_mut = get_sigmainstaller_mut();
        assert!(installer_mut.is_some());

        if let Some(installer) = installer_mut {
            installer.start();
            assert!(installer.is_running());
        }
    }

    #[test]
    fn test_convenience_create_config() {
        init_sigmainstaller();

        let config = InstallConfig::new("install1".to_string());
        let created = create_install_config(config);
        assert!(created);

        let installer = get_sigmainstaller().unwrap();
        assert_eq!(installer.config_count(), 1);
    }
}
