// SigmaOS Advanced GUI Installer Wizard
// Calamares-inspired graphical installer wizard with dual-boot alongside partitioning

#![no_std]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

/// Installer Screen / Calamares Module Sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerScreen {
    Welcome,
    Language,
    Location,
    Keyboard,
    Partitioning,
    UserSetup,
    SystemConfiguration,
    Summary,
    InstallationProgress,
    Complete,
}

/// Partitioning Operation Strategy
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
    Ntfs,
    Fat32,
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

/// Co-Resident Detected Operating System for Dual-Boot
#[derive(Debug, Clone)]
pub struct DetectedOperatingSystem {
    pub name: String,
    pub device_partition: String,
    pub filesystem: FilesystemType,
    pub total_size_mb: u64,
    pub free_space_mb: u64,
    pub min_shrink_mb: u64,
}

impl DetectedOperatingSystem {
    pub fn new(
        name: &str,
        device_partition: &str,
        fs: FilesystemType,
        total_mb: u64,
        free_mb: u64,
    ) -> Self {
        let min_shrink_mb = free_mb.saturating_sub(10240); // Keep 10GB margin
        Self {
            name: String::from(name),
            device_partition: String::from(device_partition),
            filesystem: fs,
            total_size_mb: total_mb,
            free_space_mb: free_mb,
            min_shrink_mb,
        }
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
    pub auto_login: bool,
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
            auto_login: false,
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

    pub fn with_auto_login(mut self, auto: bool) -> Self {
        self.auto_login = auto;
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

/// GUI Calamares-Style Installer Wizard Engine
pub struct GuiInstallerWizard {
    pub current_screen: InstallerScreen,
    pub screens_visited: Vec<InstallerScreen>,
    pub disk_info: Vec<DiskInfo>,
    pub detected_operating_systems: Vec<DetectedOperatingSystem>,
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
        let mut wizard = Self {
            current_screen: InstallerScreen::Welcome,
            screens_visited: Vec::new(),
            disk_info: Vec::new(),
            detected_operating_systems: Vec::new(),
            selected_disk: None,
            partitioning_operation: PartitioningOperation::Automatic,
            custom_partitions: Vec::new(),
            user_accounts: Vec::new(),
            system_config: SystemConfiguration::new(),
            installation_progress: 0,
            installation_log: Vec::new(),
        };
        wizard.scan_hardware_and_os();
        wizard
    }

    /// Scans co-resident operating systems for Calamares dual-boot alongside mode
    pub fn scan_hardware_and_os(&mut self) {
        let mut nvme = DiskInfo::new("/dev/nvme0n1", 512000, "Samsung NVMe SSD 512GB");
        nvme.add_partition(PartitionEntry::new(
            "/dev/nvme0n1p1",
            512,
            FilesystemType::Fat32,
            "/boot/efi",
        ).with_flag("esp"));
        nvme.add_partition(PartitionEntry::new(
            "/dev/nvme0n1p2",
            250000,
            FilesystemType::Ntfs,
            "",
        ));

        self.disk_info.push(nvme);

        self.detected_operating_systems.push(DetectedOperatingSystem::new(
            "Windows 11 Home",
            "/dev/nvme0n1p2",
            FilesystemType::Ntfs,
            250000,
            120000,
        ));
        self.detected_operating_systems.push(DetectedOperatingSystem::new(
            "Ubuntu 24.04 LTS",
            "/dev/sda2",
            FilesystemType::Ext4,
            100000,
            60000,
        ));
    }

    /// Navigate to next screen in Calamares module sequence
    pub fn next_screen(&mut self) -> Result<(), InstallerError> {
        self.screens_visited.push(self.current_screen);

        self.current_screen = match self.current_screen {
            InstallerScreen::Welcome => InstallerScreen::Language,
            InstallerScreen::Language => InstallerScreen::Location,
            InstallerScreen::Location => InstallerScreen::Keyboard,
            InstallerScreen::Keyboard => InstallerScreen::Partitioning,
            InstallerScreen::Partitioning => InstallerScreen::UserSetup,
            InstallerScreen::UserSetup => InstallerScreen::SystemConfiguration,
            InstallerScreen::SystemConfiguration => InstallerScreen::Summary,
            InstallerScreen::Summary => InstallerScreen::InstallationProgress,
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

    /// Set partitioning operation (e.g. Alongside for Dual-Boot)
    pub fn set_partitioning_operation(&mut self, operation: PartitioningOperation) {
        self.partitioning_operation = operation;
        self.log(&format!("Partitioning operation: {:?}", operation));
    }

    /// Calculate Dual-Boot Alongside partitioning layout
    pub fn calculate_alongside_layout(&mut self, target_os_partition: &str, allocate_sigma_mb: u64) -> Result<Vec<PartitionEntry>, InstallerError> {
        let target_os = self
            .detected_operating_systems
            .iter()
            .find(|os| os.device_partition == target_os_partition)
            .ok_or(InstallerError::InvalidConfiguration)?;

        if allocate_sigma_mb > target_os.min_shrink_mb {
            return Err(InstallerError::PartitioningFailed);
        }

        let mut partitions = Vec::new();
        // 1. Shrunk OS partition
        let remaining_os_mb = target_os.total_size_mb - allocate_sigma_mb;
        partitions.push(PartitionEntry::new(
            &target_os.device_partition,
            remaining_os_mb,
            target_os.filesystem,
            "preserves_existing_os",
        ));

        // 2. SigmaOS ESP EFI Partition
        partitions.push(PartitionEntry::new(
            "/dev/nvme0n1p3",
            512,
            FilesystemType::Fat32,
            "/boot/efi",
        ).with_flag("boot").with_flag("esp"));

        // 3. SigmaOS Root Partition
        let root_mb = allocate_sigma_mb.saturating_sub(4512);
        partitions.push(PartitionEntry::new(
            "/dev/nvme0n1p4",
            root_mb,
            FilesystemType::Btrfs,
            "/",
        ));

        // 4. Swap Partition
        partitions.push(PartitionEntry::new(
            "/dev/nvme0n1p5",
            4000,
            FilesystemType::Swap,
            "swap",
        ));

        self.custom_partitions = partitions.clone();
        self.partitioning_operation = PartitioningOperation::Alongside;
        self.log(&format!(
            "Configured Dual-Boot Alongside OS: {} (Allocated {}MB for SigmaOS)",
            target_os.name, allocate_sigma_mb
        ));

        Ok(partitions)
    }

    /// Add custom partition
    pub fn add_custom_partition(&mut self, partition: PartitionEntry) {
        self.custom_partitions.push(partition);
        self.log(&format!(
            "Added custom partition: {} -> {}",
            partition.device, partition.mount_point
        ));
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
        self.log("Starting Calamares installation execution pipeline");
        Ok(())
    }

    /// Update installation progress
    pub fn update_progress(&mut self, progress: u32) {
        self.installation_progress = progress.min(100);
        self.log(&format!(
            "Installation progress: {}%",
            self.installation_progress
        ));
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
            InstallerScreen::Keyboard => "Select keyboard layout",
            InstallerScreen::Partitioning => "Configure disk partitioning & dual-boot alongside setup",
            InstallerScreen::UserSetup => "Create user accounts",
            InstallerScreen::SystemConfiguration => "Configure system settings",
            InstallerScreen::Summary => "Review installation summary before committing",
            InstallerScreen::InstallationProgress => "Installing SigmaOS",
            InstallerScreen::Complete => "Installation Complete",
        }
    }

    /// Get installation summary
    pub fn get_installation_summary(&self) -> InstallationSummary {
        InstallationSummary {
            target_disk: self.selected_disk.clone().unwrap_or_default(),
            partitioning_operation: self.partitioning_operation,
            user_count: self.user_accounts.len(),
            hostname: self.system_config.hostname.clone(),
            filesystem: match self.partitioning_operation {
                PartitioningOperation::Automatic | PartitioningOperation::Alongside => {
                    FilesystemType::Btrfs
                }
                _ => FilesystemType::Ext4,
            },
            dual_boot_detected: !self.detected_operating_systems.is_empty(),
        }
    }
}

impl Default for GuiInstallerWizard {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct InstallationSummary {
    pub target_disk: String,
    pub partitioning_operation: PartitioningOperation,
    pub user_count: usize,
    pub hostname: String,
    pub filesystem: FilesystemType,
    pub dual_boot_detected: bool,
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

        // Boot partition
        partitions.push(
            PartitionEntry::new("/dev/sda1", self.boot_size_mb, FilesystemType::Ext4, "/boot")
                .with_flag("boot"),
        );

        // Swap partition
        partitions.push(PartitionEntry::new(
            "/dev/sda2",
            self.swap_size_mb,
            FilesystemType::Swap,
            "swap",
        ));

        // Root partition
        partitions.push(PartitionEntry::new(
            "/dev/sda3",
            self.root_size_mb,
            FilesystemType::Btrfs,
            "/",
        ));

        // Home partition
        partitions.push(PartitionEntry::new(
            "/dev/sda4",
            self.home_size_mb,
            FilesystemType::Btrfs,
            "/home",
        ));

        partitions
    }

    pub fn validate_layout(&self, partitions: &[PartitionEntry]) -> Result<(), &'static str> {
        let total_size: u64 = partitions.iter().map(|p| p.size_mb).sum();

        if total_size > self.disk_size_mb {
            return Err("Total partition size exceeds disk capacity");
        }

        let has_root = partitions.iter().any(|p| p.mount_point == "/");
        if !has_root {
            return Err("Missing root partition");
        }

        Ok(())
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
    fn test_dual_boot_alongside_calculation() {
        let mut wizard = GuiInstallerWizard::new();
        assert!(!wizard.detected_operating_systems.is_empty());

        let layout = wizard.calculate_alongside_layout("/dev/nvme0n1p2", 50000).unwrap();
        assert_eq!(layout.len(), 4);
        assert_eq!(wizard.partitioning_operation, PartitioningOperation::Alongside);
    }

    #[test]
    fn test_installation_summary() {
        let mut wizard = GuiInstallerWizard::new();
        wizard.select_disk("/dev/nvme0n1");
        wizard.add_user_account(UserAccount::new("sovereign", "secret123"));

        let summary = wizard.get_installation_summary();
        assert_eq!(summary.target_disk, "/dev/nvme0n1");
        assert!(summary.dual_boot_detected);
    }
}
