// SPDX-License-Identifier: MIT
// SigmaOS System Installer - Arch Linux Inspired
// Implements Arch-style installation system similar to archinstall

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Installation target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallTarget {
    Disk,
    Usb,
    Network,
    Container,
    Vm,
}

impl InstallTarget {
    pub fn as_str(&self) -> &str {
        match self {
            InstallTarget::Disk => "disk",
            InstallTarget::Usb => "usb",
            InstallTarget::Network => "network",
            InstallTarget::Container => "container",
            InstallTarget::Vm => "vm",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "disk" => Some(InstallTarget::Disk),
            "usb" => Some(InstallTarget::Usb),
            "network" => Some(InstallTarget::Network),
            "container" => Some(InstallTarget::Container),
            "vm" => Some(InstallTarget::Vm),
            _ => None,
        }
    }
}

/// Partition scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionScheme {
    Mbr,
    Gpt,
    None,
}

impl PartitionScheme {
    pub fn as_str(&self) -> &str {
        match self {
            PartitionScheme::Mbr => "mbr",
            PartitionScheme::Gpt => "gpt",
            PartitionScheme::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mbr" => Some(PartitionScheme::Mbr),
            "gpt" => Some(PartitionScheme::Gpt),
            "none" => Some(PartitionScheme::None),
            _ => None,
        }
    }
}

/// Filesystem type for installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallFilesystem {
    Ext4,
    Xfs,
    Btrfs,
    F2fs,
    Ntfs,
    Vfat,
    Swap,
}

impl InstallFilesystem {
    pub fn as_str(&self) -> &str {
        match self {
            InstallFilesystem::Ext4 => "ext4",
            InstallFilesystem::Xfs => "xfs",
            InstallFilesystem::Btrfs => "btrfs",
            InstallFilesystem::F2fs => "f2fs",
            InstallFilesystem::Ntfs => "ntfs",
            InstallFilesystem::Vfat => "vfat",
            InstallFilesystem::Swap => "swap",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ext4" => Some(InstallFilesystem::Ext4),
            "xfs" => Some(InstallFilesystem::Xfs),
            "btrfs" => Some(InstallFilesystem::Btrfs),
            "f2fs" => Some(InstallFilesystem::F2fs),
            "ntfs" => Some(InstallFilesystem::Ntfs),
            "vfat" => Some(InstallFilesystem::Vfat),
            "swap" => Some(InstallFilesystem::Swap),
            _ => None,
        }
    }

    pub fn get_mkfs_command(&self) -> String {
        match self {
            InstallFilesystem::Ext4 => "mkfs.ext4".to_string(),
            InstallFilesystem::Xfs => "mkfs.xfs".to_string(),
            InstallFilesystem::Btrfs => "mkfs.btrfs".to_string(),
            InstallFilesystem::F2fs => "mkfs.f2fs".to_string(),
            InstallFilesystem::Ntfs => "mkfs.ntfs".to_string(),
            InstallFilesystem::Vfat => "mkfs.vfat".to_string(),
            InstallFilesystem::Swap => "mkswap".to_string(),
        }
    }
}

/// Boot loader type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootLoader {
    SystemdBoot,
    Grub,
    Syslinux,
    Lilo,
    Refind,
    None,
}

impl BootLoader {
    pub fn as_str(&self) -> &str {
        match self {
            BootLoader::SystemdBoot => "systemd-boot",
            BootLoader::Grub => "grub",
            BootLoader::Syslinux => "syslinux",
            BootLoader::Lilo => "lilo",
            BootLoader::Refind => "refind",
            BootLoader::None => "none",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "systemd-boot" => Some(BootLoader::SystemdBoot),
            "grub" => Some(BootLoader::Grub),
            "syslinux" => Some(BootLoader::Syslinux),
            "lilo" => Some(BootLoader::Lilo),
            "refind" => Some(BootLoader::Refind),
            "none" => Some(BootLoader::None),
            _ => None,
        }
    }

    pub fn get_install_command(&self) -> Option<Vec<String>> {
        match self {
            BootLoader::SystemdBoot => Some(vec!["bootctl".to_string(), "install".to_string()]),
            BootLoader::Grub => Some(vec!["grub-install".to_string()]),
            BootLoader::Syslinux => Some(vec!["syslinux".to_string(), "-i".to_string()]),
            BootLoader::Lilo => Some(vec!["lilo".to_string()]),
            BootLoader::Refind => Some(vec!["refind-install".to_string()]),
            BootLoader::None => None,
        }
    }
}

/// Installation step
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStep {
    DetectHardware,
    PartitionDisk,
    FormatFilesystems,
    MountFilesystems,
    InstallBaseSystem,
    InstallKernel,
    InstallBootloader,
    ConfigureSystem,
    CreateUser,
    InstallDesktop,
    InstallDrivers,
    Finalize,
}

impl InstallStep {
    pub fn as_str(&self) -> &str {
        match self {
            InstallStep::DetectHardware => "detect-hardware",
            InstallStep::PartitionDisk => "partition-disk",
            InstallStep::FormatFilesystems => "format-filesystems",
            InstallStep::MountFilesystems => "mount-filesystems",
            InstallStep::InstallBaseSystem => "install-base-system",
            InstallStep::InstallKernel => "install-kernel",
            InstallStep::InstallBootloader => "install-bootloader",
            InstallStep::ConfigureSystem => "configure-system",
            InstallStep::CreateUser => "create-user",
            InstallStep::InstallDesktop => "install-desktop",
            InstallStep::InstallDrivers => "install-drivers",
            InstallStep::Finalize => "finalize",
        }
    }

