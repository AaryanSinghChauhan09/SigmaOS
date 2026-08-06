# SigmaOS Wiki Home

> **SigmaOS**: The world's most advanced sovereign, bare-metal operating system for the next generation of silicon sovereignty.

[![Build Status](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/sigma_master_ci.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions)
[![Security Scan](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/codeql-analysis.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning)
[![License](https://img.shields.io/badge/license-MIT%20OR%20GPL--2.0-blue)](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/LICENSE.md)

## 🚀 Quick Navigation

### Getting Started
- [Getting Started](Getting-Started) - Installation and first boot
- [Building SigmaOS](Building-SigmaOS) - Compile from source
- [Architecture Overview](Architecture-Overview) - System architecture
- [Boot Process](Boot-Process-Architecture) - How SigmaOS boots

### Core Documentation
- [Architecture](Architecture) - Full architecture documentation
- [Kernel Internals](Kernel-Internals) - Deep dive into the kernel
- [Filesystem Specification](Filesystem-Spec-and-Virtual-FS) - SigmaFS and VFS
- [Networking](Networking) - Network stack documentation
- [Drivers](Driver-Development-Guide) - Device driver framework

### Development
- [Contributing](Contributing) - How to contribute
- [Code of Conduct](Code-of-Conduct) - Community standards
- [Testing Guide](Testing-Guide) - Running tests
- [Roadmap](Future_Development_Roadmap) - What's planned

### Security
- [Security Policy](SECURITY) - Vulnerability reporting
- [Security Hardening](Security-Hardening-Guide) - Security practices  
- [Security Scanning Fixes](Security-Scanning-Fixes) - Code scanning results
- [Zero Trust Networking](zero_trust_network) - ZTN implementation

### Package Management
- [Package Management](Package-Management) - sigpkg documentation
- [AUR Helper](AUR-Helper) - Arch-compatible package helper
- [Universal Package Manager](Universal-Package-Manager) - Multi-format support

### Linux & BSD Inspirations
- [Linux BSD Inspirations](LINUX_BSD_INSPIRATIONS) - Feature absorption status
- [Arch Linux Parity](Arch_Linux_Parity_Roadmap) - Arch Linux features
- [Distro Absorption Blueprint](Distro-Absorption-Blueprint) - Multi-distro strategy
- [BSD Distros Analysis](BSD_DISTROS_ANALYSIS) - BSD OS comparisons

### Advanced Topics
- [AI Subsystem](AI-Subsystem) - SigmaAI architecture
- [Virtualization](Virtualization-and-Containers) - VMs and containers
- [Driver Management](Driver_Management_Roadmap) - Hardware support roadmap
- [Zenith Desktop](Zenith-System-Improvement-Plan) - Desktop environment

## 📊 Status Dashboard

| Component | Status | Notes |
|-----------|--------|-------|
| Microkernel | ✅ Active | EEVDF + BORE scheduler |
| Memory Management | ✅ Active | Buddy + Slab + Custom Vec/HashMap |
| Network Stack | ✅ Active | TCP/IP + IPv6 + TLS |
| Filesystem | ✅ Active | SigmaFS + VFS layer |
| Package Manager (sigpkg) | ✅ Active | AUR + universal format |
| AI Subsystem | ✅ Active | No external AI deps |
| Shell (sigma-sh) | ✅ Active | REPL with scripting |
| Desktop (Zenith) | 🔄 In Progress | HTML5-based UI |
| Driver Framework | ✅ Active | Universal driver API |
| Security | ✅ Active | W^X, ASLR, capabilities |

## 🔧 Key Design Principles

1. **Zero External Dependencies** - No Cargo production dependencies
2. **Sovereign Code** - All algorithms implemented from scratch
3. **no_std Kernel** - Kernel/klib uses `#![no_std]`
4. **Linux/BSD Inspiration** - Best ideas absorbed from existing OSes
5. **Security First** - W^X, ASLR, seccomp-like policies

## 📚 All Wiki Pages

### Architecture & Design
- [Architecture](Architecture) | [Architecture Overview](Architecture-Overview) | [Kernel Internals](Kernel-Internals)
- [AI Daemon Architecture](AI_DAEMON_ARCH) | [Metakernel Orchestration Blueprint](METAKERNEL_ORCHESTRATION_BLUEPRINT)
- [Constellation Mesh Roadmap](Constellation_Mesh_Roadmap) | [Policy Mechanism Roadmap](Policy_Mechanism_Roadmap)

### Roadmaps
- [Future Development Roadmap](Future_Development_Roadmap) | [3 Year Strategic Vision](3-Year-Strategic-Vision)
- [Gap Closing Roadmap](Gap_Closing_Roadmap) | [Linux Distro Parity Roadmap](LINUX_DISTRO_PARITY_ROADMAP)
- [Driver Management Roadmap](Driver_Management_Roadmap) | [Realtime HPC Scheduling Roadmap](REALTIME_HPC_SCHEDULING_ROADMAP)

### Security
- [Security Policy](SECURITY) | [Security Hardening Guide](Security-Hardening-Guide)
- [Defensive Audit Systems Blueprint](DEFENSIVE_AUDIT_SYSTEMS_BLUEPRINT)
- [Parrot Security Parity Blueprint](PARROT_SECURITY_PARITY_BLUEPRINT)
- [Qubes Isolation Roadmap](QUBES_ISOLATION_ROADMAP)

### Hardware & Drivers
- [Hardware Compatibility](Hardware-Compatibility) | [Driver Development Guide](Driver-Development-Guide)
- [Universal Driver Support Plan](UNIVERSAL_DRIVER_SUPPORT_PLAN)

### Community
- [Contributing](Contributing) | [Contributor FAQ](CONTRIBUTOR_FAQ)
- [Code of Conduct](Code-of-Conduct) | [Governance](RELEASE_GOVERNANCE_CI_CD)
- [Changelog](CHANGELOG)

---
*Last updated: 2026-08-06 | [Edit this page](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Home/_edit)*
