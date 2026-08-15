// SigmaOS GUI Installer Wizard
// Inspired by Debian installer, Arch Linux guided installation, and Ubuntu ubiquity
// Supports preseed configuration, automated installation, and comprehensive user guidance

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct InstallerConfig {
    pub language: String,
    pub keyboard_layout: String,
    pub timezone: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub disk: String,
    pub partition_scheme: PartitionScheme,
    pub filesystem: FilesystemType,
    pub boot_loader: BootLoader,
    pub desktop_environment: Option<String>,
    pub packages: Vec<String>,
    pub network_config: NetworkConfig,
    pub encryption: bool,
    pub swap_size: u64, // MB
    pub preseed_file: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum PartitionScheme {
    Auto,
    Manual,
    LVM,
    BtrfsSubvolumes,
}

#[derive(Debug, Clone, Copy)]
pub enum FilesystemType {
    Ext4,
    Btrfs,
    XFS,
    ZFS,
}

#[derive(Debug, Clone, Copy)]
pub enum BootLoader {
    GRUB,
    SystemdBoot,
    None,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub use_dhcp: bool,
    pub static_ip: Option<String>,
    pub gateway: Option<String>,
    pub dns: Vec<String>,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        InstallerConfig {
            language: "en_US".to_string(),
            keyboard_layout: "us".to_string(),
            timezone: "UTC".to_string(),
            hostname: "sigmaos".to_string(),
            username: "sigma".to_string(),
            password: "sigma".to_string(),
            disk: "/dev/sda".to_string(),
            partition_scheme: PartitionScheme::Auto,
            filesystem: FilesystemType::Ext4,
            boot_loader: BootLoader::GRUB,
            desktop_environment: Some("zenith".to_string()),
            packages: vec![],
            network_config: NetworkConfig {
                use_dhcp: true,
                static_ip: None,
                gateway: None,
                dns: vec![],
            },
            encryption: false,
            swap_size: 2048,
            preseed_file: None,
        }
    }
}

pub struct SigmaInstaller {
    config: InstallerConfig,
    current_step: InstallerStep,
    steps: Vec<InstallerStep>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerStep {
    Welcome,
    Language,
    Keyboard,
    Network,
    DiskSelection,
    Partitioning,
    Filesystem,
    UserAccount,
    BootLoader,
    PackageSelection,
    Installation,
    Complete,
}

impl SigmaInstaller {
    pub fn new() -> Self {
        SigmaInstaller {
            config: InstallerConfig::default(),
            current_step: InstallerStep::Welcome,
            steps: vec![
                InstallerStep::Welcome,
                InstallerStep::Language,
                InstallerStep::Keyboard,
                InstallerStep::Network,
                InstallerStep::DiskSelection,
                InstallerStep::Partitioning,
                InstallerStep::Filesystem,
                InstallerStep::UserAccount,
                InstallerStep::BootLoader,
                InstallerStep::PackageSelection,
                InstallerStep::Installation,
                InstallerStep::Complete,
            ],
        }
    }

    pub fn with_preseed(path: &str) -> Result<Self, String> {
        let mut installer = Self::new();
        installer.load_preseed(path)?;
        Ok(installer)
    }

    pub fn load_preseed(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read preseed file: {}", e))?;
        
        // Parse preseed configuration (simplified key=value format)
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                self.apply_preseed_setting(key.trim(), value.trim());
            }
        }
        