    pub fn get_order(&self) -> u32 {
        match self {
            InstallStep::DetectHardware => 1,
            InstallStep::PartitionDisk => 2,
            InstallStep::FormatFilesystems => 3,
            InstallStep::MountFilesystems => 4,
            InstallStep::InstallBaseSystem => 5,
            InstallStep::InstallKernel => 6,
            InstallStep::InstallBootloader => 7,
            InstallStep::ConfigureSystem => 8,
            InstallStep::CreateUser => 9,
            InstallStep::InstallDesktop => 10,
            InstallStep::InstallDrivers => 11,
            InstallStep::Finalize => 12,
        }
    }
}

/// Installation status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

impl InstallStatus {
    pub fn as_str(&self) -> &str {
        match self {
            InstallStatus::Pending => "pending",
            InstallStatus::InProgress => "in-progress",
            InstallStatus::Completed => "completed",
            InstallStatus::Failed => "failed",
            InstallStatus::Skipped => "skipped",
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, InstallStatus::Completed | InstallStatus::Skipped)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, InstallStatus::Failed)
    }
}

/// Partition configuration
#[derive(Debug, Clone)]
pub struct PartitionConfig {
    pub device: String,
    pub partition_number: u32,
    pub size: String,
    pub filesystem: InstallFilesystem,
    pub mount_point: Option<String>,
    pub is_boot: bool,
    pub is_esp: bool,
    pub is_swap: bool,
    pub is_root: bool,
    pub label: Option<String>,
    pub uuid: Option<String>,
}

impl PartitionConfig {
    /// Create a new partition config
    pub fn new(device: impl Into<String>, partition_number: u32, size: impl Into<String>, filesystem: InstallFilesystem) -> Self {
        Self {
            device: device.into(),
            partition_number,
            size: size.into(),
            filesystem,
            mount_point: None,
            is_boot: false,
            is_esp: false,
            is_swap: false,
            is_root: false,
            label: None,
            uuid: None,
        }
    }

    /// Set mount point
    pub fn mount_point(mut self, mount_point: impl Into<String>) -> Self {
        self.mount_point = Some(mount_point.into());
        self
    }

    /// Mark as boot partition
    pub fn boot(mut self) -> Self {
        self.is_boot = true;
        self
    }

    /// Mark as EFI System Partition
    pub fn esp(mut self) -> Self {
        self.is_esp = true;
        self
    }

    /// Mark as swap
    pub fn swap(mut self) -> Self {
        self.is_swap = true;
        self.filesystem = InstallFilesystem::Swap;
        self
    }

    /// Mark as root
    pub fn root(mut self) -> Self {
        self.is_root = true;
        self.mount_point = Some("/".to_string());
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }
}

/// Installation configuration
#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub target: InstallTarget,
    pub target_device: Option<String>,
    pub partition_scheme: PartitionScheme,
    pub partitions: Vec<PartitionConfig>,
    pub bootloader: BootLoader,
    pub kernel: String,
    pub desktop_environment: Option<String>,
    pub display_manager: Option<String>,
    pub network_config: Option<String>,
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub keyboard_layout: String,
    pub root_password: Option<String>,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub user_groups: Vec<String>,
    pub packages: Vec<String>,
    pub mirrors: Vec<String>,
    pub swap_size: Option<String>,
    pub enable_lvm: bool,
    pub enable_encryption: bool,
    pub encryption_password: Option<String>,
    pub custom_kernel_params: Vec<String>,
    pub install_drivers: bool,
    pub install_firmware: bool,
    pub enable_firewall: bool,
    pub enable_sshd: bool,
    pub enable_cron: bool,
    pub enable_ntp: bool,
}

impl InstallConfig {
    /// Create a new installation config
    pub fn new() -> Self {
        Self {
            target: InstallTarget::Disk,
            target_device: None,
            partition_scheme: PartitionScheme::Gpt,
            partitions: Vec::new(),
            bootloader: BootLoader::SystemdBoot,
            kernel: "linux".to_string(),
            desktop_environment: None,
            display_manager: None,
            network_config: None,
            hostname: "sigmaos".to_string(),
            timezone: "UTC".to_string(),
            locale: "C".to_string(),
            keyboard_layout: "us".to_string(),
            root_password: None,
            user_name: None,
            user_password: None,
            user_groups: vec!["wheel".to_string()],
            packages: Vec::new(),
            mirrors: Vec::new(),
            swap_size: None,
            enable_lvm: false,
            enable_encryption: false,
            encryption_password: None,
            custom_kernel_params: Vec::new(),
            install_drivers: true,
            install_firmware: true,
            enable_firewall: true,
            enable_sshd: false,
            enable_cron: true,
            enable_ntp: true,
        }
    }

    /// Set target
    pub fn target(mut self, target: InstallTarget) -> Self {
        self.target = target;
        self
    }

