// SigmaOS Advanced GUI Installer Wizard
// Enhanced graphical installer with comprehensive partitioning, user setup, and system configuration
// Inspired by Calamares, Ubiquity, and Windows installation wizards

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Installer Screen Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerScreen {
    Welcome,
    Language,
    Location,
    Partitioning,
    UserSetup,
    SystemConfiguration,
    InstallationProgress,
    Complete,
}

/// Partitioning Operation Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitioningOperation {
    Automatic,
    Manual,
    Alongside,
    ReplacePartition,
    Custom,
}

/// Filesystem Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    Btrfs,
    Xfs,
    Zfs,
    F2fs,
    Swap,
}

/// Partition Entry
#[derive(Debug, Clone)]
pub struct PartitionEntry {
    pub device: String,
    pub start_sector: u64,
    pub end_sector: u64,
    pub filesystem: FilesystemType,
    pub mount_point: String,
    pub size_mb: u64,
    pub flags: Vec<String>,
}

impl PartitionEntry {
    pub fn new(device: &str, size_mb: u64, fs: FilesystemType, mount: &str) -> Self {
        Self {
            device: String::from(device),
            start_sector: 0,
            end_sector: 0,
            filesystem: fs,
            mount_point: String::from(mount),
            size_mb,
            flags: Vec::new(),
        }
    }

    pub fn with_flag(mut self, flag: &str) -> Self {
        self.flags.push(String::from(flag));
        self
    }
}

/// Disk Information
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub device: String,
    pub size_mb: u64,
    pub model: String,
    pub partitions: Vec<PartitionEntry>,
}

impl DiskInfo {
    pub fn new(device: &str, size_mb: u64, model: &str) -> Self {
        Self {
            device: String::from(device),
            size_mb,
            model: String::from(model),
            partitions: Vec::new(),
        }
    }

    pub fn add_partition(&mut self, partition: PartitionEntry) {
        self.partitions.push(partition);
    }

    pub fn get_free_space(&self) -> u64 {
        let used_space: u64 = self.partitions.iter().map(|p| p.size_mb).sum();
        self.size_mb.saturating_sub(used_space)
    }
}

/// User Account Configuration
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub is_admin: bool,
    pub home_directory: String,
    pub shell: String,
}

impl UserAccount {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            full_name: String::new(),
            password: String::from(password),
            is_admin: true,
            home_directory: format!("/home/{}", username),
            shell: String::from("/bin/sigma-sh"),
        }
    }

    pub fn with_full_name(mut self, name: &str) -> Self {
        self.full_name = String::from(name);
        self
    }

    pub fn with_admin(mut self, admin: bool) -> Self {
        self.is_admin = admin;
        self
    }
}

/// System Configuration
#[derive(Debug, Clone)]
pub struct SystemConfiguration {
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub keyboard_layout: String,
    pub network_config: NetworkConfig,
    pub services: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub use_dhcp: bool,
    pub static_ip: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
}

impl SystemConfiguration {
    pub fn new() -> Self {
        Self {
            hostname: String::from("sigmaos-pc"),
            timezone: String::from("UTC"),
            locale: String::from("en_US.UTF-8"),
            keyboard_layout: String::from("us"),
            network_config: NetworkConfig {
                use_dhcp: true,
                static_ip: None,
                gateway: None,
                dns_servers: vec![String::from("8.8.8.8"), String::from("8.8.4.4")],
            },
            services: vec![
                String::from("networking"),
                String::from("sshd"),
                String::from("cron"),
            ],
        }
    }
}

/// GUI Installer Wizard State
pub struct GuiInstallerWizard {
    pub current_screen: InstallerScreen,
    pub screens_visited: Vec<InstallerScreen>,
    pub disk_info: Vec<DiskInfo>,
    pub selected_disk: Option<String>,
    pub partitioning_operation: PartitioningOperation,
    pub custom_partitions: Vec<PartitionEntry>,
    pub user_accounts: Vec<UserAccount>,
    pub system_config: SystemConfiguration,
    pub installation_progress: u32,
    pub installation_log: Vec<String>,
}

impl GuiInstallerWizard {
    pub fn new() -> Self {
        Self {
            current_screen: InstallerScreen::Welcome,
            screens_visited: Vec::new(),
            disk_info: Vec::new(),
            selected_disk: None,
            partitioning_operation: PartitioningOperation::Automatic,
            custom_partitions: Vec::new(),
            user_accounts: Vec::new(),
            system_config: SystemConfiguration::new(),
            installation_progress: 0,
            installation_log: Vec::new(),
        }
    }

    /// Navigate to next screen
    pub fn next_screen(&mut self) -> Result<(), InstallerError> {
        self.screens_visited.push(self.current_screen);
        
        self.current_screen = match self.current_screen {
            InstallerScreen::Welcome => InstallerScreen::Language,
            InstallerScreen::Language => InstallerScreen::Location,
            InstallerScreen::Location => InstallerScreen::Partitioning,
            InstallerScreen::Partitioning => InstallerScreen::UserSetup,
            InstallerScreen::UserSetup => InstallerScreen::SystemConfiguration,
            InstallerScreen::SystemConfiguration => InstallerScreen::InstallationProgress,
            InstallerScreen::InstallationProgress => InstallerScreen::Complete,
            InstallerScreen::Complete => return Err(InstallerError::AlreadyComplete),
        };

        Ok(())
    }

