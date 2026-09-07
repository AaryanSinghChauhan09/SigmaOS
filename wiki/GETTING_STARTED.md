# Getting Started with SigmaOS

Welcome to SigmaOS, the world's most advanced sovereign, bare-metal operating system for the next generation of silicon sovereignty.

## Table of Contents
- [Overview](#overview)
- [System Requirements](#system-requirements)
- [Installation Methods](#installation-methods)
- [First Boot](#first-boot)
- [Basic Usage](#basic-usage)
- [Configuration](#configuration)
- [Getting Help](#getting-help)

## Overview

SigmaOS is a revolutionary operating system designed from the ground up to provide:

- **Complete Digital Sovereignty**: Full control over your computing environment
- **Advanced Security**: Post-quantum cryptography, hardware-based isolation
- **Universal Compatibility**: Linux, BSD, Windows application support
- **AI Integration**: Built-in AI capabilities for system optimization
- **Modular Architecture**: Extensible shard-based system

## System Requirements

### Minimum Requirements
- **CPU**: 64-bit x86_64 or ARM64 processor (2+ cores)
- **RAM**: 4GB (8GB recommended)
- **Storage**: 32GB available space (64GB recommended)
- **Graphics**: Any DirectX 11 or Vulkan compatible GPU
- **Network**: Ethernet or Wi-Fi adapter

### Recommended Requirements
- **CPU**: Modern multi-core processor (8+ cores)
- **RAM**: 16GB or more
- **Storage**: 256GB NVMe SSD
- **Graphics**: Dedicated GPU with 4GB+ VRAM
- **Network**: Gigabit Ethernet + Wi-Fi 6

## Installation Methods

### 1. Live USB Installation
```bash
# Download the latest SigmaOS ISO
wget https://releases.sigmaos.org/latest/sigmaos-latest.iso

# Create bootable USB (replace /dev/sdX with your USB device)
sudo dd if=sigmaos-latest.iso of=/dev/sdX bs=4M status=progress

# Boot from USB and follow installation wizard
```

### 2. Dual Boot Installation
SigmaOS can coexist with other operating systems:
- Automatic dual-boot detection
- GRUB bootloader integration
- Secure Boot support

### 3. Virtual Machine
Perfect for testing:
```bash
# QEMU/KVM example
qemu-system-x86_64 -enable-kvm -m 8G -smp 4 \
  -drive if=virtio,file=sigmaos.qcow2,format=qcow2 \
  -netdev user,id=net0 -device virtio-net,netdev=net0
```

## First Boot

### Initial Setup Wizard
1. **Language & Region**: Select your preferred language and timezone
2. **Network Configuration**: Connect to Wi-Fi or configure Ethernet
3. **User Account**: Create your primary user account
4. **Security Setup**: Configure encryption and authentication
5. **Profile Selection**: Choose your usage profile (Developer, Professional, Gaming, etc.)

### Desktop Environment
SigmaOS boots into the Zenith Desktop by default:
- **Sovereign Explorer**: Advanced file manager
- **Neural Search**: AI-powered search across system and web
- **Task Manager**: Real-time system monitoring
- **Settings Panel**: System configuration interface

## Basic Usage

### Package Management
```bash
# Install software
sigpkg install firefox

# Search for packages
sigpkg search "video editor"

# Update system
sigpkg update && sigpkg upgrade

# Install from multiple package formats
sigpkg install app.deb        # Debian packages
sigpkg install app.rpm        # RPM packages
sigpkg install app.flatpak    # Flatpak apps
sigpkg install app.appimage   # AppImage files
```

### Command Line Interface
```bash
# Sigma Shell (enhanced bash compatibility)
sigma-sh

# System information
sigma-monitor --system

# Network configuration
sigma-net --status

# Security tools
sigma-secure --scan

# AI assistant
sigma-ai "optimize my system for gaming"
```

### Universal App Support
- **Linux Apps**: Native compatibility layer
- **Windows Apps**: Built-in Wine-based compatibility
- **BSD Apps**: FreeBSD compatibility layer
- **Android Apps**: Waydroid integration (coming soon)

## Configuration

### System Profiles
Switch between optimized configurations:
```bash
# Gaming profile
sigma-profile set gaming

# Development profile
sigma-profile set developer

# Server profile
sigma-profile set server

# Privacy-focused profile
sigma-profile set privacy
```

### Hardware Optimization
```bash
# GPU optimization
sigma-optimize --gpu

# CPU scheduler tuning
sigma-optimize --cpu --scheduler zen

# Memory management
sigma-optimize --memory --zram

# Storage optimization
sigma-optimize --storage --btrfs
```

## Getting Help

### Documentation
- **In-system help**: `man sigmaos` or `sigma-help`
- **Online wiki**: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
- **User manual**: `/usr/share/doc/sigmaos/`

### Community Support
- **GitHub Issues**: Report bugs and request features
- **Community Forum**: https://forum.sigmaos.org
- **Discord Server**: https://discord.gg/sigmaos
- **Telegram Group**: https://t.me/sigmaos_community

### Professional Support
- **Enterprise Support**: enterprise@sigmaos.org
- **Developer Support**: developer@sigmaos.org
- **Training Services**: Available for organizations

## Next Steps

1. **Explore the Desktop**: Familiarize yourself with Zenith Desktop
2. **Install Your Apps**: Use sigpkg to install your favorite software
3. **Configure Security**: Set up encryption and secure authentication
4. **Join the Community**: Connect with other SigmaOS users
5. **Customize Your System**: Explore themes, extensions, and shards

Welcome to the future of computing with SigmaOS!