    /// Set target device
    pub fn target_device(mut self, device: impl Into<String>) -> Self {
        self.target_device = Some(device.into());
        self
    }

    /// Set partition scheme
    pub fn partition_scheme(mut self, scheme: PartitionScheme) -> Self {
        self.partition_scheme = scheme;
        self
    }

    /// Add partition
    pub fn partition(mut self, partition: PartitionConfig) -> Self {
        self.partitions.push(partition);
        self
    }

    /// Set bootloader
    pub fn bootloader(mut self, bootloader: BootLoader) -> Self {
        self.bootloader = bootloader;
        self
    }

    /// Set kernel
    pub fn kernel(mut self, kernel: impl Into<String>) -> Self {
        self.kernel = kernel.into();
        self
    }

    /// Set desktop environment
    pub fn desktop_environment(mut self, de: impl Into<String>) -> Self {
        self.desktop_environment = Some(de.into());
        self
    }

    /// Set display manager
    pub fn display_manager(mut self, dm: impl Into<String>) -> Self {
        self.display_manager = Some(dm.into());
        self
    }

    /// Set network config
    pub fn network_config(mut self, config: impl Into<String>) -> Self {
        self.network_config = Some(config.into());
        self
    }

    /// Set hostname
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.hostname = hostname.into();
        self
    }

    /// Set timezone
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = timezone.into();
        self
    }

    /// Set locale
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    /// Set keyboard layout
    pub fn keyboard_layout(mut self, layout: impl Into<String>) -> Self {
        self.keyboard_layout = layout.into();
        self
    }

    /// Set root password
    pub fn root_password(mut self, password: impl Into<String>) -> Self {
        self.root_password = Some(password.into());
        self
    }

    /// Set user
    pub fn user(mut self, name: impl Into<String>, password: impl Into<String>) -> Self {
        self.user_name = Some(name.into());
        self.user_password = Some(password.into());
        self
    }

    /// Add user group
    pub fn user_group(mut self, group: impl Into<String>) -> Self {
        self.user_groups.push(group.into());
        self
    }

    /// Add package
    pub fn package(mut self, package: impl Into<String>) -> Self {
        self.packages.push(package.into());
        self
    }

    /// Add mirror
    pub fn mirror(mut self, mirror: impl Into<String>) -> Self {
        self.mirrors.push(mirror.into());
        self
    }

    /// Set swap size
    pub fn swap_size(mut self, size: impl Into<String>) -> Self {
        self.swap_size = Some(size.into());
        self
    }

    /// Enable LVM
    pub fn enable_lvm(mut self, enable: bool) -> Self {
        self.enable_lvm = enable;
        self
    }

    /// Enable encryption
    pub fn enable_encryption(mut self, enable: bool, password: Option<impl Into<String>>) -> Self {
        self.enable_encryption = enable;
        if let Some(pwd) = password {
            self.encryption_password = Some(pwd.into());
        }
        self
    }

    /// Add kernel parameter
    pub fn kernel_param(mut self, param: impl Into<String>) -> Self {
        self.custom_kernel_params.push(param.into());
        self
    }

    /// Enable/disable drivers
    pub fn install_drivers(mut self, install: bool) -> Self {
        self.install_drivers = install;
        self
    }

    /// Enable/disable firmware
    pub fn install_firmware(mut self, install: bool) -> Self {
        self.install_firmware = install;
        self
    }

    /// Enable/disable firewall
    pub fn enable_firewall(mut self, enable: bool) -> Self {
        self.enable_firewall = enable;
        self
    }

    /// Enable/disable SSH
    pub fn enable_sshd(mut self, enable: bool) -> Self {
        self.enable_sshd = enable;
        self
    }

    /// Enable/disable cron
    pub fn enable_cron(mut self, enable: bool) -> Self {
        self.enable_cron = enable;
        self
    }

    /// Enable/disable NTP
    pub fn enable_ntp(mut self, enable: bool) -> Self {
        self.enable_ntp = enable;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        // Check target device
        if self.target == InstallTarget::Disk && self.target_device.is_none() {
            return Err("Target device required for disk installation".to_string());
        }

        // Check partitions
        if self.partitions.is_empty() && self.target == InstallTarget::Disk {
            return Err("At least one partition required".to_string());
        }

        // Check for root partition
        if !self.partitions.iter().any(|p| p.is_root) {
            return Err("Root partition required".to_string());
        }

        // Check for boot partition
        if self.partition_scheme == PartitionScheme::Gpt && !self.partitions.iter().any(|p| p.is_esp) {
            return Err("EFI System Partition required for GPT".to_string());
        }

        // Check encryption password
        if self.enable_encryption && self.encryption_password.is_none() {
            return Err("Encryption password required".to_string());
        }

        Ok(())
    }

    /// Export configuration
    pub fn export_config(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS Installation Configuration\n");
        content.push_str("# Generated by SigmaOS Installer\n\n");

        content.push_str(&format!("target={}\n", self.target.as_str()));
        if let Some(device) = &self.target_device {
            content.push_str(&format!("target_device={}\n", device));
        }
        content.push_str(&format!("partition_scheme={}\n", self.partition_scheme.as_str()));
        content.push_str(&format!("bootloader={}\n", self.bootloader.as_str()));
        content.push_str(&format!("kernel={}\n", self.kernel));
        content.push_str(&format!("hostname={}\n", self.hostname));
        content.push_str(&format!("timezone={}\n", self.timezone));
        content.push_str(&format!("locale={}\n", self.locale));
        content.push_str(&format!("keyboard_layout={}\n", self.keyboard_layout));

        if let Some(de) = &self.desktop_environment {
            content.push_str(&format!("desktop_environment={}\n", de));
        }
        if let Some(dm) = &self.display_manager {
            content.push_str(&format!("display_manager={}\n", dm));
        }

        content.push_str(&format!("enable_lvm={}\n", self.enable_lvm));
        content.push_str(&format!("enable_encryption={}\n", self.enable_encryption));
        content.push_str(&format!("install_drivers={}\n", self.install_drivers));
        content.push_str(&format!("install_firmware={}\n", self.install_firmware));
        content.push_str(&format!("enable_firewall={}\n", self.enable_firewall));
        content.push_str(&format!("enable_sshd={}\n", self.enable_sshd));
        content.push_str(&format!("enable_cron={}\n", self.enable_cron));
        content.push_str(&format!("enable_ntp={}\n", self.enable_ntp));

        content.push_str("\n## Partitions\n");
        for partition in &self.partitions {
            content.push_str(&format!("partition={} number={} size={} fs={} mount={} boot={} esp={} swap={} root={}\n",
                partition.device,
                partition.partition_number,
                partition.size,
                partition.filesystem.as_str(),
                partition.mount_point.as_deref().unwrap_or("none"),
                partition.is_boot,
                partition.is_esp,
                partition.is_swap,
                partition.is_root
            ));
        }

        content.push_str("\n## Packages\n");
        for package in &self.packages {
            content.push_str(&format!("package={}\n", package));
        }

        content.push_str("\n## Mirrors\n");
        for mirror in &self.mirrors {
            content.push_str(&format!("mirror={}\n", mirror));
        }

        content.push_str("\n## Kernel Parameters\n");
        for param in &self.custom_kernel_params {
            content.push_str(&format!("kernel_param={}\n", param));
        }

        content
    }

    /// Import configuration
    pub fn import_config(&mut self, content: &str) -> Result<(), String> {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "target" => {
                        if let Some(target) = InstallTarget::from_str(value) {
                            self.target = target;
                        }
                    }
                    "target_device" => self.target_device = Some(value.to_string()),
                    "partition_scheme" => {
                        if let Some(scheme) = PartitionScheme::from_str(value) {
                            self.partition_scheme = scheme;
                        }
                    }
                    "bootloader" => {
                        if let Some(bootloader) = BootLoader::from_str(value) {
                            self.bootloader = bootloader;
                        }
                    }
                    "kernel" => self.kernel = value.to_string(),
                    "hostname" => self.hostname = value.to_string(),
                    "timezone" => self.timezone = value.to_string(),
                    "locale" => self.locale = value.to_string(),
                    "keyboard_layout" => self.keyboard_layout = value.to_string(),
                    "desktop_environment" => self.desktop_environment = Some(value.to_string()),
                    "display_manager" => self.display_manager = Some(value.to_string()),
                    "enable_lvm" => self.enable_lvm = value == "true",
                    "enable_encryption" => self.enable_encryption = value == "true",
                    "install_drivers" => self.install_drivers = value == "true",
                    "install_firmware" => self.install_firmware = value == "true",
                    "enable_firewall" => self.enable_firewall = value == "true",
                    "enable_sshd" => self.enable_sshd = value == "true",
                    "enable_cron" => self.enable_cron = value == "true",
                    "enable_ntp" => self.enable_ntp = value == "true",
                    "package" => self.packages.push(value.to_string()),
                    "mirror" => self.mirrors.push(value.to_string()),
                    "kernel_param" => self.custom_kernel_params.push(value.to_string()),
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Installation step status
#[derive(Debug, Clone)]
pub struct InstallStepStatus {
    pub step: InstallStep,
    pub status: InstallStatus,
    pub message: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub error: Option<String>,
}

impl InstallStepStatus {
    /// Create a new step status
    pub fn new(step: InstallStep) -> Self {
        Self {
            step,
            status: InstallStatus::Pending,
            message: String::new(),
            start_time: None,
            end_time: None,
            error: None,
        }
    }

    /// Set status
    pub fn set_status(&mut self, status: InstallStatus) {
        self.status = status;
        if status == InstallStatus::InProgress {
            self.start_time = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());
        } else if status.is_completed() || status.is_failed() {
            self.end_time = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());
        }
    }

    /// Set message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Set error
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error = Some(error.into());
        self.status = InstallStatus::Failed;
        self.end_time = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
    }

    /// Get duration
    pub fn get_duration(&self) -> Option<u64> {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            Some(end - start)
        } else {
            None
        }
    }
}

