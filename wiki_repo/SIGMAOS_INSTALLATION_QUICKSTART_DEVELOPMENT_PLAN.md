# 💿 SigmaOS ISO Installation & Getting Started Subsystem (`installation_quickstart`) Strategic Development Plan

## Executive Summary & Design Vision

Installing an operating system must be fast (under 5 minutes), secure by default (full-disk LUKS2 encryption), flexible across partition layouts (full-disk wipe vs. unallocated free-space dual boot), and adaptable to specialized provisioning workflows (deferred OEM owner setup, unattended fleet deployments, and no-encryption overrides).

Drawing direct architectural inspiration from **Omarchy Installation Workflow**, Calamares GUI installer, Archinstall (`archinstall`), Fedora Anaconda kickstart, and Debian preseed installers, the **SigmaOS ISO Installation Subsystem** (`InstallerScreen`, `gui_wizard.rs`, `SovereignEditionBuilder`, `ArchinstallEngine`) provides a zero-dependency, Calamares-inspired installer wizard written natively in Safe Rust.

---

## 1. Installation Workflow & Architecture

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                    Bootable Live ISO Image (`sigmaos.iso`)                │
│       - USB Flashing: balenaEtcher, caligula, or `dd`                     │
│       - BIOS/UEFI Checklist: Disable Secure Boot & Windows BitLocker      │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              `InstallerScreen` Interactive Configuration Wizard           │
│     - Welcome -> Language -> Location -> Keyboard Selection               │
│     - OEM Deferral Override: Press `Ctrl + C` on 1st screen to defer      │
│       keyboard/user setup to first boot for new owners                    │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│            Partitioning Mode & Encryption Configuration                   │
│     - Mode A: Full-Disk Wipe (Takes over drive, LUKS2 encrypted)          │
│     - Mode B: Free-Space Install (Unallocated space, dual boot Windows)    │
│     - Encryption Override: Press `Ctrl + C` on format prompt for unencrypted │
│     - Pre-Boot Keyboard Warning: Wired / 2.4GHz dongle required for LUKS   │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Fast Sub-5-Minute System Deployment Engine                  │
│     - Btrfs / ZFS Subvolume Creation (`@`, `@home`, `@snapshots`)         │
│     - Unattended Fleet Mode: Auto-reads config manifest from USB drive     │
│     - Complete Onboarding & Reboot Prompt                                 │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Key Installation Features & Protocols

### 2.1 Full-Disk LUKS2 Encryption & Dual-Boot Modes
- **Full-Disk Wipe**: Default selection wiping target drive, setting up Btrfs subvolumes (`@`, `@home`, `@var`) wrapped inside a LUKS2 encrypted container.
- **Free-Space Dual Boot**: Automatically detects unallocated space on drives alongside existing Windows or BSD installations. Guides users through disabling BitLocker in Windows before partitioning.

### 2.2 Hardware Advice & Pre-Boot Passphrase Keyboard Compatibility
- **Pre-Boot Passphrase Entry**: Pre-boot LUKS passphrase prompts execute prior to Bluetooth driver initialization. The installer prompts users to connect a **wired USB keyboard** or a **2.4GHz wireless dongle keyboard** for reliable pre-boot passphrase entry.

### 2.3 OEM Deferred Setup for New Owners (`Ctrl + C` Staging)
- Pressing `Ctrl + C` on the initial keyboard selection screen switches the installer to **OEM Staging Mode**.
- The system installs system binaries immediately, deferring personal configuration (keyboard layout, username, password, timezone) until the new owner boots the machine for the first time. The passphrase chosen on first boot secures the LUKS encrypted container.

### 2.4 Unattended Fleet Installs & No-Encryption Override
- **Unattended Installs**: When a second USB drive or VM virtio-blk volume containing `sigma-unattended.json` or Anaconda kickstart manifest is present, the installer bypasses the wizard and deploys headless.
- **No-Encryption Override**: Hitting `Ctrl + C` on the disk format confirmation screen allows advanced users (e.g., remote VMs or throwaway testbeds) to install without LUKS encryption.

---

## 3. Phased Development Roadmap

### Phase 1: Native `gui_wizard.rs` & LUKS2 Btrfs Engine (Q4 2026)
- Stabilize `InstallerScreen` sequence (`Welcome` -> `Keyboard` -> `Partitioning` -> `UserSetup` -> `InstallationProgress`) in `src/installer/gui_wizard.rs`.
- Implement automated LUKS2 disk formatting and Btrfs subvolume layout creation.
- Add free-space unallocated drive detection for dual-booting.

### Phase 2: OEM Deferred Setup & Keyboard Pre-Boot Prompts (Q1 2027)
- Build OEM staging mode (`Ctrl + C` trigger on keyboard selection), creating first-boot setup wizard (`sigmaos-first-boot`).
- Display pre-boot keyboard hardware advice dialogs (wired / 2.4GHz dongle requirements).
- Implement no-encryption override trigger (`Ctrl + C` on disk format confirmation).

### Phase 3: Unattended Fleet Deployment & ISO Builder (Q2 2027)
- Deploy `sovereign_edition_builder` ISO generator script (`scripts/build-iso.sh`).
- Integrate `sigma-unattended.json` manifest reader for automated VM and server fleet provisioning.
- Add Windows BitLocker detection and guidance prompts.

### Phase 4: Installation Speed Optimization & CI Smoke Testing (Q3 2027+)
- Optimize image extraction and kernel initramfs generation to achieve sub-2-minute installation times on NVMe storage.
- Integrate QEMU live ISO smoke testing into `./scripts/qemu_smoke_test.sh` and CI runner.
- Conduct fuzz testing against partitioning and manifest parsers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All installation components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/installer/gui_wizard.rs` (`InstallerScreen` sequence & persona selection)
- `src/compatibility/fedora_missing_components.rs` (`FedoraAnacondaKickstartEngine`)
- `src/distro/arch_complete_parity_suite.rs` (`ArchChrootSetupEngine` & mkinitcpio preset generation)
- `scripts/qemu_smoke_test.sh` (headless live ISO boot & installation smoke test)
