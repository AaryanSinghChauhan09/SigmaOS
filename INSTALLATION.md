# SigmaOS Installation Guide

## Table of Contents
1. [Hardware Requirements](#hardware-requirements)
2. [Installation Methods](#installation-methods)
3. [Step-by-Step Installation](#step-by-step-installation)
4. [Post-Installation Configuration](#post-installation-configuration)
5. [Troubleshooting](#troubleshooting)

## Hardware Requirements

### Minimum Requirements
- **CPU**: x86_64 or ARM64 processor
- **RAM**: 512MB minimum, 2GB recommended
- **Storage**: 10GB minimum, 20GB recommended
- **Graphics**: VGA-compatible or better
- **Network**: Ethernet or wireless adapter (optional for basic installation)

### Recommended Requirements
- **CPU**: Multi-core processor (4+ cores)
- **RAM**: 4GB or more
- **Storage**: 50GB or more (SSD recommended)
- **Graphics**: GPU with 3D acceleration
- **Network**: High-speed network adapter

## Installation Methods

### 1. USB Installation
The most common and recommended method for installing SigmaOS.

#### Prerequisites
- USB drive (at least 4GB)
- Another computer to create the bootable USB
- SigmaOS ISO image

#### Creating Bootable USB
```bash
# On Linux
dd if=sigmaos.iso of=/dev/sdX bs=4M status=progress

# On macOS
sudo dd if=sigmaos.iso of=/dev/diskX bs=4m

# On Windows
Use Rufus or balenaEtcher
```

### 2. Network Installation (PXE)
Install SigmaOS over the network from a PXE server.

#### Setup
```bash
# Configure DHCP server
# Configure TFTP server
# Boot from network
```

### 3. Virtual Machine Installation
Install SigmaOS in a virtual machine for testing or development.

#### Supported Platforms
- QEMU/KVM
- VirtualBox
- VMware
- Hyper-V

## Step-by-Step Installation

### 1. Boot from Installation Media
1. Insert bootable USB or boot from network
2. Select "Install SigmaOS" from boot menu
3. Wait for system to boot

### 2. Configure System
```bash
# Set keyboard layout
sigsetup keyboard

# Set language
sigsetup language en_US

# Set timezone
sigsetup timezone America/New_York
```

### 3. Partition Disk
```bash
# Automatic partitioning (recommended for beginners)
sigpart auto /dev/sda

# Manual partitioning (advanced users)
sigpart manual /dev/sda
```

#### Recommended Partition Scheme
- **EFI System Partition**: 512MB (FAT32)
- **Boot Partition**: 1GB (ext4)
- **Root Partition**: Remaining space (ext4)
- **Swap Partition**: 2GB or equal to RAM (swap)

### 4. Install Base System
```bash
# Install base packages
siginstall base

# Install kernel
siginstall kernel

# Install bootloader
siginstall bootloader
```

### 5. Configure System
```bash
# Set hostname
sighostname my-sigmaos

# Set root password
sigpasswd root

# Create user
siguser add john
siguser set-password john
```

### 6. Configure Network
```bash
# Automatic DHCP
signet dhcp eth0

# Static IP
signet static eth0 192.168.1.100 255.255.255.0 192.168.1.1
```

### 7. Install Desktop Environment (Optional)
```bash
# Install Zenith Desktop
siginstall zenith-desktop

# Install additional applications
siginstall web-browser
siginstall office-suite
```

### 8. Finalize Installation
```bash
# Generate initramfs
sigmkinitramfs

# Update bootloader
sigupdate-bootloader

# Reboot
reboot
```

## Post-Installation Configuration

### 1. Update System
```bash
# Update package database
sigpkg update

# Upgrade all packages
sigpkg upgrade
```

### 2. Configure User Account
```bash
# Add user to sudoers
siguser sudo john

# Configure user groups
siguser group-add john wheel
siguser group-add john audio
siguser group-add john video
```

### 3. Configure Network
```bash
# Enable network services
siginit enable NetworkManager
siginit start NetworkManager

# Configure WiFi
nmcli device wifi connect "SSID" password "password"
```

### 4. Configure Desktop Environment
```bash
# Set default display manager
sigset-display-manager zenith

# Configure themes
sigtheme set sigmaos-dark

# Configure fonts
sigfont set "DejaVu Sans" 12
```

### 5. Install Additional Software
```bash
# Install development tools
siginstall base-devel
siginstall rust
siginstall git

# Install multimedia tools
siginstall vlc
siginstall gimp
siginstall blender
```

## Troubleshooting

### Boot Issues
#### System won't boot
- Check boot order in BIOS/UEFI
- Verify bootloader installation
- Try legacy BIOS mode if UEFI fails

#### Boot hangs
- Boot with single-user mode
- Check kernel logs with `dmesg`
- Disable non-essential services

### Installation Issues
#### Partition errors
- Ensure disk is not mounted
- Check disk health with `fsck`
- Try different partitioning scheme

#### Package installation fails
- Check network connection
- Update package database
- Clear package cache

### Network Issues
#### No network connection
- Check cable connection
- Verify network interface with `ip link`
- Check network configuration

#### WiFi won't connect
- Verify WiFi hardware is detected
- Check driver support
- Try WPA2 instead of WPA3

### Desktop Environment Issues
#### Desktop won't start
- Check X server logs
- Verify graphics driver is loaded
- Try different display manager

#### Graphics issues
- Install proper GPU driver
- Check GPU support
- Try software rendering

## Additional Resources

- [SigmaOS Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- [SigmaOS GitHub Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
- [Community Forums](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

## Next Steps

After installation, consider:
- [System Configuration Guide](./CONFIGURATION.md)
- [Package Management Guide](./PACKAGE_MANAGEMENT.md)
- [Security Hardening Guide](./SECURITY.md)
- [Development Guide](./DEVELOPMENT.md)