/// System installer
#[derive(Debug, Clone)]
pub struct SystemInstaller {
    pub config: InstallConfig,
    pub step_statuses: BTreeMap<InstallStep, InstallStepStatus>,
    pub current_step: Option<InstallStep>,
    pub install_log: Vec<String>,
    pub is_installing: bool,
    pub install_start_time: Option<u64>,
    pub install_end_time: Option<u64>,
}

impl SystemInstaller {
    /// Create a new system installer
    pub fn new(config: InstallConfig) -> Self {
        let mut step_statuses = BTreeMap::new();
        for step in [
            InstallStep::DetectHardware,
            InstallStep::PartitionDisk,
            InstallStep::FormatFilesystems,
            InstallStep::MountFilesystems,
            InstallStep::InstallBaseSystem,
            InstallStep::InstallKernel,
            InstallStep::InstallBootloader,
            InstallStep::ConfigureSystem,
            InstallStep::CreateUser,
            InstallStep::InstallDesktop,
            InstallStep::InstallDrivers,
            InstallStep::Finalize,
        ] {
            step_statuses.insert(step, InstallStepStatus::new(step));
        }

        Self {
            config,
            step_statuses,
            current_step: None,
            install_log: Vec::new(),
            is_installing: false,
            install_start_time: None,
            install_end_time: None,
        }
    }

