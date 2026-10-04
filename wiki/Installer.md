# Installer

SigmaOS ships a guided, fault-tolerant installer written entirely in Rust. It supports graphical (TUI + GUI) and headless modes, UEFI + BIOS, full disk encryption, RAID, Btrfs/SigmaFS, and automatic post-install configuration — outclassing Mint's Ubiquity, Calamares, and Omarchy's `omarchy-install` script in reliability and automation depth.

---

## Architecture Overview

```
 ┌─────────────────────────────────────────────────────┐
 │              Installer Frontend                      │
 │  GUI (Wayland)  │  TUI (crossterm)  │  Headless CLI │
 └──────────────────────┬──────────────────────────────┘
                        │
 ┌──────────────────────▼──────────────────────────────┐
 │           SigmaInstaller Core (src/installer/)       │
 │  StepEngine │ DiskPartitioner │ FsFormatter          │
 │  Bootloader │ UserSetup │ NetworkSetup │ Recovery     │
 └──────────────────────┬──────────────────────────────┘
                        │
 ┌──────────────────────▼──────────────────────────────┐
 │                 Hardware Layer                        │
 │  GPT/MBR │ LUKS2 │ LVM │ RAID │ NVMe │ eMMC         │
 └─────────────────────────────────────────────────────┘
```

---

## Installation Steps

| Step | Description |
|------|-------------|
| 1. Hardware Detection | CPU, RAM, disks, GPU, NIC identification |
| 2. Locale & Timezone | Region, keyboard layout, NTP setup |
| 3. Disk Setup | Partitioning: GPT/MBR, LUKS2 encryption, LVM, RAID |
| 4. Filesystem | Format: SigmaFS / Btrfs / ext4 + subvolume layout |
| 5. Bootloader | GRUB2 / systemd-boot; dual-boot detection |
| 6. Base Install | Package extraction, kernel install, initramfs |
| 7. User Setup | Root password, initial user, sudo config |
| 8. Network | Ethernet/Wi-Fi, hostname, DNS |
| 9. Drivers | GPU, Wi-Fi firmware, printer auto-detection |
| 10. Post-Install | Timeshift snapshot, update mirrors, first-boot services |

---

## Disk Partitioning

### Guided Modes
- **Erase disk**: single-user, automatic partitioning
- **Alongside existing OS**: shrink + install dual-boot
- **Manual**: full control with live partition editor

### Partition Schemes (UEFI)

```
/dev/sda1  512MB   EFI System Partition (vfat)
/dev/sda2  1GB     /boot (ext4)
/dev/sda3  [rest]  LUKS2 container
  └── LVM VG sigma
      ├── lv-root  50GB  SigmaFS/Btrfs (/)
      ├── lv-home  [rem] SigmaFS/Btrfs (/home)
      └── lv-swap  [2xRAM] swap
```

### Btrfs Subvolume Layout

```
@           → /
@home       → /home
@snapshots  → /.snapshots
@var-log    → /var/log
@var-cache  → /var/cache
```

---

## Encryption

- **LUKS2** with Argon2id KDF (memory-hard, GPU-resistant)
- Keyfile + passphrase dual-factor support
- TPM2 seal/unseal for passwordless boot on trusted hardware
- Recovery key printed as QR code during install

---

## Full Disk Encryption + TPM2 Flow

```
Boot → UEFI → GRUB (unlocks EFI-signed kernel)
           → systemd-boot → kernel
                         → initramfs
                               └── TPM2 PCR check
                                     ├── Match → auto-unlock LUKS2
                                     └── Fail  → prompt passphrase
```

---

## Recovery & Rollback

- Automatic **Timeshift** snapshot taken immediately post-install
- Boot menu entry: `SigmaOS (Recovery)` → boots snapshot
- Live USB mode with full installer re-run capability
- `sigma-rescue` CLI for chroot-based system repair

---

## Comparison vs Ubiquity / Calamares / omarchy-install

| Feature | Ubiquity | Calamares | omarchy-install | **SigmaInstaller** |
|---------|----------|-----------|-----------------|---------------------|
| Language | Python | C++ | Bash | **Rust** |
| TUI mode | ❌ | ❌ | ✅ (basic) | ✅ Full |
| LUKS2 | ✅ | ✅ | ❌ | ✅ + TPM2 |
| Btrfs subvols | ❌ | ✅ | ❌ | ✅ |
| Auto snapshot | ❌ | ❌ | ❌ | ✅ Timeshift |
| AI driver detect | ❌ | ❌ | ❌ | ✅ |
| Headless/CI | ❌ | ❌ | ✅ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/installer/` | Installer core and steps |
| `src/iso/` | ISO builder and live environment |
| `src/provisioning/` | Post-install provisioning |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/installer/`, `src/iso/`, `src/provisioning/`
> - Update partition scheme when new filesystem support is added
> - Keep TPM2/LUKS2 flow diagram accurate
> - Update comparison table when Calamares or Ubiquity gain new features
