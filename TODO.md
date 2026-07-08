# SigmaOS TODO List

This document tracks pending tasks and unimplemented features for SigmaOS development.

## High Priority Tasks

### Driver Expansion
- [x] GPU Drivers (NVIDIA, AMD, Intel) - OOP-based driver framework with Device trait
- [x] Wi-Fi Chipset Drivers (iwlwifi) - Intel Wi-Fi driver with WifiDevice trait
- [x] Bluetooth Drivers (BlueZ compatibility layer) - BlueZ-compatible adapter
- [x] USB Controller Drivers (XHCI, EHCI, UHCI, OHCI) - Full USB controller family with UsbController trait
- [x] Printer Drivers (CUPS compatibility) - USB printer driver with PrinterDevice trait
- [x] Touchpad Drivers (Synaptics, ELAN) - Touchpad drivers with TouchpadDevice trait
- [x] Additional Wi-Fi Drivers (MT7921, RTW88) - MediaTek and Realtek Wi-Fi drivers
- [x] Additional GPU Drivers (VIA, SiS, Matrox) - Legacy GPU vendor drivers
- [x] ARM Board Support (Raspberry Pi, embedded devices) - BCM2835, BCM2711 SoC drivers

### Package Management
- [x] sigma_pkg dependency resolver implementation - OOP-based with DependencyResolver trait
- [ ] Package rollback functionality
- [ ] Sandboxed package installs
- [ ] Delta updates support
- [x] Package signing verification - ED25519 signature verification implemented

### Networking
- [ ] VPN client/server support (WireGuard, OpenVPN)
- [x] Firewall with user-defined rules (iptables/nftables compatibility) - Firewall trait with rule evaluation
- [ ] Advanced routing (BGP, OSPF)
- [ ] QoS controls
- [ ] Network monitoring dashboard
- [x] Ethernet NIC drivers (e1000e) - Intel e1000e driver with EthernetDevice trait

### Security
- [x] SELinux/AppArmor-style MAC implementation - MandatoryAccessControl trait with sandboxing
- [ ] Secure Boot integration with TPM
- [x] Sandbox improvements (capability-based) - Process sandboxing with profiles
- [ ] PGP key generation for security@sigmaos.dev

## Medium Priority Tasks

### Desktop & UX
- [ ] Zenith Desktop accessibility features (screen readers, high-contrast)
- [ ] Touch/gesture support for tablets
- [ ] Theme engine for customization
- [ ] Unified Control Center
- [ ] Onboarding wizard for new users

### Documentation
- [ ] Kernel architecture documentation
- [x] Driver development guide - OOP-based driver framework documented
- [x] Package manager usage guide - Dependency resolver documented
- [x] POSIX layer overview - Syscall handler with POSIX compatibility documented
- [ ] Arch Wiki-style knowledge base
- [x] Audio driver documentation - HDA/ALSA driver documented
- [x] Storage driver documentation - NVMe/AHCI driver documented
- [x] Network driver documentation - e1000e driver documented
- [x] Input driver documentation - PS/2 keyboard/mouse driver documented
- [x] Camera driver documentation - UVC/V4L2 driver documented
- [x] Printer driver documentation - USB/CUPS driver documented

### Developer Tools
- [ ] IDE integration (VS Code, JetBrains, Eclipse plugins)
- [ ] Custom build system with modular configs
- [ ] Debugging tools (kernel logs, crash analyzers, profilers)
- [ ] Performance benchmarking tools

## Low Priority Tasks

### Community & Governance
- [ ] SigmaOS Foundation establishment
- [ ] Contributor programs and hackathons
- [ ] Documentation sprints
- [ ] Bounty programs for security bugs

### Cross-Platform
- [ ] SigmaOS Mobile variant
- [ ] IoT/embedded device support
- [ ] Cloud orchestration layer
- [ ] Gaming layer (Vulkan/DirectX compatibility)

## Known Issues

### Build System
- [ ] Bootloader integration in progress (see INSTALL.md)
- [ ] Justfile commands need testing on all platforms

### Documentation Gaps
- [ ] SECURITY.md has placeholder contact info
- [ ] Missing docs/security/pgp-key.asc
- [ ] Missing scripts/sign_release.sh
- [ ] MAINTAINERS file has formatting errors

### Testing
- [ ] Comprehensive kernel test suite
- [x] Driver testing framework - OOP-based driver testing with traits
- [x] Integration tests for package manager - Dependency resolver with conflict detection
- [ ] Security audit of kernel code

## Completed Features (Reference)

See [Implementation-Progress.md](wiki_repo/Implementation-Progress.md) for completed features including:
- Self-Healing Kernel
- Immutable Audit Trail
- SigmaOS SDK
- Full IPv6 Stack
- AI-Native OS Orchestrator
- POSIX Compatibility Layer
- Adaptive Resource Scheduler
- Zero-Copy File System
- Lightweight Containers
- Mandatory Access Control
- Secure Boot & TPM Integration
- Encrypted File System

## Contribution Guidelines

When implementing a feature from this list:
1. Check if there's an existing RFC or discussion
2. Create a feature branch following CONTRIBUTING.md guidelines
3. Add tests for new functionality
4. Update relevant documentation
5. Submit PR with Signed-off-by line

For questions or to discuss implementation details, see [CONTRIBUTING.md](CONTRIBUTING.md).
