# SigmaOS TODO List

This document tracks pending tasks and unimplemented features for SigmaOS development.

## High Priority Tasks

### Driver Expansion
- [x] GPU Drivers (NVIDIA, AMD, Intel) - OOP-based driver framework with Device trait
- [x] Wi-Fi Chipset Drivers (iwlwifi) - Intel Wi-Fi driver with WifiDevice trait
- [x] Bluetooth Drivers (BlueZ compatibility layer) - BlueZ-compatible adapter
- [ ] USB Controller Drivers (XHCI, EHCI, UHCI, OHCI) - Existing stub implementation
- [ ] Printer Drivers (CUPS compatibility)
- [ ] ARM Board Support (Raspberry Pi, embedded devices)

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
- [ ] Driver testing framework
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
