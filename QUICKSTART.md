# SigmaOS Quick Start Guide

> Get SigmaOS running in under 10 minutes.

***

## 🚀 Prerequisites

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| **CPU** | x86\_64, AArch64, or RISC-V | AMD Zen 3+ / Intel Alder Lake+ |
| **RAM** | 512 MB | 8 GB+ |
| **Storage** | 4 GB | 50 GB+ NVMe |
| **GPU** | Any (software rendering) | Vulkan-capable |
| **Rust** | 1.75+ | Latest stable |

***

## 💾 Installation Options

### Option 1: ISO (Recommended for Hardware)

```bash
# Download latest ISO
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/latest/SigmaOS-x86_64.iso

# Verify integrity
sha256sum SigmaOS-x86_64.iso

# Flash to USB (Linux)
dd if=SigmaOS-x86_64.iso of=/dev/sdX bs=4M status=progress

# Flash to USB (macOS)
diskutil unmountDisk /dev/diskN
dd if=SigmaOS-x86_64.iso of=/dev/rdiskN bs=4m
```

### Option 2: QEMU Virtual Machine

```bash
# Install QEMU
apt install qemu-system-x86  # Debian/Ubuntu
brew install qemu             # macOS

# Run SigmaOS in QEMU
qemu-system-x86_64 \
  -m 2G \
  -enable-kvm \
  -cdrom SigmaOS-x86_64.iso \
  -boot d \
  -display gtk \
  -net nic -net user
```

### Option 3: Build from Source

```bash
# Clone repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview

# Install build dependencies
sudo apt install -y nasm grub-pc-bin grub-efi-amd64-bin xorriso mtools

# Build SigmaOS
cargo build --release

# Build bootable ISO
bash scripts/build-iso.sh

# Run in QEMU
bash scripts/run-qemu.sh
```

***

## 📱 First Boot Experience

### 1. Language & Locale

    Welcome to SigmaOS!
    > Select language: [English] [Hindi] [Spanish] [French] [German] [Chinese]
    > Select timezone: [Auto-detect] [Manual]
    > Select keyboard layout: [US QWERTY] [DVORAK] [AZERTY]

### 2. User Setup

    > Create username: sigma_user
    > Set password: ***
    > Enable sudo: [Yes] [No]
    > Full disk encryption: [Yes (recommended)] [No]

### 3. Desktop Selection

    Choose desktop environment:
    > [1] Pantheon Desktop (Elementary-inspired, recommended)
    > [2] Minimal (no desktop, CLI only)
    > [3] GNOME Compatibility Mode
    > [4] KDE Plasma Compatibility Mode

***

## 📦 Package Management

```bash
# Update system
sigma-pkg update
sigma-pkg upgrade

# Search packages
sigma-pkg search firefox

# Install package
sigma-pkg install firefox

# Install from User Package Store (AUR-equivalent)
sigma-ups install visual-studio-code

# Install Flatpak (sandboxed)
sigma-pkg install --flatpak com.spotify.Client

# Remove package
sigma-pkg remove firefox

# Clean cache
sigma-pkg clean
```

***

## 🔧 Essential Configuration

### Network

```bash
# Configure wired network (DHCP)
sigma-net connect --dhcp eth0

# Configure WiFi
sigma-net wifi list
sigma-net wifi connect "MyNetwork" --password "mypassword"

# Configure VPN
sigma-net vpn add wireguard my-vpn /path/to/wg0.conf
sigma-net vpn connect my-vpn
```

### Audio

```bash
# List audio devices
sigma-audio list

# Set default output
sigma-audio set-default --output "Built-in Audio"

# Volume control
sigma-audio volume set 75
```

### Display

```bash
# List displays
sigma-display list

# Set resolution
sigma-display set --monitor HDMI-1 --resolution 1920x1080 --rate 144

# Multi-monitor setup
sigma-display extend --left HDMI-1 --right DP-1
```

***

## 🧠 AI Features

```bash
# Start AI assistant
sigma-ai start

# Ask a question
sigma-ai ask "How do I configure firewall rules?"

# Enable voice interface
sigma-ai voice --enable

# Run local LLM (Llama 3)
sigma-ai llm --model llama3-8b --interactive
```

***

## 🛠️ Developer Setup

```bash
# Install development tools
sigma-pkg install --group development

# Enable dev container
code .  # VS Code detects .devcontainer automatically

# Run tests
cargo test

# Build in release mode
cargo build --release

# Generate documentation
cargo doc --open
```

***

## 🧪 Virtualization

```bash
# Create VM
sigma-vm create ubuntu-24 \
  --image ubuntu-24.04.iso \
  --ram 4G \
  --disk 50G \
  --type kvm

# Start VM
sigma-vm start ubuntu-24

# Connect to VM console
sigma-vm console ubuntu-24

# Live snapshot
sigma-vm snapshot ubuntu-24 --name pre-upgrade

# Restore snapshot
sigma-vm restore ubuntu-24 --snapshot pre-upgrade
```

***

## 📚 Getting Help

| Resource | Link |
|----------|------|
| **Documentation** | [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) |
| **Issues** | [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) |
| **Discussions** | [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions) |
| **Source** | [Repository](https://github.com/AaryanSinghChauhan09/SigmaOS) |

```bash
# Built-in help
sigma-ctl help
sigma-pkg --help
man sigma-net
```

***

*SigmaOS Quick Start Guide | Updated: 2026-08-23*
