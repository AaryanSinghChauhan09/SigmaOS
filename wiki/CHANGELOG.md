# SigmaOS Changelog

All notable changes to SigmaOS will be documented in this file.

## [Unreleased]

### Added

#### Phase G Foundation Hardening
- Capability-token delegation system for microkernel security
- Deterministic interrupt handling with priority queues
- Zero-copy IPC system with <100μs latency target
- Comprehensive fuzzing harness for kernel message passing
- Cache-aware scheduling algorithm with NUMA optimization

#### Branch Merges
- feature/improve-kernel-headers-linux-inspired: Linux-inspired kernel headers
- feature/wireshark-distro-improvements: Network analysis and Wireshark parity
- fix/mem-leak-custom-vec-drop: Memory leak fixes in custom Vec implementation
- improve-package-manager-and-containers: Enhanced package management and container runtime
- improve-sigmaos-systemd: Systemd-inspired init system improvements
- improve-sshd: SSH daemon security and OpenSSH parity
- universal-driver-support: Comprehensive driver and device management
- Multiple jules-* branches: Various improvements and bug fixes

#### Documentation
- Comprehensive README.md with project overview
- Detailed ARCHITECTURE.md with system architecture
- SECURITY.md with security features and best practices
- CONTRIBUTING.md with contribution guidelines
- Updated wiki/README.md with wiki navigation

### Changed

- Improved kernel memory management with Windows NT-style pool management
- Enhanced scheduling with EEVDF and BORE support
- Improved compatibility layers for Linux distributions
- Enhanced security with vulnerability detection and management
- Improved network stack with firewall and socket improvements

### Fixed

- Memory leak in custom Vec implementation
- Various compilation issues across kernel modules
- Driver framework improvements
- Package store fixes

## [Previous Releases]

### Version 0.1.0

#### Initial Release
- Basic kernel implementation
- Round-robin scheduler
- Buddy allocator
- Basic syscall dispatcher
- Page table walker
- Slab allocator
- Framebuffer driver
- Basic driver support

---

*For more detailed information about changes, see the [GitHub Commits](https://github.com/AaryanSinghChauhan09/SigmaOS/commits/main)*
