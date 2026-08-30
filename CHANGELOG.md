# SigmaOS Changelog

All notable changes to SigmaOS are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[Unreleased]

### Added

*   S-AI multi-agent orchestrator with local LLM routing engine
*   Activity manager paging and segmentation support
*   CachyOS-inspired performance optimizations (EEVDF, THP, zRAM)
*   OKR/Governance engine (`src/governance/okr.rs`)
*   3-Year Strategic Vision Roadmap
*   Comprehensive wiki documentation suite
*   Components reference table
*   Linux distro ideas catalog
*   Security policy and reporting procedures
*   Package manager documentation
*   Networking stack documentation
*   Kernel scheduler documentation

### Changed

*   Merged all feature branches into main (clean single-branch repo)
*   Enhanced wiki with 12+ structured pages
*   Updated branch consolidation reports
*   Universal package manager with support for apt, yum, pacman, snap, flatpak
*   Cross-platform compatibility layer for Windows .exe, macOS .dmg, Android .apk
*   AI-powered system-level automation with predictive capabilities

### Fixed

*   Merge conflicts resolved across all feature branches
*   Branch cleanup completed (only main remains)

## \[0.5.0-beta] - 2026-Q2

### Added

*   Sigma Shell (Wayland compositor)
*   Palette Theme Engine v1
*   Sentinel Security daemon v1
*   eBPF firewall (XDP/TC)
*   WireGuard built-in VPN
*   AUR compatibility layer
*   Flatpak runtime integration
*   sigma-pkg v1.0
*   SELinux enforcing mode
*   AppArmor profiles
*   Btrfs auto-snapshots
*   UEFI Secure Boot

### Changed

*   Migrated kernel core to Rust (safety-critical paths)
*   Updated EEVDF scheduler config
*   Enhanced S-AI Orchestrator v0.5

## \[0.1.0-alpha] - 2026-Q1

### Added

*   Initial hybrid kernel implementation
*   Basic EEVDF scheduler
*   Memory manager (NUMA-aware)
*   VFS layer
*   eBPF runtime
*   sigma-init (systemd fork)
*   Basic package manager
*   x86\_64 support
*   ARM64 initial port
*   Built-in virtualization support with KVM/QEMU, Docker, Kubernetes
*   Unified dashboard system with real-time monitoring
*   Accessibility framework with vision/hearing/mobility/cognitive support
*   Customization engine with Samsung Modes & Routines-style automation
*   Gamified productivity system with achievements and Pomodoro timer
*   Cross-device orchestration for IoT and smart home integration
*   Round-robin scheduler with time-sliced execution
*   USB HID keyboard driver with event handling
*   VESA framebuffer driver with mode switching
*   Package recipe system for build automation

### Changed

*   Enhanced buddy allocator with memory initialization and statistics
*   Improved dependency resolution in package manager
*   Updated security framework with additional capability checks

### Fixed

*   Integer overflow vulnerability in buddy allocator
*   Integer overflow vulnerabilities in filesystem read/write operations
*   Various memory safety issues identified by code scanning

### Security

*   Added GitHub Actions CI workflow with security checks
*   Implemented Dependabot for automated dependency updates
*   Enhanced SECURITY.md with comprehensive security policy
*   Post-quantum cryptography support (Kyber-1024, Dilithium-5)
*   Capability-based access control
*   Secure boot implementation
*   Memory safety guarantees through Rust

## \[0.0.1] - 2024-07-01

### Added

*   Project initialization
*   Basic repository structure
*   Initial documentation
*   CI/CD pipeline setup