    /// Create system installer with default config
    pub fn with_default_config() -> Self {
        Self::new(InstallConfig::new())
    }

    /// Validate installation configuration
    pub fn validate(&self) -> Result<(), String> {
        self.config.validate()
    }

    /// Start installation
    pub fn install(&mut self) -> Result<(), String> {
        // Validate configuration
        self.validate()?;

        // Set installation state
        self.is_installing = true;
        self.install_start_time = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        self.log("Starting SigmaOS installation...");

        // Execute installation steps in order
        let steps = [
            InstallStep::DetectHardware,
            InstallStep::PartitionDisk,
            InstallStep::FormatFilesystems,
            InstallStep::MountFilesystems,
            InstallStep::InstallBaseSystem,
            InstallStep::InstallKernel,
            InstallStep::InstallBootloader,
            InstallStep::ConfigureSystem,
            InstallStep::CreateUser,
            InstallStep::InstallDesktop,
            InstallStep::InstallDrivers,
            InstallStep::Finalize,
        ];

        for step in steps {
            self.execute_step(step)?;
        }

        self.is_installing = false;
        self.install_end_time = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        self.log("Installation completed successfully!");
        Ok(())
    }

    /// Execute installation step
    fn execute_step(&mut self, step: InstallStep) -> Result<(), String> {
        self.current_step = Some(step);
        let step_status = self.step_statuses.get_mut(&step).unwrap();
        step_status.set_status(InstallStatus::InProgress);
        step_status.set_message(format!("Executing {}", step.as_str()));

        self.log(&format!("Executing step: {}", step.as_str()));

        let result = match step {
            InstallStep::DetectHardware => self.step_detect_hardware(),
            InstallStep::PartitionDisk => self.step_partition_disk(),
            InstallStep::FormatFilesystems => self.step_format_filesystems(),
            InstallStep::MountFilesystems => self.step_mount_filesystems(),
            InstallStep::InstallBaseSystem => self.step_install_base_system(),
            InstallStep::InstallKernel => self.step_install_kernel(),
            InstallStep::InstallBootloader => self.step_install_bootloader(),
            InstallStep::ConfigureSystem => self.step_configure_system(),
            InstallStep::CreateUser => self.step_create_user(),
            InstallStep::InstallDesktop => self.step_install_desktop(),
            InstallStep::InstallDrivers => self.step_install_drivers(),
            InstallStep::Finalize => self.step_finalize(),
        };

        match result {
            Ok(()) => {
                step_status.set_status(InstallStatus::Completed);
                step_status.set_message(format!("{} completed", step.as_str()));
                self.log(&format!("Step {} completed", step.as_str()));
                Ok(())
            }
            Err(e) => {
                step_status.set_error(e.clone());
                self.log(&format!("Step {} failed: {}", step.as_str(), e));
                Err(e)
            }
        }
    }

    /// Detect hardware step
    fn step_detect_hardware(&self) -> Result<(), String> {
        // In real implementation, would detect hardware
        // For now, just log the detection
        self.log("Detecting hardware...");
        Ok(())
    }

    /// Partition disk step
    fn step_partition_disk(&self) -> Result<(), String> {
        let device = self.config.target_device.as_deref()
            .ok_or("Target device not specified")?;

        self.log(&format!("Partitioning disk: {}", device));

        // Create partition table
        match self.config.partition_scheme {
            PartitionScheme::Gpt => {
                self.run_command("parted", &["-s", device, "mklabel", "gpt"])?;
            }
            PartitionScheme::Mbr => {
                self.run_command("parted", &["-s", device, "mklabel", "msdos"])?;
            }
            PartitionScheme::None => {}
        }

        // Create partitions
        for partition in &self.config.partitions {
            self.log(&format!("Creating partition {} on {}", partition.partition_number, partition.device));

            // Create partition
            let size = &partition.size;
            self.run_command("parted", &["-s", &partition.device, "mkpart", "primary", size, "100%"])?;

            // Set partition flags
            if partition.is_boot {
                self.run_command("parted", &["-s", &partition.device, "set", "1", "boot", "on"])?;
            }
            if partition.is_esp {
                self.run_command("parted", &["-s", &partition.device, "set", "1", "esp", "on"])?;
            }
        }

        Ok(())
    }