    /// Navigate to previous screen
    pub fn previous_screen(&mut self) -> Result<(), InstallerError> {
        if let Some(screen) = self.screens_visited.pop() {
            self.current_screen = screen;
            Ok(())
        } else {
            Err(InstallerError::NoPreviousScreen)
        }
    }

    /// Set selected disk for installation
    pub fn select_disk(&mut self, disk: &str) {
        self.selected_disk = Some(String::from(disk));
        self.log(&format!("Selected disk: {}", disk));
    }

    /// Set partitioning operation
    pub fn set_partitioning_operation(&mut self, operation: PartitioningOperation) {
        self.partitioning_operation = operation;
        self.log(&format!("Partitioning operation: {:?}", operation));
    }

    /// Add custom partition
    pub fn add_custom_partition(&mut self, partition: PartitionEntry) {
        self.custom_partitions.push(partition);
        self.log(&format!("Added custom partition: {} -> {}", 
            partition.device, partition.mount_point));
    }

    /// Add user account
    pub fn add_user_account(&mut self, user: UserAccount) {
        self.user_accounts.push(user);
        self.log(&format!("Added user account: {}", user.username));
    }

    /// Update system configuration
    pub fn update_system_config(&mut self, config: SystemConfiguration) {
        self.system_config = config;
        self.log("Updated system configuration");
    }

    /// Start installation process
    pub fn start_installation(&mut self) -> Result<(), InstallerError> {
        if self.selected_disk.is_none() {
            return Err(InstallerError::NoDiskSelected);
        }

        if self.user_accounts.is_empty() {
            return Err(InstallerError::NoUserAccounts);
        }

        self.current_screen = InstallerScreen::InstallationProgress;
        self.installation_progress = 0;
        self.log("Starting installation process");
        Ok(())
    }

    /// Update installation progress
    pub fn update_progress(&mut self, progress: u32) {
        self.installation_progress = progress.min(100);
        self.log(&format!("Installation progress: {}%", self.installation_progress));
    }

    /// Add installation log entry
    pub fn log(&mut self, message: &str) {
        self.installation_log.push(String::from(message));
    }

    /// Get current screen description
    pub fn get_screen_description(&self) -> &str {
        match self.current_screen {
            InstallerScreen::Welcome => "Welcome to SigmaOS Installer",
            InstallerScreen::Language => "Select your language",
            InstallerScreen::Location => "Select your location and timezone",
            InstallerScreen::Partitioning => "Configure disk partitioning",
            InstallerScreen::UserSetup => "Create user accounts",
            InstallerScreen::SystemConfiguration => "Configure system settings",
            InstallerScreen::InstallationProgress => "Installing SigmaOS",
            InstallerScreen::Complete => "Installation Complete",
        }
    }

