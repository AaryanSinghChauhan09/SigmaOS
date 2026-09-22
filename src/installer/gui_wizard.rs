// SigmaOS Advanced GUI Installer Wizard & Modular Setup Configurator
// Calamares-inspired graphical installer wizard with persona setup flow and dual-boot alongside partitioning

use std::string::String;
use std::vec::Vec;
use std::vec;

/// Installer Screen / Calamares Module Sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerScreen {
    Welcome,
    Language,
    Location,
    Keyboard,
    Partitioning,
    PersonaSelection,
    ModuleChooser,
    DesktopChoiceScreen,
    PackageProfileScreen,
    UserSetup,
    SystemConfiguration,
    Summary,
    InstallationProgress,
    Complete,
    CompleteOnboarding,
}

pub type InstallerStep = InstallerScreen;
pub type UserAccountConfig = UserAccount;

/// Network configuration for installer
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub use_dhcp: bool,
    pub static_ip: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
}

/// Partitioning Operation Strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionStrategy {
    EraseDisk,
    InstallAlongsideExisting,
    ManualCustomPartitions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitioningOperation {
    Automatic,
    EraseDisk,
    InstallAlongside,
    Alongside,
    Manual,
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
    pub is_admin: bool,
    pub auto_login: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct PrivacySettings {
    pub enable_telemetry: bool,
    pub send_crash_reports: bool,
    pub location_services: bool,
}

impl SystemConfiguration {
    pub fn new() -> Self {
        Self {
            hostname: String::from("sigmaos-pc"),
            is_admin: true,
            auto_login: false,
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

// =========================================================================
// MODULAR SETUP CONFIGURATOR ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerPersona {
    Developer,
    Compliance,
    Student,
    Gaming,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupModule {
    Productivity,
    Media,
    Networking,
    Recovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopChoice {
    ZenithDefault,
    XfceLightweight,
    GnomeStandard,
    KdePlasmaFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageProfileTier {
    Minimal,
    Standard,
    Full,
}

#[derive(Debug, Clone)]
pub struct HardwareOptimizerSuggestion {
    pub recommended_desktop: DesktopChoice,
    pub recommended_profile: PackageProfileTier,
    pub is_low_spec: bool,
    pub explanation: String,
}

pub struct ModularInstallerSetupConfigurator {
    pub persona: InstallerPersona,
    pub active_modules: Vec<SetupModule>,
    pub desktop_environment: DesktopChoice,
    pub package_profile: PackageProfileTier,
    pub enable_selinux_hardening: bool,
    pub enable_compliance_dashboards: bool,
}

impl ModularInstallerSetupConfigurator {
    pub fn new() -> Self {
        Self {
            persona: InstallerPersona::Minimal,
            active_modules: vec![SetupModule::Recovery],
            desktop_environment: DesktopChoice::ZenithDefault,
            package_profile: PackageProfileTier::Minimal,
            enable_selinux_hardening: true,
            enable_compliance_dashboards: false,
        }
    }

    pub fn auto_detect_hardware_optimization(ram_mb: u64, cpu_cores: usize) -> HardwareOptimizerSuggestion {
        if ram_mb < 3072 || cpu_cores <= 2 {
            HardwareOptimizerSuggestion {
                recommended_desktop: DesktopChoice::ZenithDefault,
                recommended_profile: PackageProfileTier::Minimal,
                is_low_spec: true,
                explanation: format!("Low-spec detected ({}MB RAM, {} cores). Recommending Zenith Minimal profile.", ram_mb, cpu_cores),
            }
        } else {
            HardwareOptimizerSuggestion {
                recommended_desktop: DesktopChoice::ZenithDefault,
                recommended_profile: PackageProfileTier::Standard,
                is_low_spec: false,
                explanation: format!("Standard hardware detected ({}MB RAM, {} cores). Recommending Zenith Standard profile.", ram_mb, cpu_cores),
            }
        }
    }

    pub fn configure_persona(&mut self, persona: InstallerPersona) {
        self.persona = persona;
        match persona {
            InstallerPersona::Developer => {
                self.active_modules = vec![SetupModule::Productivity, SetupModule::Networking, SetupModule::Recovery];
                self.package_profile = PackageProfileTier::Standard;
            }
            InstallerPersona::Compliance => {
                self.active_modules = vec![SetupModule::Productivity, SetupModule::Recovery];
                self.package_profile = PackageProfileTier::Standard;
                self.enable_compliance_dashboards = true;
                self.enable_selinux_hardening = true;
            }
            InstallerPersona::Student => {
                self.active_modules = vec![SetupModule::Productivity, SetupModule::Media];
                self.package_profile = PackageProfileTier::Standard;
            }
            InstallerPersona::Gaming => {
                self.active_modules = vec![SetupModule::Media, SetupModule::Networking];
                self.package_profile = PackageProfileTier::Full;
                self.desktop_environment = DesktopChoice::KdePlasmaFull;
            }
            InstallerPersona::Minimal => {
                self.active_modules = vec![SetupModule::Recovery];
                self.package_profile = PackageProfileTier::Minimal;
            }
        }
    }
}

impl Default for ModularInstallerSetupConfigurator {
    fn default() -> Self {
        Self::new()
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
    pub setup_configurator: ModularInstallerSetupConfigurator,
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
            setup_configurator: ModularInstallerSetupConfigurator::new(),
            installation_progress: 0,
            installation_log: Vec::new(),
        };
        wizard.scan_hardware_and_os();
        wizard
    }

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

    pub fn next_screen(&mut self) -> Result<(), InstallerError> {
        self.screens_visited.push(self.current_screen);

        self.current_screen = match self.current_screen {
            InstallerScreen::Welcome => InstallerScreen::Language,
            InstallerScreen::Language => InstallerScreen::Location,
            InstallerScreen::Location => InstallerScreen::Keyboard,
            InstallerScreen::Keyboard => InstallerScreen::Partitioning,
            InstallerScreen::Partitioning => InstallerScreen::PersonaSelection,
            InstallerScreen::PersonaSelection => InstallerScreen::ModuleChooser,
            InstallerScreen::ModuleChooser => InstallerScreen::DesktopChoiceScreen,
            InstallerScreen::DesktopChoiceScreen => InstallerScreen::PackageProfileScreen,
            InstallerScreen::PackageProfileScreen => InstallerScreen::UserSetup,
            InstallerScreen::UserSetup => InstallerScreen::SystemConfiguration,
            InstallerScreen::SystemConfiguration => InstallerScreen::Summary,
            InstallerScreen::Summary => InstallerScreen::InstallationProgress,
            InstallerScreen::InstallationProgress => InstallerScreen::Complete,
            InstallerScreen::Complete | InstallerScreen::CompleteOnboarding => return Err(InstallerError::AlreadyComplete),
        };

        Ok(())
    }

    pub fn previous_screen(&mut self) -> Result<(), InstallerError> {
        if let Some(screen) = self.screens_visited.pop() {
            self.current_screen = screen;
            Ok(())
        } else {
            Err(InstallerError::NoPreviousScreen)
        }
    }

    pub fn select_disk(&mut self, disk: &str) {
        self.selected_disk = Some(String::from(disk));
        self.log(&format!("Selected disk: {}", disk));
    }

    pub fn set_partitioning_operation(&mut self, operation: PartitioningOperation) {
        self.partitioning_operation = operation;
        self.log(&format!("Partitioning operation: {:?}", operation));
    }

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
        let remaining_os_mb = target_os.total_size_mb - allocate_sigma_mb;
        partitions.push(PartitionEntry::new(
            &target_os.device_partition,
            remaining_os_mb,
            target_os.filesystem,
            "preserves_existing_os",
        ));

        partitions.push(PartitionEntry::new(
            "/dev/nvme0n1p3",
            512,
            FilesystemType::Fat32,
            "/boot/efi",
        ).with_flag("boot").with_flag("esp"));

        let root_mb = allocate_sigma_mb.saturating_sub(4512);
        partitions.push(PartitionEntry::new(
            "/dev/nvme0n1p4",
            root_mb,
            FilesystemType::Btrfs,
            "/",
        ));

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

    pub fn add_custom_partition(&mut self, partition: PartitionEntry) {
        self.log(&format!(
            "Added custom partition: {} -> {}",
            partition.device, partition.mount_point
        ));
        self.custom_partitions.push(partition);
    }

    pub fn add_user_account(&mut self, user: UserAccount) {
        self.log(&format!("Added user account: {}", user.username));
        self.user_accounts.push(user);
    }

    pub fn update_system_config(&mut self, config: SystemConfiguration) {
        self.system_config = config;
        self.log("Updated system configuration");
    }

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

    pub fn update_progress(&mut self, progress: u32) {
        self.installation_progress = progress.min(100);
        self.log(&format!(
            "Installation progress: {}%",
            self.installation_progress
        ));
    }

    pub fn log(&mut self, message: &str) {
        self.installation_log.push(String::from(message));
    }

    pub fn get_screen_description(&self) -> &str {
        match self.current_screen {
            InstallerScreen::Welcome => "Welcome to SigmaOS Installer",
            InstallerScreen::Language => "Select your language",
            InstallerScreen::Location => "Select your location and timezone",
            InstallerScreen::Keyboard => "Select keyboard layout",
            InstallerScreen::Partitioning => "Configure disk partitioning & dual-boot alongside setup",
            InstallerScreen::PersonaSelection => "Select user persona profile",
            InstallerScreen::ModuleChooser => "Choose optional OS feature modules",
            InstallerScreen::DesktopChoiceScreen => "Select desktop environment",
            InstallerScreen::PackageProfileScreen => "Select package profile tier",
            InstallerScreen::UserSetup => "Create user accounts",
            InstallerScreen::SystemConfiguration => "Configure system settings",
            InstallerScreen::Summary => "Review installation summary before committing",
            InstallerScreen::InstallationProgress => "Installing SigmaOS",
            InstallerScreen::Complete | InstallerScreen::CompleteOnboarding => "Installation Complete",
        }
    }

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

        partitions.push(
            PartitionEntry::new("/dev/sda1", self.boot_size_mb, FilesystemType::Ext4, "/boot")
                .with_flag("boot"),
        );

        partitions.push(PartitionEntry::new(
            "/dev/sda2",
            self.swap_size_mb,
            FilesystemType::Swap,
            "swap",
        ));

        partitions.push(PartitionEntry::new(
            "/dev/sda3",
            self.root_size_mb,
            FilesystemType::Btrfs,
            "/",
        ));

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
    fn test_gui_wizard_step_navigation() {
        let mut wizard = GuiInstallerWizard::new();
        assert_eq!(wizard.current_screen, InstallerScreen::Welcome);

        assert!(wizard.next_screen().is_ok());
        assert_eq!(wizard.current_screen, InstallerScreen::Language);
    }

    #[test]
    fn test_modular_setup_configurator() {
        let mut config = ModularInstallerSetupConfigurator::new();
        config.configure_persona(InstallerPersona::Developer);

        assert_eq!(config.persona, InstallerPersona::Developer);
        assert!(config.active_modules.contains(&SetupModule::Productivity));

        let hardware_opt = ModularInstallerSetupConfigurator::auto_detect_hardware_optimization(2048, 2);
        assert!(hardware_opt.is_low_spec);
        assert_eq!(hardware_opt.recommended_desktop, DesktopChoice::ZenithDefault);
    }
}
