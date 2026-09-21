# Strategic Development Plan for the SigmaOS Automated Single-Boot Installer

## Executive Summary
This document establishes the strategic 5-phase development roadmap for the SigmaOS Automated Single-Boot Installer (`src/installer/lightning_installer.rs`, `src/installer/system_installer.rs`, `src/installer/gui_wizard.rs`). Inspired by premier Linux and BSD deployment frameworks (`archinstall`, Fedora Kickstart/Anaconda, Debian Preseed, FreeBSD `bsdinstall`, OpenBSD `autoinstall`, and Omarchy Linux single-disk takeover), SigmaOS provides an intelligent, non-interactive, single-boot installation pipeline that formats target NVMe/SATA storage drives, deploys encrypted atomic root images, configures TPM 2.0 PCR-bound `sigma-boot` bootloaders, and provisions user personas in under 60 seconds.

---

## 1. Automated Installer Benchmark & Inspiration Matrix

| Installer Framework | Distribution / OS Origin | Key Features Absorbed by SigmaOS | SigmaOS Integration Layer |
| :--- | :--- | :--- | :--- |
| **archinstall** | Arch Linux | Scriptable CLI answer files, automated partition layout generation | `src/installer/lightning_installer.rs` |
| **Kickstart / Anaconda** | Fedora / RHEL | Unattended answer file parsing (`sigmainstall.yaml`), network Kickstart | `src/installer/system_installer.rs` (`UnattendedKickstartParser`) |
| **Debian Preseed** | Debian / Ubuntu | Preseeded answer database (`debconf` parity) for headless deployments | `src/installer/system_installer.rs` |
| **bsdinstall & autoinstall** | FreeBSD / OpenBSD | `install.conf` automated disk wiping, ZFS pool auto-creation | `src/installer/lightning_installer.rs` |
| **Omarchy Single-Disk Takeover** | Omarchy Linux | Zero-prompt single-drive wipe & takeover mode (`--auto-takeover`) | `src/installer/gui_wizard.rs` (`SigmaSetupSystemEngine`) |

---

## 2. Strategic 5-Phase Automated Single-Boot Installer Roadmap

```
┌───────────────────────────────────────────────────────────────────────────┐
│              SIGMAOS AUTOMATED SINGLE-BOOT INSTALLER ROADMAP              │
└───────────────────────────────────────────────────────────────────────────┘
   Phase 1: Hardware Detection & Disk Selection
   ├── Fast NVMe 1.4 / SATA drive discovery & performance benchmarking
   ├── Automatic primary installation disk selection (`/dev/nvme0n1` preferred)
   └── UEFI 2.8+ ESP partition & Secure Boot keys validation

   Phase 2: Non-Interactive Unattended Answer File Automation
   ├── YAML/TOML answer file engine (`sigmainstall.yaml` Kickstart parity)
   ├── Preseeded user accounts, SSH keys, timezone, and locale settings
   └── Headless network provisioning via DHCP or static IP config

   Phase 3: Automated GPT & Btrfs/ZFS Layout Creation
   ├── Single-boot UEFI ESP partition creation (`/boot/efi`, FAT32)
   ├── Sub-second LUKS2 / PQC Dilithium-5 full disk encryption setup
   └── Automated Btrfs/ZFS subvolume dataset creation (`@root`, `@home`, `@snapshots`)

   Phase 4: Atomic Image Extraction & Bootloader Installation
   ├── CAS (Content-Addressed Store) squashed image extraction to root subvolume
   ├── `sigma-boot` / systemd-boot installation with TPM 2.0 PCR binding
   └── Sub-second initramfs generation (`SovereignFastInitramfsGenerator`)

   Phase 5: First-Boot System Personalization & Persona Provisioning
   ├── Automatic hardware driver profile installation (NVIDIA/AMD/Intel)
   ├── System persona setup (Developer, Compliance, Student, Gaming)
   └── Post-install reboot trigger & boot media ejection
```

---

## 3. Detailed Phase Architecture

### Phase 1: Hardware Detection & Automatic Disk Selection
- **Storage Discovery**: Probes block storage devices via `src/kernel/block_dev.rs` and ranks target disks by bus speed (PCIe Gen4/Gen5 NVMe > SATA SSD > HDD).
- **Single-Boot Takeover**: In `--auto-takeover` mode, automatically selects the fastest non-removable disk, prompting for confirmation only if multiple internal drives are detected.
- **Firmware Validation**: Confirms 64-bit UEFI environment, checks Secure Boot status, and verifies TPM 2.0 module availability.

### Phase 2: Non-Interactive Unattended Answer File Automation
- **Answer File Engine**: Parses `sigmainstall.yaml` or network Kickstart URLs (`sigmainstall --config=http://install.local/preseed.yaml`).
- **Preseeded Configuration**:
  ```yaml
  # Sample sigmainstall.yaml
  target_disk: auto # Automatically selects fastest NVMe drive
  wipe_disk: true   # Single-boot full drive takeover
  filesystem: btrfs # Options: btrfs, zfs, ext4
  encryption: luks2 # LUKS2 with TPM 2.0 binding
  hostname: sigma-station
  user:
    username: sovereign
    sudo: true
  persona: developer # Pre-installs Rust, GCC, Docker, Zenith Desktop
  ```

### Phase 3: Automated GPT & Btrfs/ZFS Partitioning Layout
- **Partition Layout**:
  - Partition 1: UEFI ESP (`/boot/efi`, 512 MB, FAT32)
  - Partition 2: Sovereign Boot (`/boot`, 1 GB, Ext4/XFS)
  - Partition 3: Full Disk Encrypted Root Container (Remaining capacity, LUKS2/PQC)
- **Subvolume Creation**: Configures Btrfs or ZFS CoW subvolumes for atomic snapshots (`@root`, `@home`, `@var_log`, `@snapshots`).

### Phase 4: Atomic Image Extraction & Bootloader Installation
- **Atomic Image Transfer**: Unpacks the pre-compiled, content-addressed squashed rootfs image into the target root subvolume in under 30 seconds.
- **Bootloader Configuration**: Installs `sigma-boot` into the EFI System Partition, writing Secure Boot keys and binding LUKS2 encryption keys to TPM 2.0 PCR 0, 2, and 7 registers.
- **Initramfs Generation**: Builds a minimal `#![no_std]` zero-dependency initramfs image containing essential storage and filesystem drivers.

### Phase 5: Post-Install Personalization & Persona Provisioning
- **Driver Auto-Configuration**: Detects GPU (NVIDIA, AMD, Intel) and Wi-Fi chipsets, auto-loading required firmware blobs (`src/drivers/`).
- **Persona Provisioning**: Applies user persona presets (Developer, Compliance, Student, Gaming) by configuring desktop themes, shell aliases, and system packages.
- **Ejection & Reboot**: Ejects installation media and reboots directly into the newly installed, single-boot SigmaOS Desktop Edition.

---

## 4. Verification & Testing Strategy
All automated installer components are verified using headless QEMU virtual machine tests:
```bash
# Run installer unit tests
cargo test --package sigmaos --lib installer

# Run QEMU automated single-boot installation smoke test
bash scripts/qemu_smoke_test.sh

# Run full native test runner
bash run_sigma_tests.sh
```