    /// Format filesystems step
    fn step_format_filesystems(&self) -> Result<(), String> {
        for partition in &self.config.partitions {
            let device = format!("{}{}", partition.device, partition.partition_number);
            let fs_type = partition.filesystem;

            self.log(&format!("Formatting {} as {}", device, fs_type.as_str()));

            // Create filesystem
            let mkfs_cmd = fs_type.get_mkfs_command();
            let mut args = vec![device.as_str()];
            if let Some(label) = &partition.label {
                args.push("-L");
                args.push(label.as_str());
            }

            self.run_command(&mkfs_cmd, &args)?;
        }

        Ok(())
    }

    /// Mount filesystems step
    fn step_mount_filesystems(&self) -> Result<(), String> {
        // Create mount points
        for partition in &self.config.partitions {
            if let Some(mount_point) = &partition.mount_point {
                let device = format!("{}{}", partition.device, partition.partition_number);

                self.log(&format!("Mounting {} at {}", device, mount_point));

                // Create mount point
                fs::create_dir_all(mount_point).map_err(|e| format!("Failed to create mount point: {}", e))?;

                // Mount filesystem
                self.run_command("mount", &[&device, mount_point])?;
            }
        }

        Ok(())
    }

    /// Install base system step
    fn step_install_base_system(&self) -> Result<(), String> {
        self.log("Installing base system...");

        // In real implementation, would install base packages
        // For now, just simulate
        let base_packages = vec!["base", "base-devel", "linux-firmware"];
        for package in base_packages {
            self.log(&format!("Installing package: {}", package));
        }

        Ok(())
    }

    /// Install kernel step
    fn step_install_kernel(&self) -> Result<(), String> {
        self.log(&format!("Installing kernel: {}", self.config.kernel));

        // In real implementation, would install kernel package
        // For now, just simulate
        self.log(&format!("Installing kernel package: {}", self.config.kernel));

        Ok(())
    }

