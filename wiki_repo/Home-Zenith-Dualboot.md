# ⚡ SigmaOS v15.0 Zenith — Dual-Boot Edition

> **Run SigmaOS alongside Windows or Linux. One GRUB. Two worlds. Zero compromise.**

[![Release](https://img.shields.io/badge/release-v15.0--zenith--dualboot-orange)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-dualboot)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64-green)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Compatibility](https://img.shields.io/badge/dual--boot-Windows%20%7C%20Linux-blue)](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 📋 Overview

**SigmaOS Zenith Dual-Boot** is the coexistence edition designed for professionals who need SigmaOS's sovereign performance alongside an existing Windows or Linux installation. It installs safely alongside your current OS using a shared GRUB2 bootloader with SigmaOS-sovereign theming.

The dual-boot edition **never touches your existing OS partitions** — it uses GPT-safe placement with full partition table awareness, and its GRUB configuration is fully reversible.

| Property | Value |
|---|---|
| Edition | Zenith Dual-Boot |
| Version | v15.0.0 |
| Kernel | Sovereign Lattice Microkernel v15.0 |
| Boot Manager | GRUB2 (Sovereign-Themed) + rEFInd compatible |
| Compatible With | Windows 10/11, Ubuntu 20.04+, Fedora 36+, Arch Linux, Debian 11+ |
| Architecture | x86_64, ARM64 |
| Target | Machines with existing Windows or Linux installations |
| Desktop | Zenith Industrial Desktop (Z-DESKTOP) |

---

## ⚡ Key Features

### 🔀 Coexistence Engine

- **GPT-Safe Installer**: Reads existing partition table before writing — never overwrites foreign partitions
- **Windows Fast Startup Aware**: Detects and disables Windows Fast Startup to prevent filesystem corruption during coexistence
- **NTFS-Safe Mode**: Can read Windows NTFS partitions (read-only by default for safety)
- **Linux EFI Share**: Shares EFI System Partition with existing Linux — no EFI duplication
- **Reversible Install**: Uninstaller restores original bootloader state completely
- **S-GUARD Isolation**: Kernel-level isolation ensures SigmaOS processes never contaminate cross-OS shared volumes

### 🔒 Security in Dual-Boot Context

- **Per-OS TPM PCR Isolation**: Each OS gets isolated TPM Platform Configuration Registers — keys never cross OS boundaries
- **PQC-Hardened Keychain**: SigmaOS keypairs are isolated from Windows Certificate Store and Linux keyring
- **S-ARMOR Cross-Boundary**: Prevents shared mount points from leaking to SigmaOS process table
- **Secure Boot Key Management**: SigmaOS enrolls its own Secure Boot keys in a dedicated MOK (Machine Owner Key) database

### 🚀 Full Zenith Performance

- All Standalone performance features active on SigmaOS partition
- **S-CFS Scheduler**: Hardware-native scheduling when running SigmaOS
- **Sovereign Memory Management**: Full NUMA-aware paging on SigmaOS boot
- **NVMe Direct I/O**: Available on SigmaOS partitions (isolated from Windows storage)

### 🖥️ Dual-Boot Bootloader

- **GRUB2 Sovereign Theme**: Graphical boot menu with SigmaOS identity
- **Timeout Configuration**: Configurable OS selection timeout (default: 10 seconds)
- **Quick-Select Shortcuts**: Keyboard shortcuts for instant OS selection
- **Boot Entry Manager**: `sigma-bootmgr` CLI to add/remove OS entries
- **Windows Hibernation Detection**: Warns if Windows hibernation is active before mounting

---

## 💻 System Requirements

| Component | Minimum | Recommended |
|---|---|---|
| CPU | x86_64 (SSE4.2+) | Intel 10th Gen+ / AMD Zen 3+ |
| RAM | 8 GB | 16 GB+ (for comfortable cross-OS use) |
| Free Storage | 25 GB unallocated | 80 GB+ unallocated NVMe |
| Firmware | UEFI 2.4+ | UEFI 2.6+ |
| Existing OS | Windows 10+, Ubuntu 20.04+, or compatible Linux | Windows 11 or Ubuntu 22.04+ |
| EFI Partition | Shared (>=200MB free) | Dedicated (512MB) |

---

## 🛠️ Installation Guide

### ⚠️ Before You Begin — Critical Checklist

> **Do these steps BEFORE booting the installer:**

- [ ] **Back up all critical data** on your existing OS
- [ ] **Disable Windows Fast Startup**: `Control Panel → Power Options → Choose what power buttons do → Turn on fast startup → UNCHECK`
- [ ] **Disable Windows Hibernation**: Open CMD as Admin → `powercfg /h off`
- [ ] **Shrink your existing partition** to create free space (at least 25 GB unallocated)
- [ ] **Note your existing OS drive letters/partition UUIDs** for reference
- [ ] **Disable BitLocker** on any Windows drives (can be re-enabled after install)

### Step 1 — Shrink Existing Partition

**Windows (Disk Management):**

```
1. Right-click Start → Disk Management
2. Right-click OS partition (usually C:) → Shrink Volume
3. Enter size to shrink (minimum 25600 MB for SigmaOS)
4. Click Shrink — unallocated space appears
```

**Linux (GParted / parted):**

```bash
sudo apt install gparted         # Install GParted if needed

sudo gparted                     # Open GParted

# Right-click your partition → Resize/Move → Shrink from right side

# Apply changes — creates unallocated space
```

### Step 2 — Download and Flash ISO

```bash

# Download Dual-Boot ISO

curl -LO https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0-zenith-dualboot/SigmaOS-v15.0-Zenith-Dualboot-x86_64.iso

# Flash to USB (Linux/macOS)

sudo dd if=SigmaOS-v15.0-Zenith-Dualboot-x86_64.iso of=/dev/sdX bs=4M status=progress && sync
```

### Step 3 — Boot the Dual-Boot Installer

1. Boot from USB → Select **"Install SigmaOS Dual-Boot"**
2. The installer detects existing OSes automatically:
   - Windows: Shows Windows drive, version, and EFI partition
   - Linux: Shows distro name, kernel version, and EFI partition
3. Installer confirms safe coexistence — shows partition map preview

### Step 4 — Partition Configuration

```

# Installer will show unallocated space and suggest layout:

[EXISTING OS PARTITION]  ←  UNTOUCHED
[UNALLOCATED SPACE]  →  SigmaOS installer claims this:
  └─ /boot/efi  (shared with existing, or new 512MB)
  └─ swap       (4–8 GB, encrypted)
  └─ /          (20 GB+, SLF filesystem)
  └─ /home      (remaining space)
```

### Step 5 — GRUB2 Configuration

The installer automatically detects and adds all existing OS entries:

```bash

# Preview of generated grub.cfg entries:

menuentry "SigmaOS v15.0 Zenith" { ... }
menuentry "Windows 11 (on /dev/sda1)" { ... }
menuentry "Ubuntu 22.04 LTS (on /dev/sda5)" { ... }
```

Customization options:
- Default OS selection (which OS boots without input)
- Timeout duration (5–30 seconds)
- Boot resolution (1080p/4K graphical menu)

### Step 6 — Complete & Reboot

1. Review final summary — confirm no existing partitions modified
2. Click **Install** — completes in 8–20 minutes
3. Reboot → GRUB2 Sovereign menu appears
4. Select SigmaOS for first-boot configuration

### Step 7 — First Boot in Dual-Boot Mode

```bash

# Dual-boot specific post-install

sigma-bootmgr --list                    # Show all detected OS entries

sigma-bootmgr --set-default sigmaos     # Set SigmaOS as default

sigma-bootmgr --timeout 10              # 10-second selection window

sigma-dualcoex --detect-windows         # Detect Windows partition safely

sigma-dualcoex --mount-ntfs /dev/sda1   # Mount Windows partition (read-only)

```

---

## 🔧 Dual-Boot Management Functions

### sigma-bootmgr — Boot Manager CLI

```bash
sigma-bootmgr --list                    # List all boot entries

sigma-bootmgr --add "Windows 11" /dev/sda1  # Add boot entry manually

sigma-bootmgr --remove <entry-id>       # Remove a boot entry

sigma-bootmgr --set-default <entry-id>  # Set default OS

sigma-bootmgr --timeout <seconds>       # Set selection timeout

sigma-bootmgr --repair                  # Repair GRUB2 if corrupted

sigma-bootmgr --restore-windows-boot    # Restore original Windows bootloader

```

### sigma-dualcoex — Coexistence Manager

```bash
sigma-dualcoex --detect-all             # Detect all installed OSes

sigma-dualcoex --mount-ntfs /dev/sda1   # Mount Windows NTFS (read-only)

sigma-dualcoex --unmount-ntfs           # Safely unmount Windows volumes

sigma-dualcoex --check-fastboot         # Check if Windows FastBoot is active

sigma-dualcoex --check-hibernation      # Check for Windows hibernation file

sigma-dualcoex --share-files /mnt/win/Users/Shared  # Access shared folder

```

### Cross-OS File Sharing

```bash

# Access Windows user files (read-only, safe)

ls /mnt/windows/Users/YourName/Documents

# Shared data directory (writable — on dedicated exFAT or FAT32 partition)

ls /mnt/shared/
sigma-dualcoex --create-shared-partition --size 5G  # Create shared space

```

---

## 🔄 Uninstalling SigmaOS (Reversing Dual-Boot)

```bash

# From SigmaOS (removes SigmaOS, restores original bootloader)

sigma-bootmgr --uninstall-sigmaos

# From Windows (if SigmaOS is already removed from disk)

# Boot into Windows Recovery → Advanced Options → Command Prompt

bootrec /fixmbr
bootrec /fixboot
bootrec /rebuildbcd
```

---

## 📊 Compatibility Matrix

| OS | Dual-Boot Support | Shared EFI | NTFS Access | Notes |
|---|---|---|---|---|
| Windows 10 | ✅ Full | ✅ Yes | ✅ Read-Only | Disable Fast Startup |
| Windows 11 | ✅ Full | ✅ Yes | ✅ Read-Only | Disable Hibernation + Fast Startup |
| Ubuntu 20.04+ | ✅ Full | ✅ Yes | N/A | GRUB2 chain-load |
| Fedora 36+ | ✅ Full | ✅ Yes | N/A | GRUB2 chain-load |
| Arch Linux | ✅ Full | ✅ Yes | N/A | Manual GRUB entry may be needed |
| macOS | ⚠️ Partial | ❌ No | N/A | rEFInd required on Intel Mac |
| Debian 11+ | ✅ Full | ✅ Yes | N/A | GRUB2 chain-load |

---

## 🆘 Support & Resources

- **Release Page**: [v15.0-zenith-dualboot](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-dualboot)
- **Dual-Boot Coexistence Guide**: [Dual-Boot-Coexistence](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Dual-Boot-Coexistence)
- **Compatibility Matrix**: [Dual-Boot-Compatibility-Matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Dual-Boot-Compatibility-Matrix)
- **Issue Tracker**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

---

*SigmaOS v15.0 Zenith Dual-Boot — Sovereign power without leaving the familiar behind.*
