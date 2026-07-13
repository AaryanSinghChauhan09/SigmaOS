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
- [x] Package rollback functionality - Snapshot-based rollback system with circular buffer
- [x] Sandboxed package installs - Network/filesystem isolation with resource limits
- [x] Delta updates support - Binary patch application with version verification
- [x] Package signing verification - ED25519 signature verification implemented

### Networking
- [x] VPN client/server support (WireGuard, OpenVPN) - WireGuard tunnel with key generation, OpenVPN with cert management, kill switch with firewall integration
- [x] Firewall with user-defined rules (iptables/nftables compatibility) - Firewall trait with rule evaluation
- [x] Advanced routing (BGP, OSPF) - BGP peer management, route selection, OSPF areas, SPF algorithm
- [x] QoS controls - Token bucket rate limiting, traffic classification, queue management (FIFO, Priority, WFQ, CBQ, HTB)
- [x] Network monitoring dashboard - Interface stats, connection monitoring, traffic analysis, alert thresholds
- [x] Ethernet NIC drivers (e1000e) - Intel e1000e driver with EthernetDevice trait

### Security
- [x] SELinux/AppArmor-style MAC implementation - MandatoryAccessControl trait with sandboxing
- [x] Secure Boot integration with TPM - TPM device with PCR extend/read, seal/unseal, quote, and Secure Boot integration
- [x] Sandbox improvements (capability-based) - Process sandboxing with profiles
- [x] PGP key generation for security@sigmaos.dev - Ed25519-like key generation, signing, verification, and export

## Medium Priority Tasks

### Desktop & UX
- [x] Zenith Desktop accessibility features (screen readers, high-contrast) - Screen reader TTS engine, WCAG AAA contrast validation, magnifier with 2-16x zoom, keyboard navigation with focus rings
- [x] Touch/gesture support for tablets - Multi-touch tracking, gesture recognition (tap, double-tap, long-press, swipe, pinch, rotate, pan, scroll), configurable thresholds
- [x] Theme engine for customization - Light/Dark/Auto/Custom modes, color palettes, typography settings, spacing, border radius, shadows, animations, custom colors, theme save/load/export/import
- [x] Unified Control Center - Centralized settings panel with Network, Display, Sound, Bluetooth, WiFi, Power, Storage, Accessibility, Security, Accounts, Updates, About panels, quick settings, search
- [x] Onboarding wizard for new users - Multi-step wizard (Welcome, Language, Region, Keyboard, Network, Privacy, Account, Theme, Accessibility, Complete), user/system configuration, progress tracking

### Documentation
- [x] Kernel architecture documentation - Comprehensive kernel architecture document with all subsystems
- [x] Driver development guide - OOP-based driver framework documented
- [x] Package manager usage guide - Dependency resolver documented
- [x] POSIX layer overview - Syscall handler with POSIX compatibility documented
- [x] Arch Wiki-style knowledge base - Full reference covering installation, kernel internals, drivers, security, and package management
- [x] Audio driver documentation - HDA/ALSA driver documented
- [x] Storage driver documentation - NVMe/AHCI driver documented
- [x] Network driver documentation - e1000e driver documented
- [x] Input driver documentation - PS/2 keyboard/mouse driver documented
- [x] Camera driver documentation - UVC/V4L2 driver documented
- [x] Printer driver documentation - USB/CUPS driver documented

### Developer Tools
- [x] IDE integration (VS Code, JetBrains, Eclipse plugins) - VS Code extension with syntax highlighting, IntelliSense, build integration, debugging; JetBrains plugin with CMake/Rust support; Eclipse CDT integration
- [x] Custom build system with modular configs - CMake-based build system with profiles (desktop, microkernel, cloud, mobile, rtos, distributed), modular component configuration, cross-compilation support
- [x] Debugging tools (kernel logs, crash analyzers, profilers) - Kernel logs with filtering, crash analyzers with dump analysis, CPU/memory/I/O/lock profilers
- [x] Performance benchmarking tools - CPU, memory, I/O, and system benchmarks

## Low Priority Tasks

### Community & Governance
- [x] SigmaOS Foundation establishment - Full governance charter with board structure, TSC, working groups, grant programs, and financial model
- [x] Contributor programs and hackathons - Defined in Foundation Charter: Developer Grant Program, Mentorship Program, Annual Hackathon
- [ ] Documentation sprints
- [x] Bounty programs for security bugs - Full Bug Bounty Program with severity classification ($50–$10,000), rules of engagement, legal safe harbor

### Cross-Platform
- [x] SigmaOS Mobile variant - Full ARM64/RISC-V architecture spec: telephony, touch, camera HAL, sensor hub, power profiles (SigmaOS-Mobile-Spec.md)
- [x] IoT/embedded device support - Sigma Atom spec: MCU/SBC/industrial targets, RT scheduler, peripheral HAL, OTA, industrial protocols (IoT-Embedded-Spec.md)
- [x] Cloud orchestration layer - Sigma Nebula spec: sovereign container runtime, multi-node scheduler, eBPF networking, GitOps, service mesh (Cloud-Orchestration-Spec.md)
- [x] Gaming layer (Vulkan/DirectX compatibility) - Sigma Forge spec: Vulkan ICD, D3D11/12→Vulkan (sigma-dx), frame pacing, gamepad support (Gaming-Layer-Spec.md)

## Known Issues

### Build System
- [ ] Bootloader integration in progress (see INSTALL.md)
- [ ] Justfile commands need testing on all platforms

### New Code Implementations (Session 2)
- [x] sigma-shield packet filter - kernel/net/firewall/sigma_shield.rs: stateful firewall with conn tracking, rate limiting, rule engine
- [x] Thermal & Power HAL daemon - kernel/hal/thermal/mod.rs: thermal zones, DVFS, fan control, battery management, power profiles
- [x] cgroup-aware namespace accounting - kernel/security/cgroups/mod.rs: hierarchical cgroups with CPU/memory/IO/network/PID accounting
- [x] CODEOWNERS per subsystem - .github/CODEOWNERS: all major subsystems mapped with extended entries for new modules
- [x] Arch-Wiki knowledge base - SigmaOS.wiki/Knowledge-Base.md: comprehensive OS reference

### Documentation Gaps
- [x] SECURITY.md has placeholder contact info - Updated PGP key fingerprint reference
- [x] Missing docs/security/pgp-key.asc - File exists with placeholder PGP key
- [x] Missing scripts/sign_release.sh - Implemented signature flow with GPG and Cosign PQC fallback
- [x] MAINTAINERS file has formatting errors - File verified, no formatting errors found

### Testing
- [x] Comprehensive kernel test suite - Test runner with suite management, scheduler tests (thread creation, priority scheduling, context switch, state transitions, CPU time accounting, overflow protection), syscall dispatch tests (read, write, open, close, mmap, unknown syscall, register preservation, error handling), VFS tests (mount, open, write, read, write/read match, close, FD exhaustion, invalid FD, offset tracking, inode allocation)
- [x] Driver testing framework - OOP-based driver testing with traits
- [x] Integration tests for package manager - Dependency resolver with conflict detection
- [x] Security audit of kernel code - Static analysis rules (buffer overflow, use-after-free, double-free, integer overflow, null pointer dereference, race condition, memory leak, information leak, privilege escalation, unsafe functions, hardcoded credentials, cryptographic weakness), audit report generation, severity classification (Info, Low, Medium, High, Critical), CWE mapping, CVSS scoring

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
