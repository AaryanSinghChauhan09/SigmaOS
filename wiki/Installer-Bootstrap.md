# SigmaOS Installer Bootstrap Components

## Overview
SigmaOS installer providing interactive and unattended installation modes, competitive with Arch archinstall, Mint Ubiquity, and Omarchy ISO installer.

## Features
- **Interactive and unattended installation modes**
- **Btrfs snapshots support** (like Omarchy)
- **Dual-boot detection** (like Omarchy)
- **Secure defaults** (like OpenBSD)
- **Minimal kernel philosophy** (only what's needed)
- **Multiple bootloader options**: GRUB2, systemd-boot, rEFInd
- **Multiple filesystems**: Btrfs, Ext4, XFS
- **Fail-fast validation** before installation
- **Layered stage-based execution**

## Structures

| Structure | Description |
|-----------|-------------|
| **Installer** | Main installer with stage-based execution |
| **InstallConfig** | Installation configuration (mode, target, bootloader, etc.) |
| **InstallTarget** | Target disk and partition layout |
| **InstallMode** | Interactive, Unattended, OEM |
| **BootloaderType** | Grub2, SystemdBoot, RefInd |
| **FilesystemType** | Btrfs, Ext4, Xfs |
| **InstallStage** | HardwareDetection → Partitioning → Formatting → BaseInstall → BootloaderInstall → SystemConfiguration → PackageInstall → Finalize |
| **InstallResult** | Success, Failed, Cancelled |

## Installation Stages

1. **HardwareDetection** - Detect available disks, CPU, memory
2. **Partitioning** - Partition target disk
3. **Formatting** - Format partitions with chosen filesystems
4. **BaseInstall** - Install base SigmaOS system
5. **BootloaderInstall** - Install chosen bootloader with dual-boot support
6. **SystemConfiguration** - Set username, hostname, timezone, locale
7. **PackageInstall** - Install additional packages via sigpkg
8. **Finalize** - Finalize and prepare for reboot

## Status
- **Implementation**: Complete
- **Dependencies**: sigpkg, bootloader, filesystem tools
- **Roadmap Phase**: Installer (P1)

## Related Files
- `src/installer/bootstrap.rs` - Installer implementation
- `wiki/Installer-Bootstrap.md` - Wiki documentation

## Principles Followed
BOOTLOADER_INTEGRATION, MINIMAL_KERNEL_PHILOSOPHY, KEEP_IT_SIMPLE_STUPID,
LAYERED DESIGN, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM, SECURE_DEFAULTS,
DISCOVERABILITY