    /// Validate current screen
    pub fn validate_current_screen(&self) -> Result<(), InstallerError> {
        match self.current_screen {
            InstallerScreen::Partitioning => {
                if self.selected_disk.is_none() {
                    return Err(InstallerError::NoDiskSelected);
                }
            }
            InstallerScreen::UserSetup => {
                if self.user_accounts.is_empty() {
                    return Err(InstallerError::NoUserAccounts);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Get installation summary
    pub fn get_installation_summary(&self) -> InstallationSummary {
        InstallationSummary {
            target_disk: self.selected_disk.clone().unwrap_or_default(),
            partitioning_operation: self.partitioning_operation,
            user_count: self.user_accounts.len(),
            hostname: self.system_config.hostname.clone(),
            filesystem: match self.partitioning_operation {
                PartitioningOperation::Automatic => FilesystemType::Btrfs,
                _ => FilesystemType::Ext4,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstallationSummary {
    pub target_disk: String,
    pub partitioning_operation: PartitioningOperation,
    pub user_count: usize,
    pub hostname: String,
    pub filesystem: FilesystemType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallerError {
    NoDiskSelected,
    NoUserAccounts,
    AlreadyComplete,
    NoPreviousScreen,
    PartitioningFailed,
    InstallationFailed,
    InvalidConfiguration,
}

/// Advanced Partitioning Calculator
pub struct PartitioningCalculator {
    pub disk_size_mb: u64,
    pub swap_size_mb: u64,
    pub boot_size_mb: u64,
    pub root_size_mb: u64,
    pub home_size_mb: u64,
}

impl PartitioningCalculator {
    pub fn new(disk_size_mb: u64) -> Self {
        let swap_size_mb = if disk_size_mb >= 8192 { 4096 } else { 2048 };
        let boot_size_mb = 512;
        let root_size_mb = (disk_size_mb - swap_size_mb - boot_size_mb) / 3;
        let home_size_mb = disk_size_mb - swap_size_mb - boot_size_mb - root_size_mb;

        Self {
            disk_size_mb,
            swap_size_mb,
            boot_size_mb,
            root_size_mb,
            home_size_mb,
        }
    }

    pub fn calculate_automatic_layout(&self) -> Vec<PartitionEntry> {
        let mut partitions = Vec::new();
        let mut current_sector = 2048; // Start after MBR

        // Boot partition
        partitions.push(PartitionEntry::new("/dev/sda1", self.boot_size_mb, FilesystemType::Ext4, "/boot")
            .with_flag("boot"));

        current_sector += (self.boot_size_mb * 2048); // Convert MB to sectors

        // Swap partition
        partitions.push(PartitionEntry::new("/dev/sda2", self.swap_size_mb, FilesystemType::Swap, "swap"));

        current_sector += (self.swap_size_mb * 2048);

        // Root partition
        partitions.push(PartitionEntry::new("/dev/sda3", self.root_size_mb, FilesystemType::Btrfs, "/"));

        current_sector += (self.root_size_mb * 2048);

        // Home partition
        partitions.push(PartitionEntry::new("/dev/sda4", self.home_size_mb, FilesystemType::Btrfs, "/home"));

        partitions
    }

    pub fn validate_layout(&self, partitions: &[PartitionEntry]) -> Result<(), &'static str> {
        let total_size: u64 = partitions.iter().map(|p| p.size_mb).sum();
        
        if total_size > self.disk_size_mb {
            return Err("Total partition size exceeds disk capacity");
        }

        let has_root = partitions.iter().any(|p| p.mount_point == "/");
        let has_boot = partitions.iter().any(|p| p.mount_point == "/boot");
        
        if !has_root {
            return Err("Missing root partition");
        }

        if !has_boot {
            return Err("Missing boot partition");
        }

        Ok(())
    }
}

/// Theme Configuration for Installer
#[derive(Debug, Clone)]
pub struct InstallerTheme {
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
    pub accent_color: String,
}

impl InstallerTheme {
    pub fn new() -> Self {
        Self {
            primary_color: String::from("#2C3E50"),
            secondary_color: String::from("#3498DB"),
            background_color: String::from("#ECF0F1"),
            text_color: String::from("#2C3E50"),
            accent_color: String::from("#E74C3C"),
        }
    }

    pub fn dark_theme() -> Self {
        Self {
            primary_color: String::from("#1a1a1a"),
            secondary_color: String::from("#4a90e2"),
            background_color: String::from("#2d2d2d"),
            text_color: String::from("#ffffff"),
            accent_color: String::from("#ff6b6b"),
        }
    }
}

/// Installer Accessibility Settings
#[derive(Debug, Clone)]
pub struct AccessibilitySettings {
    pub high_contrast: bool,
    pub large_text: bool,
    pub screen_reader: bool,
    pub reduced_motion: bool,
}

impl AccessibilitySettings {
    pub fn new() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            screen_reader: false,
            reduced_motion: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_wizard_navigation() {
        let mut wizard = GuiInstallerWizard::new();
        assert_eq!(wizard.current_screen, InstallerScreen::Welcome);
        
        assert!(wizard.next_screen().is_ok());
        assert_eq!(wizard.current_screen, InstallerScreen::Language);
    }

    #[test]
    fn test_partition_entry_creation() {
        let partition = PartitionEntry::new("/dev/sda1", 512, FilesystemType::Ext4, "/boot")
            .with_flag("boot");
        
        assert_eq!(partition.mount_point, "/boot");
        assert!(partition.flags.contains(&String::from("boot")));
    }

    #[test]
    fn test_disk_info() {
        let mut disk = DiskInfo::new("/dev/sda", 102400, "Test Disk");
        disk.add_partition(PartitionEntry::new("/dev/sda1", 512, FilesystemType::Ext4, "/boot"));
        
        assert_eq!(disk.partitions.len(), 1);
        assert!(disk.get_free_space() > 0);
    }

    #[test]
    fn test_user_account() {
        let user = UserAccount::new("testuser", "password123")
            .with_full_name("Test User")
            .with_admin(true);
        
        assert_eq!(user.username, "testuser");
        assert!(user.is_admin);
    }

    #[test]
    fn test_partitioning_calculator() {
        let calculator = PartitioningCalculator::new(102400); // 100GB
        let layout = calculator.calculate_automatic_layout();
        
        assert!(!layout.is_empty());
        assert!(calculator.validate_layout(&layout).is_ok());
    }

    #[test]
    fn test_installer_theme() {
        let theme = InstallerTheme::dark_theme();
        assert_eq!(theme.background_color, "#2d2d2d");
    }

    #[test]
    fn test_installation_summary() {
        let mut wizard = GuiInstallerWizard::new();
        wizard.select_disk("/dev/sda");
        wizard.add_user_account(UserAccount::new("user", "pass"));
        
        let summary = wizard.get_installation_summary();
        assert_eq!(summary.target_disk, "/dev/sda");
        assert_eq!(summary.user_count, 1);
    }
}