        self.config.preseed_file = Some(path.to_string());
        Ok(())
    }

    fn apply_preseed_setting(&mut self, key: &str, value: &str) {
        match key {
            "language" => self.config.language = value.to_string(),
            "keyboard" => self.config.keyboard_layout = value.to_string(),
            "timezone" => self.config.timezone = value.to_string(),
            "hostname" => self.config.hostname = value.to_string(),
            "username" => self.config.username = value.to_string(),
            "password" => self.config.password = value.to_string(),
            "disk" => self.config.disk = value.to_string(),
            "partition_scheme" => {
                self.config.partition_scheme = match value {
                    "auto" => PartitionScheme::Auto,
                    "manual" => PartitionScheme::Manual,
                    "lvm" => PartitionScheme::LVM,
                    "btrfs" => PartitionScheme::BtrfsSubvolumes,
                    _ => PartitionScheme::Auto,
                };
            }
            "filesystem" => {
                self.config.filesystem = match value {
                    "ext4" => FilesystemType::Ext4,
                    "btrfs" => FilesystemType::Btrfs,
                    "xfs" => FilesystemType::XFS,
                    "zfs" => FilesystemType::ZFS,
                    _ => FilesystemType::Ext4,
                };
            }
            "boot_loader" => {
                self.config.boot_loader = match value {
                    "grub" => BootLoader::GRUB,
                    "systemd" => BootLoader::SystemdBoot,
                    "none" => BootLoader::None,
                    _ => BootLoader::GRUB,
                };
            }
            "encryption" => self.config.encryption = value == "true",
            "swap_size" => {
                self.config.swap_size = value.parse().unwrap_or(2048);
            }
            "use_dhcp" => {
                self.config.network_config.use_dhcp = value == "true";
            }
            "desktop" => {
                self.config.desktop_environment = if value == "none" {
                    None
                } else {
                    Some(value.to_string())
                };
            }
            _ => {} // Ignore unknown keys
        }
    }

    pub fn get_current_step(&self) -> InstallerStep {
        self.current_step
    }

    pub fn next_step(&mut self) -> Option<InstallerStep> {
        let current_index = self.steps.iter().position(|&s| s == self.current_step)?;
        if current_index + 1 < self.steps.len() {
            self.current_step = self.steps[current_index + 1];
            Some(self.current_step)
        } else {
            None
        }
    }

    pub fn previous_step(&mut self) -> Option<InstallerStep> {
        let current_index = self.steps.iter().position(|&s| s == self.current_step)?;
        if current_index > 0 {
            self.current_step = self.steps[current_index - 1];
            Some(self.current_step)
        } else {
            None
        }
    }

    pub fn get_step_title(&self, step: InstallerStep) -> &'static str {
        match step {
            InstallerStep::Welcome => "Welcome to SigmaOS",
            InstallerStep::Language => "Select Language",
            InstallerStep::Keyboard => "Configure Keyboard",
            InstallerStep::Network => "Configure Network",
            InstallerStep::DiskSelection => "Select Installation Disk",
            InstallerStep::Partitioning => "Partition Disk",
            InstallerStep::Filesystem => "Configure Filesystem",
            InstallerStep::UserAccount => "Create User Account",
            InstallerStep::BootLoader => "Install Boot Loader",
            InstallerStep::PackageSelection => "Select Packages",
            InstallerStep::Installation => "Installing SigmaOS",
            InstallerStep::Complete => "Installation Complete",
        }
    }

    pub fn get_step_description(&self, step: InstallerStep) -> &'static str {
        match step {
            InstallerStep::Welcome => "This wizard will guide you through the installation of SigmaOS.",
            InstallerStep::Language => "Select your preferred language for the installation.",
            InstallerStep::Keyboard => "Configure your keyboard layout.",
            InstallerStep::Network => "Configure network settings for your system.",
            InstallerStep::DiskSelection => "Select the disk where SigmaOS will be installed.",
            InstallerStep::Partitioning => "Choose how to partition your disk.",
            InstallerStep::Filesystem => "Select the filesystem type for your partitions.",
            InstallerStep::UserAccount => "Create a user account for your system.",
            InstallerStep::BootLoader => "Configure the boot loader for your system.",
            InstallerStep::PackageSelection => "Select additional packages to install.",
            InstallerStep::Installation => "SigmaOS is being installed. Please wait...",
            InstallerStep::Complete => "Installation complete! You can now reboot into your new system.",
        }
    }

    pub fn execute_installation(&self) -> Result<(), String> {
        // Implementation would involve:
        // 1. Partition disk according to scheme
        // 2. Create filesystems
        // 3. Install base system
        // 4. Configure bootloader
        // 5. Configure network
        // 6. Create user accounts
        // 7. Install selected packages
        
        println!("Starting installation with configuration:");
        println!("  Disk: {}", self.config.disk);
        println!("  Partition Scheme: {:?}", self.config.partition_scheme);
        println!("  Filesystem: {:?}", self.config.filesystem);
        println!("  Boot Loader: {:?}", self.config.boot_loader);
        
        // Simulate installation steps
        self.partition_disk()?;
        self.create_filesystems()?;
        self.install_base_system()?;
        self.configure_bootloader()?;
        self.configure_network()?;
        self.create_user_account()?;
        self.install_packages()?;
        
        Ok(())
    }

    fn partition_disk(&self) -> Result<(), String> {
        println!("Partitioning disk {}...", self.config.disk);
        // Implementation would use fdisk/parted based on partition scheme
        Ok(())
    }

    fn create_filesystems(&self) -> Result<(), String> {
        println!("Creating filesystems...");
        // Implementation would use mkfs based on filesystem type
        Ok(())
    }

    fn install_base_system(&self) -> Result<(), String> {
        println!("Installing base system...");
        // Implementation would copy kernel and userspace
        Ok(())
    }

    fn configure_bootloader(&self) -> Result<(), String> {
        println!("Configuring bootloader...");
        // Implementation would install GRUB or systemd-boot
        Ok(())
    }

    fn configure_network(&self) -> Result<(), String> {
        println!("Configuring network...");
        // Implementation would create network configuration files
        Ok(())
    }

    fn create_user_account(&self) -> Result<(), String> {
        println!("Creating user account: {}", self.config.username);
        // Implementation would use useradd/usermod
        Ok(())
    }

    fn install_packages(&self) -> Result<(), String> {
        println!("Installing packages: {:?}", self.config.packages);
        // Implementation would use package manager
        Ok(())
    }

    pub fn generate_preseed(&self) -> String {
        let mut preseed = String::new();
        preseed.push_str("# SigmaOS Preseed Configuration\n");
        preseed.push_str(&format!("language={}\n", self.config.language));
        preseed.push_str(&format!("keyboard={}\n", self.config.keyboard_layout));
        preseed.push_str(&format!("timezone={}\n", self.config.timezone));
        preseed.push_str(&format!("hostname={}\n", self.config.hostname));
        preseed.push_str(&format!("username={}\n", self.config.username));
        preseed.push_str(&format!("password={}\n", self.config.password));
        preseed.push_str(&format!("disk={}\n", self.config.disk));
        preseed.push_str(&format!("partition_scheme={}\n", 
            match self.config.partition_scheme {
                PartitionScheme::Auto => "auto",
                PartitionScheme::Manual => "manual",
                PartitionScheme::LVM => "lvm",
                PartitionScheme::BtrfsSubvolumes => "btrfs",
            }));
        preseed.push_str(&format!("filesystem={}\n",
            match self.config.filesystem {
                FilesystemType::Ext4 => "ext4",
                FilesystemType::Btrfs => "btrfs",
                FilesystemType::XFS => "xfs",
                FilesystemType::ZFS => "zfs",
            }));
        preseed.push_str(&format!("boot_loader={}\n",
            match self.config.boot_loader {
                BootLoader::GRUB => "grub",
                BootLoader::SystemdBoot => "systemd",
                BootLoader::None => "none",
            }));
        preseed.push_str(&format!("encryption={}\n", self.config.encryption));
        preseed.push_str(&format!("swap_size={}\n", self.config.swap_size));
        preseed.push_str(&format!("use_dhcp={}\n", self.config.network_config.use_dhcp));
        preseed.push_str(&format!("desktop={}\n", 
            self.config.desktop_environment.as_deref().unwrap_or("none")));
        preseed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = InstallerConfig::default();
        assert_eq!(config.language, "en_US");
        assert_eq!(config.hostname, "sigmaos");
    }

    #[test]
    fn test_installer_steps() {
        let installer = SigmaInstaller::new();
        assert_eq!(installer.get_current_step(), InstallerStep::Welcome);
    }

    #[test]
    fn test_step_navigation() {
        let mut installer = SigmaInstaller::new();
        installer.next_step();
        assert_eq!(installer.get_current_step(), InstallerStep::Language);
        installer.previous_step();
        assert_eq!(installer.get_current_step(), InstallerStep::Welcome);
    }

    #[test]
    fn test_preseed_generation() {
        let installer = SigmaInstaller::new();
        let preseed = installer.generate_preseed();
        assert!(preseed.contains("language=en_US"));
        assert!(preseed.contains("hostname=sigmaos"));
    }
}