    /// Install bootloader step
    fn step_install_bootloader(&self) -> Result<(), String> {
        self.log(&format!("Installing bootloader: {}", self.config.bootloader.as_str()));

        // Get boot partition
        let boot_partition = self.config.partitions.iter()
            .find(|p| p.is_boot || p.is_esp)
            .ok_or("Boot partition not found")?;

        let boot_device = format!("{}{}", boot_partition.device, boot_partition.partition_number);

        // Install bootloader
        if let Some(cmd) = self.config.bootloader.get_install_command() {
            let mut args = cmd[1..].to_vec();
            args.push("--boot-directory".to_string());
            args.push(boot_device.clone());
            args.push("--target".to_string());
            args.push(self.config.target_device.clone().unwrap_or_default());

            self.run_command(&cmd[0], &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
        }

        Ok(())
    }

    /// Configure system step
    fn step_configure_system(&self) -> Result<(), String> {
        self.log("Configuring system...");

        // Set hostname
        fs::write("/etc/hostname", format!("{}\n", self.config.hostname))
            .map_err(|e| format!("Failed to write hostname: {}", e))?;

        // Set timezone
        self.run_command("ln", &["-sf", &format!("/usr/share/zoneinfo/{}", self.config.timezone), "/etc/localtime"])?;

        // Set locale
        fs::write("/etc/locale.conf", format!("LANG={}\n", self.config.locale))
            .map_err(|e| format!("Failed to write locale: {}", e))?;

        // Set keyboard layout
        fs::write("/etc/vconsole.conf", format!("KEYMAP={}\n", self.config.keyboard_layout))
            .map_err(|e| format!("Failed to write vconsole.conf: {}", e))?;

        // Set root password
        if let Some(password) = &self.config.root_password {
            self.run_command("chpasswd", &[])?;
        }

        // Configure network
        if let Some(network_config) = &self.config.network_config {
            fs::write("/etc/systemd/network/10-static.network", network_config)
                .map_err(|e| format!("Failed to write network config: {}", e))?;
        }

        Ok(())
    }

    /// Create user step
    fn step_create_user(&self) -> Result<(), String> {
        if let (Some(username), Some(password)) = (&self.config.user_name, &self.config.user_password) {
            self.log(&format!("Creating user: {}", username));

            // Create user
            let mut args = vec!["-m", "-G"];
            let groups = self.config.user_groups.join(",");
            args.push(&groups);
            args.push(username);

            self.run_command("useradd", &args)?;

            // Set password
            self.run_command("chpasswd", &[])?;
        }

        Ok(())
    }

    /// Install desktop step
    fn step_install_desktop(&self) -> Result<(), String> {
        if let Some(de) = &self.config.desktop_environment {
            self.log(&format!("Installing desktop environment: {}", de));

            // In real implementation, would install DE packages
            // For now, just simulate
            self.log(&format!("Installing {} packages", de));
        }

        if let Some(dm) = &self.config.display_manager {
            self.log(&format!("Installing display manager: {}", dm));

            // Enable display manager
            self.run_command("systemctl", &["enable", dm])?;
        }

        Ok(())
    }

    /// Install drivers step
    fn step_install_drivers(&self) -> Result<(), String> {
        if self.config.install_drivers {
            self.log("Installing hardware drivers...");

            // Install common drivers
            let drivers = vec!["linux-firmware", "mesa", "vulkan-icd-loader"];
            for driver in drivers {
                self.log(&format!("Installing driver: {}", driver));
            }
        }

        if self.config.install_firmware {
            self.log("Installing firmware...");

            // Install firmware packages
            let firmware = vec!["linux-firmware", "sof-firmware", "alsa-firmware"];
            for fw in firmware {
                self.log(&format!("Installing firmware: {}", fw));
            }
        }

        Ok(())
    }

    /// Finalize step
    fn step_finalize(&self) -> Result<(), String> {
        self.log("Finalizing installation...");

        // Configure firewall
        if self.config.enable_firewall {
            self.log("Configuring firewall...");
            self.run_command("systemctl", &["enable", "firewalld"])?;
        }

        // Enable SSH
        if self.config.enable_sshd {
            self.log("Enabling SSH...");
            self.run_command("systemctl", &["enable", "sshd"])?;
        }

        // Enable cron
        if self.config.enable_cron {
            self.log("Enabling cron...");
            self.run_command("systemctl", &["enable", "cronie"])?;
        }

        // Enable NTP
        if self.config.enable_ntp {
            self.log("Enabling NTP...");
            self.run_command("systemctl", &["enable", "systemd-timesyncd"])?;
        }

        // Set default target
        self.run_command("systemctl", &["set-default", "graphical.target"])?;

        self.log("Installation finalized!");
        Ok(())
    }

    /// Run command helper
    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<(), String> {
        let output = Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to run {}: {}", cmd, e))?;

        if !output.status.success() {
            return Err(format!("{} failed: {}", cmd, String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Log message
    fn log(&self, message: &str) {
        println!("[{}] {}", SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(), message);
    }

    /// Get step status
    pub fn get_step_status(&self, step: InstallStep) -> Option<&InstallStepStatus> {
        self.step_statuses.get(&step)
    }

    /// Get all step statuses
    pub fn get_all_step_statuses(&self) -> Vec<&InstallStepStatus> {
        self.step_statuses.values().collect()
    }

    /// Get current step
    pub fn get_current_step(&self) -> Option<InstallStep> {
        self.current_step
    }

    /// Check if installation is in progress
    pub fn is_installing(&self) -> bool {
        self.is_installing
    }

    /// Get installation progress
    pub fn get_progress(&self) -> (usize, usize) {
        let total = self.step_statuses.len();
        let completed = self.step_statuses.values()
            .filter(|s| s.status.is_completed())
            .count();
        (completed, total)
    }

    /// Get installation statistics
    pub fn get_statistics(&self) -> InstallStatistics {
        let total_steps = self.step_statuses.len();
        let completed_steps = self.step_statuses.values()
            .filter(|s| s.status == InstallStatus::Completed)
            .count();
        let failed_steps = self.step_statuses.values()
            .filter(|s| s.status.is_failed())
            .count();
        let skipped_steps = self.step_statuses.values()
            .filter(|s| s.status == InstallStatus::Skipped)
            .count();

        InstallStatistics {
            total_steps,
            completed_steps,
            failed_steps,
            skipped_steps,
            progress_percent: if total_steps > 0 {
                (completed_steps as f64 / total_steps as f64) * 100.0
            } else {
                0.0
            },
            install_duration: if let (Some(start), Some(end)) = (self.install_start_time, self.install_end_time) {
                Some(end - start)
            } else {
                None
            },
        }
    }

    /// Export installation log
    pub fn export_log(&self) -> String {
        let mut log = String::new();
        log.push_str("# SigmaOS Installation Log\n");
        log.push_str(&format!("Started: {}\n", self.install_start_time.unwrap_or(0)));
        log.push_str(&format!("Ended: {}\n", self.install_end_time.unwrap_or(0)));
        log.push_str(&format!("Duration: {} seconds\n\n", self.get_statistics().install_duration.unwrap_or(0)));

        for status in self.get_all_step_statuses() {
            log.push_str(&format!("[{}] {} - {}\n", status.status.as_str(), status.step.as_str(), status.message));
            if let Some(error) = &status.error {
                log.push_str(&format!("  Error: {}\n", error));
            }
            if let Some(duration) = status.get_duration() {
                log.push_str(&format!("  Duration: {} seconds\n", duration));
            }
        }

        log
    }
}

impl Default for SystemInstaller {
    fn default() -> Self {
        Self::with_default_config()
    }
}

/// Installation statistics
#[derive(Debug, Clone)]
pub struct InstallStatistics {
    pub total_steps: usize,
    pub completed_steps: usize,
    pub failed_steps: usize,
    pub skipped_steps: usize,
    pub progress_percent: f64,
    pub install_duration: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_target() {
        assert_eq!(InstallTarget::Disk.as_str(), "disk");
        assert_eq!(InstallTarget::from_str("usb"), Some(InstallTarget::Usb));
    }

    #[test]
    fn test_partition_scheme() {
        assert_eq!(PartitionScheme::Gpt.as_str(), "gpt");
        assert_eq!(PartitionScheme::from_str("mbr"), Some(PartitionScheme::Mbr));
    }

    #[test]
    fn test_install_filesystem() {
        assert_eq!(InstallFilesystem::Ext4.as_str(), "ext4");
        assert_eq!(InstallFilesystem::from_str("btrfs"), Some(InstallFilesystem::Btrfs));
        assert_eq!(InstallFilesystem::Ext4.get_mkfs_command(), "mkfs.ext4");
    }

    #[test]
    fn test_boot_loader() {
        assert_eq!(BootLoader::SystemdBoot.as_str(), "systemd-boot");
        assert_eq!(BootLoader::from_str("grub"), Some(BootLoader::Grub));
        assert!(BootLoader::SystemdBoot.get_install_command().is_some());
    }

    #[test]
    fn test_install_step() {
        assert_eq!(InstallStep::PartitionDisk.as_str(), "partition-disk");
        assert_eq!(InstallStep::PartitionDisk.get_order(), 2);
        assert!(InstallStep::DetectHardware.get_order() < InstallStep::Finalize.get_order());
    }

    #[test]
    fn test_partition_config() {
        let partition = PartitionConfig::new("/dev/sda", 1, "512M", InstallFilesystem::Vfat)
            .esp()
            .boot();

        assert_eq!(partition.device, "/dev/sda");
        assert_eq!(partition.partition_number, 1);
        assert_eq!(partition.size, "512M");
        assert!(partition.is_esp);
        assert!(partition.is_boot);
    }

    #[test]
    fn test_install_config_builder() {
        let config = InstallConfig::new()
            .target(InstallTarget::Disk)
            .target_device("/dev/sda")
            .partition_scheme(PartitionScheme::Gpt)
            .bootloader(BootLoader::SystemdBoot)
            .kernel("linux")
            .hostname("myhost")
            .timezone("America/New_York")
            .locale("en_US.UTF-8")
            .keyboard_layout("us")
            .desktop_environment("gnome")
            .display_manager("gdm")
            .user("user", "password")
            .package("base")
            .package("linux")
            .mirror("https://mirror.example.com")
            .enable_firewall(true)
            .enable_sshd(true);

        assert_eq!(config.target, InstallTarget::Disk);
        assert_eq!(config.target_device, Some("/dev/sda".to_string()));
        assert_eq!(config.bootloader, BootLoader::SystemdBoot);
        assert_eq!(config.kernel, "linux");
        assert_eq!(config.hostname, "myhost");
        assert_eq!(config.desktop_environment, Some("gnome".to_string()));
        assert_eq!(config.user_name, Some("user".to_string()));
        assert_eq!(config.packages.len(), 2);
        assert!(config.enable_firewall);
        assert!(config.enable_sshd);
    }

    #[test]
    fn test_install_config_validation() {
        let mut config = InstallConfig::new();
        config.target = InstallTarget::Disk;
        config.target_device = Some("/dev/sda".to_string());
        config.partitions.push(PartitionConfig::new("/dev/sda", 1, "100%", InstallFilesystem::Ext4).root());

        assert!(config.validate().is_ok());

        // Remove root partition
        config.partitions.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_install_config_export() {
        let config = InstallConfig::new()
            .target(InstallTarget::Disk)
            .target_device("/dev/sda")
            .hostname("test")
            .package("base")
            .mirror("https://mirror.example.com");

        let export = config.export_config();
        assert!(export.contains("target=disk"));
        assert!(export.contains("target_device=/dev/sda"));
        assert!(export.contains("hostname=test"));
        assert!(export.contains("package=base"));
        assert!(export.contains("mirror=https://mirror.example.com"));
    }

    #[test]
    fn test_install_step_status() {
        let mut status = InstallStepStatus::new(InstallStep::PartitionDisk);
        assert_eq!(status.status, InstallStatus::Pending);

        status.set_status(InstallStatus::InProgress);
        assert_eq!(status.status, InstallStatus::InProgress);
        assert!(status.start_time.is_some());

        status.set_status(InstallStatus::Completed);
        assert_eq!(status.status, InstallStatus::Completed);
        assert!(status.end_time.is_some());
    }

    #[test]
    fn test_system_installer() {
        let config = InstallConfig::new()
            .target(InstallTarget::Disk)
            .target_device("/dev/sda")
            .partitions.push(PartitionConfig::new("/dev/sda", 1, "100%", InstallFilesystem::Ext4).root());

        let mut installer = SystemInstaller::new(config);
        assert_eq!(installer.get_all_step_statuses().len(), 12);
        assert!(!installer.is_installing());
    }

    #[test]
    fn test_install_statistics() {
        let mut installer = SystemInstaller::new(InstallConfig::new());

        // Simulate some completed steps
        if let Some(status) = installer.step_statuses.get_mut(&InstallStep::DetectHardware) {
            status.set_status(InstallStatus::Completed);
        }
        if let Some(status) = installer.step_statuses.get_mut(&InstallStep::PartitionDisk) {
            status.set_status(InstallStatus::Completed);
        }
        if let Some(status) = installer.step_statuses.get_mut(&InstallStep::FormatFilesystems) {
            status.set_status(InstallStatus::Failed);
        }

        let stats = installer.get_statistics();
        assert_eq!(stats.total_steps, 12);
        assert_eq!(stats.completed_steps, 2);
        assert_eq!(stats.failed_steps, 1);
        assert_eq!(stats.progress_percent, 16.67); // 2/12 * 100
    }
}
