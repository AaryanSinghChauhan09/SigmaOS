# Getting Started with SigmaOS

Welcome to SigmaOS! This guide will help you get started with the world's most advanced sovereign, bare-metal operating system.

## Table of Contents

- [System Requirements](#system-requirements)
- [Installation](#installation)
- [First Boot](#first-boot)
- [Basic Configuration](#basic-configuration)
- [Using the Package Manager](#using-the-package-manager)
- [Development Setup](#development-setup)
- [Getting Help](#getting-help)

## System Requirements

### Minimum Requirements

- **CPU**: x86_64 (64-bit) processor with SSE4.2 support
- **RAM**: 2 GB minimum, 4 GB recommended
- **Storage**: 10 GB minimum, 20 GB recommended
- **Graphics**: VESA-compatible GPU or supported GPU driver
- **Boot**: UEFI with Secure Boot support (optional but recommended)

### Recommended Requirements

- **CPU**: Multi-core x86_64 processor (4+ cores)
- **RAM**: 8 GB or more
- **Storage**: 50 GB SSD or NVMe
- **Graphics**: NVIDIA, AMD, or Intel GPU with supported driver
- **Network**: Ethernet or Wi-Fi adapter

### Supported Architectures

- x86_64 (primary)
- ARM64 (experimental)
- RISC-V (experimental)

## Installation

### Downloading SigmaOS

1. Visit the [SigmaOS Downloads](https://github.com/AaryanSinghChauhan09/SigmaOS/releases) page
2. Download the latest stable release ISO image
3. Verify the download using the provided BLAKE3 checksum

### Creating Boot Media

#### On Linux

```bash
# Identify your USB device
lsblk

# Write the ISO to USB (replace /dev/sdX with your device)
sudo dd if=SigmaOS-x.x.x.iso of=/dev/sdX bs=4M status=progress conv=fsync
sync
```

#### On Windows

Use [Rufus](https://rufus.ie/) or [Etcher](https://www.balena.io/etcher/) to write the ISO to your USB drive.

### Booting the Installer

1. Insert the boot media
2. Boot from USB (usually F12 or F2 during boot)
3. Select "SigmaOS Installer" from the boot menu
4. Follow the graphical installer prompts

### Installation Steps

1. **Welcome**: Select your language and keyboard layout
2. **Disk Setup**: Choose automatic partitioning or manual configuration
3. **User Setup**: Create your user account and set password
4. **Profile Selection**: Choose your deployment profile:
   - **Standalone**: Full desktop experience
   - **Server**: Headless server profile
   - **Minimal**: Minimal footprint for embedded systems
5. **Installation**: Review and confirm installation settings
6. **Complete**: Reboot into your new SigmaOS system

## First Boot

### Initial Setup Wizard

On first boot, SigmaOS will guide you through:

1. **Network Configuration**: Set up wired or wireless networking
2. **Time Zone**: Configure your time zone and NTP settings
3. **System Updates**: Check for and install updates
4. **Desktop Setup**: Configure Zenith Desktop (if selected)
5. **Security Setup**: Set up capability-based security policies

### Default Credentials

- **Root**: Disabled by default (use sudo)
- **User**: Created during installation

## Basic Configuration

### Updating the System

```bash
# Update package cache
sigpkg update

# Upgrade all packages
sigpkg upgrade-all
```

### Enabling Services

```bash
# List available services
sigctl list-services

# Enable a service
sigctl enable <service-name>

# Start a service
sigctl start <service-name>
```

### Configuring Capabilities

SigmaOS uses capability-based security. Configure process capabilities:

```bash
# View process capabilities
sigcap list <pid>

# Grant capability to process
sigcap grant <pid> <capability>

# Revoke capability from process
sigcap revoke <pid> <capability>
```

## Using the Package Manager

SigmaOS uses `.spkg` (Sovereign Package) format with the `sigpkg` tool.

### Basic Commands

```bash
# Search for packages
sigpkg search <query>

# Install a package
sigpkg install <package-name>

# Remove a package
sigpkg remove <package-name>

# Upgrade a package
sigpkg upgrade <package-name>

# List installed packages
sigpkg list-installed
```

### Transaction Management

SigmaPKG supports atomic transactions with rollback:

```bash
# Enable rollback
sigpkg set-rollback true

# View transaction history
sigpkg history

# Rollback to previous state
sigpkg rollback <transaction-id>
```

### AI-Assisted Dependency Resolution

Enable AI-assisted package management:

```bash
sigpkg set-ai-assisted true
```

## Development Setup

### Prerequisites

- Rust toolchain (stable)
- Nim compiler
- Zig compiler
- QEMU (for testing)
- Git

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust toolchain
rustup install stable
rustup component add clippy rustfmt

# Install Nim (see https://nim-lang.org/install.html)
# Install Zig (see https://ziglang.org/download/)

# Build SigmaOS
make build
```

### Running in QEMU

```bash
# Run SigmaOS in QEMU
make qemu
```

### Running Tests

```bash
# Run all tests
make test

# Run unit tests only
make test-unit

# Run integration tests
make test-integration
```

For detailed development guidelines, see [Contributing.md](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing).

## Getting Help

### Documentation

- [Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture)
- [Development Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Development-Roadmap)
- [Security Policy](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Policy)
- [Changelog](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Changelog)

### Community

- **GitHub Issues**: [Report bugs](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **Discord**: [Real-time chat](https://discord.gg/sigmaos)

### Reporting Issues

When reporting issues, please include:

- SigmaOS version (`sigctl version`)
- Hardware specifications
- Error messages and logs
- Steps to reproduce the issue

For security vulnerabilities, see [Security Policy](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Policy).

## Next Steps

- Explore the [Zenith Desktop](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Zenith-Desktop) environment
- Learn about [AI & Automation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/AI-And-Automation) features
- Set up [development environment](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing) to contribute
- Read the [Architecture documentation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture)

---

*Last Updated: 2026-07-14*
