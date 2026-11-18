// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Live OS Installer & Bootstrap Engine
//!
//! Inspired by archinstall, Omarchy setup wizards, and Debian debconf.
//! Provides disk partition layout, subvolume planning (Btrfs root/home/snapshots),
//! pacstrap/bootstrap sequence generator, and fstab generation.

use std::fmt;

/// Supported target filesystem types for SigmaOS installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallFilesystem {
    Btrfs,
    Ext4,
    Xfs,
    Zfs,
}

impl fmt::Display for InstallFilesystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Btrfs => write!(f, "btrfs"),
            Self::Ext4 => write!(f, "ext4"),
            Self::Xfs => write!(f, "xfs"),
            Self::Zfs => write!(f, "zfs"),
        }
    }
}

/// Target desktop persona to provision during bootstrap
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopPersona {
    OmarchyHyprland,
    ArchMinimalCli,
    SigmaZenithDesktop,
}

impl fmt::Display for DesktopPersona {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OmarchyHyprland => write!(f, "omarchy-hyprland"),
            Self::ArchMinimalCli => write!(f, "arch-minimal-cli"),
            Self::SigmaZenithDesktop => write!(f, "sigma-zenith"),
        }
    }
}

/// Disk partitioning layout configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionLayout {
    pub target_disk: String,
    pub efi_partition: String,
    pub root_partition: String,
    pub filesystem: InstallFilesystem,
    pub btrfs_subvolumes: Vec<String>,
}

impl PartitionLayout {
    pub fn new_btrfs_default(disk: &str) -> Self {
        Self {
            target_disk: disk.to_string(),
            efi_partition: format!("{}p1", disk),
            root_partition: format!("{}p2", disk),
            filesystem: InstallFilesystem::Btrfs,
            btrfs_subvolumes: vec![
                "@".to_string(),
                "@home".to_string(),
                "@snapshots".to_string(),
                "@var_log".to_string(),
            ],
        }
    }

    /// Generate mkfs formatting commands
    pub fn format_commands(&self) -> Vec<String> {
        let mut cmds = Vec::new();
        cmds.push(format!("mkfs.fat -F 32 {}", self.efi_partition));
        match self.filesystem {
            InstallFilesystem::Btrfs => {
                cmds.push(format!("mkfs.btrfs -f {}", self.root_partition));
                cmds.push(format!("mount {} /mnt", self.root_partition));
                for subvol in &self.btrfs_subvolumes {
                    cmds.push(format!("btrfs subvolume create /mnt/{}", subvol));
                }
                cmds.push("umount /mnt".to_string());
            }
            InstallFilesystem::Ext4 => {
                cmds.push(format!("mkfs.ext4 -F {}", self.root_partition));
            }
            InstallFilesystem::Xfs => {
                cmds.push(format!("mkfs.xfs -f {}", self.root_partition));
            }
            InstallFilesystem::Zfs => {
                cmds.push(format!("zpool create -f zroot {}", self.root_partition));
            }
        }
        cmds
    }
}

/// Installation configuration plan
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveInstallPlan {
    pub hostname: String,
    pub username: String,
    pub timezone: String,
    pub layout: PartitionLayout,
    pub persona: DesktopPersona,
    pub base_packages: Vec<String>,
}

impl LiveInstallPlan {
    pub fn new(disk: &str, hostname: &str, username: &str, persona: DesktopPersona) -> Self {
        let mut base_packages = vec![
            "sigma-base".to_string(),
            "linux".to_string(),
            "linux-firmware".to_string(),
            "networkmanager".to_string(),
            "pipewire".to_string(),
            "pipewire-pulse".to_string(),
        ];

        match persona {
            DesktopPersona::OmarchyHyprland => {
                base_packages.push("hyprland".to_string());
                base_packages.push("waybar".to_string());
                base_packages.push("wofi".to_string());
                base_packages.push("kitty".to_string());
                base_packages.push("wallust".to_string());
            }
            DesktopPersona::ArchMinimalCli => {
                base_packages.push("tmux".to_string());
                base_packages.push("neovim".to_string());
                base_packages.push("htop".to_string());
            }
            DesktopPersona::SigmaZenithDesktop => {
                base_packages.push("zenith-compositor".to_string());
                base_packages.push("zenith-shell".to_string());
            }
        }

        Self {
            hostname: hostname.to_string(),
            username: username.to_string(),
            timezone: "UTC".to_string(),
            layout: PartitionLayout::new_btrfs_default(disk),
            persona,
            base_packages,
        }
    }

    /// Generate pacstrap package bootstrap command
    pub fn pacstrap_command(&self) -> String {
        format!("pacstrap /mnt {}", self.base_packages.join(" "))
    }

    /// Generate fstab generation command
    pub fn genfstab_command(&self) -> String {
        "genfstab -U /mnt >> /mnt/etc/fstab".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_display_aaa() {
        assert_eq!(InstallFilesystem::Btrfs.to_string(), "btrfs");
        assert_eq!(InstallFilesystem::Ext4.to_string(), "ext4");
    }

    #[test]
    fn test_desktop_persona_display_aaa() {
        assert_eq!(DesktopPersona::OmarchyHyprland.to_string(), "omarchy-hyprland");
        assert_eq!(DesktopPersona::ArchMinimalCli.to_string(), "arch-minimal-cli");
    }

    #[test]
    fn test_btrfs_formatting_sequence_aaa() {
        let layout = PartitionLayout::new_btrfs_default("/dev/nvme0n1");
        let cmds = layout.format_commands();

        assert!(cmds.iter().any(|c| c.contains("mkfs.fat -F 32 /dev/nvme0n1p1")));
        assert!(cmds.iter().any(|c| c.contains("mkfs.btrfs -f /dev/nvme0n1p2")));
        assert!(cmds.iter().any(|c| c.contains("btrfs subvolume create /mnt/@home")));
        assert!(cmds.iter().any(|c| c.contains("btrfs subvolume create /mnt/@snapshots")));
    }

    #[test]
    fn test_live_install_plan_omarchy_persona_aaa() {
        let plan = LiveInstallPlan::new("/dev/sda", "sigma-pc", "user", DesktopPersona::OmarchyHyprland);

        assert_eq!(plan.hostname, "sigma-pc");
        assert_eq!(plan.username, "user");
        assert!(plan.base_packages.contains(&"hyprland".to_string()));
        assert!(plan.base_packages.contains(&"waybar".to_string()));
        assert!(plan.base_packages.contains(&"wallust".to_string()));

        let pacstrap = plan.pacstrap_command();
        assert!(pacstrap.starts_with("pacstrap /mnt"));
        assert!(pacstrap.contains("hyprland"));

        let fstab = plan.genfstab_command();
        assert_eq!(fstab, "genfstab -U /mnt >> /mnt/etc/fstab");
    }

    #[test]
    fn test_live_install_plan_arch_cli_persona_aaa() {
        let plan = LiveInstallPlan::new("/dev/vda", "arch-box", "admin", DesktopPersona::ArchMinimalCli);

        assert!(plan.base_packages.contains(&"neovim".to_string()));
        assert!(plan.base_packages.contains(&"tmux".to_string()));
        assert!(!plan.base_packages.contains(&"hyprland".to_string()));
    }
}
