# SigmaOS — Development Roadmap 2026–2028

## Current Phase Status

| Phase | Name | Status | ETA |
|-------|------|--------|-----|
| A | Foundation | ✅ Complete | 2025-Q1 |
| B | Core Kernel | ✅ Complete | 2025-Q2 |
| C | Security Subsystem | ✅ Complete | 2025-Q3 |
| D | Package Management | ✅ Complete | 2025-Q4 |
| E | Desktop Environment | ✅ Complete | 2026-Q1 |
| F | Competitor Parity | ✅ Complete | 2026-Q2 |
| **G** | **Bootable ISO** | 🔄 **Active** | 2026-Q4 |
| H | India Stack Integration | ⬜ Planned | 2027-Q1 |
| I | AI-Native Services | ⬜ Planned | 2027-Q2 |
| J | Enterprise Features | ⬜ Planned | 2027-Q4 |
| K | Hardware Partnerships | ⬜ Planned | 2028-Q2 |

***

## Phase G — Bootable ISO (Active)

### Goals

*   \[ ] Produce a bootable x86\_64 ISO image
*   \[ ] UEFI + Legacy BIOS support
*   \[ ] Live boot with optional persistence
*   \[ ] Graphical installer (Calamares-equivalent)
*   \[ ] Hardware detection and driver auto-install

### Sub-tasks

*   \[ ] GRUB integration with SigmaBoot
*   \[ ] initramfs with hardware detection
*   \[ ] ISO 9660 + El Torito standard
*   \[ ] USB write tool (`sigma-usb-writer`)
*   \[ ] Minimal live environment (shell + network)

***

## Phase H — India Stack Integration

### Goals

*   Aadhaar identity verification APIs
*   UPI payment system integration
*   DigiLocker document storage
*   BharatNet connectivity optimizations
*   Regional language support (22 scheduled languages)
*   FOSS India ecosystem compatibility

***

## Phase I — AI-Native OS Services

### Goals

*   On-device LLM inference (llama.cpp equivalent)
*   AI-powered package recommendations
*   Predictive system maintenance
*   Natural language shell (`sigma-nlsh`)
*   AI-driven security threat detection
*   Automated bug reporting and fix suggestions

***

## Phase J — Enterprise Features

### Goals

*   Active Directory / LDAP integration
*   Centralized fleet management
*   Compliance frameworks (SOC 2, ISO 27001)
*   Enterprise MDM support
*   Remote attestation for enterprise deployment
*   Extended security maintenance (ESM) channel

***

## Phase K — Hardware Partnerships

### Goals

*   SigmaOS preinstalled on partner hardware
*   Custom silicon optimizations (ARM, RISC-V)
*   Embedded/IoT variant (`sigma-embedded`)
*   Mobile variant feasibility study
*   Hardware certification program

***

## Technical Debt & Ongoing Work

### High Priority

*   \[ ] Reduce `cargo check` warnings to zero
*   \[ ] Achieve 100% test coverage on `src/klib/`
*   \[ ] Complete ELF dynamic linking support
*   \[ ] Implement `mmap()` with full POSIX semantics
*   \[ ] Finish PCI/PCIe bus enumeration

### Medium Priority

*   \[ ] GPU compute (GPGPU/CUDA-equivalent) support
*   \[ ] Audio latency optimization (< 5ms)
*   \[ ] Bluetooth audio (A2DP, HFP profiles)
*   \[ ] Printer support (CUPS-equivalent)
*   \[ ] Scanner support (SANE-equivalent)

### Low Priority

*   \[ ] Gaming compatibility layer (Wine-equivalent)
*   \[ ] Android app support (Waydroid-equivalent)
*   \[ ] Windows app compatibility (Proton-equivalent)
*   \[ ] Flatpak sandbox integration
*   \[ ] Snap confinement support

***

## Feature Requests from Linux Distros

### From Arch Linux

*   \[x] AUR-style package recipes
*   \[x] Rolling release model
*   \[ ] Offline package cache (`pacman -Sc` equivalent)
*   \[ ] Package group installation

### From NixOS

*   \[x] Atomic rollbacks
*   \[x] Declarative configuration
*   \[ ] Nix flakes equivalent (hermetic builds)
*   \[ ] Multiple system configurations per machine

### From Fedora

*   \[x] SELinux integration
*   \[x] GPG package signatures
*   \[ ] Silverblue immutable mode
*   \[ ] rpm-ostree layering equivalent

### From Ubuntu

*   \[x] LTS + rolling channel model
*   \[x] Snap-format package support (partial)
*   \[ ] Ubuntu Pro extended security features
*   \[ ] Landscape fleet management

### From OpenBSD

*   \[x] pledge() and unveil()
*   \[x] W^X memory enforcement
*   \[ ] httpd (built-in web server)
*   \[ ] pf (built-in packet filter)
*   \[ ] relayd (load balancer/proxy)

***

## Distro-Specific Ideas Still to Implement

| Distro | Feature | Status | Priority |
|--------|---------|--------|----------|
| Tails | Amnesic (RAM-only) mode | ⬜ Planned | High |
| Whonix | Tor gateway isolation | ⬜ Planned | Medium |
| Clear Linux | Intel hardware optimization | ⬜ Planned | Medium |
| Bedrock Linux | Multi-distro userland | ⬜ Planned | Low |
| Slackware | BSD-style init scripts | ⬜ Planned | Low |
| Solus | eopkg package manager parity | ⬜ Planned | Medium |
| Elementary OS | AppCenter with curated apps | ⬜ Planned | Medium |
| Pop!\_OS | Auto-tiling window manager | ⬜ Planned | Low |
| Garuda Linux | Btrfs snapshot boot menu | ⬜ Planned | Medium |
| EndeavourOS | Guided Arch-like installer | ⬜ Planned | Low |

***

*Last updated: 2026-08-23 | SigmaOS Planning Team*
