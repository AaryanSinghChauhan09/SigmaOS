<<<<<<< HEAD
# Changelog
<<<<<<< HEAD
=======

All notable changes to SigmaOS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Universal package manager with support for apt, yum, pacman, snap, flatpak
- Cross-platform compatibility layer for Windows .exe, macOS .dmg, Android .apk
- AI-powered system-level automation with predictive capabilities
- Built-in virtualization support with KVM/QEMU, Docker, Kubernetes
- Unified dashboard system with real-time monitoring
- Accessibility framework with vision/hearing/mobility/cognitive support
- Customization engine with Samsung Modes & Routines-style automation
- Gamified productivity system with achievements and Pomodoro timer
- Cross-device orchestration for IoT and smart home integration
- Round-robin scheduler with time-sliced execution
- USB HID keyboard driver with event handling
- VESA framebuffer driver with mode switching
- Package recipe system for build automation

### Changed
- Enhanced buddy allocator with memory initialization and statistics
- Improved dependency resolution in package manager
- Updated security framework with additional capability checks

### Fixed
- Integer overflow vulnerability in buddy allocator
- Integer overflow vulnerabilities in filesystem read/write operations
- Various memory safety issues identified by code scanning

### Security
- Added GitHub Actions CI workflow with security checks
- Implemented Dependabot for automated dependency updates
- Enhanced SECURITY.md with comprehensive security policy

## [0.1.0] - 2024-07-15

### Added
- Initial SigmaOS kernel implementation
- Capability-based security system
- SigmaPkg package manager foundation
- EEVDF scheduler implementation
- Buddy allocator for memory management
- Capability-based IPC system
- TCP/IP stack implementation
- Virtual filesystem with capability-based security
- GPU, storage, network, and input drivers
- Sigma-sh REPL shell
- AI-driven optimization system
- Resilience and self-healing modules

### Security
- Post-quantum cryptography support (Kyber-1024, Dilithium-5)
- Capability-based access control
- Secure boot implementation
- Memory safety guarantees through Rust

## [0.0.1] - 2024-07-01

### Added
- Project initialization
- Basic repository structure
- Initial documentation
- CI/CD pipeline setup
>>>>>>> origin/jules-14101877193021869698-2d1e023c
=======
# SigmaOS Changelog

All notable changes to the SigmaOS sovereign microkernel and system services are documented here.

## [1.1.0] - 2026-08-02
### Added
- SteamOS-style GPU driver recovery and reset in `drivers/graphics/sigma_kms.cpp`.
- Clear Linux-inspired dynamic power/performance scaling profiles in the graphics driver.
- Polymorphic universal peripheral matching and USB speed negotiation state machine in `drivers/usb/sigma_usb_hcd.cpp`.
- Zero-allocation DAG Topological Sorter (Kahn's Algorithm) in `kernel/drivers/sigma_driver_manager.cpp` to sequence loading dependencies.
- Native Hardware and Drivers test suite in `tests/sigma_test_runner.cpp` with 46 passing assertions.

## [1.0.0] - 2026-07-15
### Added
- First public release of SigmaOS sovereign system core.
- Capability-Based Sandboxing and Pledge/Unveil permission checks.
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
