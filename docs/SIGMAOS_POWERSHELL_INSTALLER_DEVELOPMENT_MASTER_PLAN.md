# SigmaOS PowerShell Automated Installer & Deployment Master Plan

## Executive Summary

The **SigmaOS PowerShell Installation & Deployment Subsystem** (`Install-SigmaOS.ps1`, `src/installer/gui_wizard.rs`, `src/compatibility/sigmawin.rs`) is engineered as a cross-platform automated deployment framework bridging Linux unattended installation models (Windows Subsystem for Linux `wsl --install`, Arch Linux `archinstall` non-interactive scripts, Fedora `Anaconda Kickstart`, Alpine `setup-alpine` unattended profiles, WSL2 Hyper-V `vhdx` disk mounting) with BSD unattended deployment frameworks (FreeBSD `bsdinstall` bsdconfig manifests, NetBSD `sysinst`).

This document defines the master development plan for the SigmaOS PowerShell installer across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │     SigmaOS Automated PowerShell Deployment Subsystem    │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│     Linux Unattended      │            │   WSL2 & Hyper-V Hybrid   │            │   BSD Automated Deploy    │
│ • Arch archinstall Script │            │ • `wsl.exe --import` Reg  │            │ • FreeBSD bsdinstall Config│
│ • Fedora Kickstart (TOML) │            │ • VHDX Fixed Volume Provision│         │ • NetBSD sysinst Unattended│
│ • Alpine setup-alpine Profile│         │ • Hyper-V VM Creation     │            │ • PQC Verification Tokens │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core PowerShell Installer Development Pillars

### Pillar 1: One-Liner Web Bootstrapper Script (`Install-SigmaOS.ps1`)
* **One-Liner Web Execution Syntax**:
  - Enable seamless web deployment syntax:
    ```powershell
    irm https://get.sigmaos.org/install.ps1 | iex
    ```
  - Parse execution flags (`-Flavor Zenith`, `-Target WSL2`, `-Target HyperV`, `-Target DualBoot`, `-Persona Developer`).

### Pillar 2: WSL2 Instance Auto-Registration (`wsl.exe --import`)
* **WSL2 Distro Import Engine**:
  - Download verified `sigmaos-rootfs.tar.xz` container images.
  - Automatically register SigmaOS as a native WSL2 distribution using `wsl.exe --import SigmaOS C:\SigmaOS sigmaos-rootfs.tar.xz --version 2`.
  - Configure `/etc/wsl.conf` for systemd init activation (`[boot] systemd=true`).

### Pillar 3: Hyper-V & QEMU VHDX Virtual Disk Auto-Provisioning
* **Fixed & Dynamic VHDX Image Generator**:
  - Automatically create and format VHDX virtual hard disk files (`New-VHD -Path "C:\SigmaOS\disk.vhdx" -SizeBytes 64GB -Fixed`).
  - Provision Hyper-V virtual machines with Generation 2 UEFI, Secure Boot keys, and VirtIO synthetic drivers.

### Pillar 4: Physical USB Flash Drive ISO Flasher Engine (`Write-SigmaVolume`)
* **PowerShell USB Flasher (Rufus / `dd` Parity)**:
  - Enumerate physical USB storage drives (`Get-Disk | Where-Type -eq "Removable"`).
  - Perform raw block-level ISO image flashing directly to USB flash drives (`Clear-Disk`, `Write-VolumeBuffer`), creating bootable x86_64 UEFI USB installer media.

### Pillar 5: Pre-Seeded Unattended Installation Manifest Parser
* **Cloud-Init & Kickstart Profile Integration**:
  - Parse JSON / TOML pre-seeded installation profiles (`sigma-kickstart.json` / `profile.toml`).
  - Pre-configure user accounts, SSH authorized keys, hostnames, timezone, default desktop persona (Developer, Gaming, Compliance, Minimal), and encrypted volume passphrases.

### Pillar 6: Cryptographic Verification & Dilithium-5 ISO Integrity Attestation
* **SHA256 & PQC Signature Attestation**:
  - Verify downloaded ISO/rootfs SHA256 checksums and Dilithium-5 post-quantum signature manifests prior to disk allocation, preventing tampered image execution.

---

## 3. Four-Phase Chronological Roadmap

```
  Phase 1: Web Bootstrapper & Signature Verifier (Months 1–3)
  ├── PowerShell `Install-SigmaOS.ps1` Web Bootstrapper Script
  ├── SHA256 & Dilithium-5 PQC Cryptographic ISO Signature Verifier
  └── Command-Line Parameter Parser (`-Target`, `-Flavor`, `-Persona`)

  Phase 2: WSL2 & Hyper-V VHDX Automated Provisioner (Months 3–6)
  ├── WSL2 `wsl.exe --import` Container RootFS Registration Engine
  ├── Hyper-V Generation 2 UEFI VHDX Virtual Hard Disk Creator
  └── Automated Systemd Init & Network Bridge Configuration

  Phase 3: Dual-Boot Partitioning & Physical USB Flasher (Months 6–9)
  ├── Removable USB Flash Drive Raw Block Image Flasher Engine
  ├── Windows BCD / EFI System Partition Dual-Boot Bootloader Entry Creator
  └── Partition Resizing & NTFS Shrink Safety Guard Engine

  Phase 4: Pre-Seeded Cloud-Init Manifests & First-Boot Onboarding (Months 9–12)
  ├── Unattended Kickstart / Cloud-Init Profile TOML Parser
  ├── Zenith Desktop Automated First-Boot Persona Onboarding
  └── 60-Second Full PowerShell Installation Benchmark & Test Suite
```

---

## 4. Verification and Benchmark Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **PowerShell One-Liner Install** | Web Execution Test | < 60 seconds total deployment time for WSL2 target |
| **Image Integrity Verification** | SHA256 / Dilithium-5 Check | 100% cryptographic match; fail-closed on tampered bytes |
| **USB Flash Drive Flasher** | Physical Flasher Benchmark | > 80 MB/s sustained USB block write throughput |
| **Dual-Boot Bootloader Entry** | Windows BCD EFI Check | Non-destructive EFI entry addition with instant rollback guard |
