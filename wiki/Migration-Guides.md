# SigmaOS Migration Guides

**Last Updated:** July 6, 2026  
**Version:** v16.3.0 Foundation

---

## Overview

This document provides comprehensive migration guides for users transitioning from other operating systems and Linux distributions to SigmaOS. Each guide covers the essential steps, considerations, and tools needed for a smooth migration.

---

## Table of Contents

1. [Migrating from Ubuntu](#migrating-from-ubuntu)
2. [Migrating from Fedora](#migrating-from-fedora)
3. [Migrating from Debian](#migrating-from-debian)
4. [Migrating from Arch Linux](#migrating-from-arch-linux)
5. [Migrating from Windows](#migrating-from-windows)
6. [Migrating from macOS](#migrating-from-macos)
7. [Application Migration](#application-migration)
8. [Data Migration](#data-migration)
9. [Configuration Migration](#configuration-migration)

---

## Migrating from Ubuntu

### Pre-Migration Checklist

- Backup all important data using external storage or cloud backup
- Document installed applications and configurations
- Note custom repositories and PPAs
- Export browser bookmarks and passwords
- Save SSH keys and GPG keys

### Installation

1. Download SigmaOS installer from official repository
2. Create bootable USB using Rufus (Windows) or dd (Linux)
3. Boot from USB and run Calamares-style installer
4. Select dual-boot option if keeping Ubuntu
5. Complete installation and reboot

### Post-Installation

#### Package Management

Ubuntu uses `apt`, SigmaOS uses `sigpkg`:

```bash
# Ubuntu
sudo apt install package

# SigmaOS
sigpkg install package
```

#### Systemd Replacements

SigmaOS uses native implementations:

- **systemd-coredump** → `sigma_coredump`
- **systemd-networkd** → Native network drivers
- **systemd-resolved** → Native DNS resolver

#### Desktop Environment

If using GNOME on Ubuntu, SigmaOS includes GNOME with Zenith Desktop integration. Your GNOME settings will be preserved during migration.

---

## Migrating from Fedora

### Pre-Migration Checklist

- Backup using Timeshift or similar tool
- Document RPM Fusion packages
- Save SELinux configurations
- Export Wayland configurations
- Note custom COPR repositories

### Installation

1. Download SigmaOS installer
2. Create bootable USB using Fedora Media Writer or dd
3. Boot and run installer
4. Select dual-boot if keeping Fedora
5. Complete installation

### Post-Installation

#### Package Management

Fedora uses `dnf`, SigmaOS uses `sigpkg`:

```bash
# Fedora
sudo dnf install package

# SigmaOS
sigpkg install package
```

#### SELinux

SigmaOS uses QubesOS-style sandboxing instead of SELinux. Security policies are managed through the native sandbox system.

#### Wayland

SigmaOS supports Wayland natively. Your Wayland configurations should work with minimal adjustments.

---

## Migrating from Debian

### Pre-Migration Checklist

- Backup using Borg or Duplicity
- Document backports repository usage
- Save /etc configurations
- Export APT sources
- Note custom kernel modules

### Installation

1. Download SigmaOS installer
2. Create bootable USB
3. Boot and run installer
4. Select dual-boot if keeping Debian
5. Complete installation

### Post-Installation

#### Package Management

Debian uses `apt`, SigmaOS uses `sigpkg`:

```bash
# Debian
sudo apt install package

# SigmaOS
sigpkg install package
```

#### Stability Focus

Like Debian, SigmaOS emphasizes stability. The custom kernel with latest Linux integration provides stable hardware support.

---

## Migrating from Arch Linux

### Pre-Migration Checklist

- Backup entire system using rsync
- Document AUR packages
- Save pacman configuration
- Export custom PKGBUILD files
- Note custom kernel parameters

### Installation

1. Download SigmaOS installer
2. Create bootable USB
3. Boot and run installer
4. Select dual-boot if keeping Arch
5. Complete installation

### Post-Installation

#### Package Management

Arch uses `pacman`, SigmaOS uses `sigpkg`:

```bash
# Arch
sudo pacman -S package

# SigmaOS
sigpkg install package
```

#### AUR Packages

SigmaOS includes many AUR packages natively. For packages not available, use the sigpkg build system similar to AUR.

#### Customization

SigmaOS maintains the flexibility of Arch while providing pre-configured stability. Manual configuration files are respected.

---

## Migrating from Windows

### Pre-Migration Checklist

- Backup using Windows Backup or third-party tool
- Export Outlook emails and contacts
- Save installed software list
- Document Windows-specific settings
- Backup Windows Registry if needed

### Installation

1. Download SigmaOS installer
2. Create bootable USB using Rufus
3. Boot from USB (may need to disable Secure Boot)
4. Run Calamares-style installer with dual-boot support
5. Shrink Windows partition during installation
6. Complete installation and reboot

### Post-Installation

#### File System

Windows uses NTFS, SigmaOS uses native filesystem. Your Windows partition will remain accessible for data transfer.

#### Applications

Windows applications won't run natively. Use SigmaOS native alternatives:

- **Microsoft Office** → LibreOffice (bundled)
- **Adobe Photoshop** → GIMP (bundled)
- **Chrome** → Native browser or Firefox
- **Steam** → Native Steam with Proton for Windows games

#### Drivers

SigmaOS includes native drivers for most hardware. Windows-specific drivers are not needed.

#### Bootloader

SigmaOS installs GRUB as bootloader, allowing selection between Windows and SigmaOS at boot.

---

## Migrating from macOS

### Pre-Migration Checklist

- Backup using Time Machine
- Export Photos library
- Save Keychain passwords
- Document macOS-specific applications
- Backup using Migration Assistant

### Installation

1. Download SigmaOS installer
2. Create bootable USB (may require special tools for Mac)
3. Boot from USB (hold Option key during boot)
4. Run installer
5. Partition disk for dual-boot
6. Complete installation

### Post-Installation

#### File System

macOS uses APFS/HFS+, SigmaOS uses native filesystem. macOS partition remains accessible.

#### Applications

macOS applications won't run natively. Use SigmaOS alternatives:

- **Pages/Numbers/Keynote** → LibreOffice
- **Final Cut Pro** → Native video editor
- **Logic Pro** → Native audio tools
- **Safari** → Native browser or Firefox

#### Hardware

SigmaOS includes native drivers for Mac hardware. Touch Bar support may require additional configuration.

#### Boot

Use rEFBoot or similar tool to select between macOS and SigmaOS at boot.

---

## Application Migration

### Office Suites

| From | To (SigmaOS) |
|------|--------------|
| Microsoft Office | LibreOffice |
| Google Docs | Native office suite |
| WPS Office | LibreOffice |

### Development Tools

| From | To (SigmaOS) |
|------|--------------|
| VS Code | Native editor with AI suggestions |
| JetBrains IDEs | Native alternatives |
| Docker | Native container support |
| Kubernetes | Native container orchestration |

### Media Tools

| From | To (SigmaOS) |
|------|--------------|
| Adobe Photoshop | GIMP |
| Adobe Premiere | Native video editor |
| Blender | Blender (bundled) |
| Inkscape | Inkscape (bundled) |

### Communication

| From | To (SigmaOS) |
|------|--------------|
| Slack | Native messaging |
| Discord | Native client |
| Zoom | Native video conferencing |
| Microsoft Teams | Native collaboration tools |

---

## Data Migration

### User Data

Use the following methods to migrate user data:

1. **External Storage**: Copy to USB drive, then copy to SigmaOS
2. **Network Transfer**: Use SCP or SFTP between systems
3. **Cloud Sync**: Upload to cloud, download to SigmaOS
4. **Dual-Boot Access**: Access Windows/macOS partition from SigmaOS

### Application Data

#### Browser Data

- **Chrome**: Sign in to sync bookmarks, passwords, history
- **Firefox**: Use Firefox Sync
- **Edge**: Export bookmarks, import to SigmaOS browser

#### Email

- **Outlook**: Export to PST, import to Thunderbird
- **Gmail**: Use IMAP to sync to native email client
- **Apple Mail**: Export to mbox format

#### Documents

- **Office Documents**: Open directly in LibreOffice
- **PDFs**: Native PDF viewer
- **Images**: Native image viewer

---

## Configuration Migration

### Shell Configuration

#### Bash to SigmaOS Shell

```bash
# Copy bash configurations
cp ~/.bashrc ~/.bash_profile ~/

# SigmaOS uses native shell with AI suggestions
# Most bash configurations will work
```

#### SSH Keys

```bash
# Copy SSH directory
cp -r ~/.ssh ~/

# Set correct permissions
chmod 700 ~/.ssh
chmod 600 ~/.ssh/id_rsa
```

#### GPG Keys

```bash
# Export GPG keys
gpg --export-secret-keys > private.key
gpg --export > public.key

# Import on SigmaOS
gpg --import private.key
gpg --import public.key
```

### Network Configuration

#### Wi-Fi

```bash
# Import Wi-Fi configurations from NetworkManager
nmcli connection show
nmcli connection export <SSID>
```

#### Static IP

Configure through native network manager or edit network configuration files.

### System Configuration

#### Hostname

```bash
# Set hostname
hostnamectl set-hostname new-hostname
```

#### Locale

```bash
# Set locale
localectl set-locale LANG=en_US.UTF-8
```

#### Timezone

```bash
# Set timezone
timedatectl set-timezone America/New_York
```

---

## Troubleshooting

### Common Issues

#### Boot Issues

If dual-boot doesn't work:

1. Boot from live USB
2. Reinstall GRUB to MBR/EFI
3. Update GRUB configuration
4. Check BIOS/UEFI boot order

#### Driver Issues

If hardware doesn't work:

1. Check native drivers in `/drivers/`
2. Use `sigpkg install` for additional drivers
3. Check kernel logs with `dmesg`
4. Report issues on GitHub

#### Application Issues

If applications don't work:

1. Check if native alternative exists
2. Use sigpkg to install missing dependencies
3. Check application logs
4. Use AI error explanation layer for help

---

## Additional Resources

- [SigmaOS Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- [Implementation Progress](Implementation-Progress.md)
- [Contributor Onboarding](Contributor-Onboarding.md)
- [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

---

## Support

For migration assistance:

1. Check this documentation
2. Search GitHub Issues
3. Create new issue with detailed information
4. Join community discussions

---

**Note:** This document is continuously updated as SigmaOS evolves. Check for the latest version before migrating.
